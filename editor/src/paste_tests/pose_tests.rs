//! End-to-end pose clipboard/history regressions (T-209.2).
use crate::{app_state::AppState, session::WorkMode};
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    ids::BoneId,
    math::Transform,
    skeleton::Bone,
};
use ankhimate_document::{
    clipboard::{Clipboard, PoseClip},
    commands::History,
};

fn fixture() -> (AppState, Vec<BoneId>) {
    let mut s = AppState::default();
    let mut bones = vec![];
    for i in 0..4 {
        bones.push(s.doc.skeleton.add_bone(Bone {
            name: format!("arm_{i}"),
            parent: bones.last().copied(),
            length: 10.,
            local_transform: Transform {
                position: glam::vec2(10. * (i + 1) as f32, 7.),
                rotation: 0.1 * (i + 1) as f32,
                scale: glam::vec2(2., 3.),
                shear: glam::vec2(0.2, -0.1),
            },
            inherit: Default::default(),
            color: Bone::default_color(),
        }));
    }
    s.refresh_pose();
    s.session.selected_bones = bones.clone();
    s.copy_pose();
    for &b in &bones {
        s.commit_bone_pose(b, Transform::default());
    }
    s.history = History::default();
    (s, bones)
}
fn document(s: &AppState) -> String {
    format!("{:?}", s.doc)
}
fn atomic_paste(mirror: bool) {
    let (mut s, bones) = fixture();
    let Clipboard::Pose(clip) = s.session.clipboard.clone() else {
        panic!()
    };
    let expected = if mirror { clip.mirrored() } else { clip };
    let before = document(&s);
    let depth = s.history.undo_depth();
    s.paste(mirror);
    let steps = s.history.undo_depth() - depth;
    eprintln!("four bones, mirrored={mirror}: {steps} Undo step(s)");
    for (&b, (_, local)) in bones.iter().zip(&expected.entries) {
        assert_eq!(s.doc.skeleton.bones[b].local_transform, *local);
    }
    let after = document(&s);
    assert_eq!(steps, 1, "a pose paste must be one transaction");
    s.session.clipboard = Clipboard::Empty;
    for _ in 0..3 {
        s.undo();
        assert_eq!(document(&s), before);
        s.redo();
        assert_eq!(document(&s), after);
    }
}
#[test]
fn four_bone_pose_paste_is_one_undo() {
    atomic_paste(false);
}
#[test]
fn four_bone_mirrored_pose_paste_is_one_undo() {
    atomic_paste(true);
}

