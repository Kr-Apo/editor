use super::*;
use ankhimate_core::{
    animation::{Animation, Axis, Key},
    math::Transform,
    skeleton::Bone,
};
use ankhimate_document::{
    Document,
    commands::key_cmds::{BoneProperty, DeleteKeys},
    snapshot::snapshot,
};
fn curve() -> Interp {
    Interp::Bezier {
        out_handle: glam::vec2(0.25, 0.25),
        in_handle: glam::vec2(0.75, 0.75),
    }
}
pub(crate) fn fixture() -> (AppState, AnimationId, KeyRef) {
    let mut s = AppState::default();
    let bone = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 1.,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("curve", 2.);
    a.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::X,
        keys: (0..3)
            .map(|i| Key {
                time: i as f32,
                value: i as f32,
                interp: curve(),
            })
            .collect(),
    });
    let anim = s.doc.animations.insert(a);
    s.session.active_animation = Some(anim);
    s.session.select_bone(Some(bone));
    s.set_work_mode(crate::session::WorkMode::Animate);
    let r = KeyRef {
        addr: TimelineAddr::Bone {
            bone,
            property: BoneProperty::Translate,
            axis: Some(Axis::X),
        },
        index: 1,
    };
    key_selection::set(&mut s, anim, vec![r.clone()]);
    (s, anim, r)
}
#[test]
fn t820_mode_choice_changes_no_data_history_or_selection() {
    let (mut s, a, r) = fixture();
    let before = snapshot(&s.doc);
    for m in [Mode::Aligned, Mode::Mirrored, Mode::Broken] {
        choose(&mut s, m).unwrap();
        assert!(snapshot(&s.doc) == before);
        assert!(!s.history.can_undo());
        assert_eq!(key_selection::read(&mut s, a), vec![r.clone()]);
    }
}
#[test]
fn t820_numeric_apply_isolated_three_exact_undo_redo() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    let before = snapshot(&s.doc);
    let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
    n.fields[0] = "0.8".into();
    n.fields[1] = "0.5".into();
    n.apply(&mut s).unwrap();
    let after = snapshot(&s.doc);
    assert!(before != after);
    assert_eq!(s.history.undo_depth(), 1);
    assert_eq!(mode(&mut s), Mode::Mirrored);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        assert_eq!(mode(&mut s), Mode::Broken);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
    key_selection::set(&mut s, a, vec![r]);
}
#[test]
fn t820_numeric_multiple_applies_never_merge() {
    let (mut s, _, _) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    for y in ["0.5", "0.6"] {
        let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
        n.fields[0] = "0.8".into();
        n.fields[1] = y.into();
        n.apply(&mut s).unwrap();
    }
    assert_eq!(s.history.undo_depth(), 2);
}
#[test]
fn t820_numeric_cancel_and_invalid_preserve_redo() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
    n.fields[1] = "0.5".into();
    n.apply(&mut s).unwrap();
    s.undo();
    key_selection::set(&mut s, a, vec![r]);
    choose(&mut s, Mode::Mirrored).unwrap();
    let before = snapshot(&s.doc);
    let mut draft = Numeric::new(&mut s, Side::Incoming).unwrap();
    draft.fields[0] = "NaN".into();
    assert!(draft.apply(&mut s).is_err());
    assert!(snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
    drop(draft);
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t820_setup_lock_absent_clip_and_readonly_refuse() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Aligned).unwrap();
    let n = Numeric::new(&mut s, Side::Incoming).unwrap();
    s.set_work_mode(crate::session::WorkMode::Setup);
    assert!(n.apply(&mut s).is_err());
    s.set_work_mode(crate::session::WorkMode::Animate);
    key_selection::set(&mut s, a, vec![r.clone()]);
    if let TimelineAddr::Bone { bone, .. } = r.addr {
        s.session.locked_bones.insert(bone, true);
    }
    assert!(choose(&mut s, Mode::Aligned).is_err());
    s.doc.animations.remove(a);
    assert!(choose(&mut s, Mode::Mirrored).is_err());
    assert!(
        editable(
            &s,
            &TimelineAddr::SlotVisible {
                slot: Default::default()
            }
        )
        .is_err()
    );
}
#[test]
fn t820_external_revision_clip_document_and_reorder_invalidate() {
    for case in 0..5 {
        let (mut s, a, r) = fixture();
        choose(&mut s, Mode::Aligned).unwrap();
        let n = Numeric::new(&mut s, Side::Incoming).unwrap();
        match case {
            0 => s.revision += 1,
            1 => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(Animation::new("other", 2.)))
            }
            2 => {
                s.replace_document(s.doc.clone());
            }
            3 => {
                if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[a].timelines[0]
                {
                    keys.swap(0, 1);
                }
            }
            _ => {
                s.dispatch(Box::new(DeleteKeys::new(a, vec![r])));
            }
        }
        assert!(n.apply(&mut s).is_err());
        assert_eq!(mode(&mut s), Mode::Broken);
    }
}
#[test]
fn t820_multiselection_keeps_broken_numeric_workflow() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Aligned).unwrap();
    let mut next = r.clone();
    next.index = 2;
    key_selection::set(&mut s, a, vec![r, next]);
    assert_eq!(mode(&mut s), Mode::Broken);
    assert!(choose(&mut s, Mode::Mirrored).is_err());
    let mut d = crate::ui::inspector::tangents::Draft::new(&mut s, a).unwrap();
    d.fields[1] = "-2".into();
    d.apply(&mut s).unwrap();
    assert_eq!(s.history.undo_depth(), 1);
}
#[test]
fn t820_aligned_drag_uses_stable_initial_opposite_length() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Aligned).unwrap();
    let initial = selected(&mut s).unwrap();
    let mut g = Gesture::begin(&mut s, a, r, Side::Incoming, 41).unwrap();
    for y in [0.6, 0.4, 0.8, 0.6] {
        g.update(&mut s, glam::vec2(0.8, y)).unwrap();
    }
    assert_eq!(s.history.undo_depth(), 1);
    let final_j = selected(&mut s).unwrap();
    assert!(
        (initial
            .vector(Side::Outgoing, initial.handle(Side::Outgoing))
            .length()
            - final_j
                .vector(Side::Outgoing, final_j.handle(Side::Outgoing))
                .length())
        .abs()
            < 1e-6
    );
    let expected = initial
        .solve(Mode::Aligned, Side::Incoming, glam::vec2(0.8, 0.6))
        .unwrap();
    assert_eq!(final_j.keys[1].interp, expected[0]);
    assert_eq!(final_j.keys[2].interp, expected[1]);
}
#[test]
fn t820_two_drags_and_numeric_apply_are_separate_undo_entries() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    for (token, y) in [(51, 0.4), (52, 0.6)] {
        let mut g = Gesture::begin(&mut s, a, r.clone(), Side::Incoming, token).unwrap();
        g.update(&mut s, glam::vec2(0.8, y)).unwrap();
    }
    let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
    n.fields[1] = "0.8".into();
    n.apply(&mut s).unwrap();
    assert_eq!(s.history.undo_depth(), 3);
}
#[test]
fn t820_undo_mid_gesture_stops_without_discarding_redo() {
    let (mut s, a, r) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    let mut g = Gesture::begin(&mut s, a, r, Side::Incoming, 61).unwrap();
    g.update(&mut s, glam::vec2(0.8, 0.5)).unwrap();
    s.undo();
    let before = snapshot(&s.doc);
    assert!(g.update(&mut s, glam::vec2(0.8, 0.6)).is_err());
    assert!(snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
}
#[test]
fn t820_same_displacement_one_or_many_drag_events_matches() {
    let mut results = Vec::new();
    for updates in [vec![0.5], vec![0.7, 0.6, 0.5]] {
        let (mut s, a, r) = fixture();
        choose(&mut s, Mode::Aligned).unwrap();
        let mut g = Gesture::begin(&mut s, a, r, Side::Incoming, 1).unwrap();
        for y in updates {
            g.update(&mut s, glam::vec2(0.8, y)).unwrap();
        }
        results.push(ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap());
    }
    assert_eq!(results[0], results[1]);
}
#[test]
fn t820_changing_driver_is_explicit_and_opposite_fields_are_not_instructions() {
    let (mut s, _, _) = fixture();
    choose(&mut s, Mode::Mirrored).unwrap();
    let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
    n.fields[1] = "0.5".into();
    n.fields[2] = "NaN".into();
    assert!(n.pair().is_ok());
    let fresh = Numeric::new(&mut s, Side::Outgoing).unwrap();
    assert_eq!(fresh.fields[2], "0.25");
}
#[test]
fn t820_save_reload_preserves_coords_samples_assets_and_no_mode_metadata() {
    let (mut s, a, _) = fixture();
    let mut png = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([10, 40, 200, 180]))
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    let bytes = png.into_inner();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "preserved-image",
        bytes.clone(),
        2,
        2,
    ));
    let before_export = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    choose(&mut s, Mode::Mirrored).unwrap();
    assert_eq!(
        before_export,
        ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
    );
    let mut n = Numeric::new(&mut s, Side::Incoming).unwrap();
    n.fields[1] = "0.5".into();
    n.apply(&mut s).unwrap();
    let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("Tangent été.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let d = Document::from_loaded(ankhimate_formats::load(&path).unwrap().0);
    assert_eq!(
        before,
        ankhimate_formats::to_json(&d.as_project_ref()).unwrap()
    );
    assert!(!before.contains("Mirrored"));
    let Timeline::BoneTranslate { keys: old, .. } = &s.doc.animations[a].timelines[0] else {
        unreachable!()
    };
    let Timeline::BoneTranslate { keys: new, .. } =
        &d.animations.values().next().unwrap().timelines[0]
    else {
        unreachable!()
    };
    for i in 0..=200 {
        let t = i as f32 / 100.;
        assert_eq!(
            ankhimate_core::animation::sample(old, t, &mut 0),
            ankhimate_core::animation::sample(new, t, &mut 0)
        );
    }
    s.replace_document(d);
    assert_eq!(mode(&mut s), Mode::Broken);
    assert_eq!(s.doc.assets.images.len(), 1);
    assert_eq!(s.doc.assets.images.values().next().unwrap().bytes, bytes);
}
#[test]
fn t820_egui_mode_click_is_real_and_does_not_author() {
    let (mut s, _, _) = fixture();
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    let mut rect = egui::Rect::NOTHING;
    let _ = ctx.run_ui(Default::default(), |ui| {
        rect = mode_ui(ui, &mut s)[2];
    });
    let pos = rect.center();
    for pressed in [true, false] {
        let _ = ctx.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            },
            |ui| {
                mode_ui(ui, &mut s);
            },
        );
    }
    assert_eq!(mode(&mut s), Mode::Mirrored);
    assert!(snapshot(&s.doc) == before);
    assert!(!s.history.can_undo());
}
#[test]
fn t820_modes_disabled_with_reason_for_endpoints_and_multiselect() {
    let (mut s, a, mut r) = fixture();
    r.index = 0;
    key_selection::set(&mut s, a, vec![r]);
    let ctx = egui::Context::default();
    let out = ctx.run_ui(Default::default(), |ui| {
        mode_ui(ui, &mut s);
    });
    assert!(out.shapes.iter().any(|sh|matches!(&sh.shape,egui::epaint::Shape::Text(t)if t.galley.text().contains("two neighboring"))));
    assert!(choose(&mut s, Mode::Aligned).is_err());
}

