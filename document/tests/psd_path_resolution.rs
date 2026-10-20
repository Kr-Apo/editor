use ankhimate_document::{Document, History, commands::psd_cmds::ImportPsd};
use ankhimate_formats::psd::{self, ImportOptions};
#[path = "../../formats/tests/support/psd_copy.rs"]
#[allow(
    dead_code,
    reason = "Shared fixture copier; cases differ across integration suites"
)]
mod psd_copy;
const SOURCE: &[u8] = include_bytes!("../../formats/tests/fixtures/nested_groups.psd");
fn seed() -> Document {
    let mut doc = Document::new();
    History::default().push(
        Box::new(ImportPsd::new(
            psd::import(SOURCE, &ImportOptions::default()).unwrap(),
            true,
            "nested",
        )),
        &mut doc,
    );
    doc
}
fn renamed() -> Vec<u8> {
    psd_copy::edit(SOURCE, |ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .rename("renamed_front")
    })
}
#[test]
fn t822_explicit_rename_can_prepare_a_provenance_update() {
    let doc = seed();
    let bytes = renamed();
    let (command, report) =
        ankhimate_document::psd_reimport::ReimportPsd::prepare(&doc, &bytes).unwrap();
    assert_eq!(report.layers.removed, ["loose_front"]);
    assert!(report.layers.added.contains(&"renamed_front".into()));
    assert!(
        command.is_none(),
        "Without explicit mapping the historical report-only contract stays intact"
    );
    let plan = ankhimate_document::psd_reimport::ReimportPsd::prepare_path_change(
        &doc,
        &bytes,
        "loose_front",
        "renamed_front",
    )
    .unwrap();
    assert_eq!(plan.preview.old_path, "loose_front");
    assert!(plan.preview.same_pixels);
}

