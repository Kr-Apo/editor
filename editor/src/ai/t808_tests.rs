//! Deterministic T-808 fixtures; no GUI, live inference or rendering.
use super::*;
use ankhimate_core::{
    attachment::{Attachment, MeshAttachment, Rect, RegionAttachment, VertexWeight},
    slot::Slot,
};
fn fixture() -> AppState {
    let mut s = state();
    let root = bone(&s, "root");
    let torso = bone(&s, "torso");
    s.doc.skeleton.bones[torso].local_transform.position = glam::vec2(5., 3.);
    let id = s.doc.skeleton.add_slot(Slot::new("body".into(), root));
    s.doc.skeleton.slots[id].attachment = Some("art".into());
    let skin = s.doc.skeleton.default_skin;
    let mut m = MeshAttachment::from_region(&RegionAttachment {
        texture: "art".into(),
        local_offset: glam::Vec2::ZERO,
        local_rotation: 0.,
        local_scale: glam::Vec2::ONE,
        width: 12.,
        height: 10.,
        uv_rect: Rect {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.,
        },
        pivot: glam::Vec2::splat(0.5),
        sequence: None,
    });
    m.weights = (0..4)
        .map(|i| {
            vec![
                VertexWeight {
                    bone: root,
                    weight: if i % 2 == 0 { 0.8 } else { 0.2 },
                },
                VertexWeight {
                    bone: torso,
                    weight: if i % 2 == 0 { 0.2 } else { 0.8 },
                },
            ]
        })
        .collect();
    s.doc.skeleton.skins[skin].set(id, "art", Attachment::Mesh(m));
    let mut png = std::io::Cursor::new(vec![]);
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        8,
        8,
        image::Rgba([30, 140, 180, 255]),
    ))
    .write_to(&mut png, image::ImageFormat::Png)
    .unwrap();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "art",
        png.into_inner(),
        8,
        8,
    ));
    ankhimate_document::rebind::rebind_meshes(&mut s.doc);
    s.refresh_pose();
    s
}
fn bone(s: &AppState, n: &str) -> ankhimate_core::ids::BoneId {
    s.doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == n)
        .unwrap()
        .0
}
fn slot(s: &AppState) -> ankhimate_core::ids::SlotId {
    s.doc
        .skeleton
        .slots
        .iter()
        .find(|(_, x)| x.name == "body")
        .unwrap()
        .0
}
fn plan() -> tools::EditPlan {
    serde_json::from_value(json!({"summary":"Correct selected existing influences", "actions":[{"op":"mesh.set_weights","skin":"default","slot":"body","attachment":"art","vertices":[{"index":0,"influences":[{"bone":"root","weight":0.6},{"bone":"torso","weight":0.4}]}]}]})).unwrap()
}
#[test]
fn existing_weighted_mesh_can_receive_an_approved_native_weight_correction() {
    assert!(
        tools::definitions(true)
            .iter()
            .any(|v| v["name"] == "ankhimate_weights"),
        "Native weight tool missing"
    );
    let mut s = fixture();
    tools::prepare(&s, &plan())
        .unwrap()
        .approve(&mut s)
        .unwrap();
}
#[test]
fn mesh_detail_is_targeted_not_automatic_geometry() {
    assert!(
        tools::definitions(false)
            .iter()
            .any(|v| v["name"] == "ankhimate_read_mesh"),
        "Paged mesh read tool missing"
    );
    let d = tools::context(&fixture());
    assert!(!d.to_string().contains("setup_vertices"));
}

