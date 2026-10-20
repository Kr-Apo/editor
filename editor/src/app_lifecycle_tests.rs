//! M001c: the production lifecycle seam without native dialogs or GPU setup.
use super::*;
use ankhimate_core::{math::Transform, skeleton::Bone};
use ankhimate_document::commands::bone_cmds::CreateBone;

#[test]
fn t818_inspection_list_and_navigation_preserve_dirty_saved_snapshot_and_redo() {
    let mut app = edited();
    app.state.undo();
    let before = document_snapshot(&app.state);
    let dirty = app.is_dirty();
    let saved = app.saved_document.clone();
    let history = (app.state.history.undo_depth(), app.state.history.can_redo());
    app.state.diagnostics.poll(
        &app.state.doc,
        app.state.revision,
        std::time::Duration::ZERO,
    );
    let _ = egui::Context::default().run_ui(Default::default(), |ui| {
        crate::diagnostics::badge(ui, &mut app.state);
        crate::diagnostics::list(ui, &mut app.state);
    });
    let index = app
        .state
        .diagnostics
        .issues
        .iter()
        .position(|i| i.code == "empty-skin")
        .unwrap();
    let ticket = app.state.diagnostics.ticket(index).unwrap();
    assert!(crate::diagnostics::navigate(&mut app.state, &ticket));
    assert_eq!(before, document_snapshot(&app.state));
    assert_eq!(dirty, app.is_dirty());
    assert_eq!(saved, app.saved_document);
    assert_eq!(
        history,
        (app.state.history.undo_depth(), app.state.history.can_redo())
    );
    app.state.redo();
    assert!(app.is_dirty());
}

#[test]
fn close_waits_for_png_cancellation_before_issuing_the_close_command() {
    let mut app = edited();
    let mut snapshot = app.state.doc.clone();
    snapshot
        .animations
        .insert(ankhimate_core::animation::Animation::new("export", 1.0));
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    app.state.session.export.png.start(
        snapshot,
        ankhimate_render::sequence::SequenceRequest {
            animation: "export".into(),
            fps: 60,
            width: 32,
            height: 32,
        },
        dir.path().to_path_buf(),
        ctx.clone(),
    );
    app.run_file_action(FileAction::Close);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert!(!app.close_allowed);
    assert!(
        app.state.session.export.png.is_running(),
        "cancelling Close preserves the export"
    );
    app.run_file_action(FileAction::Close);
    app.confirm_file_action(UnsavedChoice::Discard, None);
    let output = ctx.run_ui(Default::default(), |ui| {
        app.show_unsaved_confirmation(ui.ctx())
    });
    assert!(
        !output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|cmd| matches!(cmd, egui::ViewportCommand::Close))
    );
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while app.state.session.export.png.is_running() {
        app.state.session.export.png.poll();
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    let output = ctx.run_ui(Default::default(), |ui| {
        app.show_unsaved_confirmation(ui.ctx())
    });
    assert!(
        output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|cmd| matches!(cmd, egui::ViewportCommand::Close))
    );
}

fn edited() -> AnkhimateApp {
    let mut app = AnkhimateApp::default();
    let bone = Bone {
        name: "work".into(),
        parent: None,
        length: 10.0,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    };
    app.state.dispatch(Box::new(CreateBone::new(bone)));
    let id = app.state.doc.skeleton.update_order[0];
    app.state.session.selected_bones.push(id);
    app
}

fn psd_fixture() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../formats/tests/fixtures/tagged_rig.psd")
}

#[test]
fn a_psd_file_drop_stages_the_preview_without_replacing_work() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    let pixels = serde_json::to_value(&app.state.doc.assets).unwrap();
    let selection = app.state.session.selected_bones.clone();
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(
        egui::RawInput {
            dropped_files: vec![egui::DroppedFile {
                path: Some(psd_fixture()),
                ..Default::default()
            }],
            ..Default::default()
        },
        |ui| {
            app.handle_psd_drop(ui.ctx());
            crate::ui::canvas::ui(ui, &mut app.state, &app.theme, &app.config.grid, false);
        },
    );
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(serde_json::to_value(&app.state.doc.assets).unwrap(), pixels);
    assert_eq!(app.state.session.selected_bones, selection);
    assert_eq!(app.state.history.undo_depth(), 1);
    wait(&mut app);
    assert!(app.state.session.pending_psd.is_some());
    assert_eq!(document_snapshot(&app.state), before);
}

