use super::*;
use ankhimate_document::{clipboard::Clipboard, snapshot::snapshot};
#[allow(
    dead_code,
    reason = "Shared real weighted FFD fixture for actual egui interactions"
)]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t834_fixture.rs"
    ));
}
struct Harness {
    ctx: egui::Context,
    state: AppState,
    mods: egui::Modifiers,
}
impl Harness {
    fn new() -> Self {
        let r = rig::fixture(false);
        let mut s = AppState::default();
        s.replace_document(r.doc);
        s.session.active_animation = Some(r.source);
        s.set_work_mode(crate::session::WorkMode::Animate);
        let mut h = Self {
            ctx: Default::default(),
            state: s,
            mods: Default::default(),
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
                    egui::vec2(1000., 900.),
                )),
                time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.02),
                modifiers: self.mods,
                events,
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                let anim = self.state.session.active_animation.unwrap();
                let model = TimelineModel::build(&self.state, anim);
                super::ui(
                    ui,
                    &mut self.state,
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
                crate::ffd_clipboard::show(&ctx, &mut self.state, &crate::theme::Theme::default());
            },
        )
    }
    fn pointer(&mut self, p: egui::Pos2, button: egui::PointerButton) {
        self.frame(vec![egui::Event::PointerMoved(p)]);
        for pressed in [true, false] {
            self.frame(vec![
                egui::Event::PointerMoved(p),
                egui::Event::PointerButton {
                    pos: p,
                    button,
                    pressed,
                    modifiers: self.mods,
                },
            ]);
        }
        self.frame(vec![]);
    }
    fn key(&mut self, t: f32, ctrl: bool) {
        self.mods.ctrl = ctrl;
        self.pointer(
            egui::pos2(30. + 160. * t, 50. + ROW_H * 1.5),
            egui::PointerButton::Primary,
        );
        self.mods = Default::default();
    }
    fn context(&mut self) {
        self.pointer(
            egui::pos2(70., 50. + ROW_H * 1.5),
            egui::PointerButton::Secondary,
        );
    }
    fn click(&mut self, name: &str) {
        for _ in 0..5 {
            self.frame(vec![]);
        }
        let p = self
            .ctx
            .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t834", name))))
            .expect("actual visible action")
            .center();
        self.pointer(p, egui::PointerButton::Primary);
    }
    fn copy_three(&mut self) {
        self.key(0.25, false);
        self.key(0.75, true);
        self.key(1.25, true);
        let a = self.state.session.active_animation.unwrap();
        assert_eq!(crate::key_selection::read(&mut self.state, a).len(), 3);
        self.context();
        self.click("copy");
        assert!(matches!(&self.state.session.clipboard,Clipboard::Deform(c)if c.keys().len()==3));
    }
    fn paste_preview(&mut self, time: f32) {
        self.state.set_playhead(time);
        self.context();
        self.click("paste");
        assert!(self.state.session.ffd_paste.is_some());
    }
}
#[test]
fn t834_actual_sheet_ctrl_select_copy_preview_and_apply_one_undo() {
    let mut h = Harness::new();
    let before = snapshot(&h.state.doc);
    let rev = h.state.revision;
    h.copy_three();
    assert!(snapshot(&h.state.doc) == before);
    assert_eq!(h.state.revision, rev);
    assert_eq!(h.state.history.undo_depth(), 0);
    h.paste_preview(2.);
    assert!(snapshot(&h.state.doc) == before);
    h.click("apply");
    assert!(h.state.session.ffd_paste.is_none());
    assert_eq!(h.state.history.undo_depth(), 1);
    let a = h.state.session.active_animation.unwrap();
    let keys = rig::deform(&h.state.doc, a);
    assert_eq!(keys.len(), 6);
    assert_eq!(
        keys.iter().map(|k| k.time).collect::<Vec<_>>(),
        vec![0.25, 0.75, 1.25, 2., 2.5, 3.]
    );
    let after = snapshot(&h.state.doc);
    for _ in 0..3 {
        h.state.undo();
        assert!(snapshot(&h.state.doc) == before);
        h.state.redo();
        assert!(snapshot(&h.state.doc) == after);
    }
}
#[test]
fn t834_actual_sheet_cancel_escape_and_warning_acknowledgement() {
    let mut h = Harness::new();
    h.copy_three();
    let before = snapshot(&h.state.doc);
    h.paste_preview(5.);
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 0);
    h.click("cancel");
    assert!(h.state.session.ffd_paste.is_none());
    assert!(snapshot(&h.state.doc) == before);
    h.paste_preview(5.);
    h.frame(vec![egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }]);
    assert!(h.state.session.ffd_paste.is_none());
    h.paste_preview(5.);
    h.click("ack");
    h.click("apply");
    assert!(h.state.session.ffd_paste.is_none());
    assert_eq!(h.state.history.undo_depth(), 1);
    assert_eq!(
        h.state.doc.animations[h.state.session.active_animation.unwrap()].duration,
        4.
    );
}
#[test]
fn t834_actual_preview_stale_clipboard_playhead_selection_and_locks_disable_apply() {
    for change in 0..4 {
        let mut h = Harness::new();
        h.copy_three();
        h.paste_preview(2.);
        match change {
            0 => h.state.session.clipboard = Clipboard::Empty,
            1 => h.state.set_playhead(2.1),
            2 => h.state.session.key_selection = Default::default(),
            _ => {
                let b = h.state.doc.skeleton.bones.keys().next().unwrap();
                h.state.session.locked_bones.insert(b, true);
            }
        }
        let before = snapshot(&h.state.doc);
        h.click("apply");
        assert!(snapshot(&h.state.doc) == before);
        assert_eq!(h.state.history.undo_depth(), 0);
        h.click("cancel");
    }
}
