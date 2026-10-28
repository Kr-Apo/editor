use ankhimate_core::{
    attachment::{Attachment, LinkedMesh, MeshAttachment, VertexWeight},
    ids::SlotId,
    pose::{Pose, evaluate},
    skeleton::{Bone, Skeleton},
    slot::Slot,
};
use glam::{Vec2, vec2};
fn fixture() -> (Skeleton, Pose, SlotId) {
    let mut s = Skeleton::new();
    let b = s.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = s.add_slot(Slot {
        attachment: Some("royal".into()),
        ..Slot::new("body".into(), b)
    });
    let mut g = MeshAttachment {
        texture: "source".into(),
        setup_vertices: vec![vec2(0., 0.), vec2(10., 0.), vec2(0., 10.)],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        triangles: vec![[0, 1, 2]],
        weights: vec![
            vec![VertexWeight {
                bone: b,
                weight: 1.
            }];
            3
        ],
        ..Default::default()
    };
    let mut p = Pose::new();
    evaluate(&s, &[], &mut p);
    g.bind_to_pose(&p, p.world(b));
    let skin = s.default_skin;
    s.skins[skin].set(slot, "source", Attachment::Mesh(g));
    for (name, inherit) in [("royal", true), ("plain", false)] {
        s.skins[skin].set(
            slot,
            name,
            Attachment::Mesh(MeshAttachment {
                texture: name.into(),
                linked: Some(LinkedMesh {
                    skin: None,
                    slot: "body".into(),
                    attachment: "source".into(),
                    inherit_deform: inherit,
                }),
                ..Default::default()
            }),
        );
    }
    p.deforms
        .insert((slot, "source".into()), vec![vec2(3., 4.); 3]);
    p.deforms
        .insert((slot, "plain".into()), vec![vec2(-2., 1.); 3]);
    (s, p, slot)
}
fn source(s: &mut Skeleton, slot: SlotId) -> &mut MeshAttachment {
    let skin = s.default_skin;
    let Attachment::Mesh(m) = s.skins[skin]
        .entries
        .get_mut(&(slot, "source".into()))
        .unwrap()
    else {
        panic!()
    };
    m
}
fn link(s: &mut Skeleton, slot: SlotId) -> &mut LinkedMesh {
    let skin = s.default_skin;
    let Attachment::Mesh(m) = s.skins[skin]
        .entries
        .get_mut(&(slot, "royal".into()))
        .unwrap()
    else {
        panic!()
    };
    m.linked.as_mut().unwrap()
}
#[test]
fn a10_true_false_exact_owner_not_length() {
    let (s, mut p, slot) = fixture();
    p.deforms
        .insert((slot, "royal".into()), vec![vec2(90., 90.); 3]);
    p.deforms
        .insert((slot, "wrong".into()), vec![vec2(80., 80.); 3]);
    let r = s.resolve_mesh_pose(&[], &p, slot, "royal").unwrap();
    assert_eq!(r.deform_owner.attachment, "source");
    assert_eq!(r.texture, "royal");
    assert!(
        (r.vertices(&p).unwrap()[0] - vec2(3., 4.))
            .abs()
            .max_element()
            < 0.000001
    );
    let r = s.resolve_mesh_pose(&[], &p, slot, "plain").unwrap();
    assert_eq!(r.deform_owner.attachment, "plain");
    assert!(
        (r.vertices(&p).unwrap()[0] - vec2(-2., 1.))
            .abs()
            .max_element()
            < 0.000001
    );
}
#[test]
fn a10_no_source_timeline_is_rest_never_own_or_wrong_fallback() {
    let (s, mut p, slot) = fixture();
    p.deforms.remove(&(slot, "source".into()));
    p.deforms
        .insert((slot, "royal".into()), vec![vec2(90., 90.); 3]);
    assert_eq!(
        s.resolve_mesh_pose(&[], &p, slot, "royal")
            .unwrap()
            .vertices(&p)
            .unwrap()[0],
        Vec2::ZERO
    );
    p.deforms.remove(&(slot, "plain".into()));
    assert_eq!(
        s.resolve_mesh_pose(&[], &p, slot, "plain")
            .unwrap()
            .vertices(&p)
            .unwrap()[0],
        Vec2::ZERO
    );
}
#[test]
fn a10_explicit_empty_reset() {
    let (s, mut p, slot) = fixture();
    p.deforms.insert((slot, "source".into()), vec![]);
    assert_eq!(
        s.resolve_mesh_pose(&[], &p, slot, "royal")
            .unwrap()
            .vertices(&p)
            .unwrap()[0],
        Vec2::ZERO
    );
}
#[test]
fn a10_bad_source_skin_chain_cycle() {
    for case in 0..4 {
        let (mut s, p, slot) = fixture();
        match case {
            0 => link(&mut s, slot).attachment = "absent".into(),
            1 => link(&mut s, slot).skin = Some("unknown".into()),
            2 => link(&mut s, slot).attachment = "royal".into(),
            _ => link(&mut s, slot).attachment = "plain".into(),
        };
        assert!(s.resolve_mesh_pose(&[], &p, slot, "royal").is_err());
    }
}
#[test]
fn a10_wrong_dimensions_and_nonfinite_offsets() {
    for v in [
        vec![Vec2::ZERO; 2],
        vec![vec2(f32::NAN, 0.); 3],
        vec![vec2(f32::INFINITY, 0.); 3],
    ] {
        let (s, mut p, slot) = fixture();
        p.deforms.insert((slot, "source".into()), v);
        assert!(s.resolve_mesh_pose(&[], &p, slot, "royal").is_err());
    }
}
#[test]
fn a10_invalid_topology_influences_and_binds() {
    for case in 0..8 {
        let (mut s, p, slot) = fixture();
        let m = source(&mut s, slot);
        match case {
            0 => m.inverse_bind_matrices.clear(),
            1 => m.inverse_bind_matrices.values_mut().next().unwrap().a = 0.,
            2 => m.weights[0][0].weight = -1.,
            3 => m.weights[0][0].weight = f32::NAN,
            4 => m.triangles[0][0] = 99,
            5 => m.setup_vertices[0].x = f32::INFINITY,
            6 => {
                m.weights.pop();
            }
            _ => {
                m.uvs.pop();
            }
        }
        assert!(
            s.resolve_mesh_pose(&[], &p, slot, "royal").is_err(),
            "case{case}"
        );
    }
}
#[test]
fn a10_ambiguous_skin_refused() {
    let (mut s, p, slot) = fixture();
    s.add_skin(ankhimate_core::skin::Skin::new("alternate"));
    assert!(s.resolve_mesh_pose(&[], &p, slot, "royal").is_err());
}
#[test]
fn a10_nonlinked_rigid_and_weighted_unchanged() {
    let (mut s, p, slot) = fixture();
    for weighted in [true, false] {
        if !weighted {
            source(&mut s, slot).weights.clear();
        }
        let m = source(&mut s, slot).clone();
        let r = s.resolve_mesh_pose(&[], &p, slot, "source").unwrap();
        for (i, v) in r.vertices(&p).unwrap().iter().enumerate() {
            let d = &p.deforms[&(slot, "source".into())];
            let offsets = if weighted {
                &d[m.influence_range(i)]
            } else {
                &d[i..i + 1]
            };
            assert_eq!(
                *v,
                m.skin_vertex_with_deform(i, offsets, &p, &p.world(r.host))
            );
        }
    }
}

