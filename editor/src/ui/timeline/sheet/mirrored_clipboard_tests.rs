use super::*;
const ACTION: &str = "Paste Keys Mirrored (Offsets)";
#[test]
fn context_menu_offers_mirrored_bone_key_offsets() {
    let (mut state, anim, key) = fixture();
    state.session.clipboard = ankhimate_document::clipboard::Clipboard::Keys(
        ankhimate_document::clipboard::KeyClip::capture(
            &state.doc,
            anim,
            std::slice::from_ref(&key),
        )
        .unwrap(),
    );
    let ctx = egui::Context::default();
    let out = ctx.run_ui(Default::default(), |ui| {
        key_context_menu(
            ui,
            &mut state,
            anim,
            &Selection {
                keys: vec![key.clone()],
            },
            &crate::registry::Registry::with_builtins(),
        )
    });
    let labels: Vec<_> = out
        .shapes
        .iter()
        .filter_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) => Some(t.galley.text().to_owned()),
            _ => None,
        })
        .collect();
    assert!(
        labels.iter().any(|s| s == ACTION),
        "Missing context action: {ACTION}"
    );
}

use ankhimate_core::animation::{Axis, Interp};
use ankhimate_document::{
    clipboard::{
        Clipboard, KeyClip,
        keys::{CopiedKey, KeyData, NamedTrack, TrackKind},
    },
    commands::key_cmds::{AddKey, KeyValue, MoveKeys},
};
const OP: &str = "anim.paste_keys_mirrored_offsets";
fn full(state: &AppState) -> String {
    format!("{:?}", state.doc)
}
fn captured(state: &AppState, anim: AnimationId, keys: &[KeyRef]) -> Clipboard {
    Clipboard::Keys(KeyClip::capture(&state.doc, anim, keys).unwrap())
}
fn invoke(state: &mut AppState, anim: AnimationId, refs: &[KeyRef]) {
    let args = crate::key_clipboard::selection_args(state, anim, refs).unwrap();
    crate::registry::Registry::with_builtins()
        .try_invoke(OP, state, &args)
        .unwrap();
}
#[test]
fn real_dopesheet_context_action_pastes_a_block_once_and_undo_redo_owns_its_data() {
    let (mut state, anim, key) = fixture();
    let mut next = key.clone();
    next.index = 1;
    state.session.clipboard = captured(&state, anim, &[key.clone(), next]);
    let ctx = egui::Context::default();
    frame(&ctx, &mut state, vec![]);
    state.set_playhead(1.5);
    let before = full(&state);
    let depth = state.history.undo_depth();
    click(
        &ctx,
        &mut state,
        egui::pos2(70., 83.),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, ACTION);
    assert_eq!(state.history.undo_depth(), depth + 1);
    assert_eq!(state.history.undo_label(), Some(ACTION));
    let Timeline::BoneRotate { keys, .. } = &state.doc.animations[anim].timelines[0] else {
        panic!()
    };
    assert_eq!(keys[2], Key::linear(1.5, -12.));
    assert_eq!(keys[3], Key::stepped(2., -34.));
    let after = full(&state);
    state.session.clipboard = Clipboard::Empty;
    for _ in 0..3 {
        state.undo();
        assert_eq!(full(&state), before);
        state.redo();
        assert_eq!(full(&state), after);
    }
}
fn menu_frame(
    ctx: &egui::Context,
    state: &mut AppState,
    anim: AnimationId,
    selection: &Selection,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900., 700.),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            key_context_menu(
                ui,
                state,
                anim,
                selection,
                &crate::registry::Registry::with_builtins(),
            )
        },
    )
}
fn press_menu(ctx: &egui::Context, state: &mut AppState, anim: AnimationId, selection: &Selection) {
    let output = menu_frame(ctx, state, anim, selection, vec![]);
    let pos = output
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == ACTION => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .unwrap();
    for pressed in [true, false] {
        menu_frame(
            ctx,
            state,
            anim,
            selection,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
    }
}
#[test]
fn destination_rows_use_visible_order_even_when_selected_in_reverse_between_animations() {
    let (mut state, source, key) = fixture();
    let root = state.doc.skeleton.update_order[0];
    let source_b = state.doc.skeleton.add_bone(Bone {
        name: "source.R".into(),
        ..state.doc.skeleton.bones[root].clone()
    });
    state.doc.animations[source]
        .timelines
        .push(Timeline::BoneRotate {
            bone: source_b,
            keys: vec![Key::linear(0.25, 66.)],
        });
    let source_refs = vec![
        key,
        KeyRef {
            addr: TimelineAddr::Bone {
                bone: source_b,
                property: BoneProperty::Rotate,
                axis: None,
            },
            index: 0,
        },
    ];
    state.session.clipboard = captured(&state, source, &source_refs);
    let a = state.doc.skeleton.add_bone(Bone {
        name: "arbitrary_target_one".into(),
        ..state.doc.skeleton.bones[root].clone()
    });
    let b = state.doc.skeleton.add_bone(Bone {
        name: "unrelated_left_name".into(),
        ..state.doc.skeleton.bones[root].clone()
    });
    let mut dest = Animation::new("destination", 3.);
    for bone in [a, b] {
        dest.timelines.push(Timeline::BoneRotate {
            bone,
            keys: vec![Key::linear(0., 0.)],
        });
    }
    let anim = state.doc.animations.insert(dest);
    state.session.active_animation = Some(anim);
    state.set_playhead(1.);
    let original = format!("{:?}", state.doc.animations[source]);
    let selection = Selection {
        keys: vec![
            KeyRef {
                addr: TimelineAddr::Bone {
                    bone: b,
                    property: BoneProperty::Rotate,
                    axis: None,
                },
                index: 0,
            },
            KeyRef {
                addr: TimelineAddr::Bone {
                    bone: a,
                    property: BoneProperty::Rotate,
                    axis: None,
                },
                index: 0,
            },
        ],
    };
    press_menu(&egui::Context::default(), &mut state, anim, &selection);
    assert_eq!(state.history.undo_depth(), 1);
    for (i, value) in [-12., -66.].into_iter().enumerate() {
        let Timeline::BoneRotate { keys, .. } = &state.doc.animations[anim].timelines[i] else {
            panic!()
        };
        assert_eq!(keys[1].value, value);
    }
    assert_eq!(format!("{:?}", state.doc.animations[source]), original);
}
#[test]
fn invalid_requests_preserve_complete_document_previews_and_existing_redo_history() {
    for case in [
        "no-destination",
        "locked",
        "missing-row",
        "wrong-kind",
        "mixed-source",
        "nan-source",
        "nan-time",
        "ambiguous-name",
        "ambiguous-animation",
        "stale-index",
        "collision",
        "setup",
    ] {
        let (mut state, anim, key) = fixture();
        let bone = state.doc.skeleton.update_order[0];
        state.session.clipboard = captured(&state, anim, std::slice::from_ref(&key));
        state.dispatch(Box::new(AddKey::new(
            anim,
            key.addr.clone(),
            1.,
            KeyValue::Scalar(80.),
            Interp::Linear,
        )));
        state.undo();
        let mut refs = vec![key.clone()];
        match case {
            "no-destination" => refs.clear(),
            "locked" => {
                state.session.locked_bones.insert(bone, true);
            }
            "missing-row" => {
                state.doc.animations[anim].timelines.clear();
            }
            "wrong-kind" => {
                state.doc.animations[anim]
                    .timelines
                    .push(Timeline::BoneScale {
                        bone,
                        axis: Axis::X,
                        keys: vec![Key::linear(0., 1.)],
                    });
                refs[0].addr = TimelineAddr::Bone {
                    bone,
                    property: BoneProperty::Scale,
                    axis: Some(Axis::X),
                };
            }
            "mixed-source" => {
                if let Clipboard::Keys(clip) = &mut state.session.clipboard {
                    clip.keys.push(CopiedKey {
                        track: NamedTrack {
                            name: "slot".into(),
                            kind: TrackKind::Color,
                        },
                        key: Key::linear(0.25, KeyData::Color([1.; 4])),
                    });
                }
            }
            "nan-source" => {
                if let Clipboard::Keys(clip) = &mut state.session.clipboard {
                    clip.keys[0].key.value = KeyData::Scalar(f32::NAN);
                }
            }
            "nan-time" => state.session.playhead = f32::NAN,
            "ambiguous-name" => {
                let extra = state
                    .doc
                    .skeleton
                    .add_bone(state.doc.skeleton.bones[bone].clone());
                state.doc.skeleton.bones[extra].name = "root".into();
            }
            "ambiguous-animation" => {
                state.doc.animations.insert(Animation::new("source", 3.));
            }
            "stale-index" => refs[0].index = 99,
            "collision" => {
                if let Clipboard::Keys(clip) = &mut state.session.clipboard {
                    let mut other = clip.keys[0].clone();
                    other.track.name = "second-source".into();
                    clip.keys.push(other);
                }
            }
            "setup" => {
                state.set_work_mode(crate::session::WorkMode::Setup);
            }
            _ => unreachable!(),
        }
        let before = full(&state);
        let revision = state.revision;
        let previews = format!("{:?}", state.session.preview_locals);
        let depth = state.history.undo_depth();
        let redo_label = state.history.redo_label().map(str::to_owned);
        let args = crate::key_clipboard::selection_args(&state, anim, &refs);
        // Missing/ambiguous row arguments are rejected at the menu boundary;
        // an explicitly crafted operator request must be refused as well.
        let args = args.unwrap_or_else(|_| {
            ankhimate_document::Args::from_json(
                serde_json::json!({"animation":"source","tracks":[0],"indices":[99]}),
            )
        });
        assert!(
            !crate::key_clipboard::can_paste_mirrored_offsets(&state, &args),
            "{case}"
        );
        crate::registry::Registry::with_builtins()
            .try_invoke(OP, &mut state, &args)
            .unwrap();
        assert_eq!(full(&state), before, "{case}");
        assert_eq!(state.revision, revision, "{case}");
        assert_eq!(state.history.undo_depth(), depth);
        assert_eq!(state.history.redo_label(), redo_label.as_deref());
        assert!(state.history.can_redo());
        assert_eq!(format!("{:?}", state.session.preview_locals), previews);
    }
}
#[test]
fn incompatible_or_unselected_destination_menu_action_cannot_be_clicked() {
    for case in ["empty-selection", "locked", "slot-clipboard"] {
        let (mut state, anim, key) = fixture();
        state.session.clipboard = captured(&state, anim, std::slice::from_ref(&key));
        let mut selected = Selection {
            keys: vec![key.clone()],
        };
        match case {
            "empty-selection" => selected.keys.clear(),
            "locked" => {
                state
                    .session
                    .locked_bones
                    .insert(state.doc.skeleton.update_order[0], true);
            }
            "slot-clipboard" => {
                if let Clipboard::Keys(clip) = &mut state.session.clipboard {
                    clip.keys[0].track.kind = TrackKind::Visible;
                    clip.keys[0].key.value = KeyData::Visible(true);
                }
            }
            _ => unreachable!(),
        }
        let before = full(&state);
        press_menu(&egui::Context::default(), &mut state, anim, &selected);
        assert_eq!(full(&state), before);
        assert_eq!(state.history.undo_depth(), 0);
    }
}
#[test]
fn repeated_mirror_pastes_remain_separate_from_drag_and_normal_paste_does_not_change_clipboard() {
    let (mut state, anim, key) = fixture();
    state.session.clipboard = captured(&state, anim, std::slice::from_ref(&key));
    let original_clip = match state.session.clipboard.clone() {
        Clipboard::Keys(c) => c,
        _ => panic!(),
    };
    state.dispatch(Box::new(MoveKeys::new(anim, vec![key.clone()], 0.1)));
    let dragged = full(&state);
    let depth = state.history.undo_depth();
    state.set_playhead(1.5);
    invoke(&mut state, anim, std::slice::from_ref(&key));
    let once = full(&state);
    invoke(&mut state, anim, std::slice::from_ref(&key));
    assert_eq!(full(&state), once);
    assert_eq!(state.history.undo_depth(), depth + 2);
    let Clipboard::Keys(clip) = &state.session.clipboard else {
        panic!()
    };
    assert_eq!(*clip, original_clip);
    state.undo();
    assert_eq!(full(&state), once);
    state.undo();
    assert_eq!(full(&state), dragged);
    // Existing normal paste may still use original named tracks with no selection.
    let args = crate::key_clipboard::selection_args(&state, anim, &[]).unwrap();
    crate::key_clipboard::paste(&mut state, &args);
    let Timeline::BoneRotate { keys, .. } = &state.doc.animations[anim].timelines[0] else {
        panic!()
    };
    assert_eq!(keys.last().unwrap().value, 12.);
}
#[test]
fn mirrored_paste_rebases_t210_pending_components_through_undo_redo() {
    let (mut state, anim, key) = fixture();
    let bone = state.doc.skeleton.update_order[0];
    state.session.clipboard = captured(&state, anim, std::slice::from_ref(&key));
    state.set_playhead(1.5);
    state.session.auto_key = false;
    state.commit_bone_pose(
        bone,
        Transform {
            position: glam::vec2(0., 17.),
            ..state.pose.locals[bone]
        },
    );
    let before = full(&state);
    invoke(&mut state, anim, std::slice::from_ref(&key));
    let after = full(&state);
    for _ in 0..3 {
        assert_eq!(state.pose.locals[bone].position.y, 17.);
        assert!((state.pose.locals[bone].rotation.to_degrees() + 12.).abs() < 0.0001);
        state.undo();
        assert_eq!(full(&state), before);
        assert_eq!(state.pose.locals[bone].position.y, 17.);
        assert!((state.pose.locals[bone].rotation.to_degrees() - 34.).abs() < 0.0001);
        state.redo();
        assert_eq!(full(&state), after);
    }
    assert!(state.session.pending_pose.contains(&bone));
    assert_eq!(state.history.undo_depth(), 1);
}
#[test]
fn real_assets_skins_and_offset_keys_survive_save_reopen_after_mirrored_paste() {
    let loaded = ankhimate_formats::load(std::path::Path::new("../samples/walker.ankh"))
        .unwrap()
        .0;
    let mut state = AppState::default();
    state.replace_document(ankhimate_document::Document::from_loaded(loaded));
    let bone = state.doc.skeleton.update_order[0];
    let mut a = Animation::new("offset persistence", 3.);
    a.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::X,
        keys: vec![
            Key::linear(0.25, 10.),
            Key {
                time: 0.75,
                value: -5.,
                interp: Interp::Bezier {
                    out_handle: glam::vec2(0.2, -0.5),
                    in_handle: glam::vec2(0.8, 1.4),
                },
            },
        ],
    });
    let anim = state.doc.animations.insert(a);
    state.session.active_animation = Some(anim);
    state.set_work_mode(crate::session::WorkMode::Animate);
    state.set_playhead(1.5);
    let key = KeyRef {
        addr: TimelineAddr::Bone {
            bone,
            property: BoneProperty::Translate,
            axis: Some(Axis::X),
        },
        index: 0,
    };
    let mut next = key.clone();
    next.index = 1;
    state.session.clipboard = captured(&state, anim, &[key.clone(), next]);
    let assets = serde_json::to_value(&state.doc.assets).unwrap();
    let before = full(&state);
    invoke(&mut state, anim, std::slice::from_ref(&key));
    let after = full(&state);
    state.session.clipboard = Clipboard::Empty;
    for _ in 0..3 {
        state.undo();
        assert_eq!(full(&state), before);
        state.redo();
        assert_eq!(full(&state), after);
    }
    assert_eq!(serde_json::to_value(&state.doc.assets).unwrap(), assets);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("miroir été.ankh");
    ankhimate_formats::save(&path, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.is_clean());
    let reopened = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        json(&state)
    );
    for asset in state.doc.assets.images.values() {
        assert!(
            reopened
                .assets
                .images
                .values()
                .any(|a| a.name == asset.name && a.bytes == asset.bytes)
        );
    }
}

