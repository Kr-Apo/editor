use super::*;
use ankhimate_document::{
    Document,
    commands::key_cmds::{BoneProperty, TimelineAddr},
};
mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t827_fixture.rs"
    ));
}
fn state(d: Document, a: AnimationId) -> AppState {
    let mut s = AppState::default();
    s.replace_document(d);
    s.session.active_animation = Some(a);
    s.set_work_mode(crate::session::WorkMode::Animate);
    s
}
#[test]
fn t827_optional_editor_reference_and_measurement_probe() {
    let Some(path) = std::env::var_os("T827_EDITOR_RESULT") else {
        return;
    };
    let cases=fixture::cases().into_iter().map(|(name,d,a,r)|{let mut s=state(d,a);if let Some(first)=r.first(){set(&mut s,a,vec![first.clone()]);}set(&mut s,a,r);serde_json::json!({"name":name,"selected":format!("{:?}",read(&mut s,a)),"undo":s.history.undo_depth(),"redo":s.history.can_redo(),"revision":s.revision})}).collect::<Vec<_>>();
    let mut measures = vec![];
    for n in [1, 10, 100, 1000, 2000, 5000, 10000, 20000] {
        let t = std::time::Instant::now();
        let (d, a, r) = fixture::fixture(n, 1, false);
        let construction = t.elapsed().as_secs_f64() * 1000.;
        let mut s = state(d, a);
        set(&mut s, a, r.clone());
        let mut set_ms = vec![];
        let mut read_ms = vec![];
        let mut refs_ms = vec![];
        let mut update_ms = vec![];
        for _ in 0..3 {
            let t = std::time::Instant::now();
            let input = r.clone();
            refs_ms.push(t.elapsed().as_secs_f64() * 1000.);
            let t = std::time::Instant::now();
            set(&mut s, a, input);
            set_ms.push(t.elapsed().as_secs_f64() * 1000.);
            let t = std::time::Instant::now();
            assert_eq!(read(&mut s, a), r);
            read_ms.push(t.elapsed().as_secs_f64() * 1000.);
            let selected = s.session.key_selection.clone();
            let t = std::time::Instant::now();
            s.session.key_selection = selected;
            update_ms.push(t.elapsed().as_secs_f64() * 1000.);
        }
        measures.push(serde_json::json!({"keys":n,"constructionMs":construction,"referenceCollectionMs":refs_ms,"setMs":set_ms,"readMs":read_ms,"publishPrebuiltSelectionMs":update_ms}));
    }
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&serde_json::json!({"cases":cases,"measures":measures})).unwrap(),
    )
    .unwrap();
}