#[test]
fn psd_file_import_reveals_workspace_and_import_report_only_after_success() {
    let mut app = AnkhimateApp {
        show_startup: true,
        ..Default::default()
    };
    app.start_import(
        psd_fixture(),
        "import.psd".into(),
        crate::fileops::importers(),
        "Photoshop PSD".into(),
    );
    assert!(app.show_startup, "the worker has not handed off yet");
    wait(&mut app);
    assert!(
        app.show_startup,
        "preview alone does not replace the current view"
    );
    assert!(app.state.session.pending_psd.is_some());
    app.run_file_action(FileAction::ApplyPsd);
    assert!(
        !app.show_startup,
        "successful import must reveal the workspace"
    );
    assert_eq!(app.state.doc.skeleton.bones.len(), 4);
    assert_eq!(app.state.doc.assets.len(), 11);
    assert!(
        app.state.session.import_summary.is_some(),
        "skipped layers must remain visible"
    );
    assert!(app.is_dirty());
    assert!(app.current_path.is_none());
}

#[test]
fn failed_and_cancelled_psd_import_keep_document_selection_history_and_startup() {
    let dir = tempfile::tempdir().unwrap();
    let invalid = dir.path().join("invalid.psd");
    std::fs::write(&invalid, b"not a PSD").unwrap();
    let mut app = edited();
    app.show_startup = true;
    let before = document_snapshot(&app.state);
    let selection = app.state.session.selected_bones.clone();
    let revision = app.state.revision;
    app.finish_file_outcome(crate::fileops::FileOutcome::Cancelled);
    app.start_import(
        invalid,
        "import.psd".into(),
        crate::fileops::importers(),
        "Photoshop PSD".into(),
    );
    wait(&mut app);
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.session.selected_bones, selection);
    assert_eq!(app.state.revision, revision);
    assert_eq!(app.state.history.undo_depth(), 1);
    assert!(app.show_startup);
    assert!(app.import_report.is_none());
    assert!(app.is_dirty());
}

#[test]
fn psd_import_cancelled_by_unsaved_confirmation_keeps_pending_action_and_history() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    let selection = app.state.session.selected_bones.clone();
    app.start_import(
        psd_fixture(),
        "import.psd".into(),
        crate::fileops::importers(),
        "PSD".into(),
    );
    wait(&mut app);
    let action = FileAction::ApplyPsd;
    app.run_file_action(action.clone());
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.session.selected_bones, selection);
    assert_eq!(app.pending_file_action, Some(action));
    assert!(app.file_load.is_none());
    assert_eq!(app.state.history.undo_depth(), 1);
}

#[test]
fn an_async_psd_result_cannot_replace_a_document_edited_during_import() {
    let mut app = edited();
    app.start_import(
        psd_fixture(),
        "import.psd".into(),
        crate::fileops::importers(),
        "Photoshop PSD".into(),
    );
    app.state.doc.meta.name = "newer work".into();
    let before = document_snapshot(&app.state);
    wait(&mut app);
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.history.undo_depth(), 1);
}

#[test]
fn psd_command_dirty_selection_save_and_reopen_preserve_lifecycle_guarantees() {
    use ankhimate_document::commands::psd_cmds::ImportPsd;
    let dir = tempfile::tempdir().unwrap();
    for replace in [false, true] {
        let mut app = edited();
        let path = dir.path().join(format!("import-{replace}.ankh"));
        app.save_project_to(&path);
        assert!(
            !app.is_dirty(),
            "initial save must succeed: {:?}",
            app.status
        );
        let before = document_snapshot(&app.state);
        let old_bones: Vec<_> = app.state.doc.skeleton.bones.keys().collect();
        let source = ankhimate_formats::psd::import(
            include_bytes!("../../formats/tests/fixtures/tagged_rig.psd"),
            &Default::default(),
        )
        .unwrap();
        app.state
            .dispatch(Box::new(ImportPsd::new(source, replace, "tagged")));
        let after = document_snapshot(&app.state);
        assert!(app.is_dirty());
        if !replace {
            assert_eq!(
                app.state.session.selected_bones.len(),
                1,
                "merge retains existing valid selection"
            );
        }
        let imported_bone = app
            .state
            .doc
            .skeleton
            .bones
            .keys()
            .find(|id| !old_bones.contains(id))
            .unwrap();
        app.state.session.select_bone(Some(imported_bone));
        for _ in 0..3 {
            app.state.undo();
            assert_eq!(document_snapshot(&app.state), before);
            assert!(!app.is_dirty());
            assert!(
                app.state.session.selected_bones.is_empty(),
                "removed imported entities are pruned"
            );
            app.state.redo();
            assert_eq!(document_snapshot(&app.state), after);
            assert!(app.is_dirty());
        }
        app.save_project_to(&path);
        assert!(!app.is_dirty());
        app.state.undo();
        assert!(app.is_dirty());
        app.state.redo();
        assert!(!app.is_dirty());
        let mut reopened = AnkhimateApp::default();
        reopened.start_open(path);
        wait(&mut reopened);
        assert_eq!(document_snapshot(&reopened.state), after);
        let old_pixels: std::collections::BTreeMap<_, _> = app
            .state
            .doc
            .assets
            .images
            .values()
            .map(|a| (&a.name, &a.bytes))
            .collect();
        let new_pixels: std::collections::BTreeMap<_, _> = reopened
            .state
            .doc
            .assets
            .images
            .values()
            .map(|a| (&a.name, &a.bytes))
            .collect();
        assert_eq!(old_pixels, new_pixels);
        assert!(!reopened.is_dirty());
        assert!(
            !reopened.state.history.can_undo(),
            "open starts a fresh session"
        );
    }
}

