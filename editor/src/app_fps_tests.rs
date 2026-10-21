//! T-828: exercise the unchanged lifecycle seams with native FPS commands.
use super::*;
use ankhimate_document::commands::document_cmds::SetProjectFps;
use std::time::{Duration, Instant};
fn set(app: &mut AnkhimateApp, fps: u32) {
    app.state.dispatch(Box::new(
        SetProjectFps::new(&app.state.doc, fps).unwrap().unwrap(),
    ));
}
#[test]
fn t828_dirty_save_undo_redo_and_reopen_exact_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("cadence-é.ankh");
    let mut app = AnkhimateApp::default();
    app.save_project_to(&p);
    let before = document_snapshot(&app.state);
    assert!(!app.is_dirty());
    set(&mut app, 60);
    let after = document_snapshot(&app.state);
    assert!(app.is_dirty());
    app.state.undo();
    assert!(!app.is_dirty());
    assert_eq!(document_snapshot(&app.state), before);
    app.state.redo();
    assert!(app.is_dirty());
    assert_eq!(document_snapshot(&app.state), after);
    app.save_project_to(&p);
    assert!(!app.is_dirty());
    app.state.undo();
    assert!(app.is_dirty());
    app.state.redo();
    assert!(!app.is_dirty());
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(p));
    assert_eq!(document_snapshot(&reopened.state), after);
    assert!(!reopened.is_dirty());
}
#[test]
fn t828_save_as_success_and_failure_preserve_primary_and_history() {
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("first.ankh");
    let next = dir.path().join("second.ankh");
    let mut app = AnkhimateApp::default();
    app.save_project_to(&first);
    let bytes = std::fs::read(&first).unwrap();
    set(&mut app, 24);
    let doc = document_snapshot(&app.state);
    let rev = app.state.revision;
    app.save_project_to(dir.path());
    assert_eq!(app.current_path, Some(first.clone()));
    assert!(app.is_dirty());
    assert_eq!(document_snapshot(&app.state), doc);
    assert_eq!(std::fs::read(&first).unwrap(), bytes);
    assert_eq!(app.state.revision, rev);
    assert_eq!(app.state.history.undo_depth(), 1);
    app.save_project_to(&next);
    assert_eq!(app.current_path, Some(next.clone()));
    assert!(!app.is_dirty());
    assert_eq!(std::fs::read(first).unwrap(), bytes);
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(next));
    assert_eq!(document_snapshot(&reopened.state), doc);
}
#[test]
fn t828_autosave_uses_changed_content_sidecar_and_controlled_clock() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("primary.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    let mut app = AnkhimateApp::default();
    app.config.autosave_secs = 30;
    app.save_project_to(&primary);
    let bytes = std::fs::read(&primary).unwrap();
    let ctx = egui::Context::default();
    let start = Instant::now();
    app.poll_autosave(&ctx, start);
    assert!(!copy.exists());
    set(&mut app, 60);
    app.poll_autosave(&ctx, start);
    app.poll_autosave(&ctx, start + Duration::from_secs(29));
    assert!(!copy.exists());
    app.poll_autosave(&ctx, start + Duration::from_secs(30));
    assert!(copy.exists());
    assert!(app.is_dirty());
    let auto_bytes = std::fs::read(&copy).unwrap();
    app.poll_autosave(&ctx, start + Duration::from_secs(300));
    assert_eq!(std::fs::read(&copy).unwrap(), auto_bytes);
    assert_eq!(
        app.autosave
            .schedule(start + Duration::from_secs(301), &app.state, true),
        crate::autosave::Schedule::Idle
    );
    assert_eq!(std::fs::read(&primary).unwrap(), bytes);
    assert_eq!(app.current_path, Some(primary.clone()));
    let mut recovered = AnkhimateApp::default();
    recovered.offer_recovery(Some(&primary), false);
    assert!(recovered.recovery.is_some());
    recovered.recover_project();
    assert_eq!(recovered.state.doc.meta.fps, 60);
    assert!(recovered.is_dirty());
    assert_eq!(std::fs::read(&primary).unwrap(), bytes);
    assert!(copy.exists());
    let mut declined = AnkhimateApp::default();
    declined.offer_recovery(Some(&primary), false);
    declined.ignore_recovery();
    assert_eq!(declined.state.doc.meta.fps, 30);
    assert!(copy.exists());
}
#[test]
fn t828_fps_close_cancel_and_failed_save_preserve_pending_action() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = AnkhimateApp::default();
    set(&mut app, 60);
    let doc = document_snapshot(&app.state);
    app.run_file_action(FileAction::Close);
    assert!(app.confirm_unsaved);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert!(!app.close_allowed);
    assert_eq!(document_snapshot(&app.state), doc);
    app.run_file_action(FileAction::Close);
    app.confirm_file_action(UnsavedChoice::Save, Some(dir.path()));
    assert!(app.confirm_unsaved);
    assert!(!app.close_allowed);
    assert_eq!(app.pending_file_action, Some(FileAction::Close));
    assert_eq!(document_snapshot(&app.state), doc);
    assert_eq!(app.state.history.undo_depth(), 1);
}
#[test]
fn t828_revision_diagnostics_coalesce_and_reject_stale_navigation() {
    let mut app = AnkhimateApp::default();
    let s = &mut app.state;
    s.diagnostics.poll(&s.doc, s.revision, Duration::ZERO);
    let ticket = s.diagnostics.ticket(0).unwrap();
    let doc = document_snapshot(s);
    let scans = s.diagnostics.scans;
    for t in 1..20 {
        s.diagnostics
            .poll(&s.doc, s.revision, Duration::from_millis(t));
    }
    assert_eq!(s.diagnostics.scans, scans);
    assert_eq!(document_snapshot(s), doc);
    for (n, action) in [0, 1, 2].into_iter().enumerate() {
        let rev = s.revision;
        match action {
            0 => {
                s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
            }
            1 => s.undo(),
            _ => s.redo(),
        }
        assert_eq!(s.revision, rev + 1);
        let t = Duration::from_secs(n as u64 + 1);
        s.diagnostics.poll(&s.doc, s.revision, t);
        assert_eq!(s.diagnostics.scans, scans + n as u64);
        s.diagnostics
            .poll(&s.doc, s.revision, t + Duration::from_millis(200));
        assert_eq!(s.diagnostics.scans, scans + n as u64 + 1);
    }
    let before = document_snapshot(s);
    assert!(!crate::diagnostics::navigate(s, &ticket));
    assert_eq!(document_snapshot(s), before);
}
#[test]
fn t828_metadata_diagnostics_fallback_is_retained() {
    let mut app = AnkhimateApp::default();
    let s = &mut app.state;
    s.diagnostics.poll(&s.doc, s.revision, Duration::ZERO);
    s.doc.meta.name = "legacy metadata mutation".into();
    s.diagnostics
        .poll(&s.doc, s.revision, Duration::from_secs(1));
    s.diagnostics
        .poll(&s.doc, s.revision, Duration::from_millis(1200));
    assert_eq!(s.diagnostics.scans, 2);
    assert_eq!(s.revision, 0);
}

