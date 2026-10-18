use super::key_controls::*;
use super::*;
use crate::session::WorkMode;
use ankhimate_core::{
    animation::{Axis, Interp, Key, Timeline},
    ids::{BoneId, SlotId},
    math::Transform,
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::commands::key_cmds::{AddKey, CreateAnimation, KeyValue};

fn fixture(setup: Transform) -> (AppState, BoneId, SlotId) {
    let mut state = AppState::default();
    let bone = state.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 12.0,
        local_transform: setup,
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = state.doc.skeleton.add_slot(Slot {
        color: [0.2, 0.4, 0.6, 0.8],
        ..Slot::new("art".into(), bone)
    });
    state.dispatch(Box::new(CreateAnimation::new("clip", 3.0)));
    state.set_work_mode(WorkMode::Animate);
    state.session.auto_key = true;
    (state, bone, slot)
}

fn addr(bone: BoneId, property: BoneProperty, axis: Option<Axis>) -> TimelineAddr {
    TimelineAddr::Bone {
        bone,
        property,
        axis,
    }
}

fn insert(state: &mut AppState, addr: TimelineAddr, time: f32, value: KeyValue) {
    state.dispatch(Box::new(AddKey::new(
        state.session.active_animation.unwrap(),
        addr,
        time,
        value,
        Interp::Linear,
    )));
}

fn scalar_keys<'a>(state: &'a AppState, addr: &TimelineAddr) -> &'a [Key<f32>] {
    state.doc.animations[state.session.active_animation.unwrap()]
        .timelines
        .iter()
        .find_map(|t| match t {
            Timeline::BoneTranslate { keys, .. }
            | Timeline::BoneRotate { keys, .. }
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

fn snapshot(state: &AppState) -> String {
    ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
}

fn rotation(state: &mut AppState, bone: BoneId) {
    insert(
        state,
        addr(bone, BoneProperty::Rotate, None),
        0.0,
        KeyValue::Scalar(0.0),
    );
    insert(
        state,
        addr(bone, BoneProperty::Rotate, None),
        1.0,
        KeyValue::Scalar(45.0),
    );
    state.set_playhead(2.0);
}

fn dot_frame(ctx: &egui::Context, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(500.0, 350.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            keyed_row(ui, true, crate::edit_router::KeyState::Keyed(0), |ui| {
                ui.label("Rotate");
                false
            });
        },
    )
}

#[test]
fn inspector_key_context_actions_are_reachable_by_secondary_click() {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    let out = dot_frame(&ctx, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Circle(c) => Some(c.center),
            _ => None,
        })
        .unwrap();
    dot_frame(&ctx, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        dot_frame(
            &ctx,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Secondary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    let out = dot_frame(&ctx, vec![]);
    let labels: Vec<_> = out
        .shapes
        .iter()
        .filter_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) => Some(t.galley.text().to_owned()),
            _ => None,
        })
        .collect();
    eprintln!("secondary-click menu: {labels:?}");
    for expected in ["Revert to setup value", "Revert to animated value"] {
        assert!(
            labels.iter().any(|s| s == expected),
            "missing action: {expected}"
        );
    }
}

#[test]
fn inspector_four_states_are_property_scoped() {
    use crate::edit_router::KeyState::*;
    let (mut state, bone, _) = fixture(Transform::default());
    let status = |s: &AppState| bone_state(s, &sampled_pose(s), bone, BoneProperty::Rotate, None);
    assert_eq!(status(&state), NoTimeline);
    rotation(&mut state, bone);
    assert_eq!(status(&state), Unkeyed);
    state.set_playhead(1.0);
    assert_eq!(status(&state), Keyed(1));
    state.session.auto_key = false;
    let mut local = state.pose.locals[bone];
    local.rotation = 1.5;
    state.commit_bone_pose(bone, local);
    assert_eq!(status(&state), Modified);
    assert_eq!(
        bone_state(
            &state,
            &sampled_pose(&state),
            bone,
            BoneProperty::Translate,
            Some(Axis::X)
        ),
        NoTimeline
    );
}

