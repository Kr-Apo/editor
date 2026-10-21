use super::{
    t829_probe::{animated, logical, show},
    *,
};
use ankhimate_document::{
    commands::key_cmds::{DeleteKeys, KeyRef, TimelineAddr},
    snapshot::snapshot,
};
fn clone_state(s: &AppState) -> AppState {
    let mut d = AppState::default();
    d.replace_document(s.doc.clone());
    d.session.active_animation = s.session.active_animation;
    d.session.selected_bones = s.session.selected_bones.clone();
    d.session.selected_slots = s.session.selected_slots.clone();
    d.session.selection = s.session.selection.clone();
    d.session.playhead = s.session.playhead;
    d.session.active_skin = s.session.active_skin;
    d.revision = s.revision;
    d.set_work_mode(crate::session::WorkMode::Animate);
    d
}
fn pointer(p: egui::Pos2, pressed: bool, ctrl: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(p),
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers {
                ctrl,
                ..Default::default()
            },
        },
    ]
}
struct Pair {
    new: AppState,
    old: AppState,
    nctx: egui::Context,
    octx: egui::Context,
}
impl Pair {
    fn new(s: AppState, refs: Vec<KeyRef>) -> Self {
        let mut old = clone_state(&s);
        let mut new = s;
        let a = new.session.active_animation.unwrap();
        crate::key_selection::set(&mut new, a, refs.clone());
        crate::key_selection::set(&mut old, a, refs);
        Self {
            new,
            old,
            nctx: Default::default(),
            octx: Default::default(),
        }
    }
    fn frame(&mut self, mode: usize, events: Vec<egui::Event>, ctrl: bool) -> egui::FullOutput {
        let mods = egui::Modifiers {
            ctrl,
            ..Default::default()
        };
        let (n, h, _) = show(
            &self.nctx,
            &mut self.new,
            mode,
            false,
            events.clone(),
            mods,
            false,
        );
        let (o, k, _) = show(&self.octx, &mut self.old, mode, true, events, mods, false);
        assert_eq!(logical(&n), logical(&o));
        assert_eq!(h, k);
        let a = self.new.session.active_animation.unwrap();
        assert_eq!(
            crate::key_selection::read(&mut self.new, a),
            crate::key_selection::read(&mut self.old, a)
        );
        assert!(snapshot(&self.new.doc) == snapshot(&self.old.doc));
        assert_eq!(self.new.revision, self.old.revision);
        assert_eq!(self.new.history.undo_depth(), self.old.history.undo_depth());
        assert_eq!(
            self.new.session.selected_bones,
            self.old.session.selected_bones
        );
        assert_eq!(
            self.new.session.selected_slots,
            self.old.session.selected_slots
        );
        n
    }
    fn click(&mut self, mode: usize, p: egui::Pos2, ctrl: bool) {
        self.frame(mode, vec![egui::Event::PointerMoved(p)], ctrl);
        for pressed in [true, false] {
            self.frame(mode, pointer(p, pressed, ctrl), ctrl);
        }
        self.frame(mode, vec![], ctrl);
    }
}
#[test]
fn t829_click_ctrl_click_and_same_frame_colors_match_t828() {
    for mode in 0..3 {
        let (s, _, r) = animated(100, 1, false);
        let mut p = Pair::new(s, r[..50].to_vec());
        p.frame(mode, vec![], false);
        let out = p.frame(mode, vec![], false);
        let at = if mode < 2 {
            out.shapes
                .iter()
                .find_map(|s| {
                    if let egui::Shape::Circle(c) = &s.shape {
                        (c.radius == 4. && c.center.x > 200. && c.center.x < 250.)
                            .then_some(c.center)
                    } else {
                        None
                    }
                })
                .unwrap()
        } else {
            egui::pos2(30. + 520. * 40. / 99., 83.)
        };
        p.click(mode, at, true);
        p.click(mode, at, true);
        p.click(mode, at, false);
        let a = p.new.session.active_animation.unwrap();
        assert_eq!(crate::key_selection::read(&mut p.new, a).len(), 1);
        assert_eq!(p.new.history.undo_depth(), 0);
        let at = if mode < 2 {
            let out = p.frame(mode, vec![], false);
            out.shapes
                .iter()
                .find_map(|s| {
                    if let egui::Shape::Circle(c) = &s.shape {
                        (c.radius == 4. && c.center.x > 450. && c.center.x < 500.)
                            .then_some(c.center)
                    } else {
                        None
                    }
                })
                .unwrap()
        } else {
            egui::pos2(30. + 520. * 80. / 99., 83.)
        };
        p.frame(mode, vec![egui::Event::PointerMoved(at)], false);
        for pressed in [true, false] {
            p.frame(
                mode,
                vec![egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Secondary,
                    pressed,
                    modifiers: Default::default(),
                }],
                false,
            );
        }
        p.frame(mode, vec![], false);
    }
}
#[test]
fn t829_marquee_replace_union_and_clear_match_t828_across_views() {
    for mode in 0..3 {
        for ctrl in [false, true] {
            let (s, a, r) = animated(1000, 1, false);
            let mut p = Pair::new(s, r[..200].to_vec());
            p.frame(mode, vec![], false);
            p.frame(mode, vec![], false);
            let from = egui::pos2(16., 51.);
            let to = egui::pos2(614., 549.);
            p.frame(mode, vec![egui::Event::PointerMoved(from)], ctrl);
            p.frame(mode, pointer(from, true, ctrl), ctrl);
            // The existing sheet records its box start at drag threshold,
            // unlike the graph's press-origin lease. Exercise both conventions.
            p.frame(
                mode,
                vec![egui::Event::PointerMoved(from + egui::vec2(6., 6.))],
                ctrl,
            );
            p.frame(mode, vec![egui::Event::PointerMoved(to)], ctrl);
            p.frame(mode, pointer(to, false, ctrl), ctrl);
            p.frame(mode, vec![], false);
            assert!(
                !crate::key_selection::read(&mut p.new, a).is_empty(),
                "mode={mode} ctrl={ctrl}"
            );
            p.frame(2, vec![], false);
            p.frame(0, vec![], false);
            p.frame(1, vec![], false);
            crate::key_selection::set(&mut p.new, a, vec![]);
            crate::key_selection::set(&mut p.old, a, vec![]);
            p.frame(mode, vec![], false);
            assert!(crate::key_selection::read(&mut p.new, a).is_empty());
        }
    }
}
#[test]
fn t829_revision_clip_document_and_animation_replacement_drop_stale_views() {
    for change in 0..5 {
        let (s, a, r) = animated(1000, 1, false);
        let mut p = Pair::new(s, r);
        p.frame(0, vec![], false);
        for s in [&mut p.new, &mut p.old] {
            match change {
                0 => s.revision += 1,
                1 => {
                    let b = s.doc.animations.insert(s.doc.animations[a].clone());
                    s.session.active_animation = Some(b);
                }
                2 => {
                    let d = s.doc.clone();
                    s.replace_document(d);
                    s.session.active_animation = Some(a);
                }
                3 => s.doc.animations[a].timelines.clear(),
                _ => {
                    if let ankhimate_core::animation::Timeline::BoneRotate { keys, .. } =
                        &mut s.doc.animations[a].timelines[0]
                    {
                        keys[5].value += 1.;
                    }
                }
            }
        }
        p.frame(1, vec![], false);
        p.frame(2, vec![], false);
        let active = p.new.session.active_animation.unwrap();
        assert!(crate::key_selection::read(&mut p.new, active).is_empty());
    }
}
#[test]
fn t829_undo_redo_delete_and_sorted_move_preserve_native_policies() {
    let (s, a, r) = animated(100, 1, false);
    let mut p = Pair::new(s, r.clone());
    for s in [&mut p.new, &mut p.old] {
        s.dispatch(Box::new(DeleteKeys::new(a, vec![r[1].clone()])));
        s.undo();
        crate::key_selection::set(s, a, r.clone());
        assert!(s.history.can_redo());
    }
    let before = snapshot(&p.new.doc);
    for mode in [2, 0, 1, 2] {
        p.frame(mode, vec![], false);
        assert!(p.new.history.can_redo());
        assert!(snapshot(&p.new.doc) == before);
    }
    p.new.redo();
    p.old.redo();
    p.frame(0, vec![], false);
    p.new.undo();
    p.old.undo();
    for s in [&mut p.new, &mut p.old] {
        let moved = crate::key_selection::move_keys(s, a, &[r[3].clone()], 0.01).unwrap();
        crate::key_selection::set(s, a, moved);
    }
    p.frame(2, vec![], false);
    p.frame(0, vec![], false);
    for _ in 0..3 {
        p.new.undo();
        p.old.undo();
        p.frame(0, vec![], false);
        p.new.redo();
        p.old.redo();
        p.frame(2, vec![], false);
    }
}
#[test]
fn t829_interrupted_marquee_and_raw_normalize_switch_match_t828() {
    for interruption in 0..5 {
        let (s, _, r) = animated(100, 1, false);
        let mut p = Pair::new(s, r[..40].to_vec());
        p.frame(0, vec![], false);
        p.frame(0, vec![], false);
        let from = egui::pos2(16., 51.);
        let to = egui::pos2(400., 450.);
        p.frame(0, vec![egui::Event::PointerMoved(from)], false);
        p.frame(0, pointer(from, true, false), false);
        p.frame(0, vec![egui::Event::PointerMoved(to)], false);
        match interruption {
            0 => {
                p.frame(1, vec![], false);
            }
            1 => {
                p.frame(0, vec![egui::Event::PointerGone], false);
            }
            2 => {
                p.new.revision += 1;
                p.old.revision += 1;
            }
            3 => {
                p.frame(
                    0,
                    vec![egui::Event::Key {
                        key: egui::Key::Escape,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Default::default(),
                    }],
                    false,
                );
            }
            _ => {
                p.frame(2, vec![], false);
            }
        }
        p.frame(0, pointer(to, false, false), false);
        p.frame(0, vec![], false);
    }
}
#[test]
fn t829_ffd_single_and_mixed_refusal_preserve_visual_and_edit_guards() {
    let (mut s, a, slot, skin) = crate::ffd_keys::tests::fixture(false);
    s.session.active_skin = skin;
    let key = KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index: 1,
    };
    let mut p = Pair::new(s, vec![key.clone()]);
    p.frame(2, vec![], false);
    p.frame(0, vec![], false);
    for s in [&mut p.new, &mut p.old] {
        crate::key_selection::set(s, a, vec![key.clone(), key.clone()]);
        assert_eq!(crate::key_selection::read(s, a), vec![key.clone()]);
        let bone = s.doc.skeleton.bones.keys().next().unwrap();
        let scalar = KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: ankhimate_document::commands::key_cmds::BoneProperty::Rotate,
                axis: None,
            },
            index: 0,
        };
        crate::key_selection::set(s, a, vec![scalar, key.clone()]);
        assert_eq!(crate::key_selection::read(s, a), vec![key.clone()]);
    }
    p.frame(2, vec![], false);
    assert_eq!(p.new.history.undo_depth(), 0);
}
#[test]
fn t829_setup_skin_and_visible_track_changes_build_fresh_views() {
    let (s, a, r) = animated(100, 2, true);
    let mut p = Pair::new(s, r[..100].to_vec());
    p.frame(2, vec![], false);
    for s in [&mut p.new, &mut p.old] {
        s.set_work_mode(crate::session::WorkMode::Setup);
        s.session.selected_bones.clear();
    }
    p.frame(0, vec![], false);
    for s in [&mut p.new, &mut p.old] {
        s.set_work_mode(crate::session::WorkMode::Animate);
        let skin = ankhimate_core::skin::Skin::new("alternate");
        s.session.active_skin = s.doc.skeleton.skins.insert(skin);
    }
    p.frame(1, vec![], false);
    p.frame(2, vec![], false);
    let model = super::model::TimelineModel::build(&p.new, a);
    for group in &model.groups {
        for ctx in [&p.nctx, &p.octx] {
            ctx.data_mut(|d| d.insert_temp(egui::Id::new(("tl_fold", group.fold_id)), true));
        }
    }
    p.frame(0, vec![], false);
    p.frame(1, vec![], false);
    p.frame(2, vec![], false);
    assert_eq!(
        crate::key_selection::read(&mut p.new, a),
        crate::key_selection::read(&mut p.old, a)
    );
}
