use super::*;
use crate::{
    WorkMode,
    commands::{EditCommand, History, key_cmds::BoneProperty},
};
use ankhimate_core::{
    animation::{Animation, Axis},
    math::Transform,
    skeleton::Bone,
};
fn curve() -> Interp {
    Interp::Bezier {
        out_handle: Vec2::new(0.25, 0.25),
        in_handle: Vec2::new(0.75, 0.75),
    }
}
fn fixture(times: [f32; 3], values: [f32; 3]) -> (Document, Junction) {
    let mut d = Document::new();
    let bone = d.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 1.0,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("curve", times[2]);
    a.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::X,
        keys: (0..3)
            .map(|i| Key {
                time: times[i],
                value: values[i],
                interp: curve(),
            })
            .collect(),
    });
    let anim = d.animations.insert(a);
    let addr = TimelineAddr::Bone {
        bone,
        property: BoneProperty::Translate,
        axis: Some(Axis::X),
    };
    let j = Junction::read(&d, anim, addr, 1).unwrap();
    (d, j)
}
fn vectors(j: &Junction, p: [Interp; 2]) -> (DVec2, DVec2) {
    (
        j.vector(Side::Incoming, handles(p[0]).unwrap().1),
        j.vector(Side::Outgoing, handles(p[1]).unwrap().0),
    )
}
fn aligned(a: DVec2, b: DVec2) {
    assert!(
        (a.x * b.y - a.y * b.x).abs() < 2e-5 * (a.length() * b.length()).max(1.0),
        "{a:?} {b:?}"
    );
    assert!(a.dot(b) <= 0.0);
}
#[test]
fn t820_broken_changes_only_requested_handle() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let p = j
        .solve(Mode::Broken, Side::Incoming, Vec2::new(0.8, -2.))
        .unwrap();
    assert_eq!(p[1], j.keys[2].interp);
    assert_eq!(
        handles(p[0]).unwrap().0,
        handles(j.keys[1].interp).unwrap().0
    );
}
#[test]
fn t820_aligned_equal_intervals_preserves_opposite_length() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let old = j.vector(Side::Outgoing, j.handle(Side::Outgoing)).length();
    let (a, b) = vectors(
        &j,
        j.solve(Mode::Aligned, Side::Incoming, Vec2::new(0.8, 0.6))
            .unwrap(),
    );
    aligned(a, b);
    assert!((b.length() - old).abs() < 1e-6);
}
#[test]
fn t820_aligned_unequal_times_and_amplitudes_uses_real_geometry() {
    let (_, j) = fixture([0., 0.2, 2.], [0., 100., 101.]);
    let p = j
        .solve(Mode::Aligned, Side::Outgoing, Vec2::new(0.005, 2.))
        .unwrap();
    let (a, b) = vectors(&j, p);
    aligned(a, b);
    let old = j.vector(Side::Incoming, j.handle(Side::Incoming));
    assert!((a.length() - old.length()).abs() < 0.001);
}
#[test]
fn t820_mirrored_unequal_intervals_and_values() {
    let (_, j) = fixture([0., 0.2, 2.], [0., 100., 101.]);
    let (a, b) = vectors(
        &j,
        j.solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 1.2))
            .unwrap(),
    );
    aligned(a, b);
    assert!((a + b).length() < 0.001);
    assert!((a.x + 0.04).abs() < 1e-6);
    assert!((a.y - 20.0).abs() < 1e-4);
    println!(
        "T820_METRIC unequal mirrored incoming={a:?} outgoing={b:?} residual={}",
        (a + b).length()
    );
}
#[test]
fn t820_flat_pair_has_horizontal_solution_and_preserves_irrelevant_y() {
    let (_, j) = fixture([0., 1., 2.], [10., 10., 10.]);
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 9.))
        .unwrap();
    let (a, b) = vectors(&j, p);
    assert!((a + b).length() < 1e-6);
    assert_eq!(handles(p[1]).unwrap().0.y, 0.25);
}
#[test]
fn t820_flat_opposite_refuses_nonhorizontal_target() {
    let (_, j) = fixture([0., 1., 2.], [0., 10., 10.]);
    assert!(
        j.solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 0.8))
            .is_err()
    );
    assert!(
        j.solve(Mode::Aligned, Side::Incoming, Vec2::new(0.8, 0.8))
            .is_err()
    );
}
#[test]
fn t820_horizontal_handles_link() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let (a, b) = vectors(
        &j,
        j.solve(Mode::Aligned, Side::Incoming, Vec2::new(0.8, 1.))
            .unwrap(),
    );
    aligned(a, b);
    assert_eq!(b.y, 0.);
}
#[test]
fn t820_overshoot_negative_values_and_sign_change_preserved() {
    let (_, j) = fixture([0., 1., 2.], [-10., 0., -20.]);
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 1.5))
        .unwrap();
    let (a, b) = vectors(&j, p);
    assert!((a + b).length() < 1e-5);
    assert_eq!(handles(p[0]).unwrap().1.y, 1.5);
    assert!(handles(p[1]).unwrap().0.y > 0.);
}
#[test]
fn t820_close_values_and_large_finite_values_are_not_corrupted() {
    for v in [[1., 1.00001, 1.00002], [-f32::MAX, 0., f32::MAX]] {
        let (_, j) = fixture([0., 1., 2.], v);
        let p = j
            .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.75, 0.75))
            .unwrap();
        let (a, b) = vectors(&j, p);
        assert!((a + b).length() <= 2e-5 * a.length().max(1.));
    }
}
#[test]
fn t820_x_boundaries_and_impossible_lengths() {
    let (_, j) = fixture([0., 2., 2.25], [0., 1., 2.]);
    assert!(
        j.solve(Mode::Mirrored, Side::Incoming, Vec2::new(0., 1.))
            .is_err()
    );
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    for x in [0., 1., 0.000001, 0.999999] {
        assert!(
            j.solve(Mode::Mirrored, Side::Incoming, Vec2::new(x, 1.))
                .is_ok()
        );
    }
}
#[test]
fn t820_nonfinite_and_out_of_range_inputs_refuse() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    for h in [
        Vec2::new(f32::NAN, 0.),
        Vec2::new(0.5, f32::INFINITY),
        Vec2::new(-0.01, 1.),
        Vec2::new(1.01, 1.),
    ] {
        assert!(j.solve(Mode::Mirrored, Side::Incoming, h).is_err());
    }
}
#[test]
fn t820_f32_overflow_in_opposite_handle_refuses() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 1. + f32::EPSILON]);
    assert!(
        j.solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, f32::MAX))
            .is_err()
    );
}
#[test]
fn t820_zero_driver_policy_is_explicit() {
    let (_, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    assert!(j.solve(Mode::Aligned, Side::Incoming, Vec2::ONE).is_err());
    let (a, b) = vectors(
        &j,
        j.solve(Mode::Mirrored, Side::Incoming, Vec2::ONE).unwrap(),
    );
    assert_eq!(a + b, DVec2::ZERO);
}
#[test]
fn t820_first_last_and_invalid_times_refuse() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    for n in [0, 2, 3] {
        assert!(Junction::read(&d, j.anim, j.addr.clone(), n).is_err());
    }
    for times in [
        [0., 0., 2.],
        [0., 2., 1.],
        [0., f32::NAN, 2.],
        [0., f32::INFINITY, 2.],
    ] {
        if let Timeline::BoneTranslate { keys, .. } = &mut d.animations[j.anim].timelines[0] {
            for (i, k) in keys.iter_mut().enumerate() {
                k.time = times[i];
            }
        }
        assert!(Junction::read(&d, j.anim, j.addr.clone(), 1).is_err());
    }
}
#[test]
fn t820_linear_stepped_and_ambiguous_track_refuse() {
    for i in [Interp::Linear, Interp::Stepped] {
        let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
        if let Timeline::BoneTranslate { keys, .. } = &mut d.animations[j.anim].timelines[0] {
            keys[2].interp = i;
        }
        assert!(Junction::read(&d, j.anim, j.addr, 1).is_err());
    }
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let t = d.animations[j.anim].timelines[0].clone();
    d.animations[j.anim].timelines.push(t);
    assert!(Junction::read(&d, j.anim, j.addr, 1).is_err());
}
#[test]
fn t820_rotation_wrap_junctions_refuse_without_sampler_changes() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let bone = j.timeline.bone().unwrap();
    for v in [180., -180., 359.] {
        d.animations[j.anim].timelines = vec![Timeline::BoneRotate {
            bone,
            keys: vec![
                Key {
                    time: 0.,
                    value: 0.,
                    interp: curve(),
                },
                Key {
                    time: 1.,
                    value: v,
                    interp: curve(),
                },
                Key {
                    time: 2.,
                    value: 0.,
                    interp: curve(),
                },
            ],
        }];
        assert!(
            Junction::read(
                &d,
                j.anim,
                TimelineAddr::Bone {
                    bone,
                    property: BoneProperty::Rotate,
                    axis: None
                },
                1
            )
            .is_err()
        );
    }
}
#[test]
fn t820_native_pair_three_undo_redo_exact_document() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let before = crate::snapshot::snapshot(&d);
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 0.5))
        .unwrap();
    let mut h = History::default();
    h.push(
        j.command(&d, p, None)
            .unwrap()
            .map(|c| Box::new(c) as Box<dyn EditCommand>)
            .unwrap(),
        &mut d,
    );
    let after = crate::snapshot::snapshot(&d);
    assert!(before != after);
    assert_eq!(h.undo_depth(), 1);
    for _ in 0..3 {
        h.undo(&mut d);
        assert!(crate::snapshot::snapshot(&d) == before);
        h.redo(&mut d);
        assert!(crate::snapshot::snapshot(&d) == after);
    }
}
#[test]
fn t820_native_requires_animate_and_noop_has_no_command() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    assert!(
        j.command(&d, [j.keys[1].interp, j.keys[2].interp], None)
            .unwrap()
            .is_none()
    );
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 0.5))
        .unwrap();
    let mut h = History::default();
    assert!(!h.push_in_mode(
        Box::new(j.command(&d, p, None).unwrap().unwrap()),
        &mut d,
        WorkMode::Setup
    ));
    assert!(!h.can_undo());
}
#[test]
fn t820_stale_native_capture_refuses_and_preserves_redo() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 0.5))
        .unwrap();
    if let Timeline::BoneTranslate { keys, .. } = &mut d.animations[j.anim].timelines[0] {
        keys.swap(0, 1);
    }
    assert!(j.command(&d, p, None).is_err());
}
#[test]
fn t820_native_gesture_merges_only_contiguous_same_token() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let mut h = History::default();
    for (token, y) in [(10, 0.5), (10, 0.6), (11, 0.7)] {
        let current = Junction::read(&d, j.anim, j.addr.clone(), 1).unwrap();
        let p = j
            .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, y))
            .unwrap();
        let c = current.command(&d, p, Some(token)).unwrap().unwrap();
        h.push(Box::new(c), &mut d);
    }
    assert_eq!(h.undo_depth(), 2);
}

