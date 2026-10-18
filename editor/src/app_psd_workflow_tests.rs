//! B05: all PSD entry points stage the existing preview before mutation.
use super::*;

fn fixture() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../formats/tests/fixtures/tagged_rig.psd")
}

fn wait(app: &mut AnkhimateApp) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while app.file_load.is_some() {
        app.poll_file_load();
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

fn seed() -> AnkhimateApp {
    let mut app = AnkhimateApp::default();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
    let doc = ankhimate_document::Document::from_loaded(ankhimate_formats::load(&path).unwrap().0);
    app.state.replace_document(doc);
    app.state.session.selected_bones = vec![app.state.doc.skeleton.update_order[0]];
    app.mark_saved();
    app
}

fn snapshot(app: &AnkhimateApp) -> serde_json::Value {
    let assets: std::collections::BTreeMap<_, _> = app
        .state
        .doc
        .assets
        .images
        .values()
        .map(|asset| (&asset.name, asset))
        .collect();
    serde_json::json!({"project": document_snapshot(&app.state), "assets": assets, "psd_paths": app.state.doc.psd_layer_paths})
}

fn stage(app: &mut AnkhimateApp) {
    app.run_file_action(FileAction::PreviewPsd(fixture()));
    wait(app);
    assert!(app.state.session.pending_psd.is_some());
}

#[test]
fn file_psd_import_stages_options_without_touching_the_document_or_history() {
    let mut app = AnkhimateApp::default();
    app.state.doc.meta.name = "unsaved work".into();
    let before = document_snapshot(&app.state);
    let assets = serde_json::to_value(&app.state.doc.assets).unwrap();
    app.start_import(
        fixture(),
        "import.psd".into(),
        crate::fileops::importers(),
        "Photoshop PSD".into(),
    );
    wait(&mut app);
    assert!(
        app.state.session.pending_psd.is_some(),
        "File Import must show the existing PSD preview"
    );
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(serde_json::to_value(&app.state.doc.assets).unwrap(), assets);
    assert_eq!(app.state.history.undo_depth(), 0);
    assert!(
        !app.confirm_unsaved,
        "choosing options is not a destructive action"
    );
}

#[test]
fn preview_failure_does_not_replace_work_or_create_an_undo_step() {
    let dir = tempfile::tempdir().unwrap();
    let invalid = dir.path().join("invalid.psd");
    std::fs::write(&invalid, b"not a PSD").unwrap();
    let mut app = AnkhimateApp::default();
    app.state.doc.meta.name = "keep".into();
    let before = document_snapshot(&app.state);
    app.start_import(
        invalid,
        "import.psd".into(),
        crate::fileops::importers(),
        "Photoshop PSD".into(),
    );
    wait(&mut app);
    assert_eq!(document_snapshot(&app.state), before);
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(app.state.history.undo_depth(), 0);
    assert!(app.status.is_some(), "a parsing error is reported");
}

#[test]
fn assets_request_and_os_drop_reach_the_same_preview_and_escape_preserves_everything() {
    for route in 0..3 {
        let mut app = seed();
        let before = snapshot(&app);
        let selection = app.state.session.selected_bones.clone();
        let ctx = egui::Context::default();
        if route != 0 {
            let _ = ctx.run_ui(
                egui::RawInput {
                    dropped_files: vec![egui::DroppedFile {
                        path: Some(fixture()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                |ui| {
                    if route == 1 {
                        app.handle_psd_drop(ui.ctx());
                    } else {
                        crate::ui::canvas::ui(
                            ui,
                            &mut app.state,
                            &app.theme,
                            &app.config.grid,
                            false,
                        );
                    }
                },
            );
            if route == 2 {
                app.resolve_frame(None, false, false, None);
            }
        } else {
            app.state.session.request_psd_import = Some(fixture());
            app.resolve_frame(None, false, false, None);
        }
        wait(&mut app);
        let pending = app.state.session.pending_psd.as_ref().unwrap();
        assert_eq!(pending.scale, 1.0);
        assert!(pending.skip_hidden && pending.replace);
        assert!(!pending.nodes.is_empty());
        assert!(!pending.guesses.is_empty());
        assert_eq!(snapshot(&app), before);
        let _ = ctx.run_ui(Default::default(), |ui| app.show_psd_preview(ui.ctx()));
        let _ = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: Some(egui::Key::Escape),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| app.show_psd_preview(ui.ctx()),
        );
        assert!(app.state.session.pending_psd.is_none());
        assert_eq!(snapshot(&app), before);
        assert_eq!(app.state.session.selected_bones, selection);
        assert_eq!(app.state.history.undo_depth(), 0);
        assert!(!app.is_dirty());
    }
}

#[test]
fn replacement_cancel_failed_save_and_successful_save_follow_m001() {
    let mut app = seed();
    app.state.doc.meta.name = "unsaved old rig".into();
    let before = snapshot(&app);
    let selection = app.state.session.selected_bones.clone();
    stage(&mut app);
    app.run_file_action(FileAction::ApplyPsd);
    assert!(app.confirm_unsaved);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    assert_eq!(snapshot(&app), before);
    assert_eq!(app.state.session.selected_bones, selection);
    assert!(app.state.session.pending_psd.is_some());
    let dir = tempfile::tempdir().unwrap();
    let blocked = dir.path().join("file");
    std::fs::write(&blocked, b"keep").unwrap();
    app.run_file_action(FileAction::ApplyPsd);
    app.confirm_file_action(UnsavedChoice::Save, Some(&blocked.join("failure.ankh")));
    assert!(app.confirm_unsaved);
    assert_eq!(snapshot(&app), before);
    assert_eq!(std::fs::read(&blocked).unwrap(), b"keep");
    assert!(app.current_path.is_none());
    let saved = dir.path().join("previous.ankh");
    app.confirm_file_action(UnsavedChoice::Save, Some(&saved));
    assert!(!app.confirm_unsaved);
    assert!(
        app.current_path.is_none(),
        "replacement is an unsaved imported project"
    );
    assert!(app.state.session.pending_psd.is_none());
    let mut old = AnkhimateApp::default();
    let outcome = crate::fileops::open_path(&mut old.state, &saved);
    old.finish_file_outcome(outcome);
    assert_eq!(
        snapshot(&old),
        before,
        "old rig and pixels were saved before replacement"
    );
    assert_eq!(app.state.history.undo_depth(), 1);
    app.state.undo();
    assert_eq!(snapshot(&app), before);
}

#[test]
fn replace_and_merge_are_single_undo_steps_for_three_cycles_and_survive_save_open_export() {
    for replace in [false, true] {
        let mut app = seed();
        let before = snapshot(&app);
        let old_bones = app.state.doc.skeleton.bones.len();
        let old_assets = app.state.doc.assets.len();
        let old_ids: Vec<_> = app.state.doc.skeleton.bones.keys().collect();
        let old_asset_ids: Vec<_> = app.state.doc.assets.images.keys().collect();
        stage(&mut app);
        app.state.session.pending_psd.as_mut().unwrap().replace = replace;
        app.run_file_action(FileAction::ApplyPsd);
        assert!(!app.confirm_unsaved);
        assert!(app.state.session.import_summary.is_some());
        assert!(
            app.state.session.status.is_none(),
            "the app's import report must not mask later Save/Open results"
        );
        assert!(app.is_dirty());
        if !replace {
            assert!(app.state.doc.skeleton.bones.len() > old_bones);
            assert!(app.state.doc.assets.len() > old_assets);
            assert!(!app.state.doc.animations.is_empty());
        } else {
            assert!(app.state.doc.animations.is_empty());
        }
        let after = snapshot(&app);
        let new_ids: Vec<_> = app.state.doc.skeleton.bones.keys().collect();
        let new_asset_ids: Vec<_> = app.state.doc.assets.images.keys().collect();
        for _ in 0..3 {
            app.state.undo();
            assert_eq!(snapshot(&app), before);
            assert_eq!(
                app.state.doc.skeleton.bones.keys().collect::<Vec<_>>(),
                old_ids
            );
            assert_eq!(
                app.state.doc.assets.images.keys().collect::<Vec<_>>(),
                old_asset_ids
            );
            app.state.redo();
            assert_eq!(snapshot(&app), after);
            assert_eq!(
                app.state.doc.skeleton.bones.keys().collect::<Vec<_>>(),
                new_ids
            );
            assert_eq!(
                app.state.doc.assets.images.keys().collect::<Vec<_>>(),
                new_asset_ids
            );
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("imported.ankh");
        app.save_project_to(&path);
        assert!(!app.is_dirty());
        app.start_open(path);
        wait(&mut app);
        assert_eq!(snapshot(&app), after);
        assert!(!app.is_dirty());
        let schema = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
        let preset = ankhimate_export::presets::builtin()
            .into_iter()
            .find(|p| p.name.contains("Ankhimate"))
            .unwrap();
        let plan = ankhimate_export::run::plan(&schema, &app.state.doc.assets, &preset).unwrap();
        ankhimate_export::run::write(&plan, &dir.path().join("runtime")).unwrap();
        app.state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("render", 1.0));
        let request = ankhimate_render::sequence::SequenceRequest {
            animation: "render".into(),
            fps: 2,
            width: 24,
            height: 24,
        };
        let cancel = std::sync::atomic::AtomicBool::new(false);
        crate::png_export::export(
            &app.state.doc,
            &request,
            &dir.path().join("png"),
            &cancel,
            |_, _, _| {},
        )
        .unwrap();
        crate::spritesheet_export::export(
            &app.state.doc,
            &request,
            64,
            2,
            &dir.path().join("sheet"),
            &cancel,
            |_, _, _| {},
        )
        .unwrap();
    }
}

#[test]
fn edits_during_read_or_after_preview_and_newer_open_requests_cannot_be_overwritten() {
    let mut app = seed();
    app.run_file_action(FileAction::PreviewPsd(fixture()));
    app.state.doc.meta.name = "edited during read".into();
    let edited = snapshot(&app);
    wait(&mut app);
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(snapshot(&app), edited);
    stage(&mut app);
    app.state.doc.meta.name = "edited after preview".into();
    let newer = snapshot(&app);
    app.run_file_action(FileAction::ApplyPsd);
    app.confirm_file_action(UnsavedChoice::Discard, None);
    assert_eq!(snapshot(&app), newer);
    assert!(app.state.session.pending_psd.is_none());
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("newer.ankh");
    app.save_project_to(&path);
    app.run_file_action(FileAction::PreviewPsd(fixture()));
    app.start_open(path);
    wait(&mut app);
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(snapshot(&app), newer);
}

#[test]
fn a_failed_import_or_mode_change_keeps_preview_document_selection_and_history() {
    let mut app = seed();
    let before = snapshot(&app);
    let selection = app.state.session.selected_bones.clone();
    stage(&mut app);
    app.state.session.pending_psd.as_mut().unwrap().bytes = b"broken".to_vec();
    app.run_file_action(FileAction::ApplyPsd);
    assert!(app.state.session.pending_psd.is_some());
    assert_eq!(snapshot(&app), before);
    assert_eq!(app.state.session.selected_bones, selection);
    assert_eq!(app.state.history.undo_depth(), 0);
    assert!(!app.is_dirty());
    app.state.session.pending_psd = None;
    app.state
        .set_work_mode(ankhimate_document::WorkMode::Animate);
    app.run_file_action(FileAction::PreviewPsd(fixture()));
    assert!(app.file_load.is_none());
    assert_eq!(snapshot(&app), before);
}

#[test]
fn mixed_drops_and_second_previews_are_refused_without_partial_changes() {
    let mut app = seed();
    let before = snapshot(&app);
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(
        egui::RawInput {
            dropped_files: vec![
                egui::DroppedFile {
                    path: Some(fixture()),
                    ..Default::default()
                },
                egui::DroppedFile {
                    path: Some("other.png".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        |ui| {
            app.handle_psd_drop(ui.ctx());
            assert!(ui.input(|i| i.raw.dropped_files.is_empty()));
        },
    );
    assert!(app.file_load.is_none());
    assert_eq!(snapshot(&app), before);
    assert!(app.status.as_deref().unwrap().contains("one PSD"));
    stage(&mut app);
    app.state.session.pending_psd.as_mut().unwrap().scale = 0.5;
    app.run_file_action(FileAction::PreviewPsd("other.psd".into()));
    assert!(app.file_load.is_none());
    assert_eq!(app.state.session.pending_psd.as_ref().unwrap().scale, 0.5);
    assert_eq!(snapshot(&app), before);
}

#[test]
fn replaced_psd_and_three_undo_redo_cycles_use_current_pixels_despite_recycled_asset_ids() {
    fn check_pixels(app: &mut AnkhimateApp) {
        let mut fresh = crate::app_state::AppState {
            doc: app.state.doc.clone(),
            ..Default::default()
        };
        let expected = crate::ui::canvas::renderer::prepare_textures(&mut fresh);
        let actual = crate::ui::canvas::renderer::prepare_textures(&mut app.state);
        let expected_keys: Vec<_> = fresh
            .doc
            .assets
            .images
            .keys()
            .map(|id| (id, fresh.session.texture_keys[id]))
            .collect();
        let actual_keys: Vec<_> = app
            .state
            .doc
            .assets
            .images
            .keys()
            .map(|id| (id, app.state.session.texture_keys[id]))
            .collect();
        assert_eq!(
            actual_keys, expected_keys,
            "memoized keys must refer to the current asset bytes"
        );
        for upload in actual {
            let wanted = expected
                .iter()
                .find(|candidate| candidate.key == upload.key)
                .unwrap();
            assert_eq!(upload.rgba, wanted.rgba);
            assert_eq!((upload.width, upload.height), (wanted.width, wanted.height));
        }
    }
    let mut app = seed();
    check_pixels(&mut app);
    stage(&mut app);
    app.run_file_action(FileAction::ApplyPsd);
    check_pixels(&mut app);
    assert!(
        app.state.session.selected_bones.is_empty(),
        "replacement must not select an unrelated bone with a recycled ID"
    );
    for _ in 0..3 {
        app.state.undo();
        check_pixels(&mut app);
        app.state.redo();
        check_pixels(&mut app);
    }
}
#[path = "../../formats/tests/support/psd_copy.rs"]
#[allow(
    dead_code,
    reason = "Shared RAW fixture editor; dimension/name cases run in document integration tests"
)]
mod reimport_psd_copy;
const REIMPORT_SOURCE: &[u8] = include_bytes!("../../formats/tests/fixtures/nested_groups.psd");
fn reimport_app() -> AnkhimateApp {
    let mut app = AnkhimateApp::default();
    let p = crate::ui::psd_import::PendingPsd::new(
        "animated".into(),
        REIMPORT_SOURCE.to_vec(),
        ankhimate_formats::psd::layer_tree(REIMPORT_SOURCE).unwrap(),
    );
    crate::ui::psd_import::run_import(&mut app.state, &p).unwrap();
    let root = app.state.doc.skeleton.update_order[0];
    let mut a = ankhimate_core::animation::Animation::new("motion", 2.);
    a.timelines
        .push(ankhimate_core::animation::Timeline::BoneRotate {
            bone: root,
            keys: vec![
                ankhimate_core::animation::Key::linear(0., 0.),
                ankhimate_core::animation::Key::stepped(1., 45.),
            ],
        });
    app.state.doc.animations.insert(a);
    app.state.session.selected_bones = vec![root];
    app.state.history = Default::default();
    app.mark_saved();
    app
}
fn reimport_file(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("modified.psd");
    let bytes = reimport_psd_copy::edit(REIMPORT_SOURCE, |ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    std::fs::write(&path, bytes).unwrap();
    path
}
fn reimport_stage(app: &mut AnkhimateApp, path: std::path::PathBuf) {
    app.start_psd_preview(path);
    wait(app);
    let pending = app.state.session.pending_psd.as_mut().unwrap();
    pending.reimport = true;
    pending.replace = false;
    pending.reimport_report = Some(
        ankhimate_document::psd_reimport::ReimportPsd::prepare(&app.state.doc, &pending.bytes)
            .unwrap()
            .1,
    );
}
#[test]
fn reimport_preview_escape_preserves_selection_history_document_and_dirty_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = reimport_file(dir.path());
    let mut app = reimport_app();
    let before = snapshot(&app);
    let selection = app.state.session.selected_bones.clone();
    reimport_stage(&mut app, path);
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(Default::default(), |ui| app.show_psd_preview(ui.ctx()));
    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: Some(egui::Key::Escape),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |ui| app.show_psd_preview(ui.ctx()),
    );
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(snapshot(&app), before);
    assert_eq!(app.state.history.undo_depth(), 0);
    assert_eq!(app.state.session.selected_bones, selection);
    assert!(!app.is_dirty());
}
#[test]
fn reimport_save_as_reopen_repeated_update_and_failed_save_preserve_m001() {
    let dir = tempfile::tempdir().unwrap();
    let psd = reimport_file(dir.path());
    let mut app = reimport_app();
    let original = dir.path().join("old.ankh");
    app.save_project_to(&original);
    let before = std::fs::read(&original).unwrap();
    let old_assets = ankhimate_formats::container::asset_root(&original);
    let old_files: Vec<_> = std::fs::read_dir(&old_assets)
        .unwrap()
        .flat_map(|p| std::fs::read_dir(p.unwrap().path()).unwrap())
        .map(|p| {
            let p = p.unwrap().path();
            let b = std::fs::read(&p).unwrap();
            (p, b)
        })
        .collect();
    reimport_stage(&mut app, psd.clone());
    assert!(
        app.state
            .session
            .pending_psd
            .as_ref()
            .unwrap()
            .preservation_error
            .is_none(),
        "{:?}",
        app.state
            .session
            .pending_psd
            .as_ref()
            .unwrap()
            .preservation_error
    );
    let before_doc = snapshot(&app);
    let selection = app.state.session.selected_bones.clone();
    app.run_file_action(FileAction::ApplyPsd);
    assert!(app.state.session.pending_psd.is_none(), "{:?}", app.status);
    assert!(app.is_dirty());
    assert_eq!(app.current_path.as_ref(), Some(&original));
    assert_eq!(app.state.history.undo_depth(), 1);
    assert_eq!(app.state.session.selected_bones, selection);
    let after = snapshot(&app);
    for _ in 0..3 {
        app.state.undo();
        assert_eq!(snapshot(&app), before_doc);
        app.state.redo();
        assert_eq!(snapshot(&app), after);
    }
    let bad = dir.path().join("missing-parent").join("save.ankh");
    app.save_project_to(&bad);
    assert_eq!(app.current_path.as_ref(), Some(&original));
    assert_eq!(snapshot(&app), after);
    assert_eq!(std::fs::read(&original).unwrap(), before);
    for (p, b) in old_files {
        assert_eq!(std::fs::read(p).unwrap(), b);
    }
    let new_dir = dir.path().join("new-folder");
    std::fs::create_dir(&new_dir).unwrap();
    let new = new_dir.join("saved.ankh");
    app.save_project_to(&new);
    assert_eq!(app.current_path.as_ref(), Some(&new));
    assert!(!app.is_dirty());
    app.start_open(new);
    wait(&mut app);
    assert_eq!(snapshot(&app), after);
    reimport_stage(&mut app, psd);
    app.run_file_action(FileAction::ApplyPsd);
    assert!(!app.is_dirty());
    assert_eq!(app.state.history.undo_depth(), 0);
    assert_eq!(snapshot(&app), after);
    let schema = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &schema,
        &app.state.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    assert!(!plan.files.is_empty());
    let runtime = dir.path().join("runtime");
    ankhimate_export::run::write(&plan, &runtime).unwrap();
    let _: serde_json::Value =
        serde_json::from_slice(&std::fs::read(runtime.join("skeleton.json")).unwrap()).unwrap();
}
#[test]
fn reimport_unknown_author_fields_are_blocked_before_confirmation_and_at_apply() {
    let dir = tempfile::tempdir().unwrap();
    let psd = reimport_file(dir.path());
    let mut app = reimport_app();
    let path = dir.path().join("future.ankh.json");
    app.save_project_to(&path);
    let mut raw = serde_json::to_value(ankhimate_formats::convert::to_schema(
        &app.state.doc.as_project_ref(),
    ))
    .unwrap();
    raw["future_root"] = serde_json::json!({"must_not_drop":true});
    std::fs::write(&path, raw.to_string()).unwrap();
    app.current_path = Some(path.clone());
    let before = snapshot(&app);
    let original = std::fs::read(&path).unwrap();
    reimport_stage(&mut app, psd);
    assert!(
        app.state
            .session
            .pending_psd
            .as_ref()
            .unwrap()
            .preservation_error
            .is_some()
    );
    app.run_file_action(FileAction::ApplyPsd);
    assert_eq!(snapshot(&app), before);
    assert_eq!(app.state.history.undo_depth(), 0);
    assert_eq!(std::fs::read(path).unwrap(), original);
    assert!(app.state.session.pending_psd.is_some());
}
#[test]
fn reimport_refuses_stale_document_and_source_read_failures() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = reimport_app();
    let before = snapshot(&app);
    app.start_psd_preview(dir.path().join("absent.psd"));
    wait(&mut app);
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(snapshot(&app), before);
    let path = reimport_file(dir.path());
    reimport_stage(&mut app, path);
    app.state.doc.meta.name = "newer edit".into();
    let newer = snapshot(&app);
    app.run_file_action(FileAction::ApplyPsd);
    assert_eq!(snapshot(&app), newer);
    assert!(app.state.session.pending_psd.is_none());
    assert_eq!(app.state.history.undo_depth(), 0);
}
#[test]
fn reimport_refreshes_texture_uploads_and_refuses_animate_mode() {
    fn upload(app: &mut AnkhimateApp) -> Vec<(u64, Vec<u8>)> {
        let mut state = crate::app_state::AppState {
            doc: app.state.doc.clone(),
            ..Default::default()
        };
        let expected = crate::ui::canvas::renderer::prepare_textures(&mut state);
        let actual = crate::ui::canvas::renderer::prepare_textures(&mut app.state);
        let mut results = vec![];
        for entry in actual {
            let e = expected.iter().find(|e| e.key == entry.key).unwrap();
            assert_eq!(e.rgba, entry.rgba);
            results.push((entry.key, entry.rgba));
        }
        results.sort_by_key(|x| x.0);
        results
    }
    let dir = tempfile::tempdir().unwrap();
    let path = reimport_file(dir.path());
    let mut app = reimport_app();
    let old = upload(&mut app);
    let before = snapshot(&app);
    reimport_stage(&mut app, path.clone());
    app.state.session.work_mode = ankhimate_document::WorkMode::Animate;
    app.run_file_action(FileAction::ApplyPsd);
    assert_eq!(snapshot(&app), before);
    assert_eq!(app.state.history.undo_depth(), 0);
    assert!(app.state.session.pending_psd.is_some());
    app.state.session.work_mode = ankhimate_document::WorkMode::Setup;
    app.run_file_action(FileAction::ApplyPsd);
    assert!(app.state.session.pending_psd.is_none());
    std::fs::remove_file(&path).unwrap(); // Redo must work with no PSD source on disk.
    let new = upload(&mut app);
    assert_ne!(new, old);
    for _ in 0..3 {
        app.state.undo();
        assert_eq!(upload(&mut app), old);
        app.state.redo();
        assert_eq!(upload(&mut app), new);
    }
}
