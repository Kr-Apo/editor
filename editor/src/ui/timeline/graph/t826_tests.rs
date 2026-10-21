use super::*;
#[test]
fn t826_graph_header_exposes_normalize_control() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    let mut view = ViewState {
        mode: super::super::SheetMode::Graph,
        ..Default::default()
    };
    let out = ctx.run_ui(Default::default(), |ui| {
        super::super::header(ui, &mut s, &mut view)
    });
    assert!(
        out.shapes
            .iter()
            .any(|sh| matches!(&sh.shape,egui::Shape::Text(t) if t.galley.text()=="Normalize"))
    );
}

mod support {
    include!("t826_support.rs");
}
use crate::key_selection as selection;
use ankhimate_document::snapshot::snapshot;
use normalization::Range;
use support::*;
fn view(normalized: bool) -> ViewState {
    ViewState {
        normalized,
        ..Default::default()
    }
}
fn frame(
    ctx: &egui::Context,
    s: &mut AppState,
    norm: bool,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    render(ctx, s, view(norm), layout(), events, Default::default())
}
fn key_points(o: &egui::FullOutput) -> Vec<(egui::Pos2, egui::Color32)> {
    o.shapes
        .iter()
        .filter_map(|sh| {
            if let egui::Shape::Circle(c) = &sh.shape {
                (c.radius == KEY_R && c.fill != egui::Color32::TRANSPARENT)
                    .then_some((c.center, c.fill))
            } else {
                None
            }
        })
        .collect()
}
fn drag(
    ctx: &egui::Context,
    s: &mut AppState,
    norm: bool,
    a: egui::Pos2,
    b: egui::Pos2,
    ctrl: bool,
) {
    for events in [
        vec![],
        vec![egui::Event::PointerMoved(a)],
        pointer(a, true),
        vec![egui::Event::PointerMoved(b)],
        pointer(b, false),
    ] {
        render(
            ctx,
            s,
            view(norm),
            layout(),
            events,
            egui::Modifiers {
                ctrl,
                ..Default::default()
            },
        );
    }
}
#[test]
fn t826_different_native_units_have_identical_relative_profile() {
    for values in [[0., 250., 500.], [-45., 22.5, 90.], [0.5, 1., 1.5]] {
        let r = Range::collect(values).unwrap();
        assert_eq!(values.map(|v| r.relative(v as f64)), [0., 0.5, 1.]);
    }
}
#[test]
fn t826_equal_ranges_negative_and_extreme_finite_values() {
    for values in [
        [-10., 0., 10.],
        [-f32::MAX, 0., f32::MAX],
        [1e-30, 2e-30, 3e-30],
    ] {
        let r = Range::collect(values).unwrap();
        assert_eq!(r.relative(values[0] as f64), 0.);
        assert_eq!(r.relative(values[2] as f64), 1.);
        assert!((r.relative(values[1] as f64) - 0.5).abs() < 1e-6);
    }
    let r = Range::collect([1., f32::from_bits(1f32.to_bits() + 1)]).unwrap();
    assert_eq!(r.relative(1.), 0.);
    assert_eq!(r.relative(r.max), 1.);
}
#[test]
fn t826_constant_single_empty_and_invalid_reference_values() {
    for values in [vec![7.], vec![-10.; 5]] {
        let r = Range::collect(values).unwrap();
        assert_eq!(r.relative(r.min), 0.5);
        assert!(r.legend().contains("constant"));
    }
    assert!(Range::collect([]).is_none());
    assert!(Range::collect([f32::NAN, f32::INFINITY]).is_none());
    let r = Range::collect([f32::NAN, -2., f32::NEG_INFINITY, 2.]).unwrap();
    assert_eq!(r.count, 2);
    assert!(r.relative(f64::NAN).is_nan());
}
#[test]
fn t826_reference_scan_visits_each_value_once() {
    let visits = std::cell::Cell::new(0);
    let r = Range::collect((0..10000).map(|i| {
        visits.set(visits.get() + 1);
        i as f32
    }))
    .unwrap();
    assert_eq!(visits.get(), 10000);
    assert_eq!(r.count, 10000);
}
#[test]
fn t826_linear_stepped_bezier_sampling_and_overshoot_unchanged() {
    let r = Range::collect([0., 1.]).unwrap();
    for interp in [
        Interp::Linear,
        Interp::Stepped,
        Interp::Bezier {
            out_handle: glam::vec2(0.2, -3.),
            in_handle: glam::vec2(0.8, 4.),
        },
    ] {
        let (s, _, refs) = fixture();
        let TimelineAddr::Bone { bone, .. } = refs[0].addr else {
            unreachable!()
        };
        let t = Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::X,
            keys: vec![
                ankhimate_core::animation::Key::linear(0., 0.),
                ankhimate_core::animation::Key {
                    time: 1.,
                    value: 1.,
                    interp,
                },
            ],
        };
        for time in [0., 0.25, 0.5, 0.75, 1.] {
            let native = sample_channel(&t, time).unwrap();
            assert_eq!(r.relative(native as f64), native as f64);
        }
        assert_eq!(s.history.undo_depth(), 0);
    }
    assert!(r.relative(-0.5) < 0.);
    assert!(r.relative(1.5) > 1.);
}
#[test]
fn t826_displayed_multi_property_keys_and_segments_share_projection() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    let p = key_points(&out);
    assert_eq!(p.len(), 9);
    for index in 0..3 {
        assert_eq!(p[index].0, p[index + 3].0);
        assert_eq!(p[index].0, p[index + 6].0);
    }
    let model = TimelineModel::build(&s, anim);
    let rows = model.visible_rows(&|_| false);
    let ranges = normalization::ranges(&rows, &view(true));
    for (i, row) in rows.iter().enumerate() {
        if let VisibleRow::Property { data, .. } = row {
            let r = ranges[i][0].unwrap();
            let t = s.doc.animations[anim]
                .timelines
                .iter()
                .find(|t| data.addr.matches_timeline(t))
                .unwrap();
            for (k, v) in data.keys.iter().zip(&data.channels[0].values) {
                assert!(
                    (r.relative(sample_channel(t, k.time).unwrap() as f64) - r.relative(*v as f64))
                        .abs()
                        < 1e-7
                );
            }
        }
    }
}
#[test]
fn t826_axis_relative_label_and_no_handle_widgets_in_normalized_view() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let raw = frame(&ctx, &mut s, false, vec![]);
    assert!(
        raw.shapes
            .iter()
            .any(|sh| matches!(&sh.shape,egui::Shape::Circle(c)if c.radius==HANDLE_R))
    );
    let norm = frame(&ctx, &mut s, true, vec![]);
    for label in [
        "0",
        "0.5",
        "1",
        "Normalize: relative · View Only · Raw to edit",
    ] {
        assert!(
            norm.shapes
                .iter()
                .any(|sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text()==label)),
            "{label}"
        );
    }
    assert!(
        !norm
            .shapes
            .iter()
            .any(|sh| matches!(&sh.shape,egui::Shape::Circle(c)if c.radius==HANDLE_R))
    );
}
#[test]
fn t826_native_range_legends_identify_targets_constants_and_native_units() {
    let (s, _, refs) = fixture();
    for (r, vals) in refs.iter().zip([[0., 500.], [-45., 90.], [0.5, 1.5]]) {
        let label = normalization::label(&s, &r.addr, "property");
        assert!(label.contains("root"));
        let range = Range::collect(vals).unwrap();
        let text = range.legend();
        assert!(text.contains(&format!("{}, {}", vals[0], vals[1])));
    }
}
#[test]
fn t826_reference_ranges_include_finite_negative_and_beyond_clip_keys() {
    let (mut s, anim, _) = fixture();
    if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
        keys[0].time = -2.;
        keys[2].time = 5.;
    }
    let model = TimelineModel::build(&s, anim);
    let rows = model.visible_rows(&|_| false);
    let r = normalization::ranges(&rows, &view(true));
    let range = r.iter().find_map(|r| r.first().copied().flatten()).unwrap();
    assert_eq!((range.min, range.max), (0., 500.));
}
#[test]
fn t826_scale_is_stable_across_pan_zoom_and_fit_time_axis() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let before = key_points(&frame(&ctx, &mut s, true, vec![]));
    let after = key_points(&render(
        &ctx,
        &mut s,
        view(true),
        Layout {
            px_per_sec: 50.,
            scroll_sec: 0.1,
            ..layout()
        },
        vec![],
        Default::default(),
    ));
    for (pos, _) in after {
        let time = 0.1 + (pos.x - 30.) / 50.;
        let index = time.round() as usize;
        assert!((pos.y - before[index].0.y).abs() < 1e-4);
    }
}
#[test]
fn t826_empty_and_nonfinite_channels_are_not_manufactured() {
    let (mut s, anim, _) = fixture();
    s.doc.animations[anim].timelines.clear();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    assert!(key_points(&out).is_empty());
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text()=="No finite keys to normalize")));
    let (mut s, anim, _) = fixture();
    s.doc.animations[anim].timelines.truncate(1);
    if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
        keys[0].value = f32::NAN;
        keys[1].value = 7.;
        keys[2].value = f32::INFINITY;
    }
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    assert_eq!(key_points(&out).len(), 1);
    assert!(
        out.shapes
            .iter()
            .all(|s| s.shape.visual_bounding_rect().is_finite())
    );
}
#[test]
fn t826_overshoot_stays_outside_zero_one_and_is_visually_clipped() {
    let (mut s, anim, _) = fixture();
    s.doc.animations[anim].timelines.truncate(1);
    if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
        keys.truncate(2);
        keys[0].value = 0.;
        keys[1].value = 1.;
        keys[1].interp = Interp::Bezier {
            out_handle: glam::vec2(0.2, -3.),
            in_handle: glam::vec2(0.8, 4.),
        };
    }
    let before = snapshot(&s.doc);
    let out = frame(&egui::Context::default(), &mut s, true, vec![]);
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::Shape::Path(p)if p.points.iter().any(|p|p.y<rect().top()||p.y>rect().bottom()))));
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text().contains("Relative overshoot"))));
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t826_view_only_key_drag_and_graph_k_cannot_author_partial_edits() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    let p = key_points(&out)[1].0;
    let before = snapshot(&s.doc);
    let rev = s.revision;
    for events in [
        vec![egui::Event::PointerMoved(p)],
        pointer(p, true),
        vec![egui::Event::PointerMoved(p + egui::vec2(20., -50.))],
        pointer(p + egui::vec2(20., -50.), false),
        vec![egui::Event::Key {
            key: egui::Key::K,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    ] {
        frame(&ctx, &mut s, true, events);
    }
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), 0);
}
#[test]
fn t826_normalized_click_and_ctrl_toggle_keep_native_selection() {
    let (mut s, anim, _) = fixture();
    let ctx = egui::Context::default();
    let p = key_points(&frame(&ctx, &mut s, true, vec![]))[1].0;
    for events in [
        vec![egui::Event::PointerMoved(p)],
        pointer(p, true),
        pointer(p, false),
    ] {
        frame(&ctx, &mut s, true, events);
    }
    assert_eq!(selection::read(&mut s, anim).len(), 1);
    for events in [
        vec![egui::Event::PointerMoved(p)],
        pointer(p, true),
        pointer(p, false),
    ] {
        render(
            &ctx,
            &mut s,
            view(true),
            layout(),
            events,
            egui::Modifiers {
                ctrl: true,
                ..Default::default()
            },
        );
    }
    assert!(selection::read(&mut s, anim).is_empty());
}
#[test]
fn t826_normalized_marquee_selects_all_coincident_typed_points() {
    let (mut s, anim, _) = fixture();
    let before = snapshot(&s.doc);
    drag(
        &egui::Context::default(),
        &mut s,
        true,
        rect().min + egui::vec2(1., 1.),
        rect().max - egui::vec2(1., 1.),
        false,
    );
    let refs = selection::read(&mut s, anim);
    assert_eq!(refs.len(), 9);
    assert_eq!(refs.iter().filter(|r| r.index == 1).count(), 3);
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t826_normalized_ctrl_marquee_unions_without_duplicate_keys() {
    let (mut s, anim, refs) = fixture();
    selection::set(&mut s, anim, vec![refs[0].clone()]);
    drag(
        &egui::Context::default(),
        &mut s,
        true,
        egui::pos2(170., 280.),
        egui::pos2(210., 320.),
        true,
    );
    assert_eq!(selection::read(&mut s, anim).len(), 4);
}
#[test]
fn t826_mode_change_cancels_marquee_even_when_raw_and_relative_geometry_match() {
    let (mut s, anim, _) = fixture();
    s.doc.animations[anim].timelines.truncate(1);
    if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
        for (k, v) in keys.iter_mut().zip([0., 0.5, 1.]) {
            k.value = v;
        }
    }
    let ctx = egui::Context::default();
    let a = rect().min + egui::vec2(1., 1.);
    let b = rect().max - egui::vec2(1., 1.);
    for events in [
        vec![],
        vec![egui::Event::PointerMoved(a)],
        pointer(a, true),
        vec![egui::Event::PointerMoved(rect().center())],
    ] {
        frame(&ctx, &mut s, false, events);
    }
    frame(&ctx, &mut s, true, vec![egui::Event::PointerMoved(b)]);
    frame(&ctx, &mut s, true, pointer(b, false));
    assert!(selection::read(&mut s, anim).is_empty());
}
#[test]
fn t826_raw_key_capture_cannot_resume_after_toggle_back_while_held() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let p = key_points(&frame(&ctx, &mut s, false, vec![]))[1].0;
    let before = snapshot(&s.doc);
    frame(&ctx, &mut s, false, vec![egui::Event::PointerMoved(p)]);
    frame(&ctx, &mut s, false, pointer(p, true));
    frame(
        &ctx,
        &mut s,
        true,
        vec![egui::Event::PointerMoved(p + egui::vec2(0., 50.))],
    );
    frame(
        &ctx,
        &mut s,
        false,
        vec![egui::Event::PointerMoved(p + egui::vec2(0., 70.))],
    );
    frame(&ctx, &mut s, false, pointer(p + egui::vec2(0., 70.), false));
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t826_stale_revision_clip_and_document_cancel_normalized_marquee() {
    for case in 0..3 {
        let (mut s, anim, _) = fixture();
        let other = s.doc.animations.insert(s.doc.animations[anim].clone());
        let ctx = egui::Context::default();
        let a = rect().min + egui::vec2(1., 1.);
        let b = rect().max - egui::vec2(1., 1.);
        for events in [vec![], vec![egui::Event::PointerMoved(a)], pointer(a, true)] {
            frame(&ctx, &mut s, true, events);
        }
        match case {
            0 => s.revision += 1,
            1 => s.session.active_animation = Some(other),
            _ => {
                s.replace_document(s.doc.clone());
                s.session.active_animation = Some(anim);
                s.set_work_mode(crate::session::WorkMode::Animate);
            }
        }
        frame(&ctx, &mut s, true, vec![egui::Event::PointerMoved(b)]);
        frame(&ctx, &mut s, true, pointer(b, false));
        let active = s.session.active_animation.unwrap();
        assert!(selection::read(&mut s, active).is_empty());
    }
}
#[test]
fn t826_raw_geometry_matches_independent_t825_reference_and_returns_exactly() {
    let expected: Vec<u64> = serde_json::from_str(include_str!("t826_raw_reference.json")).unwrap();
    assert_eq!((0..3).map(raw_case).collect::<Vec<_>>(), expected);
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let before = signature(&frame(&ctx, &mut s, false, vec![]));
    frame(&ctx, &mut s, true, vec![]);
    let after = signature(&frame(&ctx, &mut s, false, vec![]));
    assert_eq!(before, after);
}

