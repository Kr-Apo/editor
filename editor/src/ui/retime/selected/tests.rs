use super::*;
use crate::session::WorkMode;
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    skeleton::Bone,
};
use ankhimate_document::{commands::document_cmds::SetProjectFps, snapshot::snapshot};
fn state() -> AppState {
    let mut s = AppState::default();
    let b = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("clip", 2.);
    a.timelines.push(Timeline::BoneRotate {
        bone: b,
        keys: vec![Key::linear(0.25, 0.), Key::linear(1., 45.)],
    });
    s.session.active_animation = Some(s.doc.animations.insert(a));
    s.session.work_mode = WorkMode::Animate;
    s.session.selected_bones = vec![b];
    s.refresh_pose();
    s
}
struct Harness {
    ctx: egui::Context,
    panel: Panel,
    state: AppState,
}
impl Harness {
    fn new() -> Self {
        let mut h = Self {
            ctx: Default::default(),
            panel: Default::default(),
            state: state(),
        };
        h.frame(vec![]);
        h.frame(vec![]);
        h
    }
    fn frame(&mut self, events: Vec<egui::Event>) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000., 900.),
            )),
            time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.02),
            events,
            ..Default::default()
        };
        let _ = self.ctx.run_ui(input, |ui| {
            let ctx = ui.ctx().clone();
            ui.horizontal(|ui| {
                let r = ui.menu_button("Pose", |ui| self.panel.menu(ui, &self.state));
                super::probe(&ctx, "pose", &r.response);
            });
            self.panel.show(&ctx, &mut self.state, &Theme::default());
        });
    }
    fn click(&mut self, name: &str) {
        self.frame(vec![]);
        let p = self
            .ctx
            .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t833", name))))
            .unwrap()
            .center();
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
    fn open(&mut self) {
        self.click("pose");
        self.click("menu");
        assert!(self.panel.draft.is_some());
        self.frame(vec![]);
    }
    fn text(&mut self, name: &str, text: &str) {
        self.click(name);
        let mods = egui::Modifiers {
            ctrl: true,
            command: true,
            ..Default::default()
        };
        self.frame(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: mods,
            },
            egui::Event::Text(text.into()),
        ]);
        self.frame(vec![]);
    }
}
fn draft(frames: &str) -> Draft {
    Draft {
        frames: frames.into(),
        ..Default::default()
    }
}
#[test]
fn t833_egui_real_pose_menu_text_prepare_apply_and_no_double_application() {
    let mut h = Harness::new();
    let before = snapshot(&h.state.doc);
    let rev = h.state.revision;
    h.open();
    h.text("frames", "3");
    for _ in 0..5 {
        h.frame(vec![]);
    }
    assert!(h.panel.draft.as_ref().unwrap().prepared.is_none());
    assert!(snapshot(&h.state.doc) == before);
    assert_eq!(h.state.revision, rev);
    h.click("prepare");
    let p = &h.panel.draft.as_ref().unwrap().prepared.as_ref().unwrap().1;
    assert_eq!(p.summary.offset, 0.1);
    assert_eq!(p.summary.bones.len(), 1);
    h.click("apply");
    assert!(h.panel.draft.is_none());
    assert_eq!(h.state.history.undo_depth(), 1);
    for _ in 0..5 {
        h.frame(vec![]);
    }
    assert_eq!(h.state.history.undo_depth(), 1);
    h.open();
    h.text("frames", "3");
    h.click("prepare");
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 2);
}
#[test]
fn t833_egui_cancel_escape_invalid_and_noop_preserve_redo() {
    let mut h = Harness::new();
    h.state.dispatch(Box::new(
        SetProjectFps::new(&h.state.doc, 60).unwrap().unwrap(),
    ));
    h.state.undo();
    let before = snapshot(&h.state.doc);
    let rev = h.state.revision;
    h.open();
    h.click("prepare");
    h.click("apply");
    assert!(h.panel.draft.is_some());
    h.click("cancel");
    assert!(h.panel.draft.is_none());
    h.open();
    h.text("frames", "NaN");
    h.click("prepare");
    assert!(h.panel.draft.as_ref().unwrap().error.is_some());
    h.click("apply");
    h.frame(vec![egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }]);
    assert!(h.panel.draft.is_none());
    assert!(snapshot(&h.state.doc) == before);
    assert_eq!(h.state.revision, rev);
    assert!(h.state.history.can_redo());
}
#[test]
fn t833_egui_disabled_without_explicit_selection_clip_or_animate() {
    for change in 0..3 {
        let mut h = Harness::new();
        match change {
            0 => h.state.session.selected_bones.clear(),
            1 => h.state.session.work_mode = WorkMode::Setup,
            _ => h.state.session.active_animation = None,
        };
        h.click("pose");
        h.click("menu");
        assert!(h.panel.draft.is_none());
    }
}
#[test]
fn t833_fps_30_60_interleaved_three_exact_history_cycles() {
    let mut s = state();
    let initial = snapshot(&s.doc);
    let mut d = draft("3");
    d.prepare(&s);
    assert_eq!(d.prepared.as_ref().unwrap().1.summary.offset, 0.1);
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
    let fps = snapshot(&s.doc);
    assert!(!d.apply(&mut s));
    d.prepare(&s);
    assert_eq!(d.prepared.as_ref().unwrap().1.summary.offset, 0.05);
    assert!(d.apply(&mut s));
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 2);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == fps);
        s.undo();
        assert!(snapshot(&s.doc) == initial);
        s.redo();
        assert!(snapshot(&s.doc) == fps);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
    assert!(!d.apply(&mut s));
}
#[test]
fn t833_all_context_leases_refuse_without_mutation_or_redo_loss() {
    for change in 0..13 {
        let mut s = state();
        s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
        s.undo();
        let mut d = draft("3");
        d.prepare(&s);
        let b = s.session.selected_bones[0];
        match change {
            0 => s.session.selected_bones.clear(),
            1 => s.session.work_mode = WorkMode::Setup,
            2 => s.session.active_animation = None,
            3 => {
                s.session.locked_bones.insert(b, true);
            }
            4 => s.doc.meta.fps = 60,
            5 => d.frames = "3.0".into(),
            6 => {
                s.revision += 1;
            }
            7 => {
                s.doc.skeleton.bones.remove(b);
                s.revision += 1;
            }
            8 => {
                let doc = s.doc.clone();
                s.replace_document(doc);
            }
            9 => {
                let id = s.session.active_animation.unwrap();
                s.doc.animations[id].timelines.clear();
            }
            11 => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(Animation::new("other", 2.)));
            }
            12 => {
                let mut bone = s.doc.skeleton.bones[b].clone();
                bone.name = "other bone".into();
                s.session.selected_bones = vec![s.doc.skeleton.add_bone(bone)];
            }
            _ => {
                let id = s.session.active_animation.unwrap();
                if let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] {
                    keys[0].time = 0.3;
                }
            }
        }
        let before = snapshot(&s.doc);
        let rev = s.revision;
        let depth = s.history.undo_depth();
        let redo = s.history.can_redo();
        assert!(!d.apply(&mut s), "change {change}");
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.revision, rev);
        assert_eq!(s.history.undo_depth(), depth);
        assert_eq!(s.history.can_redo(), redo);
    }
}
#[test]
fn t833_egui_stale_selection_and_fps_disable_apply() {
    for change in 0..6 {
        let mut h = Harness::new();
        h.open();
        h.text("frames", "3");
        h.click("prepare");
        match change {
            0 => h.state.session.selected_bones.clear(),
            1 => {
                h.state.dispatch(Box::new(
                    SetProjectFps::new(&h.state.doc, 60).unwrap().unwrap(),
                ));
            }
            2 => {
                let b = h.state.session.selected_bones[0];
                h.state.session.locked_bones.insert(b, true);
            }
            3 => {
                h.state.session.active_animation =
                    Some(h.state.doc.animations.insert(Animation::new("other", 2.)))
            }
            4 => {
                let doc = h.state.doc.clone();
                h.state.replace_document(doc);
            }
            _ => h.state.session.work_mode = WorkMode::Setup,
        }
        let before = snapshot(&h.state.doc);
        let depth = h.state.history.undo_depth();
        h.frame(vec![]);
        assert!(h.panel.draft.as_ref().unwrap().prepared.is_none());
        h.click("apply");
        assert!(snapshot(&h.state.doc) == before);
        assert_eq!(h.state.history.undo_depth(), depth);
    }
}
#[test]
fn t833_locked_targets_refuse_and_clamp_outside_require_consent() {
    let mut s = state();
    let b = s.session.selected_bones[0];
    s.session.locked_bones.insert(b, true);
    let mut d = draft("3");
    d.prepare(&s);
    assert!(d.prepared.is_none());
    assert!(!d.apply(&mut s));
    s.session.locked_bones.clear();
    for frames in ["-9", "60"] {
        let mut s = state();
        let mut d = draft(frames);
        d.prepare(&s);
        assert!(d.prepared.as_ref().unwrap().1.summary.needs_ack());
        assert!(!d.apply(&mut s));
        d.ack = true;
        assert!(d.apply(&mut s));
    }
}
#[test]
fn t833_egui_warning_checkbox_gates_apply() {
    let mut h = Harness::new();
    h.open();
    h.text("frames", "60");
    h.click("prepare");
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 0);
    h.click("ack");
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 1);
}
#[test]
fn t833_empty_tracks_and_no_animation_are_noops() {
    let mut s = state();
    let id = s.session.active_animation.unwrap();
    s.doc.animations[id].timelines.clear();
    let before = snapshot(&s.doc);
    let mut d = draft("3");
    d.prepare(&s);
    assert!(d.prepared.is_some());
    assert!(!d.apply(&mut s));
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 0);
}
