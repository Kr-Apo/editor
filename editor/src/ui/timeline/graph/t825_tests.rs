use super::*;
use crate::session::WorkMode;
use ankhimate_core::{animation::Key, math::Transform, skeleton::Bone};
fn fixture() -> (AppState, AnimationId, KeyRef) {
    let mut s = AppState::default();
    let bone = s.doc.skeleton.add_bone(Bone {
        name: "bone".into(),
        parent: None,
        length: 10.,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("clip", 2.);
    a.timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![
            Key::stepped(0., -10.),
            Key::stepped(1., 10.),
            Key::stepped(2., 0.),
        ],
    });
    let anim = s.doc.animations.insert(a);
    s.session.active_animation = Some(anim);
    s.set_work_mode(WorkMode::Animate);
    (
        s,
        anim,
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Rotate,
                axis: None,
            },
            index: 0,
        },
    )
}
fn rect() -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(600., 500.))
}
fn layout() -> Layout {
    Layout {
        sheet_x0: 30.,
        px_per_sec: 160.,
        scroll_sec: 0.,
        fps: 30,
    }
}
fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800., 900.),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            let anim = s.session.active_animation.unwrap();
            let model = TimelineModel::build(s, anim);
            super::ui(
                ui,
                s,
                anim,
                &model,
                &ViewState::default(),
                &layout(),
                rect(),
                super::super::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn pointer(p: egui::Pos2, pressed: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(p),
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Default::default(),
        },
    ]
}
fn begin(ctx: &egui::Context, s: &mut AppState) {
    frame(ctx, s, vec![]);
    frame(
        ctx,
        s,
        vec![egui::Event::PointerMoved(rect().min + egui::vec2(1., 1.))],
    );
    frame(ctx, s, pointer(rect().min + egui::vec2(1., 1.), true));
}
#[test]
fn t825_real_egui_marquee_selects_three_typed_keys_without_authoring() {
    let (mut s, anim, r) = fixture();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    let end = rect().max - egui::vec2(1., 1.);
    frame(&ctx, &mut s, vec![egui::Event::PointerMoved(end)]);
    frame(&ctx, &mut s, pointer(end, false));
    assert_eq!(
        crate::key_selection::read(&mut s, anim),
        (0..3)
            .map(|index| KeyRef {
                addr: r.addr.clone(),
                index
            })
            .collect::<Vec<_>>()
    );
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 0);
}

