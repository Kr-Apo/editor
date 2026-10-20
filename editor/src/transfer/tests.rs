use super::*;
use ankhimate_core::{
    animation::{Animation, Interp, Key, Timeline},
    skeleton::Bone,
};
use ankhimate_document::commands::bone_cmds::{CreateBone, DeleteBone};
fn bone(name: &str) -> Bone {
    Bone {
        name: name.into(),
        parent: None,
        length: 10.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}
fn source_doc() -> Document {
    use ankhimate_document::EditCommand;
    let mut d = Document::new();
    d.meta.name = "Source-A".into();
    let root = d.skeleton.add_bone(bone("source"));
    let mut child = bone("hand");
    child.parent = Some(root);
    child.local_transform.position = glam::vec2(4.0, 0.0);
    let hand = d.skeleton.add_bone(child);
    for (n, bone) in [root, hand].into_iter().enumerate() {
        let mut bytes = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(2, 2, image::Rgba([200, n as u8 * 150, 40, 255]))
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        ankhimate_document::commands::asset_cmds::ImportImage::new(
            ankhimate_core::assets::ImageAsset::new(format!("image-{n}"), bytes.into_inner(), 2, 2),
            bone,
            glam::Vec2::ZERO,
        )
        .apply(&mut d);
    }
    let curve = Interp::Bezier {
        out_handle: glam::vec2(0.2, -0.2),
        in_handle: glam::vec2(0.8, 1.2),
    };
    let mut a = Animation::new("move", 1.0);
    a.timelines = vec![
        Timeline::BoneRotate {
            bone: hand,
            keys: vec![
                Key {
                    time: 0.0,
                    value: 0.0,
                    interp: Interp::Linear,
                },
                Key {
                    time: 1.0,
                    value: 40.0,
                    interp: curve,
                },
            ],
        },
        Timeline::BoneTranslate {
            bone: root,
            axis: ankhimate_core::animation::Axis::X,
            keys: vec![
                Key {
                    time: 0.0,
                    value: 0.0,
                    interp: Interp::Linear,
                },
                Key {
                    time: 0.75,
                    value: 8.0,
                    interp: curve,
                },
            ],
        },
    ];
    d.animations.insert(a);
    d
}

fn loaded(s: &mut AppState) -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("source.ankh");
    let doc = source_doc();
    ankhimate_formats::save(&path, &doc.as_project_ref(), &[]).unwrap();
    let source = Source::load(&path).unwrap();
    let root = source.doc.skeleton.update_order[0];
    s.transfer = Panel {
        open: true,
        source: Some(source),
        root: Some(root),
        ..Default::default()
    };
    temp
}
#[test]
fn t819_review_and_cancel_preserve_destination_clipboard_and_redo() {
    let mut s = AppState::default();
    s.dispatch(Box::new(CreateBone::new(bone("old"))));
    s.undo();
    let _tmp = loaded(&mut s);
    let before = snapshot(&s.doc);
    prepare(&mut s).unwrap();
    assert!(before == snapshot(&s.doc));
    assert!(s.history.can_redo());
    cancel(&mut s);
    assert!(before == snapshot(&s.doc));
    assert!(s.history.can_redo());
    assert!(s.session.selection.is_none());
}
#[test]
fn t819_approval_once_native_history_and_diagnostics_invalidation() {
    let mut s = AppState::default();
    let _tmp = loaded(&mut s);
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::ZERO);
    let before = snapshot(&s.doc);
    prepare(&mut s).unwrap();
    approve(&mut s).unwrap();
    assert_eq!(s.history.undo_depth(), 1);
    assert!(s.session.active_bone().is_some());
    let after = snapshot(&s.doc);
    assert!(approve(&mut s).is_err());
    for _ in 0..3 {
        s.undo();
        assert!(before == snapshot(&s.doc));
        assert!(s.session.active_bone().is_none());
        s.redo();
        assert!(after == snapshot(&s.doc));
    }
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::from_millis(1));
    assert!(s.diagnostics.stale());
}
#[test]
fn t819_stale_revision_metadata_and_other_project_refuse() {
    for mode in 0..3 {
        let mut s = AppState::default();
        let _tmp = loaded(&mut s);
        prepare(&mut s).unwrap();
        match mode {
            0 => {
                s.dispatch(Box::new(CreateBone::new(bone("new"))));
            }
            1 => s.doc.meta.fps = 24,
            _ => s.replace_document(Document::new()),
        };
        let before = snapshot(&s.doc);
        let depth = s.history.undo_depth();
        assert!(approve(&mut s).is_err());
        assert!(before == snapshot(&s.doc));
        assert_eq!(depth, s.history.undo_depth());
    }
}
#[test]
fn t819_source_file_change_or_disappearance_refuses() {
    for remove in [true, false] {
        let mut s = AppState::default();
        let tmp = loaded(&mut s);
        prepare(&mut s).unwrap();
        let path = tmp.path().join("source.ankh");
        if remove {
            std::fs::remove_file(&path).unwrap();
        } else {
            let mut doc = source_doc();
            doc.meta.name = "changed".into();
            ankhimate_formats::save(&path, &doc.as_project_ref(), &[]).unwrap();
        }
        let before = snapshot(&s.doc);
        assert!(approve(&mut s).is_err());
        assert!(before == snapshot(&s.doc));
        assert!(!s.history.can_undo());
    }
}
#[test]
fn t819_parent_deleted_or_locked_and_animate_mode_refuse() {
    for mode in 0..3 {
        let mut s = AppState::default();
        let parent = s.doc.skeleton.add_bone(bone("parent"));
        let _tmp = loaded(&mut s);
        s.transfer.parent = Some(parent);
        prepare(&mut s).unwrap();
        match mode {
            0 => {
                s.dispatch(Box::new(DeleteBone::new(parent)));
            }
            1 => {
                s.session.locked_bones.insert(parent, true);
            }
            _ => s.set_work_mode(crate::session::WorkMode::Animate),
        }
        let before = snapshot(&s.doc);
        assert!(approve(&mut s).is_err());
        assert!(before == snapshot(&s.doc));
    }
}
#[test]
fn t819_egui_confirm_and_cancel_are_real_actions_in_memory() {
    for confirm in [false, true] {
        let mut s = AppState::default();
        let _tmp = loaded(&mut s);
        prepare(&mut s).unwrap();
        let ctx = egui::Context::default();
        let mut rect = None;
        let _ = ctx.run_ui(Default::default(), |ui| {
            let (a, b) = contents(ui, &mut s);
            rect = Some(if confirm { a.unwrap() } else { b });
        });
        let pos = rect.unwrap().center();
        for pressed in [true, false] {
            let input = egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            };
            let _ = ctx.run_ui(input, |ui| {
                contents(ui, &mut s);
            });
        }
        assert_eq!(
            s.doc.skeleton.bones.len(),
            if confirm {
                source_doc().skeleton.bones.len()
            } else {
                0
            }
        );
        assert_eq!(s.history.undo_depth(), usize::from(confirm));
    }
}
#[test]
fn t819_delayed_source_completion_cannot_target_newer_document() {
    let mut s = AppState::default();
    let tmp = loaded(&mut s);
    let path = tmp.path().join("source.ankh");
    let guard = Guard::capture(&s);
    let (tx, rx) = mpsc::channel();
    s.transfer.loading = Some(Loading {
        guard,
        receiver: rx,
    });
    s.doc.meta.fps = 12;
    tx.send(Source::load(&path)).ok();
    poll(&mut s, &egui::Context::default());
    assert!(
        s.transfer
            .error
            .as_ref()
            .unwrap()
            .contains("Destination changed")
    );
    assert!(!s.history.can_undo());
}
fn destination_doc(source: &Document) -> Document {
    let mut d = Document::new();
    d.meta.name = "Destination".into();
    d.skeleton.add_bone(bone("destination-parent"));
    let mut identical = source.assets.images.values().next().unwrap().clone();
    identical.name = "reusable".into();
    d.assets.add(identical);
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 10, 250, 255]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    d.assets.add(ankhimate_core::assets::ImageAsset::new(
        "image-1",
        bytes.into_inner(),
        2,
        2,
    ));
    d.animations.insert(Animation::new("move", 4.0));
    d
}
#[test]
fn t819_portability_unicode_source_unavailable_save_reload_and_native_v2() {
    let temp = tempfile::tempdir().unwrap();
    let source_dir = temp.path().join("Source été 01");
    let destination_dir = temp.path().join("Destination autonome");
    std::fs::create_dir(&source_dir).unwrap();
    std::fs::create_dir(&destination_dir).unwrap();
    let source = source_doc();
    let path = source_dir.join("Source.ankh");
    ankhimate_formats::save(&path, &source.as_project_ref(), &[]).unwrap();
    let mut s = AppState::default();
    s.replace_document(destination_doc(&source));
    let original_assets: Vec<_> = s
        .doc
        .assets
        .images
        .values()
        .map(|a| (a.name.clone(), a.bytes.clone()))
        .collect();
    let source_loaded = Source::load(&path).unwrap();
    let root = source_loaded.doc.skeleton.update_order[0];
    s.transfer = Panel {
        open: true,
        source: Some(source_loaded),
        root: Some(root),
        parent: s.doc.skeleton.update_order.first().copied(),
        ..Default::default()
    };
    prepare(&mut s).unwrap();
    let summary = s.transfer.review.as_ref().unwrap().plan.summary.clone();
    assert!(summary.images.iter().any(|a| a.reused));
    assert!(
        summary
            .images
            .iter()
            .any(|a| a.source == "image-1" && a.destination == "image-1_2")
    );
    approve(&mut s).unwrap();
    let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    let saved = destination_dir.join("Résultat autonome.ankh");
    ankhimate_formats::save(&saved, &s.doc.as_project_ref(), &[]).unwrap();
    std::fs::rename(
        &source_dir,
        temp.path()
            .join("Source retained but unavailable at original path"),
    )
    .unwrap();
    assert!(!path.exists());
    let reopened = Document::from_loaded(ankhimate_formats::load(&saved).unwrap().0);
    assert_eq!(
        before,
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap()
    );
    for (name, bytes) in original_assets {
        assert_eq!(
            reopened
                .assets
                .get(reopened.assets.by_name(&name).unwrap())
                .unwrap()
                .bytes,
            bytes
        );
    }
    for image in source.assets.images.values() {
        assert!(
            reopened
                .assets
                .images
                .values()
                .any(|a| a.bytes == image.bytes)
        );
    }
    let project = ankhimate_formats::convert::to_schema(&reopened.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &project,
        &reopened.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let out = temp.path().join("native-v2");
    ankhimate_export::run::write(&plan, &out).unwrap();
    let value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out.join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(value["format_version"], 2);
    assert!(
        !ankhimate_document::diagnostics::inspect(&reopened)
            .iter()
            .any(|i| i.severity == ankhimate_document::diagnostics::Severity::Error)
    );
    if let Some(delivery) = std::env::var_os("ANKHIMATE_T819_FIXTURES") {
        let delivery = PathBuf::from(delivery);
        assert!(!delivery.exists());
        std::fs::create_dir_all(&delivery).unwrap();
        for (name, doc) in [
            ("Source-A", &source),
            ("Destination-B", &destination_doc(&source)),
            ("Transferred-Result", &reopened),
        ] {
            let dir = delivery.join(name);
            std::fs::create_dir(&dir).unwrap();
            ankhimate_formats::save(
                &dir.join(format!("{name}.ankh")),
                &doc.as_project_ref(),
                &[],
            )
            .unwrap();
        }
    }
}
#[test]
fn t819_changed_sidecar_image_bytes_refuse_confirmation() {
    let mut s = AppState::default();
    let tmp = loaded(&mut s);
    prepare(&mut s).unwrap();
    fn image(dir: &Path) -> Option<PathBuf> {
        for e in std::fs::read_dir(dir).ok()? {
            let p = e.ok()?.path();
            if p.is_dir() {
                if let Some(found) = image(&p) {
                    return Some(found);
                }
            } else if p.extension().is_some_and(|s| s == "png") {
                return Some(p);
            }
        }
        None
    }
    let path = image(tmp.path()).expect("fixture image");
    std::fs::write(path, b"changed throwaway image").unwrap();
    let before = snapshot(&s.doc);
    assert!(approve(&mut s).is_err());
    assert!(before == snapshot(&s.doc));
    assert!(!s.history.can_undo());
}
#[test]
fn t819_source_load_error_never_replaces_destination() {
    let mut s = AppState::default();
    s.dispatch(Box::new(CreateBone::new(bone("keep"))));
    let before = snapshot(&s.doc);
    let guard = Guard::capture(&s);
    let (tx, rx) = mpsc::channel();
    s.transfer = Panel {
        open: true,
        loading: Some(Loading {
            guard,
            receiver: rx,
        }),
        ..Default::default()
    };
    tx.send(Err("fixture unreadable".into())).unwrap();
    poll(&mut s, &egui::Context::default());
    assert!(before == snapshot(&s.doc));
    assert_eq!(s.history.undo_depth(), 1);
    assert_eq!(s.transfer.error.as_deref(), Some("fixture unreadable"));
}
#[test]
fn t819_rejected_plan_retains_existing_redo_and_selection() {
    let mut s = AppState::default();
    s.dispatch(Box::new(CreateBone::new(bone("old"))));
    s.undo();
    let _tmp = loaded(&mut s);
    prepare(&mut s).unwrap();
    s.doc.meta.fps = 12;
    let selection = s.session.selection.clone();
    assert!(approve(&mut s).is_err());
    assert!(s.history.can_redo());
    assert_eq!(selection, s.session.selection);
}
#[test]
fn t819_old_diagnostic_ticket_refuses_after_transfer_and_undo() {
    let mut s = AppState::default();
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::ZERO);
    let ticket = s.diagnostics.ticket(0).unwrap();
    let _tmp = loaded(&mut s);
    prepare(&mut s).unwrap();
    approve(&mut s).unwrap();
    s.undo();
    assert!(!crate::diagnostics::navigate(&mut s, &ticket));
    assert!(s.session.selection.is_none());
}
#[test]
fn t819_project_reset_discards_old_source_reader_without_replacement() {
    let mut s = AppState::default();
    let tmp = loaded(&mut s);
    let guard = Guard::capture(&s);
    let (tx, rx) = mpsc::channel();
    s.transfer.loading = Some(Loading {
        guard,
        receiver: rx,
    });
    s.replace_document(Document::new());
    assert!(
        tx.send(Source::load(&tmp.path().join("source.ankh")))
            .is_err()
    );
    poll(&mut s, &egui::Context::default());
    assert!(s.transfer.source.is_none());
    assert!(s.doc.skeleton.bones.is_empty());
    assert!(!s.history.can_undo());
}
