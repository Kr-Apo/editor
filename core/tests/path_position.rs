mod support {
    include!("support/path_position_fixture.rs");
}
use ankhimate_core::{
    animation::Animation,
    ids::{BoneId, ConstraintId},
    skeleton::Skeleton,
};
use ankhimate_core::{
    animation::{self, Interp, Timeline},
    attachment::Attachment,
    constraints::Constraint,
    pose::{Pose, evaluate},
};
use support::fixture;
fn pose(s: &Skeleton, a: &Animation, t: f32) -> Pose {
    let mut p = Pose::new();
    evaluate(s, &[(a, t, 1.)], &mut p);
    p
}
fn static_pose(s: &Skeleton, id: ConstraintId, position: f32) -> Pose {
    let mut s = s.clone();
    if let Constraint::Path(p) = &mut s.constraints[id] {
        p.position = position;
    }
    let mut out = Pose::new();
    evaluate(&s, &[], &mut out);
    out
}
fn same(a: &Pose, b: &Pose, bones: &[BoneId]) {
    for bone in bones {
        let x = a.world(*bone);
        let y = b.world(*bone);
        assert!(
            (x.tx - y.tx).abs() < 0.0001 && (x.ty - y.ty).abs() < 0.0001,
            "{x:?} vs {y:?}"
        );
        assert!(
            (a.world_decomposed(*bone).rotation - b.world_decomposed(*bone).rotation).abs()
                < 0.0001
        );
    }
}
#[test]
fn t831_five_bone_chain_linear_progression_and_spacing() {
    let (s, a, id, bones) = fixture(false, true);
    for t in [0., 0.25, 0.5, 0.75, 1.] {
        let p = pose(&s, &a, t);
        let x = t * 0.5;
        for (i, b) in bones.iter().enumerate() {
            let d = 200. * x + 10. * i as f32;
            let expected = if d <= 100. {
                glam::vec2(d, 0.)
            } else {
                glam::vec2(100., d - 100.)
            };
            assert!(p.world_position(*b).distance(expected) < 0.0001);
        }
        same(&p, &static_pose(&s, id, x), &bones);
    }
    if let Constraint::Path(p) = &s.constraints[id] {
        assert_eq!(p.position, 0.);
    }
}
#[test]
fn t831_path_interpolations_match_scalar_and_static_reference() {
    for interp in [
        Interp::Linear,
        Interp::Stepped,
        Interp::Bezier {
            out_handle: glam::vec2(0.2, -0.1),
            in_handle: glam::vec2(0.8, 1.2),
        },
    ] {
        let (s, mut a, id, bones) = fixture(false, true);
        if let Timeline::PathPosition { keys, .. } = &mut a.timelines[0] {
            keys[1].interp = interp;
            keys[2].interp = interp;
        }
        for t in [-0.1, 0., 0.25, 0.5, 0.75, 1., 1.2] {
            let Timeline::PathPosition { keys, .. } = &a.timelines[0] else {
                panic!()
            };
            let value = animation::sample(keys, t, &mut 0).unwrap();
            same(&pose(&s, &a, t), &static_pose(&s, id, value), &bones);
        }
    }
}
#[test]
fn t831_open_closed_wrap_negative_and_outside_positions() {
    for closed in [false, true] {
        for speed in [false, true] {
            let (s, mut a, id, bones) = fixture(closed, speed);
            for value in [-2., -0.25, 0., 0.5, 1., 1.25, 3.] {
                if let Timeline::PathPosition { keys, .. } = &mut a.timelines[0] {
                    for k in keys {
                        k.value = value;
                    }
                }
                same(&pose(&s, &a, 0.25), &static_pose(&s, id, value), &bones);
            }
        }
    }
}
#[test]
fn t831_arc_length_and_vertex_index_units_differ() {
    let (s, a, _, bones) = fixture(false, true);
    let mut s2 = s.clone();
    let pathid = s2.default_skin;
    let slot = s2.slots.keys().next().unwrap();
    for skel in [&mut s2] {
        if let Attachment::Path(p) = skel.skins[pathid]
            .entries
            .get_mut(&(slot, "curve".into()))
            .unwrap()
        {
            p.vertices[1].x = 20.;
            p.vertices[2] = glam::vec2(100., 0.);
            p.constant_speed = false;
        }
    }
    let p = pose(&s2, &a, 0.5);
    assert!((p.world_position(bones[0]).x - 10.).abs() < 0.0001);
    let mut s3 = s2.clone();
    if let Attachment::Path(p) = s3.skins[pathid]
        .entries
        .get_mut(&(slot, "curve".into()))
        .unwrap()
    {
        p.constant_speed = true;
    }
    assert!((pose(&s3, &a, 0.5).world_position(bones[0]).x - 25.).abs() < 0.0001);
    let _ = s;
}
#[test]
fn t831_degenerate_duplicate_and_two_point_paths_remain_finite() {
    for vertices in [
        vec![glam::Vec2::ZERO; 3],
        vec![glam::Vec2::ZERO, glam::Vec2::ZERO, glam::vec2(100., 0.)],
        vec![glam::Vec2::ZERO, glam::vec2(100., 0.)],
        vec![glam::Vec2::ZERO],
    ] {
        for speed in [false, true] {
            let (mut s, a, _, bones) = fixture(false, speed);
            let slot = s.slots.keys().next().unwrap();
            let skin = s.default_skin;
            if let Attachment::Path(p) = s.skins[skin]
                .entries
                .get_mut(&(slot, "curve".into()))
                .unwrap()
            {
                p.vertices = vertices.clone();
            }
            for t in [0., 0.25, 0.5, 0.75, 1.] {
                let p = pose(&s, &a, t);
                for b in &bones {
                    assert!(p.world_position(*b).is_finite());
                }
            }
        }
    }
}
#[test]
fn t831_partial_mix_fresh_samples_and_clip_isolation() {
    let (mut s, a, id, bones) = fixture(true, true);
    if let Constraint::Path(p) = &mut s.constraints[id] {
        p.mix_translate = 0.35;
        p.mix_rotate = 0.4;
    }
    let b = Animation::new("other", 1.);
    let mut reused = Pose::new();
    for t in [0.75, 0.25, 0.75, 0.0] {
        evaluate(&s, &[(&a, t, 1.)], &mut reused);
        same(&reused, &pose(&s, &a, t), &bones);
    }
    evaluate(&s, &[(&b, 0.75, 1.)], &mut reused);
    let reference = static_pose(&s, id, 0.);
    same(&reused, &reference, &bones);
    assert!(reused.path_position.is_empty());
}
#[test]
fn t831_exact_constraint_identity_order_homonyms_wrong_type_and_duplicates() {
    let (mut s, a, id, bones) = fixture(false, true);
    let mut second = if let Constraint::Path(p) = s.constraints[id].clone() {
        p
    } else {
        panic!()
    };
    second.position = 0.75;
    let other = s.add_constraint(Constraint::Path(second));
    s.constraint_order.swap(0, 1);
    let p = pose(&s, &a, 0.5);
    same(&p, &static_pose(&s, id, 0.25), &bones);
    assert_eq!(p.path_position.get(id), Some(&0.25));
    assert!(p.path_position.get(other).is_none());
    let mut duplicate = a.clone();
    duplicate.timelines.extend(a.timelines.clone());
    assert!(pose(&s, &duplicate, 0.5).path_position.is_empty());
    s.remove_constraint(id);
    assert!(pose(&s, &a, 0.5).path_position.is_empty());
    let ik = ankhimate_core::constraints::IkConstraint::chain("wrong", bones[0], bones.clone());
    let new = s.add_constraint(Constraint::Ik(ik));
    assert_ne!(id, new);
    assert!(pose(&s, &a, 0.5).path_position.is_empty());
}

