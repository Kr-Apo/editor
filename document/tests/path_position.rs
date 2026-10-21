#[allow(dead_code, reason = "Shared synthetic Path fixture")]
mod support {
    include!("../../core/tests/support/path_position_fixture.rs");
}
use ankhimate_core::{
    animation::{Interp, Timeline},
    constraints::{Constraint, IkConstraint},
};
use ankhimate_document::{
    WorkMode,
    commands::{
        constraint_cmds::{RemoveConstraint, SetConstraintOrder},
        key_cmds::RetimeAnimation,
    },
    edit::Edit,
    path_keys::{Action, Capture},
    snapshot::snapshot,
};
fn rig() -> (
    Edit,
    ankhimate_core::ids::AnimationId,
    ankhimate_core::ids::ConstraintId,
) {
    let (s, a, id, _) = support::fixture(false, true);
    let mut e = Edit::default();
    e.doc.skeleton = s;
    e.mode = WorkMode::Animate;
    let animation = e.doc.animations.insert(a);
    (e, animation, id)
}
fn put(
    e: &mut Edit,
    a: ankhimate_core::ids::AnimationId,
    c: ankhimate_core::ids::ConstraintId,
    t: f32,
    v: f32,
) {
    let cap = Capture::read(&e.doc, a, c).unwrap();
    if let Some(cmd) = cap
        .command(
            &e.doc,
            Action::Put {
                time: t,
                value: v,
                interp: Interp::Linear,
            },
        )
        .unwrap()
    {
        e.dispatch(Box::new(cmd)).unwrap();
    }
}
#[test]
fn t831_native_put_replace_delete_and_three_history_cycles() {
    let (mut e, a, c) = rig();
    let before = snapshot(&e.doc);
    put(&mut e, a, c, 0.25, 0.1);
    let after = snapshot(&e.doc);
    assert_eq!(e.history.undo_depth(), 1);
    for _ in 0..3 {
        e.undo();
        assert!(snapshot(&e.doc) == before);
        e.redo();
        assert!(snapshot(&e.doc) == after);
    }
    let cap = Capture::read(&e.doc, a, c).unwrap();
    let cmd = cap
        .command(
            &e.doc,
            Action::Replace {
                index: 1,
                time: 0.3,
                value: 0.2,
                interp: Interp::Stepped,
            },
        )
        .unwrap()
        .unwrap();
    e.dispatch(Box::new(cmd)).unwrap();
    let keys = Capture::read(&e.doc, a, c).unwrap().keys;
    assert_eq!(
        (keys[1].time, keys[1].value, keys[1].interp),
        (0.3, 0.2, Interp::Stepped)
    );
    let cmd = Capture::read(&e.doc, a, c)
        .unwrap()
        .command(&e.doc, Action::Delete { index: 1 })
        .unwrap()
        .unwrap();
    e.dispatch(Box::new(cmd)).unwrap();
    assert_eq!(Capture::read(&e.doc, a, c).unwrap().keys.len(), 3);
}
#[test]
fn t831_native_noop_invalid_values_collisions_preserve_redo() {
    let (mut e, a, c) = rig();
    put(&mut e, a, c, 0.25, 0.1);
    e.undo();
    let before = snapshot(&e.doc);
    let cap = Capture::read(&e.doc, a, c).unwrap();
    assert!(
        cap.command(
            &e.doc,
            Action::Put {
                time: 0.,
                value: 0.,
                interp: Interp::Linear
            }
        )
        .unwrap()
        .is_none()
    );
    for (t, v) in [(f32::NAN, 0.), (-1., 0.), (0., f32::INFINITY)] {
        assert!(
            cap.command(
                &e.doc,
                Action::Put {
                    time: t,
                    value: v,
                    interp: Interp::Linear
                }
            )
            .is_err()
        );
    }
    assert!(
        cap.command(
            &e.doc,
            Action::Replace {
                index: 0,
                time: 0.5,
                value: 0.,
                interp: Interp::Linear
            }
        )
        .is_err()
    );
    assert!(cap.command(&e.doc, Action::Delete { index: 99 }).is_err());
    assert!(snapshot(&e.doc) == before);
    assert!(e.history.can_redo());
}
#[test]
fn t831_native_setup_refusal_and_duration_restoration() {
    let (mut e, a, c) = rig();
    let cap = Capture::read(&e.doc, a, c).unwrap();
    e.mode = WorkMode::Setup;
    assert!(
        e.dispatch(Box::new(
            cap.command(
                &e.doc,
                Action::Put {
                    time: 2.,
                    value: 1.,
                    interp: Interp::Linear
                }
            )
            .unwrap()
            .unwrap()
        ))
        .is_err()
    );
    assert_eq!(e.history.undo_depth(), 0);
    e.mode = WorkMode::Animate;
    put(&mut e, a, c, 2., 1.);
    assert_eq!(e.doc.animations[a].duration, 2.);
    e.undo();
    assert_eq!(e.doc.animations[a].duration, 1.);
    e.redo();
    assert_eq!(e.doc.animations[a].duration, 2.);
}
#[test]
fn t831_native_deleted_constraint_undo_restores_exact_ids_and_prior_key_history() {
    let (mut e, a, c) = rig();
    put(&mut e, a, c, 0.25, 0.15);
    let keyed = snapshot(&e.doc);
    let prior = Capture::read(&e.doc, a, c).unwrap();
    e.mode = WorkMode::Setup;
    e.dispatch(Box::new(RemoveConstraint::new(c))).unwrap();
    assert!(!e.doc.skeleton.constraints.contains_key(c));
    assert!(!prior.matches(&e.doc));
    let removed = snapshot(&e.doc);
    for _ in 0..3 {
        e.undo();
        assert!(e.doc.skeleton.constraints.contains_key(c));
        assert!(snapshot(&e.doc) == keyed);
        assert!(prior.matches(&e.doc));
        e.redo();
        assert!(snapshot(&e.doc) == removed);
    }
    e.undo();
    e.undo();
    assert_eq!(Capture::read(&e.doc, a, c).unwrap().keys.len(), 3);
    e.redo();
    assert!(snapshot(&e.doc) == keyed);
}
#[test]
fn t831_native_reorder_identity_homonym_wrong_type_and_stale_reference() {
    let (mut e, a, c) = rig();
    let bones = if let Constraint::Path(p) = &e.doc.skeleton.constraints[c] {
        p.bones.clone()
    } else {
        panic!()
    };
    let other = e
        .doc
        .skeleton
        .add_constraint(Constraint::Ik(IkConstraint::chain(
            "other",
            bones[0],
            bones.clone(),
        )));
    let capture = Capture::read(&e.doc, a, c).unwrap();
    e.mode = WorkMode::Setup;
    e.dispatch(Box::new(SetConstraintOrder::new(vec![other, c])))
        .unwrap();
    assert!(!capture.matches(&e.doc));
    assert_eq!(Capture::read(&e.doc, a, c).unwrap().constraint, c);
    e.undo();
    assert!(capture.matches(&e.doc));
    assert!(Capture::read(&e.doc, a, other).is_err());
    let path = e.doc.skeleton.constraints[c].clone();
    e.doc.skeleton.add_constraint(path);
    assert!(Capture::read(&e.doc, a, c).is_err());
    e.doc.skeleton.remove_constraint(c);
    let new = e
        .doc
        .skeleton
        .add_constraint(Constraint::Ik(IkConstraint::chain(
            "replacement",
            bones[0],
            bones,
        )));
    assert_ne!(new, c);
    assert!(Capture::read(&e.doc, a, c).is_err());
}
#[test]
fn t831_native_duplicate_track_disabled_missing_bone_slot_and_bad_path_refused() {
    for change in 0..6 {
        let (mut e, a, c) = rig();
        match change {
            0 => {
                let t = e.doc.animations[a].timelines[0].clone();
                e.doc.animations[a].timelines.push(t);
            }
            1 => {
                if let Constraint::Path(p) = &mut e.doc.skeleton.constraints[c] {
                    p.bones.clear();
                }
            }
            2 => {
                let slot = if let Constraint::Path(p) = &e.doc.skeleton.constraints[c] {
                    p.slot
                } else {
                    panic!()
                };
                e.doc.skeleton.slots.remove(slot);
            }
            3 => e.doc.skeleton.skins[e.doc.skeleton.default_skin]
                .entries
                .clear(),
            4 => {
                if let Constraint::Path(p) = &mut e.doc.skeleton.constraints[c] {
                    p.position = f32::NAN;
                }
            }
            _ => {
                if let Constraint::Path(p) = &mut e.doc.skeleton.constraints[c] {
                    p.mix_translate = 0.;
                    p.mix_rotate = 0.;
                }
            }
        }
        if change == 5 {
            let cap = Capture::read(&e.doc, a, c).unwrap();
            assert!(
                cap.command(
                    &e.doc,
                    Action::Put {
                        time: 0.25,
                        value: 0.1,
                        interp: Interp::Linear
                    }
                )
                .is_err()
            );
        } else {
            assert!(Capture::read(&e.doc, a, c).is_err(), "case {change}");
        }
    }
}
#[test]
fn t831_retime_includes_path_position_and_restores_full_snapshot() {
    let (mut e, a, c) = rig();
    let before = snapshot(&e.doc);
    let p = RetimeAnimation::prepare_numeric(a, &e.doc.animations[a], 2., 0.).unwrap();
    assert_eq!(p.summary.keys, 3);
    assert_eq!(p.summary.duration_after, 2.);
    e.dispatch(Box::new(p.command)).unwrap();
    assert_eq!(
        Capture::read(&e.doc, a, c)
            .unwrap()
            .keys
            .iter()
            .map(|k| k.time)
            .collect::<Vec<_>>(),
        vec![0., 1., 2.]
    );
    let after = snapshot(&e.doc);
    for _ in 0..3 {
        e.undo();
        assert!(snapshot(&e.doc) == before);
        e.redo();
        assert!(snapshot(&e.doc) == after);
    }
}
#[test]
fn t831_other_animation_setup_and_geometry_unchanged_by_key_actions() {
    let (mut e, a, c) = rig();
    let other = e.doc.animations.insert(e.doc.animations[a].clone());
    let prior = e.doc.animations[other].clone();
    let setup = serde_json::to_value(&e.doc.skeleton.constraints[c]).unwrap();
    let skin = e.doc.skeleton.default_skin;
    let entries =
        serde_json::to_value(e.doc.skeleton.skins[skin].entries.values().next().unwrap()).unwrap();
    put(&mut e, a, c, 0.7, -0.4);
    assert_eq!(e.doc.animations[other], prior);
    assert_eq!(
        serde_json::to_value(&e.doc.skeleton.constraints[c]).unwrap(),
        setup
    );
    assert_eq!(
        serde_json::to_value(e.doc.skeleton.skins[skin].entries.values().next().unwrap()).unwrap(),
        entries
    );
    if let Timeline::PathPosition { keys, .. } = &e.doc.animations[a].timelines[0] {
        assert!(keys.iter().any(|k| k.value == -0.4));
    }
}

