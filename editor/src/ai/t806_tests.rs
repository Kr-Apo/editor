//! T-806 headless regressions: no GUI driver or live inference.
use super::*;

fn bone_plan() -> Value {
    json!({"summary":"Accessory estimate in parent-local setup units", "actions":[{"op":"bone.create","name":"accessory","parent":"torso","position":[10,20],"rotation_degrees":15,"length":8}]})
}
#[test]
fn rigging_tool_is_advertised_and_create_bone_is_typed() {
    assert!(
        tools::definitions(true)
            .iter()
            .any(|d| d["name"] == "ankhimate_rig"),
        "Native rigging tool missing"
    );
    let p: tools::EditPlan =
        serde_json::from_value(bone_plan()).expect("Typed bone creation missing");
    let mut s = state();
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    assert!(s.doc.skeleton.bones.values().any(|b| b.name == "accessory"));
}
#[test]
fn viewport_observation_has_a_consent_request_operation() {
    assert!(
        tools::definitions(false)
            .iter()
            .any(|d| d["name"] == "ankhimate_viewport"),
        "No consent-gated observation request"
    );
}

fn plan(v: Value) -> tools::EditPlan {
    serde_json::from_value(v).unwrap()
}
fn spec() -> vision::FrameSpec {
    vision::FrameSpec {
        rect: egui::Rect::from_min_size(egui::pos2(2.0, 2.0), egui::vec2(4.0, 4.0)),
        pixels_per_point: 1.0,
        revision: 1,
        camera: crate::ui::canvas::camera::Camera2D {
            position: glam::vec2(10.0, 20.0),
            zoom: 2.0,
        },
        playhead: 0.0,
        setup: true,
    }
}
fn preview(route: String, revision: u64) -> vision::Preview {
    let source = egui::ColorImage::filled([8, 8], egui::Color32::GREEN);
    let (image, mut input) = vision::crop(&source, &spec()).unwrap();
    input.try_unknown = true; // T-809: explicit permission to try this synthetic unqualified model.
    vision::Preview {
        image,
        input,
        revision,
        route,
        approved: false,
        created: Instant::now(),
        texture: None,
    }
}
fn poll(p: &mut Panel, s: &AppState) {
    let end = Instant::now() + Duration::from_secs(5);
    while p.job.is_some() {
        p.poll(s);
        assert!(Instant::now() < end);
        std::thread::sleep(Duration::from_millis(2));
    }
}

