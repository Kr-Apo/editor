//! T-807: native data, synthetic assets and HTTP mocks only; no GUI/model.
use super::*;
use ankhimate_core::{
    attachment::{Attachment, Rect, RegionAttachment},
    slot::Slot,
};
fn fixture() -> AppState {
    let mut s = state();
    let root = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "root")
        .unwrap()
        .0;
    let torso = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    s.doc.skeleton.bones[torso].local_transform.position = glam::vec2(25.0, 30.0);
    s.doc.skeleton.bones[torso].local_transform.rotation = 20f32.to_radians();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        8,
        8,
        image::Rgba([20, 160, 220, 255]),
    ))
    .write_to(&mut bytes, image::ImageFormat::Png)
    .unwrap();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "art",
        bytes.into_inner(),
        8,
        8,
    ));
    for (name, bone) in [("accessory", root), ("costume", torso)] {
        let mut slot = Slot::new(name.into(), bone);
        slot.attachment = Some("art".into());
        let id = s.doc.skeleton.add_slot(slot);
        let skin = s.doc.skeleton.default_skin;
        s.doc.skeleton.skins[skin].set(
            id,
            "art",
            Attachment::Region(RegionAttachment {
                texture: "art".into(),
                local_offset: glam::vec2(2.0, 3.0),
                local_rotation: 10f32.to_radians(),
                local_scale: glam::Vec2::ONE,
                width: 10.0,
                height: 8.0,
                uv_rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                },
                pivot: glam::vec2(0.3, 0.4),
                sequence: None,
            }),
        );
    }
    s.refresh_pose();
    s
}
fn transform_plan() -> Value {
    json!({"summary":"Place the existing accessory region","actions":[{"op":"attachment.set_region","skin":"default","slot":"accessory","attachment":"art","x":4,"y":5,"rotation":15,"scale_x":1.1,"scale_y":0.9}]})
}
#[test]
fn assembly_tool_and_region_transform_are_available() {
    assert!(
        tools::definitions(true)
            .iter()
            .any(|d| d["name"] == "ankhimate_assemble"),
        "Native assembly tool missing"
    );
    let mut s = fixture();
    let p: tools::EditPlan =
        serde_json::from_value(transform_plan()).expect("Typed region operation missing");
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
}
#[test]
fn attachment_metadata_exposes_local_world_bone_and_graphic_facts() {
    let s = fixture();
    let data = tools::context(&s);
    let a = data["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["slot"] == "accessory")
        .unwrap();
    assert_eq!(a["bone"], "root");
    assert!(a["local"].is_object());
    assert!(a["setup_world_corners"].is_array());
    assert!(a["graphic"].is_object());
    assert!(data["slots"].is_array());
}

fn plan(v: Value) -> tools::EditPlan {
    serde_json::from_value(v).unwrap()
}
fn slot(s: &AppState, name: &str) -> ankhimate_core::ids::SlotId {
    s.doc
        .skeleton
        .slots
        .iter()
        .find(|(_, x)| x.name == name)
        .unwrap()
        .0
}
fn bone(s: &AppState, name: &str) -> ankhimate_core::ids::BoneId {
    s.doc
        .skeleton
        .bones
        .iter()
        .find(|(_, x)| x.name == name)
        .unwrap()
        .0
}
fn region(s: &AppState) -> &RegionAttachment {
    match s.doc.skeleton.skins[s.doc.skeleton.default_skin]
        .get(slot(s, "accessory"), "art")
        .unwrap()
    {
        Attachment::Region(r) => r,
        _ => panic!(),
    }
}
fn associate(target: &str) -> Value {
    json!({"summary":"Bind one region preserving FK Setup corners", "actions":[{"op":"slot.set_bone","skin":"default","slot":"accessory","attachment":"art","bone":target,"keep_world":true}]})
}
fn combined() -> tools::EditPlan {
    let mut v = associate("handle");
    v["actions"].as_array_mut().unwrap().insert(0,json!({"op":"bone.create","name":"handle","parent":"torso","position":[8,2],"rotation_degrees":30,"length":6}));
    v["actions"].as_array_mut().unwrap().insert(1,json!({"op":"bone.setup","bone":"handle","position":[9,3],"rotation_degrees":25,"scale":[1.2,1.2]}));
    v["actions"]
        .as_array_mut()
        .unwrap()
        .push(transform_plan()["actions"][0].clone());
    v["actions"]
        .as_array_mut()
        .unwrap()
        .push(json!({"op":"slot.set_draw_order","slots":["costume","accessory"]}));
    plan(v)
}
fn rejected(s: &AppState, p: &tools::EditPlan, code: &str) {
    let before = snapshot(s);
    let redo = s.history.redo_label();
    let e = tools::validate(&tools::snapshot(s), p)
        .err()
        .expect("unsafe plan accepted");
    assert_eq!(e.code, code, "{e}");
    assert!(!e.document_changed);
    assert!(e.action_index.is_some() || code == "setup_required");
    assert_eq!(snapshot(s), before);
    assert_eq!(s.history.redo_label(), redo);
}
#[test]
fn region_transform_preserves_image_pivot_uv_dimensions_and_other_entries() {
    let mut s = fixture();
    let original = serde_json::to_value(region(&s)).unwrap();
    let other = snapshot(&s)["skins"].clone();
    let edit = tools::prepare(&s, &plan(transform_plan())).unwrap();
    assert_eq!(
        edit.review[0]["before"]["local"]["position"],
        json!([2.0, 3.0])
    );
    assert_eq!(
        edit.review[0]["after"]["local"]["position"],
        json!([4.0, 5.0])
    );
    edit.approve(&mut s).unwrap();
    let r = region(&s);
    assert_eq!(r.local_offset, glam::vec2(4., 5.));
    assert!((r.local_rotation.to_degrees() - 15.).abs() < 1e-5);
    assert_eq!(r.local_scale, glam::vec2(1.1, 0.9));
    let mut result = serde_json::to_value(r).unwrap();
    for f in ["local_offset", "local_rotation", "local_scale"] {
        result[f] = original[f].clone();
    }
    assert_eq!(result, original);
    s.undo();
    assert_eq!(snapshot(&s)["skins"], other);
}
#[test]
fn association_preserves_all_four_world_corners_for_representable_frames() {
    for scale in [
        glam::vec2(1., 1.),
        glam::vec2(1.5, 1.5),
        glam::vec2(-1., 1.),
    ] {
        let mut s = fixture();
        let target = bone(&s, "torso");
        s.doc.skeleton.bones[target].local_transform.scale = scale;
        s.refresh_pose();
        let id = slot(&s, "accessory");
        let before = assembly::corners(&s.doc.skeleton, id, region(&s));
        let edit = tools::prepare(&s, &plan(associate("torso"))).unwrap();
        assert_eq!(edit.review[0]["before"]["bone"], "root");
        assert_eq!(edit.review[0]["after"]["bone"], "torso");
        edit.approve(&mut s).unwrap();
        for (a, b) in assembly::corners(&s.doc.skeleton, id, region(&s))
            .into_iter()
            .zip(before)
        {
            assert!(
                a.distance(b) < assembly::WORLD_TOLERANCE,
                "{scale:?}: {a:?} {b:?}"
            );
        }
    }
}
#[test]
fn combined_batch_has_one_undo_three_cycles_save_reload_and_preserves_assets_clips() {
    let mut s = fixture();
    tools::prepare(&s, &breathing("torso"))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    s.set_work_mode(crate::session::WorkMode::Setup);
    let before = snapshot(&s);
    let animations = rig::value(&s.doc.animations);
    let assets = s
        .doc
        .assets
        .images
        .values()
        .map(|a| a.bytes.clone())
        .collect::<Vec<_>>();
    tools::prepare(&s, &combined())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let after = snapshot(&s);
    assert_eq!(s.history.undo_label(), Some("AI Assembly Edit"));
    assert_eq!(rig::value(&s.doc.animations), animations);
    assert_eq!(
        s.doc.skeleton.slots[slot(&s, "accessory")].bone,
        bone(&s, "handle")
    );
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), after);
        assert_eq!(
            s.doc.skeleton.slots[slot(&s, "accessory")].bone,
            bone(&s, "handle")
        );
    }
    assert_eq!(
        s.doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>(),
        assets
    );
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Assemblage-épreuve.ankh");
    assert!(matches!(
        crate::fileops::write_to(&s, &file),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut re = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut re, &file),
        crate::fileops::FileOutcome::Opened(_)
    ));
    assert_eq!(snapshot(&re), after);
    assert_eq!(
        re.doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>(),
        assets
    );
}
#[test]
fn assembly_and_later_animation_survive_recreated_bone_ids() {
    let mut s = fixture();
    tools::prepare(&s, &combined())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let assembled = snapshot(&s);
    tools::prepare(&s, &breathing("handle"))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let animated = snapshot(&s);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), assembled);
        s.undo();
        assert!(!s.doc.skeleton.bones.values().any(|b| b.name == "handle"));
        s.redo();
        assert_eq!(snapshot(&s), assembled);
        s.redo();
        assert_eq!(snapshot(&s), animated);
    }
}
#[test]
fn shared_slot_association_is_refused_but_exact_owned_region_transform_is_scoped() {
    for alternate_skin in [false, true] {
        let mut s = fixture();
        let id = slot(&s, "accessory");
        let r = region(&s).clone();
        let skin = if alternate_skin {
            s.doc
                .skeleton
                .skins
                .insert(ankhimate_core::skin::Skin::new("royal"))
        } else {
            s.doc.skeleton.default_skin
        };
        let name = if alternate_skin { "art" } else { "other" };
        s.doc.skeleton.skins[skin].set(id, name, Attachment::Region(r));
        rejected(&s, &plan(associate("torso")), "shared_slot");
        let untouched = serde_json::to_value(s.doc.skeleton.skins[skin].get(id, name)).unwrap();
        tools::prepare(&s, &plan(transform_plan()))
            .unwrap()
            .approve(&mut s)
            .unwrap();
        assert_eq!(
            serde_json::to_value(s.doc.skeleton.skins[skin].get(id, name)).unwrap(),
            untouched
        );
    }
    let mut s = fixture();
    let skin = s
        .doc
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("royal"));
    s.session.active_skin = skin;
    s.refresh_pose();
    let row = tools::context(&s)["slots"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["slot"] == "accessory")
        .unwrap()
        .clone();
    assert_eq!(row["uses_default_fallback"], true);
    let mut v = transform_plan();
    v["actions"][0]["skin"] = json!("royal");
    rejected(&s, &plan(v), "attachment_not_owned");
}
#[test]
fn weighted_linked_mesh_and_other_geometry_remain_read_only_and_intact() {
    use ankhimate_core::attachment::{ClippingAttachment, MeshAttachment, VertexWeight};
    let mut s = fixture();
    let id = slot(&s, "accessory");
    let skin = s.doc.skeleton.default_skin;
    let root = bone(&s, "root");
    let mut mesh = MeshAttachment::from_region(region(&s));
    mesh.weights = vec![
        vec![VertexWeight {
            bone: root,
            weight: 1.
        }];
        4
    ];
    mesh.inverse_bind_matrices
        .insert(root, s.doc.skeleton.setup_world(root).invert().unwrap());
    let binding = mesh.inverse_bind_matrices.clone();
    let raw = serde_json::to_value(&mesh).unwrap();
    s.doc.skeleton.skins[skin].set(id, "art", Attachment::Mesh(mesh));
    rejected(&s, &plan(transform_plan()), "attachment_read_only");
    rejected(&s, &plan(associate("torso")), "attachment_read_only");
    let a = tools::context(&s)["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["slot"] == "accessory")
        .unwrap()
        .clone();
    assert_eq!(a["editable"], false);
    assert_eq!(a["mesh"]["weighted"], true);
    if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin].get(id, "art") {
        assert_eq!(serde_json::to_value(m).unwrap(), raw);
        for (b, matrix) in binding {
            assert_eq!(m.inverse_bind_matrices[&b], matrix);
        }
    }
    s.doc.skeleton.skins[skin].set(
        id,
        "art",
        Attachment::Clipping(ClippingAttachment {
            vertices: vec![glam::Vec2::ZERO; 3],
            end_slot: None,
        }),
    );
    rejected(&s, &plan(transform_plan()), "attachment_read_only");
}
#[test]
fn unsafe_conversions_are_rejected_before_any_publication() {
    let mut s = fixture();
    let target = bone(&s, "torso");
    s.doc.skeleton.bones[target].local_transform.scale = glam::vec2(2., 1.);
    rejected(&s, &plan(associate("torso")), "nonrepresentable_conversion");
    s.doc.skeleton.bones[target].local_transform.scale = glam::vec2(0., 1.);
    rejected(&s, &plan(associate("torso")), "singular_conversion");
    s.doc.skeleton.bones[target].local_transform.scale = glam::Vec2::ONE;
    s.doc.skeleton.skins[s.doc.skeleton.default_skin]
        .bones
        .push(target);
    rejected(&s, &plan(associate("torso")), "skin_bone_association");
    let mut v = associate("torso");
    v["actions"][0]["keep_world"] = json!(false);
    rejected(&s, &plan(v), "keep_world_required");
}
#[test]
fn missing_ambiguous_locked_and_nonfinite_operations_preserve_redo() {
    let mut s = fixture();
    s.dispatch(Box::new(CreateAnimation::new("keep-redo", 1.)));
    s.undo();
    for (field, value, code) in [
        ("skin", json!("absent"), "skin_not_unique"),
        ("slot", json!("absent"), "slot_not_unique"),
        ("attachment", json!("absent"), "attachment_not_owned"),
        ("scale_x", json!(0), "invalid_region_transform"),
        ("x", json!(1e7), "invalid_region_transform"),
        ("rotation", json!(400000), "invalid_region_transform"),
    ] {
        let mut v = transform_plan();
        v["actions"][0][field] = value;
        rejected(&s, &plan(v), code);
    }
    rejected(&s, &plan(associate("missing")), "bone_not_unique");
    let mut v = combined();
    v.actions.push(plan(associate("missing")).actions.remove(0));
    rejected(&s, &v, "bone_not_unique");
    for n in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut p = plan(transform_plan());
        if let tools::Action::RegionTransform { x, .. } = &mut p.actions[0] {
            *x = n;
        }
        rejected(&s, &p, "invalid_region_transform");
    }
    let target = bone(&s, "torso");
    s.session.locked_bones.insert(target, true);
    rejected(&s, &plan(associate("torso")), "bone_locked");
    s.session.locked_bones.clear();
    let root = bone(&s, "root");
    s.session.locked_bones.insert(root, true);
    rejected(&s, &plan(transform_plan()), "bone_locked");
    s.session.locked_bones.clear();
    let b = s.doc.skeleton.bones[target].clone();
    s.doc.skeleton.bones.insert(b);
    s.doc.skeleton.rebuild_update_order();
    rejected(&s, &plan(associate("torso")), "bone_not_unique");
}
#[test]
fn reorder_is_complete_deterministic_and_respects_locks_shared_slots_and_clipping() {
    let order = plan(
        json!({"summary":"Explicit Setup order", "actions":[{"op":"slot.set_draw_order","slots":["costume","accessory"]}]}),
    );
    let mut s = fixture();
    let before = snapshot(&s);
    tools::prepare(&s, &order).unwrap().approve(&mut s).unwrap();
    assert_eq!(
        s.doc.skeleton.draw_order,
        vec![slot(&s, "costume"), slot(&s, "accessory")]
    );
    s.undo();
    assert_eq!(snapshot(&s), before);
    for (slots, code) in [
        (json!(["accessory"]), "invalid_draw_order"),
        (json!(["costume", "costume"]), "duplicate_slot"),
        (json!(["costume", "missing"]), "slot_not_unique"),
    ] {
        let mut p = serde_json::to_value(&order).unwrap();
        p["actions"][0]["slots"] = slots;
        rejected(&s, &plan(p), code);
    }
    let torso = bone(&s, "torso");
    s.session.locked_bones.insert(torso, true);
    rejected(&s, &order, "bone_locked");
    s.session.locked_bones.clear();
    let id = slot(&s, "costume");
    let sk = s.doc.skeleton.default_skin;
    let a = s.doc.skeleton.skins[sk].get(id, "art").unwrap().clone();
    s.doc.skeleton.skins[sk].set(id, "other", a);
    rejected(&s, &order, "shared_or_nonregion_slot");
    s.doc.skeleton.skins[sk]
        .entries
        .remove(&(id, "other".into()));
    s.doc.skeleton.skins[sk].set(
        id,
        "art",
        Attachment::Clipping(ankhimate_core::attachment::ClippingAttachment {
            vertices: vec![],
            end_slot: None,
        }),
    );
    rejected(&s, &order, "clipping_order_protected");
}
#[test]
fn approval_revalidates_locks_revision_and_setup_and_refusal_is_noop() {
    let mut s = fixture();
    let before = snapshot(&s);
    let edit = tools::prepare(&s, &combined()).unwrap();
    drop(edit);
    assert_eq!(snapshot(&s), before);
    let edit = tools::prepare(&s, &combined()).unwrap();
    let root = bone(&s, "root");
    s.session.locked_bones.insert(root, true);
    assert!(edit.approve(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    s.session.locked_bones.clear();
    let edit = tools::prepare(&s, &combined()).unwrap();
    s.dispatch(Box::new(CreateAnimation::new("newer", 1.)));
    let newer = snapshot(&s);
    assert!(edit.approve(&mut s).is_err());
    assert_eq!(snapshot(&s), newer);
    s.set_work_mode(crate::session::WorkMode::Animate);
    rejected(&s, &combined(), "setup_required");
    s.set_work_mode(crate::session::WorkMode::Setup);
    let mut mixed = combined();
    mixed.actions.push(breathing("torso").actions.remove(0));
    rejected(&s, &mixed, "setup_required");
    let mut p = Panel {
        pending: Some(Pending {
            edit: tools::prepare(&s, &combined()).unwrap(),
            ids: vec!["assembly-call".into()],
            proposal_id: "proposal-assembly".into(),
        }),
        ..Default::default()
    };
    p.resolve(&mut s, false).unwrap();
    assert_eq!(snapshot(&s), newer);
    assert!(p.pending.is_none());
    assert!(p.resolve(&mut s, true).is_err());
}
#[test]
fn inventory_is_bounded_deterministic_redacts_paths_and_contains_no_asset_bytes() {
    let mut s = fixture();
    s.doc.assets.images.values_mut().next().unwrap().source_path =
        Some("/private/source.png".into());
    let d = tools::context(&s);
    assert_eq!(tools::context(&s), d);
    let text = d.to_string();
    assert!(
        !text.contains("source_path") && !text.contains("/private/") && !text.contains("bytes")
    );
    let id = slot(&s, "accessory");
    let sk = s.doc.skeleton.default_skin;
    if let Some(Attachment::Region(r)) = s.doc.skeleton.skins[sk]
        .entries
        .get_mut(&(id, "art".into()))
    {
        r.texture = "/private/image.png".into();
    }
    let d = tools::context(&s);
    assert!(!d.to_string().contains("/private/"));
    let a = d["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["slot"] == "accessory")
        .unwrap();
    assert_eq!(a["graphic"]["reference_redacted"], true);
}
#[test]
fn mocked_read_then_assembly_returns_a_validated_proposal_not_a_write() {
    let s = fixture();
    let before = snapshot(&s);
    let (base, rx) = server(vec![
        (
            200,
            native_call("ankhimate_read", json!({"section":"slots"})),
            Duration::ZERO,
        ),
        (
            200,
            native_call(
                "ankhimate_assemble",
                serde_json::to_value(combined()).unwrap(),
            ),
            Duration::ZERO,
        ),
    ]);
    let turn = wait(transport::start_at(
        Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        key(),
        vec![Message::text("user", "Assemble existing region")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert_eq!(turn.plan.unwrap().actions.len(), 5);
    assert_eq!(snapshot(&s), before);
    let first = rx.recv().unwrap();
    assert!(first["tools"].to_string().contains("ankhimate_assemble"));
    let second = rx.recv().unwrap();
    assert!(second.to_string().contains("entry_count_all_skins"));
    assert!(!second.to_string().contains("data:image"));
}
#[test]
fn mocked_unsafe_association_is_corrected_with_structured_error_in_same_budget() {
    let mut s = fixture();
    let id = slot(&s, "accessory");
    let sk = s.doc.skeleton.default_skin;
    let r = region(&s).clone();
    s.doc.skeleton.skins[sk].set(id, "other", Attachment::Region(r));
    let before = snapshot(&s);
    let (base, rx) = server(vec![
        (
            200,
            native_call("ankhimate_assemble", associate("torso")),
            Duration::ZERO,
        ),
        (
            200,
            native_call("ankhimate_assemble", transform_plan()),
            Duration::ZERO,
        ),
    ]);
    let turn = wait(transport::start_at(
        Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        key(),
        vec![Message::text("user", "Place region safely")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert!(matches!(
        turn.plan.unwrap().actions[0],
        tools::Action::RegionTransform { .. }
    ));
    rx.recv().unwrap();
    let request = rx.recv().unwrap().to_string();
    assert!(request.contains("shared_slot") && request.contains("document_changed"));
    assert_eq!(snapshot(&s), before);
}

#[test]
fn assembly_manual_examples_match_typed_schemas_and_budget_with_vision() {
    let g = manual::select(
        "Observe ce personnage et assemble son accessoire region",
        true,
    );
    assert!(g.sections.contains(&"ASSEMBLY"));
    assert!(g.sections.contains(&"VISION"));
    assert!(g.text.len() <= manual::MAX_BYTES);
    let rows: Vec<Value> =
        serde_json::from_str(include_str!("../../../docs/ai/assembly-examples.json")).unwrap();
    let definition = assembly::definition();
    let variants = definition["parameters"]["properties"]["actions"]["items"]["oneOf"]
        .as_array()
        .unwrap();
    for row in rows {
        let p = plan(row["plan"].clone());
        for a in &p.actions {
            let a = serde_json::to_value(a).unwrap();
            let schema = variants
                .iter()
                .find(|v| v["properties"]["op"]["const"] == a["op"])
                .unwrap();
            assert_eq!(
                a.as_object().unwrap().len(),
                schema["required"].as_array().unwrap().len()
            );
            for field in schema["required"].as_array().unwrap() {
                assert!(a.get(field.as_str().unwrap()).is_some());
            }
        }
        let mut s = fixture();
        let before = snapshot(&s);
        let edit = tools::prepare(&s, &p);
        assert_eq!(edit.is_ok(), row["valid"].as_bool().unwrap());
        if let Ok(edit) = edit {
            edit.approve(&mut s).unwrap();
            s.undo();
            assert_eq!(snapshot(&s), before);
        }
    }
}
#[test]
fn association_refuses_constrained_rigs_and_linked_mesh() {
    let mut s = fixture();
    let root = bone(&s, "root");
    let target = bone(&s, "torso");
    s.doc
        .skeleton
        .add_constraint(ankhimate_core::constraints::Constraint::Ik(
            ankhimate_core::constraints::IkConstraint::aim("aim", target, root),
        ));
    rejected(&s, &plan(associate("torso")), "constrained_association");
    let mut s = fixture();
    let id = slot(&s, "accessory");
    let sk = s.doc.skeleton.default_skin;
    let mut m = ankhimate_core::attachment::MeshAttachment::from_region(region(&s));
    m.linked = Some(ankhimate_core::attachment::LinkedMesh {
        skin: Some("default".into()),
        slot: "costume".into(),
        attachment: "art".into(),
        inherit_deform: true,
    });
    s.doc.skeleton.skins[sk].set(id, "art", Attachment::Mesh(m));
    rejected(&s, &plan(associate("torso")), "attachment_read_only");
}

#[test]
fn metadata_caps_counts_and_mode_editability_are_truthful() {
    let mut s = fixture();
    let root = bone(&s, "root");
    let sk = s.doc.skeleton.default_skin;
    let art = region(&s).clone();
    for i in 0..260 {
        let id = s
            .doc
            .skeleton
            .add_slot(Slot::new(format!("extra-{i}"), root));
        s.doc.skeleton.skins[sk].set(id, "art", Attachment::Region(art.clone()));
    }
    s.refresh_pose();
    let d = tools::context(&s);
    assert_eq!(d["slots"].as_array().unwrap().len(), 256);
    assert_eq!(d["attachments"].as_array().unwrap().len(), 256);
    assert_eq!(d["counts"]["slots"], 262);
    assert_eq!(d["counts"]["attachments"], 262);
    s.set_work_mode(crate::session::WorkMode::Animate);
    assert!(
        tools::context(&s)["attachments"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["editable"] == false)
    );
}
#[test]
fn ambiguous_slot_skin_and_empty_destination_are_not_guessed() {
    let mut s = fixture();
    let old = slot(&s, "accessory");
    let copy = s.doc.skeleton.slots[old].clone();
    s.doc.skeleton.slots.insert(copy);
    rejected(&s, &plan(transform_plan()), "slot_not_unique");
    let mut s = fixture();
    s.doc
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("default"));
    rejected(&s, &plan(transform_plan()), "skin_not_unique");
    rejected(&fixture(), &plan(associate("")), "bone_not_unique");
}
#[test]
fn region_sequence_and_native_saved_data_survive_transform_undo_redo() {
    let mut s = fixture();
    let id = slot(&s, "accessory");
    let sk = s.doc.skeleton.default_skin;
    if let Some(Attachment::Region(r)) = s.doc.skeleton.skins[sk]
        .entries
        .get_mut(&(id, "art".into()))
    {
        r.sequence = Some(ankhimate_core::attachment::Sequence {
            frames: vec!["art".into(), "art".into()],
            fps: 12.,
            mode: ankhimate_core::attachment::SequenceMode::Loop,
            setup_index: 1,
        });
    }
    let sequence = serde_json::to_value(&region(&s).sequence).unwrap();
    let before = snapshot(&s);
    tools::prepare(&s, &plan(transform_plan()))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let after = snapshot(&s);
    assert_eq!(
        serde_json::to_value(&region(&s).sequence).unwrap(),
        sequence
    );
    s.undo();
    assert_eq!(snapshot(&s), before);
    s.redo();
    assert_eq!(snapshot(&s), after);
}
#[test]
fn real_panel_approval_applies_exactly_once_and_refusal_preserves_redo() {
    let mut s = fixture();
    s.dispatch(Box::new(CreateAnimation::new("keep-redo", 1.)));
    s.undo();
    let redo = s.history.redo_label().map(str::to_owned);
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
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &combined()).unwrap(),
        ids: vec!["c1".into()],
        proposal_id: "a1".into(),
    });
    p.resolve(&mut s, false).unwrap();
    assert_eq!(snapshot(&s), before);
    assert_eq!(s.history.redo_label(), redo.as_deref());
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &combined()).unwrap(),
        ids: vec!["c2".into()],
        proposal_id: "a2".into(),
    });
    p.resolve(&mut s, true).unwrap();
    let after = snapshot(&s);
    assert_eq!(p.last_result.as_ref().unwrap()["status"], "applied");
    assert!(p.resolve(&mut s, true).is_err());
    assert_eq!(snapshot(&s), after);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