use crate::key_selection as selection;
use ankhimate_document::{commands::bone_cmds::SetBoneTransform, snapshot::snapshot};
fn finish(ctx: &egui::Context, s: &mut AppState) {
    let end = rect().max - egui::vec2(1., 1.);
    frame(ctx, s, vec![egui::Event::PointerMoved(end)]);
    frame(ctx, s, pointer(end, false));
}
fn render(
    ctx: &egui::Context,
    s: &mut AppState,
    events: Vec<egui::Event>,
    modifiers: egui::Modifiers,
    lay: Layout,
    view: ViewState,
    focused: bool,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800., 900.),
            )),
            events,
            modifiers,
            focused,
            ..Default::default()
        },
        |ui| {
            let anim = s.session.active_animation.unwrap();
            let model = TimelineModel::build(s, anim);
            super::ui(
                ui,
                s,
                anim,
                &model,
                &view,
                &lay,
                rect(),
                super::super::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn drag(
    ctx: &egui::Context,
    s: &mut AppState,
    start: egui::Pos2,
    end: egui::Pos2,
    mods: egui::Modifiers,
) {
    render(ctx, s, vec![], mods, layout(), ViewState::default(), true);
    render(
        ctx,
        s,
        vec![egui::Event::PointerMoved(start)],
        mods,
        layout(),
        ViewState::default(),
        true,
    );
    render(
        ctx,
        s,
        pointer(start, true),
        mods,
        layout(),
        ViewState::default(),
        true,
    );
    render(
        ctx,
        s,
        vec![egui::Event::PointerMoved(end)],
        mods,
        layout(),
        ViewState::default(),
        true,
    );
    render(
        ctx,
        s,
        pointer(end, false),
        mods,
        layout(),
        ViewState::default(),
        true,
    );
}
fn refs(r: &KeyRef) -> Vec<KeyRef> {
    (0..3)
        .map(|index| KeyRef {
            addr: r.addr.clone(),
            index,
        })
        .collect()
}
fn corners() -> [egui::Pos2; 4] {
    [
        rect().min + egui::vec2(1., 1.),
        rect().max - egui::vec2(1., 1.),
        rect().left_bottom() + egui::vec2(1., -1.),
        rect().right_top() + egui::vec2(-1., 1.),
    ]
}
#[test]
fn t825_real_drags_in_all_four_directions_select_same_keys() {
    let c = corners();
    for (a, b) in [(0, 1), (1, 0), (2, 3), (3, 2)] {
        let (mut s, anim, r) = fixture();
        let ctx = egui::Context::default();
        drag(&ctx, &mut s, c[a], c[b], Default::default());
        assert_eq!(selection::read(&mut s, anim), refs(&r));
    }
}
#[test]
fn t825_geometry_inside_outside_boundary_negative_and_clipped() {
    use marquee::{Point, band, pick};
    let (_, _, r) = fixture();
    let viewport = egui::Rect::from_min_max(egui::pos2(-10., -10.), egui::pos2(10., 10.));
    let points = vec![
        Point {
            key: r.clone(),
            center: egui::pos2(-5., -5.),
        },
        Point {
            key: KeyRef {
                index: 1,
                ..r.clone()
            },
            center: egui::pos2(5., 5.),
        },
        Point {
            key: KeyRef {
                index: 2,
                ..r.clone()
            },
            center: egui::pos2(12., 12.),
        },
    ];
    let b = band(egui::pos2(20., 20.), egui::pos2(-5., -5.), viewport).unwrap();
    assert_eq!(pick(&points, b, &[]), refs(&r)[..2]);
    assert!(band(egui::pos2(1., 1.), egui::pos2(1., 1.), viewport).is_none());
    for p in [egui::pos2(f32::NAN, 0.), egui::pos2(0., f32::INFINITY)] {
        assert!(band(p, egui::Pos2::ZERO, viewport).is_none());
    }
}
#[test]
fn t825_duplicate_screen_positions_preserve_full_typed_addresses() {
    let (mut s, anim, r) = fixture();
    let bone = match r.addr {
        TimelineAddr::Bone { bone, .. } => bone,
        _ => unreachable!(),
    };
    s.doc.animations[anim]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: vec![
                Key::stepped(0., -10.),
                Key::stepped(1., 10.),
                Key::stepped(2., 0.),
            ],
        });
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    finish(&ctx, &mut s);
    let selected = selection::read(&mut s, anim);
    assert_eq!(selected.len(), 6);
    assert_eq!(selected.iter().filter(|k| k.index == 0).count(), 2);
}
#[test]
fn t825_empty_replace_clears_and_ctrl_empty_preserves() {
    for ctrl in [false, true] {
        let (mut s, anim, r) = fixture();
        selection::set(&mut s, anim, vec![r.clone()]);
        let ctx = egui::Context::default();
        drag(
            &ctx,
            &mut s,
            egui::pos2(500., 60.),
            egui::pos2(600., 70.),
            egui::Modifiers {
                ctrl,
                ..Default::default()
            },
        );
        assert_eq!(
            selection::read(&mut s, anim),
            if ctrl { vec![r] } else { vec![] }
        );
    }
}
#[test]
fn t825_ctrl_unions_without_duplicates_and_shift_is_not_additive() {
    for ctrl in [true, false] {
        let (mut s, anim, r) = fixture();
        selection::set(&mut s, anim, vec![r.clone()]);
        let ctx = egui::Context::default();
        // A small band around the middle (high-valued) key, below the top status label.
        drag(
            &ctx,
            &mut s,
            egui::pos2(170., 75.),
            egui::pos2(210., 110.),
            egui::Modifiers {
                ctrl,
                shift: !ctrl,
                ..Default::default()
            },
        );
        let mut expected = if ctrl { vec![r.clone()] } else { vec![] };
        expected.push(KeyRef { index: 1, ..r });
        assert_eq!(selection::read(&mut s, anim), expected);
    }
}
#[test]
fn t825_escape_during_drag_preserves_selection_author_data_and_redo() {
    let (mut s, anim, r) = fixture();
    selection::set(&mut s, anim, vec![r.clone()]);
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(rect().center())],
    );
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
    finish(&ctx, &mut s);
    assert_eq!(selection::read(&mut s, anim), vec![r]);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 0);
}
#[test]
fn t825_click_and_subthreshold_jitter_do_not_replace_selection() {
    for jitter in [0., 0.1] {
        let (mut s, anim, r) = fixture();
        selection::set(&mut s, anim, vec![r.clone()]);
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        let p = rect().min + egui::vec2(1. + jitter, 1. + jitter);
        frame(&ctx, &mut s, vec![egui::Event::PointerMoved(p)]);
        frame(&ctx, &mut s, pointer(p, false));
        assert_eq!(selection::read(&mut s, anim), vec![r]);
    }
}
#[test]
fn t825_release_outside_and_pointer_gone_cancel_without_resume() {
    for gone in [false, true] {
        let (mut s, anim, r) = fixture();
        selection::set(&mut s, anim, vec![r.clone()]);
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(rect().center())],
        );
        if gone {
            frame(&ctx, &mut s, vec![egui::Event::PointerGone]);
        } else {
            frame(&ctx, &mut s, pointer(egui::pos2(750., 850.), false));
        }
        finish(&ctx, &mut s);
        assert_eq!(selection::read(&mut s, anim), vec![r]);
    }
}
#[test]
fn t825_focus_loss_and_hidden_graph_frame_cancel() {
    for hidden in [false, true] {
        let (mut s, anim, r) = fixture();
        selection::set(&mut s, anim, vec![r.clone()]);
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(rect().center())],
        );
        if hidden {
            let _ = ctx.run_ui(Default::default(), |_| {});
        } else {
            render(
                &ctx,
                &mut s,
                vec![],
                Default::default(),
                layout(),
                ViewState::default(),
                false,
            );
        }
        finish(&ctx, &mut s);
        assert_eq!(selection::read(&mut s, anim), vec![r]);
    }
}
#[test]
fn t825_direct_key_mutation_before_drag_threshold_never_selects_reused_index() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[anim].timelines[0] else {
        unreachable!()
    };
    keys[1].time = 1.1;
    finish(&ctx, &mut s);
    assert!(selection::read(&mut s, anim).is_empty());
}
#[test]
fn t825_changed_track_data_structure_time_and_mode_cancel() {
    for case in 0..5 {
        let (mut s, anim, _) = fixture();
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(rect().center())],
        );
        match case {
            0 => {
                if let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[anim].timelines[0]
                {
                    keys[0].value += 1.;
                }
            }
            1 => {
                if let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[anim].timelines[0]
                {
                    keys.remove(0);
                }
            }
            2 => {
                s.doc.animations[anim].timelines.clear();
            }
            3 => {
                s.session.work_mode = WorkMode::Setup;
            }
            _ => {
                s.revision += 1;
            }
        };
        finish(&ctx, &mut s);
        assert!(selection::read(&mut s, anim).is_empty(), "case {case}");
    }
}
#[test]
fn t825_external_key_selection_and_bone_selection_invalidate_gesture() {
    for bone_change in [false, true] {
        let (mut s, anim, r) = fixture();
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(rect().center())],
        );
        if bone_change {
            if let TimelineAddr::Bone { bone, .. } = r.addr {
                s.session.select_bone(Some(bone));
            }
        } else {
            selection::set(&mut s, anim, vec![r.clone()]);
        }
        finish(&ctx, &mut s);
        assert_eq!(
            selection::read(&mut s, anim),
            if bone_change { vec![] } else { vec![r] }
        );
    }
}
#[test]
fn t825_replaced_document_with_identical_indices_cannot_receive_old_selection() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    let d = s.doc.clone();
    s.replace_document(d);
    s.session.active_animation = Some(anim);
    s.set_work_mode(WorkMode::Animate);
    finish(&ctx, &mut s);
    assert!(selection::read(&mut s, anim).is_empty());
}
#[test]
fn t825_changed_animation_cannot_receive_old_selection() {
    let (mut s, anim, _) = fixture();
    let new = s.doc.animations.insert(s.doc.animations[anim].clone());
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    s.session.active_animation = Some(new);
    finish(&ctx, &mut s);
    assert!(selection::read(&mut s, new).is_empty());
}
#[test]
fn t825_undo_redo_invalidate_gesture_and_selection_is_not_history() {
    let (mut s, anim, r) = fixture();
    let bone = match r.addr {
        TimelineAddr::Bone { bone, .. } => bone,
        _ => unreachable!(),
    };
    s.set_work_mode(WorkMode::Setup);
    let mut local = s.doc.skeleton.bones[bone].local_transform;
    local.position.x += 5.;
    s.dispatch(Box::new(SetBoneTransform::new(bone, local)));
    s.set_work_mode(WorkMode::Animate);
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    s.undo();
    let before = snapshot(&s.doc);
    finish(&ctx, &mut s);
    assert!(selection::read(&mut s, anim).is_empty());
    assert!(snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
    s.redo();
    let after = snapshot(&s.doc);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t825_wheel_zoom_alt_middle_and_secondary_never_become_marquee() {
    for case in 0..4 {
        let (mut s, anim, _) = fixture();
        let ctx = egui::Context::default();
        if case == 0 {
            let c = corners();
            drag(
                &ctx,
                &mut s,
                c[0],
                c[1],
                egui::Modifiers {
                    alt: true,
                    ..Default::default()
                },
            );
        } else {
            begin(&ctx, &mut s);
            let event = match case {
                1 => egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    phase: egui::TouchPhase::Move,
                    delta: egui::vec2(0., 50.),
                    modifiers: Default::default(),
                },
                n => egui::Event::PointerButton {
                    pos: rect().center(),
                    button: if n == 2 {
                        egui::PointerButton::Middle
                    } else {
                        egui::PointerButton::Secondary
                    },
                    pressed: true,
                    modifiers: Default::default(),
                },
            };
            frame(&ctx, &mut s, vec![event]);
            finish(&ctx, &mut s);
        }
        assert!(selection::read(&mut s, anim).is_empty());
    }
}
#[test]
fn t825_changed_zoom_or_pan_cancels_existing_gesture() {
    for zoom in [false, true] {
        let (mut s, anim, _) = fixture();
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        let mut lay = layout();
        if zoom {
            lay.px_per_sec *= 2.
        } else {
            lay.scroll_sec = 0.2;
        }
        render(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(rect().center())],
            Default::default(),
            lay,
            ViewState::default(),
            true,
        );
        finish(&ctx, &mut s);
        assert!(selection::read(&mut s, anim).is_empty());
    }
}
#[test]
fn t825_existing_key_drag_has_priority_over_empty_background() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, vec![]);
    let p = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Circle(c) = &sh.shape {
                (c.radius == KEY_R && (c.center.x - 190.).abs() < 0.1).then_some(c.center)
            } else {
                None
            }
        })
        .unwrap();
    let before = snapshot(&s.doc);
    frame(&ctx, &mut s, vec![egui::Event::PointerMoved(p)]);
    frame(&ctx, &mut s, pointer(p, true));
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(p + egui::vec2(0., 25.))],
    );
    frame(&ctx, &mut s, pointer(p + egui::vec2(0., 25.), false));
    assert_eq!(selection::read(&mut s, anim).len(), 1);
    assert!(snapshot(&s.doc) != before);
    assert!(s.history.undo_depth() > 0);
}
#[test]
fn t825_curve_segment_crossing_empty_box_is_not_key_selection() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    drag(
        &ctx,
        &mut s,
        egui::pos2(90., 500.),
        egui::pos2(120., 535.),
        Default::default(),
    );
    assert!(selection::read(&mut s, anim).is_empty());
}