#[test]
fn inspector_setup_return_keys_only_rotation_for_zero_and_nonzero_setup() {
    for degrees in [0.0_f32, 37.0] {
        let setup = Transform {
            rotation: degrees.to_radians(),
            ..Default::default()
        };
        let (mut state, bone, _) = fixture(setup);
        rotation(&mut state, bone);
        // An unrelated, non-rest animated channel must not receive a new key.
        let pos = addr(bone, BoneProperty::Translate, Some(Axis::Y));
        insert(&mut state, pos.clone(), 0.0, KeyValue::Scalar(9.0));
        let before_pos = scalar_keys(&state, &pos).to_vec();
        let before = snapshot(&state);
        let depth = state.history.undo_depth();
        bone_action(
            &mut state,
            bone,
            BoneProperty::Rotate,
            None,
            DotAction::RevertSetup,
        );
        assert!((state.pose.locals[bone].rotation - setup.rotation).abs() < 1e-6);
        assert_eq!(
            scalar_keys(&state, &addr(bone, BoneProperty::Rotate, None))
                .last()
                .unwrap(),
            &Key::linear(2.0, 0.0)
        );
        assert_eq!(scalar_keys(&state, &pos), before_pos);
        assert_eq!(
            state.doc.animations[state.session.active_animation.unwrap()]
                .timelines
                .len(),
            2
        );
        assert_eq!(state.history.undo_depth(), depth + 1);
        let after = snapshot(&state);
        state.undo();
        assert_eq!(snapshot(&state), before);
        state.redo();
        assert_eq!(snapshot(&state), after);
    }
}