#[test]
fn t831_first_late_key_seeds_setup_once_in_one_undo() {
    let (mut e, a, c) = rig();
    e.doc.animations[a].timelines.clear();
    let before = snapshot(&e.doc);
    put(&mut e, a, c, 0.5, 0.7);
    let keys = Capture::read(&e.doc, a, c).unwrap().keys;
    assert_eq!(keys.len(), 2);
    assert_eq!((keys[0].time, keys[0].value), (0., 0.));
    assert_eq!(e.history.undo_depth(), 1);
    e.undo();
    assert!(snapshot(&e.doc) == before);
}

#[test]
fn t831_native_ambiguous_named_references_refuse_without_history() {
    for kind in 0..4 {
        let (mut e, a, c) = rig();
        match kind {
            0 => {
                let b = e.doc.skeleton.bones.values().next().unwrap().clone();
                e.doc.skeleton.bones.insert(b);
            }
            1 => {
                let s = e.doc.skeleton.slots.values().next().unwrap().clone();
                e.doc.skeleton.slots.insert(s);
            }
            2 => {
                let s = e.doc.skeleton.skins[e.doc.skeleton.default_skin].clone();
                e.doc.skeleton.skins.insert(s);
            }
            _ => {
                let root = e.doc.skeleton.slots.values().next().unwrap().bone;
                e.doc.skeleton.bones[root].parent = Some(root);
            }
        }
        let before = snapshot(&e.doc);
        assert!(Capture::read(&e.doc, a, c).is_err());
        assert!(snapshot(&e.doc) == before);
        assert!(!e.history.can_undo());
    }
}