#[test]
fn t820_outgoing_driver_changes_only_the_join_pair() {
    let (_, j) = fixture([0., 0.2, 2.], [0., 100., 101.]);
    let p = j
        .solve(Mode::Mirrored, Side::Outgoing, Vec2::new(0.05, 2.))
        .unwrap();
    let (a, b) = vectors(&j, p);
    aligned(a, b);
    assert!((a + b).length() < 1e-5);
    assert_eq!(
        handles(p[0]).unwrap().0,
        handles(j.keys[1].interp).unwrap().0
    );
    assert_eq!(
        handles(p[1]).unwrap().1,
        handles(j.keys[2].interp).unwrap().1
    );
}

#[test]
fn t820_alpha_track_links_without_changing_color_values() {
    let (mut d, j) = fixture([0., 1., 2.], [0., 1., 2.]);
    let slot = d.skeleton.add_slot(ankhimate_core::slot::Slot::new(
        "alpha".into(),
        j.timeline.bone().unwrap(),
    ));
    d.animations[j.anim].timelines = vec![Timeline::SlotColor {
        slot,
        keys: (0..3)
            .map(|i| Key {
                time: i as f32,
                value: [0.1, 0.2, 0.3, 0.25 * i as f32],
                interp: curve(),
            })
            .collect(),
    }];
    let j = Junction::read(&d, j.anim, TimelineAddr::SlotColor { slot }, 1).unwrap();
    let p = j
        .solve(Mode::Mirrored, Side::Incoming, Vec2::new(0.8, 0.5))
        .unwrap();
    j.command(&d, p, None).unwrap().unwrap().apply(&mut d);
    let Timeline::SlotColor { keys, .. } = &d.animations[j.anim].timelines[0] else {
        unreachable!()
    };
    for (i, k) in keys.iter().enumerate() {
        assert_eq!(k.value, [0.1, 0.2, 0.3, 0.25 * i as f32]);
        assert_eq!(k.time, i as f32);
    }
}