use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    attachment::{Attachment, LinkedMesh, MeshAttachment, Sequence},
};
use ankhimate_document::{psd_reimport::ReimportPsd, snapshot::snapshot};
const ORIGINAL: &[u8] = include_bytes!("fixtures/t822/original.psd");
const RENAMED: &[u8] = include_bytes!("fixtures/t822/renamed.psd");
const MOVED: &[u8] = include_bytes!("fixtures/t822/moved.psd");
const RESIZED: &[u8] = include_bytes!("fixtures/t822/resized.psd");
fn simple() -> Document {
    let mut d = Document::new();
    History::default().push(
        Box::new(ImportPsd::new(
            psd::import(ORIGINAL, &ImportOptions::default()).unwrap(),
            true,
            "hero",
        )),
        &mut d,
    );
    let bone = d.skeleton.update_order[0];
    let mut a = Animation::new("kept", 3.);
    a.timelines.push(Timeline::BoneRotate {
        bone,
        keys: vec![
            Key::linear(0., 0.),
            Key::stepped(1., 0.7),
            Key::linear(3., 0.),
        ],
    });
    d.animations.insert(a);
    d
}
fn name(d: &Document) -> String {
    d.psd_layer_paths
        .iter()
        .find(|(_, p)| *p == "hero/arm/hand")
        .unwrap()
        .0
        .clone()
}
fn owner(d: &Document) -> (ankhimate_core::ids::SlotId, String) {
    let n = name(d);
    d.skeleton.skins[d.skeleton.default_skin]
        .entries
        .iter()
        .find(|(_, a)| matches!(a,Attachment::Region(r) if r.texture==n))
        .map(|(k, _)| k.clone())
        .unwrap()
}
fn check_refused(d: &Document, source: &[u8], asset: &str, path: &str) {
    let before = snapshot(d);
    assert!(ReimportPsd::prepare_path_change(d, source, asset, path).is_err());
    assert!(snapshot(d) == before);
}
fn apply(d: &mut Document, source: &[u8], path: &str) -> History {
    let p = ReimportPsd::prepare_path_change(d, source, &name(d), path).unwrap();
    let c = p.into_command(d, source).unwrap();
    let mut h = History::default();
    h.push(Box::new(c), d);
    h
}
#[test]
fn t822_same_pixels_changes_only_provenance_and_retains_encoded_bytes_and_ids() {
    use image::ImageEncoder;
    let mut d = simple();
    let n = name(&d);
    let id = d.assets.by_name(&n).unwrap();
    let a = &mut d.assets.images[id];
    let rgba = image::load_from_memory(&a.bytes).unwrap().to_rgba8();
    let mut bytes = vec![];
    image::codecs::png::PngEncoder::new_with_quality(
        &mut bytes,
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
    a.bytes = bytes;
    let mut expected = d.clone();
    expected
        .psd_layer_paths
        .insert(n.clone(), "hero/arm/hand_right".into());
    let plan = ReimportPsd::prepare_path_change(&d, RENAMED, &n, "hero/arm/hand_right").unwrap();
    assert!(plan.preview.same_pixels);
    apply(&mut d, RENAMED, "hero/arm/hand_right");
    assert!(snapshot(&d) == snapshot(&expected));
    assert_eq!(d.assets.by_name(&n), Some(id));
}
#[test]
fn t822_group_move_changes_pixels_preserving_full_author_structure() {
    let mut d = simple();
    let mut expected = d.clone();
    let n = name(&d);
    let id = d.assets.by_name(&n).unwrap();
    let incoming = psd::import(MOVED, &ImportOptions::default()).unwrap();
    let next = incoming
        .layer_paths
        .iter()
        .find(|(_, p)| *p == "hero/arms/arm/hand")
        .unwrap()
        .0;
    let new = &incoming.assets.images[incoming.assets.by_name(next).unwrap()];
    expected.assets.images[id].bytes = new.bytes.clone();
    expected
        .psd_layer_paths
        .insert(n, "hero/arms/arm/hand".into());
    let plan =
        ReimportPsd::prepare_path_change(&d, MOVED, &name(&d), "hero/arms/arm/hand").unwrap();
    assert!(!plan.preview.same_pixels);
    apply(&mut d, MOVED, "hero/arms/arm/hand");
    assert!(snapshot(&d) == snapshot(&expected));
}
#[test]
fn t822_resize_32_to_64_48_reuses_exact_region_policy_and_preserves_pivot_uv() {
    let mut d = simple();
    let (slot, key) = owner(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, key.clone()))
        .unwrap()
    else {
        panic!()
    };
    r.width = 24.;
    r.height = 40.;
    r.pivot = glam::vec2(0.2, 0.7);
    r.local_offset = glam::vec2(17., 3.);
    r.local_rotation = 0.37;
    r.local_scale = glam::vec2(0.8, 1.4);
    let mut expected = d.clone();
    expected
        .psd_layer_paths
        .insert(name(&d), "hero/arm_right/hand".into());
    let (command, report) = ReimportPsd::prepare(&expected, RESIZED).unwrap();
    assert!(report.conflicts.is_empty());
    History::default().push(Box::new(command.unwrap()), &mut expected);
    let p =
        ReimportPsd::prepare_path_change(&d, RESIZED, &name(&d), "hero/arm_right/hand").unwrap();
    assert_eq!(p.preview.old_size, (32, 32));
    assert_eq!(p.preview.new_size, (64, 48));
    apply(&mut d, RESIZED, "hero/arm_right/hand");
    assert!(snapshot(&d) == snapshot(&expected));
    let Attachment::Region(r) = d.skeleton.skins[skin].get(slot, &key).unwrap() else {
        panic!()
    };
    assert_eq!((r.width, r.height), (48., 60.));
}
#[test]
fn t822_three_exact_undo_redo_cycles_need_no_source_and_keep_older_history() {
    let mut d = simple();
    let mut h = History::default();
    let bone = d.skeleton.update_order[0];
    let t = d.skeleton.bones[bone].local_transform;
    h.push(
        Box::new(ankhimate_document::commands::bone_cmds::SetBoneTransform::new(bone, t)),
        &mut d,
    );
    let before = snapshot(&d);
    let source = RESIZED.to_vec();
    let plan =
        ReimportPsd::prepare_path_change(&d, &source, &name(&d), "hero/arm_right/hand").unwrap();
    h.push(Box::new(plan.into_command(&d, &source).unwrap()), &mut d);
    drop(source);
    let after = snapshot(&d);
    assert_eq!(h.undo_depth(), 2);
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert!(snapshot(&d) == before);
        assert!(h.redo(&mut d));
        assert!(snapshot(&d) == after);
    }
    assert!(h.undo(&mut d));
    assert_eq!(h.undo_label(), Some("Transform Bone"));
}
#[test]
fn t822_prepare_cancel_and_refusal_preserve_document_and_redo() {
    let mut d = simple();
    let n = name(&d);
    let mut h = apply(&mut d, RENAMED, "hero/arm/hand_right");
    h.undo(&mut d);
    let before = snapshot(&d);
    drop(ReimportPsd::prepare_path_change(&d, RENAMED, &n, "hero/arm/hand_right").unwrap());
    check_refused(&d, RENAMED, &n, "missing");
    assert!(h.can_redo());
    assert!(snapshot(&d) == before);
    assert!(h.redo(&mut d));
}
#[test]
fn t822_document_or_source_snapshot_change_invalidates_the_plan() {
    let d = simple();
    let p =
        ReimportPsd::prepare_path_change(&d, RENAMED, &name(&d), "hero/arm/hand_right").unwrap();
    let mut newer = d.clone();
    newer.meta.name = "new document".into();
    assert!(p.clone().into_command(&newer, RENAMED).is_err());
    assert!(p.into_command(&d, MOVED).is_err());
}
#[test]
fn t822_duplicate_paths_and_claimed_destinations_are_refused() {
    let mut d = simple();
    let n = name(&d);
    check_refused(
        &d,
        include_bytes!("fixtures/t822/duplicate.psd"),
        &n,
        "hero/arm/hand_right",
    );
    let anchor = d
        .psd_layer_paths
        .iter()
        .find(|(_, p)| *p == "anchor")
        .unwrap()
        .0
        .clone();
    d.psd_layer_paths
        .insert(anchor, "hero/arm/hand_right".into());
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}
#[test]
fn t822_multiple_assets_claiming_old_provenance_are_refused() {
    let mut d = simple();
    let n = name(&d);
    let mut a = d.assets.images[d.assets.by_name(&n).unwrap()].clone();
    a.name = "second".into();
    d.assets.add(a);
    d.psd_layer_paths
        .insert("second".into(), "hero/arm/hand".into());
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}
#[test]
fn t822_other_missing_source_affiliation_is_not_guessed() {
    let mut d = simple();
    let n = name(&d);
    let anchor = d
        .psd_layer_paths
        .iter()
        .find(|(_, p)| *p == "anchor")
        .unwrap()
        .0
        .clone();
    d.psd_layer_paths
        .insert(anchor, "separate_psd/anchor".into());
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}
#[test]
fn t822_missing_asset_provenance_is_refused() {
    let mut d = simple();
    let n = name(&d);
    d.psd_layer_paths
        .insert("missing_image".into(), "other".into());
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}
#[test]
fn t822_existing_or_absent_destination_and_unknown_old_asset_are_refused() {
    let d = simple();
    let n = name(&d);
    check_refused(&d, RENAMED, &n, "anchor");
    check_refused(&d, RENAMED, &n, "not present");
    check_refused(&d, RENAMED, "unknown", "hero/arm/hand_right");
    check_refused(&d, ORIGINAL, &n, "hero/arm/hand");
}
#[test]
fn t822_bad_source_pixels_and_dimensions_are_refused() {
    let mut d = simple();
    let n = name(&d);
    check_refused(&d, b"not PSD", &n, "hero/arm/hand_right");
    let id = d.assets.by_name(&n).unwrap();
    d.assets.images[id].bytes = vec![];
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
    let mut d = simple();
    let id = d.assets.by_name(&n).unwrap();
    d.assets.images[id].width += 1;
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
    let empty = psd_copy::edit(RENAMED, |ls| {
        let l = ls.iter_mut().find(|l| l.name() == "hand_right").unwrap();
        for (id, plane) in &mut l.channels {
            if *id == -1 {
                plane[2..].fill(0)
            }
        }
    });
    check_refused(&simple(), &empty, &n, "hero/arm/hand_right");
}
#[test]
fn t822_hidden_new_layer_semantic_tags_sequence_and_flattening_are_refused() {
    let d = simple();
    let n = name(&d);
    for (source, path) in [
        (
            include_bytes!("fixtures/t822/semantic.psd").as_slice(),
            "hero/arm/hand_right [mesh]",
        ),
        (
            include_bytes!("fixtures/t822/sequence.psd").as_slice(),
            "hero/arm/hand_01",
        ),
        (
            include_bytes!("fixtures/t822/flattened.psd").as_slice(),
            "hero/arm_right [merge]",
        ),
    ] {
        check_refused(&d, source, &n, path);
    }
    let hidden = psd_copy::edit(RENAMED, |ls| {
        ls.iter_mut()
            .find(|l| l.name() == "hand_right")
            .unwrap()
            .hide()
    });
    check_refused(&d, &hidden, &n, "hero/arm/hand_right");
}
#[test]
fn t822_shared_region_and_skin_override_are_refused_even_for_identical_pixels() {
    for same_image in [true, false] {
        let mut d = simple();
        let n = name(&d);
        let (slot, key) = owner(&d);
        let mut a = d.skeleton.skins[d.skeleton.default_skin]
            .get(slot, &key)
            .unwrap()
            .clone();
        if !same_image {
            let Attachment::Region(r) = &mut a else {
                panic!()
            };
            r.texture = "anchor".into();
        }
        let skin = d
            .skeleton
            .add_skin(ankhimate_core::skin::Skin::new("override"));
        d.skeleton.skins[skin].set(slot, key, a);
        check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
    }
}
#[test]
fn t822_mesh_weighted_linked_and_ffd_target_are_refused() {
    for kind in 0..4 {
        let mut d = simple();
        let n = name(&d);
        let (slot, key) = owner(&d);
        let skin = d.skeleton.default_skin;
        if kind == 3 {
            let mut a = Animation::new("deform", 2.);
            a.timelines.push(Timeline::Deform {
                slot,
                attachment: key.clone(),
                keys: vec![Key::linear(0., vec![])],
            });
            d.animations.insert(a);
        } else {
            let mut m = MeshAttachment {
                texture: n.clone(),
                ..Default::default()
            };
            if kind == 1 {
                m.weights = vec![vec![ankhimate_core::attachment::VertexWeight {
                    bone: d.skeleton.slots[slot].bone,
                    weight: 1.,
                }]];
            }
            if kind == 2 {
                m.linked = Some(LinkedMesh {
                    skin: None,
                    slot: d.skeleton.slots[slot].name.clone(),
                    attachment: key.clone(),
                    inherit_deform: true,
                });
            }
            d.skeleton.skins[skin].set(slot, key, Attachment::Mesh(m));
        }
        check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
    }
}
#[test]
fn t822_region_sequence_and_link_to_region_are_refused() {
    let mut d = simple();
    let n = name(&d);
    let (slot, key) = owner(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, key.clone()))
        .unwrap()
    else {
        panic!()
    };
    r.sequence = Some(Sequence {
        frames: vec![n.clone()],
        ..Default::default()
    });
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
    let mut d = simple();
    d.skeleton.skins[skin].set(
        slot,
        "link",
        Attachment::Mesh(MeshAttachment {
            linked: Some(LinkedMesh {
                skin: None,
                slot: d.skeleton.slots[slot].name.clone(),
                attachment: key,
                inherit_deform: true,
            }),
            ..Default::default()
        }),
    );
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}
#[test]
fn t822_existing_global_conflict_refuses_mapping_and_every_safe_pixel_update() {
    let d = seed();
    let source = psd_copy::edit(SOURCE, |ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .rename("renamed_front");
        let l = ls.iter_mut().find(|l| l.name() == "back").unwrap();
        l.recolor();
        l.hide();
    });
    check_refused(&d, &source, "loose_front", "renamed_front");
}
#[test]
fn t822_safe_exact_path_updates_are_disclosed_in_the_same_atomic_command() {
    let mut d = seed();
    let source = psd_copy::edit(SOURCE, |ls| {
        ls.iter_mut()
            .find(|l| l.name() == "loose_front")
            .unwrap()
            .rename("renamed_front");
        ls.iter_mut()
            .find(|l| l.name() == "back")
            .unwrap()
            .recolor();
    });
    let before = snapshot(&d);
    let p = ReimportPsd::prepare_path_change(&d, &source, "loose_front", "renamed_front").unwrap();
    assert!(p.preview.affected_assets.contains(&"back".into()));
    assert!(p.preview.affected_assets.contains(&"loose_front".into()));
    let mut h = History::default();
    h.push(Box::new(p.into_command(&d, &source).unwrap()), &mut d);
    assert_eq!(h.undo_depth(), 1);
    h.undo(&mut d);
    assert!(snapshot(&d) == before);
}
#[test]
fn t822_nonfinite_region_geometry_is_not_silently_preserved() {
    let mut d = simple();
    let n = name(&d);
    let (slot, key) = owner(&d);
    let skin = d.skeleton.default_skin;
    let Attachment::Region(r) = d.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, key))
        .unwrap()
    else {
        panic!()
    };
    r.local_rotation = f32::NAN;
    check_refused(&d, RENAMED, &n, "hero/arm/hand_right");
}

