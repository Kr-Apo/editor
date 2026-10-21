use super::*;
use crate::{commands::EditCommand, doc::Document};
use ankhimate_core::{
    animation::{Axis, Interp, Key, Marker, sample},
    ids::{BoneId, ConstraintId, SlotId},
};
fn clip(ts: &[f32]) -> Animation {
    let mut a = Animation::new("numeric", 2.0);
    a.timelines.push(Timeline::BoneRotate {
        bone: BoneId::default(),
        keys: ts
            .iter()
            .enumerate()
            .map(|(i, t)| Key::linear(*t, i as f32 * 10.))
            .collect(),
    });
    a
}
fn prepared(a: &Animation, f: f64, o: f64) -> PreparedRetime {
    RetimeAnimation::prepare_numeric(AnimationId::default(), a, f, o).unwrap()
}
fn applied(a: &Animation, p: PreparedRetime) -> Animation {
    let mut d = Document::new();
    let id = d.animations.insert(a.clone());
    let mut command = p.command;
    command.anim = id;
    command.apply(&mut d);
    d.animations[id].clone()
}
#[test]
fn t830_numeric_factors_offsets_and_duration_match_native() {
    for f in [1., 0.5, 2., 1.25, 0.001] {
        for o in [0., 0.1, -0.1] {
            if f == 0.001 && o < 0. {
                continue;
            }
            let a = clip(&[0.25, 0.5, 1.75]);
            let p = prepared(&a, f, o);
            let summary = p.summary.clone();
            let after = applied(&a, p);
            let expected: Vec<_> = [0.25, 0.5, 1.75]
                .map(|t| (t * f as f32 + o as f32).max(0.))
                .into();
            assert_eq!(times(&after.timelines[0]), expected);
            assert_eq!(after.duration, if o == 0. { 2. * f as f32 } else { 2. });
            assert_eq!(summary.duration_after, after.duration);
            assert_eq!(summary.after_range, Some((expected[0], expected[2])));
        }
    }
}
#[test]
fn t830_numeric_refuses_invalid_parameters_and_times() {
    let a = clip(&[0., 1.]);
    for f in [0., -1., f64::NAN, f64::INFINITY, 1e100, 1e-100] {
        assert!(RetimeAnimation::prepare_numeric(AnimationId::default(), &a, f, 0.).is_err());
    }
    for o in [f64::NAN, f64::INFINITY, 1e100, 1e-100] {
        assert!(RetimeAnimation::prepare_numeric(AnimationId::default(), &a, 1., o).is_err());
    }
    for ts in [
        vec![f32::NAN],
        vec![f32::INFINITY],
        vec![-0.1],
        vec![1., 0.],
        vec![0., 0.],
    ] {
        assert!(
            RetimeAnimation::prepare_numeric(AnimationId::default(), &clip(&ts), 2., 0.).is_err()
        );
    }
    for duration in [-1., f32::NAN, f32::INFINITY] {
        let mut a = a.clone();
        a.duration = duration;
        assert!(RetimeAnimation::prepare_numeric(AnimationId::default(), &a, 1., 0.).is_err());
    }
}
#[test]
fn t830_numeric_collisions_clamp_and_outside_duration() {
    for (ts, f, o) in [
        (vec![0., 0.1, 1.], 1., -0.2),
        (vec![1., f32::from_bits(1f32.to_bits() + 1)], 1., 1e8),
    ] {
        let e = RetimeAnimation::prepare_numeric(AnimationId::default(), &clip(&ts), f, o)
            .err()
            .unwrap();
        assert!(e.contains("collision"), "{e}");
    }
    let a = clip(&[0.1, 1.]);
    let p = prepared(&a, 1., -0.2);
    assert_eq!(p.summary.clamped, 1);
    assert_eq!(times(&applied(&a, p).timelines[0]), vec![0., 0.8]);
    let p = prepared(&clip(&[0., 2.]), 1., 0.1);
    assert_eq!(p.summary.outside_duration, 1);
}
#[test]
fn t830_numeric_f32_neighbours_limits_and_zero_duration() {
    let a = clip(&[1., f32::from_bits(1f32.to_bits() + 1)]);
    assert!(prepared(&a, 2., 0.).summary.changed_keys == 2);
    assert!(
        RetimeAnimation::prepare_numeric(AnimationId::default(), &clip(&[f32::MAX]), 2., 0.)
            .is_err()
    );
    let mut a = clip(&[]);
    a.duration = f32::MAX;
    assert!(RetimeAnimation::prepare_numeric(AnimationId::default(), &a, 2., 0.).is_err());
    a.duration = 0.;
    assert_eq!(prepared(&a, 2., 0.).summary.duration_after, 0.);
    a.timelines = clip(&[0.5]).timelines;
    let p = prepared(&a, 0.5, 0.);
    assert_eq!(p.summary.outside_duration, 1);
}
#[test]
fn t830_numeric_preserves_all_values_curves_and_native_scope() {
    let b = BoneId::default();
    let s = SlotId::default();
    let c = ConstraintId::default();
    let scalar = vec![
        Key {
            time: 0.25,
            value: 0.3,
            interp: Interp::Bezier {
                out_handle: glam::vec2(0.2, -0.4),
                in_handle: glam::vec2(0.8, 1.4),
            },
        },
        Key {
            time: 1.25,
            value: 0.7,
            interp: Interp::Stepped,
        },
    ];
    let mut a = Animation::new("all", 2.);
    a.timelines = vec![
        Timeline::BoneTranslate {
            bone: b,
            axis: Axis::X,
            keys: scalar.clone(),
        },
        Timeline::BoneRotate {
            bone: b,
            keys: scalar.clone(),
        },
        Timeline::BoneScale {
            bone: b,
            axis: Axis::Y,
            keys: scalar.clone(),
        },
        Timeline::BoneShear {
            bone: b,
            axis: Axis::X,
            keys: scalar.clone(),
        },
        Timeline::SlotVisible {
            slot: s,
            keys: vec![Key::linear(0.25, true)],
        },
        Timeline::SlotColor {
            slot: s,
            keys: vec![Key::linear(0.25, [0.1, 0.2, 0.3, 0.4])],
        },
        Timeline::SlotAttachment {
            slot: s,
            keys: vec![Key::linear(0.25, Some("art".into()))],
        },
        Timeline::DrawOrder {
            keys: vec![Key::linear(0.25, vec![(s, 1)])],
        },
        Timeline::IkMix {
            constraint: c,
            keys: scalar.clone(),
        },
        Timeline::IkBendDirection {
            constraint: c,
            keys: scalar.clone(),
        },
        Timeline::IkSoftness {
            constraint: c,
            keys: scalar.clone(),
        },
        Timeline::TransformConstraintMix {
            constraint: c,
            keys: vec![Key::linear(0.25, Default::default())],
        },
        Timeline::Deform {
            slot: s,
            attachment: "mesh".into(),
            keys: vec![Key::linear(0.25, vec![glam::vec2(2., 3.)])],
        },
    ];
    a.events.push(ankhimate_core::animation::EventKey {
        time: 0.75,
        name: "hit".into(),
        int_value: 1,
        float_value: 0.25,
        string_value: "payload".into(),
        audio: String::new(),
        volume: 1.,
        balance: 0.,
    });
    a.markers.push(Marker::new(0.5, "impact"));
    a.bone_offsets.push((b, 0.1));
    let p = prepared(&a, 2., 0.);
    assert_eq!(p.summary.tracks, 13);
    assert_eq!(p.summary.keys, 20);
    let after = applied(&a, p);
    let mut expected = a.clone();
    super::super::retime_clip(&mut expected, 2., 0.);
    assert_eq!(after, expected);
    // Restore only time leaves byte-identical serialized payloads, including curve modes/handles.
    let original = serde_json::to_value(&a).unwrap();
    let mut changed = serde_json::to_value(&after).unwrap();
    for (old, new) in original["timelines"]
        .as_array()
        .unwrap()
        .iter()
        .zip(changed["timelines"].as_array_mut().unwrap())
    {
        let old = old.as_object().unwrap().values().next().unwrap();
        let new = new.as_object_mut().unwrap().values_mut().next().unwrap();
        for (a, b) in old["keys"]
            .as_array()
            .unwrap()
            .iter()
            .zip(new["keys"].as_array_mut().unwrap())
        {
            b["time"] = a["time"].clone();
        }
    }
    changed["duration"] = original["duration"].clone();
    assert_eq!(original, changed);
}
#[test]
fn t830_numeric_sampling_is_preserved_at_corresponding_times() {
    for interp in [
        Interp::Linear,
        Interp::Stepped,
        Interp::Bezier {
            out_handle: glam::vec2(0.3, -0.2),
            in_handle: glam::vec2(0.7, 1.2),
        },
    ] {
        let mut a = clip(&[0.25, 1.25]);
        if let Timeline::BoneRotate { keys, .. } = &mut a.timelines[0] {
            keys[1].interp = interp;
        }
        let after = applied(&a, prepared(&a, 1.25, 0.125));
        if let (Timeline::BoneRotate { keys: a, .. }, Timeline::BoneRotate { keys: b, .. }) =
            (&a.timelines[0], &after.timelines[0])
        {
            for i in 0..=100 {
                let t = 0.25 + i as f32 / 100.;
                let x = sample(a, t, &mut 0).unwrap();
                let y = sample(b, t * 1.25 + 0.125, &mut 0).unwrap();
                assert!((x - y).abs() < 0.0001, "{x} {y}");
            }
        }
    }
}
#[test]
fn t830_numeric_legacy_constructors_and_exact_three_cycle_undo() {
    let mut d = Document::new();
    let a = clip(&[0.1, 1.]);
    let id = d.animations.insert(a.clone());
    let mut cmd = RetimeAnimation::offset(id, -0.2);
    cmd.apply(&mut d);
    assert_eq!(d.animations[id].duration, 2.);
    cmd.revert(&mut d);
    assert_eq!(d.animations[id], a);
    let mut cmd = RetimeAnimation::scaled(id, 0.);
    cmd.apply(&mut d);
    assert_eq!(d.animations[id].duration, 0.02);
    cmd.revert(&mut d);
    let mut cmd = RetimeAnimation::prepare_numeric(id, &a, 1.25, -0.2)
        .unwrap()
        .command;
    cmd.apply(&mut d);
    let after = d.animations[id].clone();
    for _ in 0..3 {
        cmd.revert(&mut d);
        assert_eq!(d.animations[id], a);
        cmd.apply(&mut d);
        assert_eq!(d.animations[id], after);
    }
}
