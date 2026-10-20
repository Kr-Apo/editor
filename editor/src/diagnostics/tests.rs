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
fn scan(s: &mut AppState, ms: u64) {
    s.diagnostics
        .poll(&s.doc, s.revision, Duration::from_millis(ms));
}
fn ticket(s: &AppState, code: &str) -> Ticket {
    s.diagnostics
        .ticket(
            s.diagnostics
                .issues
                .iter()
                .position(|i| i.code == code)
                .expect(code),
        )
        .unwrap()
}
#[test]
fn t818_first_scan_and_unchanged_frames_are_cached() {
    let mut s = AppState::default();
    scan(&mut s, 0);
    assert_eq!(s.diagnostics.scans, 1);
    for i in 1..100 {
        s.session.camera.zoom = 1.0 + i as f32;
        scan(&mut s, i);
    }
    assert_eq!(s.diagnostics.scans, 1);
}
#[test]
fn t818_commands_undo_redo_are_coalesced_and_stale_immediately() {
    let mut s = AppState::default();
    scan(&mut s, 0);
    s.dispatch(Box::new(CreateBone::new(bone("a"))));
    scan(&mut s, 10);
    assert!(s.diagnostics.stale());
    assert!(s.diagnostics.ticket(0).is_none());
    s.undo();
    scan(&mut s, 80);
    s.redo();
    scan(&mut s, 110);
    scan(&mut s, 309);
    assert_eq!(s.diagnostics.scans, 1);
    scan(&mut s, 310);
    assert_eq!(s.diagnostics.scans, 2);
    assert!(!s.diagnostics.stale());
}
#[test]
fn t818_direct_metadata_is_observed_without_full_frame_snapshots() {
    let mut s = AppState::default();
    scan(&mut s, 0);
    s.doc.meta.fps = 0;
    scan(&mut s, 1);
    assert!(s.diagnostics.stale());
    scan(&mut s, 201);
    assert!(s.diagnostics.issues.iter().any(|i| i.code == "invalid-fps"));
    assert_eq!(s.revision, 0);
}
#[test]
fn t818_old_ticket_refused_after_delete_undo_and_document_swap() {
    let mut s = AppState::default();
    let id = s.doc.skeleton.add_bone(bone("victim"));
    let slot = s
        .doc
        .skeleton
        .add_slot(ankhimate_core::slot::Slot::new("slot".into(), id));
    s.doc.skeleton.slots[slot].attachment = Some("missing".into());
    scan(&mut s, 0);
    let t = ticket(&s, "unknown-attachment");
    s.dispatch(Box::new(DeleteBone::new(id)));
    s.undo();
    let selection = s.session.selection.clone();
    assert!(!navigate(&mut s, &t));
    assert_eq!(s.session.selection, selection);
    s.replace_document(Document::new());
    assert!(!navigate(&mut s, &t));
    assert!(s.session.selection.is_none());
}
#[test]
fn t818_snapshot_rejects_out_of_band_reference_change_without_revision() {
    let mut s = AppState::default();
    let id = s.doc.skeleton.add_bone(bone("a"));
    let slot = s
        .doc
        .skeleton
        .add_slot(ankhimate_core::slot::Slot::new("slot".into(), id));
    s.doc.skeleton.slots[slot].attachment = Some("missing".into());
    scan(&mut s, 0);
    let t = ticket(&s, "unknown-attachment");
    s.doc.skeleton.slots[slot].name = "changed".into();
    assert!(!navigate(&mut s, &t));
    assert!(s.session.selection.is_none());
    scan(&mut s, 1);
    assert_eq!(s.diagnostics.scans, 2);
    assert!(!s.diagnostics.stale());
}
#[test]
fn t818_key_navigation_uses_shared_selection_without_authoring() {
    let mut s = AppState::default();
    let id = s.doc.skeleton.add_bone(bone("a"));
    let mut a = Animation::new("clip", 1.0);
    a.timelines.push(Timeline::BoneRotate {
        bone: id,
        keys: vec![Key {
            time: 2.0,
            value: 30.0,
            interp: Interp::Linear,
        }],
    });
    let clip = s.doc.animations.insert(a);
    scan(&mut s, 0);
    let t = ticket(&s, "key-outside-clip");
    let before = snapshot(&s.doc);
    let history = s.history.undo_depth();
    let revision = s.revision;
    assert!(navigate(&mut s, &t));
    assert_eq!(s.session.active_animation, Some(clip));
    assert_eq!(s.session.playhead, 2.0);
    assert_eq!(crate::key_selection::read(&mut s, clip).len(), 1);
    assert!(before == snapshot(&s.doc));
    assert_eq!(history, s.history.undo_depth());
    assert_eq!(revision, s.revision);
}
#[test]
fn t818_pending_preview_navigation_is_refused_without_losing_preview() {
    let mut s = AppState::default();
    let id = s.doc.skeleton.add_bone(bone("a"));
    scan(&mut s, 0);
    let t = ticket(&s, "empty-skin");
    s.session.set_preview_local(id, Default::default());
    assert!(!navigate(&mut s, &t));
    assert!(s.session.has_previews());
}
#[test]
fn t818_scan_and_badge_panel_are_non_mutating_egui() {
    let mut s = AppState::default();
    s.dispatch(Box::new(CreateBone::new(bone("a"))));
    s.undo();
    let doc = snapshot(&s.doc);
    let history = (s.history.undo_depth(), s.history.can_redo());
    let revision = s.revision;
    scan(&mut s, 0);
    let ctx = egui::Context::default();
    let mut rect = egui::Rect::NOTHING;
    let _ = ctx.run_ui(Default::default(), |ui| {
        rect = badge(ui, &mut s).rect;
    });
    let pos = rect.center();
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
            badge(ui, &mut s);
        });
    }
    assert!(s.diagnostics.open);
    let _ = ctx.run_ui(Default::default(), |ui| {
        list(ui, &mut s);
    });
    assert!(doc == snapshot(&s.doc));
    assert_eq!(history, (s.history.undo_depth(), s.history.can_redo()));
    assert_eq!(revision, s.revision);
    assert!(s.session.selection.is_none());
    assert!(!s.session.has_previews());
}
#[test]
fn t818_500_bones_deterministic_cpu_measurement_and_scan_counts() {
    let mut s = AppState::default();
    let mut parent = None;
    for i in 0..500 {
        let mut b = bone(&format!("bone-{i:03}"));
        b.parent = parent;
        parent = Some(s.doc.skeleton.add_bone(b));
    }
    let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    scan(&mut s, 0);
    let issues = s.diagnostics.issues.clone();
    let elapsed = s.diagnostics.last_elapsed;
    for i in 1..500 {
        scan(&mut s, i);
    }
    assert_eq!(s.diagnostics.scans, 1);
    assert_eq!(issues, diagnostics::inspect(&s.doc));
    s.dispatch(Box::new(CreateBone::new(bone("extra"))));
    scan(&mut s, 600);
    scan(&mut s, 800);
    assert_eq!(s.diagnostics.scans, 2);
    s.undo();
    scan(&mut s, 900);
    scan(&mut s, 1100);
    assert_eq!(s.diagnostics.scans, 3);
    assert_eq!(
        before,
        ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
    );
    s.redo();
    scan(&mut s, 1200);
    scan(&mut s, 1400);
    assert_eq!(s.diagnostics.scans, 4);
    println!(
        "T818_PERF bones=500 issues={} scan_us={} unchanged_frames=499 extra_scans=0 command_undo_redo_scans=3",
        issues.len(),
        elapsed.as_micros()
    );
}
#[test]
fn t818_egui_row_click_navigates_and_badge_can_close() {
    let mut s = AppState::default();
    scan(&mut s, 0);
    let ctx = egui::Context::default();
    let mut rect = None;
    let _ = ctx.run_ui(Default::default(), |ui| {
        rect = list(ui, &mut s);
    });
    let pos = rect.unwrap().center();
    let before = snapshot(&s.doc);
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
            list(ui, &mut s);
        });
    }
    assert_eq!(s.session.focus_tab, Some(Tab::Skins));
    assert!(before == snapshot(&s.doc));
    assert!(!s.history.can_undo());
    s.diagnostics.open = true;
    let _ = ctx.run_ui(Default::default(), |ui| {
        rect = Some(badge(ui, &mut s).rect);
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
            badge(ui, &mut s);
        });
    }
    assert!(!s.diagnostics.open);
    assert!(before == snapshot(&s.doc));
}
#[test]
fn t818_navigation_invalidates_geometry_and_does_not_retarget_old_keys() {
    let mut s = AppState::default();
    let id = s.doc.skeleton.add_bone(bone("root"));
    let slot = s
        .doc
        .skeleton
        .add_slot(ankhimate_core::slot::Slot::new("slot".into(), id));
    s.doc.skeleton.slots[slot].attachment = Some("absent".into());
    scan(&mut s, 0);
    s.session.selected_vertices = vec![1, 2];
    let epoch = s.session.geometry_epoch;
    let t = ticket(&s, "unknown-attachment");
    assert!(navigate(&mut s, &t));
    assert!(s.session.selected_vertices.is_empty());
    assert!(s.session.geometry_epoch > epoch);
    assert_eq!(s.session.active_slot(), Some(slot));
}
#[test]
fn t818_unchanged_samples_save_reload_and_native_export() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
    let mut s = AppState::default();
    s.replace_document(Document::from_loaded(
        ankhimate_formats::load(&path).unwrap().0,
    ));
    let before = snapshot(&s.doc);
    let schema = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let preset = ankhimate_export::presets::default_preset();
    let plan = ankhimate_export::run::plan(&schema, &s.doc.assets, &preset).unwrap();
    scan(&mut s, 0);
    assert!(
        !s.diagnostics
            .issues
            .iter()
            .any(|i| i.severity == diagnostics::Severity::Error),
        "{:?}",
        s.diagnostics.issues
    );
    let _ = egui::Context::default().run_ui(Default::default(), |ui| {
        list(ui, &mut s);
    });
    assert!(before == snapshot(&s.doc));
    let after = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    let temp = tempfile::tempdir().unwrap();
    let a = temp.path().join("a");
    let b = temp.path().join("b");
    ankhimate_export::run::write(&plan, &a).unwrap();
    ankhimate_export::run::write(&after, &b).unwrap();
    fn files(dir: &std::path::Path) -> Vec<(std::path::PathBuf, Vec<u8>)> {
        fn walk(
            root: &std::path::Path,
            dir: &std::path::Path,
            out: &mut Vec<(std::path::PathBuf, Vec<u8>)>,
        ) {
            for e in std::fs::read_dir(dir).unwrap() {
                let p = e.unwrap().path();
                if p.is_dir() {
                    walk(root, &p, out);
                } else {
                    out.push((
                        p.strip_prefix(root).unwrap().to_path_buf(),
                        std::fs::read(p).unwrap(),
                    ));
                }
            }
        }
        let mut out = Vec::new();
        walk(dir, dir, &mut out);
        out.sort();
        out
    }
    assert_eq!(files(&a), files(&b));
    let saved = temp.path().join("copy.ankh");
    ankhimate_formats::save(&saved, &s.doc.as_project_ref(), &[]).unwrap();
    let reopened = Document::from_loaded(ankhimate_formats::load(&saved).unwrap().0);
    assert_eq!(
        ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap()
    );
    assert!(
        !ankhimate_formats::to_json(&reopened.as_project_ref())
            .unwrap()
            .contains("diagnostics")
    );
}
#[test]
fn t818_500_bones_200_slots_and_asset_cost_is_inspection_only() {
    let mut s = AppState::default();
    let mut parent = None;
    for i in 0..500 {
        let mut b = bone(&format!("b-{i:03}"));
        b.parent = parent;
        parent = Some(s.doc.skeleton.add_bone(b));
    }
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::new_rgba8(16, 16)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    for i in 0..33 {
        s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
            format!("asset-{i}"),
            bytes.get_ref().clone(),
            16,
            16,
        ));
    }
    let skin = s.doc.skeleton.default_skin;
    for i in 0..200 {
        let slot = s.doc.skeleton.add_slot(ankhimate_core::slot::Slot::new(
            format!("slot-{i:03}"),
            parent.unwrap(),
        ));
        s.doc.skeleton.slots[slot].attachment = Some("mesh".into());
        s.doc.skeleton.skins[skin].set(
            slot,
            "mesh",
            ankhimate_core::attachment::Attachment::Mesh(
                ankhimate_core::attachment::MeshAttachment {
                    texture: "asset-0".into(),
                    setup_vertices: vec![glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y],
                    uvs: vec![glam::Vec2::ZERO; 3],
                    triangles: vec![[0, 1, 2]],
                    ..Default::default()
                },
            ),
        );
    }
    let before = snapshot(&s.doc);
    scan(&mut s, 0);
    let initial = s.diagnostics.issues.clone();
    assert_eq!(initial.len(), 32);
    assert!(initial.iter().all(|i| i.code == "unused-image"));
    for i in 1..100 {
        scan(&mut s, i);
    }
    assert_eq!(s.diagnostics.scans, 1);
    assert!(before == snapshot(&s.doc));
    assert_eq!(initial, diagnostics::inspect(&s.doc));
    println!(
        "T818_PERF bones=500 slots=200 images=33 issues={} scan_us={} unchanged_frames=99 extra_scans=0",
        initial.len(),
        s.diagnostics.last_elapsed.as_micros()
    );
}
