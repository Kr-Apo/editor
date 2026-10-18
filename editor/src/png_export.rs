//! Image sequence orchestration; the PNG API and output remain compatible.
//! The CPU renderer and M001 publisher retain their existing responsibilities;
//! this module connects them on a document snapshot.
use ankhimate_document::Document;
use ankhimate_export::{
    run::{self, Plan},
    template::RenderedFile,
};
use ankhimate_render::{
    encoding::FrameEncoding,
    framing::{FrameOptions, PreparedFrames, padded},
    sequence::{Sequence, SequenceRequest},
};
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn frame_name(index: u32) -> String {
    format!("frame_{index:06}.png")
}

pub(crate) fn output_plan(doc: &Document, request: &SequenceRequest) -> Result<Plan, String> {
    output_plan_encoded(doc, request, FrameEncoding::Png)
}
pub(crate) fn output_plan_encoded(
    doc: &Document,
    request: &SequenceRequest,
    encoding: FrameEncoding,
) -> Result<Plan, String> {
    let count = request.frame_count(doc).map_err(|e| e.to_string())?;
    Ok(Plan {
        files: vec![RenderedFile {
            path: "sequence.json".into(),
            contents: "{}".into(),
        }],
        binaries: (0..count)
            .map(|i| {
                (
                    if encoding == FrameEncoding::Png {
                        frame_name(i)
                    } else {
                        encoding.frame_name(i)
                    },
                    Vec::new(),
                )
            })
            .collect(),
    })
}

/// Existing names are deliberately refused: use a fresh destination for each
/// sequence. Frames are rendered, encoded and staged one at a time.
/// No authored document state is changed.
pub fn export(
    doc: &Document,
    request: &SequenceRequest,
    destination: &Path,
    cancel: &AtomicBool,
    progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    export_encoded(
        doc,
        request,
        FrameEncoding::Png,
        destination,
        cancel,
        progress,
    )
}

pub fn export_encoded(
    doc: &Document,
    request: &SequenceRequest,
    encoding: FrameEncoding,
    destination: &Path,
    cancel: &AtomicBool,
    progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    export_options(
        doc,
        request,
        encoding,
        FrameOptions::default(),
        destination,
        cancel,
        progress,
    )
}

