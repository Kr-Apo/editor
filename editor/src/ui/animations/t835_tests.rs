use super::*;
use ankhimate_core::animation::Animation;

fn texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
    fn visit(s: &egui::Shape, out: &mut Vec<String>) {
        match s {
            egui::Shape::Text(t) => out.push(t.galley.job.text.clone()),
            egui::Shape::Vec(v) => {
                for x in v {
                    visit(x, out)
                }
            }
            _ => {}
        }
    }
    let mut out = vec![];
    for s in shapes {
        visit(&s.shape, &mut out)
    }
    out
}
struct Harness {
    ctx: egui::Context,
    state: AppState,
}
impl Harness {
    fn new() -> Self {
        let mut state = AppState::default();
        for name in ["Zulu", "Été", "Idle"] {
            state.doc.animations.insert(Animation::new(name, 1.));
        }
        state.session.active_animation = state.doc.animations.keys().nth(1);
        let mut h = Self {
            ctx: Default::default(),
            state,
        };
        h.frame(vec![]);
        h.frame(vec![]);
        h
    }
    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800., 700.),
                )),
                time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.02),
                events,
                ..Default::default()
            },
            |ui| super::ui(ui, &mut self.state, &crate::theme::Theme::default()),
        )
    }
    fn pointer(&mut self, p: egui::Pos2, b: egui::PointerButton) {
        self.frame(vec![egui::Event::PointerMoved(p)]);
        for pressed in [true, false] {
            self.frame(vec![
                egui::Event::PointerMoved(p),
                egui::Event::PointerButton {
                    pos: p,
                    button: b,
                    pressed,
                    modifiers: Default::default(),
                },
            ]);
        }
        self.frame(vec![]);
    }
}
#[test]
fn t835_manager_exposes_actual_move_actions() {
    let mut h = Harness::new();
    h.pointer(egui::pos2(100., 75.), egui::PointerButton::Secondary);
    let out = h.frame(vec![]);
    let labels = texts(&out.shapes);
    assert!(
        labels.iter().any(|s| s == "Move Up"),
        "actual menu lacks Move Up: {labels:?}"
    );
    assert!(labels.iter().any(|s| s == "Move Down"));
}

#[test]
fn t835_active_delete_uses_first_author_clip_without_undoing_user_selection() {
    let mut h = Harness::new();
    let ids = h.ids();
    h.menu(ids[1]);
    h.action(ids[1], "up");
    h.menu(ids[1]);
    h.click_text("Delete");
    assert_eq!(h.state.session.active_animation, Some(ids[0]));
    assert_eq!(h.ids(), vec![ids[0], ids[2]]);
    h.state.undo();
    assert_eq!(h.ids(), vec![ids[1], ids[0], ids[2]]);
    assert_eq!(h.state.session.active_animation, Some(ids[0]));
    h.state.undo();
    assert_eq!(h.ids(), ids);
    h.state.redo();
    h.state.redo();
    assert_eq!(h.ids(), vec![ids[0], ids[2]]);
    assert_eq!(h.state.session.active_animation, Some(ids[0]));
}