#[test]
fn inspector_setup_returns_respect_each_axis_and_scale_factor() {
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
            let (mut state, bone, _) = fixture(setup);
            let address = addr(bone, property, Some(axis));
            let rest = if property == BoneProperty::Scale {
                1.0
            } else {
                0.0
            };
            insert(&mut state, address.clone(), 0.0, KeyValue::Scalar(rest));
            insert(
                &mut state,
                address.clone(),
                1.0,
                KeyValue::Scalar(rest + 2.0),
            );
            state.set_playhead(2.0);
            bone_action(
                &mut state,
                bone,
                property,
                Some(axis),
                DotAction::RevertSetup,
            );
            assert_eq!(state.pose.locals[bone], setup);
            assert_eq!(
                scalar_keys(&state, &address).last().unwrap(),
                &Key::linear(2.0, rest)
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
fn inspector_pending_setup_return_and_manual_dot_preserve_other_previews() {
    let (mut state, bone, slot) = fixture(Transform::default());
    rotation(&mut state, bone);
    state.session.auto_key = false;
    let mut local = state.pose.locals[bone];
    local.position.x = 13.0;
    state.commit_bone_pose(bone, local);
    state.commit_slot_color(slot, [0.7; 4]);
    let before = snapshot(&state);
    bone_action(
        &mut state,
        bone,
        BoneProperty::Rotate,
        None,
        DotAction::RevertSetup,
    );
    assert_eq!(snapshot(&state), before);
    assert_eq!(state.pose.locals[bone].rotation, 0.0);
    assert_eq!(state.pose.locals[bone].position.x, 13.0);
    bone_action(&mut state, bone, BoneProperty::Rotate, None, DotAction::Key);
    assert_eq!(
        scalar_keys(&state, &addr(bone, BoneProperty::Rotate, None))
            .last()
            .unwrap(),
        &Key::linear(2.0, 0.0)
    );
    assert_eq!(state.pose.locals[bone].position.x, 13.0);
    assert_eq!(state.pose.slot_colors[slot], [0.7; 4]);
    assert!(state.session.pending_pose.contains(&bone));
    assert_eq!(
        bone_state(
            &state,
            &sampled_pose(&state),
            bone,
            BoneProperty::Rotate,
            None
        ),
        crate::edit_router::KeyState::Keyed(2)
    );
}

#[test]
fn inspector_revert_animated_restores_sample_not_setup_or_key_deletion() {
    let (mut state, bone, slot) = fixture(Transform::default());
    rotation(&mut state, bone);
    for time in [0.5, 1.0] {
        state.set_playhead(time);
        let sampled = state.pose.locals[bone];
        state.session.auto_key = false;
        let mut local = sampled;
        local.rotation = 1.9;
        local.position.y = 14.0;
        state.commit_bone_pose(bone, local);
        state.commit_slot_color(slot, [0.3; 4]);
        let before = snapshot(&state);
        let depth = state.history.undo_depth();
        let rev = state.revision;
        bone_action(
            &mut state,
            bone,
            BoneProperty::Rotate,
            None,
            DotAction::RevertAnimated,
        );
        assert_eq!(state.pose.locals[bone].rotation, sampled.rotation);
        assert_eq!(state.pose.locals[bone].position.y, 14.0);
        assert_eq!(state.pose.slot_colors[slot], [0.3; 4]);
        assert_eq!(snapshot(&state), before);
        assert_eq!(state.history.undo_depth(), depth);
        assert_eq!(state.revision, rev);
        bone_action(
            &mut state,
            bone,
            BoneProperty::Translate,
            Some(Axis::Y),
            DotAction::RevertAnimated,
        );
        assert!(!state.session.pending_pose.contains(&bone));
        assert!(state.session.pending_slot_colors.contains_key(slot));
    }
}

#[test]
fn inspector_revert_without_pending_or_unanimated_setup_is_history_noop() {
    let (mut state, bone, slot) = fixture(Transform::default());
    let before = snapshot(&state);
    let depth = state.history.undo_depth();
    let rev = state.revision;
    for action in [DotAction::RevertAnimated, DotAction::RevertSetup] {
        bone_action(&mut state, bone, BoneProperty::Rotate, None, action);
    }
    color_action(&mut state, slot, DotAction::RevertAnimated);
    assert_eq!(snapshot(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    assert_eq!(state.revision, rev);
}

#[test]
fn inspector_setup_reset_is_explicit_per_property_undoable_and_separate_from_drag() {
    for (property, axis) in [
        (BoneProperty::Rotate, None),
        (BoneProperty::Translate, Some(Axis::X)),
        (BoneProperty::Translate, Some(Axis::Y)),
        (BoneProperty::Scale, Some(Axis::X)),
        (BoneProperty::Scale, Some(Axis::Y)),
        (BoneProperty::Shear, Some(Axis::X)),
        (BoneProperty::Shear, Some(Axis::Y)),
    ] {
        let (mut state, bone, _) = fixture(Transform::default());
        rotation(&mut state, bone);
        state.set_work_mode(WorkMode::Setup);
        let local = Transform {
            position: glam::vec2(3.0, 7.0),
            rotation: 0.4,
            scale: glam::vec2(2.0, 3.0),
            shear: glam::vec2(0.2, -0.1),
        };
        state.commit_bone_pose(bone, local); // Reset must undo to this, not pre-drag.
        let before = snapshot(&state);
        let depth = state.history.undo_depth();
        bone_action(&mut state, bone, property, axis, DotAction::ResetDefault);
        let mut expected = local;
        match property {
            BoneProperty::Rotate => expected.rotation = 0.0,
            BoneProperty::Translate => expected.position[axis.unwrap().index()] = 0.0,
            BoneProperty::Scale => expected.scale[axis.unwrap().index()] = 1.0,
            BoneProperty::Shear => expected.shear[axis.unwrap().index()] = 0.0,
        }
        assert_eq!(state.doc.skeleton.bones[bone].local_transform, expected);
        assert_eq!(state.history.undo_depth(), depth + 1);
        let after = snapshot(&state);
        state.undo();
        assert_eq!(snapshot(&state), before);
        state.redo();
        assert_eq!(snapshot(&state), after);
        let depth = state.history.undo_depth();
        bone_action(&mut state, bone, property, axis, DotAction::ResetDefault);
        assert_eq!(state.history.undo_depth(), depth);
    }
}

#[test]
fn inspector_existing_key_is_replaced_once_and_roundtrips() {
    let (mut state, bone, _) = fixture(Transform::default());
    rotation(&mut state, bone);
    state.set_playhead(1.0);
    bone_action(
        &mut state,
        bone,
        BoneProperty::Rotate,
        None,
        DotAction::RevertSetup,
    );
    assert_eq!(
        scalar_keys(&state, &addr(bone, BoneProperty::Rotate, None)),
        [Key::linear(0.0, 0.0), Key::linear(1.0, 0.0)]
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("controls.ankh");
    ankhimate_formats::save(&path, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.is_clean());
    let doc = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        snapshot(&state)
    );
}

#[test]
fn inspector_slot_colour_pending_revert_manual_key_and_reset_contract() {
    let (mut state, bone, slot) = fixture(Transform::default());
    let address = TimelineAddr::SlotColor { slot };
    insert(&mut state, address.clone(), 0.0, KeyValue::Color([0.0; 4]));
    insert(&mut state, address, 1.0, KeyValue::Color([1.0; 4]));
    state.set_playhead(0.5);
    state.session.auto_key = false;
    let before = snapshot(&state);
    let depth = state.history.undo_depth();
    state.commit_slot_color(slot, [0.9; 4]);
    assert_eq!(
        color_state(&state, slot),
        crate::edit_router::KeyState::Modified
    );
    let mut local = state.pose.locals[bone];
    local.position.x = 13.0;
    state.commit_bone_pose(bone, local);
    color_action(&mut state, slot, DotAction::RevertAnimated);
    assert_eq!(state.pose.slot_colors[slot], [0.5; 4]);
    assert_eq!(state.pose.locals[bone].position.x, 13.0);
    assert_eq!(snapshot(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    color_action(&mut state, slot, DotAction::RevertSetup);
    assert_eq!(state.pose.slot_colors[slot], [0.2, 0.4, 0.6, 0.8]);
    assert_eq!(snapshot(&state), before);
    color_action(&mut state, slot, DotAction::Key);
    assert!(!state.session.pending_slot_colors.contains_key(slot));
    assert_eq!(state.pose.slot_colors[slot], [0.2, 0.4, 0.6, 0.8]);
    assert_eq!(state.pose.locals[bone].position.x, 13.0);
    let keyed = snapshot(&state);
    state.undo();
    assert_eq!(snapshot(&state), before);
    state.redo();
    assert_eq!(snapshot(&state), keyed);
    state.set_work_mode(WorkMode::Setup);
    let before = snapshot(&state);
    color_action(&mut state, slot, DotAction::ResetDefault);
    assert_eq!(state.doc.skeleton.slots[slot].color, [1.0; 4]);
    state.undo();
    assert_eq!(snapshot(&state), before);
}

#[test]
fn inspector_slot_setup_return_autokey_and_manual_k_are_both_functional() {
    for auto in [true, false] {
        let (mut state, _, slot) = fixture(Transform::default());
        insert(
            &mut state,
            TimelineAddr::SlotColor { slot },
            0.0,
            KeyValue::Color([0.9; 4]),
        );
        state.set_playhead(1.0);
        state.session.auto_key = auto;
        let before = snapshot(&state);
        color_action(&mut state, slot, DotAction::RevertSetup);
        if !auto {
            assert_eq!(snapshot(&state), before);
            state.key_pending_pose();
        }
        assert!(!state.session.has_pending_pose());
        assert_eq!(state.pose.slot_colors[slot], [0.2, 0.4, 0.6, 0.8]);
        assert!(
            matches!(&state.doc.animations[state.session.active_animation.unwrap()].timelines[0],Timeline::SlotColor {keys,..} if keys.len()==2 && keys[1].time==1.0 && keys[1].value==[0.2,0.4,0.6,0.8])
        );
    }
}

fn panel_frame(
    ctx: &egui::Context,
    state: &mut AppState,
    slot: Option<SlotId>,
    events: Vec<egui::Event>,
    modifiers: egui::Modifiers,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(650.0, 1600.0),
            )),
            events,
            modifiers,
            ..Default::default()
        },
        |ui| {
            if let Some(slot) = slot {
                slot_inspector(ui, state, slot);
            } else {
                super::ui(ui, state);
            }
        },
    )
}

fn panel_click(
    ctx: &egui::Context,
    state: &mut AppState,
    slot: Option<SlotId>,
    pos: egui::Pos2,
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
) {
    panel_frame(
        ctx,
        state,
        slot,
        vec![egui::Event::PointerMoved(pos)],
        modifiers,
    );
    for pressed in [true, false] {
        panel_frame(
            ctx,
            state,
            slot,
            vec![egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            }],
            modifiers,
        );
    }
}

fn first_dot(out: &egui::FullOutput) -> egui::Pos2 {
    out.shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Circle(c) if c.radius == 4.5 => Some(c.center),
            _ => None,
        })
        .expect("inspector key indicator")
}

