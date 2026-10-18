//! Property-scoped inspector actions. Key creation stays in edit_router/AddKey.
use super::*;
use ankhimate_core::{
    animation::Axis,
    ids::{BoneId, SlotId},
    math::Transform,
    pose::Pose,
};
use ankhimate_document::commands::key_cmds::{AddKey, DeleteKeys, KeyRef, KeyValue};

/// A fresh sample deliberately excludes pending UI overrides and physics steps.
pub(super) fn sampled_pose(state: &AppState) -> Pose {
    let mut pose = Pose::default();
    let animations: Vec<_> = state
        .session
        .active_animation
        .filter(|_| state.session.is_animating())
        .and_then(|id| state.doc.animations.get(id))
        .map(|a| vec![(a, state.session.playhead, 1.0)])
        .unwrap_or_default();
    ankhimate_core::pose::evaluate(&state.doc.skeleton, &animations, &mut pose);
    pose
}

fn component(t: Transform, property: BoneProperty, axis: Option<Axis>) -> f32 {
    let i = axis.unwrap_or(Axis::X).index();
    match property {
        BoneProperty::Rotate => t.rotation,
        BoneProperty::Translate => t.position[i],
        BoneProperty::Scale => t.scale[i],
        BoneProperty::Shear => t.shear[i],
    }
}

fn copy_component(
    target: &mut Transform,
    source: Transform,
    property: BoneProperty,
    axis: Option<Axis>,
) {
    let i = axis.unwrap_or(Axis::X).index();
    match property {
        BoneProperty::Rotate => target.rotation = source.rotation,
        BoneProperty::Translate => target.position[i] = source.position[i],
        BoneProperty::Scale => target.scale[i] = source.scale[i],
        BoneProperty::Shear => target.shear[i] = source.shear[i],
    }
}

pub(super) fn bone_state(
    state: &AppState,
    sampled: &Pose,
    bone: BoneId,
    property: BoneProperty,
    axis: Option<Axis>,
) -> crate::edit_router::KeyState {
    if state.session.pending_pose.contains(&bone)
        && state
            .session
            .preview_locals
            .get(bone)
            .zip(sampled.locals.get(bone))
            .is_some_and(|(pending, sample)| {
                component(*pending, property, axis) != component(*sample, property, axis)
            })
    {
        return crate::edit_router::KeyState::Modified;
    }
    crate::edit_router::key_state(
        &state.doc,
        &state.session,
        &TimelineAddr::Bone {
            bone,
            property,
            axis,
        },
    )
}

/// Remove one override, retaining the other properties, bones and slot previews.
fn clear_bone_property(
    state: &mut AppState,
    bone: BoneId,
    property: BoneProperty,
    axis: Option<Axis>,
) {
    if !state.session.pending_pose.contains(&bone) {
        return;
    }
    let sampled = sampled_pose(state);
    if let (Some(pending), Some(sample)) = (
        state.session.preview_locals.get_mut(bone),
        sampled.locals.get(bone),
    ) {
        copy_component(pending, *sample, property, axis);
        if *pending == *sample {
            state.session.preview_locals.remove(bone);
            state.session.pending_pose.retain(|&id| id != bone);
        }
    }
    state.refresh_pose();
}

