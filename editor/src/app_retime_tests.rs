#[test]
fn t830_pose_menu_has_numeric_action() {
    let source = include_str!("app.rs");
    assert!(
        source.contains("self.retime.menu(ui, &self.state)"),
        "Pose lacks the numeric Retime action"
    );
}

use super::*;
use ankhimate_document::commands::key_cmds::RetimeAnimation;
#[allow(
    dead_code,
    reason = "Reuse accepted native weighted fixture without duplicating author data"
)]
mod weighted {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
fn apply(app: &mut AnkhimateApp, scale: f64, offset: f64) {
    let id = app.state.doc.animations.keys().next().unwrap();
    app.state.session.work_mode = crate::session::WorkMode::Animate;
    app.state.session.active_animation = Some(id);
    let p =
        RetimeAnimation::prepare_numeric(id, &app.state.doc.animations[id], scale, offset).unwrap();
    assert!(app.state.dispatch(Box::new(p.command)));
}
#[test]
fn t830_retime_dirty_save_save_as_reload_assets_and_export_v2() {
    let (doc, skin, slot) = weighted::fixture(true);
    let mut app = AnkhimateApp::default();
    app.state.replace_document(doc);
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("before.ankh");
    let next = dir.path().join("retimé.ankh");
    app.save_project_to(&first);
    assert!(!app.is_dirty());
    let old_bytes = std::fs::read(&first).unwrap();
    let before = ankhimate_document::snapshot::snapshot(&app.state.doc);
    let binds = weighted::mesh(&app.state.doc, skin, slot)
        .inverse_bind_matrices
        .clone();
    apply(&mut app, 1.25, 0.);
    let after = ankhimate_document::snapshot::snapshot(&app.state.doc);
    assert!(app.is_dirty());
    assert_eq!(app.state.history.undo_depth(), 1);
    assert_eq!(
        weighted::mesh(&app.state.doc, skin, slot).inverse_bind_matrices,
        binds
    );
    for _ in 0..3 {
        app.state.undo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == before);
        assert!(!app.is_dirty());
        app.state.redo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == after);
        assert!(app.is_dirty());
    }
    let project = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    let plan = ankhimate_export::run::plan(&project, &app.state.doc.assets, &preset).unwrap();
    for file in &plan.files {
        let json: serde_json::Value = serde_json::from_str(&file.contents).unwrap();
        assert_eq!(json["format_version"], 2);
    }
    app.save_project_to(dir.path());
    assert!(app.is_dirty());
    assert_eq!(app.current_path, Some(first.clone()));
    app.save_project_to(&next);
    assert!(!app.is_dirty());
    assert_eq!(std::fs::read(&first).unwrap(), old_bytes);
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(next));
    assert_eq!(
        document_snapshot(&reopened.state),
        document_snapshot(&app.state)
    );
    assert!(!reopened.is_dirty());
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
    if let Some(dest) = std::env::var_os("T830_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        app.save_project_to(&dest.join("Retimed.ankh"));
        app.state.undo();
        app.save_project_to(&dest.join("Before.ankh"));
        ankhimate_export::run::write(&plan, &dest.join("Native-v2")).unwrap();
    }
}
#[test]
fn t830_separate_applies_never_merge_or_consume_redo_on_preview() {
    let (doc, _, _) = weighted::fixture(true);
    let mut app = AnkhimateApp::default();
    app.state.replace_document(doc);
    let before = document_snapshot(&app.state);
    apply(&mut app, 2., 0.);
    let middle = document_snapshot(&app.state);
    apply(&mut app, 0.5, 0.);
    assert_eq!(app.state.history.undo_depth(), 2);
    assert_eq!(document_snapshot(&app.state), before);
    app.state.undo();
    assert_eq!(document_snapshot(&app.state), middle);
    let rev = app.state.revision;
    let id = app.state.session.active_animation.unwrap();
    assert!(RetimeAnimation::prepare_numeric(id, &app.state.doc.animations[id], -1., 0.).is_err());
    assert!(app.state.history.can_redo());
    assert_eq!(app.state.revision, rev);
}