fn text_position(out: &egui::FullOutput, label: &str) -> egui::Pos2 {
    out.shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == label => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "missing UI action {label}; labels: {:?}",
                out.shapes
                    .iter()
                    .filter_map(|s| match &s.shape {
                        egui::epaint::Shape::Text(t) => Some((t.pos, t.galley.text())),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            )
        })
}

#[test]
fn inspector_real_bone_and_slot_menus_dispatch_actions_without_cross_property_edits() {
    for is_slot in [false, true] {
        let (mut state, bone, slot) = fixture(Transform::default());
        state.session.select_bone(Some(bone));
        rotation(&mut state, bone);
        insert(
            &mut state,
            TimelineAddr::SlotColor { slot },
            0.0,
            KeyValue::Color([0.9; 4]),
        );
        let before = snapshot(&state);
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        let target = is_slot.then_some(slot);
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        panel_click(
            &ctx,
            &mut state,
            target,
            first_dot(&out),
            egui::PointerButton::Secondary,
            Default::default(),
        );
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        let pos = text_position(&out, "Revert to setup value");
        // The menu is also exposed to assistive input, not just drawn text.
        assert!(
            out.platform_output
                .accesskit_update
                .unwrap()
                .nodes
                .iter()
                .any(|(_, n)| n.label() == Some("Revert to setup value"))
        );
        panel_click(
            &ctx,
            &mut state,
            target,
            pos,
            egui::PointerButton::Primary,
            Default::default(),
        );
        if is_slot {
            assert_eq!(state.pose.slot_colors[slot], [0.2, 0.4, 0.6, 0.8]);
            assert!((state.pose.locals[bone].rotation.to_degrees() - 45.0).abs() < 1e-4);
        } else {
            assert_eq!(
                scalar_keys(&state, &addr(bone, BoneProperty::Rotate, None))
                    .last()
                    .unwrap(),
                &Key::linear(2.0, 0.0)
            );
            assert_eq!(state.pose.slot_colors[slot], [0.9; 4]);
        }
        state.undo();
        assert_eq!(snapshot(&state), before);
    }
}

