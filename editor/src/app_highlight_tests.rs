use super::*;
use crate::ui::timeline::t829_probe::{animated, show};
#[test]
fn t829_large_highlight_preserves_dirty_saved_snapshot_redo_and_three_cycles() {
    let (state, a, r) = animated(1000, 1, false);
    let mut app = AnkhimateApp {
        state,
        ..Default::default()
    };
    app.mark_saved();
    let cmd = ankhimate_document::commands::key_cmds::DeleteKeys::new(a, vec![r[1].clone()]);
    app.state.dispatch(Box::new(cmd));
    let after = document_snapshot(&app.state);
    app.state.undo();
    let before = document_snapshot(&app.state);
    let saved = app.saved_document.clone();
    let rev = app.state.revision;
    let depth = app.state.history.undo_depth();
    crate::key_selection::set(&mut app.state, a, r.clone());
    let ctx = egui::Context::default();
    for mode in [0, 1, 2, 1, 0] {
        show(
            &ctx,
            &mut app.state,
            mode,
            false,
            vec![],
            Default::default(),
            false,
        );
    }
    assert_eq!(document_snapshot(&app.state), before);
    assert_eq!(app.saved_document, saved);
    assert!(!app.is_dirty());
    assert!(app.state.history.can_redo());
    assert_eq!(app.state.revision, rev);
    assert_eq!(app.state.history.undo_depth(), depth);
    assert_eq!(crate::key_selection::read(&mut app.state, a), r);
    for _ in 0..3 {
        app.state.redo();
        assert_eq!(document_snapshot(&app.state), after);
        assert!(app.is_dirty());
        show(
            &ctx,
            &mut app.state,
            0,
            false,
            vec![],
            Default::default(),
            false,
        );
        app.state.undo();
        assert_eq!(document_snapshot(&app.state), before);
        assert!(!app.is_dirty());
    }
}
#[allow(
    dead_code,
    reason = "Reuse accepted weighted native fixture, never historical projects"
)]
mod weighted {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
#[test]
fn t829_readonly_highlight_preserves_save_reload_assets_and_export_and_delivers_fixtures() {
    use ankhimate_document::commands::key_cmds::{BoneProperty, KeyRef, TimelineAddr};
    let (doc, _, _) = weighted::fixture(true);
    let a = doc.animations.keys().next().unwrap();
    let mut s = AppState::default();
    s.replace_document(doc);
    s.session.active_animation = Some(a);
    s.session.select_bone(s.doc.skeleton.bones.keys().next());
    s.set_work_mode(crate::session::WorkMode::Animate);
    let bone = s.doc.skeleton.bones.keys().next().unwrap();
    s.doc.animations[a]
        .timelines
        .push(ankhimate_core::animation::Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: (0..1000)
                .map(|i| ankhimate_core::animation::Key::linear(i as f32 / 1000., i as f32 / 100.))
                .collect(),
        });
    let refs = (0..1000)
        .map(|index| KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Translate,
                axis: Some(ankhimate_core::animation::Axis::X),
            },
            index,
        })
        .collect();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("highlight.ankh");
    let second = dir.path().join("copy.ankh");
    let mut app = AnkhimateApp {
        state: s,
        ..Default::default()
    };
    app.save_project_to(&path);
    assert_eq!(app.current_path, Some(path.clone()));
    let bytes = std::fs::read(&path).unwrap();
    let before = document_snapshot(&app.state);
    let export = |s: &AppState| {
        ankhimate_export::run::plan(
            &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
            &s.doc.assets,
            &ankhimate_export::presets::default_preset(),
        )
        .unwrap()
    };
    let exported = export(&app.state);
    crate::key_selection::set(&mut app.state, a, refs);
    let ctx = egui::Context::default();
    for mode in [2, 0, 1, 2] {
        show(
            &ctx,
            &mut app.state,
            mode,
            false,
            vec![],
            Default::default(),
            false,
        );
    }
    assert_eq!(document_snapshot(&app.state), before);
    assert!(!app.is_dirty());
    let fresh = export(&app.state);
    assert_eq!(exported.files, fresh.files);
    assert_eq!(exported.binaries, fresh.binaries);
    app.save_project_to(&path);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    app.save_project_to(&second);
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(second));
    assert_eq!(document_snapshot(&reopened.state), before);
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
    if let Some(dest) = std::env::var_os("T829_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        for n in [100, 1000, 10000] {
            let (s, _, _) = animated(n, 1, false);
            ankhimate_formats::save(
                &dest.join(format!("Keys-{n}.ankh")),
                &s.doc.as_project_ref(),
                &[],
            )
            .unwrap();
        }
    }
}