#[test]
fn cancel_preserves_document_selection_history_and_suspends_pending_action() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    let selected = app.state.session.selected_bones.clone();
    let revision = app.state.revision;
    app.run_file_action(FileAction::New);
    assert!(app.confirm_unsaved);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.session.selected_bones, selected);
    assert_eq!(app.state.revision, revision);
    assert_eq!(app.state.history.undo_depth(), 1);
    assert!(!app.confirm_unsaved);
    assert_eq!(app.pending_file_action, Some(FileAction::New));
    app.state.undo();
    assert!(app.state.history.can_redo());
    app.state.redo();
    assert_eq!(document_snapshot(&app.state), before);
}

#[test]
fn discard_creates_a_clean_project_only_after_explicit_confirmation() {
    let mut app = edited();
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Discard, None);
    assert!(app.state.doc.skeleton.bones.is_empty());
    assert!(app.current_path.is_none());
    assert!(!app.is_dirty());
    assert!(!app.state.history.can_undo());
}

#[test]
fn save_failure_aborts_pending_new_and_preserves_selection_history_and_path() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = edited();
    app.current_path = Some(dir.path().join("original.ankh"));
    let before = document_snapshot(&app.state);
    let selected = app.state.session.selected_bones.clone();
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Save, Some(dir.path()));
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.session.selected_bones, selected);
    assert!(app.state.history.can_undo());
    assert!(app.is_dirty());
    assert_eq!(app.current_path, Some(dir.path().join("original.ankh")));
    assert!(!app.close_allowed);
    assert!(app.status.as_deref().unwrap().contains("save failed"));
}

#[test]
fn save_before_new_finishes_on_disk_before_document_replacement() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("saved.ankh");
    let mut app = edited();
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Save, Some(&path));
    assert_eq!(
        ankhimate_formats::load(&path)
            .unwrap()
            .0
            .skeleton
            .bones
            .len(),
        1
    );
    assert!(app.state.doc.skeleton.bones.is_empty());
    assert!(!app.is_dirty());
}

#[test]
fn save_as_changes_active_path_only_after_success_and_preserves_undo_redo() {
    let dir = tempfile::tempdir().unwrap();
    let original = dir.path().join("original.ankh");
    let target = dir.path().join("copy.ankh");
    let mut app = edited();
    app.current_path = Some(original.clone());
    app.save_project_to(dir.path());
    assert_eq!(app.current_path, Some(original));
    assert!(app.is_dirty());
    app.save_project_to(&target);
    assert_eq!(app.current_path, Some(target));
    assert!(!app.is_dirty());
    assert!(app.state.history.can_undo());
    app.state.undo();
    assert!(app.is_dirty());
    app.state.redo();
    assert!(!app.is_dirty(), "redo reaches the real saved content");
}

#[test]
fn close_is_confirmed_and_cancelled_without_replacing_the_document() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    app.run_file_action(FileAction::Close);
    assert!(!app.close_allowed);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.pending_file_action, Some(FileAction::Close));
    app.run_file_action(FileAction::Close);
    app.confirm_file_action(UnsavedChoice::Discard, None);
    assert!(app.close_allowed);
    assert_eq!(document_snapshot(&app.state), before);
}

#[test]
fn rapid_actions_do_not_replace_a_pending_confirmation() {
    let mut app = edited();
    app.run_file_action(FileAction::New);
    app.run_file_action(FileAction::OpenPath("other.ankh".into()));
    app.run_file_action(FileAction::Close);
    assert_eq!(app.pending_file_action, Some(FileAction::New));
    assert!(!app.close_allowed);
    assert!(app.file_load.is_none());
}

#[test]
fn failed_open_keeps_the_current_document_and_history() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = edited();
    let before = document_snapshot(&app.state);
    app.run_file_action(FileAction::OpenPath(dir.path().join("absent.ankh")));
    app.confirm_file_action(UnsavedChoice::Discard, None);
    wait(&mut app);
    assert_eq!(document_snapshot(&app.state), before);
    assert!(app.state.history.can_undo());
    assert!(app.current_path.is_none());
    assert!(app.is_dirty());
}

