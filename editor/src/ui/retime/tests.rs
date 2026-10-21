use super::*;
use crate::session::WorkMode;
use ankhimate_core::{
    animation::{Animation, Key},
    skeleton::Bone,
};
use ankhimate_document::{
    commands::{
        document_cmds::SetProjectFps,
        key_cmds::{AddKey, BoneProperty, KeyValue, TimelineAddr},
    },
    snapshot::snapshot,
};
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
    s.refresh_pose();
    s
}
fn draft(f: &str, o: &str) -> Draft {
    Draft {
        factor: f.into(),
        frames: o.into(),
        ..Default::default()
    }
}
#[test]
fn t830_prepare_cancel_noop_and_errors_preserve_history() {
    let mut s = state();
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
    s.undo();
    let before = snapshot(&s.doc);
    let rev = s.revision;
    for (f, o) in [
        ("1", "0"),
        ("0", "0"),
        ("NaN", "0"),
        ("2", "inf"),
        ("bad", "1"),
        ("1", "-60"),
    ] {
        let mut d = draft(f, o);
        d.prepare(&s);
        assert!(!d.apply(&mut s));
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.revision, rev);
        assert!(s.history.can_redo());
    }
    let mut d = draft("2", "0");
    d.prepare(&s);
    drop(d);
    assert!(snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
}
#[test]
fn t830_fps_interleaving_rejects_stale_apply_then_three_cycles() {
    let mut s = state();
    let initial = snapshot(&s.doc);
    let mut d = draft("1", "3");
    d.prepare(&s);
    assert_eq!(d.prepared.as_ref().unwrap().1.summary.offset, 0.1);
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
    let fps = snapshot(&s.doc);
    assert!(!d.apply(&mut s));
    assert!(snapshot(&s.doc) == fps);
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
fn t830_all_context_changes_invalidate_lease() {
    for change in 0..9 {
        let mut s = state();
        let mut d = draft("2", "0");
        d.prepare(&s);
        match change {
            0 => s.session.work_mode = WorkMode::Setup,
            1 => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(Animation::new("other", 1.)))
            }
            2 => s
                .session
                .selected_bones
                .push(s.doc.skeleton.bones.keys().next().unwrap()),
            3 => {
                let b = s.doc.skeleton.bones.keys().next().unwrap();
                s.session.locked_bones.insert(b, true);
            }
            4 => {
                s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
                s.undo();
            }
            5 => {
                let doc = s.doc.clone();
                s.replace_document(doc);
            }
            6 => d.factor = "0.5".into(),
            7 => d.frames = "3".into(),
            _ => s.doc.meta.fps = 60,
        }
        let before = snapshot(&s.doc);
        let rev = s.revision;
        let depth = s.history.undo_depth();
        assert!(!d.apply(&mut s), "change {change}");
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.revision, rev);
        assert_eq!(s.history.undo_depth(), depth);
    }
}
#[test]
fn t830_scope_independent_of_none_one_multiple_bones_and_other_clip() {
    for count in 0..3 {
        let mut s = state();
        let id = s.session.active_animation.unwrap();
        let original = s.doc.animations[id].clone();
        let other = s.doc.animations.insert(original.clone());
        let b = s.doc.skeleton.bones.keys().next().unwrap();
        s.session.selected_bones = vec![b; count];
        let mut d = draft("2", "0");
        d.prepare(&s);
        assert!(d.apply(&mut s));
        assert_eq!(s.doc.animations[other], original);
        assert_eq!(s.doc.animations[id].duration, 4.);
    }
}
#[test]
fn t830_locked_bone_setup_missing_clip_invalid_fps_refused() {
    let mut s = state();
    let b = s.doc.skeleton.bones.keys().next().unwrap();
    s.session.locked_bones.insert(b, true);
    assert!(prepare(&s, "2", "0").is_err());
    s.session.locked_bones.insert(b, false);
    s.session.work_mode = WorkMode::Setup;
    assert!(prepare(&s, "2", "0").is_err());
    s.session.work_mode = WorkMode::Animate;
    for fps in [0, 241, u32::MAX] {
        s.doc.meta.fps = fps;
        assert!(prepare(&s, "2", "0").is_err());
    }
    s.doc.meta.fps = 30;
    s.session.active_animation = None;
    assert!(prepare(&s, "2", "0").is_err());
}
#[test]
fn t830_warning_ack_clamp_metadata_and_outside_duration() {
    let mut s = state();
    let id = s.session.active_animation.unwrap();
    s.doc.animations[id]
        .markers
        .push(ankhimate_core::animation::Marker::new(1., "hit"));
    let before = s.doc.animations[id].markers.clone();
    let mut d = draft("2", "3");
    d.prepare(&s);
    assert!(needs_ack(&d.prepared.as_ref().unwrap().1));
    assert!(!d.apply(&mut s));
    d.acknowledged = true;
    assert!(d.apply(&mut s));
    assert_eq!(s.doc.animations[id].markers, before);
    let mut s = state();
    let mut d = draft("1", "-9");
    d.prepare(&s);
    assert_eq!(d.prepared.as_ref().unwrap().1.summary.clamped, 1);
    assert!(!d.apply(&mut s));
    d.acknowledged = true;
    assert!(d.apply(&mut s));
}
#[test]
fn t830_key_edit_interleaving_preserves_complete_snapshots() {
    let mut s = state();
    let id = s.session.active_animation.unwrap();
    let b = s.doc.skeleton.bones.keys().next().unwrap();
    let original = snapshot(&s.doc);
    let mut d = draft("1.25", "0");
    d.prepare(&s);
    assert!(d.apply(&mut s));
    let retimed = snapshot(&s.doc);
    s.dispatch(Box::new(AddKey::new(
        id,
        TimelineAddr::Bone {
            bone: b,
            property: BoneProperty::Rotate,
            axis: None,
        },
        0.8,
        KeyValue::Scalar(21.),
        Default::default(),
    )));
    let keyed = snapshot(&s.doc);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == retimed);
        s.undo();
        assert!(snapshot(&s.doc) == original);
        s.redo();
        assert!(snapshot(&s.doc) == retimed);
        s.redo();
        assert!(snapshot(&s.doc) == keyed);
    }
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
                probe(&ctx, "pose", &r.response);
            });
            self.panel.show(&ctx, &mut self.state, &Theme::default());
        });
    }
    fn click(&mut self, name: &str) {
        self.frame(vec![]);
        let p = self
            .ctx
            .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(("t830", name))))
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
#[test]
fn t830_egui_real_pose_action_fields_prepare_apply_twice() {
    let mut h = Harness::new();
    let before = snapshot(&h.state.doc);
    h.open();
    assert_eq!(h.panel.draft.as_ref().unwrap().factor, "1");
    assert_eq!(h.panel.draft.as_ref().unwrap().frames, "0");
    h.text("factor", "1.25");
    h.text("frames", "0");
    assert!(snapshot(&h.state.doc) == before);
    assert_eq!(h.state.history.undo_depth(), 0);
    h.click("prepare");
    assert_eq!(
        h.panel
            .draft
            .as_ref()
            .unwrap()
            .prepared
            .as_ref()
            .unwrap()
            .1
            .summary
            .duration_after,
        2.5
    );
    h.click("apply");
    assert!(h.panel.draft.is_none());
    assert_eq!(h.state.history.undo_depth(), 1);
    h.open();
    h.text("factor", "2");
    h.click("prepare");
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 2);
    let id = h.state.session.active_animation.unwrap();
    assert_eq!(h.state.doc.animations[id].duration, 5.);
}
#[test]
fn t830_egui_cancel_escape_invalid_and_noop_never_mutate() {
    let mut h = Harness::new();
    let before = snapshot(&h.state.doc);
    h.open();
    h.click("prepare");
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 0);
    h.click("cancel");
    assert!(h.panel.draft.is_none());
    h.open();
    h.text("factor", "NaN");
    h.click("prepare");
    assert!(h.panel.draft.as_ref().unwrap().error.is_some());
    h.click("apply");
    assert!(snapshot(&h.state.doc) == before);
    h.frame(vec![egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }]);
    assert!(h.panel.draft.is_none());
    assert!(snapshot(&h.state.doc) == before);
}
#[test]
fn t830_egui_stale_fps_preview_disables_apply_until_prepare() {
    let mut h = Harness::new();
    h.open();
    h.text("frames", "3");
    h.click("prepare");
    h.state.dispatch(Box::new(
        SetProjectFps::new(&h.state.doc, 60).unwrap().unwrap(),
    ));
    let before = snapshot(&h.state.doc);
    h.frame(vec![]);
    assert!(h.panel.draft.as_ref().unwrap().prepared.is_none());
    h.click("apply");
    assert!(snapshot(&h.state.doc) == before);
    h.click("prepare");
    assert_eq!(
        h.panel
            .draft
            .as_ref()
            .unwrap()
            .prepared
            .as_ref()
            .unwrap()
            .1
            .summary
            .offset,
        0.05
    );
    h.click("apply");
    assert_eq!(h.state.history.undo_depth(), 2);
}
#[test]
fn t830_egui_setup_menu_is_disabled() {
    let mut h = Harness::new();
    h.state.session.work_mode = WorkMode::Setup;
    h.click("pose");
    h.click("menu");
    assert!(h.panel.draft.is_none());
}
#[test]
fn t830_preparation_cpu_observation_no_per_frame_capture() {
    for n in [100, 1000, 10000] {
        let mut s = state();
        let id = s.session.active_animation.unwrap();
        if let Timeline::BoneRotate { keys, .. } = &mut s.doc.animations[id].timelines[0] {
            *keys = (0..n)
                .map(|i| Key::linear(i as f32 * 0.01, i as f32))
                .collect();
        }
        s.doc.animations[id].duration = n as f32 * 0.01;
        let mut d = draft("1.25", "0");
        let t = std::time::Instant::now();
        d.prepare(&s);
        let elapsed = t.elapsed();
        assert_eq!(d.prepared.as_ref().unwrap().1.summary.keys, n);
        let ptr = &d.prepared.as_ref().unwrap().1 as *const _;
        for _ in 0..100 {
            d.invalidate(&s);
            assert_eq!(&d.prepared.as_ref().unwrap().1 as *const _, ptr);
        }
        println!(
            "T830_PREPARE keys={n} micros={} validation_frames=100",
            elapsed.as_micros()
        );
    }
}

