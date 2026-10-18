use ankhimate_core::{
    animation::{Animation, Axis, Interp, Key, Timeline},
    attachment::Attachment,
    ids::{AnimationId, BoneId, SlotId},
    math::Transform,
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::{
    Document, WorkMode,
    clipboard::{
        KeyClip,
        keys::{CopiedKey, KeyData, NamedTrack, TrackKind, address},
    },
    commands::{
        History,
        key_cmds::{BoneProperty, KeyRef, PasteKeys},
    },
};

fn fixture() -> (Document, AnimationId, BoneId, SlotId) {
    let mut doc = Document::new();
    let bone = doc.skeleton.add_bone(Bone {
        name: "arm".into(),
        parent: None,
        length: 5.0,
        local_transform: Transform {
            position: glam::vec2(7.0, 9.0),
            rotation: 0.3,
            ..Default::default()
        },
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = doc.skeleton.add_slot(Slot::new("hand".into(), bone));
    let anim = doc.animations.insert(Animation::new("source", 1.0));
    doc.animations[anim].timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![
            Key::linear(0.25, 10.0),
            Key {
                time: 0.75,
                value: 45.0,
                interp: bezier(),
            },
        ],
    });
    (doc, anim, bone, slot)
}
fn bezier() -> Interp {
    Interp::Bezier {
        out_handle: glam::vec2(0.2, -0.5),
        in_handle: glam::vec2(0.8, 1.4),
    }
}
fn refs(doc: &Document, anim: AnimationId) -> Vec<KeyRef> {
    doc.animations[anim]
        .timelines
        .iter()
        .flat_map(|t| {
            let addr = address(t).unwrap();
            let len = match t {
                Timeline::BoneRotate { keys, .. }
                | Timeline::BoneTranslate { keys, .. }
                | Timeline::BoneScale { keys, .. }
                | Timeline::BoneShear { keys, .. } => keys.len(),
                Timeline::SlotColor { keys, .. } => keys.len(),
                Timeline::SlotVisible { keys, .. } => keys.len(),
                Timeline::SlotAttachment { keys, .. } => keys.len(),
                _ => 0,
            };
            (0..len).map(move |index| KeyRef {
                addr: addr.clone(),
                index,
            })
        })
        .collect()
}
fn json(doc: &Document) -> String {
    ankhimate_formats::to_json(&doc.as_project_ref()).unwrap()
}
fn paste(
    doc: &mut Document,
    anim: AnimationId,
    clip: &KeyClip,
    time: f32,
    targets: &[ankhimate_document::commands::key_cmds::TimelineAddr],
    history: &mut History,
) {
    let cmd = PasteKeys::new(doc, anim, clip, time, targets).unwrap();
    assert!(history.push_in_mode(Box::new(cmd), doc, WorkMode::Animate));
}

#[test]
fn single_key_capture_is_immediate_by_value_and_does_not_edit() {
    let (mut doc, anim, _, _) = fixture();
    let before = json(&doc);
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)[1..]).unwrap();
    assert_eq!(json(&doc), before);
    assert_eq!(clip.keys[0].key.interp, bezier());
    doc.animations.remove(anim);
    let target = doc.animations.insert(Animation::new("destination", 1.0));
    let mut history = History::default();
    paste(&mut doc, target, &clip, 2.0, &[], &mut history);
    assert!(
        matches!(&doc.animations[target].timelines[0],Timeline::BoneRotate {keys,..} if keys==&vec![Key {time:2.0,value:45.0,interp:bezier()}])
    );
}

