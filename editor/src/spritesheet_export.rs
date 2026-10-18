//! Spritesheet publication through the M004.1 generated-PNG transaction.
use ankhimate_document::Document;
use ankhimate_export::{
    run::{self, Plan},
    template::RenderedFile,
};
use ankhimate_render::{
    framing::{FrameOptions, PreparedFrames},
    sequence::{Sequence, SequenceRequest},
    spritesheet::{self, Layout},
};
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

pub(crate) fn output_plan(
    doc: &Document,
    request: &SequenceRequest,
    max_page: u32,
    padding: u32,
) -> Result<(Layout, Plan), String> {
    let count = request.frame_count(doc).map_err(|e| e.to_string())?;
    let layout = Layout::new(count, request.width, request.height, max_page, padding)
        .map_err(|e| e.to_string())?;
    let plan = Plan {
        files: vec![RenderedFile {
            path: "spritesheet.json".into(),
            contents: "{}".into(),
        }],
        binaries: layout
            .pages()
            .iter()
            .map(|p| (spritesheet::page_name(p.index), Vec::new()))
            .collect(),
    };
    Ok((layout, plan))
}

pub fn export(
    doc: &Document,
    request: &SequenceRequest,
    max_page: u32,
    padding: u32,
    destination: &Path,
    cancel: &AtomicBool,
    progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    export_options(
        doc,
        request,
        max_page,
        FrameOptions {
            padding,
            ..Default::default()
        },
        destination,
        cancel,
        progress,
    )
}

