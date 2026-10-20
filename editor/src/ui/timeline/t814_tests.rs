use super::*;
use crate::app_state::AppState;
use ankhimate_core::{
    animation::{Animation, Interp, Key, Timeline},
    math::Transform,
    skeleton::Bone,
};
use ankhimate_document::commands::key_cmds::{BoneProperty, KeyRef, SetInterp, TimelineAddr};

pub(super) fn fixture() -> (AppState, ankhimate_core::ids::AnimationId, KeyRef) {
    let mut s = AppState::default();
    let bone = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.0,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("curve", 1.0);
    a.timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![
            Key::linear(0.0, 0.0),
            Key {
                time: 1.0,
                value: 45.0,
                interp: Interp::Bezier {
                    out_handle: glam::vec2(0.25, -0.5),
                    in_handle: glam::vec2(0.75, 1.5),
                },
            },
        ],
    });
    let id = s.doc.animations.insert(a);
    s.session.active_animation = Some(id);
    s.session.select_bone(Some(bone));
    s.set_work_mode(crate::session::WorkMode::Animate);
    s.refresh_pose();
    (
        s,
        id,
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Rotate,
                axis: None,
            },
            index: 1,
        },
    )
}
fn sheet_frame(
    ctx: &egui::Context,
    s: &mut AppState,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            let id = s.session.active_animation.unwrap();
            let model = model::TimelineModel::build(s, id);
            sheet::ui(
                ui,
                s,
                id,
                &model,
                &mut ViewState::default(),
                &Layout {
                    sheet_x0: 30.0,
                    px_per_sec: 160.0,
                    scroll_sec: 0.0,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15.0, 50.0), egui::vec2(600.0, 500.0)),
                Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.0,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
#[test]
fn t814_dopesheet_selection_reaches_numeric_inspector() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    sheet_frame(&ctx, &mut s, vec![]);
    let pos = egui::pos2(190.0, 83.0);
    sheet_frame(&ctx, &mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        sheet_frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 900.0),
            )),
            ..Default::default()
        },
        |ui| crate::ui::inspector::ui(ui, &mut s),
    );
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::epaint::Shape::Text(t)if t.galley.text().contains("Bezier tangents"))),"A selected Bezier key has no numeric tangent controls");
}
#[test]
fn t814_interpolation_command_cannot_edit_in_setup() {
    let (mut s, id, key) = fixture();
    s.set_work_mode(crate::session::WorkMode::Setup);
    let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    assert!(
        !s.dispatch(Box::new(SetInterp::new(id, vec![key], Interp::Stepped))),
        "SetInterp must enforce Animate at the command boundary"
    );
    assert_eq!(
        ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
        before
    );
}
#[test]
fn t814_extreme_finite_curve_bounds_do_not_create_nonfinite_paint_geometry() {
    let (mut s, id, _) = fixture();
    let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] else {
        unreachable!()
    };
    keys[0].value = -f32::MAX;
    keys[1].value = f32::MAX;
    let ctx = egui::Context::default();
    let model = model::TimelineModel::build(&s, id);
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 900.0),
            )),
            ..Default::default()
        },
        |ui| {
            graph::ui(
                ui,
                &mut s,
                id,
                &model,
                &ViewState::default(),
                &Layout {
                    sheet_x0: 30.0,
                    px_per_sec: 160.0,
                    scroll_sec: 0.0,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15.0, 50.0), egui::vec2(600.0, 500.0)),
                Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.0,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    );
    assert!(
        out.shapes
            .iter()
            .all(|sh| sh.shape.visual_bounding_rect().is_finite()),
        "Finite keys overflow the graph's f32 value range"
    );
}