#[test]
fn t822_unrelated_weighted_mesh_binds_ffd_and_curves_remain_exact() {
    let mut d = simple();
    let skin = d.skeleton.default_skin;
    let slot = d
        .skeleton
        .draw_order
        .iter()
        .find(|id| d.skeleton.slots[**id].name.contains("anchor"))
        .copied()
        .unwrap();
    let key = d.skeleton.slots[slot].attachment.clone().unwrap();
    let bone = d.skeleton.slots[slot].bone;
    let m = MeshAttachment {
        texture: "anchor".into(),
        setup_vertices: vec![glam::vec2(0., 0.), glam::vec2(2., 0.), glam::vec2(0., 2.)],
        uvs: vec![glam::vec2(0., 0.), glam::vec2(1., 0.), glam::vec2(0., 1.)],
        triangles: vec![[0, 1, 2]],
        weights: vec![vec![ankhimate_core::attachment::VertexWeight { bone, weight: 1. }]; 3],
        inverse_bind_matrices: std::collections::HashMap::from([(
            bone,
            ankhimate_core::transforms::Affine2::IDENTITY,
        )]),
        ..Default::default()
    };
    d.skeleton.skins[skin].set(slot, key.clone(), Attachment::Mesh(m));
    let mut a = Animation::new("ffd kept", 3.);
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: key,
        keys: vec![
            Key::linear(0., vec![]),
            Key::stepped(1., vec![glam::vec2(1., 2.); 3]),
            Key::linear(3., vec![]),
        ],
    });
    d.animations.insert(a);
    let mut expected = d.clone();
    expected
        .psd_layer_paths
        .insert(name(&d), "hero/arm/hand_right".into());
    apply(&mut d, RENAMED, "hero/arm/hand_right");
    assert!(snapshot(&d) == snapshot(&expected));
}

#[test]
fn t822_old_ui_flattened_group_is_not_reinterpreted_as_a_leaf() {
    let mut d = Document::new();
    let mut options = ImportOptions::default();
    options.flatten.insert("hero/arm".into());
    History::default().push(
        Box::new(ImportPsd::new(
            psd::import(ORIGINAL, &options).unwrap(),
            true,
            "flat",
        )),
        &mut d,
    );
    let n = d
        .psd_layer_paths
        .iter()
        .find(|(_, p)| *p == "hero/arm")
        .unwrap()
        .0
        .clone();
    check_refused(&d, RESIZED, &n, "hero/arm_right/hand");
}
