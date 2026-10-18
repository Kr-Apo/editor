//! Exercise the actual editor commit path, including its command history.
use super::*;
use crate::{app_state::AppState, session::WorkMode};
use ankhimate_core::{animation::Key, skeleton::Bone};
use ankhimate_document::commands::{bone_cmds::CreateBone, key_cmds::CreateAnimation};

fn rig(setup: Transform) -> (AppState, BoneId) {
    let mut state = AppState::default();
    state.dispatch(Box::new(CreateBone::new(Bone {
        name: "root".into(),
        parent: None,
        length: 10.0,
        local_transform: setup,
        inherit: Default::default(),
        color: Bone::default_color(),
    })));
    let bone = state.doc.skeleton.update_order[0];
    state.dispatch(Box::new(CreateAnimation::new("return", 3.0)));
    state.set_work_mode(WorkMode::Animate);
    state.session.auto_key = true;
    (state, bone)
}

fn addr(bone: BoneId, property: BoneProperty, axis: Option<Axis>) -> TimelineAddr {
    TimelineAddr::Bone {
        bone,
        property,
        axis,
    }
}

fn key(state: &mut AppState, addr: &TimelineAddr, time: f32, value: f32) {
    state.dispatch(Box::new(AddKey::new(
        state.session.active_animation.unwrap(),
        addr.clone(),
        time,
        KeyValue::Scalar(value),
        Interp::Linear,
    )));
}

fn keys<'a>(state: &'a AppState, addr: &TimelineAddr) -> &'a [Key<f32>] {
    state.doc.animations[state.session.active_animation.unwrap()]
        .timelines
        .iter()
        .find_map(|t| match t {
            Timeline::BoneRotate { keys, .. }
            | Timeline::BoneTranslate { keys, .. }
            | Timeline::BoneScale { keys, .. }
            | Timeline::BoneShear { keys, .. }
                if addr.matches_timeline(t) =>
            {
                Some(keys.as_slice())
            }
            _ => None,
        })
        .unwrap()
}

fn json(state: &AppState) -> String {
    ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
}

fn rotating(setup: Transform) -> (AppState, BoneId, TimelineAddr) {
    let (mut state, bone) = rig(setup);
    let addr = addr(bone, BoneProperty::Rotate, None);
    key(&mut state, &addr, 0.0, 0.0);
    key(&mut state, &addr, 1.0, 45.0);
    state.set_playhead(2.0);
    (state, bone, addr)
}

#[test]
fn autokey_exact_zero_return_creates_a_real_key() {
    let (mut state, bone, addr) = rotating(Transform::default());
    state.commit_bone_pose(bone, Transform::default());
    eprintln!(
        "rotation keys after explicit setup return: {:?}",
        keys(&state, &addr)
    );
    assert_eq!(keys(&state, &addr).len(), 3, "missing return at t=2");
    assert_eq!(keys(&state, &addr)[2], Key::linear(2.0, 0.0));
    assert_eq!(
        state.doc.animations[state.session.active_animation.unwrap()]
            .timelines
            .len(),
        1
    );
}

#[test]
fn autokey_nonzero_setup_return_is_an_offset_of_zero() {
    let setup = Transform {
        rotation: 37.0_f32.to_radians(),
        ..Default::default()
    };
    let (mut state, bone, addr) = rotating(setup);
    state.commit_bone_pose(bone, setup);
    assert_eq!(keys(&state, &addr).last().unwrap(), &Key::linear(2.0, 0.0));
    assert!((state.pose.locals[bone].rotation.to_degrees() - 37.0).abs() < 1e-4);
}

#[test]
fn autokey_each_animated_axis_can_return_without_keying_other_axes() {
    for property in [
        BoneProperty::Translate,
        BoneProperty::Scale,
        BoneProperty::Shear,
    ] {
        for axis in [Axis::X, Axis::Y] {
            let setup = Transform {
                position: glam::vec2(3.0, -7.0),
                scale: glam::vec2(2.0, 3.0),
                shear: glam::vec2(0.2, -0.1),
                ..Default::default()
            };
            let (mut state, bone) = rig(setup);
            let addr = addr(bone, property, Some(axis));
            let rest = if property == BoneProperty::Scale {
                1.0
            } else {
                0.0
            };
            key(&mut state, &addr, 0.0, rest);
            key(&mut state, &addr, 1.0, rest + 2.0);
            state.set_playhead(2.0);
            state.commit_bone_pose(bone, setup);
            assert_eq!(
                keys(&state, &addr).last().unwrap(),
                &Key::linear(2.0, rest),
                "{property:?} {axis:?}"
            );
            assert_eq!(
                state.doc.animations[state.session.active_animation.unwrap()]
                    .timelines
                    .len(),
                1
            );
        }
    }
}

#[test]
fn autokey_setup_noop_does_not_animate_unanimated_or_empty_channels() {
    let (mut state, bone) = rig(Transform::default());
    // Even an empty track is not an animated value to return from.
    state.doc.animations[state.session.active_animation.unwrap()]
        .timelines
        .push(Timeline::BoneRotate { bone, keys: vec![] });
    let before = json(&state);
    let depth = state.history.undo_depth();
    state.set_playhead(2.0);
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
}