use crate::{key_selection as selection, ui::inspector::tangents::Draft};
use ankhimate_document::{clipboard::KeyClip, commands::key_cmds::DeleteKeys};
fn snapshot(s: &AppState) -> String {
    ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
}
fn draft(s: &mut AppState, id: ankhimate_core::ids::AnimationId, key: KeyRef) -> Draft {
    selection::set(s, id, vec![key]);
    Draft::new(s, id).unwrap()
}
fn interp(s: &AppState, id: ankhimate_core::ids::AnimationId, key: &KeyRef) -> Interp {
    KeyClip::capture(&s.doc, id, std::slice::from_ref(key))
        .unwrap()
        .keys[0]
        .key
        .interp
}
#[test]
fn t814_numeric_apply_three_undo_redo_cycles_preserve_full_document() {
    let (mut s, id, key) = fixture();
    let before = snapshot(&s);
    let mut d = draft(&mut s, id, key.clone());
    d.fields = ["0.12345679", "-2.25", "0.9876543", "3.75"].map(str::to_owned);
    d.apply(&mut s).unwrap();
    let after = snapshot(&s);
    assert_ne!(before, after);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        assert!(selection::read(&mut s, id).is_empty());
        s.redo();
        assert_eq!(snapshot(&s), after);
        assert!(selection::read(&mut s, id).is_empty());
    }
}
#[test]
fn t814_numeric_applies_do_not_merge_with_each_other_or_a_drag() {
    let (mut s, id, key) = fixture();
    let before = snapshot(&s);
    s.dispatch(Box::new(
        SetInterp::new(
            id,
            vec![key.clone()],
            Interp::Bezier {
                out_handle: glam::vec2(0.1, -1.0),
                in_handle: glam::vec2(0.9, 2.0),
            },
        )
        .with_gesture(12),
    ));
    let drag = snapshot(&s);
    let mut d = draft(&mut s, id, key.clone());
    d.fields[1] = "-3".into();
    d.apply(&mut s).unwrap();
    let first = snapshot(&s);
    let mut d = Draft::new(&mut s, id).unwrap();
    d.fields[1] = "-4".into();
    d.apply(&mut s).unwrap();
    s.undo();
    assert_eq!(snapshot(&s), first);
    s.undo();
    assert_eq!(snapshot(&s), drag);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_one_tangent_gesture_merges_but_next_gesture_is_separate() {
    let (mut s, id, key) = fixture();
    let before = snapshot(&s);
    for y in [-1.0, -2.0, -3.0] {
        s.dispatch(Box::new(
            SetInterp::new(
                id,
                vec![key.clone()],
                Interp::Bezier {
                    out_handle: glam::vec2(0.2, y),
                    in_handle: glam::vec2(0.8, 2.0),
                },
            )
            .with_gesture(11),
        ));
    }
    let first = snapshot(&s);
    s.dispatch(Box::new(
        SetInterp::new(id, vec![key], Interp::Linear).with_gesture(12),
    ));
    s.undo();
    assert_eq!(snapshot(&s), first);
    s.undo();
    assert_eq!(snapshot(&s), before);
    s.redo();
    assert_eq!(snapshot(&s), first);
}
#[test]
fn t814_invalid_numbers_preserve_document_and_redo() {
    for field in 0..4 {
        for value in ["NaN", "inf", "-inf", "1e100", "oops", ""] {
            let (mut s, id, key) = fixture();
            let mut d = draft(&mut s, id, key.clone());
            d.fields[1] = "-4".into();
            d.apply(&mut s).unwrap();
            let expected = snapshot(&s);
            s.undo();
            let before = snapshot(&s);
            let mut d = draft(&mut s, id, key);
            d.fields[field] = value.into();
            assert!(d.apply(&mut s).is_err(), "{field}: {value}");
            assert_eq!(snapshot(&s), before);
            s.redo();
            assert_eq!(snapshot(&s), expected);
        }
    }
}
#[test]
fn t814_normalized_time_is_bounded_y_is_free_and_precision_is_preserved() {
    let (mut s, id, key) = fixture();
    for value in ["-0.001", "1.001"] {
        let mut d = draft(&mut s, id, key.clone());
        d.fields[0] = value.into();
        assert!(d.apply(&mut s).is_err());
        d.fields[0] = "0.2".into();
        d.fields[2] = value.into();
        assert!(d.apply(&mut s).is_err());
    }
    let mut d = draft(&mut s, id, key.clone());
    d.fields = ["0", "-123.45679", "1", "123.45679"].map(str::to_owned);
    d.apply(&mut s).unwrap();
    let reset = Draft::new(&mut s, id).unwrap();
    assert_eq!(reset.fields, d.fields);
    assert!(matches!(interp(&s, id, &key), Interp::Bezier { .. }));
}
#[test]
fn t814_reading_drafts_or_canceling_never_changes_the_document() {
    let (mut s, id, key) = fixture();
    let before = snapshot(&s);
    let mut d = draft(&mut s, id, key);
    d.fields[1] = "-22".into();
    assert_eq!(snapshot(&s), before);
    let _reset = Draft::new(&mut s, id).unwrap();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_non_bezier_first_key_and_mixed_selection_do_not_convert_silently() {
    for curve in [Interp::Linear, Interp::Stepped] {
        let (mut s, id, key) = fixture();
        let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] else {
            unreachable!()
        };
        keys[1].interp = curve;
        let before = snapshot(&s);
        selection::set(&mut s, id, vec![key]);
        assert!(Draft::new(&mut s, id).is_err());
        assert_eq!(snapshot(&s), before);
    }
    let (mut s, id, mut key) = fixture();
    key.index = 0;
    selection::set(&mut s, id, vec![key]);
    assert!(Draft::new(&mut s, id).is_err());
}
#[test]
fn t814_multi_key_bezier_edit_is_one_undo_and_preserves_values_times() {
    let (mut s, id, key) = fixture();
    let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] else {
        unreachable!()
    };
    keys.push(Key {
        time: 2.0,
        value: -20.0,
        interp: keys[1].interp,
    });
    s.doc.animations[id].duration = 2.0;
    let before = snapshot(&s);
    selection::set(&mut s, id, vec![key.clone(), KeyRef { index: 2, ..key }]);
    let mut d = Draft::new(&mut s, id).unwrap();
    d.fields[1] = "-5".into();
    d.apply(&mut s).unwrap();
    let selected = selection::read(&mut s, id);
    let clip = KeyClip::capture(&s.doc, id, &selected).unwrap();
    assert_eq!(clip.keys[0].key.interp, clip.keys[1].key.interp);
    assert_eq!(clip.keys[0].key.time, 1.0);
    assert_eq!(
        clip.keys[1].key.value,
        ankhimate_document::clipboard::keys::KeyData::Scalar(-20.0)
    );
    s.undo();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_deleted_or_reordered_indices_never_target_another_key() {
    let (mut s, id, key) = fixture();
    let d = draft(&mut s, id, key.clone());
    s.dispatch(Box::new(DeleteKeys::new(
        id,
        vec![KeyRef {
            index: 0,
            ..key.clone()
        }],
    )));
    assert!(selection::read(&mut s, id).is_empty());
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    s.undo();
    assert!(selection::read(&mut s, id).is_empty());
    selection::set(&mut s, id, vec![key.clone()]);
    let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] else {
        unreachable!()
    };
    keys.insert(0, Key::linear(0.1, 22.0));
    assert!(selection::read(&mut s, id).is_empty());
}
#[test]
fn t814_animation_or_document_change_clears_session_selection() {
    let (mut s, id, key) = fixture();
    let d = draft(&mut s, id, key.clone());
    let other = s.doc.animations.insert(Animation::new("other", 1.0));
    s.session.active_animation = Some(other);
    assert!(selection::read(&mut s, other).is_empty());
    assert!(d.apply(&mut s).is_err());
    s.session.active_animation = Some(id);
    assert!(selection::read(&mut s, id).is_empty());
    selection::set(&mut s, id, vec![key]);
    s.replace_document(Default::default());
    assert!(selection::read(&mut s, id).is_empty());
}
#[test]
fn t814_own_move_rebases_selection_by_time_after_crossing() {
    let (mut s, id, key) = fixture();
    let moved = selection::move_keys(&mut s, id, std::slice::from_ref(&key), -1.25);
    assert!(moved.is_none(), "Clamp collision must be refused");
    let moved = selection::move_keys(
        &mut s,
        id,
        std::slice::from_ref(&KeyRef {
            index: 0,
            ..key.clone()
        }),
        1.5,
    )
    .unwrap();
    assert_eq!(moved[0].index, 1);
    let clip = KeyClip::capture(&s.doc, id, &moved).unwrap();
    assert_eq!(clip.keys[0].key.time, 1.5);
    assert_eq!(
        clip.keys[0].key.value,
        ankhimate_document::clipboard::keys::KeyData::Scalar(0.0)
    );
    assert_eq!(selection::read(&mut s, id), moved);
}
#[test]
fn t814_stale_draft_locked_bone_and_setup_mode_are_refused() {
    let (mut s, id, key) = fixture();
    let mut d = draft(&mut s, id, key.clone());
    d.fields[1] = "-7".into();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        unreachable!()
    };
    s.session.locked_bones.insert(bone, true);
    assert!(d.apply(&mut s).is_err());
    s.session.locked_bones.insert(bone, false);
    s.set_work_mode(crate::session::WorkMode::Setup);
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_save_reload_preserves_tangent_bits_and_sampled_curve() {
    let (mut s, id, key) = fixture();
    let mut d = draft(&mut s, id, key);
    d.fields = ["0.12345679", "-1.2345679", "0.8765432", "2.3456788"].map(str::to_owned);
    d.apply(&mut s).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("curve.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    let doc = ankhimate_document::Document::from_loaded(loaded);
    let src = &s.doc.animations[id];
    let dst = doc
        .animations
        .values()
        .find(|a| a.name == src.name)
        .unwrap();
    let Timeline::BoneRotate { keys: a, .. } = &src.timelines[0] else {
        unreachable!()
    };
    let Timeline::BoneRotate { keys: b, .. } = &dst.timelines[0] else {
        unreachable!()
    };
    assert_eq!(a, b);
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        let x = ankhimate_core::animation::sample_angle_degrees(a, t, &mut 0);
        let y = ankhimate_core::animation::sample_angle_degrees(b, t, &mut 0);
        assert_eq!(x, y);
    }
}
#[test]
fn t814_duplicate_addresses_and_missing_keys_refuse_selection() {
    let (mut s, id, key) = fixture();
    let duplicate = s.doc.animations[id].timelines[0].clone();
    s.doc.animations[id].timelines.push(duplicate);
    selection::set(&mut s, id, vec![key.clone()]);
    assert!(selection::read(&mut s, id).is_empty());
    s.doc.animations[id].timelines.pop();
    selection::set(&mut s, id, vec![KeyRef { index: 99, ..key }]);
    assert!(selection::read(&mut s, id).is_empty());
}
fn graph_frame(
    ctx: &egui::Context,
    s: &mut AppState,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            let id = s.session.active_animation.unwrap();
            let model = model::TimelineModel::build(s, id);
            graph::ui(
                ui,
                s,
                id,
                &model,
                &ViewState::default(),
                &Layout {
                    sheet_x0: 30.0,
                    px_per_sec: 160.0,
                    scroll_sec: 0.0,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15.0, 50.0), egui::vec2(600.0, 500.0)),
                Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.0,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn texts(out: &egui::FullOutput) -> Vec<String> {
    out.shapes
        .iter()
        .filter_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                Some(t.galley.text().to_owned())
            } else {
                None
            }
        })
        .collect()
}
#[test]
fn t814_fit_is_stable_while_primary_pointer_is_down() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let before = texts(&graph_frame(&ctx, &mut s, vec![]));
    let id = s.session.active_animation.unwrap();
    let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] else {
        unreachable!()
    };
    keys[1].value = 100.0;
    let pos = egui::pos2(700.0, 700.0);
    let during = texts(&graph_frame(
        &ctx,
        &mut s,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
        ],
    ));
    let numeric = |labels: Vec<String>| {
        labels
            .into_iter()
            .filter(|s| s.parse::<f64>().is_ok())
            .collect::<Vec<_>>()
    };
    assert_eq!(numeric(before.clone()), numeric(during));
    let after = texts(&graph_frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    ));
    assert_ne!(numeric(before), numeric(after));
}
#[test]
fn t814_graph_click_selects_the_same_key_for_sheet_and_inspector() {
    let (mut s, id, key) = fixture();
    let ctx = egui::Context::default();
    let out = graph_frame(&ctx, &mut s, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| {
            if let egui::Shape::Circle(c) = &s.shape
                && c.radius == 4.0
                && (c.center.x - 190.0).abs() < 0.01
            {
                Some(c.center)
            } else {
                None
            }
        })
        .unwrap();
    graph_frame(&ctx, &mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        graph_frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    assert_eq!(selection::read(&mut s, id), vec![key.clone()]);
    let before = snapshot(&s);
    sheet_frame(&ctx, &mut s, vec![]);
    assert_eq!(selection::read(&mut s, id), vec![key]);
    assert_eq!(snapshot(&s), before);
    assert!(Draft::new(&mut s, id).is_ok());
}
#[test]
fn t814_framing_constant_single_key_short_and_extreme_curves_is_finite() {
    for values in [
        vec![0.0],
        vec![1.0, 1.0],
        vec![1.0, 1.000001],
        vec![-1e30, 1e30],
        vec![-f32::MAX, f32::MAX],
    ] {
        let (mut s, id, _) = fixture();
        let Timeline::BoneRotate { bone, .. } = &mut s.doc.animations[id].timelines[0] else {
            unreachable!()
        };
        let bone = *bone;
        let keys = values
            .into_iter()
            .enumerate()
            .map(|(i, v)| Key::linear(i as f32 * 1e-6, v))
            .collect();
        s.doc.animations[id].timelines[0] = Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys,
        };
        s.doc.animations[id].duration = 0.0;
        let before = snapshot(&s);
        let out = graph_frame(&egui::Context::default(), &mut s, vec![]);
        assert!(
            out.shapes
                .iter()
                .all(|s| s.shape.visual_bounding_rect().is_finite())
        );
        assert_eq!(snapshot(&s), before);
    }
}
#[test]
fn t814_snap_badge_and_free_alt_do_not_change_authored_values() {
    let (mut s, _, _) = fixture();
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    assert!(texts(&graph_frame(&ctx, &mut s, vec![])).contains(&"snap: frames".into()));
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 900.0),
            )),
            modifiers: egui::Modifiers {
                alt: true,
                ..Default::default()
            },
            ..Default::default()
        },
        |ui| {
            let id = s.session.active_animation.unwrap();
            let model = model::TimelineModel::build(&s, id);
            graph::ui(
                ui,
                &mut s,
                id,
                &model,
                &ViewState::default(),
                &Layout {
                    sheet_x0: 30.0,
                    px_per_sec: 160.0,
                    scroll_sec: 0.0,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15.0, 50.0), egui::vec2(600.0, 500.0)),
                Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.0,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    );
    assert!(texts(&out).contains(&"free".into()));
    assert_eq!(snapshot(&s), before);
    let l = Layout {
        sheet_x0: 30.0,
        px_per_sec: 160.0,
        scroll_sec: 2.0,
        fps: 30,
    };
    assert!((l.x_to_time(l.time_to_x(2.123)) - 2.123).abs() < 1e-6);
    assert!((l.snap_time(2.123) - 64.0 / 30.0).abs() < 1e-6);
}