pub fn export_options(
    doc: &Document,
    request: &SequenceRequest,
    max_page: u32,
    options: FrameOptions,
    destination: &Path,
    cancel: &AtomicBool,
    mut progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    let padding = options.padding;
    options
        .validate(request.width, request.height)
        .map_err(|e| e.to_string())?;
    let count = request.frame_count(doc).map_err(|e| e.to_string())?;
    // Reject impossible dimensions before any staging, texture decoding or I/O.
    let schema = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let diagnostics = ankhimate_export::preflight::inspect(
        &schema,
        &doc.assets,
        ankhimate_export::preflight::Target::Rendered,
        Some(&request.animation),
    );
    if diagnostics.has_errors() {
        return Err(diagnostics.errors());
    }
    let (layout, mut plan) = output_plan(doc, request, max_page, padding)?;
    if !plan.survey(destination).replaced.is_empty() {
        return Err(
            "Spritesheet output already exists; choose a new folder. Nothing was replaced".into(),
        );
    }
    let sequence = Sequence::prepare(doc, request, |done, total| {
        if done % (total / 100).max(1) == 0 {
            progress("Framing", done, total);
        }
        !cancel.load(Ordering::Relaxed)
    })
    .map_err(|e| e.to_string())?;
    let prepared = if options.crops() {
        Some(
            PreparedFrames::prepare(&sequence, options, |done, total| {
                progress("Pixel bounds", done, total);
                !cancel.load(Ordering::Relaxed)
            })
            .map_err(|e| e.to_string())?,
        )
    } else {
        None
    };
    let camera = sequence.camera();
    let clip = doc
        .animations
        .values()
        .find(|clip| clip.name == request.animation)
        .unwrap();
    let duration_ms = 1000.0 / f64::from(request.fps);
    plan.files[0].contents=serde_json::to_string_pretty(&serde_json::json!({
        "spritesheet_version":1,"animation":request.animation,"fps":request.fps,"duration":clip.duration,
        "looping":clip.looping,"frame_count":count,"sampling":"i / fps; end exclusive",
        "alpha":"straight","padding":padding,"extrude":padding,"trim":false,"rotated":false,
        "max_page_size":max_page,"rgba_buffer_bound_bytes":layout.max_rgba_bytes(),
        "skin":doc.skeleton.skins[doc.skeleton.default_skin].name,
        "camera":{"center":camera.center.to_array(),"zoom":camera.zoom},
        "origin":{"pivot":{"x":0.5,"y":0.5},"world":camera.center.to_array(),
            "world_zero_in_frame":{"x":request.width as f32*0.5-camera.center.x*camera.zoom,"y":request.height as f32*0.5+camera.center.y*camera.zoom}},
        "pages":layout.pages().iter().map(|page|serde_json::json!({"index":page.index,"image":spritesheet::page_name(page.index),"width":page.width,"height":page.height})).collect::<Vec<_>>(),
        "frames":layout.frames().iter().map(|frame|serde_json::json!({"index":frame.index,"name":format!("frame_{:06}",frame.index),"page":frame.page,"rect":frame.rect,
            "source_size":{"w":request.width,"h":request.height},"duration_ms":duration_ms,"pivot":{"x":0.5,"y":0.5}})).collect::<Vec<_>>()
    })).map_err(|e|e.to_string())?;
    if let Some(prepared) = &prepared {
        let mut metadata: serde_json::Value =
            serde_json::from_str(&plan.files[0].contents).map_err(|e| e.to_string())?;
        let mut mapping = serde_json::to_value(prepared).map_err(|e| e.to_string())?;
        metadata["trim"] = serde_json::json!(true);
        // Cropping can temporarily hold both the source and its cropped copy.
        metadata["rgba_buffer_bound_bytes"] = serde_json::json!(
            layout.max_rgba_bytes() + u64::from(request.width) * u64::from(request.height) * 4
        );
        for (index, frame) in layout.frames().iter().enumerate() {
            let r = prepared.frames[index].source_rect;
            let rect = ankhimate_render::spritesheet::PixelRect {
                w: r.w,
                h: r.h,
                ..frame.rect
            };
            metadata["frames"][index]["rect"] = serde_json::json!(rect);
            mapping["frames"][index]["content_rect"] = serde_json::json!(rect);
            let page = &layout.pages()[frame.page as usize];
            mapping["frames"][index]["image_size"] =
                serde_json::json!({"w":page.width,"h":page.height});
        }
        metadata["reconstruction"] = mapping;
        plan.files[0].contents =
            serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    }
    run::write_generated_pngs(
        &plan,
        destination,
        |index, _| {
            let report = |done| {
                progress("Rendering", done, count);
                !cancel.load(Ordering::Relaxed)
            };
            let png = if let Some(prepared) = &prepared {
                spritesheet::render_page_cropped(
                    &sequence,
                    &layout,
                    index as u32,
                    Some(prepared),
                    report,
                )
            } else {
                spritesheet::render_page(&sequence, &layout, index as u32, report)
            }
            .map_err(|e| e.to_string())?;
            Ok(png.bytes)
        },
        || cancel.load(Ordering::Relaxed),
    )
    .map_err(|e| e.to_string())?;
    progress("Complete", count, count);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ankhimate_core::animation::{Animation, Axis, Key, Timeline};
    use ankhimate_document::{Document, History, commands::psd_cmds::ImportPsd};
    use ankhimate_render::sequence::SequenceRequest;

    fn fixtures() -> Vec<(Document, String)> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
        let walker = Document::from_loaded(ankhimate_formats::load(&path).unwrap().0);
        let imported = ankhimate_formats::psd::import(
            include_bytes!("../../formats/tests/fixtures/tagged_rig.psd"),
            &Default::default(),
        )
        .unwrap();
        let mut tagged = Document::new();
        History::default().push(
            Box::new(ImportPsd::new(imported, true, "tagged")),
            &mut tagged,
        );
        let mut animation = Animation::new("tagged\"雪", 1.0);
        animation.timelines.push(Timeline::BoneTranslate {
            bone: tagged.skeleton.update_order[0],
            axis: Axis::X,
            keys: vec![Key::linear(0.0, 0.0), Key::linear(1.0, 10.0)],
        });
        tagged.animations.insert(animation);
        vec![
            (walker.clone(), "walk".into()),
            (walker, "leg_ik".into()),
            (tagged, "tagged\"雪".into()),
        ]
    }

    #[test]
    fn three_real_clips_reconstruct_m0041_pixels_and_exports_are_deterministic() {
        for (doc, clip) in fixtures() {
            let request = SequenceRequest {
                animation: clip.clone(),
                fps: 60,
                width: 24,
                height: 20,
            };
            let dir = tempfile::tempdir().unwrap();
            let repeated = tempfile::tempdir().unwrap();
            let individual = tempfile::tempdir().unwrap();
            let cancel = AtomicBool::new(false);
            let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
            crate::png_export::export(&doc, &request, individual.path(), &cancel, |_, _, _| {})
                .unwrap();
            export(&doc, &request, 128, 2, dir.path(), &cancel, |_, _, _| {}).unwrap();
            export(
                &doc,
                &request,
                128,
                2,
                repeated.path(),
                &cancel,
                |_, _, _| {},
            )
            .unwrap();
            let bytes = std::fs::read(dir.path().join("spritesheet.json")).unwrap();
            assert_eq!(
                bytes,
                std::fs::read(repeated.path().join("spritesheet.json")).unwrap()
            );
            let meta: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(meta["spritesheet_version"], 1);
            assert_eq!(meta["frame_count"], 60);
            assert_eq!(meta["animation"], clip);
            assert_eq!(meta["frames"].as_array().unwrap().len(), 60);
            assert_eq!(meta["pages"].as_array().unwrap().len(), 3);
            let sequence_meta: serde_json::Value = serde_json::from_slice(
                &std::fs::read(individual.path().join("sequence.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(meta["camera"], sequence_meta["camera"]);
            for page in meta["pages"].as_array().unwrap() {
                let name = page["image"].as_str().unwrap();
                assert_eq!(
                    std::fs::read(dir.path().join(name)).unwrap(),
                    std::fs::read(repeated.path().join(name)).unwrap()
                );
            }
            for frame in meta["frames"].as_array().unwrap() {
                let index = frame["index"].as_u64().unwrap() as u32;
                let page = &meta["pages"][frame["page"].as_u64().unwrap() as usize];
                let pixels = image::open(dir.path().join(page["image"].as_str().unwrap()))
                    .unwrap()
                    .to_rgba8();
                let rect = &frame["rect"];
                let crop = image::imageops::crop_imm(
                    &pixels,
                    rect["x"].as_u64().unwrap() as u32,
                    rect["y"].as_u64().unwrap() as u32,
                    24,
                    20,
                )
                .to_image();
                let expected =
                    image::open(individual.path().join(crate::png_export::frame_name(index)))
                        .unwrap()
                        .to_rgba8();
                assert_eq!(crop, expected, "{clip} frame {index}");
                assert_eq!(frame["source_size"], serde_json::json!({"w":24,"h":20}));
                assert!((frame["duration_ms"].as_f64().unwrap() - 1000.0 / 60.0).abs() < 1e-9);
            }
            assert_eq!(
                before,
                ankhimate_formats::to_json(&doc.as_project_ref()).unwrap()
            );
        }
    }

    #[test]
    fn failure_cancel_collision_and_impossible_page_preserve_previous_files() {
        let (doc, clip) = fixtures().remove(0);
        let request = SequenceRequest {
            animation: clip,
            fps: 60,
            width: 24,
            height: 20,
        };
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("keep.txt"), b"previous output").unwrap();
        let cancel = AtomicBool::new(false);
        assert!(
            export(
                &doc,
                &request,
                128,
                2,
                dir.path(),
                &cancel,
                |phase, done, _| {
                    if phase == "Rendering" && done >= 2 {
                        cancel.store(true, Ordering::Relaxed);
                    }
                }
            )
            .is_err()
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        assert!(
            export(
                &doc,
                &request,
                128,
                2,
                &dir.path().join("keep.txt"),
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        std::fs::write(dir.path().join("sheet_000000.png"), b"old sheet").unwrap();
        assert!(
            export(
                &doc,
                &request,
                128,
                2,
                dir.path(),
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        assert_eq!(
            std::fs::read(dir.path().join("sheet_000000.png")).unwrap(),
            b"old sheet"
        );
        let impossible = dir.path().join("not-created");
        assert!(
            export(
                &doc,
                &request,
                16,
                2,
                &impossible,
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        assert!(!impossible.exists());
        assert_eq!(
            std::fs::read(dir.path().join("keep.txt")).unwrap(),
            b"previous output"
        );
    }
}
