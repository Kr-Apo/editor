//! End-to-end rendered-output reconstruction, on isolated temporary files.
use crate::{png_export, spritesheet_export};
use ankhimate_document::Document;
use ankhimate_render::{encoding::FrameEncoding, framing::FrameOptions, sequence::SequenceRequest};
use std::{path::Path, sync::atomic::AtomicBool};
fn walker() -> Document {
    Document::from_loaded(
        ankhimate_formats::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh"),
        )
        .unwrap()
        .0,
    )
}
fn request(clip: &str) -> SequenceRequest {
    SequenceRequest {
        animation: clip.into(),
        fps: 4,
        width: 64,
        height: 64,
    }
}
fn metadata(dir: &Path, name: &str) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(dir.join(name)).unwrap()).unwrap()
}
#[test]
fn trimmed_sequence_provides_reconstruction_offsets() {
    let doc = walker();
    let dir = tempfile::tempdir().unwrap();
    png_export::export_options(
        &doc,
        &request("walk"),
        FrameEncoding::Png,
        FrameOptions {
            trim: true,
            ..Default::default()
        },
        dir.path(),
        &AtomicBool::new(false),
        |_, _, _| {},
    )
    .unwrap();
    let meta = metadata(dir.path(), "sequence.json");
    assert!(
        meta["reconstruction"].is_object(),
        "Requested trim is unavailable: no source rectangle / encoded content rectangle"
    );
}
#[test]
fn trimmed_sheet_provides_reconstruction_offsets() {
    let doc = walker();
    let dir = tempfile::tempdir().unwrap();
    spritesheet_export::export_options(
        &doc,
        &request("walk"),
        128,
        FrameOptions {
            trim: true,
            padding: 2,
            ..Default::default()
        },
        dir.path(),
        &AtomicBool::new(false),
        |_, _, _| {},
    )
    .unwrap();
    let meta = metadata(dir.path(), "spritesheet.json");
    assert_eq!(
        meta["trim"], true,
        "Requested trim is unavailable: spritesheets always declare trim=false"
    );
}