#[test]
fn t830_egui_document_clip_selection_mode_and_history_invalidate() {
    for change in 0..7 {
        let mut h = Harness::new();
        h.open();
        h.text("factor", "2");
        h.click("prepare");
        assert!(h.panel.draft.as_ref().unwrap().prepared.is_some());
        match change {
            0 => h
                .state
                .session
                .selected_bones
                .push(h.state.doc.skeleton.bones.keys().next().unwrap()),
            1 => {
                h.state.session.active_animation =
                    Some(h.state.doc.animations.insert(Animation::new("other", 1.)))
            }
            2 => {
                let doc = h.state.doc.clone();
                h.state.replace_document(doc);
            }
            3 => h.state.session.work_mode = WorkMode::Setup,
            4 => {
                h.state.dispatch(Box::new(
                    SetProjectFps::new(&h.state.doc, 60).unwrap().unwrap(),
                ));
                h.state.undo();
            }
            5 => {
                h.state.dispatch(Box::new(
                    SetProjectFps::new(&h.state.doc, 60).unwrap().unwrap(),
                ));
                h.state.undo();
                h.state.redo();
            }
            _ => {
                let b = h.state.doc.skeleton.bones.keys().next().unwrap();
                let slot = h
                    .state
                    .doc
                    .skeleton
                    .add_slot(ankhimate_core::slot::Slot::new("slot".into(), b));
                h.state.session.selected_slots.push(slot);
            }
        }
        let before = snapshot(&h.state.doc);
        let depth = h.state.history.undo_depth();
        h.frame(vec![]);
        assert!(
            h.panel.draft.as_ref().unwrap().prepared.is_none(),
            "change {change}"
        );
        h.click("apply");
        assert!(snapshot(&h.state.doc) == before);
        assert_eq!(h.state.history.undo_depth(), depth);
    }
}
#[test]
fn t830_slot_and_ffd_locks_refuse_whole_operation() {
    use ankhimate_core::slot::Slot;
    let mut s = state();
    let b = s.doc.skeleton.bones.keys().next().unwrap();
    let slot = s.doc.skeleton.add_slot(Slot::new("slot".into(), b));
    let id = s.session.active_animation.unwrap();
    for track in [
        Timeline::SlotVisible {
            slot,
            keys: vec![Key::linear(0., true)],
        },
        Timeline::Deform {
            slot,
            attachment: "mesh".into(),
            keys: vec![Key::linear(0., vec![])],
        },
    ] {
        s.doc.animations[id].timelines = vec![track];
        s.session.locked_bones.insert(b, true);
        assert!(prepare(&s, "2", "0").is_err());
        s.session.locked_bones.insert(b, false);
        assert!(prepare(&s, "2", "0").is_ok());
    }
}

#[test]
fn t831_retime_path_respects_disabled_and_slot_bone_locks() {
    let (mut s, _, c) = crate::path_keys::tests::state();
    assert!(prepare(&s, "2", "0").is_ok());
    let slot =
        if let ankhimate_core::constraints::Constraint::Path(p) = &s.doc.skeleton.constraints[c] {
            p.slot
        } else {
            panic!()
        };
    s.session
        .locked_bones
        .insert(s.doc.skeleton.slots[slot].bone, true);
    assert!(prepare(&s, "2", "0").is_err());
    s.session.locked_bones.clear();
    if let ankhimate_core::constraints::Constraint::Path(p) = &mut s.doc.skeleton.constraints[c] {
        p.mix_rotate = 0.;
        p.mix_translate = 0.;
    }
    assert!(prepare(&s, "2", "0").is_err());
}
