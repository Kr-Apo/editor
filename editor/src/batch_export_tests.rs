use crate::batch_export::{Batch, Settings};
use crate::ui::png_sequence::PngUiState;
use ankhimate_core::animation::Animation;
use ankhimate_document::Document;
use ankhimate_render::{encoding::FrameEncoding, framing::FrameOptions};
#[test]
fn batch_one_operation_publishes_two_clips() {
    let mut doc = Document::new();
    doc.animations.insert(Animation::new("first", 1.0));
    doc.animations.insert(Animation::new("second", 2.0));
    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("batch");
    let mut png = PngUiState::default();
    let settings = Settings {
        fps: 4,
        width: 8,
        height: 8,
        encoding: FrameEncoding::Png,
        framing: FrameOptions::default(),
        max_page: None,
    };
    let job = Batch::new(&doc, &doc.animations.keys().collect::<Vec<_>>(), settings).unwrap();
    png.start_batch(doc, job, dest.clone(), eframe::egui::Context::default());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while png.is_running() {
        png.poll();
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    assert!(png.message.as_ref().unwrap().is_ok());
    assert!(
        dest.join("batch.json").is_file(),
        "A single user operation cannot publish two independent clips: batch manifest missing"
    );
}

use crate::{png_export, spritesheet_export};
use ankhimate_render::framing::BoundsMode;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
fn settings() -> Settings {
    Settings {
        fps: 4,
        width: 24,
        height: 20,
        encoding: FrameEncoding::Png,
        framing: FrameOptions::default(),
        max_page: None,
    }
}
fn walker() -> Document {
    Document::from_loaded(
        ankhimate_formats::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh"),
        )
        .unwrap()
        .0,
    )
}
fn clips() -> Document {
    let mut doc = walker();
    let walk = doc
        .animations
        .values()
        .find(|a| a.name == "walk")
        .unwrap()
        .clone();
    doc.animations
        .values_mut()
        .find(|a| a.name == "leg_ik")
        .unwrap()
        .duration = 2.0;
    let mut short = walk;
    short.name = "court 蛙".into();
    short.duration = 0.25;
    doc.animations.insert(short);
    doc
}
fn metadata(root: &Path, name: &str) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(root.join(name)).unwrap()).unwrap()
}
fn equal_tree(actual: &Path, expected: &Path) {
    for entry in std::fs::read_dir(expected).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            assert_eq!(
                std::fs::read(actual.join(p.file_name().unwrap())).unwrap(),
                std::fs::read(&p).unwrap(),
                "{}",
                p.display()
            );
        }
    }
    assert_eq!(
        std::fs::read_dir(actual).unwrap().count(),
        std::fs::read_dir(expected).unwrap().count()
    );
}
#[test]
fn batch_single_clip_preserves_legacy_metadata_and_pixels() {
    let doc = walker();
    let id = doc
        .animations
        .iter()
        .find(|(_, a)| a.name == "walk")
        .unwrap()
        .0;
    let temp = tempfile::tempdir().unwrap();
    let batch = Batch::new(&doc, &[id], settings()).unwrap();
    batch
        .export(
            &doc,
            &temp.path().join("batch"),
            &AtomicBool::new(false),
            |_, _, _| {},
        )
        .unwrap();
    png_export::export(
        &doc,
        &settings().request("walk"),
        &temp.path().join("single"),
        &AtomicBool::new(false),
        |_, _, _| {},
    )
    .unwrap();
    equal_tree(
        &temp.path().join("batch").join(&batch.clips[0].folder),
        &temp.path().join("single"),
    );
    assert!(!PngUiState::default().batch);
}
#[test]
fn batch_three_real_clips_keep_independent_counts_cameras_and_reconstruction_in_all_formats() {
    let doc = clips();
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    let ids = doc.animations.keys().collect::<Vec<_>>();
    for (encoding, max_page) in [
        (FrameEncoding::Png, None),
        (FrameEncoding::WebpLossless, None),
        (
            FrameEncoding::Jpeg {
                background: [24, 80, 160],
            },
            None,
        ),
        (FrameEncoding::Png, Some(64)),
    ] {
        let config = Settings {
            encoding,
            max_page,
            framing: FrameOptions {
                bounds: BoundsMode::GlobalPixels,
                trim: true,
                padding: 1,
            },
            ..settings()
        };
        let batch = Batch::new(&doc, &ids, config).unwrap();
        assert_eq!(batch.total_frames, 13);
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("batch");
        let mut complete = false;
        assert_eq!(
            batch
                .export(&doc, &output, &AtomicBool::new(false), |phase, _, _| {
                    if phase == "Complete" {
                        complete = true;
                    }
                })
                .unwrap(),
            13
        );
        assert!(complete);
        let manifest = metadata(&output, "batch.json");
        assert_eq!(manifest["total_frames"], 13);
        assert_eq!(manifest["clips"].as_array().unwrap().len(), 3);
        let mut cameras = vec![];
        for (i, clip) in batch.clips.iter().enumerate() {
            let standalone = temp.path().join(format!("single-{i}"));
            let req = config.request(&clip.name);
            if let Some(page) = max_page {
                spritesheet_export::export_options(
                    &doc,
                    &req,
                    page,
                    config.framing,
                    &standalone,
                    &AtomicBool::new(false),
                    |_, _, _| {},
                )
                .unwrap();
            } else {
                png_export::export_options(
                    &doc,
                    &req,
                    encoding,
                    config.framing,
                    &standalone,
                    &AtomicBool::new(false),
                    |_, _, _| {},
                )
                .unwrap();
            }
            let child = output.join(&clip.folder);
            equal_tree(&child, &standalone);
            let name = if max_page.is_some() {
                "spritesheet.json"
            } else {
                "sequence.json"
            };
            let meta = metadata(&child, name);
            assert_eq!(meta["frame_count"], clip.frame_count);
            assert_eq!(meta["duration"], f64::from(clip.duration));
            assert_eq!(meta["fps"], 4);
            assert_eq!(meta["sampling"], "i / fps; end exclusive");
            assert_eq!(
                meta["reconstruction"]["frames"].as_array().unwrap().len(),
                clip.frame_count as usize
            );
            if clip.name == "leg_ik" && max_page.is_some() {
                assert_eq!(meta["pages"].as_array().unwrap().len(), 2);
            }
            assert_eq!(manifest["clips"][i]["animation"], clip.name);
            assert_eq!(manifest["clips"][i]["frame_count"], clip.frame_count);
            cameras.push(meta["camera"].clone());
        }
        assert!(
            cameras.windows(2).any(|a| a[0] != a[1]),
            "each clip must have its own camera"
        );
    }
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn batch_two_clips_have_exact_fractional_frame_counts_and_no_selection_order_dependency() {
    let mut doc = Document::new();
    let first = doc.animations.insert(Animation::new("A", 0.1));
    let second = doc.animations.insert(Animation::new("B", 0.25));
    let config = Settings {
        fps: 30,
        ..settings()
    };
    let a = Batch::new(&doc, &[first, second], config).unwrap();
    let b = Batch::new(&doc, &[second, first], config).unwrap();
    assert_eq!(
        a.clips.iter().map(|c| c.frame_count).collect::<Vec<_>>(),
        vec![3, 8]
    );
    assert_eq!(a.total_frames, 11);
    assert_eq!(
        a.plan
            .files
            .iter()
            .find(|f| f.path == "batch.json")
            .unwrap()
            .contents,
        b.plan
            .files
            .iter()
            .find(|f| f.path == "batch.json")
            .unwrap()
            .contents
    );
    let temp = tempfile::tempdir().unwrap();
    a.export(
        &doc,
        &temp.path().join("two"),
        &AtomicBool::new(false),
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(
        metadata(&temp.path().join("two"), "batch.json")["total_frames"],
        11
    );
}
#[test]
fn batch_names_are_portable_bounded_unique_and_preserve_original_names_and_duplicate_clips() {
    let mut doc = Document::new();
    let mut names = vec![
        "walk".to_string(),
        "walk".into(),
        "a/b".into(),
        "a\\b".into(),
        "A:B".into(),
        "a|b".into(),
        "CON".into(),
        "COM¹.txt".into(),
        "NUL. ".into(),
        "..".into(),
        "  ".into(),
        "Égyptien 蛙 royal".into(),
        "é".into(),
        "e\u{301}".into(),
        "quote\"\n".into(),
    ];
    names.push("蛙".repeat(300));
    for (i, name) in names.iter().enumerate() {
        doc.animations
            .insert(Animation::new(name, if i == 1 { 2.0 } else { 1.0 }));
    }
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    let batch = Batch::new(
        &doc,
        &doc.animations.keys().collect::<Vec<_>>(),
        Settings {
            width: 8,
            height: 8,
            ..settings()
        },
    )
    .unwrap();
    let mut folders = std::collections::BTreeSet::new();
    for clip in &batch.clips {
        assert!(clip.folder.encode_utf16().count() <= 80);
        assert!(folders.insert(clip.folder.to_lowercase()));
        ankhimate_export::batch::folder_name(&clip.folder).unwrap();
    }
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("Batch 雪");
    batch
        .export(&doc, &output, &AtomicBool::new(false), |_, _, _| {})
        .unwrap();
    let manifest = metadata(&output, "batch.json");
    for (i, name) in names.iter().enumerate() {
        assert_eq!(manifest["clips"][i]["animation"], *name);
        let meta = metadata(&output.join(&batch.clips[i].folder), "sequence.json");
        assert_eq!(meta["animation"], *name);
        assert_eq!(meta["frame_count"], if i == 1 { 8 } else { 4 });
    }
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn batch_rejects_empty_invalid_stale_duplicate_selections_and_physics() {
    let mut doc = walker();
    let ids = doc.animations.keys().collect::<Vec<_>>();
    assert!(Batch::new(&doc, &[], settings()).is_err());
    assert!(Batch::new(&doc, &[ids[0], ids[0]], settings()).is_err());
    let mut invalid = settings();
    invalid.fps = 0;
    assert!(Batch::new(&doc, &ids, invalid).is_err());
    invalid = settings();
    invalid.width = 0;
    assert!(Batch::new(&doc, &ids, invalid).is_err());
    let gone = doc.animations.insert(Animation::new("gone", 1.0));
    doc.animations.remove(gone);
    assert!(Batch::new(&doc, &[gone], settings()).is_err());
    let bad = doc.animations.insert(Animation::new("bad", 0.0));
    assert!(Batch::new(&doc, &[bad], settings()).is_err());
    let bone = doc.skeleton.update_order[0];
    doc.skeleton
        .add_constraint(ankhimate_core::constraints::Constraint::Physics(
            ankhimate_core::constraints::PhysicsConstraint::sway("sway", bone),
        ));
    assert!(Batch::new(&doc, &ids, settings()).is_err());
}
#[test]
fn batch_errors_on_second_or_third_clip_never_publish_and_clean_staging() {
    let doc = clips();
    let batch = Batch::new(&doc, &doc.animations.keys().collect::<Vec<_>>(), settings()).unwrap();
    let before = ankhimate_formats::to_json(&doc.as_project_ref()).unwrap();
    for fail in [1usize, 2] {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("prior.backup"), b"keep").unwrap();
        let output = temp.path().join("batch");
        let threshold = batch.clips[..fail]
            .iter()
            .map(|c| u64::from(c.frame_count))
            .sum::<u64>();
        let mut injected = false;
        let mut completed = false;
        let error = batch
            .export(&doc, &output, &AtomicBool::new(false), |phase, done, _| {
                if phase == "Complete" {
                    completed = true;
                }
                if phase == "Clip complete" && done == threshold && !injected {
                    let stage = std::fs::read_dir(temp.path())
                        .unwrap()
                        .map(|e| e.unwrap().path())
                        .find(|p| {
                            p.file_name()
                                .unwrap()
                                .to_string_lossy()
                                .starts_with(".ankhimate-batch-")
                        })
                        .unwrap();
                    std::fs::write(
                        stage.join(&batch.clips[fail].folder),
                        b"simulated write obstruction",
                    )
                    .unwrap();
                    injected = true;
                }
            })
            .unwrap_err();
        assert!(injected && !completed);
        assert!(error.contains(&batch.clips[fail].folder), "{error}");
        assert!(!output.exists());
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
        assert_eq!(
            std::fs::read(temp.path().join("prior.backup")).unwrap(),
            b"keep"
        );
    }
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn batch_cancellation_during_render_and_destination_race_preserve_existing_files() {
    let doc = clips();
    let batch = Batch::new(
        &doc,
        &doc.animations.keys().collect::<Vec<_>>(),
        Settings {
            encoding: FrameEncoding::WebpLossless,
            framing: FrameOptions {
                trim: true,
                padding: 1,
                bounds: BoundsMode::GlobalPixels,
            },
            ..settings()
        },
    )
    .unwrap();
    for threshold in [1, 5] {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("batch");
        let cancel = AtomicBool::new(false);
        let mut complete = false;
        assert!(
            batch
                .export(&doc, &output, &cancel, |phase, done, _| {
                    if phase.contains("Rendering") && done >= threshold {
                        cancel.store(true, Ordering::Relaxed);
                    }
                    if phase == "Complete" {
                        complete = true;
                    }
                })
                .is_err()
        );
        assert!(!complete);
        assert!(!output.exists());
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 0);
    }
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("batch");
    ankhimate_export::batch::inspect(&batch.plan, &output).unwrap();
    let mut appeared = false;
    assert!(
        batch
            .export(&doc, &output, &AtomicBool::new(false), |phase, _, _| {
                if phase == "Clip complete" && !appeared {
                    std::fs::create_dir(&output).unwrap();
                    std::fs::write(output.join("old.txt"), b"keep").unwrap();
                    appeared = true;
                }
            })
            .is_err()
    );
    assert!(appeared);
    assert_eq!(std::fs::read(output.join("old.txt")).unwrap(), b"keep");
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
}
