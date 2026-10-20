use super::*;
use ankhimate_core::{
    assets::ImageAsset,
    attachment::{Attachment, Rect, RegionAttachment},
    skeleton::Bone,
    slot::Slot,
};
fn bone(name: &str, parent: Option<BoneId>) -> Bone {
    Bone {
        name: name.into(),
        parent,
        length: 10.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}
fn fixture() -> (Document, BoneId) {
    let mut s = Document::new();
    let root = s.skeleton.add_bone(bone("arm", None));
    let mut png = std::io::Cursor::new(Vec::new());
    image::DynamicImage::new_rgba8(2, 2)
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    s.assets
        .add(ImageAsset::new("arm.png", png.into_inner(), 2, 2));
    let slot = s.skeleton.add_slot(Slot::new("arm-slot".into(), root));
    s.skeleton.slots[slot].attachment = Some("arm".into());
    s.skeleton.skins[s.skeleton.default_skin].set(
        slot,
        "arm",
        Attachment::Region(RegionAttachment {
            texture: "arm.png".into(),
            local_offset: glam::Vec2::ZERO,
            local_rotation: 0.0,
            local_scale: glam::Vec2::ONE,
            width: 2.0,
            height: 2.0,
            uv_rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
            pivot: glam::Vec2::splat(0.5),
            sequence: None,
        }),
    );
    (s, root)
}
#[test]
fn t819_transfer_brings_actual_image_bytes_and_regions() {
    let (s, r) = fixture();
    let mut d = Document::new();
    let p = Plan::prepare(&s, &d, r, None).unwrap();
    p.into_command(&d).unwrap().apply(&mut d);
    assert_eq!(d.assets.len(), 1);
    assert_eq!(
        d.assets
            .get(d.assets.by_name("arm.png").unwrap())
            .unwrap()
            .bytes,
        s.assets
            .get(s.assets.by_name("arm.png").unwrap())
            .unwrap()
            .bytes
    );
    assert_eq!(d.skeleton.slots.len(), 1);
}
#[test]
fn t819_missing_image_refuses_before_any_mutation() {
    let (mut s, r) = fixture();
    s.assets.images.values_mut().next().unwrap().bytes.clear();
    let d = Document::new();
    assert!(Plan::prepare(&s, &d, r, None).is_err());
}
use crate::{commands::History, snapshot::snapshot};
use ankhimate_core::{
    animation::{Animation, Axis, Interp, Key, Marker, Timeline},
    constraints::{Constraint, IkConstraint},
    skin::Skin,
};
fn png(name: &str, color: [u8; 4]) -> ImageAsset {
    let mut b = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba(color))
        .write_to(&mut b, image::ImageFormat::Png)
        .unwrap();
    ImageAsset::new(name, b.into_inner(), 2, 2)
}
fn full() -> (Document, BoneId) {
    let (mut s, r) = fixture();
    let child = s.skeleton.add_bone(bone("hand", Some(r)));
    let target = s.skeleton.add_bone(bone("target", Some(r)));
    s.assets.add(png("hand.png", [255, 0, 20, 255]));
    let slot = s.skeleton.add_slot(Slot::new("hand-slot".into(), child));
    s.skeleton.slots[slot].attachment = Some("hand".into());
    let a = s.skeleton.skins[s.skeleton.default_skin]
        .entries
        .values()
        .next()
        .unwrap()
        .clone();
    let Attachment::Region(mut region) = a else {
        panic!()
    };
    region.texture = "hand.png".into();
    s.skeleton.skins[s.skeleton.default_skin].set(slot, "hand", Attachment::Region(region));
    s.skeleton
        .add_constraint(Constraint::Ik(IkConstraint::aim("aim", target, child)));
    let mut a = Animation::new("move", 2.0);
    a.looping = false;
    a.bone_offsets = vec![(child, 0.125)];
    a.markers.push(Marker {
        time: 0.5,
        name: "impact".into(),
        color: [1.0; 4],
    });
    let keys = vec![
        Key {
            time: 0.0,
            value: 0.0,
            interp: Interp::Linear,
        },
        Key {
            time: 1.0,
            value: 30.0,
            interp: Interp::Bezier {
                out_handle: glam::vec2(0.2, -0.4),
                in_handle: glam::vec2(0.8, 1.4),
            },
        },
    ];
    a.timelines = vec![
        Timeline::BoneTranslate {
            bone: r,
            axis: Axis::X,
            keys: keys.clone(),
        },
        Timeline::BoneTranslate {
            bone: r,
            axis: Axis::Y,
            keys: vec![Key {
                time: 0.0,
                value: 2.0,
                interp: Interp::Stepped,
            }],
        },
        Timeline::BoneRotate { bone: child, keys },
    ];
    s.animations.insert(a);
    (s, r)
}
fn transfer(s: &Document, d: &mut Document, r: BoneId, parent: Option<BoneId>) -> Summary {
    let p = Plan::prepare(s, d, r, parent).unwrap();
    let summary = p.summary.clone();
    p.into_command(d).unwrap().apply(d);
    summary
}
#[test]
fn t819_collision_renames_are_previewed_and_destination_bytes_never_overwritten() {
    let (s, r) = full();
    let mut d = Document::new();
    d.skeleton.add_bone(bone("arm", None));
    d.assets.add(png("arm.png", [10, 20, 30, 255]));
    let old = snapshot(&d);
    let p = Plan::prepare(&s, &d, r, None).unwrap();
    assert!(old == snapshot(&d));
    assert!(
        p.summary
            .images
            .iter()
            .any(|a| a.source == "arm.png" && a.destination == "arm.png_2" && !a.reused)
    );
    assert_eq!(p.summary.bones[0], ("arm".into(), "arm_2".into()));
    let bytes = d.assets.images.values().next().unwrap().bytes.clone();
    p.into_command(&d).unwrap().apply(&mut d);
    assert_eq!(
        d.assets
            .get(d.assets.by_name("arm.png").unwrap())
            .unwrap()
            .bytes,
        bytes
    );
    assert_eq!(d.skeleton.bones.len(), 4);
}
#[test]
fn t819_identical_asset_under_different_name_is_reused_by_bytes_and_dimensions() {
    let (s, r) = fixture();
    let mut d = Document::new();
    let mut image = s.assets.images.values().next().unwrap().clone();
    image.name = "existing".into();
    d.assets.add(image);
    let result = transfer(&s, &mut d, r, None);
    assert!(result.images[0].reused);
    assert_eq!(result.images[0].destination, "existing");
    assert_eq!(d.assets.len(), 1);
    let a = d.skeleton.skins[d.skeleton.default_skin]
        .entries
        .values()
        .next()
        .unwrap();
    assert!(matches!(a,Attachment::Region(r)if r.texture=="existing"));
}
#[test]
fn t819_homonymous_clips_are_distinct_and_all_curves_axes_metadata_preserved() {
    let (s, r) = full();
    let mut d = Document::new();
    d.animations.insert(Animation::new("move", 8.0));
    d.animations.insert(Animation::new("move_2", 9.0));
    let result = transfer(&s, &mut d, r, None);
    assert_eq!(result.clips, vec![("move".into(), "move_3".into())]);
    let a = d.animations.values().find(|a| a.name == "move_3").unwrap();
    let old = s.animations.values().next().unwrap();
    assert_eq!(a.duration, old.duration);
    assert_eq!(a.looping, old.looping);
    assert_eq!(a.markers, old.markers);
    assert_eq!(a.bone_offsets[0].1, 0.125);
    for (src, dst) in old.timelines.iter().zip(&a.timelines) {
        let mut expected = src.clone();
        expected.set_bone(dst.bone().unwrap());
        assert_eq!(&expected, dst);
    }
    assert_eq!(
        d.animations
            .values()
            .find(|a| a.name == "move")
            .unwrap()
            .duration,
        8.0
    );
}
#[test]
fn t819_source_ids_never_retarget_existing_destination_bones() {
    let (s, r) = full();
    let mut d = Document::new();
    let existing = d.skeleton.add_bone(bone("stranger", None));
    assert_eq!(r, existing);
    transfer(&s, &mut d, r, Some(existing));
    let a = d.animations.values().next().unwrap();
    assert!(a.timelines.iter().all(|t| t.bone() != Some(existing)));
    assert_eq!(d.skeleton.bones[existing].name, "stranger");
    for c in d.skeleton.constraints.values() {
        assert!(!c.affected_bones().contains(&existing));
        assert_ne!(c.target(), Some(existing));
    }
}
#[test]
fn t819_local_setup_values_under_parent_are_retained_without_world_promise() {
    let (mut s, r) = fixture();
    s.skeleton.bones[r].local_transform.position = glam::vec2(4.0, 8.0);
    let mut d = Document::new();
    let p = d.skeleton.add_bone(bone("parent", None));
    d.skeleton.bones[p].local_transform.position = glam::vec2(100.0, 200.0);
    let result = transfer(&s, &mut d, r, Some(p));
    let b = d.skeleton.bones.values().find(|b| b.name == "arm").unwrap();
    assert_eq!(b.parent, Some(p));
    assert_eq!(b.local_transform, s.skeleton.bones[r].local_transform);
    assert!(result.notes.iter().any(|s| s.contains("NOT preserved")));
}
#[test]
fn t819_one_undo_and_three_cycles_restore_exact_native_ids_assets_and_metadata() {
    let (s, r) = full();
    let mut d = Document::new();
    d.skeleton.add_bone(bone("existing", None));
    d.meta.name = "Destination".into();
    let before = snapshot(&d);
    let p = Plan::prepare(&s, &d, r, None).unwrap();
    let mut h = History::default();
    h.push(Box::new(p.into_command(&d).unwrap()), &mut d);
    let after = snapshot(&d);
    assert_eq!(h.undo_depth(), 1);
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert!(before == snapshot(&d));
        assert!(h.redo(&mut d));
        assert!(after == snapshot(&d));
    }
}
#[test]
fn t819_stale_plan_missing_parent_or_root_refuses() {
    let (s, r) = full();
    let mut d = Document::new();
    let p = Plan::prepare(&s, &d, r, None).unwrap();
    d.meta.fps = 1;
    assert!(p.into_command(&d).is_err());
    assert!(Plan::prepare(&s, &d, Default::default(), None).is_err());
    assert!(Plan::prepare(&s, &d, r, Some(Default::default())).is_err());
}
#[test]
fn t819_external_constraints_and_dynamic_constraints_refuse() {
    for physics in [true, false] {
        let (mut s, r) = fixture();
        let outside = s.skeleton.add_bone(bone("outside", None));
        s.skeleton.add_constraint(if physics {
            Constraint::Physics(ankhimate_core::constraints::PhysicsConstraint::sway(
                "physics", r,
            ))
        } else {
            Constraint::Ik(IkConstraint::aim("external", outside, r))
        });
        assert!(Plan::prepare(&s, &Document::new(), r, None).is_err());
    }
}
#[test]
fn t819_mesh_linked_clipping_and_path_attachments_refuse() {
    for n in 0..3 {
        let (mut s, r) = fixture();
        let skin = s.skeleton.default_skin;
        let slot = s.skeleton.slots.keys().next().unwrap();
        s.skeleton.skins[skin].set(
            slot,
            "extra",
            match n {
                0 => Attachment::Mesh(Default::default()),
                1 => Attachment::Clipping(ankhimate_core::attachment::ClippingAttachment {
                    vertices: vec![glam::Vec2::ZERO; 3],
                    end_slot: None,
                }),
                _ => Attachment::Path(Default::default()),
            },
        );
        assert!(Plan::prepare(&s, &Document::new(), r, None).is_err());
    }
}
#[test]
fn t819_alternate_skin_and_conditional_gating_refuse() {
    for gated in [true, false] {
        let (mut s, r) = fixture();
        let mut alt = Skin::new("royal");
        if gated {
            alt.bones.push(r);
        } else {
            let default = &s.skeleton.skins[s.skeleton.default_skin];
            let (k, a) = default.entries.iter().next().unwrap();
            alt.entries.insert(k.clone(), a.clone());
        }
        s.skeleton.skins.insert(alt);
        assert!(Plan::prepare(&s, &Document::new(), r, None).is_err());
    }
}
#[test]
fn t819_relevant_slot_ffd_constraint_draworder_and_events_refuse() {
    for n in 0..5 {
        let (mut s, r) = full();
        let slot = s.skeleton.slots.keys().next().unwrap();
        let constraint = s.skeleton.constraints.keys().next().unwrap();
        let a = s.animations.values_mut().next().unwrap();
        match n {
            0 => a.timelines.push(Timeline::SlotVisible {
                slot,
                keys: Vec::new(),
            }),
            1 => a.timelines.push(Timeline::Deform {
                slot,
                attachment: "arm".into(),
                keys: Vec::new(),
            }),
            2 => a.timelines.push(Timeline::IkMix {
                constraint,
                keys: Vec::new(),
            }),
            3 => a.timelines.push(Timeline::DrawOrder {
                keys: vec![Key {
                    time: 0.0,
                    value: vec![(slot, 1)],
                    interp: Interp::Stepped,
                }],
            }),
            _ => a.events.push(ankhimate_core::animation::EventKey {
                time: 0.5,
                name: "event".into(),
                int_value: 0,
                float_value: 0.0,
                string_value: String::new(),
                audio: String::new(),
                volume: 1.0,
                balance: 0.0,
            }),
        }
        assert!(Plan::prepare(&s, &Document::new(), r, None).is_err());
    }
}
#[test]
fn t819_simulated_failure_after_staging_never_changes_live_document_or_assets() {
    let (s, r) = full();
    let d = Document::new();
    let before = snapshot(&d);
    let result = Plan::prepare_checked(&s, &d, r, None, |staged| {
        assert_eq!(staged.assets.len(), 2);
        assert_eq!(staged.skeleton.bones.len(), 3);
        Err("Simulated final validation failure".into())
    });
    assert!(result.is_err());
    assert!(before == snapshot(&d));
}
#[test]
fn t819_invalid_source_topology_numbers_or_missing_reference_refuse() {
    for n in 0..4 {
        let (mut s, r) = fixture();
        match n {
            0 => s.skeleton.bones[r].local_transform.rotation = f32::NAN,
            1 => s.skeleton.slots.values_mut().next().unwrap().attachment = Some("absent".into()),
            2 => s.skeleton.draw_order.push(Default::default()),
            _ => s.skeleton.bones[r].color[0] = f32::NAN,
        }
        assert!(Plan::prepare(&s, &Document::new(), r, None).is_err());
    }
}
#[test]
fn t819_source_and_destination_unchanged_during_prepare() {
    let (s, r) = full();
    let d = Document::new();
    let a = snapshot(&s);
    let b = snapshot(&d);
    let _ = Plan::prepare(&s, &d, r, None).unwrap();
    assert!(a == snapshot(&s));
    assert!(b == snapshot(&d));
}
#[test]
fn t819_duplicate_logical_asset_identity_refuses_in_either_document() {
    for source in [true, false] {
        let (mut s, r) = fixture();
        let mut d = Document::new();
        let a = s.assets.images.values().next().unwrap().clone();
        if source {
            s.assets.images.insert(a);
        } else {
            d.assets.images.insert(a.clone());
            d.assets.images.insert(a);
        }
        assert!(Plan::prepare(&s, &d, r, None).is_err());
    }
}
#[test]
fn t819_regions_preserve_pivot_uv_dark_tint_and_relative_draw_order() {
    let (mut s, r) = full();
    s.skeleton.draw_order.reverse();
    let slot = s.skeleton.draw_order[0];
    s.skeleton.slots[slot].dark_color = Some([0.1, 0.2, 0.3, 0.4]);
    if let Attachment::Region(region) = s.skeleton.skins[s.skeleton.default_skin]
        .entries
        .values_mut()
        .next()
        .unwrap()
    {
        region.pivot = glam::vec2(0.1, 0.8);
        region.uv_rect.x = 0.25;
        region.uv_rect.w = 0.5;
    }
    let mut source = ankhimate_formats::convert::to_schema(&s.as_project_ref());
    let mut d = Document::new();
    transfer(&s, &mut d, r, None);
    let mut dest = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    assert_eq!(source.draw_order, dest.draw_order);
    source.slots.sort_by(|a, b| a.name.cmp(&b.name));
    dest.slots.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(
        serde_json::to_value(&source.slots).unwrap(),
        serde_json::to_value(&dest.slots).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&source.skins).unwrap(),
        serde_json::to_value(&dest.skins).unwrap()
    );
}
#[test]
fn t819_internal_transform_constraint_remaps_and_order_is_retained() {
    let (mut s, r) = full();
    let ids = s.skeleton.update_order.clone();
    let c = s.skeleton.add_constraint(Constraint::Transform(
        ankhimate_core::constraints::TransformConstraint {
            name: "follow".into(),
            target: ids[2],
            bones: vec![ids[1]],
            offsets: Default::default(),
            mix: Default::default(),
            local: true,
            relative: false,
        },
    ));
    s.skeleton.constraint_order.retain(|id| *id != c);
    s.skeleton.constraint_order.insert(0, c);
    let mut d = Document::new();
    d.skeleton.add_bone(bone("old", None));
    transfer(&s, &mut d, r, None);
    let names: Vec<_> = d
        .skeleton
        .constraint_order
        .iter()
        .map(|id| d.skeleton.constraints[*id].name())
        .collect();
    assert_eq!(names, vec!["follow", "aim"]);
    assert_eq!(d.skeleton.constraints.len(), 2);
}
#[test]
fn t819_sequence_frames_follow_the_same_exact_asset_mapping() {
    let (mut s, r) = fixture();
    s.assets.add(png("second", [20, 30, 40, 90]));
    let skin = s.skeleton.default_skin;
    if let Attachment::Region(a) = s.skeleton.skins[skin].entries.values_mut().next().unwrap() {
        a.sequence = Some(ankhimate_core::attachment::Sequence {
            frames: vec!["arm.png".into(), "second".into()],
            ..Default::default()
        });
    }
    let mut d = Document::new();
    d.assets.add(png("second", [1, 2, 3, 255]));
    transfer(&s, &mut d, r, None);
    let a = d.skeleton.skins[d.skeleton.default_skin]
        .entries
        .values()
        .next()
        .unwrap();
    let Attachment::Region(a) = a else { panic!() };
    assert_eq!(
        a.sequence.as_ref().unwrap().frames,
        vec!["arm.png", "second_2"]
    );
}
#[test]
fn t819_empty_animation_set_and_unrelated_tracks_do_not_invent_clips() {
    let (s, r) = fixture();
    let mut d = Document::new();
    let summary = transfer(&s, &mut d, r, None);
    assert!(summary.clips.is_empty());
    assert!(d.animations.is_empty());
}
#[test]
fn t819_transfer_and_following_edit_keep_native_ids_across_full_history_replay() {
    let (s, r) = full();
    let mut d = Document::new();
    let p = Plan::prepare(&s, &d, r, None).unwrap();
    let root = p.root;
    let mut h = History::default();
    h.push(Box::new(p.into_command(&d).unwrap()), &mut d);
    let mut t = d.skeleton.bones[root].local_transform;
    t.position.x = 30.0;
    h.push(
        Box::new(crate::commands::bone_cmds::SetBoneTransform::new(root, t)),
        &mut d,
    );
    let after = snapshot(&d);
    for _ in 0..3 {
        h.undo(&mut d);
        h.undo(&mut d);
        assert!(d.skeleton.bones.is_empty());
        h.redo(&mut d);
        h.redo(&mut d);
        assert!(after == snapshot(&d));
    }
}
#[test]
fn t819_selection_limits_refuse_without_publishing() {
    let mut s = Document::new();
    let r = s.skeleton.add_bone(bone("root", None));
    for i in 0..MAX_BONES {
        s.skeleton.add_bone(bone(&format!("child-{i}"), Some(r)));
    }
    let d = Document::new();
    assert!(
        Plan::prepare(&s, &d, r, None)
            .err()
            .unwrap()
            .contains("512")
    );
}
