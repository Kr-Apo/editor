//! Actual lifecycle handoffs, driven headlessly (no native picker or GPU).
use super::*;
use crate::t840b1_physics_tests::{assert_fresh, assert_same_physics, document, ids, warmed};

fn app() -> AnkhimateApp {
    let mut app = AnkhimateApp {
        state: warmed(),
        ..Default::default()
    };
    app.mark_saved();
    app
}
fn wait(app: &mut AnkhimateApp) {
    let until = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while app.file_load.is_some() {
        app.poll_file_load();
        assert!(std::time::Instant::now() < until);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}
#[test]
fn t840b1_new_cancel_and_failed_save_keep_simulation_discard_resets_it() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = app();
    let old = ids(&app.state);
    app.saved_document = None; // Unsaved document: force the real confirmation flow.
    let physics = app.state.physics.clone();
    let doc = document_snapshot(&app.state);
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert_same_physics(&app.state, &physics);
    assert_eq!(document_snapshot(&app.state), doc);
    assert_eq!(app.pending_file_action, Some(FileAction::New));
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Save, Some(dir.path()));
    assert_same_physics(&app.state, &physics);
    assert_eq!(document_snapshot(&app.state), doc);
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Discard, None);
    assert_fresh(&app.state, old);
    assert!(!app.is_dirty());
}

#[test]
fn t840b1_failed_invalid_and_stale_background_loads_preserve_live_context() {
    let dir = tempfile::tempdir().unwrap();
    for content in [None, Some("not a project"), Some("{\"version\":999999}")] {
        let path = dir.path().join("bad.ankh.json");
        if let Some(text) = content {
            std::fs::write(&path, text).unwrap();
        }
        let mut app = app();
        let physics = app.state.physics.clone();
        let doc = document_snapshot(&app.state);
        app.start_open(path);
        wait(&mut app);
        assert_same_physics(&app.state, &physics);
        assert_eq!(document_snapshot(&app.state), doc);
        assert!(app.status.as_ref().is_some_and(|s| s.contains("failed")));
    }
    // Controlled completion avoids a thread race: a successful but stale load is rejected.
    let mut app = app();
    let physics = app.state.physics.clone();
    let (send, receive) = std::sync::mpsc::channel();
    app.file_load = Some(FileLoad {
        revision: app.state.revision,
        document: document_snapshot(&app.state),
        label: "test".into(),
        receiver: receive,
    });
    let loaded = ankhimate_formats::from_json(include_str!(
        "../../export/tests/fixtures/t840a/parent.ankh.json"
    ))
    .unwrap();
    app.state.revision += 1;
    send.send(BackgroundLoad::Opened(
        dir.path().join("stale.ankh"),
        Ok(loaded),
    ))
    .unwrap();
    app.poll_file_load();
    assert_same_physics(&app.state, &physics);
    assert!(app.status.as_ref().unwrap().contains("Load cancelled"));
}

#[test]
fn t840b1_background_open_and_recovery_success_start_fresh_save_keeps_live_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("loaded.ankh");
    let mut app = app();
    let old = ids(&app.state);
    let physics = app.state.physics.clone();
    let saved = app.saved_document.clone();
    app.save_project_to(&path);
    assert!(!app.is_dirty());
    assert_eq!(app.saved_document, saved);
    assert_same_physics(&app.state, &physics);
    app.state.physics.paused = true;
    app.start_open(path.clone());
    wait(&mut app);
    assert_fresh(&app.state, old);
    assert!(!app.is_dirty());
    app.state = warmed();
    app.mark_saved();
    let copy = crate::autosave::path_for(Some(&path)).unwrap();
    assert!(matches!(
        crate::fileops::write_to(&app.state, &copy),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let bytes = std::fs::read(&copy).unwrap();
    app.offer_recovery(Some(&path), false);
    assert!(app.recovery.is_some());
    app.state.physics.paused = true;
    app.recover_project();
    assert_fresh(&app.state, old);
    assert!(app.is_dirty());
    assert_eq!(std::fs::read(copy).unwrap(), bytes);
}

#[test]
fn t840b1_psd_replace_resets_merge_and_failures_retain_state_history_survives() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../formats/tests/fixtures/tagged_rig.psd");
    for replace in [false, true] {
        let mut app = app();
        app.state.set_work_mode(crate::session::WorkMode::Setup);
        let old = ids(&app.state);
        let physics = app.state.physics.clone();
        let before = ankhimate_document::snapshot::snapshot(&app.state.doc);
        let mut pending = crate::ui::psd_import::PendingPsd::read(&path).unwrap();
        pending.replace = replace;
        // A mode refusal and a decoding failure must leave the active context intact.
        app.state.set_work_mode(crate::session::WorkMode::Animate);
        assert!(crate::ui::psd_import::run_import(&mut app.state, &pending).is_err());
        assert_same_physics(&app.state, &physics);
        app.state.set_work_mode(crate::session::WorkMode::Setup);
        let bytes = pending.bytes.clone();
        pending.bytes = b"broken".to_vec();
        assert!(crate::ui::psd_import::run_import(&mut app.state, &pending).is_err());
        assert_same_physics(&app.state, &physics);
        pending.bytes = bytes;
        crate::ui::psd_import::run_import(&mut app.state, &pending).unwrap();
        if replace {
            assert_fresh(&app.state, old);
        } else {
            assert_same_physics(&app.state, &physics);
        }
        let after = ankhimate_document::snapshot::snapshot(&app.state.doc);
        assert_eq!(app.state.history.undo_depth(), 1);
        for _ in 0..3 {
            app.state.undo();
            assert!(before == ankhimate_document::snapshot::snapshot(&app.state.doc));
            assert!(app.state.history.can_redo());
            if replace {
                assert_fresh(&app.state, old);
            }
            app.state.redo();
            assert!(after == ankhimate_document::snapshot::snapshot(&app.state.doc));
            if replace {
                assert_fresh(&app.state, old);
            }
        }
    }
    // Foreign format adoption shares the same document replacement boundary.
    let mut app = app();
    let old = ids(&app.state);
    app.state.physics.paused = true;
    crate::fileops::adopt(
        &mut app.state,
        ankhimate_formats::from_json(
            &ankhimate_formats::to_json(&document().as_project_ref()).unwrap(),
        )
        .unwrap(),
        &path,
    );
    assert_fresh(&app.state, old);
}

#[test]
fn t840b1_failed_and_declined_recovery_preserve_document_and_paused_simulation() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("recover.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    let mut app = app();
    app.state.physics.paused = true;
    let physics = app.state.physics.clone();
    let doc = document_snapshot(&app.state);
    std::fs::write(&copy, b"corrupt recovery").unwrap();
    app.offer_recovery(Some(&primary), false);
    assert!(app.recovery.is_some());
    app.recover_project();
    assert_same_physics(&app.state, &physics);
    assert_eq!(document_snapshot(&app.state), doc);
    assert!(app.recovery.is_some());
    app.ignore_recovery();
    assert_same_physics(&app.state, &physics);
    assert_eq!(document_snapshot(&app.state), doc);
    assert!(app.recovery.is_none());
    assert_eq!(std::fs::read(copy).unwrap(), b"corrupt recovery");
}
