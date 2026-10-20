use super::*;
use crate::tangent_linking::{self, Mode, Side, tests::fixture};
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
            let model = super::super::model::TimelineModel::build(s, anim);
            super::ui(
                ui,
                s,
                anim,
                &model,
                &ViewState::default(),
                &Layout {
                    sheet_x0: 30.,
                    px_per_sec: 160.,
                    scroll_sec: 0.,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(600., 500.)),
                super::super::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn pointer(pos: egui::Pos2, pressed: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(pos),
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Default::default(),
        },
    ]
}
fn start(ctx: &egui::Context, s: &mut AppState) -> egui::Pos2 {
    let out = frame(ctx, s, vec![]);
    // B incoming = .75s => x150. Locate actual painted handle, not a guessed y.
    let pos = out
        .shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Circle(c) = &sh.shape {
                (c.radius == HANDLE_R && (c.center.x - 150.).abs() < 0.01).then_some(c.center)
            } else {
                None
            }
        })
        .expect("Incoming handle must be drawn");
    frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
    frame(ctx, s, pointer(pos, true));
    pos
}
#[test]
fn t820_real_graph_drag_links_two_interps_and_merges() {
    let (mut s, _, _) = fixture();
    tangent_linking::choose(&mut s, Mode::Mirrored).unwrap();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let ctx = egui::Context::default();
    let pos = start(&ctx, &mut s);
    for d in [10., 20., 30.] {
        frame(
            &ctx,
            &mut s,
            vec![egui::Event::PointerMoved(pos + egui::vec2(0., d))],
        );
    }
    frame(&ctx, &mut s, pointer(pos + egui::vec2(0., 30.), false));
    assert_eq!(s.history.undo_depth(), 1);
    let j = tangent_linking::selected(&mut s).unwrap();
    let a = j.vector(Side::Incoming, j.handle(Side::Incoming));
    let b = j.vector(Side::Outgoing, j.handle(Side::Outgoing));
    assert!((a + b).length() < 1e-5);
    let after = ankhimate_document::snapshot::snapshot(&s.doc);
    assert!(before != after);
    for _ in 0..3 {
        s.undo();
        assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
        s.redo();
        assert!(ankhimate_document::snapshot::snapshot(&s.doc) == after);
    }
}
#[test]
fn t820_graph_undo_mid_pointer_gesture_cannot_resume_on_old_index() {
    let (mut s, _, _) = fixture();
    tangent_linking::choose(&mut s, Mode::Aligned).unwrap();
    let ctx = egui::Context::default();
    let pos = start(&ctx, &mut s);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(pos + egui::vec2(0., 20.))],
    );
    assert_eq!(s.history.undo_depth(), 1);
    s.undo();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(pos + egui::vec2(0., 40.))],
    );
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
}
#[test]
fn t820_graph_broken_drag_keeps_opposite_segment_unchanged() {
    let (mut s, a, _) = fixture();
    let before = tangent_linking::selected(&mut s).unwrap().keys[2].interp;
    let ctx = egui::Context::default();
    let pos = start(&ctx, &mut s);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(pos + egui::vec2(0., 20.))],
    );
    frame(&ctx, &mut s, pointer(pos + egui::vec2(0., 20.), false));
    assert_eq!(s.history.undo_depth(), 1);
    let Timeline::BoneTranslate { keys, .. } = &s.doc.animations[a].timelines[0] else {
        unreachable!()
    };
    assert_eq!(keys[2].interp, before);
}