use ankhimate_core::{
    animation::{Animation, Axis, Key, Timeline},
    assets::ImageAsset,
    attachment::{Attachment, Rect, RegionAttachment},
    glam::Vec2,
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_render::{framing::BoundsMode, sequence::Sequence};
use image::{ImageEncoder, RgbaImage};
use serde_json::Value;
fn fixture() -> Document {
    let mut doc = Document::new();
    let mut art = RgbaImage::new(8, 8);
    for y in 2..6 {
        for x in 2..6 {
            art.put_pixel(
                x,
                y,
                image::Rgba([40, 160, 220, if x == 2 || y == 2 { 1 } else { 160 }]),
            );
        }
    }
    let mut bytes = vec![];
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(art.as_raw(), 8, 8, image::ExtendedColorType::Rgba8)
        .unwrap();
    doc.assets.add(ImageAsset::new("margins", bytes, 8, 8));
    let bone = doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 1.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut slot = Slot::new("art".into(), bone);
    slot.attachment = Some("a".into());
    let slot = doc.skeleton.add_slot(slot);
    let skin = doc.skeleton.default_skin;
    for (name, width, height) in [("a", 20.0, 12.0), ("b", 14.0, 24.0)] {
        doc.skeleton.skins[skin].set(
            slot,
            name,
            Attachment::Region(RegionAttachment {
                texture: "margins".into(),
                width,
                height,
                local_offset: Vec2::ZERO,
                local_rotation: 0.0,
                local_scale: Vec2::ONE,
                uv_rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                },
                pivot: Vec2::splat(0.5),
                sequence: None,
            }),
        );
    }
    let mut clip = Animation::new("move", 1.0);
    clip.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::X,
        keys: vec![Key::linear(0.0, -20.0), Key::linear(0.75, 20.0)],
    });
    clip.timelines.push(Timeline::SlotAttachment {
        slot,
        keys: vec![
            Key::linear(0.0, Some("a".into())),
            Key::linear(0.25, Some("b".into())),
            Key::linear(0.75, None),
        ],
    });
    doc.animations.insert(clip);
    doc.animations.insert(Animation::new("still", 1.0));
    doc
}
fn u(v: &Value) -> u32 {
    v.as_u64().unwrap() as u32
}
fn restore(image: &RgbaImage, mapping: &Value, size: &Value) -> RgbaImage {
    let mut out = RgbaImage::new(u(&size["w"]), u(&size["h"]));
    let src = &mapping["source_rect"];
    let content = &mapping["content_rect"];
    assert_eq!(src["w"], content["w"]);
    assert_eq!(src["h"], content["h"]);
    assert_eq!(
        image.dimensions(),
        (
            u(&mapping["image_size"]["w"]),
            u(&mapping["image_size"]["h"])
        )
    );
    for y in 0..u(&src["h"]) {
        for x in 0..u(&src["w"]) {
            out.put_pixel(
                x + u(&src["x"]),
                y + u(&src["y"]),
                *image.get_pixel(x + u(&content["x"]), y + u(&content["y"])),
            );
        }
    }
    out
}
fn options(bounds: BoundsMode, trim: bool, padding: u32) -> FrameOptions {
    FrameOptions {
        bounds,
        trim,
        padding,
    }
}
#[test]
fn rgba_reconstruction_is_exact_for_moving_switching_disappearing_and_static_art() {
    let doc = fixture();
    for clip in ["move", "still"] {
        for bounds in [BoundsMode::Historical, BoundsMode::GlobalPixels] {
            for trim in [false, true] {
                for encoding in [FrameEncoding::Png, FrameEncoding::WebpLossless] {
                    let req = request(clip);
                    let seq = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
                    let dir = tempfile::tempdir().unwrap();
                    let opts = options(bounds, trim, 2);
                    assert_eq!(
                        png_export::export_options(
                            &doc,
                            &req,
                            encoding,
                            opts,
                            dir.path(),
                            &AtomicBool::new(false),
                            |_, _, _| {}
                        )
                        .unwrap(),
                        4
                    );
                    let meta = metadata(dir.path(), "sequence.json");
                    let reconstruction = &meta["reconstruction"];
                    assert_eq!(reconstruction["version"], 1);
                    assert_eq!(reconstruction["required"], true);
                    assert_eq!(meta["sampling"], "i / fps; end exclusive");
                    assert_eq!(meta["fps"], 4);
                    assert_eq!(meta["duration"], 1.0);
                    let mut sizes = vec![];
                    for i in 0..4 {
                        let decoded = image::open(dir.path().join(encoding.frame_name(i)))
                            .unwrap()
                            .to_rgba8();
                        let map = &reconstruction["frames"][i as usize];
                        let expected = seq.render_rgba(i).unwrap();
                        assert_pixels(
                            &restore(&decoded, map, &reconstruction["source_size"]),
                            &expected,
                            &format!(
                                "{clip} {bounds:?} trim={trim} {encoding:?} frame={i}, mapping {map}"
                            ),
                        );
                        if clip == "move" {
                            assert_eq!(map["empty"], i == 3);
                        }
                        sizes.push(decoded.dimensions());
                    }
                    if !trim {
                        assert!(
                            sizes.iter().all(|s| *s == sizes[0]),
                            "common global framing is stable"
                        );
                    }
                    if clip == "still" {
                        assert!(
                            u(&reconstruction["frames"][0]["source_rect"]["w"]) < 64
                                || !trim && bounds == BoundsMode::Historical
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn trimmed_real_clips_have_deterministic_bytes_camera_and_timing() {
    let doc = walker();
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    for clip in ["walk", "leg_ik"] {
        let mut req = request(clip);
        req.fps = 60;
        for encoding in [FrameEncoding::Png, FrameEncoding::WebpLossless] {
            let one = tempfile::tempdir().unwrap();
            let two = tempfile::tempdir().unwrap();
            let opts = options(BoundsMode::GlobalPixels, true, 1);
            for dir in [one.path(), two.path()] {
                assert_eq!(
                    png_export::export_options(
                        &doc,
                        &req,
                        encoding,
                        opts,
                        dir,
                        &AtomicBool::new(false),
                        |_, _, _| {}
                    )
                    .unwrap(),
                    60
                );
                assert_eq!(std::fs::read_dir(dir).unwrap().count(), 61);
            }
            let a = metadata(one.path(), "sequence.json");
            assert_eq!(a, metadata(two.path(), "sequence.json"));
            let seq = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
            assert_eq!(
                a["camera"],
                serde_json::json!({"center":seq.camera().center.to_array(),"zoom":seq.camera().zoom})
            );
            for i in 0..60 {
                let first = std::fs::read(one.path().join(encoding.frame_name(i))).unwrap();
                assert_eq!(
                    first,
                    std::fs::read(two.path().join(encoding.frame_name(i))).unwrap()
                );
                let rgba = image::load_from_memory(&first).unwrap().to_rgba8();
                assert_eq!(
                    restore(
                        &rgba,
                        &a["reconstruction"]["frames"][i as usize],
                        &a["reconstruction"]["source_size"]
                    ),
                    seq.render_rgba(i).unwrap()
                );
            }
        }
    }
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn jpeg_uses_rgba_crop_bounds_before_background_and_preserves_opaque_fill() {
    let doc = fixture();
    let req = request("move");
    let seq = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
    let opts = options(BoundsMode::GlobalPixels, true, 2);
    let jpeg = tempfile::tempdir().unwrap();
    let png = tempfile::tempdir().unwrap();
    let bg = [24u8, 80, 160];
    for (encoding, dir) in [
        (FrameEncoding::Jpeg { background: bg }, jpeg.path()),
        (FrameEncoding::Png, png.path()),
    ] {
        png_export::export_options(
            &doc,
            &req,
            encoding,
            opts,
            dir,
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
    }
    let meta = metadata(jpeg.path(), "sequence.json");
    assert_eq!(
        meta["reconstruction"],
        metadata(png.path(), "sequence.json")["reconstruction"]
    );
    assert_eq!(meta["background"], serde_json::json!([24, 80, 160, 255]));
    let mut max_mae = 0f64;
    let mut max_rmse = 0f64;
    for i in 0..4 {
        let rgb = image::open(
            jpeg.path()
                .join(FrameEncoding::Jpeg { background: bg }.frame_name(i)),
        )
        .unwrap()
        .to_rgb8();
        let raw = image::open(png.path().join(FrameEncoding::Png.frame_name(i)))
            .unwrap()
            .to_rgba8();
        assert_eq!(rgb.dimensions(), raw.dimensions());
        let mut sum = 0f64;
        let mut sq = 0f64;
        for (a, b) in raw.pixels().zip(rgb.pixels()) {
            for (c, bg) in bg.iter().enumerate() {
                let alpha = u32::from(a.0[3]);
                let expected = ((u32::from(a.0[c]) * alpha + u32::from(*bg) * (255 - alpha) + 127)
                    / 255) as u8;
                let error = f64::from(expected.abs_diff(b.0[c]));
                sum += error;
                sq += error * error;
            }
        }
        let n = f64::from(rgb.width() * rgb.height() * 3);
        let mae = sum / n;
        let rmse = (sq / n).sqrt();
        max_mae = max_mae.max(mae);
        max_rmse = max_rmse.max(rmse);
        assert!(
            mae <= 4.0 && rmse <= 12.0,
            "frame {i} MAE {mae}, RMSE {rmse}"
        );
        if i == 3 {
            assert!(seq.render_rgba(i).unwrap().pixels().all(|p| p.0[3] == 0));
            assert_eq!(rgb.dimensions(), (5, 5));
            for p in rgb.pixels() {
                for (a, b) in p.0.iter().zip(bg) {
                    assert!(a.abs_diff(b) <= 2);
                }
            }
        }
    }
    eprintln!("T6012 JPEG crop max MAE={max_mae:.6}, RMSE={max_rmse:.6}; limits 4/12");
}
#[test]
fn cropped_sheets_are_multipage_isolated_and_reconstruct_exact_pixels() {
    let doc = fixture();
    let mut req = request("move");
    req.fps = 12;
    let seq = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
    for bounds in [BoundsMode::Historical, BoundsMode::GlobalPixels] {
        for trim in [false, true] {
            if !trim && bounds == BoundsMode::Historical {
                continue;
            }
            let dir = tempfile::tempdir().unwrap();
            let opts = options(bounds, trim, 2);
            spritesheet_export::export_options(
                &doc,
                &req,
                136,
                opts,
                dir.path(),
                &AtomicBool::new(false),
                |_, _, _| {},
            )
            .unwrap();
            let repeat = tempfile::tempdir().unwrap();
            spritesheet_export::export_options(
                &doc,
                &req,
                136,
                opts,
                repeat.path(),
                &AtomicBool::new(false),
                |_, _, _| {},
            )
            .unwrap();
            for entry in std::fs::read_dir(dir.path()).unwrap() {
                let path = entry.unwrap().path();
                assert_eq!(
                    std::fs::read(&path).unwrap(),
                    std::fs::read(repeat.path().join(path.file_name().unwrap())).unwrap()
                );
            }
            let meta = metadata(dir.path(), "spritesheet.json");
            assert_eq!(meta["pages"].as_array().unwrap().len(), 3);
            assert_eq!(meta["trim"], true);
            assert_eq!(meta["padding"], 2);
            assert_eq!(meta["extrude"], 2);
            for i in 0..12 {
                let frame = &meta["frames"][i];
                let mapping = &meta["reconstruction"]["frames"][i];
                assert_eq!(frame["rect"], mapping["content_rect"]);
                assert_eq!(frame["source_size"], serde_json::json!({"w":64,"h":64}));
                assert!((frame["duration_ms"].as_f64().unwrap() - 1000.0 / 12.0).abs() < 1e-9);
                let page = &meta["pages"][u(&frame["page"]) as usize];
                let image = image::open(dir.path().join(page["image"].as_str().unwrap()))
                    .unwrap()
                    .to_rgba8();
                assert_eq!(
                    restore(&image, mapping, &meta["reconstruction"]["source_size"]),
                    seq.render_rgba(i as u32).unwrap()
                );
                let r = &frame["rect"];
                for y in 0..u(&r["h"]) + 4 {
                    for x in 0..u(&r["w"]) + 4 {
                        let expected = image.get_pixel(
                            u(&r["x"]) + x.saturating_sub(2).min(u(&r["w"]) - 1),
                            u(&r["y"]) + y.saturating_sub(2).min(u(&r["h"]) - 1),
                        );
                        assert_eq!(
                            image.get_pixel(u(&r["x"]) - 2 + x, u(&r["y"]) - 2 + y),
                            expected
                        );
                    }
                }
                assert!(image.dimensions().0 <= 136 && image.dimensions().1 <= 136);
            }
        }
    }
}
#[test]
fn entirely_transparent_clips_have_valid_lossless_and_jpeg_representations() {
    let mut doc = Document::new();
    doc.animations.insert(Animation::new("empty", 1.0));
    for encoding in [
        FrameEncoding::Png,
        FrameEncoding::WebpLossless,
        FrameEncoding::Jpeg {
            background: [255; 3],
        },
    ] {
        let dir = tempfile::tempdir().unwrap();
        png_export::export_options(
            &doc,
            &request("empty"),
            encoding,
            options(BoundsMode::GlobalPixels, true, 0),
            dir.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        let meta = metadata(dir.path(), "sequence.json");
        for map in meta["reconstruction"]["frames"].as_array().unwrap() {
            assert_eq!(map["empty"], true);
            assert_eq!(
                map["source_rect"],
                serde_json::json!({"x":0,"y":0,"w":1,"h":1})
            );
        }
        let pixel = image::open(dir.path().join(encoding.frame_name(0)))
            .unwrap()
            .to_rgba8();
        assert_eq!(pixel.dimensions(), (1, 1));
        assert_eq!(
            pixel.get_pixel(0, 0).0,
            if matches!(encoding, FrameEncoding::Jpeg { .. }) {
                [255; 4]
            } else {
                [0; 4]
            }
        );
    }
}
#[test]
fn cancellation_invalid_options_and_collisions_never_replace_previous_files() {
    use std::sync::atomic::Ordering;
    let doc = fixture();
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    let opts = options(BoundsMode::GlobalPixels, true, 2);
    for encoding in [
        FrameEncoding::Png,
        FrameEncoding::WebpLossless,
        FrameEncoding::Jpeg {
            background: [24, 80, 160],
        },
    ] {
        for sheet in [false, true] {
            if sheet && encoding != FrameEncoding::Png {
                continue;
            }
            for phase in ["Pixel bounds", "Rendering"] {
                let dir = tempfile::tempdir().unwrap();
                std::fs::write(dir.path().join("prior.backup"), b"keep").unwrap();
                let cancel = AtomicBool::new(false);
                let mut completed = false;
                let progress = |p, done, _| {
                    if p == phase && done >= 1 {
                        cancel.store(true, Ordering::Relaxed);
                    }
                    if p == "Complete" {
                        completed = true;
                    }
                };
                let result = if sheet {
                    spritesheet_export::export_options(
                        &doc,
                        &request("move"),
                        68,
                        opts,
                        dir.path(),
                        &cancel,
                        progress,
                    )
                } else {
                    png_export::export_options(
                        &doc,
                        &request("move"),
                        encoding,
                        opts,
                        dir.path(),
                        &cancel,
                        progress,
                    )
                };
                assert!(result.is_err());
                assert!(!completed);
                assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
                assert_eq!(
                    std::fs::read(dir.path().join("prior.backup")).unwrap(),
                    b"keep"
                );
                let name = if sheet {
                    "sheet_000000.png".to_string()
                } else {
                    encoding.frame_name(0)
                };
                std::fs::write(dir.path().join(&name), b"prior image").unwrap();
                let result = if sheet {
                    spritesheet_export::export_options(
                        &doc,
                        &request("move"),
                        68,
                        opts,
                        dir.path(),
                        &AtomicBool::new(false),
                        |_, _, _| {},
                    )
                } else {
                    png_export::export_options(
                        &doc,
                        &request("move"),
                        encoding,
                        opts,
                        dir.path(),
                        &AtomicBool::new(false),
                        |_, _, _| {},
                    )
                };
                assert!(result.is_err());
                assert_eq!(
                    std::fs::read(dir.path().join(&name)).unwrap(),
                    b"prior image"
                );
            }
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("existing.backup");
    std::fs::write(&sentinel, b"last valid project").unwrap();
    for encoding in [
        FrameEncoding::Png,
        FrameEncoding::WebpLossless,
        FrameEncoding::Jpeg {
            background: [255; 3],
        },
    ] {
        assert!(
            png_export::export_options(
                &doc,
                &request("move"),
                encoding,
                opts,
                &sentinel,
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last valid project");
    }
    let dest = dir.path().join("not-created");
    let invalid = FrameOptions {
        padding: 33,
        ..opts
    };
    assert!(
        png_export::export_options(
            &doc,
            &request("move"),
            FrameEncoding::Png,
            invalid,
            &dest,
            &AtomicBool::new(false),
            |_, _, _| {}
        )
        .is_err()
    );
    assert!(!dest.exists());
    assert!(
        spritesheet_export::export_options(
            &doc,
            &request("move"),
            64,
            opts,
            &dest,
            &AtomicBool::new(false),
            |_, _, _| {}
        )
        .is_err()
    );
    assert!(!dest.exists());
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn save_as_and_reopen_preserve_author_data_and_cropped_export_pixels() {
    let doc = fixture();
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("original.ankh");
    ankhimate_formats::save(&path, &doc.as_project_ref(), &[]).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let second = dir.path().join("copie-蛙.ankh");
    ankhimate_formats::save(&second, &doc.as_project_ref(), &[]).unwrap();
    let reopened = Document::from_loaded(ankhimate_formats::load(&second).unwrap().0);
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        before
    );
    let opts = options(BoundsMode::GlobalPixels, true, 2);
    for (n, document) in [("before", &doc), ("reopened", &reopened)] {
        png_export::export_options(
            document,
            &request("move"),
            FrameEncoding::WebpLossless,
            opts,
            &dir.path().join(n),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
    }
    for i in 0..4 {
        assert_eq!(
            std::fs::read(
                dir.path()
                    .join("before")
                    .join(FrameEncoding::WebpLossless.frame_name(i))
            )
            .unwrap(),
            std::fs::read(
                dir.path()
                    .join("reopened")
                    .join(FrameEncoding::WebpLossless.frame_name(i))
            )
            .unwrap()
        );
    }
    assert_eq!(bytes, std::fs::read(&path).unwrap());
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}

fn assert_pixels(actual: &RgbaImage, expected: &RgbaImage, context: &str) {
    let first = actual
        .enumerate_pixels()
        .zip(expected.enumerate_pixels())
        .find(|(a, b)| a.2 != b.2);
    assert!(actual == expected, "{context}: first mismatch {first:?}");
}