fn wait(app: &mut AnkhimateApp) {
    for _ in 0..200 {
        app.poll_file_load();
        if app.file_load.is_none() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    panic!("background load did not complete");
}

#[test]
fn edits_while_loading_prevent_a_late_handoff_even_without_revision_change() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("incoming.ankh");
    crate::fileops::write_to(&AppState::default(), &path);
    let mut app = AnkhimateApp::default();
    app.start_open(path);
    app.state.doc.meta.name = "edit after load started".into();
    wait(&mut app);
    assert_eq!(app.state.doc.meta.name, "edit after load started");
    assert!(app.current_path.is_none());
}

#[test]
fn cancelled_confirmation_preserves_an_existing_background_operation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("incoming.ankh");
    crate::fileops::write_to(&AppState::default(), &path);
    let mut app = AnkhimateApp::default();
    app.start_open(path);
    app.state.doc.meta.name = "edited".into();
    app.run_file_action(FileAction::New);
    app.poll_file_load();
    assert!(
        app.file_load.is_some(),
        "handoff is paused while confirmation is up"
    );
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert!(app.file_load.is_some());
    assert_eq!(app.pending_file_action, Some(FileAction::New));
}

#[test]
fn newer_open_requests_win_and_old_results_cannot_replace_them() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.ankh");
    let b = dir.path().join("b.ankh");
    let mut source = AppState::default();
    source.doc.meta.name = "A".into();
    crate::fileops::write_to(&source, &a);
    source.doc.meta.name = "B".into();
    crate::fileops::write_to(&source, &b);
    let mut app = AnkhimateApp::default();
    app.start_open(a);
    app.start_open(b.clone());
    wait(&mut app);
    assert_eq!(app.state.doc.meta.name, "B");
    assert_eq!(app.current_path, Some(b));
    app.poll_file_load();
    assert_eq!(app.state.doc.meta.name, "B");
}

#[test]
fn missing_primary_still_offers_named_recovery_and_decline_keeps_work() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("lost.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    let mut source = AppState::default();
    source.doc.meta.name = "crashed work".into();
    crate::fileops::write_to(&source, &copy);
    let mut app = AnkhimateApp {
        show_startup: true,
        ..Default::default()
    };
    app.open_initial(Some(primary));
    assert!(app.recovery.is_some());
    let before = document_snapshot(&app.state);
    app.ignore_recovery();
    assert!(app.recovery.is_none());
    assert_eq!(document_snapshot(&app.state), before);
    assert!(copy.exists());
}

#[test]
fn corrupt_primary_can_recover_even_from_an_older_copy() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("corrupt.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    let mut source = AppState::default();
    source.doc.meta.name = "recover me".into();
    crate::fileops::write_to(&source, &copy);
    std::thread::sleep(std::time::Duration::from_millis(25));
    std::fs::write(&primary, b"interrupted write").unwrap();
    let mut app = AnkhimateApp::default();
    app.open_initial(Some(primary.clone()));
    assert!(app.recovery.is_some());
    app.recover_project();
    assert_eq!(app.state.doc.meta.name, "recover me");
    assert_eq!(app.current_path, Some(primary.clone()));
    assert!(app.is_dirty());
    assert_eq!(
        std::fs::read(primary).unwrap(),
        b"interrupted write",
        "recovery is offered/loaded, never silently saved"
    );
}

#[test]
fn failed_recovery_preserves_current_document_selection_and_history() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("recovery.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    std::fs::write(&copy, b"bad payload").unwrap();
    let mut app = edited();
    let before = document_snapshot(&app.state);
    let selection = app.state.session.selected_bones.clone();
    app.offer_recovery(Some(&primary), false);
    app.recover_project();
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.state.session.selected_bones, selection);
    assert!(app.state.history.can_undo());
    assert!(copy.exists());
    assert!(
        app.recovery.is_some(),
        "failed recovery stays available for Ignore/retry"
    );
}

#[test]
fn a_stale_recovery_prompt_cannot_replace_a_newer_document() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("recovery.ankh");
    let copy = crate::autosave::path_for(Some(&primary)).unwrap();
    crate::fileops::write_to(&AppState::default(), &copy);
    let mut app = AnkhimateApp::default();
    app.offer_recovery(Some(&primary), false);
    app.state.doc.meta.name = "newer work".into();
    app.recover_project();
    assert_eq!(app.state.doc.meta.name, "newer work");
    assert!(app.current_path.is_none());
}