impl Harness {
    fn ids(&self) -> Vec<AnimationId> {
        self.state.doc.animation_ids()
    }
    fn rect(&self, id: AnimationId, key: &str) -> (egui::Rect, bool) {
        self.ctx
            .data(|d| d.get_temp::<(egui::Rect, bool)>(egui::Id::new(("t835", (id, key)))))
            .expect("actual action/row")
    }
    fn menu(&mut self, id: AnimationId) {
        let p = self.rect(id, "row").0.center();
        self.pointer(p, egui::PointerButton::Secondary);
        for _ in 0..3 {
            self.frame(vec![]);
        }
    }
    fn action(&mut self, id: AnimationId, key: &str) {
        let p = self.rect(id, key).0.center();
        self.pointer(p, egui::PointerButton::Primary);
    }
    fn labels(&mut self) -> Vec<String> {
        texts(&self.frame(vec![]).shapes)
    }
    fn click_text(&mut self, label: &str) {
        fn find(s: &egui::Shape, label: &str) -> Option<egui::Pos2> {
            match s {
                egui::Shape::Text(t) if t.galley.job.text.ends_with(label) => {
                    Some(t.pos + egui::vec2(10., 5.))
                }
                egui::Shape::Vec(v) => v.iter().rev().find_map(|s| find(s, label)),
                _ => None,
            }
        }
        let out = self.frame(vec![]);
        let pos = out
            .shapes
            .iter()
            .rev()
            .find_map(|s| find(&s.shape, label))
            .expect("visible action text");
        self.pointer(pos, egui::PointerButton::Primary);
    }
}
#[test]
fn t835_actual_click_moves_only_order_and_three_cycles() {
    use ankhimate_document::snapshot::snapshot;
    let mut h = Harness::new();
    let ids = h.ids();
    let before = snapshot(&h.state.doc);
    h.menu(ids[1]);
    h.action(ids[1], "up");
    assert_eq!(h.ids(), vec![ids[1], ids[0], ids[2]]);
    assert_eq!(h.state.session.active_animation, Some(ids[1]));
    assert_eq!(h.state.history.undo_depth(), 1);
    let labels = h.labels();
    let names: Vec<_> = labels
        .iter()
        .filter(|s| ["Été", "Zulu", "Idle"].contains(&s.as_str()))
        .map(String::as_str)
        .collect();
    assert_eq!(names, vec!["Été", "Zulu", "Idle"]);
    let after = snapshot(&h.state.doc);
    for _ in 0..3 {
        h.state.undo();
        h.frame(vec![]);
        assert!(snapshot(&h.state.doc) == before);
        h.state.redo();
        h.frame(vec![]);
        assert!(snapshot(&h.state.doc) == after);
    }
    h.menu(ids[1]);
    h.action(ids[1], "down");
    assert_eq!(h.ids(), ids);
    assert_eq!(h.state.history.undo_depth(), 2);
}
#[test]
fn t835_boundary_buttons_are_disabled_and_clicks_preserve_redo() {
    let mut h = Harness::new();
    let ids = h.ids();
    h.menu(ids[0]);
    assert!(!h.rect(ids[0], "up").1);
    h.action(ids[0], "up");
    assert_eq!(h.state.history.undo_depth(), 0);
    h.menu(ids[2]);
    assert!(!h.rect(ids[2], "down").1);
    h.action(ids[2], "down");
    assert_eq!(h.ids(), ids);
    move_clip(&mut h.state, ids[1], Direction::Up);
    h.state.undo();
    h.menu(ids[0]);
    h.action(ids[0], "up");
    assert!(h.state.history.can_redo());
    assert_eq!(h.state.history.undo_depth(), 0);
}
#[test]
fn t835_modes_and_unmoved_active_identity_are_preserved() {
    for mode in [
        crate::session::WorkMode::Setup,
        crate::session::WorkMode::Animate,
    ] {
        let mut h = Harness::new();
        let ids = h.ids();
        h.state.session.work_mode = mode;
        h.state.session.active_animation = Some(ids[2]);
        h.menu(ids[0]);
        h.action(ids[0], "down");
        assert_eq!(h.state.session.active_animation, Some(ids[2]));
        assert_eq!(h.state.session.work_mode, mode);
        h.state.session.active_animation = Some(ids[0]);
        h.state.undo();
        assert_eq!(h.state.session.active_animation, Some(ids[0]));
        h.state.redo();
        assert_eq!(h.state.session.active_animation, Some(ids[0]));
    }
}
#[test]
fn t835_stale_popup_revision_clip_mode_document_and_hidden_frame_refuse() {
    use ankhimate_document::commands::key_cmds::SetAnimationMeta;
    for case in 0..5 {
        let mut h = Harness::new();
        let ids = h.ids();
        h.menu(ids[1]);
        let pos = h.rect(ids[1], "up").0.center();
        match case {
            0 => {
                h.state
                    .dispatch(Box::new(SetAnimationMeta::new(ids[0], 2., true)));
            }
            1 => h.state.session.active_animation = Some(ids[0]),
            2 => h.state.session.work_mode = crate::session::WorkMode::Animate,
            3 => {
                let d = h.state.doc.clone();
                h.state.replace_document(d);
            }
            _ => {
                for _ in 0..3 {
                    let _ = h.ctx.run_ui(Default::default(), |_| {});
                }
            }
        }
        let before = ankhimate_document::snapshot::snapshot(&h.state.doc);
        let depth = h.state.history.undo_depth();
        h.pointer(pos, egui::PointerButton::Primary);
        assert!(
            ankhimate_document::snapshot::snapshot(&h.state.doc) == before,
            "case {case}"
        );
        assert_eq!(h.state.history.undo_depth(), depth);
    }
}
#[test]
fn t835_display_empty_singleton_and_old_manager_actions() {
    let mut h = Harness::new();
    let ids = h.ids();
    let before = ankhimate_document::snapshot::snapshot(&h.state.doc);
    for _ in 0..10 {
        h.frame(vec![]);
    }
    assert!(ankhimate_document::snapshot::snapshot(&h.state.doc) == before);
    assert_eq!(h.state.history.undo_depth(), 0);
    h.menu(ids[1]);
    let labels = h.labels();
    for old in ["Rename…", "Duplicate", "Delete"] {
        assert!(labels.iter().any(|s| s.ends_with(old)));
    }
    h.click_text("Duplicate");
    assert_eq!(h.state.doc.animations.len(), 4);
    let copy = *h.ids().last().unwrap();
    assert_eq!(h.state.doc.animations[copy].name, "Été_2");
    h.menu(copy);
    h.click_text("Delete");
    assert!(!h.state.doc.animations.contains_key(copy));
    h.state.undo();
    assert!(h.state.doc.animations.contains_key(copy));
    h.state.redo();
    assert!(!h.state.doc.animations.contains_key(copy));
    h.state
        .replace_document(ankhimate_document::Document::new());
    let labels = h.labels();
    assert!(labels.iter().any(|s| s == "No animations yet"));
    h.state.create_animation();
    h.frame(vec![]);
    let id = h.ids()[0];
    h.menu(id);
    assert!(!h.rect(id, "up").1 && !h.rect(id, "down").1);
}
