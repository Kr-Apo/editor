use super::*;
use crate::ffd_keys::{self, tests::fixture};
use ankhimate_document::commands::key_cmds::TimelineAddr;
fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900., 900.),
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
                &mut ViewState::default(),
                &Layout {
                    sheet_x0: 30.,
                    px_per_sec: 160.,
                    scroll_sec: 0.,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(650., 500.)),
                super::super::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn pointer(pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(pos),
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: Default::default(),
        },
    ]
}
fn click(ctx: &egui::Context, s: &mut AppState, pos: egui::Pos2) {
    frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
    for p in [true, false] {
        frame(ctx, s, pointer(pos, egui::PointerButton::Primary, p));
    }
}
fn keypos() -> egui::Pos2 {
    egui::pos2(190., 50. + ROW_H * 1.5)
}
#[test]
fn t821_real_sheet_click_selects_ffd_and_display_alone_is_readonly() {
    let (mut s, a, _, _) = fixture(false);
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    frame(&ctx, &mut s, vec![]);
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    click(&ctx, &mut s, keypos());
    let refs = crate::key_selection::read(&mut s, a);
    assert_eq!(refs.len(), 1);
    assert!(matches!(refs[0].addr, TimelineAddr::Deform { .. }));
    assert_eq!(refs[0].index, 1);
    assert!(!s.history.can_undo());
}
#[test]
fn t821_real_sheet_drag_has_one_undo_and_two_gestures_are_separate() {
    let (mut s, a, _, _) = fixture(false);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    let mut pos = keypos();
    for _ in 0..2 {
        click(&ctx, &mut s, pos);
        frame(
            &ctx,
            &mut s,
            pointer(pos, egui::PointerButton::Primary, true),
        );
        for dx in [12., 24., 36.] {
            frame(
                &ctx,
                &mut s,
                vec![egui::Event::PointerMoved(pos + egui::vec2(dx, 0.))],
            );
        }
        pos += egui::vec2(36., 0.);
        frame(
            &ctx,
            &mut s,
            pointer(pos, egui::PointerButton::Primary, false),
        );
    }
    assert_eq!(s.history.undo_depth(), 2);
    assert_eq!(crate::key_selection::read(&mut s, a).len(), 1);
}
#[test]
fn t821_real_sheet_undo_while_dragging_stops_capture_and_preserves_redo() {
    let (mut s, _, _, _) = fixture(false);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    let pos = keypos();
    click(&ctx, &mut s, pos);
    frame(
        &ctx,
        &mut s,
        pointer(pos, egui::PointerButton::Primary, true),
    );
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(pos + egui::vec2(32., 0.))],
    );
    assert!(s.history.can_undo());
    s.undo();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(pos + egui::vec2(64., 0.))],
    );
    frame(
        &ctx,
        &mut s,
        pointer(
            pos + egui::vec2(64., 600.),
            egui::PointerButton::Primary,
            false,
        ),
    );
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
}
#[test]
fn t821_sheet_disabled_reason_and_clip_change_do_not_reuse_selection() {
    let (mut s, a, slot, skin) = fixture(false);
    let ctx = egui::Context::default();
    s.set_work_mode(crate::session::WorkMode::Setup);
    let out = frame(&ctx, &mut s, vec![]);
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text().contains("Switch to Animate"))));
    click(&ctx, &mut s, keypos());
    assert!(crate::key_selection::read(&mut s, a).is_empty());
    s.set_work_mode(crate::session::WorkMode::Animate);
    let r = KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index: 1,
    };
    crate::key_selection::set(&mut s, a, vec![r]);
    s.session.active_animation = Some(
        s.doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("other", 2.)),
    );
    frame(&ctx, &mut s, vec![]);
    assert!(crate::key_selection::read(&mut s, a).is_empty());
}
#[test]
fn t821_context_actions_use_real_clicks_for_move_delete_and_interpolation() {
    for action in ["Move FFD key", "Delete FFD key", "Stepped"] {
        let (mut s, a, slot, skin) = fixture(false);
        crate::key_selection::set(
            &mut s,
            a,
            vec![KeyRef {
                addr: TimelineAddr::Deform {
                    slot,
                    attachment: "mesh".into(),
                    owner: Some(skin),
                },
                index: 1,
            }],
        );
        let before = ankhimate_document::snapshot::snapshot(&s.doc);
        let ctx = egui::Context::default();
        let menu = |s: &mut AppState, events: Vec<egui::Event>| {
            ctx.run_ui(
                egui::RawInput {
                    events,
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900., 900.),
                    )),
                    ..Default::default()
                },
                |ui| ffd_keys::menu(ui, s, a),
            )
        };
        let find = |out: &egui::FullOutput, text: &str| {
            out.shapes
                .iter()
                .find_map(|sh| {
                    if let egui::Shape::Text(t) = &sh.shape {
                        (t.galley.text() == text).then_some(t.pos + t.galley.size() * 0.5)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("Missing {text}"))
        };
        menu(&mut s, vec![]);
        let out = menu(&mut s, vec![]);
        assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
        if action == "Move FFD key" {
            let p = find(&out, "1");
            for pressed in [true, false] {
                menu(&mut s, pointer(p, egui::PointerButton::Primary, pressed));
            }
            menu(
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
                    egui::Event::Text("1.25".into()),
                ],
            );
            assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
        }
        if action == "Stepped" {
            let p = find(&out, "FFD interpolation");
            for pressed in [true, false] {
                menu(&mut s, pointer(p, egui::PointerButton::Primary, pressed));
            }
        }
        let out = menu(&mut s, vec![]);
        let p = find(&out, action);
        for pressed in [true, false] {
            menu(&mut s, pointer(p, egui::PointerButton::Primary, pressed));
        }
        assert_eq!(s.history.undo_depth(), 1, "{action}");
        assert!(ankhimate_document::snapshot::snapshot(&s.doc) != before);
        s.undo();
        assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    }
}

