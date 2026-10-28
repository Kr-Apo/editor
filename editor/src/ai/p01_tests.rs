use super::*;

#[test]
fn p01_ai_deform_read_is_paged_asset_free_and_available_without_editing() {
    let mut s = state();
    let p = plan(&s, 0., 3.);
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let clip = s.doc.animations[s.session.active_animation.unwrap()]
        .name
        .clone();
    let mut args = artistic::ReadDeform {
        animation: clip,
        skin: "default".into(),
        slot: "cape_slot".into(),
        attachment: "cape".into(),
        key_index: 0,
        start: 0,
        count: 2,
    };
    let before = serialized(&s);
    let source = tools::snapshot(&s);
    let page = artistic::read(&source, &args).unwrap();
    assert_eq!(page["offsets"].as_array().unwrap().len(), 2);
    assert_eq!(page["next_start"], 2);
    assert_eq!(serialized(&s), before);
    assert!(!page.to_string().contains("source_path"));
    args.count = 33;
    assert!(artistic::read(&source, &args).is_err());
    args.count = 1;
    args.key_index = 99;
    assert!(artistic::read(&source, &args).is_err());
    assert!(
        tools::definitions(false)
            .iter()
            .any(|d| d["name"] == "ankhimate_read_deform")
    );
}

fn state() -> crate::app_state::AppState {
    let mut s = crate::app_state::AppState {
        doc: ankhimate_document::Document::from_loaded(
            ankhimate_formats::from_json(include_str!("../../tests/fixtures/p01-cape.json"))
                .unwrap(),
        ),
        ..Default::default()
    };
    s.create_animation();
    s.set_work_mode(crate::session::WorkMode::Animate);
    s.refresh_pose();
    s.rebind_meshes();
    s.history = Default::default();
    s
}
fn plan(s: &crate::app_state::AppState, time: f32, value: f32) -> tools::EditPlan {
    let clip = &s.doc.animations[s.session.active_animation.unwrap()].name;
    let m = s.doc.skeleton.skins[s.doc.skeleton.default_skin]
        .entries
        .values()
        .next()
        .unwrap();
    let ankhimate_core::attachment::Attachment::Mesh(m) = m else {
        unreachable!()
    };
    serde_json::from_value(json!({"summary":"Bounded cape FFD key","actions":[{"op":"deform.key","animation":clip,"skin":"default","slot":"cape_slot","attachment":"cape","time":time,"offsets":vec![[value,0.];m.influence_count()]}]})).unwrap()
}
fn serialized(s: &crate::app_state::AppState) -> String {
    ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
}

#[test]
fn p01_ai_ffd_requires_approval_is_atomic_and_survives_three_undo_redo_cycles() {
    let mut s = state();
    let before = serialized(&s);
    let p = plan(&s, 0., 0.);
    let proposal = tools::prepare(&s, &p).unwrap();
    assert_eq!(serialized(&s), before);
    drop(proposal);
    assert!(!s.history.can_undo());
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let after = serialized(&s);
    for _ in 0..3 {
        s.undo();
        assert_eq!(serialized(&s), before);
        s.redo();
        assert_eq!(serialized(&s), after);
    }
    let p = plan(&s, 0., 12.);
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let id = s.session.active_animation.unwrap();
    assert!(
        matches!(s.doc.animations[id].timelines.last(),Some(ankhimate_core::animation::Timeline::Deform{keys,..}) if keys.len()==1 && keys[0].value[0].x==12.)
    );
    let native = serialized(&s);
    let reopened =
        ankhimate_document::Document::from_loaded(ankhimate_formats::from_json(&native).unwrap());
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        native
    );
}

#[test]
fn p01_ai_invalid_lot_stale_revision_and_changed_mesh_preserve_redo() {
    let mut s = state();
    let p = plan(&s, 0., 0.);
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    s.undo();
    let before = serialized(&s);
    let mut bad = p.clone();
    bad.actions.push(
        serde_json::from_value(
            json!({"op":"event.add","animation":"missing","name":"impact","time":0.5}),
        )
        .unwrap(),
    );
    assert!(tools::prepare(&s, &bad).is_err());
    assert_eq!(serialized(&s), before);
    assert!(s.history.can_redo());
    let prepared = tools::prepare(&s, &p).unwrap();
    s.revision += 1;
    assert!(prepared.approve(&mut s).is_err());
    assert_eq!(serialized(&s), before);
    assert!(s.history.can_redo());
    let prepared = tools::prepare(&s, &p).unwrap();
    if let Some(ankhimate_core::attachment::Attachment::Mesh(m)) = s.doc.skeleton.skins
        [s.doc.skeleton.default_skin]
        .entries
        .values_mut()
        .next()
    {
        m.setup_vertices[0].x += 1.;
    }
    let changed = serialized(&s);
    assert!(prepared.approve(&mut s).is_err());
    assert_eq!(serialized(&s), changed);
    assert!(s.history.can_redo());
}