#[test]
fn interrupted_save_manifest_offers_payload_with_original_assets() {
    use ankhimate_core::assets::ImageAsset;
    use ankhimate_document::commands::asset_cmds::ImportImage;
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("interrupted.ankh");
    let mut source = edited().state;
    let root = source.doc.skeleton.update_order[0];
    source.dispatch(Box::new(ImportImage::new(
        ImageAsset::new("image", vec![1, 2, 3], 1, 1),
        root,
        glam::vec2(0.0, 0.0),
    )));
    crate::fileops::write_to(&source, &primary);
    let stage = dir.path().join(".ankh-save-interrupted-test");
    std::fs::create_dir(&stage).unwrap();
    let destination = std::fs::canonicalize(&primary).unwrap();
    std::fs::write(
        stage.join("manifest.json"),
        serde_json::to_vec(
            &serde_json::json!({"save_version":1,"destination": destination.to_str().unwrap()}),
        )
        .unwrap(),
    )
    .unwrap();
    std::fs::rename(&primary, stage.join("previous")).unwrap();
    std::fs::write(stage.join("new"), b"partial write").unwrap();
    let mut app = AnkhimateApp::default();
    app.open_initial(Some(primary.clone()));
    assert_eq!(
        app.recovery.as_ref().unwrap().autosave,
        std::fs::canonicalize(stage.join("previous")).unwrap()
    );
    app.recover_project();
    assert_eq!(
        app.state.doc.assets.images.values().next().unwrap().bytes,
        [1, 2, 3]
    );
    assert_eq!(app.current_path, Some(primary));
    assert!(app.is_dirty());
    assert!(stage.join("previous").exists());
}

#[test]
fn clean_project_does_not_autosave_over_an_unanswered_recovery() {
    let app = AnkhimateApp::default();
    assert!(!app.is_dirty());
    // The production frame gates scheduling on dirty and on recovery=None.
    assert!(!app.should_autosave());
}

#[test]
fn cancelled_save_picker_keeps_confirmation_and_never_runs_pending_close() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    app.run_file_action(FileAction::Close);
    app.finish_pending_save(crate::fileops::FileOutcome::Cancelled);
    assert!(app.confirm_unsaved);
    assert_eq!(app.pending_file_action, Some(FileAction::Close));
    assert!(!app.close_allowed);
    assert_eq!(document_snapshot(&app.state), before);
}

#[test]
fn failed_saves_abort_pending_close_and_open_without_starting_load() {
    let dir = tempfile::tempdir().unwrap();
    for action in [
        FileAction::Close,
        FileAction::OpenPath(dir.path().join("incoming.ankh")),
    ] {
        let mut app = edited();
        let before = document_snapshot(&app.state);
        app.run_file_action(action.clone());
        app.confirm_file_action(UnsavedChoice::Save, Some(dir.path()));
        assert_eq!(document_snapshot(&app.state), before);
        assert_eq!(app.pending_file_action, Some(action));
        assert!(app.confirm_unsaved);
        assert!(app.file_load.is_none());
        assert!(!app.close_allowed);
    }
}

#[test]
fn root_window_close_is_intercepted_and_escape_preserves_pending_request() {
    let mut app = edited();
    let before = document_snapshot(&app.state);
    let ctx = egui::Context::default();
    let mut input = egui::RawInput::default();
    input
        .viewports
        .get_mut(&egui::ViewportId::ROOT)
        .unwrap()
        .events
        .push(egui::ViewportEvent::Close);
    let output = ctx.run_ui(input, |ui| {
        app.handle_close_request(ui.ctx());
        app.show_unsaved_confirmation(ui.ctx());
    });
    assert!(
        output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|c| matches!(c, egui::ViewportCommand::CancelClose))
    );
    assert!(app.confirm_unsaved);
    let mut escape = egui::RawInput::default();
    escape.events.push(egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    });
    let _ = ctx.run_ui(escape, |ui| app.show_unsaved_confirmation(ui.ctx()));
    assert!(!app.confirm_unsaved);
    assert_eq!(app.pending_file_action, Some(FileAction::Close));
    assert_eq!(document_snapshot(&app.state), before);
    assert!(app.state.history.can_undo());
}

#[cfg(windows)]
#[test]
fn locked_save_as_target_preserves_both_files_active_path_and_dirty_state() {
    use std::os::windows::fs::OpenOptionsExt;
    let dir = tempfile::tempdir().unwrap();
    let original = dir.path().join("original.ankh");
    let target = dir.path().join("target.ankh");
    let mut app = edited();
    app.save_project_to(&original);
    let original_bytes = std::fs::read(&original).unwrap();
    std::fs::write(&target, b"previous target").unwrap();
    let held = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(1)
        .open(&target)
        .unwrap();
    app.state.doc.meta.name = "new work".into();
    app.save_project_to(&target);
    drop(held);
    assert_eq!(app.current_path, Some(original.clone()));
    assert!(app.is_dirty());
    assert_eq!(std::fs::read(original).unwrap(), original_bytes);
    assert_eq!(std::fs::read(target).unwrap(), b"previous target");
}