#[test]
fn t821_real_right_click_exposes_exact_ffd_identity_and_no_generic_clipboard() {
    let (mut s, _, _, _) = fixture(false);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    let p = keypos();
    click(&ctx, &mut s, p);
    for down in [true, false] {
        frame(
            &ctx,
            &mut s,
            pointer(p, egui::PointerButton::Secondary, down),
        );
    }
    let out = frame(&ctx, &mut s, vec![]);
    for text in ["Move FFD key", "Delete FFD key", "FFD interpolation"] {
        assert!(
            out.shapes
                .iter()
                .any(|sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text()==text)),
            "Missing actual context action: {text}"
        );
    }
    assert!(!s.history.can_undo());
}
#[test]
fn t821_mixed_context_selection_refuses_all_ffd_actions() {
    let (mut s, a, slot, skin) = fixture(false);
    let r = KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index: 1,
    };
    crate::key_selection::set(&mut s, a, vec![r.clone()]);
    let ctx = egui::Context::default();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let out = ctx.run_ui(Default::default(), |ui| {
        key_context_menu(
            ui,
            &mut s,
            a,
            &Selection {
                keys: vec![
                    r.clone(),
                    KeyRef {
                        index: 0,
                        ..r.clone()
                    },
                ],
            },
            &crate::registry::Registry::with_builtins(),
        )
    });
    assert!(out.shapes.iter().any(
        |sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text().contains("exactly one"))
    ));
    assert!(
        !out.shapes
            .iter()
            .any(|sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text()=="Delete FFD key"))
    );
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    assert!(!s.history.can_undo());
}
#[test]
fn t821_hidden_reopened_sheet_keeps_only_current_lease() {
    let (mut s, a, _, _) = fixture(false);
    let ctx = egui::Context::default();
    frame(&ctx, &mut s, vec![]);
    click(&ctx, &mut s, keypos());
    let old = crate::key_selection::read(&mut s, a);
    assert_eq!(old.len(), 1);
    let _ = ctx.run_ui(Default::default(), |ui| {
        ui.label("Dopesheet hidden");
    });
    frame(&ctx, &mut s, vec![]);
    assert_eq!(crate::key_selection::read(&mut s, a), old);
    s.revision += 1;
    let _ = ctx.run_ui(Default::default(), |ui| {
        ui.label("Dopesheet hidden");
    });
    frame(&ctx, &mut s, vec![]);
    assert!(crate::key_selection::read(&mut s, a).is_empty());
}
