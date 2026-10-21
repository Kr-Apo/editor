use super::*;
use crate::{commands::History, snapshot::snapshot};
use ankhimate_core::{
    animation::{Animation, Axis, Interp, Marker},
    ids::{ConstraintId, SlotId},
    skeleton::Bone,
};
fn bone(name: &str, parent: Option<BoneId>) -> Bone {
    Bone {
        name: name.into(),
        parent,
        length: 10.,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}
fn fixture() -> (Document, AnimationId, Vec<BoneId>) {
    let mut d = Document::new();
    let parent = d.skeleton.add_bone(bone("parent", None));
    let child = d.skeleton.add_bone(bone("child", Some(parent)));
    let other = d.skeleton.add_bone(bone("other", None));
    let empty = d.skeleton.add_bone(bone("empty", None));
    let mut a = Animation::new("clip", 2.);
    let keys = vec![
        Key {
            time: 0.25,
            value: -15.,
            interp: Interp::Bezier {
                out_handle: glam::vec2(0.3, -0.5),
                in_handle: glam::vec2(0.7, 1.5),
            },
        },
        Key {
            time: 1.75,
            value: 42.,
            interp: Interp::Stepped,
        },
    ];
    for b in [parent, child, other] {
        a.timelines.push(Timeline::BoneRotate {
            bone: b,
            keys: keys.clone(),
        });
        for axis in Axis::BOTH {
            a.timelines.extend([
                Timeline::BoneTranslate {
                    bone: b,
                    axis,
                    keys: keys.clone(),
                },
                Timeline::BoneScale {
                    bone: b,
                    axis,
                    keys: keys.clone(),
                },
                Timeline::BoneShear {
                    bone: b,
                    axis,
                    keys: keys.clone(),
                },
            ]);
        }
    }
    a.timelines.extend([
        Timeline::SlotVisible {
            slot: SlotId::default(),
            keys: vec![Key::linear(0.25, true)],
        },
        Timeline::SlotColor {
            slot: SlotId::default(),
            keys: vec![Key::linear(0.25, [0.2; 4])],
        },
        Timeline::SlotAttachment {
            slot: SlotId::default(),
            keys: vec![Key::linear(0.25, Some("art".into()))],
        },
        Timeline::DrawOrder {
            keys: vec![Key::linear(0.25, vec![])],
        },
        Timeline::IkMix {
            constraint: ConstraintId::default(),
            keys: keys.clone(),
        },
        Timeline::IkBendDirection {
            constraint: ConstraintId::default(),
            keys: keys.clone(),
        },
        Timeline::IkSoftness {
            constraint: ConstraintId::default(),
            keys: keys.clone(),
        },
        Timeline::TransformConstraintMix {
            constraint: ConstraintId::default(),
            keys: vec![Key::linear(0.25, Default::default())],
        },
        Timeline::PathPosition {
            constraint: ConstraintId::default(),
            keys: keys.clone(),
        },
        Timeline::Deform {
            slot: SlotId::default(),
            attachment: "mesh".into(),
            keys: vec![Key::linear(0.25, vec![glam::Vec2::ONE])],
        },
    ]);
    a.markers.push(Marker::new(0.5, "hit"));
    a.bone_offsets.push((parent, 0.2));
    a.events.push(ankhimate_core::animation::EventKey {
        time: 0.7,
        name: "event".into(),
        int_value: 1,
        float_value: 2.,
        string_value: "payload".into(),
        audio: String::new(),
        volume: 1.,
        balance: 0.,
    });
    let id = d.animations.insert(a.clone());
    a.name = "other clip".into();
    d.animations.insert(a);
    (d, id, vec![parent, child, other, empty])
}
fn apply(d: &mut Document, id: AnimationId, bones: &[BoneId], frames: f64, h: &mut History) {
    let p = PreparedBoneOffset::prepare(d, id, bones, frames).unwrap();
    if let Some(c) = p.into_command(d).unwrap() {
        assert!(h.push_in_mode(c, d, WorkMode::Animate));
    }
}
#[test]
fn t833_exact_selected_channels_payloads_exclusions_and_three_history_cycles() {
    let (mut d, id, b) = fixture();
    let before = snapshot(&d);
    let original = d.clone();
    let selected = &b[..2];
    let p = PreparedBoneOffset::prepare(&d, id, selected, 3.).unwrap();
    assert_eq!(p.summary.tracks, 14);
    assert_eq!(p.summary.keys, 28);
    assert_eq!(p.summary.offset, 0.1);
    assert_eq!(p.summary.before_range, Some((0.25, 1.75)));
    assert_eq!(p.summary.after_range, Some((0.35, 1.85)));
    let mut h = History::default();
    h.push_in_mode(
        p.into_command(&d).unwrap().unwrap(),
        &mut d,
        WorkMode::Animate,
    );
    let after = snapshot(&d);
    assert_eq!(h.undo_depth(), 1);
    let mut expected = original.clone();
    for t in &mut expected.animations[id].timelines {
        if t.bone().is_some_and(|v| selected.contains(&v)) {
            for k in keys_mut(t) {
                k.time += 0.1;
            }
        }
    }
    assert!(snapshot(&d) == snapshot(&expected));
    assert_eq!(
        serde_json::to_vec(&d.animations[id].timelines[14..]).unwrap(),
        serde_json::to_vec(&original.animations[id].timelines[14..]).unwrap()
    );
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert!(snapshot(&d) == before);
        assert!(h.redo(&mut d));
        assert!(snapshot(&d) == after);
    }
}
#[test]
fn t833_parent_selection_never_expands_and_missing_channels_are_allowed() {
    for indices in [vec![0], vec![0, 1], vec![0, 2], vec![1], vec![3]] {
        let (mut d, id, b) = fixture();
        d.animations[id]
            .timelines
            .retain(|t| !matches!(t, Timeline::BoneScale { .. }));
        let before = d.animations[id].clone();
        let chosen: Vec<_> = indices.iter().map(|i| b[*i]).collect();
        let mut h = History::default();
        apply(&mut d, id, &chosen, 3., &mut h);
        for (old, new) in before.timelines.iter().zip(&d.animations[id].timelines) {
            if !old.bone().is_some_and(|v| chosen.contains(&v)) {
                assert_eq!(old, new);
            }
        }
        assert_eq!(h.undo_depth(), usize::from(indices != vec![3]));
    }
}
#[test]
fn t833_frame_and_boundary_matrix_native_f32_formula() {
    for frames in [0., 1., -1., 3., -3., 0.5, -0.25] {
        let (mut d, id, b) = fixture();
        let before = d.animations[id].clone();
        let mut h = History::default();
        apply(&mut d, id, &b[..1], frames, &mut h);
        let offset = (frames / 30.) as f32;
        for (old, new) in before.timelines.iter().zip(&d.animations[id].timelines) {
            if old.bone() == Some(b[0]) {
                assert_eq!(
                    numeric::times(new),
                    numeric::times(old)
                        .iter()
                        .map(|t| (*t + offset).max(0.))
                        .collect::<Vec<_>>()
                );
            }
        }
        assert_eq!(d.animations[id].duration, 2.);
        assert_eq!(h.undo_depth(), usize::from(frames != 0.));
    }
}
#[test]
fn t833_invalid_selection_duplicate_tracks_deleted_bones_no_mutation() {
    let (mut d, id, b) = fixture();
    let before = snapshot(&d);
    for chosen in [vec![], vec![b[0], b[0]], vec![BoneId::default()]] {
        assert!(PreparedBoneOffset::prepare(&d, id, &chosen, 3.).is_err());
        assert!(snapshot(&d) == before);
    }
    let removed = d.skeleton.add_bone(bone("removed", None));
    d.skeleton.bones.remove(removed);
    assert!(PreparedBoneOffset::prepare(&d, id, &[removed], 3.).is_err());
    let duplicate = d.animations[id].timelines[0].clone();
    d.animations[id].timelines.push(duplicate);
    assert!(PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).is_err());
}
#[test]
fn t833_numeric_invalids_and_f32_collision_preserve_redo() {
    for frames in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e100,
        1e-100,
        -300.,
        3e9,
    ] {
        let (mut d, id, b) = fixture();
        let mut h = History::default();
        apply(&mut d, id, &b[..1], 3., &mut h);
        h.undo(&mut d);
        let old = snapshot(&d);
        assert!(
            PreparedBoneOffset::prepare(&d, id, &b[..1], frames).is_err(),
            "{frames}"
        );
        assert!(snapshot(&d) == old);
        assert!(h.can_redo());
    }
    for ts in [
        vec![0., 0.01, 1.],
        vec![1., f32::from_bits(1f32.to_bits() + 1)],
        vec![f32::MAX],
    ] {
        let (mut d, id, b) = fixture();
        d.animations[id].timelines = vec![Timeline::BoneRotate {
            bone: b[0],
            keys: ts.iter().map(|t| Key::linear(*t, 3.)).collect(),
        }];
        let frames = if ts[0] == 0. {
            -3.
        } else if ts[0] == f32::MAX {
            f64::from(f32::MAX) * 30.
        } else {
            3e9
        };
        assert!(PreparedBoneOffset::prepare(&d, id, &b[..1], frames).is_err());
    }
}
#[test]
fn t833_invalid_source_order_nonfinite_duration_and_fps() {
    for ts in [
        vec![f32::NAN],
        vec![f32::INFINITY],
        vec![-1.],
        vec![1., 0.],
        vec![0., 0.],
    ] {
        let (mut d, id, b) = fixture();
        d.animations[id].timelines = vec![Timeline::BoneRotate {
            bone: b[0],
            keys: ts.iter().map(|t| Key::linear(*t, 0.)).collect(),
        }];
        assert!(PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).is_err());
    }
    for duration in [-1., f32::NAN, f32::INFINITY] {
        let (mut d, id, b) = fixture();
        d.animations[id].duration = duration;
        assert!(PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).is_err());
    }
    for fps in [0, 241, u32::MAX] {
        let (mut d, id, b) = fixture();
        d.meta.fps = fps;
        assert!(PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).is_err());
    }
}
#[test]
fn t833_safe_single_clamp_and_outside_keys_reported_duration_fixed() {
    let (mut d, id, b) = fixture();
    let p = PreparedBoneOffset::prepare(&d, id, &b[..1], -9.).unwrap();
    assert_eq!(p.summary.clamped, 7);
    assert!(p.summary.needs_ack());
    let p = PreparedBoneOffset::prepare(&d, id, &b[..1], 30.).unwrap();
    assert_eq!(p.summary.outside_duration, 7);
    assert!(p.summary.needs_ack());
    let mut h = History::default();
    apply(&mut d, id, &b[..1], 30., &mut h);
    assert_eq!(d.animations[id].duration, 2.);
    assert_eq!(
        numeric::times(&d.animations[id].timelines[0]),
        vec![1.25, 2.75]
    );
}
#[test]
fn t833_stale_exact_track_set_payload_and_fps_refused_before_history() {
    for change in 0..5 {
        let (mut d, id, b) = fixture();
        let p = PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).unwrap();
        match change {
            0 => {
                d.animations[id].timelines.remove(0);
            }
            1 => {
                keys_mut(&mut d.animations[id].timelines[0])[0].value = 9.;
            }
            2 => d.meta.fps = 60,
            3 => d.animations[id].duration = 3.,
            _ => {
                d.skeleton.bones.remove(b[0]);
            }
        }
        let before = snapshot(&d);
        assert!(p.into_command(&d).is_err());
        assert!(snapshot(&d) == before);
    }
}
#[test]
fn t833_setup_refusal_noop_and_separate_applies() {
    let (mut d, id, b) = fixture();
    let mut h = History::default();
    let before = snapshot(&d);
    let p = PreparedBoneOffset::prepare(&d, id, &b[..1], 3.).unwrap();
    assert!(!h.push_in_mode(
        p.into_command(&d).unwrap().unwrap(),
        &mut d,
        WorkMode::Setup
    ));
    assert!(snapshot(&d) == before);
    assert!(!h.can_undo());
    assert!(
        PreparedBoneOffset::prepare(&d, id, &b[..1], 0.)
            .unwrap()
            .into_command(&d)
            .unwrap()
            .is_none()
    );
    apply(&mut d, id, &b[..1], 3., &mut h);
    apply(&mut d, id, &b[..1], 3., &mut h);
    assert_eq!(h.undo_depth(), 2);
}
#[test]
fn t833_selected_offset_cpu_stages() {
    for (count, keys) in [(1, 10), (10, 100), (100, 1000)] {
        let mut d = Document::new();
        let mut a = Animation::new("load", 100.);
        let mut bones = vec![];
        for i in 0..count {
            let b = d.skeleton.add_bone(bone(&format!("b{i}"), None));
            bones.push(b);
            a.timelines.push(Timeline::BoneRotate {
                bone: b,
                keys: (0..keys)
                    .map(|k| Key::linear(k as f32 * 0.01, k as f32))
                    .collect(),
            });
        }
        let id = d.animations.insert(a);
        let mut h = History::default();
        let t = std::time::Instant::now();
        let p = PreparedBoneOffset::prepare(&d, id, &bones, 3.).unwrap();
        let prep = t.elapsed();
        let t = std::time::Instant::now();
        let c = p.into_command(&d).unwrap().unwrap();
        let validate = t.elapsed();
        let t = std::time::Instant::now();
        h.push_in_mode(c, &mut d, WorkMode::Animate);
        let apply = t.elapsed();
        let t = std::time::Instant::now();
        h.undo(&mut d);
        let undo = t.elapsed();
        let t = std::time::Instant::now();
        h.redo(&mut d);
        eprintln!(
            "T833 CPU {count} bones x {keys} keys: prepare={prep:?} validate={validate:?} apply={apply:?} undo={undo:?} redo={:?}",
            t.elapsed()
        );
    }
}