#[test]
fn inspector_real_menu_revert_animated_keeps_current_time_key_and_other_pending_fields() {
    for is_slot in [false, true] {
        let (mut state, bone, slot) = fixture(Transform::default());
        state.session.select_bone(Some(bone));
        rotation(&mut state, bone);
        state.set_playhead(1.0);
        state.session.auto_key = false;
        let mut local = state.pose.locals[bone];
        local.rotation = 2.0;
        local.position.x = 9.0;
        state.commit_bone_pose(bone, local);
        state.commit_slot_color(slot, [0.9; 4]);
        let before = snapshot(&state);
        let depth = state.history.undo_depth();
        let ctx = egui::Context::default();
        let target = is_slot.then_some(slot);
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        panel_click(
            &ctx,
            &mut state,
            target,
            first_dot(&out),
            egui::PointerButton::Secondary,
            Default::default(),
        );
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        panel_click(
            &ctx,
            &mut state,
            target,
            text_position(&out, "Revert to animated value"),
            egui::PointerButton::Primary,
            Default::default(),
        );
        assert_eq!(snapshot(&state), before);
        assert_eq!(state.history.undo_depth(), depth);
        assert_eq!(state.pose.locals[bone].position.x, 9.0);
        if is_slot {
            assert_eq!(state.pose.slot_colors[slot], [0.2, 0.4, 0.6, 0.8]);
            assert_eq!(state.pose.locals[bone].rotation, 2.0);
        } else {
            assert!((state.pose.locals[bone].rotation.to_degrees() - 45.0).abs() < 1e-4);
            assert_eq!(state.pose.slot_colors[slot], [0.9; 4]);
        }
    }
}