#[test]
fn t826_header_real_toggle_is_session_view_state_only() {
    let (mut s, _, _) = fixture();
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    let mut v = ViewState {
        mode: super::super::SheetMode::Graph,
        ..Default::default()
    };
    let draw = |s: &mut AppState, v: &mut ViewState, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1600., 900.),
                )),
                events,
                ..Default::default()
            },
            |ui| super::super::header(ui, s, v),
        )
    };
    let out = draw(&mut s, &mut v, vec![]);
    let p = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                (t.galley.text() == "Normalize").then_some(t.pos + t.galley.size() * 0.5)
            } else {
                None
            }
        })
        .unwrap();
    draw(&mut s, &mut v, vec![egui::Event::PointerMoved(p)]);
    draw(&mut s, &mut v, pointer(p, true));
    draw(&mut s, &mut v, pointer(p, false));
    assert!(v.normalized);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 0);
    ctx.data_mut(|d| d.insert_temp(egui::Id::new("tl_view"), v));
    s.replace_document(s.doc.clone());
    assert!(
        ctx.data(|d| d.get_temp::<ViewState>(egui::Id::new("tl_view")))
            .unwrap()
            .normalized
    );
    assert!(!ViewState::default().normalized);
}
#[test]
fn t826_property_colors_remain_distinct_on_dark_and_light_palettes() {
    for theme in [
        crate::theme::Theme::default(),
        crate::theme::Theme {
            panel_fill: "#ffffff".into(),
            channel_translate: "#1144aa".into(),
            channel_rotate: "#227733".into(),
            channel_scale: "#886611".into(),
            ..Default::default()
        },
    ] {
        let (mut s, anim, _) = fixture();
        let ctx = egui::Context::default();
        let out = ctx.run_ui(Default::default(), |ui| {
            let model = TimelineModel::build(&s, anim);
            super::ui(
                ui,
                &mut s,
                anim,
                &model,
                &view(true),
                &layout(),
                rect(),
                super::super::Style {
                    theme: &theme,
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        });
        let p = key_points(&out);
        for (i, name) in ["translate", "rotate", "scale"].iter().enumerate() {
            assert_eq!(p[i * 3].1, theme.channel_color(name));
        }
        assert_ne!(p[0].1, p[3].1);
    }
}
#[test]
fn t826_slot_alpha_and_shear_keep_native_units_and_project_independently() {
    let (mut s, anim, refs) = fixture();
    let TimelineAddr::Bone { bone, .. } = refs[0].addr else {
        unreachable!()
    };
    let slot = s
        .doc
        .skeleton
        .add_slot(ankhimate_core::slot::Slot::new("alpha".into(), bone));
    s.doc.animations[anim].timelines = vec![
        Timeline::SlotColor {
            slot,
            keys: vec![
                ankhimate_core::animation::Key::linear(0., [1., 1., 1., 0.2]),
                ankhimate_core::animation::Key::linear(1., [1., 1., 1., 0.8]),
            ],
        },
        Timeline::BoneShear {
            bone,
            axis: ankhimate_core::animation::Axis::Y,
            keys: vec![
                ankhimate_core::animation::Key::linear(0., -20.),
                ankhimate_core::animation::Key::linear(1., 40.),
            ],
        },
    ];
    let before = snapshot(&s.doc);
    let p = key_points(&frame(&egui::Context::default(), &mut s, true, vec![]));
    assert_eq!(p.len(), 4);
    assert_eq!(p[0].0, p[2].0);
    assert_eq!(p[1].0, p[3].0);
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t826_raw_t820_linked_handle_edits_still_use_native_metric_and_one_undo() {
    use crate::tangent_linking::{self, Mode, Side};
    let (mut s, _, _) = tangent_linking::tests::fixture();
    tangent_linking::choose(&mut s, Mode::Mirrored).unwrap();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, false, vec![]);
    let p = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Circle(c) = &sh.shape {
                (c.radius == HANDLE_R && (c.center.x - 150.).abs() < 0.01).then_some(c.center)
            } else {
                None
            }
        })
        .unwrap();
    let before = snapshot(&s.doc);
    for events in [
        vec![egui::Event::PointerMoved(p)],
        pointer(p, true),
        vec![egui::Event::PointerMoved(p + egui::vec2(0., 20.))],
        pointer(p + egui::vec2(0., 20.), false),
    ] {
        frame(&ctx, &mut s, false, events);
    }
    assert!(snapshot(&s.doc) != before);
    assert_eq!(s.history.undo_depth(), 1);
    let j = tangent_linking::selected(&mut s).unwrap();
    assert!(
        (j.vector(Side::Incoming, j.handle(Side::Incoming))
            + j.vector(Side::Outgoing, j.handle(Side::Outgoing)))
        .length()
            < 1e-5
    );
}
#[test]
fn t826_constant_and_single_key_are_centered_without_fake_motion() {
    for single in [false, true] {
        let (mut s, anim, _) = fixture();
        s.doc.animations[anim].timelines.truncate(1);
        if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
            for k in keys.iter_mut() {
                k.value = 42.;
            }
            if single {
                keys.truncate(1);
            }
        }
        let p = key_points(&frame(&egui::Context::default(), &mut s, true, vec![]));
        assert_eq!(p.len(), if single { 1 } else { 3 });
        assert!(p.iter().all(|p| (p.0.y - rect().center().y).abs() < 1e-5));
    }
}
#[test]
fn t826_invalid_time_and_hidden_rows_do_not_change_other_reference_ranges() {
    let (mut s, anim, _) = fixture();
    if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
        keys[0].time = f32::NAN;
    }
    let model = TimelineModel::build(&s, anim);
    let rows = model.visible_rows(&|_| false);
    let ranges = normalization::ranges(&rows, &view(true));
    assert_eq!(ranges[1][0].unwrap().count, 2);
    assert_eq!(ranges[1][0].unwrap().min, 250.);
    let mut v = view(true);
    v.soloed.insert(model.groups[0].rows[1].row_id);
    let ranges = normalization::ranges(&rows, &v);
    assert!(ranges[1].is_empty());
    assert_eq!(ranges[2][0].unwrap().min, -45.);
}
#[test]
fn t826_native_range_tooltip_is_reachable_without_an_author_edit() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    let p = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                t.galley
                    .text()
                    .contains("Normalize: relative")
                    .then_some(t.pos + t.galley.size() * 0.5)
            } else {
                None
            }
        })
        .unwrap();
    let mut text = String::new();
    // Egui treats every PointerMoved as motion, even at the same position.
    // Move once, then let the normal tooltip delay elapse without input.
    frame(&ctx, &mut s, true, vec![egui::Event::PointerMoved(p)]);
    for _ in 0..80 {
        let out = frame(&ctx, &mut s, true, vec![]);
        text = out
            .shapes
            .iter()
            .filter_map(|sh| {
                if let egui::Shape::Text(t) = &sh.shape {
                    Some(t.galley.text())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if text.contains("native [0, 500]") {
            break;
        }
    }
    assert!(text.contains("native [0, 500]"), "{text}");
    assert!(text.contains("root / translate x"));
    assert_eq!(s.history.undo_depth(), 0);
}
#[test]
fn t826_normalized_selection_preserves_redo_and_three_native_cycles() {
    use ankhimate_document::commands::bone_cmds::SetBoneTransform;
    let (mut s, anim, refs) = fixture();
    let TimelineAddr::Bone { bone, .. } = refs[0].addr else {
        unreachable!()
    };
    s.set_work_mode(crate::session::WorkMode::Setup);
    let before = snapshot(&s.doc);
    let mut local = s.doc.skeleton.bones[bone].local_transform;
    local.position.x += 10.;
    s.dispatch(Box::new(SetBoneTransform::new(bone, local)));
    let after = snapshot(&s.doc);
    s.undo();
    s.set_work_mode(crate::session::WorkMode::Animate);
    let rev = s.revision;
    drag(
        &egui::Context::default(),
        &mut s,
        true,
        rect().min + egui::vec2(1., 1.),
        rect().max - egui::vec2(1., 1.),
        false,
    );
    assert_eq!(selection::read(&mut s, anim).len(), 9);
    assert_eq!(s.revision, rev);
    assert!(s.history.can_redo());
    assert!(snapshot(&s.doc) == before);
    for _ in 0..3 {
        s.redo();
        assert!(snapshot(&s.doc) == after);
        s.undo();
        assert!(snapshot(&s.doc) == before);
    }
}
#[allow(
    dead_code,
    reason = "Reuse the accepted native weighted fixture without touching original assets"
)]
mod assets {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
#[test]
fn t826_save_reload_exports_assets_and_ffd_are_unchanged() {
    let (mut doc, _, slot) = assets::fixture(true);
    let a = doc.animations.values_mut().next().unwrap();
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![ankhimate_core::animation::Key::linear(
            0.,
            vec![glam::Vec2::ZERO; 8],
        )],
    });
    let mut s = AppState::default();
    s.replace_document(doc);
    let anim = s.doc.animations.keys().next().unwrap();
    s.session.active_animation = Some(anim);
    s.set_work_mode(crate::session::WorkMode::Animate);
    let before = snapshot(&s.doc);
    let rev = s.revision;
    let preset = ankhimate_export::presets::default_preset();
    let a = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("view.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, true, vec![]);
    assert_eq!(key_points(&out).len(), 3);
    let b = ankhimate_export::run::plan(
        &ankhimate_formats::convert::to_schema(&s.doc.as_project_ref()),
        &s.doc.assets,
        &preset,
    )
    .unwrap();
    assert_eq!(a.files, b.files);
    assert_eq!(a.binaries, b.binaries);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), 0);
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let loaded = ankhimate_formats::load(&path).unwrap().0;
    assert!(loaded.report.lossy.is_empty() && loaded.report.dangling.is_empty());
    let d = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(d.animations.len(), s.doc.animations.len());
    assert_eq!(
        d.assets.images.values().next().unwrap().bytes,
        s.doc.assets.images.values().next().unwrap().bytes
    );
}
#[test]
fn t826_observational_cpu_raw_normalize_100_1000_10000() {
    let Some(path) = std::env::var_os("T826_PERF_PATH") else {
        return;
    };
    let mut report = vec![];
    for n in [100, 1000, 10000] {
        let (mut s, anim, refs) = fixture();
        s.doc.animations[anim].timelines.truncate(1);
        if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0] {
            *keys = (0..n)
                .map(|i| {
                    ankhimate_core::animation::Key::stepped(
                        i as f32 / (n - 1) as f32 * 2.,
                        i as f32,
                    )
                })
                .collect();
        }
        let model = TimelineModel::build(&s, anim);
        let rows = model.visible_rows(&|_| false);
        let timeline = &s.doc.animations[anim].timelines[0];
        let range = Range::collect((0..n).map(|i| i as f32)).unwrap();
        let mut stats = vec![];
        let mut transform = vec![];
        let mut sampling = vec![];
        for _ in 0..3 {
            let t = std::time::Instant::now();
            std::hint::black_box(normalization::ranges(&rows, &view(true)));
            stats.push(t.elapsed().as_secs_f64() * 1000.);
            let t = std::time::Instant::now();
            let mut sum = 0.;
            for i in 0..n {
                sum += range.relative(std::hint::black_box(i as f64));
            }
            std::hint::black_box(sum);
            transform.push(t.elapsed().as_secs_f64() * 1000.);
            let t = std::time::Instant::now();
            for x in (15..=615).step_by(2) {
                std::hint::black_box(sample_channel(
                    timeline,
                    layout().x_to_time(x as f32).max(0.),
                ));
            }
            sampling.push(t.elapsed().as_secs_f64() * 1000.);
        }
        let ctx = egui::Context::default();
        let mut modes = vec![];
        for norm in [false, true] {
            frame(&ctx, &mut s, norm, vec![]);
            let mut times = vec![];
            for _ in 0..3 {
                let t = std::time::Instant::now();
                std::hint::black_box(frame(&ctx, &mut s, norm, vec![]));
                times.push(t.elapsed().as_secs_f64() * 1000.);
            }
            modes.push(serde_json::json!({"normalized":norm,"displayMs":times}));
        }
        let picked = (0..n)
            .map(|index| KeyRef {
                addr: refs[0].addr.clone(),
                index,
            })
            .collect();
        let t = std::time::Instant::now();
        selection::set(&mut s, anim, picked);
        let native_set = t.elapsed().as_secs_f64() * 1000.;
        report.push(serde_json::json!({"keys":n,"profile":"debug","runs":3,"rangeMs":stats,"transformMs":transform,"samplingMs":sampling,"modes":modes,"sharedNativeSelectionSetMs":native_set,"selectionRuns":1,"scope":"bounds; f64 relative transform; existing 301 samples; headless display including existing sampling and point gathering; shared selection separately, no GPU"}));
    }
    std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
}
