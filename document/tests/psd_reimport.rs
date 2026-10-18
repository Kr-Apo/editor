use ankhimate_document::{Document, History, commands::psd_cmds::ImportPsd};
use ankhimate_formats::psd::{self, ImportOptions};
use serde_json::{Value, json};
#[path = "../../formats/tests/support/psd_copy.rs"]
mod psd_copy;
const SOURCE: &[u8] = include_bytes!("../../formats/tests/fixtures/nested_groups.psd");
fn imported(bytes: &[u8]) -> psd::PsdImport {
    psd::import(bytes, &ImportOptions::default()).unwrap()
}
fn seed() -> Document {
    let mut d = Document::new();
    let mut h = History::default();
    h.push(
        Box::new(ImportPsd::new(imported(SOURCE), true, "nested")),
        &mut d,
    );
    let raw = json!({"version":1,"name":"animated","fps":60,"bones":[{"name":"root","length":1}],"animations":[{"name":"clip","duration":2,"timelines":[{"kind":"bone_rotate","bone":"root","keys":[{"time":0,"value":0,"curve":"linear"},{"time":1,"value":45,"curve":"bezier","handles":[0.2,0.1,0.7,1]}]}]}]});
    let mut anim = ankhimate_formats::from_json(&raw.to_string())
        .unwrap()
        .animations
        .values()
        .next()
        .unwrap()
        .clone();
    if let ankhimate_core::animation::Timeline::BoneRotate { bone, .. } = &mut anim.timelines[0] {
        *bone = d.skeleton.update_order[0];
    }
    d.animations.insert(anim);
    d
}
fn snapshot(d: &Document) -> Value {
    let skins: Vec<_> = d
        .skeleton
        .skins
        .iter()
        .map(|(id, s)| {
            let mut entries: Vec<_> = s.entries.iter().collect();
            entries.sort_by_key(|(k, _)| (*k).clone());
            json!({"id":id,"name":s.name,"entries":entries})
        })
        .collect();
    json!({"schema":ankhimate_formats::convert::to_schema(&d.as_project_ref()),"assets":d.assets,"bones":d.skeleton.bones,"slots":d.skeleton.slots,"skins":skins,"animations":d.animations,"constraints":d.skeleton.constraints})
}
#[test]
fn changed_psd_pixels_update_existing_images_without_duplicating_the_animated_rig() {
    let mut d = seed();
    let before = snapshot(&d);
    let counts = (d.skeleton.bones.len(), d.skeleton.slots.len());
    let changed = psd_copy::edit(SOURCE, |layers| {
        layers
            .iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    assert_ne!(
        imported(&changed)
            .assets
            .images
            .values()
            .next()
            .unwrap()
            .bytes
            .len(),
        0
    );
    let mut h = History::default();
    let (command, report) =
        ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &changed).unwrap();
    assert!(report.conflicts.is_empty(), "{report:?}");
    h.push(Box::new(command.unwrap()), &mut d);
    assert_eq!(
        (d.skeleton.bones.len(), d.skeleton.slots.len()),
        counts,
        "reimport must not merge a second rig"
    );
    assert_eq!(snapshot(&d)["animations"], before["animations"]);
    assert_eq!(snapshot(&d)["bones"], before["bones"]);
    assert_ne!(snapshot(&d)["assets"], before["assets"]);
}
fn changed(change: impl FnOnce(&mut Vec<psd_copy::Layer>)) -> Vec<u8> {
    psd_copy::edit(SOURCE, change)
}
fn target(
    d: &Document,
) -> (
    ankhimate_core::ids::AssetId,
    ankhimate_core::ids::SlotId,
    String,
) {
    let name = d
        .psd_layer_paths
        .iter()
        .find(|(_, p)| p.as_str() == "loose_front")
        .unwrap()
        .0;
    let id = d.assets.by_name(name).unwrap();
    let ((slot,key),_)=d.skeleton.skins[d.skeleton.default_skin].entries.iter().find(|(_,a)|matches!(a,ankhimate_core::attachment::Attachment::Region(r) if &r.texture==name)).unwrap();
    (id, *slot, key.clone())
}
#[test]
fn exact_state_and_older_history_survive_three_cycles_without_source_reads() {
    use ankhimate_document::commands::bone_cmds::SetBoneTransform;
    let mut d = seed();
    let mut h = History::default();
    let bone = d.skeleton.update_order[0];
    let mut t = d.skeleton.bones[bone].local_transform;
    t.position.x += 13.;
    h.push(Box::new(SetBoneTransform::new(bone, t)), &mut d);
    let before = snapshot(&d);
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert_eq!(r.layers.changed, vec!["loose_front"]);
    h.push(Box::new(cmd.unwrap()), &mut d);
    let after = snapshot(&d);
    drop(source);
    assert_eq!(h.undo_depth(), 2);
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert_eq!(snapshot(&d), before);
        assert!(h.redo(&mut d));
        assert_eq!(snapshot(&d), after);
    }
    assert!(h.undo(&mut d));
    assert_eq!(h.undo_label(), Some("Transform Bone"));
    assert!(h.undo(&mut d));
    assert!(h.redo(&mut d));
    assert_eq!(snapshot(&d), before);
}
#[test]
fn resized_region_preserves_authored_pivot_transform_and_pixel_scale() {
    use ankhimate_core::attachment::Attachment;
    let mut d = seed();
    let (id, slot, key) = target(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, key.clone()))
        .unwrap()
    else {
        panic!()
    };
    r.width = 24.;
    r.height = 36.;
    r.pivot = glam::vec2(0.2, 0.7);
    r.local_offset = glam::vec2(17., 3.);
    r.local_rotation = 0.37;
    r.local_scale = glam::vec2(0.8, 1.4);
    let old = r.clone();
    let source = changed(|ls| {
        let l = ls.iter_mut().find(|l| l.name() == "loose_front").unwrap();
        l.recolor();
        l.resize(4, 5);
    });
    let (cmd, report) =
        ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(report.conflicts.is_empty(), "{report:?}");
    let mut h = History::default();
    h.push(Box::new(cmd.unwrap()), &mut d);
    let Attachment::Region(r) = d.skeleton.skins[skin].get(slot, &key).unwrap() else {
        panic!()
    };
    assert_eq!((r.width, r.height), (8., 15.));
    assert_eq!(
        (r.pivot, r.local_offset, r.local_rotation, r.local_scale),
        (
            old.pivot,
            old.local_offset,
            old.local_rotation,
            old.local_scale
        )
    );
    assert_eq!(
        (d.assets.images[id].width, d.assets.images[id].height),
        (4, 5)
    );
    let pixels = image::load_from_memory(&d.assets.images[id].bytes).unwrap();
    assert_eq!((pixels.width(), pixels.height()), (4, 5));
}
#[test]
fn same_leaf_name_in_distinct_nested_groups_updates_only_the_exact_path() {
    let mut d = seed();
    let before = d.assets.clone();
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "twin")
            .unwrap()
            .recolor()
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(r.conflicts.is_empty(), "{r:?}");
    assert_eq!(r.layers.changed.len(), 1);
    assert!(r.layers.changed[0].ends_with("/twin"));
    History::default().push(Box::new(cmd.unwrap()), &mut d);
    let modified: Vec<_> = d
        .assets
        .images
        .iter()
        .filter(|(id, a)| a.bytes != before.images[*id].bytes)
        .collect();
    assert_eq!(modified.len(), 1);
    assert_eq!(d.psd_layer_paths[&modified[0].1.name], r.layers.changed[0]);
}
#[test]
fn identical_reimport_is_noop_and_does_not_add_history() {
    let d = seed();
    let before = snapshot(&d);
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, SOURCE).unwrap();
    assert!(cmd.is_none());
    assert!(r.conflicts.is_empty());
    assert_eq!(r.layers.unchanged, 7);
    assert_eq!(snapshot(&d), before);
}
#[test]
fn additions_removals_and_renames_only_report_and_never_mutate() {
    let d = seed();
    let before = snapshot(&d);
    for kind in 0..4 {
        let source = changed(|ls| {
            let i = ls.iter().position(|l| l.name() == "loose_front").unwrap();
            match kind {
                0 => {
                    let mut extra = ls[i].clone();
                    extra.rename("new_front");
                    ls.insert(i, extra);
                }
                1 => {
                    ls.remove(i);
                }
                2 => ls[i].rename("renamed_front"),
                _ => ls
                    .iter_mut()
                    .find(|l| l.name() == "branch [bone]")
                    .unwrap()
                    .rename("renamed [bone]"),
            }
        });
        let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
        assert!(cmd.is_none());
        assert!(r.conflicts.is_empty(), "{r:?}");
        assert!(!r.layers.added.is_empty() || !r.layers.removed.is_empty());
        if kind >= 2 {
            assert!(!r.layers.added.is_empty() && !r.layers.removed.is_empty());
        }
        assert_eq!(snapshot(&d), before);
    }
}
#[test]
fn ambiguous_source_or_document_paths_refuse_every_change() {
    let d = seed();
    let duplicate = include_bytes!("../../formats/tests/fixtures/nested_duplicate_siblings.psd");
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, duplicate).unwrap();
    assert!(cmd.is_none() && !r.conflicts.is_empty());
    let mut d = seed();
    let (id, _, _) = target(&d);
    let mut asset = d.assets.images[id].clone();
    asset.name = "duplicate".into();
    d.assets.add(asset);
    d.psd_layer_paths
        .insert("duplicate".into(), "loose_front".into());
    let before = snapshot(&d);
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, SOURCE).unwrap();
    assert!(cmd.is_none() && !r.conflicts.is_empty());
    assert_eq!(snapshot(&d), before);
}
#[test]
fn hidden_modified_layer_is_refused_without_partial_updates() {
    let d = seed();
    let before = snapshot(&d);
    let source = changed(|ls| {
        let l = ls.iter_mut().find(|l| l.name() == "loose_front").unwrap();
        l.recolor();
        l.hide();
        ls.iter_mut()
            .find(|l| l.name() == "back")
            .unwrap()
            .recolor();
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(cmd.is_none());
    assert!(r.conflicts.iter().any(|e| e.contains("hidden")));
    assert_eq!(snapshot(&d), before);
}
#[test]
fn shared_skin_images_or_weighted_mesh_ffd_refuse_entire_batch() {
    use ankhimate_core::animation::{Key, Timeline};
    use ankhimate_core::attachment::{Attachment, MeshAttachment, VertexWeight};
    for shared in [true, false] {
        let mut d = seed();
        let (_, slot, key) = target(&d);
        let skin = d.skeleton.default_skin;
        let original = d.skeleton.skins[skin].get(slot, &key).unwrap().clone();
        if shared {
            let royal = d
                .skeleton
                .add_skin(ankhimate_core::skin::Skin::new("royal"));
            d.skeleton.skins[royal].set(slot, key.clone(), original);
        } else {
            let Attachment::Region(r) = original else {
                panic!()
            };
            let bone = d.skeleton.slots[slot].bone;
            let mut m = MeshAttachment {
                texture: r.texture,
                setup_vertices: vec![glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y],
                triangles: vec![[0, 1, 2]],
                uvs: vec![glam::Vec2::ZERO; 3],
                ..Default::default()
            };
            m.weights = vec![vec![VertexWeight { bone, weight: 1. }]; 3];
            d.skeleton.skins[skin].set(slot, key.clone(), Attachment::Mesh(m));
            d.animations
                .values_mut()
                .next()
                .unwrap()
                .timelines
                .push(Timeline::Deform {
                    slot,
                    attachment: key,
                    keys: vec![
                        Key::linear(0., vec![glam::Vec2::ZERO; 3]),
                        Key::stepped(1., vec![glam::Vec2::ONE; 3]),
                    ],
                });
        }
        let before = snapshot(&d);
        let source = changed(|ls| {
            ls.iter_mut()
                .find(|l| l.name() == "loose_front")
                .unwrap()
                .recolor();
            ls.iter_mut()
                .find(|l| l.name() == "back")
                .unwrap()
                .recolor();
        });
        let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
        assert!(cmd.is_none() && !r.conflicts.is_empty());
        assert_eq!(snapshot(&d), before);
    }
}
#[test]
fn source_parse_failure_and_prepared_preview_cancel_leave_document_exact() {
    let d = seed();
    let before = snapshot(&d);
    assert!(ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, b"invalid").is_err());
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    let (cmd, _) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    drop(cmd);
    assert_eq!(snapshot(&d), before);
}
#[test]
fn changed_pixels_preserve_other_skins_constraints_curves_and_sampled_bone_poses() {
    use ankhimate_core::{
        animation::{Axis, Key, Timeline},
        attachment::Attachment,
        pose::{Pose, evaluate},
    };
    let mut d = seed();
    let bone = d.skeleton.update_order[0];
    let a = d.animations.values_mut().next().unwrap();
    a.timelines.push(Timeline::BoneTranslate {
        bone,
        axis: Axis::Y,
        keys: vec![Key::linear(0., 0.), Key::stepped(1., 10.)],
    });
    a.timelines.push(Timeline::BoneScale {
        bone,
        axis: Axis::X,
        keys: vec![Key::linear(0., 1.), Key::linear(1., 1.2)],
    });
    let (_, slot, key) = target(&d);
    let old = d.skeleton.skins[d.skeleton.default_skin]
        .get(slot, &key)
        .unwrap()
        .clone();
    let Attachment::Region(mut alt) = old else {
        panic!()
    };
    let id = d.assets.add(ankhimate_core::assets::ImageAsset::new(
        "royal",
        vec![9, 8, 7],
        1,
        1,
    ));
    alt.texture = d.assets.images[id].name.clone();
    let skin = d
        .skeleton
        .add_skin(ankhimate_core::skin::Skin::new("royal"));
    d.skeleton.skins[skin].set(slot, key, Attachment::Region(alt));
    let ids: Vec<_> = d.skeleton.update_order.to_vec();
    d.skeleton
        .add_constraint(ankhimate_core::constraints::Constraint::Ik(
            ankhimate_core::constraints::IkConstraint::two_bone(
                "kept IK",
                ids[3],
                [ids[0], ids[1]],
            ),
        ));
    let before = snapshot(&d);
    let samples: Vec<_> = [0., 0.25, 0.5, 0.75, 1., 2.]
        .into_iter()
        .map(|time| {
            let mut p = Pose::new();
            evaluate(
                &d.skeleton,
                &[(d.animations.values().next().unwrap(), time, 1.)],
                &mut p,
            );
            serde_json::to_value(&p.worlds).unwrap()
        })
        .collect();
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(r.conflicts.is_empty());
    History::default().push(Box::new(cmd.unwrap()), &mut d);
    let after = snapshot(&d);
    for field in ["bones", "slots", "skins", "animations", "constraints"] {
        assert_eq!(after[field], before[field], "{field}");
    }
    for (i, time) in [0., 0.25, 0.5, 0.75, 1., 2.].into_iter().enumerate() {
        let mut p = Pose::new();
        evaluate(
            &d.skeleton,
            &[(d.animations.values().next().unwrap(), time, 1.)],
            &mut p,
        );
        assert_eq!(serde_json::to_value(&p.worlds).unwrap(), samples[i]);
    }
}
#[test]
fn corrupt_existing_image_or_dimensions_refuse_the_entire_operation() {
    for corrupt_bytes in [true, false] {
        let mut d = seed();
        let (id, _, _) = target(&d);
        if corrupt_bytes {
            d.assets.images[id].bytes = vec![1, 2, 3];
        } else {
            d.assets.images[id].width += 1;
        }
        let before = snapshot(&d);
        let source = changed(|ls| {
            ls.iter_mut()
                .find(|l| l.name() == "loose_front")
                .unwrap()
                .recolor()
        });
        let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
        assert!(cmd.is_none());
        assert!(r.conflicts.iter().any(|e| e.contains("verified")));
        assert_eq!(snapshot(&d), before);
    }
}
#[test]
fn identical_rgba_in_a_differently_encoded_png_is_unchanged() {
    use image::ImageEncoder;
    let mut d = seed();
    let (id, _, _) = target(&d);
    let old = &mut d.assets.images[id];
    let rgba = image::load_from_memory(&old.bytes).unwrap().to_rgba8();
    let mut encoded = vec![];
    image::codecs::png::PngEncoder::new_with_quality(
        &mut encoded,
        image::codecs::png::CompressionType::Best,
        image::codecs::png::FilterType::NoFilter,
    )
    .write_image(
        rgba.as_raw(),
        rgba.width(),
        rgba.height(),
        image::ExtendedColorType::Rgba8,
    )
    .unwrap();
    old.bytes = encoded;
    let before = snapshot(&d);
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, SOURCE).unwrap();
    assert!(cmd.is_none());
    assert!(r.conflicts.is_empty());
    assert_eq!(r.layers.unchanged, 7);
    assert_eq!(snapshot(&d), before);
}
#[test]
fn sequence_backed_regions_are_refused_without_replacing_the_attachment() {
    use ankhimate_core::attachment::{Attachment, Sequence};
    let mut d = seed();
    let (_, slot, key) = target(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, key))
        .unwrap()
    else {
        panic!()
    };
    r.sequence = Some(Sequence {
        frames: vec![r.texture.clone()],
        ..Default::default()
    });
    let before = snapshot(&d);
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .recolor()
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(cmd.is_none() && !r.conflicts.is_empty());
    assert_eq!(snapshot(&d), before);
}
#[test]
fn a_present_but_cleared_layer_is_reported_as_refused_not_missing() {
    let d = seed();
    let before = snapshot(&d);
    let source = changed(|ls| {
        let l = ls.iter_mut().find(|l| l.name() == "loose_front").unwrap();
        let (_, alpha) = l.channels.iter_mut().find(|(id, _)| *id == -1).unwrap();
        alpha[2..].fill(0);
        ls.iter_mut()
            .find(|l| l.name() == "back")
            .unwrap()
            .recolor();
    });
    let (cmd, r) = ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(cmd.is_none());
    assert!(!r.layers.removed.contains(&"loose_front".into()));
    assert!(r.conflicts.iter().any(|e| e.contains("still present")));
    assert_eq!(snapshot(&d), before);
}
#[test]
fn updating_another_layer_preserves_unaffected_weighted_mesh_and_ffd_keys() {
    use ankhimate_core::{
        animation::{Key, Timeline},
        attachment::{Attachment, MeshAttachment, VertexWeight},
    };
    let mut d = seed();
    let (_, slot, key) = target(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin].get(slot, &key).unwrap().clone() else {
        panic!()
    };
    let bone = d.skeleton.slots[slot].bone;
    let mesh = MeshAttachment {
        texture: r.texture,
        setup_vertices: vec![glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y],
        uvs: vec![glam::Vec2::ZERO; 3],
        triangles: vec![[0, 1, 2]],
        weights: vec![vec![VertexWeight { bone, weight: 1. }]; 3],
        ..Default::default()
    };
    d.skeleton.skins[skin].set(slot, key.clone(), Attachment::Mesh(mesh));
    d.animations
        .values_mut()
        .next()
        .unwrap()
        .timelines
        .push(Timeline::Deform {
            slot,
            attachment: key,
            keys: vec![
                Key::linear(0., vec![glam::Vec2::ZERO; 3]),
                Key::stepped(1., vec![glam::Vec2::ONE; 3]),
            ],
        });
    let before = snapshot(&d);
    let source = changed(|ls| {
        ls.iter_mut()
            .find(|l| l.name() == "back")
            .unwrap()
            .recolor()
    });
    let (cmd, report) =
        ankhimate_document::psd_reimport::ReimportPsd::prepare(&d, &source).unwrap();
    assert!(report.conflicts.is_empty(), "{report:?}");
    History::default().push(Box::new(cmd.unwrap()), &mut d);
    let after = snapshot(&d);
    for field in ["bones", "slots", "skins", "animations", "constraints"] {
        assert_eq!(after[field], before[field], "{field}");
    }
    assert_ne!(after["assets"], before["assets"]);
}
