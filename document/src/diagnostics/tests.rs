use super::*;
use ankhimate_core::{animation::Animation, assets::ImageAsset};
use ankhimate_core::{
    animation::{Interp, Key, Timeline},
    attachment::{Attachment, MeshAttachment, VertexWeight},
    constraints::{Constraint, IkConstraint},
    skeleton::Bone,
    slot::Slot,
};
fn bone(name: &str) -> Bone {
    Bone {
        name: name.into(),
        parent: None,
        length: 10.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}
fn fixture() -> (
    Document,
    ankhimate_core::ids::BoneId,
    ankhimate_core::ids::SlotId,
) {
    let mut doc = Document::new();
    let b = doc.skeleton.add_bone(bone("root"));
    let slot = doc.skeleton.add_slot(Slot::new("body".into(), b));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::new_rgba8(2, 2)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    doc.assets
        .add(ImageAsset::new("body.png", bytes.into_inner(), 2, 2));
    let m = MeshAttachment {
        texture: "body.png".into(),
        setup_vertices: vec![glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y],
        uvs: vec![glam::Vec2::ZERO; 3],
        triangles: vec![[0, 1, 2]],
        ..Default::default()
    };
    doc.skeleton.skins[doc.skeleton.default_skin].set(slot, "body", Attachment::Mesh(m));
    doc.skeleton.slots[slot].attachment = Some("body".into());
    (doc, b, slot)
}
fn mesh(doc: &mut Document) -> &mut MeshAttachment {
    let skin = doc.skeleton.default_skin;
    let a = doc.skeleton.skins[skin]
        .entries
        .values_mut()
        .next()
        .unwrap();
    let Attachment::Mesh(m) = a else { panic!() };
    m
}
fn has(doc: &Document, code: &str, severity: Severity) -> bool {
    inspect(doc)
        .iter()
        .any(|i| i.code == code && i.severity == severity)
}
fn fingerprint(doc: &Document) -> String {
    format!("{doc:?}")
}
#[test]
fn t818_missing_image_is_an_integrity_error() {
    let mut doc = Document::new();
    doc.assets.add(ImageAsset::new("missing", Vec::new(), 1, 1));
    let issues = inspect(&doc);
    assert!(
        issues
            .iter()
            .any(|i| i.code == "invalid-image" && i.severity == Severity::Error)
    );
}
#[test]
fn t818_static_positive_clip_is_information_not_corruption() {
    let mut doc = Document::new();
    doc.animations.insert(Animation::new("held pose", 1.0));
    let issues = inspect(&doc);
    assert!(
        issues
            .iter()
            .any(|i| i.code == "static-clip" && i.severity == Severity::Info)
    );
    assert!(!issues.iter().any(|i| i.severity == Severity::Error));
}
#[test]
fn t818_valid_rigid_fixture_has_no_alert() {
    let (d, _, _) = fixture();
    assert!(inspect(&d).is_empty());
}
#[test]
fn t818_mixed_rigid_vertices_are_valid_but_zero_weight_influence_is_error() {
    let (mut d, b, _) = fixture();
    mesh(&mut d).weights = vec![
        vec![],
        vec![VertexWeight {
            bone: b,
            weight: 1.0,
        }],
        vec![],
    ];
    crate::rebind::rebind_meshes(&mut d);
    assert!(!has(&d, "invalid-weights", Severity::Error));
    mesh(&mut d).weights[1][0].weight = 0.0;
    assert!(has(&d, "invalid-weights", Severity::Error));
}
#[test]
fn t818_weight_sum_is_warning_not_repair() {
    let (mut d, b, _) = fixture();
    mesh(&mut d).weights = vec![
        vec![VertexWeight {
            bone: b,
            weight: 0.3
        }];
        3
    ];
    let before = fingerprint(&d);
    assert!(has(&d, "weight-sum", Severity::Warning));
    assert_eq!(before, fingerprint(&d));
}
#[test]
fn t818_missing_influence_negative_and_nonfinite_weights_are_errors() {
    for weight in [-1.0, f32::NAN, f32::INFINITY] {
        let (mut d, b, _) = fixture();
        mesh(&mut d).weights = vec![vec![VertexWeight { bone: b, weight }]; 3];
        assert!(has(&d, "invalid-weights", Severity::Error));
    }
    let (mut d, _, _) = fixture();
    mesh(&mut d).weights = vec![
        vec![VertexWeight {
            bone: Default::default(),
            weight: 1.0
        }];
        3
    ];
    assert!(has(&d, "invalid-weights", Severity::Error));
}
#[test]
fn t818_bind_fallback_is_warning_and_rigid_needs_no_bind() {
    let (mut d, b, _) = fixture();
    assert!(!has(&d, "bind-fallback", Severity::Warning));
    mesh(&mut d).weights = vec![
        vec![VertexWeight {
            bone: b,
            weight: 1.0
        }];
        3
    ];
    assert!(has(&d, "bind-fallback", Severity::Warning));
    crate::rebind::rebind_meshes(&mut d);
    assert!(!has(&d, "bind-fallback", Severity::Warning));
    mesh(&mut d).inverse_bind_matrices.get_mut(&b).unwrap().a = f32::NAN;
    assert!(has(&d, "bind-fallback", Severity::Warning));
}
#[test]
fn t818_mesh_topology_uv_weight_count_and_edges() {
    for kind in 0..4 {
        let (mut d, _, _) = fixture();
        let m = mesh(&mut d);
        match kind {
            0 => m.uvs.clear(),
            1 => m.triangles[0][0] = 99,
            2 => m.weights = vec![Vec::new()],
            _ => m.edges = vec![[0, 99]],
        };
        assert!(inspect(&d).iter().any(|i| i.severity == Severity::Error));
    }
}
#[test]
fn t818_setup_and_animated_missing_attachment() {
    let (mut d, _, s) = fixture();
    d.skeleton.slots[s].attachment = Some("missing".into());
    assert!(has(&d, "unknown-attachment", Severity::Error));
    d.skeleton.slots[s].attachment = Some("body".into());
    let mut a = Animation::new("a", 1.0);
    a.timelines.push(Timeline::SlotAttachment {
        slot: s,
        keys: vec![Key {
            time: 0.0,
            value: Some("missing".into()),
            interp: Interp::Stepped,
        }],
    });
    d.animations.insert(a);
    assert!(has(&d, "unknown-attachment", Severity::Error));
}
#[test]
fn t818_partial_skin_fallback_and_absent_default_override() {
    let (mut d, _, s) = fixture();
    let alt = d
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("alt"));
    assert!(!has(&d, "attachment-fallback-missing", Severity::Warning));
    let default = d.skeleton.default_skin;
    let a = d.skeleton.skins[default].remove(s, "body").unwrap();
    d.skeleton.skins[alt].set(s, "body", a);
    assert!(has(&d, "attachment-fallback-missing", Severity::Warning));
}
#[test]
fn t818_missing_skin_slot_and_gating_refs() {
    let (mut d, _, s) = fixture();
    d.skeleton.slots.remove(s);
    assert!(has(&d, "invalid-attachment", Severity::Error));
    let (mut d, _, _) = fixture();
    let default = d.skeleton.default_skin;
    d.skeleton.skins[default].bones.push(Default::default());
    assert!(has(&d, "invalid-skin", Severity::Error));
}
#[test]
fn t818_unused_image_is_info_and_sequence_counts_as_use() {
    let (mut d, _, _) = fixture();
    let a = d.assets.images.values().next().unwrap().clone();
    let mut a2 = a.clone();
    a2.name = "other.png".into();
    d.assets.add(a2);
    assert!(has(&d, "unused-image", Severity::Info));
    let seq = ankhimate_core::attachment::Sequence {
        frames: vec!["other.png".into()],
        ..Default::default()
    };
    mesh(&mut d).sequence = Some(seq);
    assert!(!has(&d, "unused-image", Severity::Info));
    d.assets.images.values_mut().next().unwrap().source_path =
        Some("not-required-on-this-machine".into());
    assert!(!has(&d, "missing-image", Severity::Error));
}
#[test]
fn t818_key_time_value_duration_and_outside_location() {
    let (mut d, b, _) = fixture();
    let mut a = Animation::new("a", 1.0);
    a.timelines.push(Timeline::BoneRotate {
        bone: b,
        keys: vec![Key {
            time: 2.0,
            value: 4.0,
            interp: Interp::Linear,
        }],
    });
    let id = d.animations.insert(a);
    assert!(
        inspect(&d)
            .iter()
            .any(|i| i.code == "key-outside-clip" && matches!(i.location, Location::Key { .. }))
    );
    d.animations[id].duration = f32::NAN;
    assert!(has(&d, "invalid-clip", Severity::Error));
    d.animations[id].duration = 1.0;
    if let Timeline::BoneRotate { keys, .. } = &mut d.animations[id].timelines[0] {
        keys[0].value = f32::NAN;
        keys[0].time = f32::NAN;
    }
    assert!(has(&d, "invalid-key-time", Severity::Error));
    assert!(has(&d, "invalid-key-value", Severity::Error));
}
#[test]
fn t818_duplicate_track_and_missing_timeline_bone() {
    let (mut d, b, _) = fixture();
    let mut a = Animation::new("a", 1.0);
    let t = Timeline::BoneRotate {
        bone: b,
        keys: Vec::new(),
    };
    a.timelines = vec![t.clone(), t];
    d.animations.insert(a);
    assert!(has(&d, "duplicate-timeline", Severity::Error));
    d.skeleton.bones.remove(b);
    assert!(has(&d, "missing-bone", Severity::Error));
}
#[test]
fn t818_draw_order_duplicates_and_missing_refs() {
    let (mut d, _, s) = fixture();
    d.skeleton.draw_order = vec![s, s];
    assert!(has(&d, "duplicate-draw-order", Severity::Error));
    d.skeleton.draw_order = vec![Default::default()];
    assert!(has(&d, "missing-slot", Severity::Error));
}
#[test]
fn t818_duplicate_draw_offset_is_warning_first_wins_not_corruption() {
    let (mut d, _, s) = fixture();
    let mut a = Animation::new("a", 1.0);
    a.timelines.push(Timeline::DrawOrder {
        keys: vec![Key {
            time: 0.0,
            value: vec![(s, 1), (s, 2)],
            interp: Interp::Stepped,
        }],
    });
    d.animations.insert(a);
    assert!(has(&d, "duplicate-draw-offset", Severity::Warning));
}
#[test]
fn t818_ik_chain_missing_target_repeated_and_disjoint() {
    let (mut d, b, _) = fixture();
    let t = d.skeleton.add_bone(bone("target"));
    let c = d
        .skeleton
        .add_constraint(Constraint::Ik(IkConstraint::aim("ik", t, b)));
    assert!(!has(&d, "invalid-ik-chain", Severity::Error));
    for chain in [vec![], vec![b, b], vec![b, t]] {
        if let Constraint::Ik(ik) = &mut d.skeleton.constraints[c] {
            ik.bones = chain;
        }
        assert!(has(&d, "invalid-ik-chain", Severity::Error));
    }
    if let Constraint::Ik(ik) = &mut d.skeleton.constraints[c] {
        ik.bones = vec![b];
        ik.target = Default::default();
    }
    assert!(has(&d, "invalid-ik-chain", Severity::Error));
}
#[test]
fn t818_ik_skipped_ancestor_is_valid_and_reversed_is_not() {
    let (mut d, b, _) = fixture();
    let mut child = bone("child");
    child.parent = Some(b);
    let child = d.skeleton.add_bone(child);
    let mut tip = bone("tip");
    tip.parent = Some(child);
    let tip = d.skeleton.add_bone(tip);
    let target = d.skeleton.add_bone(bone("target"));
    assert!(
        ik_structure(
            &d.skeleton,
            &IkConstraint::chain("ik", target, vec![b, tip])
        )
        .is_ok()
    );
    assert!(
        ik_structure(
            &d.skeleton,
            &IkConstraint::chain("ik", target, vec![tip, b])
        )
        .is_err()
    );
}
#[test]
fn t818_ffd_stale_count_and_empty_reset() {
    let (mut d, _, s) = fixture();
    let mut a = Animation::new("a", 1.0);
    a.timelines.push(Timeline::Deform {
        slot: s,
        attachment: "body".into(),
        keys: vec![Key {
            time: 0.0,
            value: vec![glam::Vec2::ZERO],
            interp: Interp::Linear,
        }],
    });
    let id = d.animations.insert(a);
    assert!(has(&d, "invalid-deform", Severity::Error));
    if let Timeline::Deform { keys, .. } = &mut d.animations[id].timelines[0] {
        keys[0].value.clear();
    }
    assert!(!has(&d, "invalid-deform", Severity::Error));
}
#[test]
fn t818_target_fidelity_does_not_become_author_document_error() {
    let (mut d, b, _) = fixture();
    d.skeleton.add_constraint(Constraint::Physics(
        ankhimate_core::constraints::PhysicsConstraint::sway("sway", b),
    ));
    let native = inspect(&d);
    assert!(!native.iter().any(|i| i.code.contains("unqualified")
        || i.code.contains("runtime-physics")
        || i.code == "rendered-physics"));
    let p = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    let r = ankhimate_export::preflight::inspect(
        &p,
        &d.assets,
        ankhimate_export::preflight::Target::Rendered,
        None,
    );
    assert!(r.has_errors());
}
#[test]
fn t818_deterministic_dedup_and_zero_mutation_full_document() {
    let (mut d, _, s) = fixture();
    let mut a = Animation::new("broken", 1.0);
    a.timelines.push(Timeline::Deform {
        slot: s,
        attachment: "absent".into(),
        keys: Vec::new(),
    });
    d.animations.insert(a);
    let before = fingerprint(&d);
    let a = inspect(&d);
    let b = inspect(&d);
    assert_eq!(a, b);
    let mut unique = a.clone();
    unique.dedup();
    assert_eq!(unique, a);
    assert_eq!(before, fingerprint(&d));
}
#[test]
fn t818_linked_geometry_missing_or_cyclic_is_reported_without_mutation() {
    let (mut d, _, _) = fixture();
    mesh(&mut d).linked = Some(ankhimate_core::attachment::LinkedMesh {
        skin: None,
        slot: "body".into(),
        attachment: "missing".into(),
        inherit_deform: true,
    });
    let before = fingerprint(&d);
    assert!(has(&d, "invalid-linked-mesh", Severity::Error));
    assert_eq!(before, fingerprint(&d));
    mesh(&mut d).linked.as_mut().unwrap().attachment = "body".into();
    assert!(has(&d, "invalid-linked-mesh", Severity::Error));
}
#[test]
fn t818_inline_ffd_count_and_invalid_coordinates() {
    let (mut d, _, _) = fixture();
    mesh(&mut d)
        .ffd_keyframes
        .push(ankhimate_core::attachment::FfdKeyframe {
            time: 0.0,
            vertex_offsets: vec![glam::Vec2::NAN],
        });
    assert!(has(&d, "invalid-inline-deform", Severity::Error));
}
#[test]
fn t818_nonfinite_polygon_and_path_are_author_errors_without_fidelity_warning() {
    let (mut d, _, s) = fixture();
    let skin = d.skeleton.default_skin;
    d.skeleton.skins[skin].set(
        s,
        "clip",
        Attachment::Clipping(ankhimate_core::attachment::ClippingAttachment {
            vertices: vec![glam::Vec2::NAN],
            end_slot: None,
        }),
    );
    d.skeleton.skins[skin].set(
        s,
        "path",
        Attachment::Path(ankhimate_core::attachment::PathAttachment {
            vertices: vec![glam::Vec2::INFINITY],
            ..Default::default()
        }),
    );
    assert!(has(&d, "invalid-polygon", Severity::Error));
    assert!(has(&d, "invalid-path", Severity::Error));
    assert!(!inspect(&d).iter().any(|i| i.code.contains("unqualified")));
}
#[test]
fn t818_missing_native_evaluation_order_and_invalid_constraint_address() {
    let (mut d, b, _) = fixture();
    d.skeleton.update_order.clear();
    assert!(has(&d, "bone-update-order", Severity::Error));
    d.skeleton.rebuild_update_order();
    let c = d.skeleton.add_constraint(Constraint::Physics(
        ankhimate_core::constraints::PhysicsConstraint::sway("physics", b),
    ));
    let mut a = Animation::new("wrong", 1.0);
    a.timelines.push(Timeline::IkMix {
        constraint: c,
        keys: Vec::new(),
    });
    d.animations.insert(a);
    assert!(has(&d, "invalid-track-address", Severity::Error));
}