fn legacy_paste(s: &mut AppState, mirrored: bool) {
    let Clipboard::Pose(clip) = s.session.clipboard.clone() else {
        panic!()
    };
    let clip = if mirrored { clip.mirrored() } else { clip };
    for (name, value) in clip.entries {
        let id = s
            .doc
            .skeleton
            .bones
            .iter()
            .find(|(_, b)| b.name == name)
            .unwrap()
            .0;
        s.commit_bone_pose(id, value);
    }
}
fn animate(s: &mut AppState, bones: &[BoneId], time: f32) {
    let mut a = Animation::new("destination", 1.);
    a.timelines.push(Timeline::BoneRotate {
        bone: bones[0],
        keys: vec![
            Key::stepped(0., 5.),
            Key {
                time: 0.5,
                value: 13.,
                interp: ankhimate_core::animation::Interp::Bezier {
                    out_handle: glam::vec2(0.2, -0.1),
                    in_handle: glam::vec2(0.8, 1.1),
                },
            },
        ],
    });
    let id = s.doc.animations.insert(a);
    s.session.active_animation = Some(id);
    s.set_work_mode(WorkMode::Animate);
    s.set_playhead(time);
    s.history = History::default();
}
#[test]
fn setup_and_animate_match_legacy_numerics_at_all_playheads_with_three_full_round_trips() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        for mirrored in [false, true] {
            for time in [0., 0.75, 2.] {
                let (mut s, bones) = fixture();
                let (mut old, old_bones) = fixture();
                if mode == WorkMode::Animate {
                    animate(&mut s, &bones, time);
                    animate(&mut old, &old_bones, time);
                }
                let before = document(&s);
                let revision = s.revision;
                legacy_paste(&mut old, mirrored);
                s.paste(mirrored);
                assert_eq!(
                    document(&s),
                    document(&old),
                    "numerical contract {mode:?} {mirrored} {time}"
                );
                assert_eq!(s.history.undo_depth(), 1);
                assert_eq!(s.revision, revision + 1, "one document notification");
                let after = document(&s);
                if mode == WorkMode::Setup {
                    assert!(s.doc.animations.is_empty());
                }
                s.session.clipboard = Clipboard::Empty;
                for _ in 0..3 {
                    s.undo();
                    assert_eq!(document(&s), before);
                    s.redo();
                    assert_eq!(document(&s), after);
                }
            }
        }
    }
}
#[test]
fn single_bone_paste_after_drag_and_two_pastes_are_three_independent_transactions() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        let (mut s, bones) = fixture();
        if mode == WorkMode::Animate {
            animate(&mut s, &bones, 0.75);
        }
        if let Clipboard::Pose(clip) = &mut s.session.clipboard {
            clip.entries.truncate(1);
        }
        let original = document(&s);
        let local = Transform {
            position: glam::vec2(6., 0.),
            ..Default::default()
        };
        s.commit_bone_pose(bones[0], local);
        let dragged = document(&s);
        let drag_depth = s.history.undo_depth();
        s.paste(false);
        assert_eq!(s.history.undo_depth(), drag_depth + 1);
        let normal = document(&s);
        s.paste(true);
        assert_eq!(s.history.undo_depth(), drag_depth + 2);
        let mirrored = document(&s);
        s.undo();
        assert_eq!(document(&s), normal);
        s.undo();
        assert_eq!(document(&s), dragged);
        for _ in 0..drag_depth {
            s.undo();
        }
        assert_eq!(document(&s), original);
        for _ in 0..drag_depth {
            s.redo();
        }
        s.redo();
        s.redo();
        assert_eq!(document(&s), mirrored);
    }
}
#[test]
fn auto_key_off_retains_session_only_previews_and_validates_before_any_preview_write() {
    for mirror in [false, true] {
        let (mut s, bones) = fixture();
        let (mut old, old_bones) = fixture();
        animate(&mut s, &bones, 0.75);
        animate(&mut old, &old_bones, 0.75);
        s.session.auto_key = false;
        old.session.auto_key = false;
        let before = document(&s);
        s.paste(mirror);
        legacy_paste(&mut old, mirror);
        assert_eq!(document(&s), before);
        assert_eq!(s.history.undo_depth(), 0);
        assert_eq!(s.session.pending_pose, old.session.pending_pose);
        for b in &bones {
            assert_eq!(s.pose.locals[*b], old.pose.locals[*b]);
        }
        let previews = format!("{:?}", s.session.preview_locals);
        if let Clipboard::Pose(clip) = &mut s.session.clipboard {
            clip.entries.last_mut().unwrap().0 = "absent".into();
        }
        s.paste(mirror);
        assert_eq!(format!("{:?}", s.session.preview_locals), previews);
        assert_eq!(document(&s), before);
    }
}
#[test]
fn pending_previews_with_auto_key_on_follow_the_existing_pose_commit_contract() {
    for mirror in [false, true] {
        let (mut s, bones) = fixture();
        let (mut old, old_bones) = fixture();
        animate(&mut s, &bones, 0.75);
        animate(&mut old, &old_bones, 0.75);
        for state in [&mut s, &mut old] {
            state.session.auto_key = false;
            state.commit_bone_pose(
                bones[0],
                Transform {
                    position: glam::vec2(0., 99.),
                    ..state.pose.locals[bones[0]]
                },
            );
            state.session.auto_key = true;
        }
        s.paste(mirror);
        legacy_paste(&mut old, mirror);
        assert_eq!(document(&s), document(&old));
        assert_eq!(s.session.pending_pose, old.session.pending_pose);
        for &b in &bones {
            assert_eq!(s.pose.locals[b], old.pose.locals[b]);
        }
        let after = document(&s);
        for _ in 0..3 {
            s.undo();
            s.redo();
            assert_eq!(document(&s), after);
            assert_eq!(s.pose.locals[bones[0]], old.pose.locals[bones[0]]);
        }
    }
}
#[test]
fn missing_locked_ambiguous_duplicate_and_nonfinite_targets_refuse_the_entire_pose() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        for case in [
            "missing",
            "locked",
            "ambiguous",
            "duplicate",
            "value",
            "empty",
            "stale-animation",
            "time",
        ] {
            let (mut s, bones) = fixture();
            if mode == WorkMode::Animate {
                animate(&mut s, &bones, 0.75);
            }
            match case {
                "missing" => {
                    if let Clipboard::Pose(clip) = &mut s.session.clipboard {
                        clip.entries[3].0 = "absent".into();
                    }
                }
                "locked" => {
                    s.session.locked_bones.insert(bones[3], true);
                }
                "ambiguous" => {
                    let b = s.doc.skeleton.bones[bones[3]].clone();
                    let duplicate = s.doc.skeleton.add_bone(b);
                    s.doc.skeleton.bones[duplicate].name = "arm_3".into();
                }
                "duplicate" => {
                    if let Clipboard::Pose(clip) = &mut s.session.clipboard {
                        clip.entries[3].0 = clip.entries[0].0.clone();
                    }
                }
                "value" => {
                    if let Clipboard::Pose(clip) = &mut s.session.clipboard {
                        clip.entries[3].1.rotation = f32::NAN;
                    }
                }
                "empty" => s.session.clipboard = Clipboard::Pose(PoseClip::default()),
                "stale-animation" => {
                    if mode == WorkMode::Setup {
                        continue;
                    }
                    s.doc.animations.remove(s.session.active_animation.unwrap());
                }
                "time" => {
                    if mode == WorkMode::Setup {
                        continue;
                    }
                    s.session.playhead = f32::NAN;
                }
                _ => unreachable!(),
            }
            s.refresh_pose();
            let before = document(&s);
            let revision = s.revision;
            let pose = format!("{:?}", s.pose.locals);
            s.paste(false);
            assert_eq!(document(&s), before, "{mode:?} {case}");
            assert_eq!(s.history.undo_depth(), 0, "{case}");
            assert_eq!(s.revision, revision);
            assert_eq!(format!("{:?}", s.pose.locals), pose);
            assert!(
                s.session
                    .status
                    .as_ref()
                    .unwrap()
                    .starts_with("Paste refused:"),
                "{case}"
            );
        }
    }
}
#[test]
fn refused_pose_preserves_existing_redo_history_and_incompatible_clipboards_do_nothing() {
    let (mut s, bones) = fixture();
    s.commit_bone_pose(
        bones[0],
        Transform {
            rotation: 0.3,
            ..Default::default()
        },
    );
    s.undo();
    assert!(s.history.can_redo());
    let before = document(&s);
    for clip in [
        Clipboard::Empty,
        Clipboard::Keys(ankhimate_document::clipboard::KeyClip { keys: vec![] }),
        Clipboard::Pose(PoseClip {
            entries: vec![("missing".into(), Transform::default())],
        }),
    ] {
        s.session.clipboard = clip;
        s.paste(false);
        assert!(s.history.can_redo());
        assert_eq!(document(&s), before);
        assert_eq!(s.history.undo_depth(), 0);
    }
}
#[test]
fn source_animation_deletion_and_bone_id_churn_do_not_invalidate_named_pose_values() {
    let (mut s, bones) = fixture();
    animate(&mut s, &bones, 0.75);
    s.commit_bone_pose(
        bones[3],
        Transform {
            position: glam::vec2(3., 2.),
            ..Default::default()
        },
    );
    s.copy_pose();
    let Clipboard::Pose(copied) = s.session.clipboard.clone() else {
        panic!()
    };
    let source = s.session.active_animation.unwrap();
    s.doc.animations.remove(source);
    let old = bones[3];
    let value = s.doc.skeleton.bones[old].clone();
    s.doc.skeleton.remove_bone(old);
    let new = s.doc.skeleton.add_bone(value);
    assert_ne!(old, new);
    let dest = s
        .doc
        .animations
        .insert(Animation::new("new destination", 3.));
    s.session.active_animation = Some(dest);
    s.history = History::default();
    s.set_playhead(1.25);
    s.paste(false);
    assert_eq!(s.history.undo_depth(), 1);
    assert!(
        s.doc.animations[dest]
            .timelines
            .iter()
            .any(|t| matches!(t,Timeline::BoneTranslate {bone,..} if *bone==new))
    );
    assert_eq!(copied.entries.len(), 4);
    let after = document(&s);
    s.session.clipboard = Clipboard::Empty;
    for _ in 0..3 {
        s.undo();
        assert!(s.doc.animations[dest].timelines.is_empty());
        s.redo();
        assert_eq!(document(&s), after);
    }
}
#[test]
fn rotation_only_pose_adds_no_other_channels_and_keeps_existing_curves_and_times() {
    let (mut s, bones) = fixture();
    animate(&mut s, &bones, 0.75);
    let anim = s.session.active_animation.unwrap();
    s.session.clipboard = Clipboard::Pose(PoseClip {
        entries: vec![(
            "arm_0".into(),
            Transform {
                rotation: 0.4,
                ..Default::default()
            },
        )],
    });
    let original = s.doc.animations[anim].timelines[0].clone();
    s.paste(false);
    assert_eq!(s.doc.animations[anim].timelines.len(), 1);
    let Timeline::BoneRotate { keys: old, .. } = original else {
        panic!()
    };
    let Timeline::BoneRotate { keys, .. } = &s.doc.animations[anim].timelines[0] else {
        panic!()
    };
    assert_eq!(&keys[..2], old.as_slice());
    assert_eq!(keys[2].time, 0.75);
    assert_eq!(
        keys[2].value,
        ankhimate_core::transforms::wrap_angle(0.4).to_degrees()
    );
    assert_eq!(keys[2].interp, ankhimate_core::animation::Interp::Linear);
}
#[test]
fn real_assets_and_skins_survive_pose_paste_undo_redo_and_save_reopen() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        let (loaded, _) =
            ankhimate_formats::load(std::path::Path::new("../samples/walker.ankh")).unwrap();
        let mut s = AppState::default();
        s.replace_document(ankhimate_document::Document::from_loaded(loaded));
        let mut alternate = s.doc.skeleton.skins[s.doc.skeleton.default_skin].clone();
        alternate.name = "royal test".into();
        s.doc.skeleton.add_skin(alternate);
        assert!(!s.doc.assets.is_empty());
        let bones = s
            .doc
            .skeleton
            .update_order
            .iter()
            .take(4)
            .copied()
            .collect::<Vec<_>>();
        s.session.selected_bones = bones.clone();
        s.refresh_pose();
        s.copy_pose();
        for &b in &bones {
            let mut local = s.doc.skeleton.bones[b].local_transform;
            local.rotation += 0.25;
            s.commit_bone_pose(b, local);
        }
        if mode == WorkMode::Animate {
            animate(&mut s, &bones, 0.75);
        }
        s.history = History::default();
        let before = document(&s);
        let assets = serde_json::to_value(&s.doc.assets).unwrap();
        s.paste(true);
        assert_eq!(s.history.undo_depth(), 1);
        let after = document(&s);
        for _ in 0..3 {
            s.undo();
            assert_eq!(document(&s), before);
            s.redo();
            assert_eq!(document(&s), after);
        }
        assert_eq!(serde_json::to_value(&s.doc.assets).unwrap(), assets);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pose été.ankh");
        ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
        let (loaded, _) = ankhimate_formats::load(&path).unwrap();
        assert!(loaded.report.is_clean());
        let reopened = ankhimate_document::Document::from_loaded(loaded);
        assert_eq!(
            ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
            ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
        );
        for asset in s.doc.assets.images.values() {
            assert!(
                reopened
                    .assets
                    .images
                    .values()
                    .any(|a| a.name == asset.name && a.bytes == asset.bytes)
            );
        }
    }
}
#[test]
fn global_shortcuts_invoke_the_atomic_pose_transaction_and_keep_typing_priority() {
    let ops = crate::registry::Registry::with_builtins();
    let map = crate::keymap::Keymap::builtin();
    for (id, mirror) in [("edit.paste", false), ("edit.paste_mirrored", true)] {
        let (mut s, _) = fixture();
        let before = document(&s);
        let chord = map.chord_for(id).unwrap();
        let mods = eframe::egui::Modifiers {
            ctrl: chord.ctrl,
            shift: chord.shift,
            alt: chord.alt,
            ..Default::default()
        };
        let fired = map.resolve_pressed(false, &mods, |k| k == chord.key);
        assert!(fired.contains(&id));
        for op in fired {
            ops.invoke(op, &mut s);
        }
        assert_eq!(s.history.undo_depth(), 1);
        assert_eq!(
            s.history.undo_label(),
            Some(if mirror {
                "Paste Mirrored Pose"
            } else {
                "Paste Pose"
            })
        );
        s.undo();
        assert_eq!(document(&s), before);
        assert!(
            !map.resolve_pressed(true, &mods, |k| k == chord.key)
                .contains(&id)
        );
    }
}

