use super::*;
#[allow(dead_code, reason = "Shared synthetic native author fixture")]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/path_document_fixture.rs"
    ));
}
use ankhimate_document::{commands::document_cmds::SetProjectFps, snapshot::snapshot};
pub(crate) fn state() -> (AppState, AnimationId, ConstraintId) {
    let (doc, a, c) = rig::path_document();
    let mut s = AppState::default();
    s.replace_document(doc);
    s.session.active_animation = Some(a);
    s.session.work_mode = WorkMode::Animate;
    s.session.select_constraint(c);
    s.refresh_pose();
    (s, a, c)
}
#[test]
fn t831_editor_stale_revision_clip_document_playhead_selection_mode_and_locks_refuse() {
    for change in 0..9 {
        let (mut s, a, c) = state();
        let stamp = Stamp::new(&s);
        match change {
            0 => {
                s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
                s.undo();
            }
            1 => s.session.active_animation = None,
            2 => {
                let doc = s.doc.clone();
                s.replace_document(doc);
            }
            3 => s.session.playhead = 0.25,
            4 => s.session.selection = None,
            5 => s.session.work_mode = WorkMode::Setup,
            6 => {
                let b = s.doc.skeleton.bones.keys().next().unwrap();
                s.session.locked_bones.insert(b, true);
            }
            7 => {
                s.doc.skeleton.remove_constraint(c);
            }
            _ => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(s.doc.animations[a].clone()));
            }
        }
        let before = snapshot(&s.doc);
        let rev = s.revision;
        let depth = s.history.undo_depth();
        assert!(
            apply(
                &mut s,
                &stamp,
                c,
                Action::Put {
                    time: 0.25,
                    value: 0.1,
                    interp: Interp::Linear
                }
            )
            .is_err(),
            "case {change}"
        );
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.revision, rev);
        assert_eq!(s.history.undo_depth(), depth);
    }
}
#[test]
fn t831_editor_auto_key_off_preview_k_and_context_clear() {
    let (mut s, a, c) = state();
    s.session.auto_key = false;
    s.session.playhead = 0.25;
    s.refresh_pose();
    let before = snapshot(&s.doc);
    let stamp = Stamp::new(&s);
    preview(&mut s, &stamp, c, 0.8).unwrap();
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.pose.path_position.get(c), Some(&0.8));
    assert!(s.session.has_pending_pose());
    s.key_pending_pose();
    assert!(!s.session.has_pending_pose());
    assert!(
        Capture::read(&s.doc, a, c)
            .unwrap()
            .keys
            .iter()
            .any(|k| k.time == 0.25 && k.value == 0.8)
    );
    assert_eq!(s.history.undo_depth(), 1);
    s.undo();
    assert!(snapshot(&s.doc) == before);
    let stamp = Stamp::new(&s);
    preview(&mut s, &stamp, c, 0.7).unwrap();
    s.session.playhead = 0.75;
    s.refresh_pose();
    assert!(s.session.path_preview.is_none());
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t831_editor_bounded_dopesheet_row_shared_selection_and_no_graph_surrogate() {
    let (mut s, a, c) = state();
    let model = crate::ui::timeline::model::TimelineModel::build(&s, a);
    let row = model
        .groups
        .iter()
        .flat_map(|g| &g.rows)
        .find(|r| r.label == "Path Position")
        .unwrap();
    assert_eq!(row.addr, TimelineAddr::PathPosition { constraint: c });
    assert!(row.channels.is_empty());
    assert!(!row.read_only);
    let key = KeyRef {
        addr: row.addr.clone(),
        index: 1,
    };
    crate::key_selection::set(&mut s, a, vec![key.clone()]);
    assert_eq!(crate::key_selection::read(&mut s, a), vec![key.clone()]);
    assert!(crate::key_selection::move_keys(&mut s, a, &[key], 0.1).is_none());
    assert!(!s.history.can_undo());
}
struct H {
    ctx: egui::Context,
    s: AppState,
    c: ConstraintId,
    key: Option<KeyRef>,
}
impl H {
    fn new() -> Self {
        let (s, _, c) = state();
        let mut h = Self {
            ctx: Default::default(),
            s,
            c,
            key: None,
        };
        h.frame(vec![]);
        h.frame(vec![]);
        h
    }
    fn frame(&mut self, events: Vec<egui::Event>) {
        let _ = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000., 1100.),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                if let Some(k) = &self.key {
                    let a = self.s.session.active_animation.unwrap();
                    key_menu(ui, &mut self.s, a, k);
                } else {
                    crate::ui::inspector::ui(ui, &mut self.s);
                }
                let _ = ui.button("Elsewhere");
            },
        );
    }
    fn click(&mut self, name: &str) {
        self.frame(vec![]);
        let p = self
            .ctx
            .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t831", name))))
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
    fn text(&mut self, name: &str, text: &str) {
        self.click(name);
        let m = egui::Modifiers {
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
                modifiers: m,
            },
            egui::Event::Text(text.into()),
        ]);
        self.frame(vec![]);
    }
}
#[test]
fn t831_egui_inspector_actual_key_action_and_auto_key_completion() {
    let mut h = H::new();
    let a = h.s.session.active_animation.unwrap();
    let before = snapshot(&h.s.doc);
    h.text("value", "0.7");
    assert!(snapshot(&h.s.doc) == before);
    h.click("key");
    assert_eq!(h.s.history.undo_depth(), 1);
    assert_eq!(Capture::read(&h.s.doc, a, h.c).unwrap().keys[0].value, 0.7);
    let after = snapshot(&h.s.doc);
    for _ in 0..3 {
        h.s.undo();
        assert!(snapshot(&h.s.doc) == before);
        h.s.redo();
        assert!(snapshot(&h.s.doc) == after);
    }
    h.s.session.playhead = 0.25;
    h.s.refresh_pose();
    h.frame(vec![]);
    h.text("value", "0.9");
    h.frame(vec![egui::Event::Key {
        key: egui::Key::Tab,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }]);
    h.frame(vec![]);
    assert!(
        Capture::read(&h.s.doc, a, h.c)
            .unwrap()
            .keys
            .iter()
            .any(|k| k.time == 0.25 && k.value == 0.9)
    );
}
#[test]
fn t831_egui_key_menu_changes_time_value_and_delete() {
    let mut h = H::new();
    let a = h.s.session.active_animation.unwrap();
    h.key = Some(KeyRef {
        addr: TimelineAddr::PathPosition { constraint: h.c },
        index: 1,
    });
    h.frame(vec![]);
    h.text("value", "0.4");
    h.text("time", "0.6");
    h.click("key");
    let keys = Capture::read(&h.s.doc, a, h.c).unwrap().keys;
    assert_eq!((keys[1].time, keys[1].value), (0.6, 0.4));
    h.click("delete");
    assert_eq!(Capture::read(&h.s.doc, a, h.c).unwrap().keys.len(), 2);
    assert_eq!(h.s.history.undo_depth(), 2);
}
#[test]
fn t831_egui_invalid_input_and_stale_draft_never_mutate() {
    let mut h = H::new();
    let before = snapshot(&h.s.doc);
    h.text("value", "NaN");
    h.click("key");
    assert!(snapshot(&h.s.doc) == before);
    assert_eq!(h.s.history.undo_depth(), 0);
    h.text("value", "0.4");
    h.s.session.playhead = 0.75;
    h.s.refresh_pose();
    h.frame(vec![]);
    h.click("key");
    assert!(snapshot(&h.s.doc) == before);
    assert_eq!(h.s.history.undo_depth(), 0);
}
#[test]
fn t831_editor_retime_new_track_rejects_locked_target() {
    let (mut s, _, c) = state();
    let p = if let ankhimate_core::constraints::Constraint::Path(p) = &s.doc.skeleton.constraints[c]
    {
        p
    } else {
        panic!()
    };
    s.session.locked_bones.insert(p.bones[0], true);
    assert!(editable(&s, c).is_err());
}
