use super::*;
use ankhimate_document::commands::key_cmds::TimelineAddr;
fn channels() -> Vec<(BoneProperty, Option<Axis>, bool)> {
    vec![
        (BoneProperty::Translate, Some(Axis::X), true),
        (BoneProperty::Translate, Some(Axis::Y), false),
        (BoneProperty::Rotate, None, true),
        (BoneProperty::Scale, Some(Axis::X), false),
        (BoneProperty::Scale, Some(Axis::Y), false),
        (BoneProperty::Shear, Some(Axis::X), true),
        (BoneProperty::Shear, Some(Axis::Y), true),
    ]
}
fn block() -> KeyClip {
    KeyClip {
        keys: channels()
            .iter()
            .flat_map(|(property, axis, _)| {
                [-240., 0., 123.5]
                    .iter()
                    .enumerate()
                    .map(move |(i, value)| CopiedKey {
                        track: NamedTrack {
                            name: "source.left".into(),
                            kind: TrackKind::Bone(*property, *axis),
                        },
                        key: Key {
                            time: 0.25 + 0.5 * i as f32,
                            value: KeyData::Scalar(*value),
                            interp: [Interp::Linear, Interp::Stepped, bezier()][i],
                        },
                    })
            })
            .collect(),
    }
}
fn targets(bone: BoneId) -> Vec<TimelineAddr> {
    channels()
        .into_iter()
        .map(|(property, axis, _)| TimelineAddr::Bone {
            bone,
            property,
            axis,
        })
        .collect()
}
#[test]
fn all_seven_channels_negate_only_defined_offsets_and_keep_every_handle_time_and_order() {
    let original = block();
    let mirrored = original.mirrored_offsets().unwrap();
    assert_eq!(original.keys.len(), 21);
    for (index, (a, b)) in original.keys.iter().zip(&mirrored.keys).enumerate() {
        assert_eq!(a.track, b.track);
        assert_eq!(a.key.time, b.key.time);
        assert_eq!(a.key.interp, b.key.interp);
        let (KeyData::Scalar(a), KeyData::Scalar(b)) = (&a.key.value, &b.key.value) else {
            panic!()
        };
        let expected = if channels()[index / 3].2 { -*a } else { *a };
        assert_eq!(
            b.to_bits(),
            expected.to_bits(),
            "channel {index}, including signed zero"
        );
    }
    assert_eq!(original, block(), "source clipboard is not mutated");
    assert_eq!(mirrored.mirrored_offsets().unwrap(), original);
}
#[test]
fn mirror_uses_the_same_mapping_replacement_and_duration_transaction_as_normal_paste() {
    let (mut doc, anim, bone, _) = fixture();
    let original = block();
    let mapped = targets(bone);
    let before = format!("{doc:?}");
    let mut history = History::default();
    let command = PasteKeys::new_mirrored_offsets(&doc, anim, &original, 1.25, &mapped).unwrap();
    assert!(history.push_in_mode(Box::new(command), &mut doc, WorkMode::Animate));
    assert_eq!(history.undo_depth(), 1);
    assert_eq!(doc.animations[anim].duration, 2.25);
    let after = format!("{doc:?}");
    for _ in 0..3 {
        assert!(history.undo(&mut doc));
        assert_eq!(format!("{doc:?}"), before);
        assert!(history.redo(&mut doc));
        assert_eq!(format!("{doc:?}"), after);
    }
    // A normal paste of the transformed owned block is exactly the same edit.
    history.undo(&mut doc);
    let cmd = PasteKeys::new(
        &doc,
        anim,
        &original.mirrored_offsets().unwrap(),
        1.25,
        &mapped,
    )
    .unwrap();
    history.push(Box::new(cmd), &mut doc);
    assert_eq!(format!("{doc:?}"), after);
}
#[test]
fn mixed_nonbone_types_bad_axes_and_invalid_values_times_or_curves_are_rejected() {
    for case in [
        "color",
        "visible",
        "attachment",
        "axis",
        "value",
        "nan-time",
        "negative-time",
        "handle-x",
        "handle-nan",
        "empty",
    ] {
        let (doc, anim, bone, _) = fixture();
        let mut clip = block();
        let k = clip.keys.last_mut().unwrap();
        match case {
            "color" => {
                k.track.kind = TrackKind::Color;
                k.key.value = KeyData::Color([1.; 4]);
            }
            "visible" => {
                k.track.kind = TrackKind::Visible;
                k.key.value = KeyData::Visible(true);
            }
            "attachment" => {
                k.track.kind = TrackKind::Attachment;
                k.key.value = KeyData::Attachment(None);
            }
            "axis" => k.track.kind = TrackKind::Bone(BoneProperty::Rotate, Some(Axis::X)),
            "value" => k.key.value = KeyData::Scalar(f32::INFINITY),
            "nan-time" => k.key.time = f32::NAN,
            "negative-time" => k.key.time = -1.,
            "handle-x" => {
                k.key.interp = Interp::Bezier {
                    out_handle: glam::vec2(1.1, 0.),
                    in_handle: glam::vec2(0.8, 1.2),
                }
            }
            "handle-nan" => {
                k.key.interp = Interp::Bezier {
                    out_handle: glam::vec2(0.2, f32::NAN),
                    in_handle: glam::vec2(0.8, 1.2),
                }
            }
            "empty" => clip.keys.clear(),
            _ => unreachable!(),
        }
        let before = format!("{doc:?}");
        assert!(
            PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &targets(bone)).is_err(),
            "{case}"
        );
        assert_eq!(format!("{doc:?}"), before);
    }
}
#[test]
fn explicit_destinations_are_mandatory_and_collisions_wrong_types_and_bad_playheads_refuse() {
    let (doc, anim, bone, slot) = fixture();
    let clip = block();
    assert!(PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &[]).is_err());
    let mut mapped = targets(bone);
    mapped[1] = mapped[0].clone();
    assert!(PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &mapped).is_err());
    mapped = targets(bone);
    mapped[0] = TimelineAddr::SlotColor { slot };
    assert!(PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &mapped).is_err());
    for time in [f32::NAN, f32::INFINITY, -1., f32::MAX] {
        assert!(
            PasteKeys::new_mirrored_offsets(&doc, anim, &clip, time, &targets(bone)).is_err(),
            "invalid/overflowed/collapsed time {time}"
        );
    }
}
#[test]
fn absent_or_ambiguous_destinations_and_malformed_destination_tracks_refuse_before_mutation() {
    for case in ["missing", "duplicate-name", "duplicate-track", "bad-times"] {
        let (mut doc, anim, bone, _) = fixture();
        let clip = block();
        let mapped = targets(bone);
        match case {
            "missing" => {
                doc.skeleton.remove_bone(bone);
            }
            "duplicate-name" => {
                let b = doc.skeleton.bones[bone].clone();
                let duplicate = doc.skeleton.add_bone(b);
                doc.skeleton.bones[duplicate].name = "arm".into();
            }
            "duplicate-track" => {
                let t = doc.animations[anim].timelines[0].clone();
                doc.animations[anim].timelines.push(t);
            }
            "bad-times" => {
                if let Timeline::BoneRotate { keys, .. } = &mut doc.animations[anim].timelines[0] {
                    keys[1].time = keys[0].time;
                }
            }
            _ => unreachable!(),
        }
        let before = format!("{doc:?}");
        assert!(
            PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &mapped).is_err(),
            "{case}"
        );
        assert_eq!(format!("{doc:?}"), before);
    }
}
#[test]
fn asymmetric_setups_do_not_change_offsets_or_promise_absolute_pose_reflection() {
    let (mut doc, anim, source, _) = fixture();
    let target = doc.skeleton.add_bone(Bone {
        name: "unrelated_destination".into(),
        local_transform: Transform {
            position: glam::vec2(100., -30.),
            rotation: 0.9,
            scale: glam::vec2(2., 3.),
            ..Default::default()
        },
        ..doc.skeleton.bones[source].clone()
    });
    let source_setup = doc.skeleton.bones[source].local_transform;
    let target_setup = doc.skeleton.bones[target].local_transform;
    let clip = KeyClip {
        keys: vec![CopiedKey {
            track: NamedTrack {
                name: "arbitrary_source_name".into(),
                kind: TrackKind::Bone(BoneProperty::Translate, Some(Axis::X)),
            },
            key: Key::linear(0., KeyData::Scalar(12.)),
        }],
    };
    let mut h = History::default();
    h.push(
        Box::new(
            PasteKeys::new_mirrored_offsets(&doc, anim, &clip, 0., &[targets(target)[0].clone()])
                .unwrap(),
        ),
        &mut doc,
    );
    let mut pose = ankhimate_core::pose::Pose::default();
    ankhimate_core::pose::evaluate(&doc.skeleton, &[(&doc.animations[anim], 0., 1.)], &mut pose);
    assert_eq!(pose.locals[target].position.x, 88.);
    assert_ne!(
        pose.locals[target].position.x,
        -(source_setup.position.x + 12.)
    );
    assert_eq!(doc.skeleton.bones[source].local_transform, source_setup);
    assert_eq!(doc.skeleton.bones[target].local_transform, target_setup);
}
#[test]
fn rotation_raw_values_and_shortest_arc_sampling_are_both_preserved_at_180_boundaries() {
    use ankhimate_core::animation::sample_angle_degrees;
    for (a, b) in [
        (0., 180.),
        (0., -180.),
        (179., -179.),
        (-179., 179.),
        (180., -180.),
        (-180., 180.),
        (179.999, -179.999),
        (540., -540.),
    ] {
        for interp in [Interp::Linear, Interp::Stepped, bezier()] {
            let clip = KeyClip {
                keys: vec![
                    CopiedKey {
                        track: NamedTrack {
                            name: "turn".into(),
                            kind: TrackKind::Bone(BoneProperty::Rotate, None),
                        },
                        key: Key::linear(0., KeyData::Scalar(a)),
                    },
                    CopiedKey {
                        track: NamedTrack {
                            name: "turn".into(),
                            kind: TrackKind::Bone(BoneProperty::Rotate, None),
                        },
                        key: Key {
                            time: 1.,
                            value: KeyData::Scalar(b),
                            interp,
                        },
                    },
                ],
            };
            let mirrored = clip.mirrored_offsets().unwrap();
            let scalar = |clip: &KeyClip| {
                clip.keys
                    .iter()
                    .map(|k| Key {
                        time: k.key.time,
                        value: match k.key.value {
                            KeyData::Scalar(v) => v,
                            _ => panic!(),
                        },
                        interp: k.key.interp,
                    })
                    .collect::<Vec<_>>()
            };
            let from = scalar(&clip);
            let to = scalar(&mirrored);
            assert_eq!(to[0].value.to_bits(), (-a).to_bits());
            assert_eq!(to[1].value.to_bits(), (-b).to_bits());
            let mut samples = vec![];
            for time in [0., 0.25, 0.5, 0.75, 1.] {
                let x = sample_angle_degrees(&from, time, &mut 0).unwrap();
                let y = sample_angle_degrees(&to, time, &mut 0).unwrap();
                assert!(
                    (x + y).abs() <= 0.0001,
                    "{a}->{b}, {interp:?}, t={time}: {x} vs {y}"
                );
                samples.push((time, x, y));
            }
            if interp == Interp::Linear {
                eprintln!("angles {a}->{b}, source/mirrored samples: {samples:?}");
            }
        }
    }
}
#[test]
fn scale_translation_y_and_overshoot_samples_keep_their_original_curve_contract() {
    use ankhimate_core::animation::sample;
    for (property, axis, sign) in channels() {
        let track = NamedTrack {
            name: "channel".into(),
            kind: TrackKind::Bone(property, axis),
        };
        let original = KeyClip {
            keys: vec![
                CopiedKey {
                    track: track.clone(),
                    key: Key::linear(0., KeyData::Scalar(10.)),
                },
                CopiedKey {
                    track,
                    key: Key {
                        time: 1.,
                        value: KeyData::Scalar(30.),
                        interp: bezier(),
                    },
                },
            ],
        };
        let mirrored = original.mirrored_offsets().unwrap();
        let scalar = |clip: &KeyClip| {
            clip.keys
                .iter()
                .map(|k| Key {
                    time: k.key.time,
                    value: match k.key.value {
                        KeyData::Scalar(v) => v,
                        _ => panic!(),
                    },
                    interp: k.key.interp,
                })
                .collect::<Vec<_>>()
        };
        let mut saw_overshoot = false;
        for time in [0.05, 0.25, 0.5, 0.75, 0.95] {
            let x = sample(&scalar(&original), time, &mut 0).unwrap();
            let y = sample(&scalar(&mirrored), time, &mut 0).unwrap();
            assert!((y - if sign { -x } else { x }).abs() < 0.0001);
            saw_overshoot |= !(10.0..=30.0).contains(&x);
        }
        assert!(saw_overshoot, "the fixture must exercise real overshoot");
        assert_eq!(mirrored.keys[1].key.interp, bezier());
    }
}