use ankhimate_document::{
    clipboard::Clipboard,
    commands::{bone_cmds::SetBoneTransform, key_cmds::DeleteKeys},
    snapshot::snapshot,
};
fn with_redo(
    n: usize,
) -> (
    AppState,
    AnimationId,
    Vec<KeyRef>,
    ankhimate_document::snapshot::Snapshot,
) {
    let (d, a, r) = fixture::fixture(n, 1, false);
    let mut s = state(d, a);
    let bone = s.doc.skeleton.bones.keys().next().unwrap();
    s.set_work_mode(crate::session::WorkMode::Setup);
    let mut local = s.doc.skeleton.bones[bone].local_transform;
    local.position.x += 5.;
    assert!(s.dispatch(Box::new(SetBoneTransform::new(bone, local))));
    let after = snapshot(&s.doc);
    s.undo();
    (s, a, r, after)
}
#[test]
fn t827_large_selection_preserves_redo_and_three_native_cycles() {
    let (mut s, a, r, after) = with_redo(10000);
    let before = snapshot(&s.doc);
    let rev = s.revision;
    let depth = s.history.undo_depth();
    set(&mut s, a, r.clone());
    assert_eq!(read(&mut s, a), r);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), depth);
    assert!(s.history.can_redo());
    assert!(snapshot(&s.doc) == before);
    for _ in 0..3 {
        s.redo();
        assert!(snapshot(&s.doc) == after);
        s.undo();
        assert!(snapshot(&s.doc) == before);
    }
}
#[test]
fn t827_invalid_empty_additive_and_stale_selection_preserve_history() {
    for mode in 0..5 {
        let (mut s, a, r, after) = with_redo(10000);
        let rev = s.revision;
        set(&mut s, a, vec![r[0].clone()]);
        let mut input = r.clone();
        match mode {
            0 => input[5000].index = usize::MAX,
            1 => input.last_mut().unwrap().index = usize::MAX,
            2 => input.clear(),
            3 => input.insert(0, r[0].clone()),
            _ => {
                s.revision += 1;
                assert!(read(&mut s, a).is_empty());
            }
        }
        if mode < 4 {
            set(&mut s, a, input.clone());
            if mode < 3 {
                assert!(read(&mut s, a).is_empty());
            } else {
                assert_eq!(read(&mut s, a), input);
            }
        }
        assert!(s.history.can_redo());
        if mode < 4 {
            assert_eq!(s.revision, rev);
        }
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t827_read_revalidates_same_revision_direct_key_and_track_changes() {
    for mode in 0..5 {
        let (d, a, r) = fixture::fixture(1000, 1, false);
        let mut s = state(d, a);
        set(&mut s, a, r.clone());
        match mode {
            0 => {
                if let ankhimate_core::animation::Timeline::BoneRotate { keys, .. } =
                    &mut s.doc.animations[a].timelines[0]
                {
                    keys[500].value += 1.;
                }
            }
            1 => s.doc.animations[a].timelines.clear(),
            2 => {
                let duplicate = s.doc.animations[a].timelines[0].clone();
                s.doc.animations[a].timelines.push(duplicate);
            }
            3 => {
                s.doc.animations.remove(a);
            }
            _ => {
                let clone = s.doc.animations[a].clone();
                let b = s.doc.animations.insert(clone);
                s.session.active_animation = Some(b);
            }
        }
        assert!(read(&mut s, a).is_empty());
    }
}
#[test]
fn t827_locked_bone_and_setup_keep_native_edit_guards() {
    let (d, a, r) = fixture::fixture(20, 1, false);
    let mut s = state(d, a);
    let bone = s.doc.skeleton.bones.keys().next().unwrap();
    s.session.locked_bones.insert(bone, true);
    set(&mut s, a, r.clone());
    assert_eq!(read(&mut s, a), r);
    let before = snapshot(&s.doc);
    assert!(move_keys(&mut s, a, &r, 0.25).is_none());
    assert!(snapshot(&s.doc) == before);
    s.set_work_mode(crate::session::WorkMode::Setup);
    let args = crate::key_clipboard::selection_args(&s, a, &r).unwrap();
    crate::key_clipboard::copy(&mut s, &args);
    assert!(!matches!(s.session.clipboard, Clipboard::Keys(_)));
}
#[test]
fn t827_copy_paste_delete_and_move_consumers_keep_owned_payloads() {
    let (d, a, r) = fixture::fixture(30, 1, false);
    let mut s = state(d, a);
    let original = KeyClip::capture(&s.doc, a, &r).unwrap();
    let args = crate::key_clipboard::selection_args(&s, a, &r).unwrap();
    crate::key_clipboard::copy(&mut s, &args);
    assert!(matches!(&s.session.clipboard,Clipboard::Keys(c)if c==&original));
    let b = s
        .doc
        .animations
        .insert(ankhimate_core::animation::Animation::new(
            "destination",
            40.,
        ));
    s.session.active_animation = Some(b);
    s.session.playhead = 5.;
    let args = crate::key_clipboard::selection_args(&s, b, &[]).unwrap();
    let before = snapshot(&s.doc);
    crate::key_clipboard::paste(&mut s, &args);
    let after = snapshot(&s.doc);
    assert!(after != before);
    s.undo();
    assert!(snapshot(&s.doc) == before);
    s.session.clipboard = Clipboard::default();
    for _ in 0..3 {
        s.redo();
        assert!(snapshot(&s.doc) == after);
        s.undo();
        assert!(snapshot(&s.doc) == before);
    }
    s.session.active_animation = Some(a);
    set(&mut s, a, vec![r[1].clone(), r[3].clone()]);
    let selected = read(&mut s, a);
    assert!(move_keys(&mut s, a, &selected, 0.25).is_some());
    s.undo();
    assert!(snapshot(&s.doc) == before);
    assert!(s.dispatch(Box::new(DeleteKeys::new(a, r.clone()))));
    s.undo();
    assert!(snapshot(&s.doc) == before);
    assert_eq!(KeyClip::capture(&s.doc, a, &r).unwrap(), original);
}
#[allow(
    dead_code,
    reason = "Reuse existing native weighted fixture, not historical projects"
)]
mod assets {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
fn ffd_state() -> (AppState, AnimationId, KeyRef) {
    use ankhimate_core::animation::{Key, Timeline};
    let (d, skin, slot) = assets::fixture(true);
    let a = d.animations.keys().next().unwrap();
    let mut s = state(d, a);
    s.session.active_skin = skin;
    s.doc.animations[a].timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![Key::linear(0., vec![glam::Vec2::ZERO; 8])],
    });
    let key = KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index: 0,
    };
    (s, a, key)
}
#[test]
fn t827_single_ffd_lease_and_mixed_refusal_remain_native() {
    let (mut s, a, key) = ffd_state();
    set(&mut s, a, vec![key.clone()]);
    assert_eq!(read(&mut s, a), vec![key.clone()]);
    let scalar = KeyRef {
        addr: ankhimate_document::clipboard::keys::address(&s.doc.animations[a].timelines[0])
            .unwrap(),
        index: 0,
    };
    set(&mut s, a, vec![scalar, key.clone()]);
    assert_eq!(read(&mut s, a), vec![key.clone()]);
    if let ankhimate_core::animation::Timeline::Deform { keys, .. } =
        s.doc.animations[a].timelines.last_mut().unwrap()
    {
        keys[0].value.pop();
    }
    assert!(read(&mut s, a).is_empty());
    set(&mut s, a, vec![key]);
    assert!(read(&mut s, a).is_empty());
}
#[test]
fn t827_selection_save_reload_and_native_export_preserve_data_and_assets() {
    let (mut s, a, _) = ffd_state();
    let bone = s.doc.skeleton.bones.keys().next().unwrap();
    s.doc.animations[a]
        .timelines
        .push(ankhimate_core::animation::Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: (0..10000)
                .map(|i| ankhimate_core::animation::Key::linear(i as f32 * 0.001, i as f32 * 0.125))
                .collect(),
        });
    let addr = TimelineAddr::Bone {
        bone,
        property: BoneProperty::Translate,
        axis: Some(ankhimate_core::animation::Axis::X),
    };
    let refs = (0..10000)
        .map(|index| KeyRef {
            addr: addr.clone(),
            index,
        })
        .collect::<Vec<_>>();
    let preset = ankhimate_export::presets::default_preset();
    let before_export = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    let before = snapshot(&s.doc);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("selection.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    set(&mut s, a, refs.clone());
    assert_eq!(read(&mut s, a), refs);
    assert!(snapshot(&s.doc) == before);
    let after_export = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    assert_eq!(before_export.files, after_export.files);
    assert_eq!(before_export.binaries, after_export.binaries);
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    assert_eq!(bytes, std::fs::read(&path).unwrap());
    let loaded = ankhimate_formats::load(&path).unwrap().0;
    assert!(loaded.report.lossy.is_empty() && loaded.report.dangling.is_empty());
    let d = Document::from_loaded(loaded);
    let a = d
        .animations
        .values()
        .find(|clip| clip.name == "kept")
        .unwrap();
    assert!(a.timelines.iter().any(|t|matches!(t,ankhimate_core::animation::Timeline::BoneTranslate{keys,..}if keys.len()==10000)));
    assert_eq!(
        d.assets.images.values().next().unwrap().bytes,
        s.doc.assets.images.values().next().unwrap().bytes
    );
}