#[test]
fn inspector_real_setup_reset_menu_and_plain_button_reset_only_own_property() {
    for is_slot in [false, true] {
        let setup = Transform {
            rotation: 0.7,
            position: glam::vec2(12.0, 24.0),
            ..Default::default()
        };
        let (mut state, bone, slot) = fixture(setup);
        state.session.select_bone(Some(bone));
        state.set_work_mode(WorkMode::Setup);
        let before = snapshot(&state);
        let ctx = egui::Context::default();
        let target = is_slot.then_some(slot);
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        let reset = text_position(&out, "↺");
        panel_click(
            &ctx,
            &mut state,
            target,
            reset,
            egui::PointerButton::Secondary,
            Default::default(),
        );
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        assert!(!out.shapes.iter().any(|s| matches!(&s.shape,egui::epaint::Shape::Text(t) if t.galley.text()=="Revert to setup value")));
        panel_click(
            &ctx,
            &mut state,
            target,
            text_position(&out, "Reset to default"),
            egui::PointerButton::Primary,
            Default::default(),
        );
        if is_slot {
            assert_eq!(state.doc.skeleton.slots[slot].color, [1.0; 4]);
        } else {
            assert_eq!(state.doc.skeleton.bones[bone].local_transform.rotation, 0.0);
            assert_eq!(
                state.doc.skeleton.bones[bone].local_transform.position,
                setup.position
            );
        }
        state.undo();
        assert_eq!(snapshot(&state), before);
        // Plain reset button is deliberately equivalent to its context action.
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        panel_click(
            &ctx,
            &mut state,
            target,
            text_position(&out, "↺"),
            egui::PointerButton::Primary,
            Default::default(),
        );
        assert_ne!(snapshot(&state), before);
    }
}

#[test]
fn inspector_plain_and_alt_click_still_key_and_delete_the_same_values_as_timeline() {
    for is_slot in [false, true] {
        let (mut state, bone, slot) = fixture(Transform::default());
        state.session.select_bone(Some(bone));
        state.set_playhead(0.5);
        let before = snapshot(&state);
        let ctx = egui::Context::default();
        let target = is_slot.then_some(slot);
        let out = panel_frame(&ctx, &mut state, target, vec![], Default::default());
        let pos = first_dot(&out);
        // Build the existing timeline command separately and compare full documents.
        let value = if is_slot {
            KeyValue::Color(state.pose.slot_colors[slot])
        } else {
            crate::edit_router::bone_key_value(
                &state.doc,
                &state.pose,
                bone,
                BoneProperty::Rotate,
                None,
            )
            .unwrap()
        };
        let address = if is_slot {
            TimelineAddr::SlotColor { slot }
        } else {
            addr(bone, BoneProperty::Rotate, None)
        };
        insert(&mut state, address, 0.5, value);
        let expected = snapshot(&state);
        state.undo();
        assert_eq!(snapshot(&state), before);
        panel_click(
            &ctx,
            &mut state,
            target,
            pos,
            egui::PointerButton::Primary,
            Default::default(),
        );
        assert_eq!(snapshot(&state), expected);
        let alt = egui::Modifiers {
            alt: true,
            ..Default::default()
        };
        panel_click(
            &ctx,
            &mut state,
            target,
            pos,
            egui::PointerButton::Primary,
            alt,
        );
        assert!(
            state.doc.animations[state.session.active_animation.unwrap()]
                .timelines
                .iter()
                .all(|t| match t {
                    Timeline::BoneRotate { keys, .. } => keys.is_empty(),
                    Timeline::SlotColor { keys, .. } => keys.is_empty(),
                    _ => false,
                })
        );
    }
}

#[test]
fn inspector_slot_pending_is_discarded_on_scrub_and_modes_and_does_not_hide_new_autokeys() {
    let (mut state, _, slot) = fixture(Transform::default());
    state.session.auto_key = false;
    state.commit_slot_color(slot, [0.9; 4]);
    state.set_playhead(0.2);
    assert!(!state.session.has_pending_pose());
    state.commit_slot_color(slot, [0.9; 4]);
    state.set_work_mode(WorkMode::Setup);
    assert!(!state.session.has_pending_pose());
    state.set_work_mode(WorkMode::Animate);
    state.session.auto_key = false;
    state.commit_slot_color(slot, [0.9; 4]);
    state.session.auto_key = true;
    state.commit_slot_color(slot, [0.7; 4]);
    assert_eq!(state.pose.slot_colors[slot], [0.7; 4]);
    assert!(!state.session.has_pending_pose());
}