#[test]
fn rig_batch_is_atomic_preserves_assets_animations_references_and_three_undo_redo_cycles() {
    let mut s = state();
    tools::prepare(&s, &breathing("torso"))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    s.set_work_mode(crate::session::WorkMode::Setup);
    let before = snapshot(&s);
    let animations = rig::value(&s.doc.animations);
    let skins = snapshot(&s)["skins"].clone();
    let slots = rig::value(&s.doc.skeleton.slots);
    let mut v = bone_plan();
    v["actions"].as_array_mut().unwrap().extend([
 json!({"op":"bone.setup","bone":"accessory","position":[12,23],"rotation_degrees":-45,"scale":[1.1,0.9]}),
 json!({"op":"bone.rename","bone":"accessory","name":"sceptre"})]);
    tools::prepare(&s, &plan(v))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let after = snapshot(&s);
    assert_eq!(s.history.undo_label(), Some("AI Rig Edit"));
    assert_eq!(animations, rig::value(&s.doc.animations));
    assert_eq!(slots, rig::value(&s.doc.skeleton.slots));
    assert_eq!(skins, snapshot(&s)["skins"].clone());
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
    let bone = s
        .doc
        .skeleton
        .bones
        .values()
        .find(|b| b.name == "sceptre")
        .unwrap();
    assert_eq!(bone.local_transform.rotation, (-45f32).to_radians());
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Rig.ankh");
    assert!(matches!(
        crate::fileops::write_to(&s, &path),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut reopened = AppState::default();
    crate::fileops::open_path(&mut reopened, &path);
    assert_eq!(snapshot(&reopened), after);
}

#[test]
fn rename_preserves_sample_assets_slots_weighted_references_and_clips_on_save_reload() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
    let mut s = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut s, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    let id = s.doc.skeleton.slots.values().next().unwrap().bone;
    let name = s.doc.skeleton.bones[id].name.clone();
    let animations = rig::value(&s.doc.animations);
    let slots = rig::value(&s.doc.skeleton.slots);
    let skins = snapshot(&s)["skins"].clone();
    let bytes: Vec<_> = s
        .doc
        .assets
        .images
        .values()
        .map(|a| a.bytes.clone())
        .collect();
    assert!(!bytes.is_empty());
    let plan = plan(
        json!({"summary":"Rename while retaining references","actions":[{"op":"bone.rename","bone":name,"name":"renamed_by_agent"}]}),
    );
    tools::prepare(&s, &plan).unwrap().approve(&mut s).unwrap();
    assert_eq!(s.doc.skeleton.bones[id].name, "renamed_by_agent");
    assert_eq!(animations, rig::value(&s.doc.animations));
    assert_eq!(slots, rig::value(&s.doc.skeleton.slots));
    assert_eq!(skins, snapshot(&s)["skins"].clone());
    let after = snapshot(&s);
    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join("Rig.ankh");
    assert!(matches!(
        crate::fileops::write_to(&s, &dest),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut reopened = AppState::default();
    crate::fileops::open_path(&mut reopened, &dest);
    assert_eq!(snapshot(&reopened), after);
    assert_eq!(
        bytes,
        reopened
            .doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn invalid_rig_proposals_preserve_document_and_redo() {
    let mut s = state();
    s.dispatch(Box::new(CreateAnimation::new("existing", 2.0)));
    s.undo();
    let before = snapshot(&s);
    let redo = s.history.redo_label().map(str::to_string);
    for (field, value) in [
        ("parent", json!("missing")),
        ("parent", json!("accessory")),
        ("name", json!("torso")),
        ("length", json!(-1)),
        ("position", json!([1e30, 0])),
    ] {
        let mut v = bone_plan();
        v["actions"][0][field] = value;
        assert!(tools::prepare(&s, &plan(v)).is_err());
        assert_eq!(snapshot(&s), before);
        assert_eq!(s.history.redo_label(), redo.as_deref());
    }
    let mut v = bone_plan();
    v["actions"]
        .as_array_mut()
        .unwrap()
        .push(json!({"op":"bone.rename","bone":"missing","name":"anything"}));
    assert!(tools::prepare(&s, &plan(v)).is_err());
    assert_eq!(snapshot(&s), before);
    let mut p = plan(bone_plan());
    if let tools::Action::BoneCreate {
        rotation_degrees, ..
    } = &mut p.actions[0]
    {
        *rotation_degrees = f32::NAN;
    }
    assert!(tools::prepare(&s, &p).is_err());
    let torso = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    s.session.locked_bones.insert(torso, true);
    assert!(tools::prepare(&s, &plan(bone_plan())).is_err());
    s.session.locked_bones.clear();
    let prepared = tools::prepare(&s, &plan(bone_plan())).unwrap();
    s.session.locked_bones.insert(torso, true);
    assert!(prepared.approve(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}

#[test]
fn rigging_refuses_cycles_modes_reparenting_and_stale_approval() {
    let mut s = state();
    let original = snapshot(&s);
    let edit = tools::prepare(&s, &plan(bone_plan())).unwrap();
    s.dispatch(Box::new(CreateAnimation::new("later", 1.0)));
    assert!(edit.approve(&mut s).is_err());
    s.set_work_mode(crate::session::WorkMode::Animate);
    assert!(tools::prepare(&s, &plan(bone_plan())).is_err());
    s.set_work_mode(crate::session::WorkMode::Setup);
    let ids: Vec<_> = s.doc.skeleton.bones.keys().collect();
    s.doc.skeleton.bones[ids[0]].parent = Some(ids[1]);
    assert_eq!(
        tools::validate(&tools::snapshot(&s), &plan(bone_plan()))
            .err()
            .unwrap()
            .code,
        "invalid_hierarchy"
    );
    assert!(serde_json::from_value::<tools::EditPlan>(json!({"summary":"not supported","actions":[{"op":"bone.reparent","bone":"torso","parent":"root"}]})).is_err());
    assert!(original["bones"].is_array());
}

#[test]
fn rig_approval_refusal_duplicates_and_future_animation_redo_keep_real_ids() {
    let mut s = state();
    let before = snapshot(&s);
    let mut p = Panel {
        settings: Settings {
            editing: true,
            tools_enabled: true,
            ..Default::default()
        },
        share_context: true,
        ..Default::default()
    };
    let pending = |s: &AppState| Pending {
        edit: tools::prepare(s, &plan(bone_plan())).unwrap(),
        ids: vec![],
        proposal_id: "rig-proposal".into(),
    };
    p.pending = Some(pending(&s));
    p.resolve(&mut s, false).unwrap();
    assert_eq!(snapshot(&s), before);
    p.pending = Some(pending(&s));
    p.resolve(&mut s, true).unwrap();
    assert_eq!(p.last_result.as_ref().unwrap()["status"], "applied");
    assert!(p.resolve(&mut s, true).is_err());
    let rigged = snapshot(&s);
    tools::prepare(&s, &breathing("accessory"))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let animated = snapshot(&s);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), rigged);
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), rigged);
        s.redo();
        assert_eq!(snapshot(&s), animated);
    }
}

#[test]
fn crop_never_contains_pixels_outside_canvas_and_mapping_matches_camera() {
    let mut image = egui::ColorImage::filled([8, 8], egui::Color32::MAGENTA);
    for y in 2..6 {
        for x in 2..6 {
            image.pixels[y * 8 + x] = egui::Color32::GREEN;
        }
    }
    let (crop, input) = vision::crop(&image, &spec()).unwrap();
    assert_eq!(crop.size, [4, 4]);
    assert!(crop.pixels.iter().all(|p| *p == egui::Color32::GREEN));
    assert_eq!(
        input.mapping["world_origin_at_image_edge"],
        json!([9.0, 21.0])
    );
    assert_eq!(
        input.mapping["world_units_per_image_pixel"],
        json!([0.5, -0.5])
    );
    use base64::Engine;
    let png = base64::engine::general_purpose::STANDARD
        .decode(input.data_url.split_once(',').unwrap().1)
        .unwrap();
    let decoded = image::load_from_memory(&png).unwrap();
    assert_eq!(decoded.to_rgba8().get_pixel(0, 0).0, [0, 255, 0, 255]);
    let mut sp = spec();
    sp.pixels_per_point = 2.0;
    sp.rect = egui::Rect::from_min_max(egui::pos2(1.1, 1.1), egui::pos2(2.9, 2.9));
    let (c, _) = vision::crop(&image, &sp).unwrap();
    assert_eq!(c.size, [2, 2]);
    assert!(c.pixels.iter().all(|p| *p == egui::Color32::GREEN));
    sp.rect = egui::Rect::from_min_max(egui::pos2(-1.0, 0.0), egui::pos2(10.0, 10.0));
    assert!(vision::crop(&image, &sp).is_err());
}

#[test]
fn image_resolution_size_consent_destination_revision_and_expiry_are_bounded() {
    let mut sp = spec();
    sp.rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2048.0, 1024.0));
    let image = egui::ColorImage::filled([2048, 1024], egui::Color32::WHITE);
    let (c, input) = vision::crop(&image, &sp).unwrap();
    assert_eq!(c.size, [1024, 512]);
    assert!(input.data_url.len() < vision::MAX_BYTES);
    let mut v = vision::Vision::default();
    assert!(
        v.authorized(Provider::NInfer, "route", 1)
            .unwrap()
            .is_none()
    );
    v.preview = Some(preview("route".into(), 1));
    assert!(v.authorized(Provider::NInfer, "route", 1).is_err());
    v.preview.as_mut().unwrap().approved = true;
    assert!(
        v.authorized(Provider::NInfer, "route", 1)
            .unwrap()
            .is_some()
    );
    assert!(
        v.authorized(Provider::OpenAI, "different-destination", 1)
            .is_err()
    );
    assert!(v.authorized(Provider::NInfer, "elsewhere", 1).is_err());
    assert!(v.authorized(Provider::NInfer, "route", 2).is_err());
    v.preview.as_mut().unwrap().created = Instant::now() - Duration::from_secs(121);
    assert!(v.authorized(Provider::NInfer, "route", 1).is_err());
    v.clear();
    assert!(v.preview.is_none());
}