#[test]
fn t820_unedited_native_exports_remain_byte_exact_across_mode_choice() {
    let (mut s, _, _) = fixture();
    let tmp = tempfile::tempdir().unwrap();
    let export = |s: &AppState, name: &str| {
        let project = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
        let plan = ankhimate_export::run::plan(
            &project,
            &s.doc.assets,
            &ankhimate_export::presets::default_preset(),
        )
        .unwrap();
        let dir = tmp.path().join(name);
        ankhimate_export::run::write(&plan, &dir).unwrap();
        let mut files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|p| {
                let p = p.unwrap().path();
                (
                    p.file_name().unwrap().to_owned(),
                    std::fs::read(&p).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        files.sort();
        files
    };
    let before = export(&s, "before");
    choose(&mut s, Mode::Mirrored).unwrap();
    let after = export(&s, "after");
    assert_eq!(before, after);
}

#[test]
fn t820_drag_other_clip_replaced_document_and_sorted_keys_stop() {
    for case in 0..3 {
        let (mut s, a, r) = fixture();
        choose(&mut s, Mode::Mirrored).unwrap();
        let mut g = Gesture::begin(&mut s, a, r, Side::Incoming, 91).unwrap();
        g.update(&mut s, glam::vec2(0.8, 0.5)).unwrap();
        match case {
            0 => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(Animation::new("other", 3.)));
            }
            1 => {
                s.replace_document(s.doc.clone());
            }
            _ => {
                if let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[a].timelines[0]
                {
                    keys.swap(0, 1);
                }
            }
        }
        let before = snapshot(&s.doc);
        assert!(g.update(&mut s, glam::vec2(0.8, 0.6)).is_err());
        assert!(snapshot(&s.doc) == before);
    }
}
