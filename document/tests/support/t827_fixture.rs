use super::Document;
use super::{BoneProperty, KeyRef, TimelineAddr};
use ankhimate_core::{
    animation::{Animation, Axis, Interp, Key, Timeline},
    ids::AnimationId,
    skeleton::Bone,
    slot::Slot,
};
pub fn fixture(n: usize, bones: usize, all_channels: bool) -> (Document, AnimationId, Vec<KeyRef>) {
    let mut d = Document::new();
    let mut a = Animation::new("selection", (n + 1) as f32);
    let mut refs = vec![];
    for b in 0..bones {
        let bone = d.skeleton.add_bone(Bone {
            name: format!("bone_{b}"),
            parent: None,
            length: 10.,
            local_transform: Default::default(),
            inherit: Default::default(),
            color: Bone::default_color(),
        });
        let mut tracks = vec![(
            TimelineAddr::Bone {
                bone,
                property: BoneProperty::Rotate,
                axis: None,
            },
            Timeline::BoneRotate {
                bone,
                keys: keys(n),
            },
        )];
        if all_channels {
            for (property, axis) in [
                (BoneProperty::Translate, Axis::X),
                (BoneProperty::Translate, Axis::Y),
                (BoneProperty::Scale, Axis::X),
                (BoneProperty::Scale, Axis::Y),
                (BoneProperty::Shear, Axis::X),
                (BoneProperty::Shear, Axis::Y),
            ] {
                let k = keys(n);
                let t = match property {
                    BoneProperty::Translate => Timeline::BoneTranslate {
                        bone,
                        axis,
                        keys: k,
                    },
                    BoneProperty::Scale => Timeline::BoneScale {
                        bone,
                        axis,
                        keys: k,
                    },
                    _ => Timeline::BoneShear {
                        bone,
                        axis,
                        keys: k,
                    },
                };
                tracks.push((
                    TimelineAddr::Bone {
                        bone,
                        property,
                        axis: Some(axis),
                    },
                    t,
                ));
            }
            let slot = d.skeleton.add_slot(Slot::new(format!("slot_{b}"), bone));
            tracks.push((
                TimelineAddr::SlotColor { slot },
                Timeline::SlotColor {
                    slot,
                    keys: (0..n)
                        .map(|i| Key::linear(i as f32, [i as f32 / n as f32, 0.5, 1., 0.7]))
                        .collect(),
                },
            ));
        }
        for (addr, t) in tracks {
            for index in 0..n {
                refs.push(KeyRef {
                    addr: addr.clone(),
                    index,
                });
            }
            a.timelines.push(t);
        }
    }
    let anim = d.animations.insert(a);
    (d, anim, refs)
}
fn keys(n: usize) -> Vec<Key<f32>> {
    (0..n)
        .map(|i| Key {
            time: i as f32,
            value: i as f32 * 0.125 - 3.,
            interp: match i % 3 {
                0 => Interp::Linear,
                1 => Interp::Stepped,
                _ => Interp::Bezier {
                    out_handle: glam::vec2(0.2, -0.4),
                    in_handle: glam::vec2(0.8, 1.5),
                },
            },
        })
        .collect()
}
pub fn cases() -> Vec<(String, Document, AnimationId, Vec<KeyRef>)> {
    let mut out = vec![];
    for (label, n, b, c) in [
        ("ten", 10, 1, false),
        ("thousand", 1000, 1, false),
        ("ten_thousand", 10000, 1, false),
        ("multichannel", 20, 2, true),
        ("ten_thousand_multitrack", 1250, 1, true),
        ("many_small_tracks", 1, 1000, false),
    ] {
        let (d, a, r) = fixture(n, b, c);
        out.push((label.into(), d, a, r));
    }
    for mode in 0..18 {
        let (mut d, a, mut r) = fixture(5, 2, true);
        let original = r.clone();
        match mode {
            0 => r.clear(),
            1 => {
                r.reverse();
                r.extend(original.iter().step_by(2).cloned());
            }
            2 => r[4].index = 999,
            3 => r.last_mut().unwrap().index = 999,
            4 => {
                let duplicate = d.animations[a].timelines[0].clone();
                d.animations[a].timelines.push(duplicate);
            }
            5 => {
                d.animations[a].timelines.remove(0);
            }
            6 => {
                let ids = d.skeleton.bones.keys().collect::<Vec<_>>();
                d.skeleton.bones[ids[1]].name = d.skeleton.bones[ids[0]].name.clone();
            }
            7 => {
                let ids = d.skeleton.slots.keys().collect::<Vec<_>>();
                d.skeleton.slots[ids[1]].name = d.skeleton.slots[ids[0]].name.clone();
                r.rotate_left(35);
            }
            8 => {
                if let TimelineAddr::Bone { bone, .. } = r[0].addr {
                    d.skeleton.bones.remove(bone);
                }
            }
            9 => {
                if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
                    keys[2].time = f32::NAN;
                }
            }
            10 => {
                if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
                    keys[2].value = f32::INFINITY;
                }
            }
            11 => {
                if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
                    keys[2].interp = Interp::Bezier {
                        out_handle: glam::vec2(-1., 0.),
                        in_handle: glam::vec2(1., 1.),
                    };
                }
            }
            12 => {
                if let TimelineAddr::Bone { axis, .. } = &mut r[0].addr {
                    *axis = Some(Axis::X);
                }
            }
            13 => {
                d.animations.remove(a);
            }
            14 => {
                if let TimelineAddr::SlotColor { slot } = r[35].addr {
                    d.skeleton.slots.remove(slot);
                }
                r.rotate_left(35);
            }
            15 => {
                r = vec![
                    r[0].clone(),
                    r[35].clone(),
                    r[5].clone(),
                    r[40].clone(),
                    r[0].clone(),
                ];
            }
            16 => {
                r[0].index = 999;
                let duplicate = d.animations[a].timelines[0].clone();
                d.animations[a].timelines.push(duplicate);
            }
            _ => {
                if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
                    keys[0].time = -1.;
                }
            }
        }
        out.push((format!("case_{mode}"), d, a, r));
    }
    out
}
