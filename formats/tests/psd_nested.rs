//! Actual PSD records, including section dividers and parent IDs. Names and
//! even pixels may repeat: only source structure decides which group owns art.
use ankhimate_core::{attachment::Attachment, pose::Pose};
use ankhimate_formats::psd::{self, ImportOptions, PsdImport};

const NESTED: &[u8] = include_bytes!("fixtures/nested_groups.psd");
fn read(bytes: &[u8]) -> PsdImport {
    psd::import(bytes, &ImportOptions::default()).unwrap()
}
fn order(imported: &PsdImport) -> Vec<&str> {
    imported
        .skeleton
        .draw_order
        .iter()
        .map(|id| imported.skeleton.slots[*id].attachment.as_deref().unwrap())
        .collect()
}

#[test]
fn nested_parser_ranges_include_descendants_but_parent_ids_identify_direct_children() {
    let source = ::psd::Psd::from_bytes(NESTED).unwrap();
    assert_eq!(source.layers().len(), 11);
    let (id, _) = source
        .groups()
        .iter()
        .find(|(_, g)| g.name() == "rig [bone]")
        .unwrap();
    let descendants = source.get_group_sub_layers(id).unwrap();
    assert_eq!(descendants.len(), 9);
    assert_eq!(
        descendants
            .iter()
            .filter(|l| l.parent_id() == Some(*id))
            .count(),
        3
    );
}

#[test]
fn nested_preview_contains_each_source_layer_once_at_its_real_path() {
    let tree = psd::layer_tree(NESTED).unwrap();
    let leaves: Vec<_> = tree.iter().filter(|n| !n.is_group).collect();
    eprintln!("source leaves=11; preview leaves={}", leaves.len());
    assert_eq!(leaves.len(), 11);
    let twins: Vec<_> = leaves.iter().filter(|n| n.name == "twin").collect();
    assert_eq!(twins.len(), 2);
    assert_eq!(
        twins
            .iter()
            .filter(|n| n.path == "rig [bone]/branch [bone]/deep [bone]/twin")
            .count(),
        1
    );
    assert!(leaves.iter().any(|n| n.name == "hidden" && !n.visible));
}

#[test]
fn nested_art_is_imported_once_in_source_draw_order() {
    let imported = read(NESTED);
    eprintln!(
        "nested bones={} slots={} images={}",
        imported.summary.bones, imported.summary.slots, imported.summary.images
    );
    assert_eq!(imported.summary.bones, 4);
    assert_eq!(imported.summary.slots, 7);
    assert_eq!(imported.summary.images, 7);
    assert_eq!(
        order(&imported),
        [
            "loose_back",
            "back",
            "twin",
            "twin_2",
            "accent",
            "sibling",
            "loose_front"
        ]
    );
}

#[test]
fn nested_parents_pivots_bounds_tags_and_source_pixels_survive() {
    let imported = read(NESTED);
    let skel = &imported.skeleton;
    let mut pose = Pose::new();
    ankhimate_core::pose::evaluate(skel, &[], &mut pose);
    for (name, parent, origin) in [
        ("rig", "root", [-26.0, 25.0]),
        ("branch", "rig", [-16.0, 15.0]),
        ("deep", "branch", [-9.0, 7.0]),
    ] {
        let (id, bone) = skel.bones.iter().find(|(_, b)| b.name == name).unwrap();
        assert_eq!(skel.bones[bone.parent.unwrap()].name, parent);
        let world = pose.world(id);
        assert!((world.tx - origin[0]).abs() < 0.0001 && (world.ty - origin[1]).abs() < 0.0001);
    }
    for (name, bone, path, bounds, rgba, alpha) in [
        (
            "back",
            "rig",
            "rig [bone]/back",
            (8, 9, 20, 20),
            [240, 30, 20, 255],
            1.0,
        ),
        (
            "twin",
            "branch",
            "rig [bone]/branch [bone]/twin",
            (12, 14, 8, 6),
            [20, 240, 30, 128],
            1.0,
        ),
        (
            "twin_2",
            "deep",
            "rig [bone]/branch [bone]/deep [bone]/twin",
            (20, 22, 8, 6),
            [20, 240, 30, 128],
            1.0,
        ),
        (
            "accent",
            "deep",
            "rig [bone]/branch [bone]/deep [bone]/top [slot:accent] [alpha:0.4]",
            (26, 21, 9, 8),
            [20, 30, 240, 255],
            0.4,
        ),
    ] {
        let (id, slot) = skel
            .slots
            .iter()
            .find(|(_, s)| s.attachment.as_deref() == Some(name))
            .unwrap();
        assert_eq!(skel.bones[slot.bone].name, bone);
        assert_eq!(imported.layer_paths[name], path);
        assert_eq!(slot.color[3], alpha);
        let Attachment::Region(a) = skel.resolve(skel.default_skin, id, name).unwrap() else {
            panic!()
        };
        let world = pose.world(slot.bone);
        let expected = [
            bounds.0 as f32 + bounds.2 as f32 / 2.0 - 32.0,
            32.0 - bounds.1 as f32 - bounds.3 as f32 / 2.0,
        ];
        assert!((a.local_offset.x + world.tx - expected[0]).abs() < 0.0001);
        assert!((a.local_offset.y + world.ty - expected[1]).abs() < 0.0001);
        let asset = imported
            .assets
            .get(imported.assets.by_name(name).unwrap())
            .unwrap();
        let image = image::load_from_memory(&asset.bytes).unwrap().to_rgba8();
        assert_eq!(image.dimensions(), (bounds.2, bounds.3));
        assert!(image.pixels().all(|p| p.0 == rgba));
    }
}

