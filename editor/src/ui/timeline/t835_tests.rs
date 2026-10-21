use super::*;
use ankhimate_core::animation::Animation;
use ankhimate_document::commands::animation_order::{Direction, MoveAnimation};
struct Harness {
    ctx: egui::Context,
    s: AppState,
}
impl Harness {
    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800., 700.),
                )),
                events,
                ..Default::default()
            },
            |ui| pick_clip_combo(ui, &mut self.s),
        )
    }
    fn click(&mut self, p: egui::Pos2) {
        self.frame(vec![egui::Event::PointerMoved(p)]);
        for pressed in [true, false] {
            self.frame(vec![
                egui::Event::PointerMoved(p),
                egui::Event::PointerButton {
                    pos: p,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                },
            ]);
        }
        self.frame(vec![]);
    }
}
#[test]
fn t835_actual_picker_displays_native_order_after_move_and_selects_same_ids() {
    let mut h = Harness {
        ctx: Default::default(),
        s: AppState::default(),
    };
    let ids: Vec<_> = ["Zulu", "Été", "Idle"]
        .iter()
        .map(|n| h.s.doc.animations.insert(Animation::new(*n, 1.)))
        .collect();
    h.s.session.active_animation = Some(ids[1]);
    let c = MoveAnimation::prepare(&h.s.doc, ids[1], Direction::Up)
        .unwrap()
        .unwrap();
    h.s.dispatch(Box::new(c));
    h.frame(vec![]);
    h.frame(vec![]);
    let rect = h
        .ctx
        .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t835", "picker"))))
        .unwrap();
    h.click(rect.center());
    for _ in 0..3 {
        h.frame(vec![]);
    }
    let out = h.frame(vec![]);
    let labels: Vec<_> = out
        .shapes
        .iter()
        .filter_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                Some((t.galley.job.text.clone(), t.pos))
            } else {
                None
            }
        })
        .collect();
    let names: Vec<_> = labels
        .iter()
        .filter(|(s, _)| ["Zulu", "Été", "Idle"].contains(&s.as_str()))
        .map(|(s, _)| s.as_str())
        .collect();
    assert!(names.ends_with(&["Été", "Zulu", "Idle"]), "{names:?}");
    let pos = labels.iter().rev().find(|(s, _)| s == "Zulu").unwrap().1 + egui::vec2(5., 5.);
    h.click(pos);
    assert_eq!(h.s.session.active_animation, Some(ids[0]));
    assert!(h.s.session.is_animating());
    assert_eq!(h.s.doc.animation_ids(), vec![ids[1], ids[0], ids[2]]);
}