#[test]
fn block_relative_times_curves_replacement_and_repeated_pastes() {
    let (mut doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let mut history = History::default();
    paste(&mut doc, anim, &clip, 0.75, &[], &mut history);
    paste(&mut doc, anim, &clip, 0.75, &[], &mut history);
    let Timeline::BoneRotate { keys, .. } = &doc.animations[anim].timelines[0] else {
        panic!()
    };
    assert_eq!(keys.len(), 3);
    assert_eq!(keys[1], Key::linear(0.75, 10.0));
    assert_eq!(
        keys[2],
        Key {
            time: 1.25,
            value: 45.0,
            interp: bezier()
        }
    );
    assert_eq!(doc.animations[anim].duration, 1.25);
    assert_eq!(history.undo_depth(), 2);
}

#[test]
fn ten_editable_tracks_preserve_complete_typed_values_axes_and_interpolation() {
    let (mut doc, anim, bone, slot) = fixture();
    doc.skeleton.skins[doc.skeleton.default_skin].set(
        slot,
        "hand \"\\ 漢",
        Attachment::Point(Default::default()),
    );
    for axis in [Axis::X, Axis::Y] {
        doc.animations[anim].timelines.extend([
            Timeline::BoneTranslate {
                bone,
                axis,
                keys: vec![Key::stepped(0.5, 3.0)],
            },
            Timeline::BoneScale {
                bone,
                axis,
                keys: vec![Key {
                    time: 0.5,
                    value: 2.0,
                    interp: bezier(),
                }],
            },
            Timeline::BoneShear {
                bone,
                axis,
                keys: vec![Key::linear(0.5, 17.0)],
            },
        ]);
    }
    doc.animations[anim].timelines.extend([
        Timeline::SlotColor {
            slot,
            keys: vec![Key {
                time: 0.5,
                value: [0.1, 0.2, 0.3, 0.4],
                interp: bezier(),
            }],
        },
        Timeline::SlotVisible {
            slot,
            keys: vec![Key {
                time: 0.5,
                value: false,
                interp: bezier(),
            }],
        },
        Timeline::SlotAttachment {
            slot,
            keys: vec![
                Key {
                    time: 0.5,
                    value: Some("hand \"\\ 漢".into()),
                    interp: Interp::Linear,
                },
                Key::stepped(0.75, None),
            ],
        },
    ]);
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let target = doc.animations.insert(Animation::new("target", 1.0));
    let mut history = History::default();
    paste(&mut doc, target, &clip, 2.25, &[], &mut history);
    let copied = KeyClip::capture(&doc, target, &refs(&doc, target)).unwrap();
    let mut expected = clip.clone();
    for k in &mut expected.keys {
        k.key.time += 2.0;
    }
    assert_eq!(copied, expected);
    assert_eq!(doc.animations[target].timelines.len(), 10);
}

#[test]
fn explicit_destination_rows_map_bones_and_keep_property_and_axis() {
    let (mut doc, anim, bone, _) = fixture();
    doc.animations[anim]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: Axis::Y,
            keys: vec![Key::stepped(0.5, 7.0)],
        });
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let mut second = doc.skeleton.bones[bone].clone();
    second.name = "other".into();
    let other = doc.skeleton.add_bone(second);
    let target = doc.animations.insert(Animation::new("target", 1.0));
    let targets = [
        ankhimate_document::commands::key_cmds::TimelineAddr::Bone {
            bone: other,
            property: BoneProperty::Rotate,
            axis: None,
        },
        ankhimate_document::commands::key_cmds::TimelineAddr::Bone {
            bone: other,
            property: BoneProperty::Translate,
            axis: Some(Axis::Y),
        },
    ];
    let mut history = History::default();
    paste(&mut doc, target, &clip, 1.0, &targets, &mut history);
    assert!(
        doc.animations[target]
            .timelines
            .iter()
            .all(|t| t.bone() == Some(other))
    );
    assert_eq!(
        doc.skeleton.bones[bone].local_transform.position,
        glam::vec2(7.0, 9.0)
    );
    let mut invalid = targets.clone();
    invalid[1] = ankhimate_document::commands::key_cmds::TimelineAddr::Bone {
        bone: other,
        property: BoneProperty::Translate,
        axis: Some(Axis::X),
    };
    let before = json(&doc);
    assert!(PasteKeys::new(&doc, target, &clip, 2.0, &invalid).is_err());
    assert_eq!(json(&doc), before);
}