#[test]
fn nested_identical_named_and_identical_pixel_siblings_remain_distinct() {
    let imported = read(include_bytes!("fixtures/nested_duplicate_siblings.psd"));
    assert_eq!(order(&imported), ["same", "same_2"]);
    assert_eq!(imported.assets.len(), 2);
    assert_eq!(imported.layer_paths["same"], imported.layer_paths["same_2"]);
    let images: Vec<_> = imported.assets.images.values().collect();
    assert_eq!(images[0].bytes, images[1].bytes);
    let positions: Vec<_> = imported
        .skeleton
        .draw_order
        .iter()
        .map(|id| {
            let slot = &imported.skeleton.slots[*id];
            let Attachment::Region(a) = imported
                .skeleton
                .resolve(
                    imported.skeleton.default_skin,
                    *id,
                    slot.attachment.as_ref().unwrap(),
                )
                .unwrap()
            else {
                panic!()
            };
            a.local_offset
        })
        .collect();
    assert_ne!(positions[0], positions[1]);
}

#[test]
fn nested_hidden_leaf_can_be_included_without_duplicates() {
    let imported = psd::import(
        NESTED,
        &ImportOptions {
            skip_hidden: false,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(imported.summary.images, 8);
    assert_eq!(
        order(&imported),
        [
            "loose_back",
            "back",
            "twin",
            "twin_2",
            "accent",
            "hidden",
            "sibling",
            "loose_front"
        ]
    );
}

#[test]
fn nested_hidden_groups_hide_their_descendants() {
    let bytes = include_bytes!("fixtures/nested_hidden_group.psd");
    let imported = read(bytes);
    assert_eq!(order(&imported), ["shown"]);
    let included = psd::import(
        bytes,
        &ImportOptions {
            skip_hidden: false,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(order(&included), ["shown", "secret"]);
}

#[test]
fn nested_skin_and_folder_tags_retain_the_child_art_in_the_selected_skin() {
    let imported = read(include_bytes!("fixtures/nested_skin.psd"));
    assert_eq!(imported.summary.images, 1);
    assert_eq!(imported.summary.slots, 1);
    let (slot_id, slot) = imported.skeleton.slots.iter().next().unwrap();
    assert_eq!(imported.skeleton.bones[slot.bone].name, "hat");
    let royal = imported
        .skeleton
        .skins
        .values()
        .find(|s| s.name == "royal")
        .unwrap();
    assert!(royal.get(slot_id, "coiffe").is_some());
    assert!(
        imported.skeleton.skins[imported.skeleton.default_skin]
            .get(slot_id, "coiffe")
            .is_none()
    );
}

#[test]
fn nested_merged_group_consumes_descendants_once_and_keeps_stack_position() {
    let imported = read(include_bytes!("fixtures/nested_merge.psd"));
    assert_eq!(imported.summary.images, 3);
    assert_eq!(imported.summary.slots, 3);
    assert_eq!(
        order(&imported),
        ["below", "merged [bone] [merge]", "above"]
    );
}

#[test]
fn nested_negative_bounds_metadata_survives_without_duplicate_images() {
    let bytes = include_bytes!("fixtures/nested_negative.psd");
    let nodes = psd::layer_tree(bytes).unwrap();
    assert_eq!(
        nodes.iter().find(|n| n.name == "off_left").unwrap().bounds,
        (-3, 4, 8, 6)
    );
    assert_eq!(
        nodes.iter().find(|n| n.name == "off_top").unwrap().bounds,
        (4, -3, 8, 6)
    );
    let imported = read(bytes);
    // This small source decodes in psd 0.3.5. Pin metadata and cardinality;
    // do not claim new off-canvas resampling fidelity from this regression.
    assert_eq!(order(&imported), ["off_left", "off_top", "inside"]);
    assert_eq!(imported.summary.images, 3);
}

#[test]
fn nested_flat_and_simple_groups_keep_the_same_order_and_pixels() {
    let flat = read(include_bytes!("fixtures/flat_groups_reference.psd"));
    let simple = read(include_bytes!("fixtures/simple_groups_reference.psd"));
    assert_eq!(order(&flat), ["back", "front"]);
    assert_eq!(order(&simple), order(&flat));
    for name in ["back", "front"] {
        let bytes = |p: &PsdImport| {
            p.assets
                .get(p.assets.by_name(name).unwrap())
                .unwrap()
                .bytes
                .clone()
        };
        assert_eq!(bytes(&flat), bytes(&simple));
    }
}

#[test]
fn nested_save_and_reopen_preserve_the_entire_import_and_asset_bytes() {
    let imported = read(NESTED);
    let animations = Default::default();
    let project = ankhimate_formats::ProjectRef {
        animation_order: &[],
        skeleton: &imported.skeleton,
        animations: &animations,
        assets: &imported.assets,
        name: "nested",
        fps: 30,
        export_presets: &[],
        psd_layer_paths: &imported.layer_paths,
    };
    let before = ankhimate_formats::to_json(&project).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("nested.ankh");
    ankhimate_formats::save(&path, &project, &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.is_clean());
    let after = ankhimate_formats::ProjectRef {
        animation_order: &[],
        skeleton: &loaded.skeleton,
        animations: &loaded.animations,
        assets: &loaded.assets,
        name: &loaded.name,
        fps: loaded.fps,
        export_presets: &loaded.export_presets,
        psd_layer_paths: &loaded.psd_layer_paths,
    };
    assert_eq!(ankhimate_formats::to_json(&after).unwrap(), before);
    for asset in imported.assets.images.values() {
        assert_eq!(
            loaded
                .assets
                .get(loaded.assets.by_name(&asset.name).unwrap())
                .unwrap()
                .bytes,
            asset.bytes
        );
    }
}