#[test]
fn readonly_and_mixed_slot_destination_rows_never_enter_the_mirror_transaction() {
    for case in [
        "draw-order",
        "ffd",
        "ik",
        "slot-color",
        "slot-visible",
        "slot-attachment",
        "mixed",
    ] {
        let (mut state, anim, key) = fixture();
        let bone = state.doc.skeleton.update_order[0];
        state.session.clipboard = captured(&state, anim, std::slice::from_ref(&key));
        let slot = state
            .doc
            .skeleton
            .add_slot(ankhimate_core::slot::Slot::new("costume".into(), bone));
        let timeline = match case {
            "draw-order" => Timeline::DrawOrder {
                keys: vec![Key::linear(0., vec![])],
            },
            "ffd" => Timeline::Deform {
                slot,
                attachment: "mesh".into(),
                keys: vec![Key::linear(0., vec![glam::Vec2::ZERO])],
            },
            "ik" => Timeline::IkMix {
                constraint: Default::default(),
                keys: vec![Key::linear(0., 1.)],
            },
            "slot-visible" => Timeline::SlotVisible {
                slot,
                keys: vec![Key::stepped(0., true)],
            },
            "slot-attachment" => Timeline::SlotAttachment {
                slot,
                keys: vec![Key::stepped(0., None)],
            },
            _ => Timeline::SlotColor {
                slot,
                keys: vec![Key::linear(0., [1.; 4])],
            },
        };
        state.doc.animations[anim].timelines.push(timeline);
        let before = full(&state);
        let indices = if case == "mixed" { vec![0, 1] } else { vec![1] };
        let args = ankhimate_document::Args::from_json(
            serde_json::json!({"animation":"source","tracks":indices,"indices":if case=="mixed" {vec![0,0]} else {vec![0]}}),
        );
        assert!(
            !crate::key_clipboard::can_paste_mirrored_offsets(&state, &args),
            "{case}"
        );
        crate::registry::Registry::with_builtins()
            .try_invoke(OP, &mut state, &args)
            .unwrap();
        assert_eq!(full(&state), before);
        assert_eq!(state.history.undo_depth(), 0);
    }
}