#[test]
fn t825_low_high_zoom_pan_select_only_visible_key_centres() {
    for (pps, scroll) in [(4., 0.), (6000., 0.), (160., 1.)] {
        let (mut s, anim, r) = fixture();
        let ctx = egui::Context::default();
        let lay = Layout {
            px_per_sec: pps,
            scroll_sec: scroll,
            ..layout()
        };
        let c = corners();
        for events in [
            vec![],
            vec![egui::Event::PointerMoved(c[0])],
            pointer(c[0], true),
            vec![egui::Event::PointerMoved(c[1])],
            pointer(c[1], false),
        ] {
            render(
                &ctx,
                &mut s,
                events,
                Default::default(),
                lay,
                ViewState::default(),
                true,
            );
        }
        let expected = refs(&r)
            .into_iter()
            .filter(|r| {
                let x = lay.time_to_x(r.index as f32);
                x >= rect().left() && x <= rect().right()
            })
            .collect::<Vec<_>>();
        assert_eq!(selection::read(&mut s, anim), expected);
    }
}
#[test]
fn t825_solo_and_folded_rows_do_not_add_hidden_tracks() {
    let (mut s, anim, r) = fixture();
    let bone = match r.addr {
        TimelineAddr::Bone { bone, .. } => bone,
        _ => unreachable!(),
    };
    s.doc.animations[anim]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: vec![
                Key::stepped(0., -10.),
                Key::stepped(1., 10.),
                Key::stepped(2., 0.),
            ],
        });
    let model = TimelineModel::build(&s, anim);
    let mut view = ViewState::default();
    view.soloed.insert(model.groups[0].rows[0].row_id);
    let ctx = egui::Context::default();
    let c = corners();
    for events in [
        vec![],
        vec![egui::Event::PointerMoved(c[0])],
        pointer(c[0], true),
        vec![egui::Event::PointerMoved(c[1])],
        pointer(c[1], false),
    ] {
        render(
            &ctx,
            &mut s,
            events,
            Default::default(),
            layout(),
            view.clone(),
            true,
        );
    }
    assert_eq!(selection::read(&mut s, anim), refs(&r));
    selection::set(&mut s, anim, vec![r.clone()]);
    ctx.data_mut(|d| d.insert_temp(egui::Id::new(("tl_fold", model.groups[0].fold_id)), true));
    drag(&ctx, &mut s, c[0], c[1], Default::default());
    assert_eq!(selection::read(&mut s, anim), vec![r]);
}
#[test]
fn t825_nonfinite_points_and_ambiguous_or_missing_track_are_never_guessed() {
    let (mut s, anim, r) = fixture();
    assert!(marquee::available(&s, &s.doc.animations[anim], &r.addr));
    let a = s.doc.animations[anim].timelines[0].clone();
    s.doc.animations[anim].timelines.push(a);
    assert!(!marquee::available(&s, &s.doc.animations[anim], &r.addr));
    s.doc.animations[anim].timelines.pop();
    if let TimelineAddr::Bone { bone, .. } = r.addr {
        s.doc.skeleton.remove_bone(bone);
    }
    assert!(!marquee::available(&s, &s.doc.animations[anim], &r.addr));
    let p = marquee::Point {
        key: r,
        center: egui::pos2(f32::NAN, 0.),
    };
    assert!(marquee::pick(&[p], rect(), &[]).is_empty());
}
#[test]
fn t825_ctrl_click_keeps_native_toggle_after_marquee() {
    let (mut s, anim, r) = fixture();
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    finish(&ctx, &mut s);
    let out = frame(&ctx, &mut s, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Circle(c) = &sh.shape {
                (c.radius == KEY_R && (c.center.x - 190.).abs() < 0.1).then_some(c.center)
            } else {
                None
            }
        })
        .unwrap();
    for events in [
        vec![egui::Event::PointerMoved(pos)],
        pointer(pos, true),
        pointer(pos, false),
    ] {
        render(
            &ctx,
            &mut s,
            events,
            egui::Modifiers {
                ctrl: true,
                ..Default::default()
            },
            layout(),
            ViewState::default(),
            true,
        );
    }
    assert_eq!(
        selection::read(&mut s, anim),
        vec![r.clone(), KeyRef { index: 2, ..r }]
    );
}
#[test]
fn t825_shared_dopesheet_selection_and_tangent_draft_invalidation() {
    use crate::tangent_linking::{self, Mode};
    use crate::ui::inspector::tangents::Draft;
    let (mut s, anim, _) = tangent_linking::tests::fixture();
    let draft = Draft::new(&mut s, anim).unwrap();
    tangent_linking::choose(&mut s, Mode::Mirrored).unwrap();
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    drag(
        &ctx,
        &mut s,
        egui::pos2(170., 51.),
        egui::pos2(400., 549.),
        Default::default(),
    );
    let selected = selection::read(&mut s, anim);
    assert_eq!(selected.len(), 2);
    assert_eq!(tangent_linking::mode(&mut s), Mode::Broken);
    assert!(tangent_linking::choose(&mut s, Mode::Aligned).is_err());
    assert!(draft.apply(&mut s).is_err());
    assert!(Draft::new(&mut s, anim).is_ok());
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800., 900.),
            )),
            ..Default::default()
        },
        |ui| {
            let model = TimelineModel::build(&s, anim);
            super::super::sheet::ui(
                ui,
                &mut s,
                anim,
                &model,
                &mut ViewState::default(),
                &layout(),
                rect(),
                super::super::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    );
    assert_eq!(selection::read(&mut s, anim), selected);
    assert!(
        output
            .shapes
            .iter()
            .filter(|sh| matches!(&sh.shape,egui::Shape::Rect(r) if r.stroke.width==2.))
            .count()
            >= 2
    );
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 0);
}
#[test]
fn t825_flat_large_negative_values_overshoot_and_deform_remain_display_only() {
    for values in [[-10., -10., -10.], [-1e30, 0., 1e30], [-30., 5., -10.]] {
        let (mut s, anim, _) = fixture();
        if let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
            for (k, v) in keys.iter_mut().zip(values) {
                k.value = v;
            }
        }
        let before = snapshot(&s.doc);
        let ctx = egui::Context::default();
        begin(&ctx, &mut s);
        finish(&ctx, &mut s);
        assert_eq!(selection::read(&mut s, anim).len(), 3);
        assert!(snapshot(&s.doc) == before);
    }
    // Existing overshooting Bezier fixture: no interpolation is changed by selection.
    let (mut s, anim, _) = super::super::t814_tests::fixture();
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    finish(&ctx, &mut s);
    assert_eq!(selection::read(&mut s, anim).len(), 2);
    assert!(snapshot(&s.doc) == before);
}
#[allow(
    dead_code,
    reason = "Shared accepted native weighted fixture; no geometry or project edits outside tests"
)]
mod support {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
#[test]
fn t825_save_reload_assets_native_export_and_ffd_selection_contract_unchanged() {
    let (mut doc, _, slot) = support::fixture(true);
    let a = doc.animations.values_mut().next().unwrap();
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![Key::linear(0., vec![glam::Vec2::ZERO; 8])],
    });
    let mut s = AppState::default();
    s.replace_document(doc);
    let anim = s.doc.animations.keys().next().unwrap();
    s.session.active_animation = Some(anim);
    s.set_work_mode(WorkMode::Animate);
    let before = snapshot(&s.doc);
    let rev = s.revision;
    let schema = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let preset = ankhimate_export::presets::default_preset();
    let a = ankhimate_export::run::plan(&schema, &s.doc.assets, &preset).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("copy.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    finish(&ctx, &mut s);
    let selected = selection::read(&mut s, anim);
    assert_eq!(selected.len(), 3);
    assert!(
        selected
            .iter()
            .all(|k| !matches!(k.addr, TimelineAddr::Deform { .. }))
    );
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), 0);
    let b = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    assert_eq!(a.files, b.files);
    assert_eq!(a.binaries, b.binaries);
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let loaded = ankhimate_formats::load(&path).unwrap().0;
    assert!(loaded.report.dangling.is_empty() && loaded.report.lossy.is_empty());
    let loaded = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        loaded.assets.images.values().next().unwrap().bytes,
        s.doc.assets.images.values().next().unwrap().bytes
    );
}
#[test]
fn t825_observational_cpu_100_1000_10000_keys() {
    let Some(path) = std::env::var_os("T825_PERF_PATH") else {
        return;
    };
    let mut rows = vec![];
    for n in [100, 1000, 10000] {
        let (mut s, anim, r) = fixture();
        let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[anim].timelines[0] else {
            unreachable!()
        };
        *keys = (0..n)
            .map(|i| Key::stepped(i as f32 / (n - 1) as f32 * 2., 0.))
            .collect();
        let points = (0..n)
            .map(|index| marquee::Point {
                key: KeyRef {
                    addr: r.addr.clone(),
                    index,
                },
                center: egui::pos2(30. + index as f32 / (n - 1) as f32 * 320., 300.),
            })
            .collect::<Vec<_>>();
        let ctx = egui::Context::default();
        frame(&ctx, &mut s, vec![]);
        let mut display = vec![];
        let mut search = vec![];
        let mut apply = vec![];
        for _ in 0..3 {
            s.session.key_selection = Default::default();
            let t = std::time::Instant::now();
            std::hint::black_box(frame(&ctx, &mut s, vec![]));
            display.push(t.elapsed().as_secs_f64() * 1000.);
            let t = std::time::Instant::now();
            let picked = marquee::pick(std::hint::black_box(&points), rect(), &[]);
            search.push(t.elapsed().as_secs_f64() * 1000.);
            assert_eq!(picked.len(), n);
            let t = std::time::Instant::now();
            selection::set(&mut s, anim, picked);
            apply.push(t.elapsed().as_secs_f64() * 1000.);
        }
        rows.push(serde_json::json!({"keys":n,"profile":if cfg!(debug_assertions){"debug"}else{"release"},"warmupFrames":1,"runs":3,"displayMs":display,"hitSearchMs":search,"nativeSelectionSetMs":apply,"scope":"in-memory egui display/point gathering; rectangle hit search + typed union; native KeySelection set, separately; no GPU"}));
    }
    std::fs::write(path, serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}

#[test]
fn t825_successful_selection_preserves_redo_content_save_point_and_revision() {
    let (mut s, anim, r) = fixture();
    let TimelineAddr::Bone { bone, .. } = r.addr else {
        unreachable!()
    };
    s.set_work_mode(WorkMode::Setup);
    let mut local = s.doc.skeleton.bones[bone].local_transform;
    local.position.x += 10.;
    s.dispatch(Box::new(SetBoneTransform::new(bone, local)));
    s.undo();
    s.set_work_mode(WorkMode::Animate);
    let before = snapshot(&s.doc);
    let revision = s.revision;
    assert!(s.history.can_redo());
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    finish(&ctx, &mut s);
    assert_eq!(selection::read(&mut s, anim).len(), 3);
    assert!(s.history.can_redo());
    assert_eq!(s.history.undo_depth(), 0);
    assert_eq!(s.revision, revision);
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t825_rectangle_feedback_is_clipped_and_selection_is_published_only_on_release() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    begin(&ctx, &mut s);
    let end = rect().center();
    let output = frame(&ctx, &mut s, vec![egui::Event::PointerMoved(end)]);
    let band = egui::Rect::from_two_pos(rect().min + egui::vec2(1., 1.), end);
    assert!(output.shapes.iter().any(|sh|matches!(&sh.shape,egui::Shape::Rect(r) if r.rect==band && r.stroke.width==1. && sh.clip_rect.contains_rect(band))));
    assert!(selection::read(&mut s, anim).is_empty());
    frame(&ctx, &mut s, pointer(end, false));
    assert_eq!(selection::read(&mut s, anim).len(), 1);
}
#[test]
fn t825_ctrl_union_with_existing_ffd_is_refused_without_changing_it() {
    let (mut doc, skin, slot) = support::fixture(true);
    let anim = doc.animations.keys().next().unwrap();
    doc.animations[anim].timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![Key::linear(0., vec![glam::Vec2::ZERO; 8])],
    });
    let mut s = AppState::default();
    s.replace_document(doc);
    s.session.active_animation = Some(anim);
    s.set_work_mode(WorkMode::Animate);
    let r = KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index: 0,
    };
    selection::set(&mut s, anim, vec![r.clone()]);
    assert_eq!(selection::read(&mut s, anim), vec![r.clone()]);
    let ctx = egui::Context::default();
    let c = corners();
    drag(
        &ctx,
        &mut s,
        c[0],
        c[1],
        egui::Modifiers {
            ctrl: true,
            ..Default::default()
        },
    );
    assert_eq!(selection::read(&mut s, anim), vec![r]);
    assert_eq!(s.history.undo_depth(), 0);
}