#[test]
fn t831_curved_polyline_world_transform_and_repeated_samples() {
    for speed in [false, true] {
        let (mut s, a, id, bones) = fixture(true, speed);
        let slot = s.slots.keys().next().unwrap();
        let root = s.slots[slot].bone;
        s.bones[root].local_transform.position = glam::vec2(17., -9.);
        s.bones[root].local_transform.rotation = 0.3;
        let skin = s.default_skin;
        if let Attachment::Path(p) = s.skins[skin]
            .entries
            .get_mut(&(slot, "curve".into()))
            .unwrap()
        {
            p.vertices = (0..9)
                .map(|i| {
                    let t = i as f32 * std::f32::consts::PI / 8.;
                    glam::vec2(t.cos() * 70., t.sin() * 40.)
                })
                .collect();
        }
        for t in [0., 0.25, 0.5, 0.75, 1.] {
            same(&pose(&s, &a, t), &static_pose(&s, id, t * 0.5), &bones);
        }
    }
}
#[test]
fn t831_extreme_closed_animated_input_never_injects_nonfinite_worlds() {
    let (s, mut a, _, bones) = fixture(true, true);
    if let Timeline::PathPosition { keys, .. } = &mut a.timelines[0] {
        for k in keys {
            k.value = f32::MAX;
        }
    }
    let p = pose(&s, &a, 0.5);
    for b in bones {
        assert!(p.world_position(b).is_finite());
    }
}
