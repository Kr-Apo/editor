use super::*;

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
    egui::pos2(110., 50. + ROW_H * 1.5)
}
#[test]
fn t831_actual_sheet_selects_path_and_context_edits_one_native_key() {
    let (mut s, a, c) = crate::path_keys::tests::state();
    let ctx = egui::Context::default();
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    frame(&ctx, &mut s, vec![]);
    frame(&ctx, &mut s, vec![]);
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    click(&ctx, &mut s, keypos());
    assert_eq!(
        crate::key_selection::read(&mut s, a),
        vec![ankhimate_document::commands::key_cmds::KeyRef {
            addr: TimelineAddr::PathPosition { constraint: c },
            index: 1
        }]
    );
    for pressed in [true, false] {
        frame(
            &ctx,
            &mut s,
            pointer(keypos(), egui::PointerButton::Secondary, pressed),
        );
    }
    frame(&ctx, &mut s, vec![]);
    let pos = ctx
        .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t831", "delete"))))
        .expect("real sheet Path menu did not open")
        .center();
    click(&ctx, &mut s, pos);
    assert_eq!(
        ankhimate_document::path_keys::Capture::read(&s.doc, a, c)
            .unwrap()
            .keys
            .len(),
        2
    );
    assert_eq!(s.history.undo_depth(), 1);
    s.undo();
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
}