pub(super) fn bone_action(
    state: &mut AppState,
    bone: BoneId,
    property: BoneProperty,
    axis: Option<Axis>,
    action: DotAction,
) {
    if state.session.is_bone_locked(bone) {
        return;
    }
    let Some(setup) = state
        .doc
        .skeleton
        .bones
        .get(bone)
        .map(|b| b.local_transform)
    else {
        return;
    };
    if !state.session.is_animating() {
        if action == DotAction::ResetDefault {
            let mut value = setup;
            copy_component(&mut value, Transform::default(), property, axis);
            if value != setup {
                // A discrete reset must not merge into the preceding spinbox drag.
                state.dispatch(Box::new(ankhimate_document::commands::Group::new(
                    vec![Box::new(
                        ankhimate_document::commands::bone_cmds::SetBoneTransform::new(bone, value),
                    )],
                    format!("Reset {} to default", property.label()),
                )));
            }
        }
        return;
    }
    match action {
        DotAction::RevertAnimated => clear_bone_property(state, bone, property, axis),
        DotAction::RevertSetup => {
            let mut value = state
                .session
                .preview_locals
                .get(bone)
                .copied()
                .or_else(|| state.pose.locals.get(bone).copied())
                .unwrap_or(setup);
            copy_component(&mut value, setup, property, axis);
            state.commit_bone_property(bone, value, property, axis);
            if state.session.auto_key {
                clear_bone_property(state, bone, property, axis);
            }
        }
        DotAction::Key => {
            let Some(anim) = state.session.active_animation else {
                return;
            };
            if let Some(value) =
                crate::edit_router::bone_key_value(&state.doc, &state.pose, bone, property, axis)
            {
                state.dispatch(Box::new(AddKey::new(
                    anim,
                    TimelineAddr::Bone {
                        bone,
                        property,
                        axis,
                    },
                    state.session.playhead,
                    value,
                    ankhimate_core::animation::Interp::Linear,
                )));
                clear_bone_property(state, bone, property, axis);
            }
        }
        DotAction::Unkey => unkey(
            state,
            TimelineAddr::Bone {
                bone,
                property,
                axis,
            },
        ),
        _ => {}
    }
}

fn unkey(state: &mut AppState, addr: TimelineAddr) {
    if let (Some(anim), crate::edit_router::KeyState::Keyed(index)) = (
        state.session.active_animation,
        crate::edit_router::key_state(&state.doc, &state.session, &addr),
    ) {
        state.dispatch(Box::new(DeleteKeys::new(
            anim,
            vec![KeyRef { addr, index }],
        )));
    }
}

pub(super) fn color_state(state: &AppState, slot: SlotId) -> crate::edit_router::KeyState {
    if state
        .session
        .pending_slot_colors
        .get(slot)
        .is_some_and(|pending| {
            sampled_pose(state)
                .slot_colors
                .get(slot)
                .is_some_and(|sample| pending != sample)
        })
    {
        crate::edit_router::KeyState::Modified
    } else {
        crate::edit_router::key_state(
            &state.doc,
            &state.session,
            &TimelineAddr::SlotColor { slot },
        )
    }
}

pub(super) fn color_action(state: &mut AppState, slot: SlotId, action: DotAction) {
    let Some(setup) = state.doc.skeleton.slots.get(slot).map(|s| s.color) else {
        return;
    };
    if !state.session.is_animating() {
        if action == DotAction::ResetDefault && setup != [1.0; 4] {
            state.dispatch(Box::new(ankhimate_document::commands::Group::new(
                vec![Box::new(
                    ankhimate_document::commands::slot_cmds::SetSlotColor::new(slot, [1.0; 4]),
                )],
                "Reset colour to white (alpha 1)",
            )));
        }
        return;
    }
    match action {
        DotAction::RevertAnimated if state.session.pending_slot_colors.remove(slot).is_some() => {
            state.refresh_pose();
        }
        DotAction::RevertSetup => {
            state.commit_slot_color(slot, setup);
            if state.session.auto_key {
                state.session.pending_slot_colors.remove(slot);
                state.refresh_pose();
            }
        }
        DotAction::Key => {
            if let (Some(anim), Some(&color)) = (
                state.session.active_animation,
                state.pose.slot_colors.get(slot),
            ) {
                state.dispatch(Box::new(AddKey::new(
                    anim,
                    TimelineAddr::SlotColor { slot },
                    state.session.playhead,
                    KeyValue::Color(color),
                    ankhimate_core::animation::Interp::Linear,
                )));
                state.session.pending_slot_colors.remove(slot);
                state.refresh_pose();
            }
        }
        DotAction::Unkey => unkey(state, TimelineAddr::SlotColor { slot }),
        _ => {}
    }
}