#[allow(
    dead_code,
    reason = "Reuse accepted weighted native test fixture without duplicating author data"
)]
mod weighted {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
#[test]
fn t828_weighted_assets_keys_binds_and_native_export_survive_fps_history() {
    use ankhimate_export::{presets, run};
    let (doc, skin, slot) = weighted::fixture(true);
    let mut app = AnkhimateApp::default();
    app.state.replace_document(doc);
    app.mark_saved();
    let before = ankhimate_document::snapshot::snapshot(&app.state.doc);
    let bind = weighted::mesh(&app.state.doc, skin, slot)
        .inverse_bind_matrices
        .clone();
    let project = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let preset = presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    let original = run::plan(&project, &app.state.doc.assets, &preset).unwrap();
    set(&mut app, 60);
    let after = ankhimate_document::snapshot::snapshot(&app.state.doc);
    let mut compare = app.state.doc.clone();
    compare.meta.fps = 30;
    assert!(ankhimate_document::snapshot::snapshot(&compare) == before);
    assert_eq!(
        weighted::mesh(&app.state.doc, skin, slot).inverse_bind_matrices,
        bind
    );
    let new_project = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let exported = run::plan(&new_project, &app.state.doc.assets, &preset).unwrap();
    // The existing exported FPS metadata changes; second-based animation data does not.
    assert_eq!(original.files.len(), exported.files.len());
    for (old, new) in original.files.iter().zip(&exported.files) {
        assert_eq!(old.path, new.path);
        let mut old: serde_json::Value = serde_json::from_str(&old.contents).unwrap();
        let mut new: serde_json::Value = serde_json::from_str(&new.contents).unwrap();
        assert_eq!(old["fps"], 30);
        assert_eq!(new["fps"], 60);
        assert_eq!(old["format_version"], 2);
        assert_eq!(new["format_version"], 2);
        old.as_object_mut().unwrap().remove("fps");
        new.as_object_mut().unwrap().remove("fps");
        assert_eq!(old, new);
    }
    assert_eq!(original.binaries, exported.binaries);
    for _ in 0..3 {
        app.state.undo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == before);
        app.state.redo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == after);
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("weighted.ankh");
    app.save_project_to(&path);
    assert_eq!(app.current_path, Some(path.clone()));
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(path));
    assert_eq!(
        document_snapshot(&reopened.state),
        document_snapshot(&app.state)
    );
    assert_eq!(
        reopened
            .state
            .doc
            .assets
            .images
            .values()
            .next()
            .unwrap()
            .bytes,
        app.state.doc.assets.images.values().next().unwrap().bytes
    );
    // Opt-in local fixture delivery uses native save; no historical project is read or modified.
    if let Some(dest) = std::env::var_os("T828_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        let high = dest.join("FPS60.ankh");
        app.save_project_to(&high);
        assert!(high.exists());
        app.state.undo();
        let low = dest.join("FPS30.ankh");
        app.save_project_to(&low);
        assert!(low.exists());
        run::write(&exported, &dest.join("Native-v2")).unwrap();
    }
}