#[test]
fn inspector_locked_bone_rejects_all_mutating_actions() {
    let (mut state, bone, _) = fixture(Transform::default());
    rotation(&mut state, bone);
    state.session.locked_bones.insert(bone, true);
    let before = snapshot(&state);
    let depth = state.history.undo_depth();
    for action in [
        DotAction::Key,
        DotAction::Unkey,
        DotAction::RevertSetup,
        DotAction::RevertAnimated,
    ] {
        bone_action(&mut state, bone, BoneProperty::Rotate, None, action);
    }
    assert_eq!(snapshot(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
}

#[test]
fn inspector_property_actions_preserve_other_bones_and_selection() {
    let (mut state, bone, _) = fixture(Transform::default());
    let mut second = state.doc.skeleton.bones[bone].clone();
    second.name = "second".into();
    let other = state.doc.skeleton.add_bone(second);
    rotation(&mut state, bone);
    state.session.select_bone(Some(bone));
    state.session.auto_key = false;
    let mut local = state.pose.locals[other];
    local.scale.y = 2.5;
    state.commit_bone_pose(other, local);
    let mut local = state.pose.locals[bone];
    local.rotation = 2.0;
    state.commit_bone_pose(bone, local);
    let other_preview = state.session.preview_locals[other];
    let before = snapshot(&state);
    bone_action(
        &mut state,
        bone,
        BoneProperty::Rotate,
        None,
        DotAction::RevertAnimated,
    );
    assert_eq!(state.session.preview_locals[other], other_preview);
    assert_eq!(state.session.active_bone(), Some(bone));
    assert_eq!(snapshot(&state), before);
    state.session.auto_key = true;
    bone_action(
        &mut state,
        bone,
        BoneProperty::Rotate,
        None,
        DotAction::RevertSetup,
    );
    assert_eq!(state.session.preview_locals[other], other_preview);
    assert_eq!(state.pose.locals[other].scale.y, 2.5);
}

#[test]
fn inspector_slot_colour_four_states_and_alpha_survive_save_reopen() {
    use crate::edit_router::KeyState::*;
    let (mut state, _, slot) = fixture(Transform::default());
    assert_eq!(color_state(&state, slot), NoTimeline);
    insert(
        &mut state,
        TimelineAddr::SlotColor { slot },
        0.0,
        KeyValue::Color([0.9; 4]),
    );
    assert_eq!(color_state(&state, slot), Keyed(0));
    state.set_playhead(0.5);
    assert_eq!(color_state(&state, slot), Unkeyed);
    state.session.auto_key = false;
    state.commit_slot_color(slot, [0.0, 0.1, 0.7, 0.25]);
    assert_eq!(color_state(&state, slot), Modified);
    color_action(&mut state, slot, DotAction::Key);
    assert_eq!(color_state(&state, slot), Keyed(1));
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("colour.ankh");
    ankhimate_formats::save(&path, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.is_clean());
    let doc = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        snapshot(&state)
    );
}

#[test]
fn inspector_each_setup_return_with_autokey_off_can_be_keyed_manually() {
    for (property, axis) in [
        (BoneProperty::Rotate, None),
        (BoneProperty::Translate, Some(Axis::X)),
        (BoneProperty::Translate, Some(Axis::Y)),
        (BoneProperty::Scale, Some(Axis::X)),
        (BoneProperty::Scale, Some(Axis::Y)),
        (BoneProperty::Shear, Some(Axis::X)),
        (BoneProperty::Shear, Some(Axis::Y)),
    ] {
        let setup = Transform {
            rotation: 0.2,
            position: glam::vec2(3.0, 7.0),
            scale: glam::vec2(2.0, 3.0),
            shear: glam::vec2(0.1, 0.2),
        };
        let (mut state, bone, _) = fixture(setup);
        let address = addr(bone, property, axis);
        let rest = if property == BoneProperty::Scale {
            1.0
        } else {
            0.0
        };
        insert(&mut state, address.clone(), 0.0, KeyValue::Scalar(rest));
        insert(
            &mut state,
            address.clone(),
            1.0,
            KeyValue::Scalar(rest + 2.0),
        );
        state.set_playhead(2.0);
        state.session.auto_key = false;
        let before = snapshot(&state);
        bone_action(&mut state, bone, property, axis, DotAction::RevertSetup);
        assert_eq!(snapshot(&state), before);
        bone_action(&mut state, bone, property, axis, DotAction::Key);
        assert_eq!(
            scalar_keys(&state, &address).last().unwrap(),
            &Key::linear(2.0, rest)
        );
        assert_eq!(
            state.doc.animations[state.session.active_animation.unwrap()]
                .timelines
                .len(),
            1
        );
        assert!(!state.session.has_pending_pose());
    }
}
