use super::*;
use ankhimate_core::animation::Interp;
use ankhimate_document::path_keys::{Action, Capture};
#[allow(dead_code, reason = "Synthetic native path project")]
mod rig {
    include!("../../document/tests/support/path_document_fixture.rs");
}
#[test]
fn t831_lifecycle_dirty_save_as_reload_assets_history_and_real_fixtures() {
    let (doc, a, c) = rig::path_document();
    let mut app = AnkhimateApp::default();
    app.state.replace_document(doc);
    let dir = tempfile::tempdir().unwrap();
    let original = dir.path().join("before.ankh");
    app.save_project_to(&original);
    assert!(!app.is_dirty());
    let bytes = std::fs::read(&original).unwrap();
    let before = document_snapshot(&app.state);
    app.state.session.work_mode = crate::session::WorkMode::Animate;
    app.state.session.active_animation = Some(a);
    let cmd = Capture::read(&app.state.doc, a, c)
        .unwrap()
        .command(
            &app.state.doc,
            Action::Put {
                time: 0.25,
                value: 0.2,
                interp: Interp::Stepped,
            },
        )
        .unwrap()
        .unwrap();
    assert!(app.state.dispatch(Box::new(cmd)));
    let after = document_snapshot(&app.state);
    assert!(app.is_dirty());
    assert_eq!(app.state.history.undo_depth(), 1);
    for _ in 0..3 {
        app.state.undo();
        assert_eq!(document_snapshot(&app.state), before);
        assert!(!app.is_dirty());
        app.state.redo();
        assert_eq!(document_snapshot(&app.state), after);
        assert!(app.is_dirty());
    }
    app.save_project_to(dir.path());
    assert!(app.is_dirty());
    assert_eq!(app.current_path, Some(original.clone()));
    let next = dir.path().join("chemin-é.ankh");
    app.save_project_to(&next);
    assert!(!app.is_dirty());
    assert_eq!(app.current_path, Some(next.clone()));
    assert_eq!(std::fs::read(&original).unwrap(), bytes);
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
    if let Some(dest) = std::env::var_os("T831_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir_all(&dest).unwrap();
        for name in [
            "AnimatedPath.ankh",
            "AnimatedPath.ankh.json",
            "AnimatedPath.ankh.min.json",
        ] {
            app.save_project_to(&dest.join(name));
            assert!(!app.is_dirty());
        }
        app.state.undo();
        app.save_project_to(&dest.join("Before.ankh"));
    }
}
#[test]
fn t831_plugin_refusal_returns_original_document_without_script_execution() {
    let (doc, _, _) = rig::path_document();
    let before = ankhimate_document::snapshot::snapshot(&doc);
    let exporter = ankhimate_plugins::exporter::JsExporter {
        id: "probe".into(),
        label: "probe".into(),
        source: "throw Error('must never execute')".into(),
        resources: Default::default(),
    };
    let Err((err, doc)) = exporter.plan(doc) else {
        panic!("unsupported animated data accepted")
    };
    assert!(err.to_string().contains("PathPosition"));
    assert!(ankhimate_document::snapshot::snapshot(&doc) == before);
}