#[test]
fn t814_multiple_channels_and_overshoot_are_framed_without_clamping_data() {
    let (mut s, id, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        unreachable!()
    };
    s.doc.animations[id]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: vec![
                Key::linear(0.0, 0.0),
                Key {
                    time: 1.0,
                    value: 100.0,
                    interp: Interp::Bezier {
                        out_handle: glam::vec2(0.25, 4.0),
                        in_handle: glam::vec2(0.75, 5.0),
                    },
                },
            ],
        });
    let before = snapshot(&s);
    let out = graph_frame(&egui::Context::default(), &mut s, vec![]);
    assert!(texts(&out).iter().any(|t| t.starts_with("overshoot ")));
    assert!(
        out.shapes
            .iter()
            .all(|s| s.shape.visual_bounding_rect().is_finite())
    );
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_nonfinite_sample_gap_is_explicit_and_does_not_rewrite_handles() {
    let (mut s, id, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        unreachable!()
    };
    s.doc.animations[id].timelines[0] = Timeline::BoneTranslate {
        bone,
        axis: ankhimate_core::animation::Axis::X,
        keys: vec![Key::linear(0.0, -f32::MAX), Key::linear(1.0, f32::MAX)],
    };
    let before = snapshot(&s);
    let out = graph_frame(&egui::Context::default(), &mut s, vec![]);
    assert!(
        texts(&out)
            .iter()
            .any(|t| t.starts_with("Non-finite samples:"))
    );
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t814_headless_numeric_text_edit_waits_for_apply_button() {
    let (mut s, id, key) = fixture();
    selection::set(&mut s, id, vec![key.clone()]);
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let frame = |s: &mut AppState, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| crate::ui::inspector::tangents::ui(ui, s),
        )
    };
    let find = |o: &egui::FullOutput, text: &str| {
        o.shapes
            .iter()
            .find_map(|s| {
                if let egui::Shape::Text(t) = &s.shape
                    && t.galley.text() == text
                {
                    Some(t.pos + t.galley.size() * 0.5)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Missing {text}; {:?}", texts(o)))
    };
    frame(&mut s, vec![]);
    let out = frame(&mut s, vec![]);
    let pos = find(&out, "-0.5");
    frame(&mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    frame(
        &mut s,
        vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            },
            egui::Event::Text("-2.5".into()),
        ],
    );
    assert_eq!(snapshot(&s), before, "Typing is a draft, not a command");
    let out = frame(&mut s, vec![]);
    assert!(texts(&out).contains(&"-2.5".into()));
    let pos = find(&out, "Apply tangents");
    frame(&mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    let Interp::Bezier { out_handle, .. } = interp(&s, id, &key) else {
        unreachable!()
    };
    assert_eq!(out_handle.y, -2.5);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