#[test]
fn authorized_image_goes_only_in_current_send_then_never_reappears_in_history() {
    let s = state();
    let (base, rx) = server(vec![
        (
            200,
            text_stream(Provider::NInfer, "Image description"),
            Duration::ZERO,
        ),
        (
            200,
            text_stream(Provider::NInfer, "Text only followup"),
            Duration::ZERO,
        ),
    ]);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            local_url: base,
            model: "vision-mock".into(),
            ..Default::default()
        },
        input: "Observe".into(),
        ..Default::default()
    };
    p.vision.preview = Some(preview(p.image_route(), s.revision));
    assert!(p.start(&s, &egui::Context::default(), false).is_err());
    assert!(rx.try_recv().is_err());
    p.vision.preview.as_mut().unwrap().approved = true;
    p.start(&s, &egui::Context::default(), false).unwrap();
    assert!(p.vision.preview.is_none());
    poll(&mut p, &s);
    let request = rx.recv().unwrap();
    let last = request["messages"].as_array().unwrap().last().unwrap();
    assert_eq!(last["content"][1]["type"], "image_url");
    assert!(
        last["content"][1]["image_url"]["url"]
            .as_str()
            .unwrap()
            .starts_with("data:image/png;base64,")
    );
    assert!(!request.to_string().contains("torso"));
    assert!(p.messages.iter().all(|m| m.image.is_none()));
    p.input = "What next?".into();
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll(&mut p, &s);
    assert!(!rx.recv().unwrap().to_string().contains("data:image"));
}

