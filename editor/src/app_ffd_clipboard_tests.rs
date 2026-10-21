use super::*;
use ankhimate_core::animation::{Key, Timeline};
use ankhimate_document::{
    clipboard::deform::{DeformClip, PreparedDeformPaste},
    commands::key_cmds::{KeyRef, PreparedBoneOffset, TimelineAddr},
};
use std::sync::Arc;
#[allow(
    dead_code,
    reason = "Shared synthetic weighted mesh plus real asset bytes"
)]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t834_fixture.rs"
    ));
}
#[test]
fn t834_app_dirty_save_as_failed_save_reload_assets_sampling_and_native_v2() {
    let r = rig::fixture(true);
    let source = r.source;
    let target = r.destination;
    let slot = r.slot;
    let skin = r.skin;
    let refs = r.refs(&[0, 1, 2]);
    let mut app = AnkhimateApp::default();
    app.state.replace_document(r.doc);
    app.state.session.work_mode = crate::session::WorkMode::Animate;
    app.state.session.active_animation = Some(source);
    app.state.refresh_pose();
    crate::key_selection::set_sheet(&mut app.state, source, refs.clone());
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("Before.ankh");
    app.save_project_to(&first);
    assert!(!app.is_dirty());
    let old = std::fs::read(&first).unwrap();
    let before = ankhimate_document::snapshot::snapshot(&app.state.doc);
    let revision = app.state.revision;
    crate::ffd_clipboard::copy(&mut app.state, source, &refs).unwrap();
    assert!(!app.is_dirty());
    assert_eq!(app.state.revision, revision);
    let ankhimate_document::clipboard::Clipboard::Deform(p) = app.state.session.clipboard.clone()
    else {
        panic!()
    };
    let prepared = PreparedDeformPaste::prepare(
        &app.state.doc,
        app.state.diagnostics.document_identity(),
        target,
        p.clone(),
        &refs[0].addr,
        2.,
    )
    .unwrap();
    let c = prepared
        .into_command(
            &app.state.doc,
            app.state.diagnostics.document_identity(),
            &p,
        )
        .unwrap();
    assert!(app.state.dispatch(c));
    let after = ankhimate_document::snapshot::snapshot(&app.state.doc);
    assert!(app.is_dirty());
    assert_eq!(app.state.history.undo_depth(), 1);
    for _ in 0..3 {
        app.state.undo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == before);
        assert!(!app.is_dirty());
        app.state.redo();
        assert!(ankhimate_document::snapshot::snapshot(&app.state.doc) == after);
    }
    app.save_project_to(dir.path());
    assert!(app.is_dirty());
    assert_eq!(app.current_path, Some(first.clone()));
    assert_eq!(std::fs::read(&first).unwrap(), old);
    let file = dir.path().join("FFD-collé.ankh");
    app.save_project_to(&file);
    assert!(!app.is_dirty());
    let expected = document_snapshot(&app.state);
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(file));
    assert_eq!(document_snapshot(&reopened.state), expected);
    assert!(!reopened.is_dirty());
    let id = reopened
        .state
        .doc
        .animations
        .iter()
        .find(|(_, a)| a.name == "FFD destination")
        .unwrap()
        .0;
    let sl = reopened
        .state
        .doc
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "slot")
        .unwrap()
        .0;
    for t in [2., 2.25, 2.5, 2.75, 3.] {
        let mut a = ankhimate_core::pose::Pose::default();
        let mut b = ankhimate_core::pose::Pose::default();
        ankhimate_core::pose::evaluate(
            &app.state.doc.skeleton,
            &[(&app.state.doc.animations[target], t, 1.)],
            &mut a,
        );
        ankhimate_core::pose::evaluate(
            &reopened.state.doc.skeleton,
            &[(&reopened.state.doc.animations[id], t, 1.)],
            &mut b,
        );
        assert_eq!(
            a.deforms[&(slot, "mesh".into())],
            b.deforms[&(sl, "mesh".into())]
        );
    }
    assert_eq!(
        app.state.doc.assets.images.values().next().unwrap().bytes,
        reopened
            .state
            .doc
            .assets
            .images
            .values()
            .next()
            .unwrap()
            .bytes
    );
    let schema = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    let plan = ankhimate_export::run::plan(&schema, &app.state.doc.assets, &preset).unwrap();
    for f in &plan.files {
        let value: serde_json::Value = serde_json::from_str(&f.contents).unwrap();
        assert_eq!(value["format_version"], 2);
    }
    let binds = if let ankhimate_core::attachment::Attachment::Mesh(m) =
        app.state.doc.skeleton.skins[skin]
            .get(slot, "mesh")
            .unwrap()
    {
        m.inverse_bind_matrices.clone()
    } else {
        panic!()
    };
    assert!(!binds.is_empty());
    if let Some(dest) = std::env::var_os("T834_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        app.save_project_to(&dest.join("FFD-Pasted.ankh"));
        ankhimate_export::run::write(&plan, &dest.join("Native-v2")).unwrap();
        app.state.undo();
        app.save_project_to(&dest.join("FFD-Before.ankh"));
    }
}
#[allow(
    dead_code,
    reason = "Real T831 Path fixture; preserve its established runtime refusal"
)]
mod native_path {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/path_document_fixture.rs"
    ));
}
#[test]
fn t834_path_position_and_selected_bone_offset_scope_are_preserved() {
    use ankhimate_core::{
        animation::Interp,
        attachment::{Attachment, MeshAttachment},
    };
    let (mut d, id, _) = native_path::path_document();
    let skin = d.skeleton.default_skin;
    let (slot, region) = d.skeleton.skins[skin]
        .entries
        .iter()
        .find_map(|((slot, name), a)| {
            if name == "art" {
                if let Attachment::Region(r) = a {
                    Some((*slot, r.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();
    let m = MeshAttachment::from_region(&region);
    let n = m.setup_vertices.len();
    d.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(m));
    let path = d.animations[id].timelines[0].clone();
    d.animations[id].timelines.push(Timeline::Deform {
        slot,
        attachment: "art".into(),
        keys: vec![
            Key::linear(0.25, vec![glam::Vec2::ZERO; n]),
            Key {
                time: 0.75,
                value: vec![glam::Vec2::ONE; n],
                interp: Interp::Stepped,
            },
        ],
    });
    let bone = d.skeleton.slots[slot].bone;
    d.animations[id].timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![Key::linear(0.25, 2.), Key::linear(0.75, 3.)],
    });
    let addr = TimelineAddr::Deform {
        slot,
        attachment: "art".into(),
        owner: Some(skin),
    };
    let refs = vec![
        KeyRef {
            addr: addr.clone(),
            index: 0,
        },
        KeyRef {
            addr: addr.clone(),
            index: 1,
        },
    ];
    let clip = Arc::new(DeformClip::capture(&d, 99, id, &refs).unwrap());
    let p = PreparedDeformPaste::prepare(&d, 99, id, clip.clone(), &addr, 2.).unwrap();
    let mut h = ankhimate_document::History::default();
    h.push_in_mode(
        p.into_command(&d, 99, &clip).unwrap(),
        &mut d,
        ankhimate_document::WorkMode::Animate,
    );
    assert_eq!(d.animations[id].timelines[0], path);
    let ffd = d.animations[id].timelines[1].clone();
    let p = PreparedBoneOffset::prepare(&d, id, &[bone], 3.).unwrap();
    h.push_in_mode(
        p.into_command(&d).unwrap().unwrap(),
        &mut d,
        ankhimate_document::WorkMode::Animate,
    );
    assert_eq!(d.animations[id].timelines[0], path);
    assert_eq!(d.animations[id].timelines[1], ffd);
    let schema = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    assert!(ankhimate_export::run::plan(&schema, &d.assets, &preset).is_err());
}
