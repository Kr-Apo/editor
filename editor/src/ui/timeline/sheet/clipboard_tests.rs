use super::*;
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    math::Transform,
    skeleton::Bone,
};
use ankhimate_document::commands::key_cmds::{BoneProperty, TimelineAddr};

fn fixture() -> (AppState, AnimationId, KeyRef) {
    let mut state = AppState::default();
    let bone = state.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.0,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut clip = Animation::new("source", 3.0);
    clip.timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![Key::linear(0.25, 12.0), Key::stepped(0.75, 34.0)],
    });
    let anim = state.doc.animations.insert(clip);
    state.session.active_animation = Some(anim);
    state.set_work_mode(crate::session::WorkMode::Animate);
    (
        state,
        anim,
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Rotate,
                axis: None,
            },
            index: 0,
        },
    )
}

#[test]
fn selected_key_menu_exposes_copy_and_paste() {
    let (mut state, anim, key) = fixture();
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
    eprintln!("selected key menu: {labels:?}");
    for label in ["Copy selected keys", "Paste keys at playhead"] {
        assert!(labels.iter().any(|s| s == label), "missing action: {label}");
    }
}

fn frame(ctx: &egui::Context, state: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(750.0, 650.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            let anim = state.session.active_animation.unwrap();
            let model = TimelineModel::build(state, anim);
            let theme = crate::theme::Theme::default();
            super::ui(
                ui,
                state,
                anim,
                &model,
                &mut ViewState::default(),
                &Layout {
                    sheet_x0: 30.0,
                    px_per_sec: 160.0,
                    scroll_sec: 0.0,
                    fps: 30,
                },
                egui::Rect::from_min_size(egui::pos2(15.0, 50.0), egui::vec2(600.0, 500.0)),
                super::super::Style {
                    theme: &theme,
                    text: 12.0,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
fn click(ctx: &egui::Context, state: &mut AppState, pos: egui::Pos2, button: egui::PointerButton) {
    frame(ctx, state, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            ctx,
            state,
            vec![egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
}
fn action(ctx: &egui::Context, state: &mut AppState, label: &str) {
    let out = frame(ctx, state, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == label => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing {label}"));
    click(ctx, state, pos, egui::PointerButton::Primary);
}
fn json(state: &AppState) -> String {
    ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
}

#[test]
fn pointer_copy_and_background_paste_are_one_undo_and_keep_t210_previews() {
    let (mut state, anim, key) = fixture();
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    frame(&ctx, &mut state, vec![]);
    click(
        &ctx,
        &mut state,
        egui::pos2(70.0, 83.0),
        egui::PointerButton::Secondary,
    );
    let before = json(&state);
    let depth = state.history.undo_depth();
    action(&ctx, &mut state, "Copy selected keys");
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    assert!(
        matches!(&state.session.clipboard,ankhimate_document::clipboard::Clipboard::Keys(c) if c.keys.len()==1)
    );
    state.set_playhead(1.5);
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    state.session.auto_key = false;
    let mut local = state.pose.locals[bone];
    local.position.y = 17.0;
    state.commit_bone_pose(bone, local);
    click(
        &ctx,
        &mut state,
        egui::pos2(380.0, 300.0),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, "Paste keys at playhead");
    assert_eq!(state.history.undo_depth(), depth + 1);
    assert!(
        matches!(&state.doc.animations[anim].timelines[0],Timeline::BoneRotate {keys,..} if keys.len()==3 && keys[2]==Key::linear(1.5,12.0))
    );
    assert_eq!(state.pose.locals[bone].position.y, 17.0);
    assert!(
        (state.pose.locals[bone].rotation.to_degrees() - 12.0).abs() < 1e-4,
        "an untouched component of a pending position must not mask the pasted rotation"
    );
    assert!(state.session.pending_pose.contains(&bone));
    let after = json(&state);
    for _ in 0..3 {
        state.undo();
        assert_eq!(json(&state), before);
        assert!((state.pose.locals[bone].rotation.to_degrees() - 34.0).abs() < 1e-4);
        assert_eq!(state.pose.locals[bone].position.y, 17.0);
        state.redo();
        assert_eq!(json(&state), after);
        assert!((state.pose.locals[bone].rotation.to_degrees() - 12.0).abs() < 1e-4);
        assert_eq!(state.pose.locals[bone].position.y, 17.0);
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pasted.ankh");
    ankhimate_formats::save(&path, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.is_clean());
    assert_eq!(
        ankhimate_formats::to_json(
            &ankhimate_document::Document::from_loaded(loaded).as_project_ref()
        )
        .unwrap(),
        after
    );
}

#[test]
fn real_menu_pastes_after_source_delete_into_an_empty_second_clip() {
    let (mut state, source, _) = fixture();
    let ctx = egui::Context::default();
    frame(&ctx, &mut state, vec![]);
    click(
        &ctx,
        &mut state,
        egui::pos2(70.0, 83.0),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, "Copy selected keys");
    state.dispatch(Box::new(
        ankhimate_document::commands::key_cmds::DeleteAnimation::new(source),
    ));
    let destination = state
        .doc
        .animations
        .insert(Animation::new("destination", 1.0));
    state.session.active_animation = Some(destination);
    state.set_playhead(2.0);
    frame(&ctx, &mut state, vec![]);
    click(
        &ctx,
        &mut state,
        egui::pos2(380.0, 300.0),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, "Paste keys at playhead");
    assert!(
        matches!(&state.doc.animations[destination].timelines[0],Timeline::BoneRotate {keys,..} if keys==&vec![Key::linear(2.0,12.0)])
    );
    assert_eq!(state.doc.animations[destination].duration, 2.0);
}

#[test]
fn selected_destination_key_row_maps_to_another_bone_in_the_real_sheet() {
    let (mut state, anim, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    let mut other = state.doc.skeleton.bones[bone].clone();
    other.name = "other".into();
    let other = state.doc.skeleton.add_bone(other);
    state.doc.animations[anim]
        .timelines
        .push(Timeline::BoneRotate {
            bone: other,
            keys: vec![Key::linear(0.25, 99.0)],
        });
    let ctx = egui::Context::default();
    frame(&ctx, &mut state, vec![]);
    click(
        &ctx,
        &mut state,
        egui::pos2(70.0, 83.0),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, "Copy selected keys");
    state.set_playhead(0.5);
    click(
        &ctx,
        &mut state,
        egui::pos2(70.0, 127.0),
        egui::PointerButton::Secondary,
    );
    action(&ctx, &mut state, "Paste keys at playhead");
    assert!(
        matches!(&state.doc.animations[anim].timelines[0],Timeline::BoneRotate {keys,..} if keys.len()==2)
    );
    assert!(
        matches!(&state.doc.animations[anim].timelines[1],Timeline::BoneRotate {keys,..} if keys==&vec![Key::linear(0.25,99.0),Key::linear(0.5,12.0)])
    );
}

#[test]
fn named_registry_operators_copy_paste_and_refuse_stale_or_readonly_arguments() {
    let (mut state, anim, key) = fixture();
    let registry = crate::registry::Registry::with_builtins();
    let args =
        crate::key_clipboard::selection_args(&state, anim, std::slice::from_ref(&key)).unwrap();
    registry
        .try_invoke("anim.copy_selected_keys", &mut state, &args)
        .unwrap();
    let copied = state.session.clipboard.describe();
    let before = json(&state);
    let depth = state.history.undo_depth();
    let invalid = ankhimate_document::Args::from_json(
        serde_json::json!({"animation":"source","tracks":[0],"indices":[999]}),
    );
    registry
        .try_invoke("anim.copy_selected_keys", &mut state, &invalid)
        .unwrap();
    assert_eq!(state.session.clipboard.describe(), copied);
    assert_eq!(json(&state), before);
    registry
        .try_invoke("anim.paste_keys_at_playhead", &mut state, &invalid)
        .unwrap();
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    state.doc.animations[anim]
        .timelines
        .push(Timeline::DrawOrder {
            keys: vec![Key::stepped(0.0, vec![])],
        });
    let readonly = ankhimate_document::Args::from_json(
        serde_json::json!({"animation":"source","tracks":[1],"indices":[0]}),
    );
    let before = json(&state);
    registry
        .try_invoke("anim.copy_selected_keys", &mut state, &readonly)
        .unwrap();
    assert!(state.session.status.as_ref().unwrap().contains("read-only"));
    assert_eq!(json(&state), before);
    state.set_work_mode(crate::session::WorkMode::Setup);
    registry
        .try_invoke("anim.paste_keys_at_playhead", &mut state, &args)
        .unwrap();
    assert_eq!(json(&state), before);
}

#[test]
fn global_clipboard_operators_keep_bones_and_poses_and_do_not_hijack_key_payloads() {
    let (mut state, anim, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    state.set_work_mode(crate::session::WorkMode::Setup);
    state.session.select_bone(Some(bone));
    state.copy_selection();
    state.paste(false);
    assert_eq!(state.doc.skeleton.bones.len(), 2);
    state.undo();
    assert_eq!(state.doc.skeleton.bones.len(), 1);
    state.copy_pose();
    state.paste(true);
    assert!(matches!(
        state.session.clipboard,
        ankhimate_document::clipboard::Clipboard::Pose(_)
    ));
    state.set_work_mode(crate::session::WorkMode::Animate);
    let args = crate::key_clipboard::selection_args(&state, anim, &[key]).unwrap();
    crate::key_clipboard::copy(&mut state, &args);
    let before = json(&state);
    let depth = state.history.undo_depth();
    state.paste(false);
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    assert!(state.session.status.as_ref().unwrap().contains("dopesheet"));
    let clipboard = state.session.clipboard.describe();
    state.set_work_mode(crate::session::WorkMode::Setup);
    state.session.select_bone(Some(bone));
    state.duplicate_selection();
    assert_eq!(state.session.clipboard.describe(), clipboard);
    assert_eq!(state.doc.skeleton.bones.len(), 2);
    state.undo();
    assert_eq!(state.doc.skeleton.bones.len(), 1);
}

#[test]
fn text_focus_retains_copy_paste_and_no_new_global_key_binding_is_added() {
    let ctx = egui::Context::default();
    let keymap = crate::keymap::Keymap::builtin();
    let mut text = "bone name".to_string();
    let _ = ctx.run_ui(Default::default(), |ui| {
        ui.text_edit_singleline(&mut text).request_focus();
    });
    for (key, event) in [
        (egui::Key::C, egui::Event::Copy),
        (egui::Key::V, egui::Event::Paste("inserted".into())),
    ] {
        let out = ctx.run_ui(
            egui::RawInput {
                modifiers: egui::Modifiers::CTRL,
                events: vec![
                    event,
                    egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::CTRL,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                ui.text_edit_singleline(&mut text);
                let typing = ctx.egui_wants_keyboard_input();
                let resolved = ui.input(|input| {
                    keymap
                        .resolve(input, typing)
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                });
                assert!(
                    resolved.is_empty(),
                    "a document operator intercepted a text clipboard action: {resolved:?}"
                );
            },
        );
        let _ = out;
    }
    assert!(text.contains("inserted"));
    assert_eq!(keymap.chord_for("anim.copy_selected_keys"), None);
    assert_eq!(keymap.chord_for("anim.paste_keys_at_playhead"), None);
    assert_eq!(
        keymap.resolve_pressed(false, &egui::Modifiers::CTRL, |k| k == egui::Key::C),
        vec!["edit.copy"]
    );
    assert_eq!(
        keymap.resolve_pressed(false, &egui::Modifiers::CTRL, |k| k == egui::Key::V),
        vec!["edit.paste"]
    );
}

#[test]
fn menu_copy_uses_row_order_instead_of_click_order_and_paste_refuses_mismatched_rows_atomically() {
    let (mut state, anim, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    state.doc.animations[anim]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: ankhimate_core::animation::Axis::Y,
            keys: vec![Key::stepped(0.5, 9.0)],
        });
    let second = KeyRef {
        addr: TimelineAddr::Bone {
            bone,
            property: BoneProperty::Translate,
            axis: Some(ankhimate_core::animation::Axis::Y),
        },
        index: 0,
    };
    let selection = Selection {
        keys: vec![second, key],
    };
    let ctx = egui::Context::default();
    let menu_frame = |state: &mut AppState, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(650.0, 500.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                clipboard_menu(
                    ui,
                    state,
                    anim,
                    &selection,
                    &crate::registry::Registry::with_builtins(),
                )
            },
        )
    };
    let out = menu_frame(&mut state, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == "Copy selected keys" => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .unwrap();
    menu_frame(&mut state, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        menu_frame(
            &mut state,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    let ankhimate_document::clipboard::Clipboard::Keys(clip) = &state.session.clipboard else {
        panic!()
    };
    assert!(matches!(
        clip.keys[0].track.kind,
        ankhimate_document::clipboard::keys::TrackKind::Bone(BoneProperty::Rotate, None)
    ));
    assert!(matches!(
        clip.keys[1].track.kind,
        ankhimate_document::clipboard::keys::TrackKind::Bone(
            BoneProperty::Translate,
            Some(ankhimate_core::animation::Axis::Y)
        )
    ));
    let before = json(&state);
    let depth = state.history.undo_depth();
    let rev = state.revision;
    let args = crate::key_clipboard::selection_args(&state, anim, &selection.keys[..1]).unwrap();
    crate::key_clipboard::paste(&mut state, &args);
    assert!(state.session.status.as_ref().unwrap().contains("row count"));
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    assert_eq!(state.revision, rev);
}

#[test]
fn malformed_owned_payload_and_setup_mode_refuse_without_document_history_or_preview_changes() {
    let (mut state, anim, key) = fixture();
    let args = crate::key_clipboard::selection_args(&state, anim, &[key]).unwrap();
    crate::key_clipboard::copy(&mut state, &args);
    let ankhimate_document::clipboard::Clipboard::Keys(clip) = &mut state.session.clipboard else {
        panic!()
    };
    clip.keys[0].key.value = ankhimate_document::clipboard::keys::KeyData::Color([1.0; 4]);
    let before = json(&state);
    let depth = state.history.undo_depth();
    let rev = state.revision;
    crate::key_clipboard::paste(&mut state, &args);
    assert!(
        state
            .session
            .status
            .as_ref()
            .unwrap()
            .contains("not representable")
    );
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
    assert_eq!(state.revision, rev);
    state.set_work_mode(crate::session::WorkMode::Setup);
    crate::key_clipboard::paste(&mut state, &args);
    assert!(
        state
            .session
            .status
            .as_ref()
            .unwrap()
            .contains("Animate mode")
    );
    assert_eq!(json(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
}

#[test]
fn key_paste_and_history_keep_explicit_pending_rotation_and_slot_colour() {
    let (mut state, anim, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    let slot = state
        .doc
        .skeleton
        .add_slot(ankhimate_core::slot::Slot::new("art".into(), bone));
    state.doc.animations[anim]
        .timelines
        .push(Timeline::SlotColor {
            slot,
            keys: vec![Key::linear(0.25, [0.4; 4])],
        });
    let selection = vec![
        key,
        KeyRef {
            addr: TimelineAddr::SlotColor { slot },
            index: 0,
        },
    ];
    let args = crate::key_clipboard::selection_args(&state, anim, &selection).unwrap();
    crate::key_clipboard::copy(&mut state, &args);
    state.set_playhead(1.0);
    state.session.auto_key = false;
    let mut pending = state.pose.locals[bone];
    pending.rotation = 1.8;
    state.commit_bone_pose(bone, pending);
    state.commit_slot_color(slot, [0.1, 0.2, 0.3, 0.25]);
    let before = json(&state);
    crate::key_clipboard::paste(&mut state, &args);
    let after = json(&state);
    assert_ne!(before, after);
    for _ in 0..3 {
        assert_eq!(state.session.preview_locals[bone].rotation, 1.8);
        assert_eq!(state.pose.slot_colors[slot], [0.1, 0.2, 0.3, 0.25]);
        state.undo();
        assert_eq!(json(&state), before);
        assert_eq!(state.session.preview_locals[bone].rotation, 1.8);
        assert_eq!(state.pose.slot_colors[slot], [0.1, 0.2, 0.3, 0.25]);
        state.redo();
        assert_eq!(json(&state), after);
    }
}

#[test]
fn clipboard_menu_reaches_the_registered_operator_including_a_shadow() {
    struct Shadow;
    impl crate::registry::Operator for Shadow {
        fn id(&self) -> &'static str {
            "anim.copy_selected_keys"
        }
        fn label(&self) -> &str {
            "shadow"
        }
        fn invoke(
            &self,
            state: &mut AppState,
            _: &ankhimate_document::Args,
        ) -> Result<crate::registry::OpResult, ankhimate_document::ArgError> {
            state.session.set_status("registered shadow reached");
            Ok(crate::registry::OpResult::done())
        }
    }
    let (mut state, anim, key) = fixture();
    let mut operators = crate::registry::Registry::with_builtins();
    operators.register(Box::new(Shadow));
    let ctx = egui::Context::default();
    let draw = |state: &mut AppState, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(650.0, 500.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                clipboard_menu(
                    ui,
                    state,
                    anim,
                    &Selection {
                        keys: vec![key.clone()],
                    },
                    &operators,
                )
            },
        )
    };
    let out = draw(&mut state, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == "Copy selected keys" => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .unwrap();
    draw(&mut state, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        draw(
            &mut state,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    assert_eq!(
        state.session.status.as_deref(),
        Some("registered shadow reached")
    );
    assert!(state.session.clipboard.is_empty());
}

#[test]
fn existing_four_bone_pose_mirror_keeps_its_numeric_contract_and_reports_undo_granularity() {
    let (mut state, _, key) = fixture();
    let TimelineAddr::Bone { bone, .. } = key.addr else {
        panic!()
    };
    state.set_work_mode(crate::session::WorkMode::Setup);
    let mut bones = vec![bone];
    for i in 1..4 {
        let mut child = state.doc.skeleton.bones[bone].clone();
        child.name = format!("arm_{i}");
        child.parent = bones.last().copied();
        bones.push(state.doc.skeleton.add_bone(child));
    }
    for (i, bone) in bones.iter().enumerate() {
        state.doc.skeleton.bones[*bone].local_transform = Transform {
            position: glam::vec2(10.0 * (i + 1) as f32, 7.0),
            rotation: 0.1 * (i + 1) as f32,
            scale: glam::vec2(2.0, 3.0),
            shear: glam::vec2(0.2, -0.1),
        };
    }
    state.refresh_pose();
    state.session.selected_bones = bones.clone();
    state.copy_pose();
    let before = json(&state);
    let depth = state.history.undo_depth();
    state.paste(true);
    let steps = state.history.undo_depth() - depth;
    eprintln!("existing four-bone pose paste history steps: {steps}");
    assert_eq!(steps, 1, "a mirrored pose paste is one Undo");
    for (i, bone) in bones.iter().enumerate() {
        let local = state.doc.skeleton.bones[*bone].local_transform;
        assert_eq!(local.position, glam::vec2(-10.0 * (i + 1) as f32, 7.0));
        assert!((local.rotation + 0.1 * (i + 1) as f32).abs() < 1e-6);
        assert_eq!(local.scale, glam::vec2(2.0, 3.0));
        assert_eq!(local.shear, glam::vec2(-0.2, 0.1));
    }
    let after = json(&state);
    for _ in 0..steps {
        state.undo();
    }
    assert_eq!(json(&state), before);
    for _ in 0..steps {
        state.redo();
    }
    assert_eq!(json(&state), after);
}

#[path = "mirrored_clipboard_tests.rs"]
mod mirrored_offsets_tests;