fn edit(v: Value) -> tools::EditPlan {
    serde_json::from_value(v).unwrap()
}
fn smooth() -> tools::EditPlan {
    edit(
        json!({"summary":"Small native shoulder smoothing","actions":[{"op":"mesh.smooth_weights","skin":"default","slot":"body","attachment":"art","vertices":[0,1],"bone":"root","strength":0.2,"iterations":2}]}),
    )
}
fn normal() -> tools::EditPlan {
    edit(
        json!({"summary":"Normalize exact influences","actions":[{"op":"mesh.normalize_weights","skin":"default","slot":"body","attachment":"art","vertices":[0,1]}]}),
    )
}
fn read(s: &AppState, start: usize, count: usize) -> Result<Value, String> {
    weights::read(
        &tools::snapshot(s),
        &weights::ReadMesh {
            skin: "default".into(),
            slot: "body".into(),
            attachment: "art".into(),
            start,
            count,
        },
    )
}
fn mesh(s: &AppState) -> &MeshAttachment {
    match s.doc.skeleton.skins[s.doc.skeleton.default_skin]
        .get(slot(s), "art")
        .unwrap()
    {
        Attachment::Mesh(m) => m,
        _ => panic!(),
    }
}
fn mesh_mut(s: &mut AppState) -> &mut MeshAttachment {
    let sl = slot(s);
    let sk = s.doc.skeleton.default_skin;
    match s.doc.skeleton.skins[sk]
        .entries
        .get_mut(&(sl, "art".into()))
        .unwrap()
    {
        Attachment::Mesh(m) => m,
        _ => panic!(),
    }
}
fn unchanged(s: &AppState, p: &tools::EditPlan, code: &str) {
    let before = snapshot(s);
    let binds = mesh(s).inverse_bind_matrices.clone();
    let redo = s.history.redo_label();
    let error = tools::validate(&tools::snapshot(s), p)
        .err()
        .expect("Unsafe mesh edit accepted");
    assert_eq!(error.code, code, "{error}");
    assert!(!error.document_changed);
    assert_eq!(snapshot(s), before);
    assert_eq!(mesh(s).inverse_bind_matrices, binds);
    assert_eq!(s.history.redo_label(), redo);
}
#[test]
fn paged_read_reports_coordinates_influences_sums_binds_without_paths_or_images() {
    let mut s = fixture();
    s.doc.assets.images.values_mut().next().unwrap().source_path =
        Some("/private/image-source".into());
    let page = read(&s, 1, 2).unwrap();
    assert_eq!(page["vertices_count"], 4);
    assert_eq!(page["count"], 2);
    assert_eq!(page["next_start"], 3);
    assert_eq!(page["vertices"][0]["index"], 1);
    assert_eq!(page["vertices"][0]["normalized"], true);
    assert_eq!(
        page["vertices"][0]["influences"].as_array().unwrap().len(),
        2
    );
    assert_eq!(page["editable"], true);
    assert!(
        page["binds"]
            .as_array()
            .unwrap()
            .iter()
            .all(|b| b["valid"] == true)
    );
    assert!(!page.to_string().contains("source") && !page.to_string().contains("bytes"));
    assert!(read(&s, 0, 33).is_err());
    assert!(read(&s, 0, 0).is_err());
    assert!(read(&s, 4, 1).is_err());
    assert_eq!(read(&s, 3, 32).unwrap()["next_start"], Value::Null);
}
#[test]
fn edits_normalize_coefficients_and_preserve_order_geometry_ffd_and_exact_bind_bits() {
    let mut s = fixture();
    let mut p = serde_json::to_value(plan()).unwrap();
    p["actions"][0]["vertices"] = json!([{"index":0,"influences":[{"bone":"torso","weight":0.2},{"bone":"root","weight":0.2}]},{"index":1,"influences":[{"bone":"root","weight":0.3},{"bone":"torso","weight":0.7}]}]);
    let original = mesh(&s).clone();
    let review = tools::prepare(&s, &edit(p)).unwrap();
    assert_eq!(
        review.review[0]["before"]["vertices"][0]["influences"][0]["weight"],
        json!(0.8f32)
    );
    assert_eq!(review.review[0]["after"]["vertices"][0]["sum"], json!(1.0));
    review.approve(&mut s).unwrap();
    assert_eq!(mesh(&s).weights[0][0].weight, 0.5);
    assert_eq!(mesh(&s).weights[0][1].weight, 0.5);
    assert_eq!(mesh(&s).weights[1][0].weight, 0.3);
    assert_eq!(
        mesh(&s).inverse_bind_matrices,
        original.inverse_bind_matrices
    );
    assert_eq!(mesh(&s).setup_vertices, original.setup_vertices);
    assert_eq!(mesh(&s).uvs, original.uvs);
    assert_eq!(mesh(&s).triangles, original.triangles);
    assert_eq!(
        mesh(&s).weights[2..]
            .iter()
            .map(|w| serde_json::to_value(w).unwrap())
            .collect::<Vec<_>>(),
        original.weights[2..]
            .iter()
            .map(|w| serde_json::to_value(w).unwrap())
            .collect::<Vec<_>>()
    );
}
#[test]
fn native_smoothing_is_deterministic_bounded_and_preserves_unselected_vertices() {
    let mut a = fixture();
    let mut b = fixture();
    let before = mesh(&a).clone();
    let p = smooth();
    tools::prepare(&a, &p).unwrap().approve(&mut a).unwrap();
    tools::prepare(&b, &p).unwrap().approve(&mut b).unwrap();
    assert_eq!(snapshot(&a), snapshot(&b));
    assert!(mesh(&a).weights[0][0].weight < 0.8);
    assert!(mesh(&a).weights[1][0].weight > 0.2);
    for i in [2, 3] {
        assert_eq!(
            serde_json::to_value(&mesh(&a).weights[i]).unwrap(),
            serde_json::to_value(&before.weights[i]).unwrap()
        );
    }
    assert_eq!(mesh(&a).inverse_bind_matrices, before.inverse_bind_matrices);
    for row in &mesh(&a).weights {
        assert!((row.iter().map(|w| w.weight).sum::<f32>() - 1.).abs() < 1e-6);
    }
}
#[test]
fn normalization_uses_native_rules_without_implicit_influence_pruning() {
    let mut s = fixture();
    mesh_mut(&mut s).weights[0][0].weight = 0.2;
    mesh_mut(&mut s).weights[0][1].weight = 0.2;
    assert_eq!(read(&s, 0, 1).unwrap()["vertices"][0]["normalized"], false);
    tools::prepare(&s, &normal())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    assert_eq!(mesh(&s).weights[0][0].weight, 0.5);
    let mut p = serde_json::to_value(plan()).unwrap();
    p["actions"][0]["vertices"][0]["influences"][0]["weight"] = json!(0.0);
    unchanged(&s, &edit(p), "influence_membership");
}
#[test]
fn invalid_selections_weights_and_influences_are_atomic_and_preserve_redo() {
    let mut s = fixture();
    s.dispatch(Box::new(CreateAnimation::new("redo", 1.)));
    s.undo();
    for vertices in [
        json!([]),
        json!([{"index":100,"influences":[]}]),
        json!([{"index":0,"influences":[]},{"index":0,"influences":[]}]),
    ] {
        let mut p = serde_json::to_value(plan()).unwrap();
        p["actions"][0]["vertices"] = vertices;
        unchanged(&s, &edit(p), "invalid_vertices");
    }
    for (bone, weight, code) in [
        ("root", -0.1, "invalid_weight"),
        ("root", 1.1, "invalid_weight"),
        ("absent", 0.4, "influence_membership"),
        ("torso", 0.4, "invalid_weight"),
    ] {
        let mut p = serde_json::to_value(plan()).unwrap();
        p["actions"][0]["vertices"][0]["influences"][0] = json!({"bone":bone,"weight":weight});
        unchanged(&s, &edit(p), code);
    }
    for number in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut p = plan();
        if let tools::Action::MeshSet { vertices, .. } = &mut p.actions[0] {
            vertices[0].influences[0].weight = number;
        }
        unchanged(&s, &p, "invalid_weight");
    }
    let mut p = serde_json::to_value(plan()).unwrap();
    p["actions"][0]["vertices"][0]["influences"] = json!([{"bone":"root","weight":1}]);
    unchanged(&s, &edit(p), "influence_membership");
    for (field, value) in [
        ("iterations", json!(0)),
        ("iterations", json!(5)),
        ("strength", json!(0)),
        ("strength", json!(1.1)),
    ] {
        let mut p = serde_json::to_value(smooth()).unwrap();
        p["actions"][0][field] = value;
        unchanged(&s, &edit(p), "invalid_smoothing");
    }
    let mut p = serde_json::to_value(smooth()).unwrap();
    p["actions"][0]["bone"] = json!("absent");
    unchanged(&s, &edit(p), "bone_not_unique");
}
#[test]
fn guard_refuses_bad_geometry_binds_limits_skin_scopes_and_constraints() {
    let mut s = fixture();
    mesh_mut(&mut s).triangles[0][0] = 99;
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s).triangles.push([0, 1, 2]);
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s).uvs.pop();
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s).setup_vertices[0] = glam::vec2(f32::NAN, 0.);
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s).inverse_bind_matrices.clear();
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    let root = bone(&s, "root");
    mesh_mut(&mut s)
        .inverse_bind_matrices
        .get_mut(&root)
        .unwrap()
        .a = 0.;
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s)
        .inverse_bind_matrices
        .get_mut(&root)
        .unwrap()
        .tx = 1.;
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    let w = mesh(&s).weights[0][0];
    mesh_mut(&mut s).weights[0] = vec![w; 5];
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    let sk = s.doc.skeleton.default_skin;
    s.doc.skeleton.skins[sk].bones.push(root);
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    let torso = bone(&s, "torso");
    s.doc
        .skeleton
        .add_constraint(ankhimate_core::constraints::Constraint::Ik(
            ankhimate_core::constraints::IkConstraint::aim("aim", torso, root),
        ));
    unchanged(&s, &plan(), "unsafe_mesh");
}
#[test]
fn shared_slots_linked_targets_and_linked_sources_are_read_only() {
    let mut s = fixture();
    let sk = s.doc.skeleton.default_skin;
    let sl = slot(&s);
    let m = mesh(&s).clone();
    s.doc.skeleton.skins[sk].set(sl, "other", Attachment::Mesh(m));
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s).linked = Some(ankhimate_core::attachment::LinkedMesh {
        skin: None,
        slot: "other".into(),
        attachment: "art".into(),
        inherit_deform: true,
    });
    unchanged(&s, &plan(), "unsafe_mesh");
    assert_eq!(read(&s, 0, 1).unwrap()["editable"], false);
    let mut s = fixture();
    let sk = s.doc.skeleton.default_skin;
    let sl = s
        .doc
        .skeleton
        .add_slot(Slot::new("linked".into(), bone(&s, "root")));
    let mut m = mesh(&s).clone();
    m.linked = Some(ankhimate_core::attachment::LinkedMesh {
        skin: None,
        slot: "body".into(),
        attachment: "art".into(),
        inherit_deform: true,
    });
    s.doc.skeleton.skins[sk].set(sl, "art", Attachment::Mesh(m));
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    s.doc
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("default"));
    unchanged(&s, &plan(), "unsafe_mesh");
}
#[test]
fn exact_identity_no_fallback_region_or_rigid_conversion() {
    let mut s = fixture();
    for (field, value) in [
        ("skin", "missing"),
        ("slot", "missing"),
        ("attachment", "missing"),
    ] {
        let mut p = serde_json::to_value(plan()).unwrap();
        p["actions"][0][field] = json!(value);
        unchanged(&s, &edit(p), "unsafe_mesh");
    }
    mesh_mut(&mut s).weights.clear();
    unchanged(&s, &plan(), "unsafe_mesh");
    assert_eq!(read(&s, 0, 1).unwrap()["type"], "rigid_mesh");
}
#[test]
fn locks_late_locks_stale_revision_and_changed_bind_are_revalidated() {
    let mut s = fixture();
    let root = bone(&s, "root");
    let torso = bone(&s, "torso");
    s.session.locked_bones.insert(torso, true);
    unchanged(&s, &plan(), "locked_influence");
    s.session.locked_bones.clear();
    s.session.weight_paint_settings.locked.push(torso);
    unchanged(&s, &plan(), "locked_influence");
    s.session.weight_paint_settings.locked.clear();
    let p = tools::prepare(&s, &plan()).unwrap();
    s.session.weight_paint_settings.locked.push(torso);
    assert!(p.approve(&mut s).is_err());
    s.session.weight_paint_settings.locked.clear();
    let p = tools::prepare(&s, &plan()).unwrap();
    mesh_mut(&mut s)
        .inverse_bind_matrices
        .get_mut(&root)
        .unwrap()
        .tx += 0.00001;
    assert!(
        p.approve(&mut s).is_err(),
        "Even valid-but-different binds must stale the proposal"
    );
    let p = tools::prepare(&s, &plan()).unwrap();
    s.dispatch(Box::new(CreateAnimation::new("newer", 1.)));
    assert!(p.approve(&mut s).is_err());
    s.set_work_mode(crate::session::WorkMode::Animate);
    unchanged(&s, &plan(), "setup_required");
}
fn ffd(s: &mut AppState) {
    use ankhimate_document::commands::key_cmds::AddDeformKey;
    s.dispatch(Box::new(CreateAnimation::new("impact", 2.)));
    let id = s
        .doc
        .animations
        .iter()
        .find(|(_, a)| a.name == "impact")
        .unwrap()
        .0;
    let sl = slot(s);
    s.dispatch(Box::new(AddDeformKey::new(
        id,
        sl,
        "art",
        0.,
        vec![glam::Vec2::ZERO; 8],
    )));
    s.dispatch(Box::new(AddDeformKey::new(
        id,
        sl,
        "art",
        1.,
        vec![glam::vec2(0.3, 0.2); 8],
    )));
    s.set_work_mode(crate::session::WorkMode::Setup);
}
#[test]
fn ffd_assets_binds_three_cycles_save_reload_and_native_v2_export() {
    let mut s = fixture();
    ffd(&mut s);
    let before = snapshot(&s);
    let binds = mesh(&s).inverse_bind_matrices.clone();
    let clips = rig::value(&s.doc.animations);
    let bytes = s
        .doc
        .assets
        .images
        .values()
        .map(|a| a.bytes.clone())
        .collect::<Vec<_>>();
    let mut p = smooth();
    p.actions.extend(plan().actions);
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let after = snapshot(&s);
    assert_eq!(s.history.undo_label(), Some("AI Weights Edit"));
    assert_eq!(rig::value(&s.doc.animations), clips);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        assert_eq!(mesh(&s).inverse_bind_matrices, binds);
        s.redo();
        assert_eq!(snapshot(&s), after);
        assert_eq!(mesh(&s).inverse_bind_matrices, binds);
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Weights-épreuve.ankh");
    assert!(matches!(
        crate::fileops::write_to(&s, &path),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut re = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut re, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    assert_eq!(snapshot(&re), after);
    assert_eq!(mesh(&re).inverse_bind_matrices, binds);
    assert_eq!(
        re.doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>(),
        bytes
    );
    let project = ankhimate_formats::convert::to_schema(&re.doc.as_project_ref());
    let output = ankhimate_export::run::plan(
        &project,
        &re.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    assert!(output.files.iter().any(|f| f.path == "skeleton.json"));
    let runtime: Value = serde_json::from_str(
        &output
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents,
    )
    .unwrap();
    assert_eq!(runtime["format_version"], 2);
    assert!(runtime.to_string().contains("weights"));
}
#[test]
fn refused_invalid_atomic_and_double_approved_lots_preserve_document_history() {
    let mut s = fixture();
    s.dispatch(Box::new(CreateAnimation::new("redo", 1.)));
    s.undo();
    let before = snapshot(&s);
    let redo = s.history.redo_label().map(str::to_owned);
    let mut panel = Panel {
        settings: Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        share_context: true,
        ..Default::default()
    };
    panel.pending = Some(Pending {
        edit: tools::prepare(&s, &plan()).unwrap(),
        ids: vec!["weights".into()],
        proposal_id: "p1".into(),
    });
    panel.resolve(&mut s, false).unwrap();
    assert_eq!(snapshot(&s), before);
    assert_eq!(s.history.redo_label(), redo.as_deref());
    let mut bad = plan();
    bad.actions.extend(edit(json!({"summary":"bad","actions":[{"op":"mesh.normalize_weights","skin":"default","slot":"body","attachment":"art","vertices":[999]}]})).actions);
    unchanged(&s, &bad, "invalid_vertices");
    panel.pending = Some(Pending {
        edit: tools::prepare(&s, &plan()).unwrap(),
        ids: vec!["weights2".into()],
        proposal_id: "p2".into(),
    });
    panel.resolve(&mut s, true).unwrap();
    let after = snapshot(&s);
    assert!(panel.resolve(&mut s, true).is_err());
    assert_eq!(snapshot(&s), after);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn mixed_or_oversized_batches_refuse_without_partial_actions() {
    let s = fixture();
    let mut p = plan();
    p.actions.extend(breathing("torso").actions);
    unchanged(&s, &p, "weight_batch_limit");
    let mut p = plan();
    p.actions = vec![p.actions[0].clone(); 17];
    unchanged(&s, &p, "weight_batch_limit");
    let mut p = plan();
    p.actions = vec![p.actions[0].clone(); 16];
    if let tools::Action::MeshSet { vertices, .. } = &mut p.actions[0] {
        *vertices = vec![vertices[0].clone(); 129];
    }
    unchanged(&s, &p, "weight_batch_limit");
}
#[test]
fn mocked_targeted_read_and_smoothing_wait_for_approval() {
    let s = fixture();
    let before = snapshot(&s);
    let (base, rx) = server(vec![
        (
            200,
            native_call(
                "ankhimate_read_mesh",
                json!({"skin":"default","slot":"body","attachment":"art","start":0,"count":2}),
            ),
            Duration::ZERO,
        ),
        (
            200,
            native_call("ankhimate_weights", serde_json::to_value(smooth()).unwrap()),
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
        vec![Message::text("user", "Inspect mesh weights near the joint")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert!(turn.plan.is_some());
    assert_eq!(snapshot(&s), before);
    rx.recv().unwrap();
    let second = rx.recv().unwrap().to_string();
    assert!(second.contains("inverse_bind") && second.contains("normalized"));
    assert!(!second.contains("data:image"));
}

#[test]
fn manual_examples_are_typed_schema_checked_and_context_stays_short() {
    let g = manual::select("Observe ce mesh pondéré et lisse ses poids", true);
    assert!(g.sections.contains(&"WEIGHTS"));
    assert!(g.sections.contains(&"VISION"));
    assert!(g.text.len() <= manual::MAX_BYTES);
    let def = weights::definition();
    let variants = def["parameters"]["properties"]["actions"]["items"]["oneOf"]
        .as_array()
        .unwrap();
    let examples: Vec<Value> =
        serde_json::from_str(include_str!("../../../docs/ai/weights-examples.json")).unwrap();
    for row in examples {
        let p = edit(row["plan"].clone());
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
            for k in a.as_object().unwrap().keys() {
                assert!(schema["properties"].get(k).is_some());
            }
        }
        let mut s = fixture();
        let before = snapshot(&s);
        let p = tools::prepare(&s, &p);
        assert_eq!(p.is_ok(), row["valid"].as_bool().unwrap());
        if let Ok(p) = p {
            p.approve(&mut s).unwrap();
            s.undo();
            assert_eq!(snapshot(&s), before);
        }
    }
    assert!(serde_json::from_value::<tools::EditPlan>(json!({"summary":"bad","actions":[{"op":"mesh.normalize_weights","skin":"default","slot":"body","attachment":"art","vertices":[0],"script":"forbidden"}]})).is_err());
}
#[test]
fn ffd_invalid_lengths_and_unrepresentable_influences_do_not_write() {
    let mut s = fixture();
    ffd(&mut s);
    let anim = s
        .doc
        .animations
        .values_mut()
        .find(|a| a.name == "impact")
        .unwrap();
    for t in &mut anim.timelines {
        if let ankhimate_core::animation::Timeline::Deform { keys, .. } = t {
            keys[0].value.pop();
        }
    }
    unchanged(&s, &plan(), "unsafe_mesh");
    let mut s = fixture();
    mesh_mut(&mut s)
        .ffd_keyframes
        .push(ankhimate_core::attachment::FfdKeyframe {
            time: 0.,
            vertex_offsets: vec![glam::Vec2::ZERO; 8],
        });
    let before = serde_json::to_value(&mesh(&s).ffd_keyframes).unwrap();
    tools::prepare(&s, &smooth())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    assert_eq!(
        serde_json::to_value(&mesh(&s).ffd_keyframes).unwrap(),
        before
    );
}
#[test]
fn deletion_undo_remaps_native_weight_inverses_without_losing_mesh_bones() {
    let mut s = fixture();
    let before = snapshot(&s);
    tools::prepare(&s, &plan())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let after = snapshot(&s);
    for _ in 0..3 {
        let id = bone(&s, "torso");
        s.dispatch(Box::new(
            ankhimate_document::commands::bone_cmds::DeleteBone::new(id),
        ));
        s.undo();
        assert_eq!(snapshot(&s), after);
        s.undo();
        assert_eq!(snapshot(&s), before);
        assert!(
            mesh(&s)
                .weights
                .iter()
                .flatten()
                .all(|w| s.doc.skeleton.bones.contains_key(w.bone))
        );
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
}
#[test]
fn page_limits_truncation_and_large_mesh_read_are_explicit() {
    let mut s = fixture();
    mesh_mut(&mut s).weights[0] = vec![mesh(&s).weights[0][0]; 12];
    let p = read(&s, 0, 1).unwrap();
    assert_eq!(p["vertices"][0]["influence_count"], 12);
    assert_eq!(p["vertices"][0]["influences"].as_array().unwrap().len(), 8);
    assert_eq!(p["vertices"][0]["influences_truncated"], true);
    assert_eq!(p["editable"], false);
    let mut s = fixture();
    mesh_mut(&mut s)
        .setup_vertices
        .resize(5000, glam::Vec2::ZERO);
    let p = read(&s, 4999, 1).unwrap();
    assert_eq!(p["count"], 1);
    assert_eq!(p["editable"], false);
    let mut s = fixture();
    let root = bone(&s, "root");
    s.doc.skeleton.bones[root].name = "very_long_bone".repeat(4000);
    assert!(read(&s, 0, 1).is_err());
}
#[test]
fn source_identity_changes_and_deleted_references_abort_late_approval() {
    let mut s = fixture();
    let p = tools::prepare(&s, &plan()).unwrap();
    let mut replacement = mesh(&s).clone();
    replacement.weights[0][0].weight = 0.7;
    let sk = s.doc.skeleton.default_skin;
    let sl = slot(&s);
    s.doc.skeleton.skins[sk].set(sl, "art", Attachment::Mesh(replacement));
    assert!(
        p.approve(&mut s).is_err(),
        "Changed weights without revision must not replace reviewed values"
    );
    let p = tools::prepare(&s, &plan()).unwrap();
    s.doc.skeleton.skins[sk].entries.remove(&(sl, "art".into()));
    assert!(p.approve(&mut s).is_err());
}
#[test]
fn real_native_export_writes_updated_coefficients_with_geometry_and_ffd() {
    let mut s = fixture();
    ffd(&mut s);
    tools::prepare(&s, &plan())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let project = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &project,
        &s.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    ankhimate_export::run::write(&plan, dir.path()).unwrap();
    let data: Value =
        serde_json::from_slice(&std::fs::read(dir.path().join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(data["format_version"], 2);
    let a = &data["skins"][0]["attachments"][0]["attachment"];
    for (i, row) in mesh(&s).weights.iter().enumerate() {
        assert_eq!(a["weights"][i]["count"], row.len());
        for (j, w) in row.iter().enumerate() {
            let exported = &a["weights"][i]["bones"][j];
            assert_eq!(exported["bone"], s.doc.skeleton.bones[w.bone].name);
            assert!((exported["weight"].as_f64().unwrap() - w.weight as f64).abs() < 1e-7);
            let point =
                mesh(&s).inverse_bind_matrices[&w.bone].transform_point(mesh(&s).setup_vertices[i]);
            assert!((exported["x"].as_f64().unwrap() - point.x as f64).abs() < 0.01);
            assert!((exported["y"].as_f64().unwrap() - point.y as f64).abs() < 0.01);
        }
    }
    assert!(data["animations"]["impact"]["deform"].is_array());
    assert!(
        !data["animations"]["impact"]["deform"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(!plan.binaries.is_empty());
}
#[test]
#[ignore = "External delivery copy, headless only; set ANKHIMATE_T808_DEMO and select this test explicitly"]
fn delivered_jade_is_an_unchanged_real_weighted_project_eligible_for_targeted_read() {
    let path = std::path::PathBuf::from(
        std::env::var_os("ANKHIMATE_T808_DEMO").expect("Set the isolated delivery project path"),
    );
    let mut s = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut s, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    let source = tools::snapshot(&s);
    let mut count = 0;
    let mut writable = 0;
    for (_, skin) in &s.doc.skeleton.skins {
        for ((sl, name), a) in &skin.entries {
            if let Attachment::Mesh(m) = a
                && !m.weights.is_empty()
            {
                count += 1;
                let args = weights::ReadMesh {
                    skin: skin.name.clone(),
                    slot: s.doc.skeleton.slots[*sl].name.clone(),
                    attachment: name.clone(),
                    start: 0,
                    count: 4,
                };
                let data = weights::read(&source, &args).unwrap();
                if data["editable"] == true {
                    writable += 1;
                }
                println!(
                    "T808_DEMO {}",
                    json!({"skin":args.skin,"slot":args.slot,"attachment":args.attachment,"vertices":m.setup_vertices.len(),"editable":data["editable"],"limitation":data["write_limitation"],"page":data["vertices"]})
                );
            }
        }
    }
    assert!(count > 0);
    assert!(writable > 0, "Need a genuinely editable weighted demo");
}
#[test]
fn mocked_invalid_weight_is_corrected_inside_the_existing_budget() {
    let s = fixture();
    let mut bad = serde_json::to_value(plan()).unwrap();
    bad["actions"][0]["vertices"][0]["influences"][0]["weight"] = json!(-1);
    let (base, rx) = server(vec![
        (200, native_call("ankhimate_weights", bad), Duration::ZERO),
        (
            200,
            native_call("ankhimate_weights", serde_json::to_value(plan()).unwrap()),
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
        vec![Message::text("user", "Correct weights")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert!(turn.plan.is_some());
    rx.recv().unwrap();
    let second = rx.recv().unwrap().to_string();
    assert!(second.contains("invalid_weight") && second.contains("document_changed"));
}