#[test]
fn ordinary_save_after_cancel_does_not_resume_the_suspended_action() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = edited();
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    app.save_project_to(&dir.path().join("keep.ankh"));
    assert_eq!(app.state.doc.skeleton.bones.len(), 1);
    assert_eq!(app.pending_file_action, Some(FileAction::New));
    assert!(!app.is_dirty());
}

#[test]
fn selection_is_not_dirty_but_metadata_is_and_clean_close_needs_no_prompt() {
    let mut app = AnkhimateApp::default();
    app.state.session.selected_bones.clear();
    assert!(!app.is_dirty());
    app.state.doc.meta.name = "changed metadata".into();
    assert!(app.is_dirty());
    app.mark_saved();
    app.run_file_action(FileAction::Close);
    assert!(app.close_allowed);
    assert!(!app.confirm_unsaved);
}

#[test]
fn recovery_prompt_can_be_drawn_on_startup_and_escape_declines_without_deleting() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("lost.ankh");
    let copy = crate::autosave::path_for(Some(&project)).unwrap();
    crate::fileops::write_to(&AppState::default(), &copy);
    let mut app = AnkhimateApp {
        show_startup: true,
        ..Default::default()
    };
    app.open_initial(Some(project));
    let before = document_snapshot(&app.state);
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
        app.show_recovery_prompt(ui.ctx())
    });
    assert!(app.recovery.is_some());
    let mut input = egui::RawInput::default();
    input.events.push(egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    });
    let _ = ctx.run_ui(input, |ui| app.show_recovery_prompt(ui.ctx()));
    assert!(app.recovery.is_none());
    assert_eq!(document_snapshot(&app.state), before);
    assert!(copy.exists());
}

#[test]
fn recovery_reports_missing_assets_without_silently_hiding_load_warnings() {
    use ankhimate_core::assets::ImageAsset;
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("lost.ankh");
    let copy = crate::autosave::path_for(Some(&project)).unwrap();
    let mut source = AppState::default();
    let asset = ImageAsset::new("missing.png", vec![1, 2, 3], 1, 1);
    let uri = ankhimate_formats::convert::asset_file_name(&asset);
    source.doc.assets.add(asset);
    crate::fileops::write_to(&source, &copy);
    std::fs::remove_file(ankhimate_formats::container::asset_root(&copy).join(uri)).unwrap();
    let mut app = AnkhimateApp::default();
    app.offer_recovery(Some(&project), false);
    app.recover_project();
    assert!(app.status.as_deref().unwrap().contains("unresolved"));
    assert!(app.status.as_deref().unwrap().contains("missing.png"));
    assert!(app.is_dirty());
}

#[test]
fn reactive_idle_frame_must_write_after_thirty_seconds() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("idle.ankh");
    let mut app = edited();
    app.current_path = Some(project.clone());
    app.config.autosave_secs = 30;
    let ctx = egui::Context::default();
    let now = std::time::Instant::now();
    // Settle egui's initial repaint requests before simulating a sleeping UI.
    for time in [0.0, 0.02, 0.04, 0.06, 0.08, 0.10, 30.1] {
        let input = egui::RawInput {
            time: Some(time),
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            app.poll_autosave(ui.ctx(), now + std::time::Duration::from_secs_f64(time))
        });
    }
    assert!(
        crate::autosave::path_for(Some(&project)).unwrap().exists(),
        "idle elapsed time must trigger a recovery copy without playback"
    );
}

#[test]
fn reactive_scheduler_requests_one_deadline_and_stops_after_success() {
    use std::time::{Duration, Instant};
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("deadline.ankh");
    let mut app = edited();
    app.current_path = Some(project.clone());
    app.config.autosave_secs = 30;
    let ctx = egui::Context::default();
    let now = Instant::now();
    let mut delay = Duration::ZERO;
    for time in [0.0, 0.02, 0.04, 0.06, 0.08, 0.10] {
        let output = ctx.run_ui(
            egui::RawInput {
                time: Some(time),
                ..Default::default()
            },
            |ui| app.poll_autosave(ui.ctx(), now + Duration::from_secs_f64(time)),
        );
        delay = output.viewport_output[&egui::ViewportId::ROOT].repaint_delay;
    }
    assert!(
        delay > Duration::from_secs(29) && delay <= Duration::from_secs(30),
        "reactive backend must receive the remaining deadline, got {delay:?}"
    );
    let output = ctx.run_ui(
        egui::RawInput {
            time: Some(30.0),
            ..Default::default()
        },
        |ui| app.poll_autosave(ui.ctx(), now + Duration::from_secs(30)),
    );
    assert!(crate::autosave::path_for(Some(&project)).unwrap().exists());
    assert_eq!(
        output.viewport_output[&egui::ViewportId::ROOT].repaint_delay,
        Duration::MAX,
        "unchanged content must not keep waking the UI"
    );
}