#[test]
fn paste_between_animations_does_not_change_source_or_unrelated_author_data() {
    let (mut s, bones) = fixture();
    animate(&mut s, &bones, 0.75);
    let source = s.session.active_animation.unwrap();
    s.copy_pose();
    let source_before = format!("{:?}", s.doc.animations[source]);
    let dest = s.doc.animations.insert(Animation::new("second", 3.));
    s.session.active_animation = Some(dest);
    s.set_playhead(1.25);
    s.history = History::default();
    let before = document(&s);
    s.paste(false);
    assert_eq!(s.history.undo_depth(), 1);
    assert_eq!(format!("{:?}", s.doc.animations[source]), source_before);
    assert!(!s.doc.animations[dest].timelines.is_empty());
    let after = document(&s);
    for _ in 0..3 {
        s.undo();
        assert_eq!(document(&s), before);
        s.redo();
        assert_eq!(document(&s), after);
    }
}

#[test]
fn identical_successive_four_bone_pastes_are_not_merged_or_dropped() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        let (mut s, bones) = fixture();
        if mode == WorkMode::Animate {
            animate(&mut s, &bones, 0.75);
        }
        let before = document(&s);
        s.paste(false);
        let after = document(&s);
        s.paste(false);
        assert_eq!(document(&s), after);
        assert_eq!(s.history.undo_depth(), 2);
        s.undo();
        assert_eq!(document(&s), after);
        s.undo();
        assert_eq!(document(&s), before);
        s.redo();
        s.redo();
        assert_eq!(document(&s), after);
    }
}
