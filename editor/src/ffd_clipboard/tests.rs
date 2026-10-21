use super::*;
use crate::session::WorkMode;
use ankhimate_core::animation::{Key, Timeline};
use ankhimate_document::{
    clipboard::{
        KeyClip,
        keys::{CopiedKey, KeyData, NamedTrack, TrackKind},
    },
    commands::{
        document_cmds::SetProjectFps,
        key_cmds::{BoneProperty, TimelineAddr},
    },
    snapshot::snapshot,
};
#[allow(dead_code, reason = "Shared real native weighted FFD test fixture")]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t834_fixture.rs"
    ));
}
fn state(weighted: bool) -> (AppState, AnimationId, AnimationId, Vec<KeyRef>) {
    let r = rig::fixture(weighted);
    let refs = r.refs(&[0, 1, 2]);
    let source = r.source;
    let dest = r.destination;
    let mut s = AppState::default();
    s.replace_document(r.doc);
    s.session.work_mode = WorkMode::Animate;
    s.session.active_animation = Some(source);
    s.refresh_pose();
    key_selection::set_sheet(&mut s, source, refs.clone());
    (s, source, dest, refs)
}
fn copied(weighted: bool) -> (AppState, AnimationId, Vec<KeyRef>) {
    let (mut s, source, dest, refs) = state(weighted);
    copy(&mut s, source, &refs).unwrap();
    s.session.active_animation = Some(dest);
    s.set_playhead(2.);
    let target = vec![KeyRef {
        addr: refs[0].addr.clone(),
        index: 0,
    }];
    key_selection::set(&mut s, dest, target.clone());
    (s, dest, target)
}
fn scalar_clip() -> KeyClip {
    KeyClip {
        keys: vec![CopiedKey {
            track: NamedTrack {
                name: "root".into(),
                kind: TrackKind::Bone(BoneProperty::Rotate, None),
            },
            key: Key::linear(0., KeyData::Scalar(1.)),
        }],
    }
}
#[test]
fn t834_copy_group_is_session_only_and_refusal_keeps_previous_payload() {
    let (mut s, a, _, refs) = state(true);
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
    s.undo();
    key_selection::set_sheet(&mut s, a, refs.clone());
    let before = snapshot(&s.doc);
    let rev = s.revision;
    let depth = s.history.undo_depth();
    copy(&mut s, a, &refs).unwrap();
    let Clipboard::Deform(original) = s.session.clipboard.clone() else {
        panic!()
    };
    assert_eq!(original.keys().len(), 3);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), depth);
    assert!(s.history.can_redo());
    let mut mixed = refs.clone();
    mixed[0].addr = TimelineAddr::Bone {
        bone: s.doc.skeleton.bones.keys().next().unwrap(),
        property: BoneProperty::Rotate,
        axis: None,
    };
    assert!(copy(&mut s, a, &mixed).is_err());
    assert!(matches!(&s.session.clipboard,Clipboard::Deform(p)if Arc::ptr_eq(p,&original)));
    s.session.clipboard = Clipboard::Keys(scalar_clip());
    assert!(copy(&mut s, a, &[]).is_err());
    assert!(matches!(&s.session.clipboard,Clipboard::Keys(p)if p==&scalar_clip()));
}
#[test]
fn t834_group_lease_stale_source_data_or_revision_clears_selection() {
    for change in 0..4 {
        let (mut s, a, _, refs) = state(false);
        assert_eq!(key_selection::read(&mut s, a), refs);
        match change {
            0 => s.revision += 1,
            1 => s.session.active_animation = None,
            2 => {
                if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
                    keys[1].value[0].x += 1.;
                }
            }
            _ => s.session.work_mode = WorkMode::Setup,
        }
        assert!(key_selection::read(&mut s, a).is_empty());
    }
}
#[test]
fn t834_paste_context_every_stale_change_preserves_document_and_redo() {
    for change in 0..12 {
        let (mut s, a, refs) = copied(true);
        s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
        s.undo();
        key_selection::set(&mut s, a, refs.clone());
        let mut d = Draft::new(&mut s, a, &refs).unwrap();
        match change {
            0 => s.revision += 1,
            1 => s.session.active_animation = None,
            2 => s.session.work_mode = WorkMode::Setup,
            3 => s.session.playhead += 0.1,
            4 => s.session.clipboard = Clipboard::Empty,
            5 => {
                if let Clipboard::Deform(c) = &s.session.clipboard {
                    s.session.clipboard = Clipboard::Deform(Arc::new((**c).clone()));
                }
            }
            6 => {
                let b = d.stamp.locks[0].0;
                s.session.locked_bones.insert(b, true);
            }
            7 => key_selection::set(&mut s, a, vec![]),
            8 => {
                let doc = s.doc.clone();
                s.replace_document(doc);
            }
            9 => {
                s.doc.animations[a].timelines.clear();
            }
            10 => {
                if let TimelineAddr::Deform {
                    slot,
                    owner: Some(skin),
                    ..
                } = &refs[0].addr
                    && let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[*skin]
                        .entries
                        .get_mut(&(*slot, "mesh".into()))
                {
                    m.setup_vertices.swap(0, 1);
                }
            }
            _ => {
                s.session.active_skin = SkinId::default();
            }
        }
        let before = snapshot(&s.doc);
        let rev = s.revision;
        let depth = s.history.undo_depth();
        let redo = s.history.can_redo();
        assert!(!d.apply(&mut s), "change{change}");
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.revision, rev);
        assert_eq!(s.history.undo_depth(), depth);
        assert_eq!(s.history.can_redo(), redo);
    }
}
#[test]
fn t834_one_apply_three_cycles_clear_clipboard_and_no_merge() {
    let (mut s, a, refs) = copied(true);
    let before = snapshot(&s.doc);
    let mut d = Draft::new(&mut s, a, &refs).unwrap();
    assert!(d.apply(&mut s));
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 1);
    assert!(!d.apply(&mut s));
    s.session.clipboard = Clipboard::Empty;
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t834_outside_duration_requires_explicit_acceptance_and_locks_refuse() {
    let (mut s, a, refs) = copied(true);
    s.set_playhead(5.);
    let mut d = Draft::new(&mut s, a, &refs).unwrap();
    assert_eq!(d.plan.as_ref().unwrap().summary.outside_duration, 3);
    assert!(!d.apply(&mut s));
    d.ack = true;
    assert!(d.apply(&mut s));
    assert_eq!(s.doc.animations[a].duration, 4.);
    let (mut s, a, refs) = copied(true);
    let TimelineAddr::Deform {
        slot,
        owner: Some(skin),
        ..
    } = refs[0].addr
    else {
        panic!()
    };
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin].get(slot, "mesh").unwrap() else {
        panic!()
    };
    let b = m.weights[0][1].bone;
    s.session.locked_bones.insert(b, true);
    assert!(Draft::new(&mut s, a, &refs).is_err());
}
#[test]
fn t834_mixed_group_and_missing_explicit_destination_refuse() {
    let (mut s, a, refs) = copied(false);
    assert!(Draft::new(&mut s, a, &[]).is_err());
    let mut mixed = refs.clone();
    mixed.push(KeyRef {
        addr: TimelineAddr::Bone {
            bone: s.doc.skeleton.bones.keys().next().unwrap(),
            property: BoneProperty::Rotate,
            axis: None,
        },
        index: 0,
    });
    assert!(Draft::new(&mut s, a, &mixed).is_err());
    let before = key_selection::read(&mut s, a);
    key_selection::set_sheet(&mut s, a, mixed);
    assert_eq!(key_selection::read(&mut s, a), before);
}
#[test]
fn t834_cross_document_payload_cannot_be_reused_even_with_cloned_mesh() {
    let (mut s, a, refs) = copied(false);
    let payload = s.session.clipboard.clone();
    let doc = s.doc.clone();
    s.replace_document(doc);
    s.session.clipboard = payload;
    s.session.work_mode = WorkMode::Animate;
    s.session.active_animation = Some(a);
    s.set_playhead(2.);
    key_selection::set(&mut s, a, refs.clone());
    assert!(Draft::new(&mut s, a, &refs).is_err());
}
