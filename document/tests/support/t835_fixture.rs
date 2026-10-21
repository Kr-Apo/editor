// Synthetic author project. No historical/personal asset is edited.
#[allow(dead_code, reason = "Shared accepted native path fixture")]
mod path_document {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/path_document_fixture.rs"
    ));
}
use ankhimate_core::{
    animation::{Animation, Axis, Interp, Key, Timeline},
    attachment::{Attachment, MeshAttachment, VertexWeight},
    ids::AnimationId,
};
use ankhimate_document::Document;
pub fn fixture() -> (Document, Vec<AnimationId>) {
    let (mut d, a, _) = path_document::path_document();
    let skin = d.skeleton.default_skin;
    let (slot, r) = d.skeleton.skins[skin]
        .entries
        .iter()
        .filter_map(|((s, _), a)| {
            if let Attachment::Region(r) = a
                && d.skeleton.bones[d.skeleton.slots[*s].bone].parent.is_some()
            {
                Some((*s, r.clone()))
            } else {
                None
            }
        })
        .min_by_key(|(slot, _)| *slot)
        .unwrap();
    let bone = d.skeleton.slots[slot].bone;
    let second = d.skeleton.bones[bone].parent.unwrap();
    let mut m = MeshAttachment::from_region(&r);
    m.weights = vec![
        vec![
            VertexWeight { bone, weight: 0.6 },
            VertexWeight {
                bone: second,
                weight: 0.4
            }
        ];
        m.setup_vertices.len()
    ];
    let n = m.influence_count();
    d.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(m));
    ankhimate_document::rebind::rebind_meshes(&mut d);
    let a0 = &mut d.animations[a];
    a0.name = "Zulu 鍵".into();
    a0.duration = 1.25;
    a0.looping = false;
    a0.timelines.push(Timeline::Deform {
        slot,
        attachment: "art".into(),
        keys: vec![
            Key::linear(0., vec![glam::Vec2::ZERO; n]),
            Key {
                time: 0.5,
                value: vec![glam::vec2(2., -1.); n],
                interp: Interp::Bezier {
                    out_handle: glam::vec2(0.2, -0.5),
                    in_handle: glam::vec2(0.8, 1.5),
                },
            },
            Key::linear(2., vec![glam::Vec2::ZERO; n]),
        ],
    });
    a0.timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![
            Key::linear(0., -180.),
            Key {
                time: 0.5,
                value: 180.,
                interp: Interp::Stepped,
            },
        ],
    });
    a0.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::X,
        keys: vec![Key::linear(0., -2.), Key::linear(2., 9.)],
    });
    a0.timelines.push(Timeline::SlotVisible {
        slot,
        keys: vec![
            Key::linear(0., true),
            Key {
                time: 0.5,
                value: false,
                interp: Interp::Stepped,
            },
        ],
    });
    a0.timelines.push(Timeline::SlotAttachment {
        slot,
        keys: vec![Key::linear(0., Some("art".into()))],
    });
    a0.events.push(ankhimate_core::animation::EventKey {
        time: 0.5,
        name: "impact 鍵".into(),
        int_value: 3,
        float_value: 0.75,
        string_value: "quote \" and \\".into(),
        audio: String::new(),
        volume: 1.,
        balance: 0.,
    });
    a0.markers
        .push(ankhimate_core::animation::Marker::new(0.75, "marker 鍵"));
    a0.bone_offsets.push((bone, 0.125));
    let mut b = a0.clone();
    b.name = "Été".into();
    b.duration = 3.;
    b.looping = true;
    let b = d.animations.insert(b);
    let c = d.animations.insert(Animation::new("Idle", 0.75));
    (d, vec![a, b, c])
}
pub fn payloads(d: &Document) -> std::collections::BTreeMap<String, serde_json::Value> {
    d.animations
        .values()
        .map(|a| (a.name.clone(), serde_json::to_value(a).unwrap()))
        .collect()
}
pub fn names(d: &Document) -> Vec<String> {
    d.animation_ids()
        .iter()
        .map(|id| d.animations[*id].name.clone())
        .collect()
}
