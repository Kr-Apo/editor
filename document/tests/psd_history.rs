//! M003.1: exercise the real PSD command with a populated document and history.
use ankhimate_core::{assets::ImageAsset, attachment::Attachment};
use ankhimate_document::{
    Document, History,
    commands::{bone_cmds::SetBoneTransform, psd_cmds::ImportPsd},
};
use ankhimate_formats::psd::{self, ImportOptions, PsdImport};
use serde_json::{Value, json};

fn imported() -> PsdImport {
    psd::import(
        include_bytes!("../../formats/tests/fixtures/tagged_rig.psd"),
        &ImportOptions::default(),
    )
    .unwrap()
}

// Include persisted data, pixels, slotmap identities and derived update order.
// Schema equality alone would miss regenerated IDs and detached asset bytes.
fn snapshot(doc: &Document) -> Value {
    let skins: Vec<_> = doc.skeleton.skins.iter().map(|(id, skin)| {
        let mut entries: Vec<_> = skin.entries.iter().collect();
        entries.sort_by_key(|(key, _)| (*key).clone());
        json!({"id": id, "name": skin.name, "entries": entries, "bones": skin.bones, "constraints": skin.constraints})
    }).collect();
    json!({
        "schema": ankhimate_formats::convert::to_schema(&doc.as_project_ref()),
        "assets": doc.assets,
        "bones": doc.skeleton.bones,
        "slots": doc.skeleton.slots,
        "skins": skins,
        "constraints": doc.skeleton.constraints,
        "constraint_order": doc.skeleton.constraint_order,
        "groups": doc.skeleton.groups,
        "group_order": doc.skeleton.group_order,
        "animations": doc.animations,
        "update_order": doc.skeleton.update_order,
        "draw_order": doc.skeleton.draw_order,
        "default_skin": doc.skeleton.default_skin,
    })
}

fn populated() -> Document {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
    let mut doc = Document::from_loaded(ankhimate_formats::load(&path).unwrap().0);
    doc.meta.name = "existing animated project".into();
    doc.meta.fps = 60;
    doc.psd_layer_paths
        .insert("existing".into(), "old/layer".into());
    doc.export_presets
        .push(json!({"future": "retained verbatim"}));
    doc
}

fn cycles(replace: bool) {
    cycles_with(replace, imported());
}

fn cycles_with(replace: bool, source: PsdImport) {
    let mut doc = populated();
    let original = snapshot(&doc);
    let bone = doc.skeleton.update_order[0];
    let old_local = doc.skeleton.bones[bone].local_transform;
    let mut local = old_local;
    local.position.x += 17.0;
    let mut history = History::default();
    history.push(Box::new(SetBoneTransform::new(bone, local)), &mut doc);
    let before = snapshot(&doc);
    history.push(
        Box::new(ImportPsd::new(source, replace, "tagged")),
        &mut doc,
    );
    let after = snapshot(&doc);
    assert_ne!(after, before);
    assert_eq!(history.undo_depth(), 2);
    for cycle in 0..3 {
        assert!(history.undo(&mut doc));
        assert_eq!(snapshot(&doc), before, "Undo cycle {cycle}");
        assert_eq!(history.undo_label(), Some("Transform Bone"));
        assert_eq!(history.redo_label(), Some("Import PSD"));
        assert!(history.redo(&mut doc));
        assert_eq!(snapshot(&doc), after, "Redo cycle {cycle}");
    }
    assert!(history.undo(&mut doc));
    assert!(history.undo(&mut doc));
    assert_eq!(snapshot(&doc), original, "the older command still works");
    assert!(history.redo(&mut doc));
    assert_eq!(snapshot(&doc), before);
    assert!(history.redo(&mut doc));
    assert_eq!(snapshot(&doc), after);
}

#[test]
fn nested_psd_replace_keeps_assets_and_older_history_over_three_cycles() {
    let imported = psd::import(
        include_bytes!("../../formats/tests/fixtures/nested_groups.psd"),
        &ImportOptions::default(),
    )
    .unwrap();
    assert_eq!(imported.summary.images, 7);
    cycles_with(true, imported);
}

#[test]
fn nested_psd_merge_keeps_assets_and_older_history_over_three_cycles() {
    let imported = psd::import(
        include_bytes!("../../formats/tests/fixtures/nested_groups.psd"),
        &ImportOptions::default(),
    )
    .unwrap();
    assert_eq!(imported.summary.images, 7);
    cycles_with(false, imported);
}

#[test]
fn psd_replace_restores_complete_document_over_three_undo_redo_cycles() {
    cycles(true);
}

#[test]
fn psd_merge_restores_complete_document_over_three_undo_redo_cycles() {
    cycles(false);
}