pub fn export_options(
    doc: &Document,
    request: &SequenceRequest,
    encoding: FrameEncoding,
    options: FrameOptions,
    destination: &Path,
    cancel: &AtomicBool,
    mut progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    options
        .validate(request.width, request.height)
        .map_err(|e| e.to_string())?;
    let count = request.frame_count(doc).map_err(|e| e.to_string())?;
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
    let mut plan = output_plan_encoded(doc, request, encoding)?;
    if !plan.survey(destination).replaced.is_empty() {
        return Err(format!(
            "{} sequence output already exists; choose a new folder. Nothing was replaced",
            encoding.label()
        ));
    }
    let sequence = Sequence::prepare(doc, request, |done, total| {
        if done % (total / 100).max(1) == 0 {
            progress("Framing", done, total);
        }
        !cancel.load(Ordering::Relaxed)
    })
    .map_err(|e| e.to_string())?;
    let prepared = if options.crops() || options.padding > 0 {
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
    let clip = doc
        .animations
        .values()
        .find(|clip| clip.name == request.animation)
        .unwrap();
    let camera = sequence.camera();
    plan.files[0].contents = serde_json::to_string_pretty(&serde_json::json!({
        "png_sequence_version":1, "animation":request.animation, "fps":request.fps,
        "duration":clip.duration, "frame_count":count, "width":request.width,"height":request.height,
        "looping":clip.looping,"sampling":"i / fps; end exclusive","first_frame":0,
        "filename_pattern":"frame_%06d.png","alpha":"straight","background":[0,0,0,0],
        "camera":{"center":camera.center.to_array(),"zoom":camera.zoom},
        "skin":doc.skeleton.skins[doc.skeleton.default_skin].name,
        "physics":"unsupported; projects with physics constraints refused"
    })).map_err(|e|e.to_string())?;
    // Keep the original metadata keys/version. The legacy version key names
    // the v1 sidecar, while filename_pattern describes its actual image encoding.
    if encoding != FrameEncoding::Png {
        let mut metadata: serde_json::Value =
            serde_json::from_str(&plan.files[0].contents).map_err(|e| e.to_string())?;
        metadata["filename_pattern"] = serde_json::json!(encoding.filename_pattern());
        metadata["alpha"] = serde_json::json!(encoding.alpha());
        metadata["background"] = serde_json::json!(encoding.background());
        plan.files[0].contents =
            serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    }
    if let Some(prepared) = &prepared {
        let mut metadata: serde_json::Value =
            serde_json::from_str(&plan.files[0].contents).map_err(|e| e.to_string())?;
        metadata["reconstruction"] = serde_json::to_value(prepared).map_err(|e| e.to_string())?;
        plan.files[0].contents =
            serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    }
    let generate = |index: usize, _: &str| {
        if cancel.load(Ordering::Relaxed) {
            return Err("export cancelled".into());
        }
        let bytes = if let Some(prepared) = &prepared {
            let rgba = sequence
                .render_rgba(index as u32)
                .map_err(|e| e.to_string())?;
            let crop = prepared
                .crop(index as u32, &rgba)
                .map_err(|e| e.to_string())?;
            drop(rgba);
            encoding
                .encode(&padded(crop, options.padding).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?
        } else if encoding == FrameEncoding::Png {
            sequence
                .render(index as u32)
                .map_err(|e| e.to_string())?
                .bytes
        } else {
            encoding
                .encode(
                    &sequence
                        .render_rgba(index as u32)
                        .map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?
        };
        progress("Rendering", index as u32 + 1, count);
        Ok(bytes)
    };
    if encoding == FrameEncoding::Png {
        run::write_generated_pngs(&plan, destination, generate, || {
            cancel.load(Ordering::Relaxed)
        })
    } else {
        run::write_generated_images(&plan, destination, generate, || {
            cancel.load(Ordering::Relaxed)
        })
    }
    .map_err(|e| e.to_string())?;
    progress("Complete", count, count);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_setup_attachment_blocks_rendered_export_without_writing() {
        let loaded = ankhimate_formats::from_json(r#"{"version":1,"name":"broken","fps":60,"bones":[{"name":"root","length":1}],"slots":[{"name":"slot","bone":"root","attachment":"ghost"}],"skins":[{"name":"default","entries":[]}],"default_skin":"default","animations":[{"name":"clip","duration":1,"looping":true,"timelines":[]}] }"#).unwrap();
        let doc = ankhimate_document::Document::from_loaded(loaded);
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("must-not-exist");
        let request = SequenceRequest {
            animation: "clip".into(),
            fps: 2,
            width: 8,
            height: 8,
        };
        assert!(
            export(
                &doc,
                &request,
                &output,
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err(),
            "missing attachment must not become a successful transparent animation"
        );
        assert!(!output.exists());
    }
    use ankhimate_core::animation::{Animation, Axis, Key, Timeline};
    use ankhimate_document::{History, commands::psd_cmds::ImportPsd};
    use ankhimate_render::sequence::SequenceRequest;
    use std::sync::atomic::AtomicBool;

    fn walker() -> ankhimate_document::Document {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
        ankhimate_document::Document::from_loaded(ankhimate_formats::load(&path).unwrap().0)
    }
    fn request(animation: &str) -> SequenceRequest {
        SequenceRequest {
            animation: animation.into(),
            fps: 60,
            width: 64,
            height: 64,
        }
    }
    #[test]
    fn walker_exports_sixty_rgba_frames_and_repeat_pixels_match() {
        let doc = walker();
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        for dir in [first.path(), second.path()] {
            export(
                &doc,
                &request("walk"),
                dir,
                &AtomicBool::new(false),
                |_, _, _| {},
            )
            .unwrap();
            assert_eq!(std::fs::read_dir(dir).unwrap().count(), 61);
        }
        for i in 0..60 {
            let a = std::fs::read(first.path().join(frame_name(i))).unwrap();
            let b = std::fs::read(second.path().join(frame_name(i))).unwrap();
            assert_eq!(a, b);
            let image = image::load_from_memory(&a).unwrap().to_rgba8();
            assert_eq!(image.dimensions(), (64, 64));
            assert_eq!(image.get_pixel(0, 0).0, [0; 4]);
        }
        let metadata: serde_json::Value =
            serde_json::from_slice(&std::fs::read(first.path().join("sequence.json")).unwrap())
                .unwrap();
        assert_eq!(metadata["frame_count"], 60);
        assert_eq!(metadata["duration"], 1.0);
        assert_eq!(metadata["sampling"], "i / fps; end exclusive");
    }
    #[test]
    fn tagged_psd_project_exports_without_modifying_document_or_assets() {
        let import = ankhimate_formats::psd::import(
            include_bytes!("../../formats/tests/fixtures/tagged_rig.psd"),
            &Default::default(),
        )
        .unwrap();
        let mut doc = ankhimate_document::Document::new();
        History::default().push(Box::new(ImportPsd::new(import, true, "tagged")), &mut doc);
        let bone = doc.skeleton.update_order[0];
        let mut clip = Animation::new("move", 1.0);
        clip.timelines.push(Timeline::BoneTranslate {
            bone,
            axis: Axis::X,
            keys: vec![Key::linear(0.0, 0.0), Key::linear(1.0, 10.0)],
        });
        doc.animations.insert(clip);
        let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
        let dir = tempfile::tempdir().unwrap();
        export(
            &doc,
            &request("move"),
            dir.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 61);
        assert_eq!(
            before,
            ankhimate_formats::to_json(&doc.as_project_ref()).unwrap()
        );
    }
    #[test]
    fn cancelled_render_collision_and_write_failure_preserve_prior_outputs() {
        let doc = walker();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("previous.txt"), b"keep").unwrap();
        let cancel = AtomicBool::new(false);
        assert!(
            export(
                &doc,
                &request("walk"),
                dir.path(),
                &cancel,
                |phase, done, _| {
                    if phase == "Rendering" && done >= 2 {
                        cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            )
            .is_err()
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        assert!(
            export(
                &doc,
                &request("walk"),
                &dir.path().join("previous.txt"),
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        std::fs::write(dir.path().join(frame_name(0)), b"old frame").unwrap();
        assert!(
            export(
                &doc,
                &request("walk"),
                dir.path(),
                &AtomicBool::new(false),
                |_, _, _| {}
            )
            .is_err()
        );
        assert_eq!(
            std::fs::read(dir.path().join(frame_name(0))).unwrap(),
            b"old frame"
        );
        assert_eq!(
            std::fs::read(dir.path().join("previous.txt")).unwrap(),
            b"keep"
        );
    }
}

#[cfg(test)]
mod image_format_tests {
    use super::*;
    fn walker() -> Document {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
        Document::from_loaded(ankhimate_formats::load(&path).unwrap().0)
    }
    fn request() -> SequenceRequest {
        SequenceRequest {
            animation: "walk".into(),
            fps: 60,
            width: 64,
            height: 64,
        }
    }
    #[test]
    fn jpeg_sequence_is_available_with_matching_names_and_bytes() {
        let doc = walker();
        let dir = tempfile::tempdir().unwrap();
        export_encoded(
            &doc,
            &request(),
            FrameEncoding::Jpeg {
                background: [255; 3],
            },
            dir.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        let first = dir.path().join("frame_000000.jpg");
        assert!(
            first.is_file(),
            "JPEG output unavailable: current sequence export writes only PNG"
        );
        assert_eq!(
            image::guess_format(&std::fs::read(first).unwrap()).unwrap(),
            image::ImageFormat::Jpeg
        );
    }

    fn metadata(path: &Path) -> serde_json::Value {
        serde_json::from_slice(&std::fs::read(path.join("sequence.json")).unwrap()).unwrap()
    }
    #[test]
    fn webp_sixty_frames_equal_png_rgba_and_repeated_exports_are_identical() {
        let doc = walker();
        let req = request();
        let png = tempfile::tempdir().unwrap();
        let webp = tempfile::tempdir().unwrap();
        let repeated = tempfile::tempdir().unwrap();
        export(
            &doc,
            &req,
            png.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        for dest in [webp.path(), repeated.path()] {
            assert_eq!(
                export_encoded(
                    &doc,
                    &req,
                    FrameEncoding::WebpLossless,
                    dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .unwrap(),
                60
            );
            assert_eq!(std::fs::read_dir(dest).unwrap().count(), 61);
        }
        for i in 0..60 {
            let b =
                std::fs::read(webp.path().join(FrameEncoding::WebpLossless.frame_name(i))).unwrap();
            assert_eq!(
                b,
                std::fs::read(
                    repeated
                        .path()
                        .join(FrameEncoding::WebpLossless.frame_name(i))
                )
                .unwrap()
            );
            let expected = image::open(png.path().join(frame_name(i)))
                .unwrap()
                .to_rgba8();
            let actual = image::load_from_memory(&b).unwrap().to_rgba8();
            assert_eq!(actual, expected, "frame {i}");
            assert_eq!(actual.dimensions(), (64, 64));
            assert_eq!(actual.get_pixel(0, 0).0, [0; 4]);
        }
        let baseline = metadata(png.path());
        let mut actual = metadata(webp.path());
        assert_eq!(actual["filename_pattern"], "frame_%06d.webp");
        actual["filename_pattern"] = baseline["filename_pattern"].clone();
        assert_eq!(
            actual, baseline,
            "same metadata fields/types/version/camera/timing"
        );
    }
    #[test]
    fn jpeg_real_clip_has_explicit_background_and_bounded_loss() {
        let doc = walker();
        let req = request();
        let dest = tempfile::tempdir().unwrap();
        let encoding = FrameEncoding::Jpeg {
            background: [24, 80, 160],
        };
        export_encoded(
            &doc,
            &req,
            encoding,
            dest.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        assert_eq!(std::fs::read_dir(dest.path()).unwrap().count(), 61);
        let sequence = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
        let mut max_mae = 0.0f64;
        let mut max_rmse = 0.0f64;
        for i in 0..60 {
            let b = std::fs::read(dest.path().join(encoding.frame_name(i))).unwrap();
            assert_eq!(image::guess_format(&b).unwrap(), image::ImageFormat::Jpeg);
            let rgb = image::load_from_memory(&b).unwrap().to_rgb8();
            assert_eq!(rgb.dimensions(), (64, 64));
            let expected = sequence.render_rgba(i).unwrap();
            let mut sum = 0.0f64;
            let mut sq = 0.0f64;
            let mut n = 0;
            for (raw, actual) in expected.pixels().zip(rgb.pixels()) {
                for c in 0..3 {
                    let a = u32::from(raw.0[3]);
                    let value = ((u32::from(raw.0[c]) * a
                        + u32::from([24u8, 80, 160][c]) * (255 - a)
                        + 127)
                        / 255) as u8;
                    let error = f64::from(value.abs_diff(actual.0[c]));
                    sum += error;
                    sq += error * error;
                    n += 1;
                }
            }
            let mae = sum / f64::from(n);
            let rmse = (sq / f64::from(n)).sqrt();
            max_mae = max_mae.max(mae);
            max_rmse = max_rmse.max(rmse);
            assert!(
                mae <= 4.0 && rmse <= 12.0,
                "JPEG frame {i}: MAE {mae}, RMSE {rmse}"
            );
            for (actual, bg) in rgb.get_pixel(0, 0).0.into_iter().zip([24u8, 80, 160]) {
                assert!(actual.abs_diff(bg) <= 3);
            }
        }
        eprintln!(
            "JPEG 60 frames: max MAE={max_mae:.6}, max RMSE={max_rmse:.6}; tolerance 4/12 in 8-bit RGB"
        );
        let meta = metadata(dest.path());
        assert_eq!(meta["alpha"], "none");
        assert_eq!(meta["background"], serde_json::json!([24, 80, 160, 255]));
        assert_eq!(meta["filename_pattern"], "frame_%06d.jpg");
        assert_eq!(meta["png_sequence_version"], 1);
        assert_eq!(meta["fps"], 60);
        assert_eq!(meta["frame_count"], 60);
        let png = tempfile::tempdir().unwrap();
        export(
            &doc,
            &req,
            png.path(),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
        let baseline = metadata(png.path());
        let mut normalized = meta;
        for key in ["filename_pattern", "alpha", "background"] {
            normalized[key] = baseline[key].clone();
        }
        assert_eq!(
            normalized, baseline,
            "JPEG retains the metadata schema, timing and camera"
        );
    }
    #[test]
    fn all_formats_cancel_or_refuse_collisions_and_write_errors_without_losing_old_files() {
        let doc = walker();
        for format in [
            FrameEncoding::Png,
            FrameEncoding::WebpLossless,
            FrameEncoding::Jpeg {
                background: [255; 3],
            },
        ] {
            let dir = tempfile::tempdir().unwrap();
            let backup = dir.path().join("old.backup");
            std::fs::write(&backup, b"old valid bytes").unwrap();
            let cancel = AtomicBool::new(false);
            let mut complete = false;
            assert!(
                export_encoded(
                    &doc,
                    &request(),
                    format,
                    dir.path(),
                    &cancel,
                    |phase, done, _| {
                        if phase == "Rendering" && done == 2 {
                            cancel.store(true, Ordering::Relaxed);
                        }
                        if phase == "Complete" {
                            complete = true;
                        }
                    }
                )
                .is_err()
            );
            assert!(!complete);
            assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
            assert_eq!(std::fs::read(&backup).unwrap(), b"old valid bytes");
            let collision = dir.path().join(format.frame_name(0));
            std::fs::write(&collision, b"previous image").unwrap();
            assert!(
                export_encoded(
                    &doc,
                    &request(),
                    format,
                    dir.path(),
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .is_err()
            );
            assert_eq!(std::fs::read(&collision).unwrap(), b"previous image");
            assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
            assert!(
                export_encoded(
                    &doc,
                    &request(),
                    format,
                    &backup,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .is_err()
            );
            assert_eq!(std::fs::read(backup).unwrap(), b"old valid bytes");
        }
    }
    #[test]
    fn unicode_directories_and_clip_names_work_without_changing_project_data() {
        let mut doc = walker();
        let clip = doc
            .animations
            .values_mut()
            .find(|a| a.name == "walk")
            .unwrap();
        clip.name = "Marche royale — 蛙".into();
        let mut req = request();
        req.animation = "Marche royale — 蛙".into();
        req.fps = 4;
        let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
        let dir = tempfile::tempdir().unwrap();
        for format in [
            FrameEncoding::Png,
            FrameEncoding::WebpLossless,
            FrameEncoding::Jpeg {
                background: [240, 230, 220],
            },
        ] {
            let dest = dir.path().join(format!("Égypte-🐸-{}", format.extension()));
            assert_eq!(
                export_encoded(
                    &doc,
                    &req,
                    format,
                    &dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .unwrap(),
                4
            );
            assert_eq!(metadata(&dest)["animation"], req.animation);
        }
        assert_eq!(
            ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
            before
        );
    }
    #[test]
    fn invalid_parameters_and_non_normal_blending_are_refused_in_every_format() {
        for format in [
            FrameEncoding::Png,
            FrameEncoding::WebpLossless,
            FrameEncoding::Jpeg {
                background: [255; 3],
            },
        ] {
            let mut doc = walker();
            let dir = tempfile::tempdir().unwrap();
            let dest = dir.path().join("not-created");
            let mut req = request();
            req.fps = 0;
            assert!(
                export_encoded(
                    &doc,
                    &req,
                    format,
                    &dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .is_err()
            );
            let slot = doc.skeleton.slots.keys().next().unwrap();
            doc.skeleton.slots[slot].blend_mode = ankhimate_core::slot::BlendMode::Additive;
            assert!(
                export_encoded(
                    &doc,
                    &request(),
                    format,
                    &dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .is_err()
            );
            assert!(!dest.exists());
        }
    }
    #[test]
    fn physics_and_missing_assets_are_refused_before_any_output_in_all_formats() {
        for format in [
            FrameEncoding::Png,
            FrameEncoding::WebpLossless,
            FrameEncoding::Jpeg {
                background: [255; 3],
            },
        ] {
            let mut doc = walker();
            let dir = tempfile::tempdir().unwrap();
            let dest = dir.path().join("not-created");
            let bone = doc.skeleton.update_order[0];
            doc.skeleton
                .add_constraint(ankhimate_core::constraints::Constraint::Physics(
                    ankhimate_core::constraints::PhysicsConstraint::sway("sway", bone),
                ));
            assert!(
                export_encoded(
                    &doc,
                    &request(),
                    format,
                    &dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .unwrap_err()
                .contains("physics")
            );
            assert!(!dest.exists());
            let mut missing = walker();
            missing.assets.images.clear();
            assert!(
                export_encoded(
                    &missing,
                    &request(),
                    format,
                    &dest,
                    &AtomicBool::new(false),
                    |_, _, _| {}
                )
                .is_err()
            );
            assert!(!dest.exists());
        }
    }
}