#[test]
fn unsupported_image_transport_refuses_before_http_and_network_errors_do_not_write() {
    let mut m = Message::text("user", "observe");
    m.image = Some(preview("unused".into(), 0).input);
    let mut settings = Settings {
        provider: Provider::Ollama,
        ..Default::default()
    };
    settings.capabilities.destination = settings.destination();
    settings.capabilities.images = super::super::capabilities::Support::Unsupported;
    let error = wait(transport::start_at(
        settings,
        key(),
        vec![m],
        None,
        false,
        egui::Context::default(),
        "http://127.0.0.1:1".into(),
    ))
    .err()
    .unwrap();
    assert!(error.contains("incompatible"));
    for code in [400, 401, 429] {
        let s = state();
        let before = snapshot(&s);
        let (base, _) = server(vec![(code, "no image support".into(), Duration::ZERO)]);
        let mut p = Panel {
            settings: Settings {
                provider: Provider::NInfer,
                local_url: base,
                model: "mock".into(),
                ..Default::default()
            },
            input: "Observe".into(),
            ..Default::default()
        };
        let mut image = preview(p.image_route(), s.revision);
        image.approved = true;
        p.vision.preview = Some(image);
        p.start(&s, &egui::Context::default(), false).unwrap();
        poll(&mut p, &s);
        assert_eq!(snapshot(&s), before);
        assert!(matches!(p.activity.phase, Phase::Error));
        assert!(p.vision.preview.is_none());
    }
}

#[test]
fn viewport_tool_requests_user_consent_without_automatic_capture() {
    let s = state();
    let (base, rx) = server(vec![
        (
            200,
            super::expert::compatible_call("ankhimate_viewport", json!({})),
            Duration::ZERO,
        ),
        (
            200,
            text_stream(Provider::NInfer, "Please capture and authorize the preview"),
            Duration::ZERO,
        ),
    ]);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            local_url: base,
            model: "mock".into(),
            tools_enabled: true,
            ..Default::default()
        },
        input: "Observe".into(),
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll(&mut p, &s);
    assert!(p.vision.open);
    assert!(!p.vision.requested && p.vision.preview.is_none());
    assert!(!rx.recv().unwrap().to_string().contains("data:image"));
    assert!(!rx.recv().unwrap().to_string().contains("data:image"));
}