#[test]
fn autokey_rotation_return_does_not_rekey_other_animated_rest_channels() {
    let (mut state, bone, rotation) = rotating(Transform::default());
    let position = addr(bone, BoneProperty::Translate, Some(Axis::X));
    key(&mut state, &position, 0.0, 0.0);
    key(&mut state, &position, 1.0, 10.0);
    key(&mut state, &position, 1.5, 0.0);
    let before = keys(&state, &position).to_vec();
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(
        keys(&state, &rotation).last().unwrap(),
        &Key::linear(2.0, 0.0)
    );
    assert_eq!(keys(&state, &position), before);
    assert_eq!(
        state.doc.animations[state.session.active_animation.unwrap()]
            .timelines
            .len(),
        2
    );
}

#[test]
fn autokey_existing_key_is_updated_without_duplicates_and_undo_restores_document() {
    let (mut state, bone, addr) = rotating(Transform::default());
    key(&mut state, &addr, 2.0, 12.0);
    let before = json(&state);
    let depth = state.history.undo_depth();
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(keys(&state, &addr).len(), 3);
    assert_eq!(keys(&state, &addr)[2], Key::linear(2.0, 0.0));
    assert_eq!(state.history.undo_depth(), depth + 1);
    let after = json(&state);
    state.undo();
    assert_eq!(json(&state), before);
    state.redo();
    assert_eq!(json(&state), after);
    // Repeated no-change commits must neither duplicate nor add undo entries.
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(state.history.undo_depth(), depth + 1);
    assert_eq!(json(&state), after);
}

#[test]
fn autokey_disabled_keeps_setup_return_pending_until_manual_key() {
    let (mut state, bone, addr) = rotating(Transform::default());
    state.session.auto_key = false;
    let before = json(&state);
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(json(&state), before);
    assert!(state.session.pending_pose.contains(&bone));
    assert_eq!(state.pose.locals[bone].rotation, 0.0);
    state.key_pending_pose();
    assert!(!state.session.has_pending_pose());
    assert_eq!(keys(&state, &addr)[2], Key::linear(2.0, 0.0));
}

#[test]
fn autokey_setup_mode_still_edits_setup_only() {
    let (mut state, bone, addr) = rotating(Transform::default());
    state.set_work_mode(WorkMode::Setup);
    let before = keys(&state, &addr).to_vec();
    let setup = Transform {
        rotation: 15.0_f32.to_radians(),
        ..Default::default()
    };
    state.commit_bone_pose(bone, setup);
    assert_eq!(state.doc.skeleton.bones[bone].local_transform, setup);
    assert_eq!(keys(&state, &addr), before);
    state.undo();
    assert_eq!(
        state.doc.skeleton.bones[bone].local_transform,
        Transform::default()
    );
}

#[test]
fn autokey_return_roundtrips_and_interpolates_before_and_after() {
    let (mut state, bone, addr) = rotating(Transform::default());
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(keys(&state, &addr).len(), 3);
    let folder = tempfile::tempdir().unwrap();
    let file = folder.path().join("return.ankh");
    ankhimate_formats::save(&file, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&file).unwrap();
    assert!(loaded.report.is_clean());
    let reopened = Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        json(&state)
    );
    let clip = reopened.animations.iter().next().unwrap().1;
    let root = reopened.skeleton.update_order[0];
    for (time, expected) in [
        (0.0, 0.0),
        (0.5, 22.5),
        (1.0, 45.0),
        (1.5, 22.5),
        (2.0, 0.0),
        (2.5, 0.0),
    ] {
        let mut pose = ankhimate_core::pose::Pose::default();
        ankhimate_core::pose::evaluate(&reopened.skeleton, &[(clip, time, 1.0)], &mut pose);
        assert!(
            (pose.locals[root].rotation.to_degrees() - expected).abs() < 1e-4,
            "t={time}"
        );
    }
}

#[test]
fn autokey_equivalent_full_turn_at_rest_is_not_a_change() {
    let (mut state, bone) = rig(Transform::default());
    let addr = addr(bone, BoneProperty::Rotate, None);
    key(&mut state, &addr, 0.0, 360.0);
    state.set_playhead(2.0);
    let before = json(&state);
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(json(&state), before);
}

#[test]
fn autokey_shortest_arc_return_does_not_change_sampling_rules() {
    let (mut state, bone) = rig(Transform::default());
    let addr = addr(bone, BoneProperty::Rotate, None);
    key(&mut state, &addr, 0.0, 170.0);
    key(&mut state, &addr, 1.0, -170.0);
    state.set_playhead(0.5);
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(keys(&state, &addr)[1], Key::linear(0.5, 0.0));
    assert_eq!(keys(&state, &addr)[2], Key::linear(1.0, -170.0));
}

#[test]
fn autokey_return_can_replace_the_first_key_at_zero() {
    let (mut state, bone, addr) = rotating(Transform::default());
    key(&mut state, &addr, 0.0, 20.0);
    state.set_playhead(0.0);
    state.commit_bone_pose(bone, Transform::default());
    assert_eq!(keys(&state, &addr)[0], Key::linear(0.0, 0.0));
    assert_eq!(keys(&state, &addr).len(), 2);
}