#[test]
fn psd_history_still_undoes_an_older_bone_creation_after_redo_recreates_its_id() {
    use ankhimate_document::commands::bone_cmds::CreateBone;
    let mut doc = populated();
    let before = snapshot(&doc);
    let mut history = History::default();
    let bone = ankhimate_core::skeleton::Bone {
        name: "before import".into(),
        parent: None,
        length: 1.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: ankhimate_core::skeleton::Bone::default_color(),
    };
    history.push(Box::new(CreateBone::new(bone)), &mut doc);
    history.push(
        Box::new(ImportPsd::new(imported(), false, "tagged")),
        &mut doc,
    );
    let after = snapshot(&doc);
    for _ in 0..3 {
        assert!(history.undo(&mut doc));
        assert!(history.undo(&mut doc));
        // A normal CreateBone undo leaves a vacant slotmap generation. Compare
        // every authored field rather than treating allocator bookkeeping as data.
        assert!(
            serde_json::to_value(ankhimate_formats::convert::to_schema(&doc.as_project_ref()))
                .unwrap()
                == before["schema"],
            "an older creation must remain undoable"
        );
        assert_eq!(serde_json::to_value(&doc.assets).unwrap(), before["assets"]);
        assert!(history.redo(&mut doc));
        assert!(history.redo(&mut doc));
        assert!(
            snapshot(&doc) == after,
            "the imported state retains its identities"
        );
    }
}

#[test]
fn psd_replace_does_not_keep_animation_references_to_the_previous_skeleton() {
    let mut doc = populated();
    assert!(!doc.animations.is_empty());
    let before = snapshot(&doc);
    let mut history = History::default();
    history.push(
        Box::new(ImportPsd::new(imported(), true, "tagged")),
        &mut doc,
    );
    assert!(
        doc.animations.is_empty(),
        "PSD contains no animations; previous timelines belong to the replaced skeleton"
    );
    assert!(history.undo(&mut doc));
    assert_eq!(snapshot(&doc), before);
}

#[test]
fn psd_merge_remaps_colliding_assets_and_sequence_frames_without_changing_old_pixels() {
    let mut source = imported();
    // Cover both supported texture-bearing attachment kinds, without changing
    // the PSD fixture: turn its sequence region into an equivalent mesh in memory.
    let default_skin = source.skeleton.default_skin;
    for attachment in source.skeleton.skins[default_skin].entries.values_mut() {
        if let Attachment::Region(region) = attachment
            && region.sequence.is_some()
        {
            let mut mesh = ankhimate_core::attachment::MeshAttachment::from_region(region);
            mesh.sequence = region.sequence.clone();
            *attachment = Attachment::Mesh(mesh);
        }
    }
    let mut doc = populated();
    for image in source.assets.images.values() {
        doc.assets.add(ImageAsset::new(
            &image.name,
            vec![1, 2, 3],
            image.width,
            image.height,
        ));
        doc.psd_layer_paths
            .insert(image.name.clone(), "original provenance".into());
    }
    let before = snapshot(&doc);
    let old_slots = doc.skeleton.draw_order.len();
    let source_slots = source.skeleton.draw_order.clone();
    let source_skeleton = source.skeleton.clone();
    let source_assets = source.assets.clone();
    let paths = source.layer_paths.clone();
    let mut history = History::default();
    history.push(Box::new(ImportPsd::new(source, false, "tagged")), &mut doc);
    for (source_skin_id, skin) in source_skeleton.skins.iter() {
        let target_skin = if source_skin_id == source_skeleton.default_skin {
            doc.skeleton.default_skin
        } else {
            doc.skeleton
                .skins
                .iter()
                .find(|(_, s)| s.name == skin.name)
                .unwrap()
                .0
        };
        for ((slot, name), attachment) in &skin.entries {
            let index = source_slots.iter().position(|id| id == slot).unwrap();
            let target_slot = doc.skeleton.draw_order[old_slots + index];
            let target = doc.skeleton.skins[target_skin]
                .get(target_slot, name)
                .unwrap();
            let textures = |a: &Attachment| -> Vec<String> {
                match a {
                    Attachment::Region(r) => std::iter::once(r.texture.clone())
                        .chain(r.sequence.iter().flat_map(|s| s.frames.clone()))
                        .collect(),
                    Attachment::Mesh(m) => std::iter::once(m.texture.clone())
                        .chain(m.sequence.iter().flat_map(|s| s.frames.clone()))
                        .collect(),
                    _ => vec![],
                }
            };
            let originals = textures(attachment);
            let merged_textures = textures(target);
            assert_eq!(
                originals.len(),
                merged_textures.len(),
                "no sequence frames may disappear"
            );
            for (original, merged) in originals.iter().zip(merged_textures) {
                assert_ne!(
                    &merged, original,
                    "new attachment must not sample the old colliding asset"
                );
                let image = &doc.assets.images[doc.assets.by_name(&merged).unwrap()];
                assert_eq!(
                    image.bytes,
                    source_assets.images[source_assets.by_name(original).unwrap()].bytes
                );
                assert_eq!(
                    doc.assets.images[doc.assets.by_name(original).unwrap()].bytes,
                    vec![1, 2, 3]
                );
                assert_eq!(doc.psd_layer_paths[original], "original provenance");
                assert_eq!(doc.psd_layer_paths[&merged], paths[original]);
            }
        }
    }
    let after = snapshot(&doc);
    for _ in 0..3 {
        assert!(history.undo(&mut doc));
        assert_eq!(snapshot(&doc), before);
        assert!(history.redo(&mut doc));
        assert_eq!(snapshot(&doc), after);
    }
}