#[test]
fn rig_examples_match_schema_and_native_validation_and_manual_keeps_bounds() {
    let defs = tools::definitions(true);
    let schema = &defs.iter().find(|d| d["name"] == "ankhimate_rig").unwrap()["parameters"]["properties"]
        ["actions"]["items"]["oneOf"];
    let rows: Value =
        serde_json::from_str(include_str!("../../../docs/ai/rig-examples.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let mut s = state();
        let p = plan(row["plan"].clone());
        for action in &p.actions {
            let v = serde_json::to_value(action).unwrap();
            let variant = schema
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["properties"]["op"]["const"] == action.operation())
                .unwrap();
            let mut actual: Vec<_> = v.as_object().unwrap().keys().map(String::as_str).collect();
            actual.sort();
            let mut expected: Vec<_> = variant["required"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            expected.sort();
            assert_eq!(actual, expected);
            assert_eq!(variant["additionalProperties"], false);
        }
        let result = tools::validate(&tools::snapshot(&s), &p);
        if row["expected"] == "valid" {
            result.unwrap().approve(&mut s).unwrap();
        } else {
            assert_eq!(
                result.err().unwrap().code,
                row["expected"].as_str().unwrap()
            );
        }
    }
    let guide = manual::select(
        "Observe mon personnage puis propose un bone parenté au torse",
        true,
    );
    assert!(guide.sections.contains(&"VISION"));
    assert!(guide.sections.contains(&"RIGGING"));
    assert!(guide.text.len() <= manual::MAX_BYTES);
    assert!(guide.text.contains("bone.create"));
}

#[test]
fn mock_model_reads_parent_then_proposes_rig_without_applying_until_approval() {
    let s = state();
    let (base, rx) = server(vec![
        (
            200,
            super::expert::compatible_call(
                "ankhimate_read",
                json!({"section":"bone","bone":"torso"}),
            ),
            Duration::ZERO,
        ),
        (
            200,
            super::expert::compatible_call("ankhimate_rig", bone_plan()),
            Duration::ZERO,
        ),
    ]);
    let before = snapshot(&s);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            local_url: base,
            model: "mock".into(),
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        share_context: true,
        input: "Propose a bone".into(),
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll(&mut p, &s);
    assert!(p.pending.is_some());
    assert_eq!(snapshot(&s), before);
    let first = rx.recv().unwrap();
    assert!(
        first["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["function"]["name"] == "ankhimate_rig")
    );
    let followup = rx.recv().unwrap();
    assert!(followup.to_string().contains("world_current_affine"));
}

#[test]
fn image_request_can_be_cancelled_and_timeout_without_native_edits() {
    for stop in [false, true] {
        let s = state();
        let before = snapshot(&s);
        let (base, rx) = server(vec![(
            200,
            text_stream(Provider::NInfer, "late"),
            Duration::from_millis(1400),
        )]);
        let mut p = Panel {
            settings: Settings {
                provider: Provider::NInfer,
                local_url: base,
                model: "mock".into(),
                timeout_seconds: 1,
                ..Default::default()
            },
            input: "Observe".into(),
            ..Default::default()
        };
        let mut image = preview(p.image_route(), s.revision);
        image.approved = true;
        p.vision.preview = Some(image);
        p.start(&s, &egui::Context::default(), false).unwrap();
        rx.recv_timeout(Duration::from_secs(2)).unwrap();
        if stop {
            p.stop();
        }
        poll(&mut p, &s);
        assert_eq!(snapshot(&s), before);
        assert_eq!(
            p.activity.phase,
            if stop { Phase::Cancelled } else { Phase::Error }
        );
        assert_eq!(p.input, "Observe");
        assert!(p.vision.preview.is_none());
    }
}

#[test]
fn native_rig_undo_respects_remapped_ids_from_later_delete_undo() {
    use ankhimate_document::commands::bone_cmds::DeleteBone;
    let mut s = state();
    let before = snapshot(&s);
    let p = plan(
        json!({"summary":"Setup torso safely","actions":[{"op":"bone.setup","bone":"torso","position":[12,20],"rotation_degrees":5,"scale":[1,1]}]}),
    );
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let torso = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    s.dispatch(Box::new(DeleteBone::new(torso)));
    s.undo();
    assert!(!s.doc.skeleton.bones.contains_key(torso));
    s.undo();
    assert_eq!(snapshot(&s), before);
    s.redo();
    assert_eq!(
        s.doc
            .skeleton
            .bones
            .values()
            .find(|b| b.name == "torso")
            .unwrap()
            .local_transform
            .position,
        glam::vec2(12.0, 20.0)
    );
}

#[test]
fn capture_event_is_bound_to_root_request_and_does_not_authorize_or_transmit() {
    let ctx = egui::Context::default();
    let mut v = vision::Vision::default();
    let sp = spec();
    let output = ctx.run_ui(egui::RawInput::default(), |_| {
        v.begin_capture(&ctx);
        v.canvas(&ctx, sp.clone(), "destination".into());
    });
    let tag = output.viewport_output[&egui::ViewportId::ROOT]
        .commands
        .iter()
        .find_map(|c| {
            if let egui::ViewportCommand::Screenshot(data) = c {
                Some(data.clone())
            } else {
                None
            }
        })
        .unwrap();
    let image = std::sync::Arc::new(egui::ColorImage::filled([8, 8], egui::Color32::WHITE));
    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Screenshot {
                viewport_id: egui::ViewportId::from_hash_of("other-window"),
                user_data: tag.clone(),
                image: image.clone(),
            }],
            ..Default::default()
        },
        |_| v.receive(&ctx, 1),
    );
    assert!(v.preview.is_none());
    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Screenshot {
                viewport_id: egui::ViewportId::ROOT,
                user_data: tag,
                image,
            }],
            ..Default::default()
        },
        |_| v.receive(&ctx, 1),
    );
    assert!(!v.busy());
    let preview = v.preview.as_ref().unwrap();
    assert!(!preview.approved);
    assert_eq!(preview.route, "destination");
    v.cancel_capture();
    assert!(v.preview.is_none());
}