#[test]
fn manual_save_cancels_the_autosave_deadline_and_later_edits_rearm_it() {
    use std::time::{Duration, Instant};
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("manual.ankh");
    let copy = crate::autosave::path_for(Some(&project)).unwrap();
    let mut app = edited();
    app.current_path = Some(project.clone());
    app.config.autosave_secs = 30;
    let ctx = egui::Context::default();
    let now = Instant::now();
    let _ = ctx.run_ui(Default::default(), |ui| app.poll_autosave(ui.ctx(), now));
    app.save_project_to(&project);
    let primary = std::fs::read(&project).unwrap();
    let _ = ctx.run_ui(Default::default(), |ui| {
        app.poll_autosave(ui.ctx(), now + Duration::from_secs(35))
    });
    assert!(!copy.exists());
    app.state.doc.meta.name = "later edit".into();
    for elapsed in [40, 69] {
        let _ = ctx.run_ui(Default::default(), |ui| {
            app.poll_autosave(ui.ctx(), now + Duration::from_secs(elapsed))
        });
        assert!(!copy.exists());
    }
    let _ = ctx.run_ui(Default::default(), |ui| {
        app.poll_autosave(ui.ctx(), now + Duration::from_secs(70))
    });
    assert_eq!(ankhimate_formats::load(&copy).unwrap().0.name, "later edit");
    assert_eq!(std::fs::read(&project).unwrap(), primary);
    assert!(app.is_dirty());
    assert!(app.state.history.can_undo());
}

#[test]
fn autosave_error_preserves_document_selection_history_and_primary() {
    use std::time::{Duration, Instant};
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("failure.ankh");
    let copy = crate::autosave::path_for(Some(&project)).unwrap();
    let mut app = edited();
    app.save_project_to(&project);
    let primary = std::fs::read(&project).unwrap();
    app.state.doc.meta.name = "unsaved edit".into();
    app.config.autosave_secs = 30;
    let document = document_snapshot(&app.state);
    let selection = app.state.session.selected_bones.clone();
    std::fs::create_dir(&copy).unwrap();
    let ctx = egui::Context::default();
    let now = Instant::now();
    for elapsed in [0, 30] {
        let _ = ctx.run_ui(Default::default(), |ui| {
            app.poll_autosave(ui.ctx(), now + Duration::from_secs(elapsed))
        });
    }
    assert!(
        app.status
            .as_deref()
            .unwrap()
            .starts_with("Recovery copy failed:")
    );
    assert_eq!(document_snapshot(&app.state), document);
    assert_eq!(app.state.session.selected_bones, selection);
    assert!(app.state.history.can_undo());
    assert!(app.is_dirty());
    assert_eq!(std::fs::read(project).unwrap(), primary);
    assert_eq!(
        app.autosave
            .schedule(now + Duration::from_secs(31), &app.state, true),
        crate::autosave::Schedule::Wait(Duration::from_secs(29))
    );
}

#[test]
fn t813_corrupt_newer_sidecar_cannot_hide_a_valid_interrupted_save() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("recovery.ankh");
    let mut source = AppState::default();
    source.doc.meta.name = "recoverable stage".into();
    assert!(matches!(
        crate::fileops::write_to(&source, &primary),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let stage = dir.path().join(".ankh-save-t813");
    std::fs::create_dir(&stage).unwrap();
    let destination = std::fs::canonicalize(&primary).unwrap();
    std::fs::write(
        stage.join("manifest.json"),
        serde_json::json!({"save_version":1,"destination":destination}).to_string(),
    )
    .unwrap();
    std::fs::rename(&primary, stage.join("previous")).unwrap();
    let sidecar = crate::autosave::path_for(Some(&primary)).unwrap();
    std::fs::write(&sidecar, b"interrupted corrupt autosave").unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .open(&sidecar)
        .unwrap()
        .set_times(
            std::fs::FileTimes::new()
                .set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60)),
        )
        .unwrap();
    assert!(
        std::fs::metadata(&sidecar).unwrap().modified().unwrap()
            > std::fs::metadata(stage.join("previous"))
                .unwrap()
                .modified()
                .unwrap()
    );
    // Both candidates qualify with a missing primary. Validity must precede mtime ranking.
    let mut app = AnkhimateApp::default();
    app.open_initial(Some(primary.clone()));
    app.recover_project();
    assert_eq!(app.state.doc.meta.name, "recoverable stage");
    assert!(app.is_dirty());
    assert_eq!(
        std::fs::read(sidecar).unwrap(),
        b"interrupted corrupt autosave"
    );
    assert!(stage.join("previous").exists());
    assert!(!primary.exists());
}

