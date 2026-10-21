use super::*;
use crate::session::WorkMode;
use ankhimate_document::snapshot::snapshot;
fn frame(ctx: &egui::Context, state: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    crate::ui::motion::configure(ctx, true);
    crate::theme::Theme::default().apply(ctx);
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            ui.horizontal(|ui| super::ui(ui, state));
        },
    )
}
fn locate(ctx: &egui::Context, s: &mut AppState, text: &str) -> egui::Pos2 {
    for _ in 0..30 {
        frame(ctx, s, vec![]);
    }
    let out = frame(ctx, s, vec![]);
    out.shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                let p = t.pos + t.galley.size() * 0.5;
                (t.galley.text() == text && sh.clip_rect.contains(p)).then_some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Missing visible control: {text}"))
}
fn click(ctx: &egui::Context, s: &mut AppState, p: egui::Pos2, button: egui::PointerButton) {
    frame(ctx, s, vec![egui::Event::PointerMoved(p)]);
    for pressed in [true, false] {
        frame(
            ctx,
            s,
            vec![egui::Event::PointerButton {
                pos: p,
                button,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
}
#[test]
fn t824_egui_real_toggle_and_context_settings_leave_document_and_history_unchanged() {
    let mut s = AppState::default();
    s.create_animation();
    s.set_work_mode(WorkMode::Animate);
    let ctx = egui::Context::default();
    let before = snapshot(&s.doc);
    let depth = s.history.undo_depth();
    let rev = s.revision;
    let pos = locate(&ctx, &mut s, crate::ui::icons::ONION_SKIN);
    click(&ctx, &mut s, pos, egui::PointerButton::Primary);
    assert!(s.session.onion_skin.enabled);
    click(&ctx, &mut s, pos, egui::PointerButton::Secondary);
    locate(&ctx, &mut s, "Opacity");
    locate(&ctx, &mut s, "Distance falloff");
    let p = locate(&ctx, &mut s, "Step between keys");
    click(&ctx, &mut s, p, egui::PointerButton::Primary);
    assert!(s.session.onion_skin.key_steps);
    click(&ctx, &mut s, pos, egui::PointerButton::Secondary);
    let p = locate(&ctx, &mut s, "Selected bones only");
    click(&ctx, &mut s, p, egui::PointerButton::Primary);
    assert!(s.session.onion_skin.selected_only);
    click(&ctx, &mut s, pos, egui::PointerButton::Secondary);
    locate(&ctx, &mut s, "Past");
    locate(&ctx, &mut s, "Future");
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), depth);
    assert_eq!(s.revision, rev);
}
#[test]
fn t824_egui_setup_toggle_cannot_activate_onion() {
    let mut s = AppState::default();
    s.create_animation();
    s.set_work_mode(WorkMode::Setup);
    let ctx = egui::Context::default();
    let pos = locate(&ctx, &mut s, crate::ui::icons::ONION_SKIN);
    click(&ctx, &mut s, pos, egui::PointerButton::Primary);
    assert!(!s.session.onion_skin.enabled);
}