#[test]
fn paste_is_one_undo_and_three_roundtrips_restore_the_complete_document_and_duration() {
    let (mut doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let before = json(&doc);
    let mut history = History::default();
    paste(&mut doc, anim, &clip, 4.0, &[], &mut history);
    assert_eq!(history.undo_depth(), 1);
    let after = json(&doc);
    for _ in 0..3 {
        assert!(history.undo(&mut doc));
        assert_eq!(json(&doc), before);
        assert!(history.redo(&mut doc));
        assert_eq!(json(&doc), after);
    }
    let loaded = ankhimate_formats::from_json(&json(&doc)).unwrap();
    assert!(loaded.report.is_clean());
    assert_eq!(json(&Document::from_loaded(loaded)), after);
}

#[test]
fn invalid_copy_indices_targets_and_ambiguous_names_are_refused() {
    let (mut doc, anim, bone, _) = fixture();
    let selection = refs(&doc, anim);
    let before = json(&doc);
    let mut invalid = selection.clone();
    invalid[0].index = 999;
    assert!(KeyClip::capture(&doc, anim, &invalid).is_err());
    assert_eq!(json(&doc), before);
    let duplicate = doc.skeleton.bones[bone].clone();
    let duplicate = doc.skeleton.add_bone(duplicate);
    doc.skeleton.bones[duplicate].name = "arm".into();
    assert!(
        KeyClip::capture(&doc, anim, &selection)
            .unwrap_err()
            .contains("ambiguous")
    );
    doc.animations.remove(anim);
    assert!(KeyClip::capture(&doc, anim, &selection).is_err());
}

#[test]
fn deleting_and_recreating_a_target_resolves_by_name_not_recycled_id() {
    let (mut doc, anim, bone, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let original = doc.skeleton.bones.remove(bone).unwrap();
    let replacement = doc.skeleton.add_bone(original);
    assert_ne!(bone, replacement);
    let target = doc.animations.insert(Animation::new("target", 1.0));
    let mut history = History::default();
    paste(&mut doc, target, &clip, 0.0, &[], &mut history);
    assert_eq!(
        doc.animations[target].timelines[0].bone(),
        Some(replacement)
    );
}

#[test]
fn nonrepresentable_block_or_missing_name_refuses_without_partial_modification() {
    let (doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let before = json(&doc);
    for malformed in [
        {
            let mut c = clip.clone();
            c.keys[1].key.value = KeyData::Color([1.0; 4]);
            c
        },
        {
            let mut c = clip.clone();
            c.keys[1].track.name = "missing".into();
            c
        },
        {
            let mut c = clip.clone();
            c.keys[1].key.time = f32::NAN;
            c
        },
        {
            let mut c = clip.clone();
            c.keys[1].key.interp = Interp::Bezier {
                out_handle: glam::vec2(2.0, 0.0),
                in_handle: glam::Vec2::ONE,
            };
            c
        },
    ] {
        assert!(PasteKeys::new(&doc, anim, &malformed, 0.0, &[]).is_err());
        assert_eq!(json(&doc), before);
    }
}

#[test]
fn negative_nonfinite_and_collapsing_paste_times_refuse_without_clamping() {
    let (doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    for time in [-1.0, f32::NAN, f32::INFINITY, f32::MAX] {
        assert!(PasteKeys::new(&doc, anim, &clip, time, &[]).is_err());
    }
    let mut negative = clip;
    negative.keys[0].key.time = -0.25;
    assert!(PasteKeys::new(&doc, anim, &negative, 0.0, &[]).is_err());
}

#[test]
fn duplicate_destination_timelines_or_key_times_refuse_atomically() {
    let (mut doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let duplicate = doc.animations[anim].timelines[0].clone();
    doc.animations[anim].timelines.push(duplicate);
    assert!(PasteKeys::new(&doc, anim, &clip, 0.0, &[]).is_err());
    doc.animations[anim].timelines.pop();
    if let Timeline::BoneRotate { keys, .. } = &mut doc.animations[anim].timelines[0] {
        keys.push(keys[1].clone());
    }
    assert!(PasteKeys::new(&doc, anim, &clip, 0.0, &[]).is_err());
}

#[test]
fn attachment_names_are_validated_before_paste_and_none_is_preserved() {
    let (doc, anim, _, _) = fixture();
    let clip = KeyClip {
        keys: vec![CopiedKey {
            track: NamedTrack {
                name: "hand".into(),
                kind: TrackKind::Attachment,
            },
            key: Key::stepped(0.0, KeyData::Attachment(Some("unknown".into()))),
        }],
    };
    assert!(PasteKeys::new(&doc, anim, &clip, 0.0, &[]).is_err());
}

#[test]
fn setup_mode_refuses_the_paste_command() {
    let (mut doc, anim, _, _) = fixture();
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    let before = json(&doc);
    let cmd = PasteKeys::new(&doc, anim, &clip, 0.0, &[]).unwrap();
    let mut history = History::default();
    assert!(!history.push_in_mode(Box::new(cmd), &mut doc, WorkMode::Setup));
    assert_eq!(json(&doc), before);
    assert_eq!(history.undo_depth(), 0);
}

#[test]
fn readonly_timelines_cannot_be_misidentified_as_bone_or_slot_keys() {
    let (_, _, _, slot) = fixture();
    let constraint = Default::default();
    for source in [
        Timeline::DrawOrder {
            keys: vec![Key::stepped(0.0, vec![])],
        },
        Timeline::IkMix {
            constraint,
            keys: vec![],
        },
        Timeline::IkBendDirection {
            constraint,
            keys: vec![],
        },
        Timeline::IkSoftness {
            constraint,
            keys: vec![],
        },
        Timeline::TransformConstraintMix {
            constraint,
            keys: vec![],
        },
        Timeline::Deform {
            slot,
            attachment: "mesh".into(),
            keys: vec![],
        },
    ] {
        assert!(address(&source).unwrap_err().contains("read-only"));
    }
}

#[test]
fn copied_values_survive_source_edits_and_leave_other_tracks_events_and_setup_intact() {
    let (mut doc, anim, bone, _) = fixture();
    doc.animations[anim]
        .timelines
        .push(Timeline::BoneTranslate {
            bone,
            axis: Axis::X,
            keys: vec![Key::stepped(0.5, 91.0)],
        });
    doc.animations[anim]
        .events
        .push(ankhimate_core::animation::EventKey {
            time: 0.5,
            name: "impact \"漢".into(),
            int_value: 7,
            float_value: 0.3,
            string_value: "payload".into(),
            audio: "sound".into(),
            volume: 0.7,
            balance: -0.2,
        });
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)[..2]).unwrap();
    if let Timeline::BoneRotate { keys, .. } = &mut doc.animations[anim].timelines[0] {
        keys[0].value = 900.0;
        keys[1].interp = Interp::Linear;
    }
    let setup = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let other = doc.animations[anim].timelines[1].clone();
    let events = doc.animations[anim].events.clone();
    paste(&mut doc, anim, &clip, 2.0, &[], &mut History::default());
    assert!(
        matches!(&doc.animations[anim].timelines[0],Timeline::BoneRotate {keys,..} if keys[2].value==10.0 && keys[3].interp==bezier())
    );
    assert_eq!(doc.animations[anim].timelines[1], other);
    assert_eq!(doc.animations[anim].events, events);
    assert_eq!(
        ankhimate_formats::convert::to_schema(&doc.as_project_ref()).bones,
        setup.bones
    );
}

#[test]
fn source_duplicate_selection_is_deduplicated_but_duplicate_times_are_not_silently_merged() {
    let (mut doc, anim, _, _) = fixture();
    let r = refs(&doc, anim)[0].clone();
    assert_eq!(
        KeyClip::capture(&doc, anim, &[r.clone(), r])
            .unwrap()
            .keys
            .len(),
        1
    );
    if let Timeline::BoneRotate { keys, .. } = &mut doc.animations[anim].timelines[0] {
        keys[1].time = keys[0].time;
    }
    let clip = KeyClip::capture(&doc, anim, &refs(&doc, anim)).unwrap();
    assert!(PasteKeys::new(&doc, anim, &clip, 0.0, &[]).is_err());
}

#[path = "key_clipboard/mirrored_offsets.rs"]
mod mirrored_offsets_tests;