#[test]
fn t813_missing_pixels_block_save_as_autosave_and_pending_new_until_native_relink() {
    use ankhimate_core::assets::ImageAsset;
    use ankhimate_document::commands::asset_cmds::{AddAsset, ReplaceAssetPixels};
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("primary.ankh");
    let target = dir.path().join("après relink.ankh");
    let mut app = edited();
    app.save_project_to(&primary);
    let primary_bytes = std::fs::read(&primary).unwrap();
    app.state.dispatch(Box::new(AddAsset::new(ImageAsset::new(
        "missing",
        vec![],
        1,
        1,
    ))));
    let snapshot = document_snapshot(&app.state);
    let selected = app.state.session.selected_bones.clone();
    let history = (app.state.history.can_undo(), app.state.history.can_redo());
    app.save_project_to(&target);
    assert_eq!(app.current_path.as_ref(), Some(&primary));
    assert!(!target.exists());
    assert!(app.is_dirty());
    assert!(app.status.as_deref().unwrap().contains("relink"));
    assert!(app.autosave.write(&app.state, Some(&primary)).is_none());
    assert!(
        app.autosave
            .last_error
            .as_deref()
            .unwrap()
            .contains("relink")
    );
    assert!(!crate::autosave::path_for(Some(&primary)).unwrap().exists());
    app.run_file_action(FileAction::New);
    app.confirm_file_action(UnsavedChoice::Save, Some(&primary));
    assert!(app.confirm_unsaved);
    assert_eq!(document_snapshot(&app.state), snapshot);
    assert_eq!(app.state.session.selected_bones, selected);
    assert_eq!(
        (app.state.history.can_undo(), app.state.history.can_redo()),
        history
    );
    assert_eq!(std::fs::read(&primary).unwrap(), primary_bytes);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    let asset = app.state.doc.assets.by_name("missing").unwrap();
    let mut png = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(1, 1, image::Rgba([40, 80, 120, 255]))
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    let bytes = png.into_inner();
    app.state.dispatch(Box::new(ReplaceAssetPixels::relink(
        asset,
        bytes.clone(),
        1,
        1,
        "replacement.png".into(),
    )));
    let relinked = document_snapshot(&app.state);
    for _ in 0..3 {
        app.state.undo();
        assert_eq!(document_snapshot(&app.state), snapshot);
        app.state.redo();
        assert_eq!(document_snapshot(&app.state), relinked);
    }
    app.save_project_to(&target);
    assert_eq!(app.current_path, Some(target.clone()));
    assert!(!app.is_dirty());
    let reopened = ankhimate_formats::load(&target).unwrap().0;
    assert!(reopened.report.is_clean());
    assert_eq!(reopened.assets.images.values().next().unwrap().bytes, bytes);
    assert_eq!(std::fs::read(primary).unwrap(), primary_bytes);
}

#[test]
fn t813_save_as_after_recovery_keeps_original_and_sidecar_and_reopens() {
    let dir = tempfile::tempdir().unwrap();
    let primary = dir.path().join("source.ankh");
    let sidecar = crate::autosave::path_for(Some(&primary)).unwrap();
    let target = dir.path().join("recovered elsewhere.ankh");
    let mut app = edited();
    app.save_project_to(&primary);
    let original = std::fs::read(&primary).unwrap();
    app.state.doc.meta.name = "recovered version".into();
    assert!(matches!(
        crate::fileops::write_to(&app.state, &sidecar),
        crate::fileops::FileOutcome::Saved(_)
    ));
    std::fs::OpenOptions::new()
        .write(true)
        .open(&sidecar)
        .unwrap()
        .set_times(
            std::fs::FileTimes::new()
                .set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60)),
        )
        .unwrap();
    let recovery_bytes = std::fs::read(&sidecar).unwrap();
    app.open_initial(Some(primary.clone()));
    app.recover_project();
    assert_eq!(app.state.doc.meta.name, "recovered version");
    assert!(app.is_dirty());
    app.run_file_action(FileAction::Close);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert!(!app.close_allowed);
    assert!(app.is_dirty());
    app.save_project_to(&target);
    assert!(!app.is_dirty());
    assert_eq!(
        ankhimate_formats::load(&target).unwrap().0.name,
        "recovered version"
    );
    assert_eq!(std::fs::read(primary).unwrap(), original);
    assert_eq!(std::fs::read(sidecar).unwrap(), recovery_bytes);
}
