use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    assets::ImageAsset,
    attachment::{Attachment, MeshAttachment, Rect, RegionAttachment, VertexWeight},
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::{Document, retrace::Plan};
use glam::{Vec2, vec2};
pub fn fixture(
    weighted: bool,
) -> (
    Document,
    ankhimate_core::ids::SkinId,
    ankhimate_core::ids::SlotId,
) {
    let mut d = Document::new();
    let b = d.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 20.,
        local_transform: ankhimate_core::math::Transform {
            position: vec2(31., -17.),
            rotation: 0.3,
            scale: vec2(1.1, 0.8),
            ..Default::default()
        },
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let child = d.skeleton.add_bone(Bone {
        name: "influence".into(),
        parent: Some(b),
        length: 10.,
        local_transform: ankhimate_core::math::Transform {
            position: vec2(5., 4.),
            rotation: -0.2,
            ..Default::default()
        },
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = d.skeleton.add_slot(Slot {
        attachment: Some("mesh".into()),
        ..Slot::new("slot".into(), b)
    });
    let skin = d.skeleton.default_skin;
    let image = image::RgbaImage::from_pixel(64, 64, image::Rgba([220, 160, 40, 255]));
    let mut encoded = std::io::Cursor::new(vec![]);
    image::DynamicImage::ImageRgba8(image)
        .write_to(&mut encoded, image::ImageFormat::Png)
        .unwrap();
    d.assets
        .add(ImageAsset::new("art", encoded.into_inner(), 64, 64));
    let region = RegionAttachment {
        texture: "art".into(),
        local_offset: Vec2::ZERO,
        local_rotation: 0.,
        local_scale: Vec2::ONE,
        width: 20.,
        height: 20.,
        uv_rect: Rect {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.,
        },
        pivot: Vec2::splat(0.5),
        sequence: None,
    };
    let mut m = MeshAttachment::from_region(&region);
    if weighted {
        m.weights = (0..4)
            .map(|i| {
                vec![
                    VertexWeight {
                        bone: b,
                        weight: 0.2 + i as f32 * 0.2,
                    },
                    VertexWeight {
                        bone: child,
                        weight: 0.8 - i as f32 * 0.2,
                    },
                ]
            })
            .collect();
    }
    d.skeleton.skins[skin].set(slot, "mesh", Attachment::Mesh(m));
    let mut a = Animation::new("kept", 3.);
    a.timelines.push(Timeline::BoneRotate {
        bone: child,
        keys: vec![
            Key::linear(0., 0.),
            Key::stepped(1., 0.5),
            Key::linear(3., 0.),
        ],
    });
    d.animations.insert(a);
    ankhimate_document::rebind::rebind_meshes(&mut d);
    (d, skin, slot)
}
pub fn mesh(
    d: &Document,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
) -> &MeshAttachment {
    let Attachment::Mesh(m) = d.skeleton.skins[skin].get(slot, "mesh").unwrap() else {
        panic!()
    };
    m
}
pub fn mesh_mut(
    d: &mut Document,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
) -> &mut MeshAttachment {
    let Attachment::Mesh(m) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "mesh".into()))
        .unwrap()
    else {
        panic!()
    };
    m
}
pub fn changed(
    d: &Document,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
) -> Result<Plan, String> {
    Plan::prepare(
        d,
        skin,
        slot,
        "mesh",
        vec![
            vec2(-9., 9.),
            vec2(-9., -9.),
            vec2(9., -9.),
            vec2(9., 9.),
            Vec2::ZERO,
        ],
        vec![
            vec2(0., 0.),
            vec2(0., 1.),
            vec2(1., 1.),
            vec2(1., 0.),
            Vec2::splat(0.5),
        ],
        vec![[0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4]],
    )
}
