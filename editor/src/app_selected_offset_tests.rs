use super::*;
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
fn apply(app: &mut AnkhimateApp) {
    use ankhimate_document::commands::key_cmds::PreparedBoneOffset;
    let id = app.state.doc.animations.keys().next().unwrap();
    let bones: Vec<_> = app.state.doc.skeleton.bones.keys().collect();
    app.state.session.work_mode = crate::session::WorkMode::Animate;
    app.state.session.active_animation = Some(id);
    app.state.session.selected_bones = bones.clone();
    let p = PreparedBoneOffset::prepare(&app.state.doc, id, &bones, 3.).unwrap();
    assert!(
        app.state
            .dispatch(p.into_command(&app.state.doc).unwrap().unwrap())
    );
}
#[test]
fn t833_offset_dirty_save_save_as_reload_assets_and_export_v2() {
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
    apply(&mut app);
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
    if let Some(dest) = std::env::var_os("T833_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        app.save_project_to(&dest.join("Offset.ankh"));
        app.state.undo();
        app.save_project_to(&dest.join("Before.ankh"));
        ankhimate_export::run::write(&plan, &dest.join("Native-v2")).unwrap();
    }
}

#[allow(
    dead_code,
    reason = "Reuse the existing real native Path constraint fixture"
)]
mod native_path_document {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/path_document_fixture.rs"
    ));
}
#[test]
fn t833_real_path_position_and_native_export_boundary_unchanged() {
    use ankhimate_core::animation::{Key, Timeline};
    use ankhimate_document::commands::key_cmds::PreparedBoneOffset;
    let (doc, id, _) = native_path_document::path_document();
    let mut app = AnkhimateApp::default();
    app.state.replace_document(doc);
    let bones: Vec<_> = app.state.doc.skeleton.bones.keys().take(2).collect();
    for b in &bones {
        app.state.doc.animations[id]
            .timelines
            .push(Timeline::BoneRotate {
                bone: *b,
                keys: vec![Key::linear(0.25, 10.), Key::linear(0.75, 20.)],
            });
    }
    let path_before = app.state.doc.animations[id].timelines[0].clone();
    let original = ankhimate_document::snapshot::snapshot(&app.state.doc);
    app.state.session.work_mode = crate::session::WorkMode::Animate;
    app.state.session.active_animation = Some(id);
    let p = PreparedBoneOffset::prepare(&app.state.doc, id, &bones, 3.).unwrap();
    assert_eq!(p.summary.tracks, 2);
    assert!(
        app.state
            .dispatch(p.into_command(&app.state.doc).unwrap().unwrap())
    );
    assert_eq!(app.state.doc.animations[id].timelines[0], path_before);
    let schema = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    assert!(
        ankhimate_export::run::plan(&schema, &app.state.doc.assets, &preset).is_err(),
        "T831 explicit PathPosition runtime refusal stays"
    );
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Path-and-Bones.ankh");
    app.save_project_to(&file);
    assert!(!app.is_dirty());
    let expected = document_snapshot(&app.state);
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(file));
    assert_eq!(expected, document_snapshot(&reopened.state));
    app.state.undo();
    assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == original);
}