#[test]
fn a10_cross_slot_source_owner_is_exact() {
    let (mut s, mut p, slot) = fixture();
    let host = s.slots[slot].bone;
    let other = s.add_slot(Slot::new("source slot".into(), host));
    let m = source(&mut s, slot).clone();
    let skin = s.default_skin;
    s.skins[skin].set(other, "source", Attachment::Mesh(m));
    link(&mut s, slot).slot = "source slot".into();
    p.deforms
        .insert((other, "source".into()), vec![vec2(7., 2.); 3]);
    let r = s.resolve_mesh_pose(&[], &p, slot, "royal").unwrap();
    assert_eq!(r.deform_owner.slot, other);
    assert!(
        (r.vertices(&p).unwrap()[0] - vec2(7., 2.))
            .abs()
            .max_element()
            < 0.000001
    );
}
#[test]
fn a10_missing_slot_nonmesh_and_deleted_influence() {
    for i in 0..3 {
        let (mut s, p, slot) = fixture();
        match i {
            0 => link(&mut s, slot).slot = "absent".into(),
            1 => {
                let skin = s.default_skin;
                s.skins[skin].set(slot, "source", Attachment::Point(Default::default()));
            }
            _ => {
                let b = source(&mut s, slot).weights[0][0].bone;
                s.bones.remove(b);
            }
        }
        assert!(s.resolve_mesh_pose(&[], &p, slot, "royal").is_err());
    }
}
#[test]
fn a10_overflow_refuses_before_vertices_are_published() {
    let (s, mut p, slot) = fixture();
    p.deforms
        .insert((slot, "source".into()), vec![vec2(f32::MAX, f32::MAX); 3]);
    let bone = s.slots[slot].bone;
    p.worlds[bone].a = f32::MAX;
    let r = s.resolve_mesh_pose(&[], &p, slot, "royal").unwrap();
    assert!(r.vertices(&p).is_err());
}

#[test]
fn a10_missing_world_or_invalid_uv_and_degenerate_triangle() {
    for i in 0..3 {
        let (mut s, mut p, slot) = fixture();
        match i {
            0 => {
                let b = s.slots[slot].bone;
                p.worlds.remove(b);
            }
            1 => source(&mut s, slot).uvs[0].x = 2.,
            _ => source(&mut s, slot).triangles[0] = [0, 0, 1],
        };
        assert!(s.resolve_mesh_pose(&[], &p, slot, "royal").is_err());
    }
}
