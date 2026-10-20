use super::*;
use ankhimate_core::animation::{Animation, Key, Timeline};
use ankhimate_document::{History, snapshot::snapshot};
const ORIGINAL: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../document/tests/fixtures/t822/original.psd"
));
const RENAMED: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../document/tests/fixtures/t822/renamed.psd"
));
const MOVED: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../document/tests/fixtures/t822/moved.psd"
));
fn fixture(source: &[u8]) -> (AppState, PendingPsd, tempfile::TempDir) {
    let mut s = AppState::default();
    let p = PendingPsd::new(
        "hero".into(),
        ORIGINAL.to_vec(),
        ankhimate_formats::psd::layer_tree(ORIGINAL).unwrap(),
    );
    super::super::run_import(&mut s, &p).unwrap();
    let mut a = Animation::new("kept", 3.);
    a.timelines.push(Timeline::BoneRotate {
        bone: s.doc.skeleton.update_order[0],
        keys: vec![Key::linear(0., 0.), Key::stepped(1., 0.5)],
    });
    s.doc.animations.insert(a);
    s.history = History::default();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("renamed.psd");
    std::fs::write(&path, source).unwrap();
    let mut p = PendingPsd::read(&path).unwrap();
    p.reimport = true;
    p.replace = false;
    p.reimport_report = Some(ReimportPsd::prepare(&s.doc, source).unwrap().1);
    (s, p, dir)
}
fn choose(s: &AppState, p: &mut PendingPsd, path: &str) {
    p.path_resolution = Resolution {
        open: true,
        old: Some("hand".into()),
        new: Some(path.into()),
        ..Default::default()
    };
    prepare(s, p).unwrap();
}
fn unchanged(s: &AppState, before: &ankhimate_document::snapshot::Snapshot, depth: usize) {
    assert!(snapshot(&s.doc) == *before);
    assert_eq!(s.history.undo_depth(), depth);
}
#[test]
fn t822_resolver_requires_both_explicit_selections_and_prepare_without_writes() {
    let (s, mut p, _dir) = fixture(RENAMED);
    let before = snapshot(&s.doc);
    p.path_resolution.open = true;
    assert!(prepare(&s, &mut p).is_err());
    assert!(p.path_resolution.old.is_none() && p.path_resolution.new.is_none());
    p.path_resolution.old = Some("hand".into());
    assert!(prepare(&s, &mut p).is_err());
    p.path_resolution.new = Some("hero/arm/hand_right".into());
    prepare(&s, &mut p).unwrap();
    assert!(ready(&p));
    unchanged(&s, &before, 0);
}
#[test]
fn t822_apply_once_then_three_cycles_without_psd_file() {
    let (mut s, mut p, dir) = fixture(RENAMED);
    choose(&s, &mut p, "hero/arm/hand_right");
    let before = snapshot(&s.doc);
    apply(&mut s, &p).unwrap();
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 1);
    assert!(apply(&mut s, &p).is_err());
    std::fs::remove_file(dir.path().join("renamed.psd")).unwrap();
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t822_cancel_and_refusal_leave_selection_dirty_revision_and_redo_untouched() {
    let (mut s, mut p, _dir) = fixture(RENAMED);
    choose(&s, &mut p, "hero/arm/hand_right");
    apply(&mut s, &p).unwrap();
    s.undo();
    let before = snapshot(&s.doc);
    let rev = s.revision;
    let selection = s.session.selected_bones.clone();
    let depth = s.history.undo_depth();
    choose(&s, &mut p, "hero/arm/hand_right");
    p.path_resolution = Default::default();
    p.path_resolution.open = true;
    assert!(apply(&mut s, &p).is_err());
    unchanged(&s, &before, depth);
    assert_eq!(rev, s.revision);
    assert_eq!(selection, s.session.selected_bones);
    assert!(s.history.can_redo());
    s.redo();
    assert_eq!(s.doc.psd_layer_paths["hand"], "hero/arm/hand_right");
}
#[test]
fn t822_changed_deleted_or_replaced_source_rejects_without_losing_redo() {
    for change in 0..3 {
        let (mut s, mut p, dir) = fixture(RENAMED);
        choose(&s, &mut p, "hero/arm/hand_right");
        let before = snapshot(&s.doc);
        let source = dir.path().join("renamed.psd");
        match change {
            0 => std::fs::write(&source, MOVED).unwrap(),
            1 => std::fs::remove_file(&source).unwrap(),
            _ => p.source_path = Some("another source.psd".into()),
        };
        assert!(apply(&mut s, &p).is_err());
        unchanged(&s, &before, 0);
    }
}
#[test]
fn t822_in_memory_source_change_rejects_stale_plan() {
    let (mut s, mut p, dir) = fixture(RENAMED);
    choose(&s, &mut p, "hero/arm/hand_right");
    p.bytes = MOVED.to_vec();
    std::fs::write(dir.path().join("renamed.psd"), MOVED).unwrap();
    let before = snapshot(&s.doc);
    assert!(apply(&mut s, &p).is_err());
    unchanged(&s, &before, 0);
}
#[test]
fn t822_mode_and_bone_lock_changes_refuse_without_consuming_plan() {
    for lock in [false, true] {
        let (mut s, mut p, _dir) = fixture(RENAMED);
        choose(&s, &mut p, "hero/arm/hand_right");
        let before = snapshot(&s.doc);
        let bone =
            s.doc.skeleton.slots[p.path_resolution.plan.as_ref().unwrap().plan.preview.slot].bone;
        if lock {
            s.session.locked_bones.insert(bone, true);
        } else {
            s.session.work_mode = WorkMode::Animate;
        }
        assert!(apply(&mut s, &p).is_err());
        unchanged(&s, &before, 0);
        s.session.work_mode = WorkMode::Setup;
        s.session.locked_bones.insert(bone, false);
        apply(&mut s, &p).unwrap();
    }
}
#[test]
fn t822_replaced_document_and_native_edit_invalidate_plan() {
    for replace in [false, true] {
        let (mut s, mut p, _dir) = fixture(RENAMED);
        choose(&s, &mut p, "hero/arm/hand_right");
        if replace {
            s.replace_document(s.doc.clone());
        } else {
            let bone = s.doc.skeleton.update_order[0];
            let t = s.doc.skeleton.bones[bone].local_transform;
            s.dispatch(Box::new(
                ankhimate_document::commands::bone_cmds::SetBoneTransform::new(bone, t),
            ));
        }
        let before = snapshot(&s.doc);
        let depth = s.history.undo_depth();
        assert!(apply(&mut s, &p).is_err());
        unchanged(&s, &before, depth);
    }
}
#[test]
fn t822_direct_document_mutation_is_detected_even_without_revision() {
    let (mut s, mut p, _dir) = fixture(RENAMED);
    choose(&s, &mut p, "hero/arm/hand_right");
    s.doc.meta.name = "external change".into();
    let before = snapshot(&s.doc);
    assert!(apply(&mut s, &p).is_err());
    unchanged(&s, &before, 0);
}
#[test]
fn t822_author_preservation_error_prevents_run_import_and_prepare() {
    let (mut s, mut p, _dir) = fixture(RENAMED);
    p.preservation_error = Some("Unknown author data cannot survive conversion".into());
    let before = snapshot(&s.doc);
    assert!(prepare(&s, &mut p).is_err());
    assert!(super::super::run_import(&mut s, &p).is_err());
    unchanged(&s, &before, 0);
}
#[test]
fn t822_save_reload_second_exact_reimport_and_v2_export_are_stable() {
    let (mut s, mut p, dir) = fixture(MOVED);
    choose(&s, &mut p, "hero/arms/arm/hand");
    let old_source = std::fs::read(dir.path().join("renamed.psd")).unwrap();
    let id = s.doc.assets.by_name("hand").unwrap();
    let encoded = s.doc.assets.images[id].bytes.clone();
    apply(&mut s, &p).unwrap();
    assert_eq!(s.doc.assets.by_name("hand"), Some(id));
    assert_ne!(s.doc.assets.images[id].bytes, encoded);
    let schema = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let file = dir.path().join("saved.ankh");
    ankhimate_formats::save(&file, &s.doc.as_project_ref(), &[]).unwrap();
    let loaded = ankhimate_formats::load(&file).unwrap().0;
    assert!(loaded.report.dangling.is_empty() && loaded.report.lossy.is_empty());
    let d = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        serde_json::to_value(ankhimate_formats::convert::to_schema(&d.as_project_ref())).unwrap(),
        serde_json::to_value(&schema).unwrap()
    );
    for a in s.doc.assets.images.values() {
        let b = &d.assets.images[d.assets.by_name(&a.name).unwrap()];
        assert_eq!(a.bytes, b.bytes);
    }
    ankhimate_formats::reimport_guard::verify_source(&file).unwrap();
    let (cmd, r) = ReimportPsd::prepare(&d, MOVED).unwrap();
    assert!(cmd.is_none() && r.conflicts.is_empty());
    assert!(!r.layers.removed.contains(&"hero/arm/hand".into()));
    assert!(!r.layers.added.contains(&"hero/arms/arm/hand".into()));
    assert!(r.unchanged_paths.contains(&"hero/arms/arm/hand".into()));
    let export = ankhimate_export::run::plan(
        &schema,
        &d.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    assert!(!export.files.is_empty());
    assert_eq!(
        std::fs::read(dir.path().join("renamed.psd")).unwrap(),
        old_source
    );
}
#[test]
fn t822_texture_uploads_follow_apply_undo_redo_without_stale_pixels() {
    fn pixels(s: &mut AppState) -> Vec<u8> {
        crate::ui::canvas::renderer::prepare_textures(s)
            .into_iter()
            .flat_map(|t| t.rgba)
            .collect()
    }
    let (mut s, mut p, _dir) = fixture(MOVED);
    let before = pixels(&mut s);
    choose(&s, &mut p, "hero/arms/arm/hand");
    apply(&mut s, &p).unwrap();
    let after = pixels(&mut s);
    assert_ne!(before, after);
    for _ in 0..3 {
        s.undo();
        assert_eq!(pixels(&mut s), before);
        s.redo();
        assert_eq!(pixels(&mut s), after);
    }
}
#[test]
fn t822_diagnostics_revision_changes_and_old_navigation_ticket_is_refused() {
    let (mut s, mut p, _dir) = fixture(RENAMED);
    s.doc
        .skeleton
        .add_skin(ankhimate_core::skin::Skin::new("empty diagnostic skin"));
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::ZERO);
    let index = s
        .diagnostics
        .issues
        .iter()
        .position(|i| i.code == "empty-skin")
        .unwrap();
    let ticket = s.diagnostics.ticket(index).unwrap();
    let old_rev = s.revision;
    choose(&s, &mut p, "hero/arm/hand_right");
    apply(&mut s, &p).unwrap();
    assert!(s.revision > old_rev);
    assert!(!crate::diagnostics::navigate(&mut s, &ticket));
    assert!(s.diagnostics.stale());
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::from_secs(1));
    assert!(!s.diagnostics.stale());
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::from_secs(2));
    assert!(!s.diagnostics.stale());
}
fn frame(
    ctx: &egui::Context,
    s: &mut AppState,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::super::PreviewChoice) {
    crate::ui::motion::configure(ctx, true);
    let mut choice = super::super::PreviewChoice::None;
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 1800.),
            )),
            events,
            ..Default::default()
        },
        |ui| choice = super::super::ui(ui.ctx(), s, &crate::theme::Theme::default()),
    );
    (out, choice)
}
fn click(ctx: &egui::Context, s: &mut AppState, text: &str) -> super::super::PreviewChoice {
    for _ in 0..30 {
        frame(ctx, s, vec![]);
    }
    let (out, _dir) = frame(ctx, s, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                (t.galley.text() == text).then_some(t.pos + t.galley.size() * 0.5)
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Missing UI text {text}"));
    frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
    let mut result = super::super::PreviewChoice::None;
    for pressed in [true, false] {
        let (_, c) = frame(
            ctx,
            s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
        if c != super::super::PreviewChoice::None {
            result = c;
        }
    }
    result
}
#[test]
fn t822_real_egui_selection_prepare_and_final_apply_are_separate_actions() {
    let (mut s, p, _dir) = fixture(RENAMED);
    let before = snapshot(&s.doc);
    s.session.pending_psd = Some(p);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    frame(&ctx, &mut s, vec![]);
    click(&ctx, &mut s, "Resolve path change…");
    assert!(
        s.session
            .pending_psd
            .as_ref()
            .unwrap()
            .path_resolution
            .old
            .is_none()
    );
    assert!(
        s.session
            .pending_psd
            .as_ref()
            .unwrap()
            .path_resolution
            .new
            .is_none()
    );
    click(&ctx, &mut s, "hand — hero/arm/hand");
    click(&ctx, &mut s, "hero/arm/hand_right");
    click(&ctx, &mut s, "Prepare path mapping");
    assert!(
        ready(s.session.pending_psd.as_ref().unwrap()),
        "old={:?}; new={:?}; error={:?}",
        s.session.pending_psd.as_ref().unwrap().path_resolution.old,
        s.session.pending_psd.as_ref().unwrap().path_resolution.new,
        s.session
            .pending_psd
            .as_ref()
            .unwrap()
            .path_resolution
            .error
    );
    unchanged(&s, &before, 0);
    assert!(
        click(&ctx, &mut s, "Apply path mapping and image batch")
            == super::super::PreviewChoice::Import
    );
    let p = s.session.pending_psd.take().unwrap();
    super::super::run_import(&mut s, &p).unwrap();
    assert_eq!(s.doc.psd_layer_paths["hand"], "hero/arm/hand_right");
    assert_eq!(s.history.undo_depth(), 1);
}
#[test]
fn t822_real_egui_cancel_mapping_and_cancel_preview_never_write() {
    let (mut s, mut p, _dir) = fixture(RENAMED);
    choose(&s, &mut p, "hero/arm/hand_right");
    s.session.pending_psd = Some(p);
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    frame(&ctx, &mut s, vec![]);
    click(&ctx, &mut s, "Cancel path mapping");
    assert!(!s.session.pending_psd.as_ref().unwrap().path_resolution.open);
    assert!(click(&ctx, &mut s, "Cancel") == super::super::PreviewChoice::Cancel);
    unchanged(&s, &before, 0);
}

#[test]
fn t822_leaving_resolution_mode_keeps_normal_import_available() {
    let (mut s, p, _dir) = fixture(RENAMED);
    let before = snapshot(&s.doc);
    s.session.pending_psd = Some(p);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    frame(&ctx, &mut s, vec![]);
    click(&ctx, &mut s, "Resolve path change…");
    assert!(s.session.pending_psd.as_ref().unwrap().path_resolution.open);
    click(&ctx, &mut s, "Update existing PSD images (keep rig)");
    assert!(!s.session.pending_psd.as_ref().unwrap().reimport);
    assert!(click(&ctx, &mut s, "Import") == super::super::PreviewChoice::Import);
    unchanged(&s, &before, 0);
}