#[test]
fn p01_ai_rejects_invalid_offsets_and_locked_or_linked_meshes() {
    for change in ["short", "nan", "time", "locked", "linked", "skin"] {
        let mut s = state();
        let mut p = plan(&s, 0., 0.);
        if let tools::Action::DeformKey {
            offsets,
            time,
            skin,
            ..
        } = &mut p.actions[0]
        {
            match change {
                "short" => {
                    offsets.pop();
                }
                "nan" => offsets[0][0] = f32::NAN,
                "time" => *time = -1.,
                "skin" => *skin = "missing".into(),
                _ => {}
            }
        }
        if change == "locked" {
            let slot = s.doc.skeleton.slots.values().next().unwrap();
            s.session.locked_bones.insert(slot.bone, true);
        }
        if change == "linked"
            && let Some(ankhimate_core::attachment::Attachment::Mesh(m)) = s.doc.skeleton.skins
                [s.doc.skeleton.default_skin]
                .entries
                .values_mut()
                .next()
        {
            m.linked = Some(ankhimate_core::attachment::LinkedMesh {
                skin: None,
                slot: "cape_slot".into(),
                attachment: "source".into(),
                inherit_deform: true,
            });
        }
        let before = serialized(&s);
        assert!(tools::prepare(&s, &p).is_err(), "{change}");
        assert_eq!(serialized(&s), before);
    }
}

#[test]
fn p01_ai_event_add_and_ffd_delete_use_one_transaction() {
    let mut s = state();
    let p = plan(&s, 0., 0.);
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let before = serialized(&s);
    let clip = s.doc.animations[s.session.active_animation.unwrap()]
        .name
        .clone();
    let p:tools::EditPlan=serde_json::from_value(json!({"summary":"Replace impact signal","actions":[{"op":"deform.key_delete","animation":clip,"skin":"default","slot":"cape_slot","attachment":"cape","time":0},{"op":"event.add","animation":clip,"name":"impact","time":0.5}]})).unwrap();
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let after = serialized(&s);
    s.undo();
    assert_eq!(serialized(&s), before);
    s.redo();
    assert_eq!(serialized(&s), after);
    assert!(
        tools::prepare(&s, &p).is_err(),
        "deletion cannot silently target an absent key"
    );
}

#[test]
fn p01_ai_bone_key_delete_is_exact_locked_and_transactional() {
    let mut s = state();
    let clip = s.doc.animations[s.session.active_animation.unwrap()]
        .name
        .clone();
    let bone = s.doc.skeleton.bones.values().next().unwrap().name.clone();
    let p:tools::EditPlan=serde_json::from_value(json!({"summary":"Exact bone key","actions":[{"op":"key.add","animation":clip,"bone":bone,"channel":"rotate","time":0.5,"value":20,"interpolation":"linear"}]})).unwrap();
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let before = serialized(&s);
    let p:tools::EditPlan=serde_json::from_value(json!({"summary":"Delete that exact key","actions":[{"op":"key.delete","animation":clip,"bone":bone,"channel":"rotate","time":0.5}]})).unwrap();
    tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    let after = serialized(&s);
    assert_ne!(before, after);
    s.undo();
    assert_eq!(serialized(&s), before);
    s.redo();
    assert_eq!(serialized(&s), after);
    assert!(tools::prepare(&s, &p).is_err());
    s.undo();
    let target = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == bone)
        .unwrap()
        .0;
    s.session.locked_bones.insert(target, true);
    assert!(tools::prepare(&s, &p).is_err());
}

#[test]
fn p01_ai_ffd_interpolation_file_roundtrip_and_native_export_preserve_geometry() {
    let mut s = state();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([10, 20, 30, 255]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "cape",
        bytes.into_inner(),
        2,
        2,
    ));
    let shape = super::assembly::fingerprint(&s.doc.skeleton);
    for (time, value) in [(0., 0.), (1., 4.)] {
        let p = plan(&s, time, value);
        tools::prepare(&s, &p).unwrap().approve(&mut s).unwrap();
    }
    assert_eq!(super::assembly::fingerprint(&s.doc.skeleton), shape);
    s.set_playhead(0.5);
    let values = s.pose.deforms.values().next().unwrap();
    assert!(values.iter().all(|v| (v.x - 2.).abs() < 1e-5));
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("approved.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    let reopened = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        serialized(&s)
    );
    assert_eq!(
        reopened.assets.images.values().next().unwrap().bytes,
        s.doc.assets.images.values().next().unwrap().bytes
    );
    let schema = ankhimate_formats::convert::to_schema(&reopened.as_project_ref());
    let export = ankhimate_export::run::plan(
        &schema,
        &reopened.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let destination = tmp.path().join("export");
    std::fs::create_dir(&destination).unwrap();
    ankhimate_export::run::write(&export, &destination).unwrap();
    let native: Value =
        serde_json::from_slice(&std::fs::read(destination.join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(native["format_version"], 2);
    assert_eq!(super::assembly::fingerprint(&s.doc.skeleton), shape);
}

#[test]
fn p01_native_artistic_operations_are_in_the_real_tool_schema() {
    let defs = tools::definitions(true);
    let animate = defs
        .iter()
        .find(|d| d["name"] == "ankhimate_animate")
        .unwrap();
    let variants = animate["parameters"]["properties"]["actions"]["items"]["oneOf"]
        .as_array()
        .unwrap();
    for op in ["deform.key", "deform.key_delete", "key.delete", "event.add"] {
        assert!(
            variants
                .iter()
                .any(|v| v["properties"]["op"]["const"] == op),
            "missing typed operation {op}"
        );
    }
}
