use super::*;
use ankhimate_core::{
    assets::ImageAsset,
    attachment::{Attachment, Rect, RegionAttachment},
    ids::{SkinId, SlotId},
    skeleton::Bone,
    skin::Skin,
    slot::Slot,
};
use ankhimate_document::commands::attachment_cmds::SetRegionImage;

#[test]
fn skin_image_library_import_does_not_create_a_slot_or_replace_shared_pixels() {
    let (mut state, royal, slot) = fixture();
    let source = tempfile::tempdir().unwrap();
    let path = source.path().join("new_art.png");
    let image = state
        .doc
        .assets
        .get(state.doc.assets.by_name("coiffe_royal").unwrap())
        .unwrap()
        .bytes
        .clone();
    std::fs::write(&path, &image).unwrap();
    let before = snapshot(&state);
    crate::ui::canvas::add_image_file(&mut state, &path);
    assert_eq!(state.doc.skeleton.slots.len(), 1);
    assert_eq!(texture(&state, royal, slot), "coiffe_blue");
    assert_eq!(state.doc.assets.images.len(), 3);
    assert_eq!(std::fs::read(&path).unwrap(), image);
    state.undo();
    assert_eq!(snapshot(&state), before);
}

fn snapshot(state: &AppState) -> String {
    ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
}

fn texture(state: &AppState, skin: SkinId, slot: SlotId) -> &str {
    let Attachment::Region(r) = state.doc.skeleton.resolve(skin, slot, "coiffe").unwrap() else {
        panic!()
    };
    &r.texture
}

fn assign(state: &mut AppState, skin: SkinId, slot: SlotId, image: &str) {
    let cmd = SetRegionImage::new(&state.doc, skin, slot, "coiffe", image).unwrap();
    assert!(state.dispatch(Box::new(cmd)));
}

fn fixture() -> (AppState, SkinId, SlotId) {
    let mut state = AppState::default();
    let bone = state.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 1.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = state.doc.skeleton.add_slot(Slot {
        attachment: Some("coiffe".into()),
        ..Slot::new("coiffe_slot".into(), bone)
    });
    for (name, color) in [
        ("coiffe_blue", [0, 40, 255, 255]),
        ("coiffe_royal", [255, 0, 180, 255]),
    ] {
        let mut bytes = Vec::new();
        image::RgbaImage::from_pixel(8, 8, image::Rgba(color))
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            )
            .unwrap();
        state.doc.assets.add(ImageAsset::new(name, bytes, 8, 8));
    }
    let region = Attachment::Region(RegionAttachment {
        texture: "coiffe_blue".into(),
        local_offset: glam::Vec2::ZERO,
        local_rotation: 0.0,
        local_scale: glam::Vec2::ONE,
        width: 8.0,
        height: 8.0,
        pivot: glam::Vec2::splat(0.5),
        sequence: None,
        uv_rect: Rect {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        },
    });
    let default = state.doc.skeleton.default_skin;
    state.doc.skeleton.skins[default].set(slot, "coiffe", region.clone());
    let royal = state.doc.skeleton.add_skin(Skin::new("royal"));
    state.doc.skeleton.skins[royal].set(slot, "coiffe", region);
    state.session.active_skin = royal;
    state.refresh_pose();
    (state, royal, slot)
}

#[test]
fn skin_image_inspector_provides_an_independent_image_choice() {
    let (mut state, _, slot) = fixture();
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(650.0, 1600.0),
            )),
            ..Default::default()
        },
        |ui| {
            attachment_inspector(ui, &mut state, slot);
        },
    );
    let tree = out.platform_output.accesskit_update.unwrap();
    let choices = tree
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == egui::accesskit::Role::ComboBox)
        .count();
    eprintln!("independent attachment image selectors: {choices}");
    assert_eq!(
        choices, 1,
        "the inspector only shows a read-only image name; cannot choose a per-skin image"
    );
}

#[test]
fn skin_image_assignment_is_local_and_undo_restores_the_complete_document() {
    let (mut state, royal, slot) = fixture();
    let default = state.doc.skeleton.default_skin;
    let other = state.doc.skeleton.add_skin(Skin::new("other"));
    let region = state.doc.skeleton.skins[default]
        .get(slot, "coiffe")
        .unwrap()
        .clone();
    state.doc.skeleton.skins[other].set(slot, "coiffe", region.clone());
    state.doc.skeleton.skins[royal].set(slot, "other_attachment", region);
    state
        .doc
        .animations
        .insert(ankhimate_core::animation::Animation::new("frappe_sol", 3.0));
    let before = snapshot(&state);
    let assets: Vec<_> = state
        .doc
        .assets
        .images
        .values()
        .map(|a| a.bytes.clone())
        .collect();
    assign(&mut state, royal, slot, "coiffe_royal");
    assert_eq!(texture(&state, royal, slot), "coiffe_royal");
    assert_eq!(texture(&state, default, slot), "coiffe_blue");
    assert_eq!(texture(&state, other, slot), "coiffe_blue");
    assert_eq!(state.doc.skeleton.slots.len(), 1);
    assert_eq!(
        state.doc.skeleton.slots[slot].attachment.as_deref(),
        Some("coiffe")
    );
    let after = snapshot(&state);
    // Compare everything except the single expected texture reference.
    let mut repaired = state.doc.clone();
    let Attachment::Region(r) = repaired.skeleton.skins[royal]
        .entries
        .get_mut(&(slot, "coiffe".into()))
        .unwrap()
    else {
        panic!()
    };
    r.texture = "coiffe_blue".into();
    assert_eq!(
        ankhimate_formats::to_json(&repaired.as_project_ref()).unwrap(),
        before
    );
    for _ in 0..3 {
        state.undo();
        assert_eq!(snapshot(&state), before);
        state.redo();
        assert_eq!(snapshot(&state), after);
    }
    assert_eq!(
        assets,
        state
            .doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn skin_image_successive_choices_are_separate_undo_steps() {
    let (mut state, royal, slot) = fixture();
    let before = snapshot(&state);
    assign(&mut state, royal, slot, "coiffe_royal");
    let first = snapshot(&state);
    assign(&mut state, royal, slot, "coiffe_blue");
    state.undo();
    assert_eq!(snapshot(&state), first);
    state.undo();
    assert_eq!(snapshot(&state), before);
    state.redo();
    state.redo();
    assert_eq!(texture(&state, royal, slot), "coiffe_blue");
}

#[test]
fn skin_image_inherited_region_creates_an_override_and_undo_restores_fallback() {
    let (mut state, royal, slot) = fixture();
    state.doc.skeleton.skins[royal].remove(slot, "coiffe");
    let before = snapshot(&state);
    assert_eq!(texture(&state, royal, slot), "coiffe_blue");
    assign(&mut state, royal, slot, "coiffe_royal");
    assert_eq!(texture(&state, royal, slot), "coiffe_royal");
    assert_eq!(
        texture(&state, state.doc.skeleton.default_skin, slot),
        "coiffe_blue"
    );
    state.undo();
    assert_eq!(snapshot(&state), before);
    assert!(
        state.doc.skeleton.skins[royal]
            .get(slot, "coiffe")
            .is_none()
    );
    state.redo();
    assert_eq!(texture(&state, royal, slot), "coiffe_royal");
}

#[test]
fn skin_image_missing_corrupt_and_inconsistent_assets_are_refused_before_history() {
    let (mut state, royal, slot) = fixture();
    state
        .doc
        .assets
        .add(ImageAsset::new("corrupt", vec![1, 2, 3], 8, 8));
    let valid = state
        .doc
        .assets
        .get(state.doc.assets.by_name("coiffe_blue").unwrap())
        .unwrap()
        .bytes
        .clone();
    state
        .doc
        .assets
        .add(ImageAsset::new("wrong_size", valid, 99, 8));
    let before = snapshot(&state);
    let depth = state.history.undo_depth();
    for (image, message) in [
        ("missing", "missing"),
        ("corrupt", "decoded"),
        ("wrong_size", "dimensions"),
    ] {
        let Err(reason) = SetRegionImage::new(&state.doc, royal, slot, "coiffe", image) else {
            panic!("accepted {image}")
        };
        assert!(reason.contains(message), "{reason}");
    }
    assert_eq!(snapshot(&state), before);
    assert_eq!(state.history.undo_depth(), depth);
}

#[test]
fn skin_image_mesh_and_sequences_are_explicitly_refused() {
    let (mut state, royal, slot) = fixture();
    state.doc.skeleton.skins[royal].set(slot, "coiffe", Attachment::Mesh(Default::default()));
    assert!(
        matches!(SetRegionImage::new(&state.doc,royal,slot,"coiffe","coiffe_royal"),Err(reason) if reason.contains("regions only"))
    );
    let mut region = state.doc.skeleton.skins[state.doc.skeleton.default_skin]
        .get(slot, "coiffe")
        .unwrap()
        .clone();
    let Attachment::Region(r) = &mut region else {
        panic!()
    };
    r.sequence = Some(ankhimate_core::attachment::Sequence {
        frames: vec!["coiffe_blue".into()],
        fps: 30.0,
        mode: ankhimate_core::attachment::SequenceMode::Loop,
        setup_index: 0,
    });
    state.doc.skeleton.skins[royal].set(slot, "coiffe", region);
    assert!(
        matches!(SetRegionImage::new(&state.doc,royal,slot,"coiffe","coiffe_royal"),Err(reason) if reason.contains("sequences"))
    );
}

#[test]
fn skin_image_assignment_is_setup_only() {
    let (mut state, royal, slot) = fixture();
    state.set_work_mode(crate::session::WorkMode::Animate);
    let before = snapshot(&state);
    let cmd = SetRegionImage::new(&state.doc, royal, slot, "coiffe", "coiffe_royal").unwrap();
    assert!(!state.dispatch(Box::new(cmd)));
    assert_eq!(snapshot(&state), before);
}

#[test]
fn skin_image_keeps_authored_geometry_and_uvs_with_different_image_dimensions() {
    let (mut state, royal, slot) = fixture();
    let mut bytes = Vec::new();
    image::RgbaImage::from_pixel(16, 24, image::Rgba([255, 0, 180, 255]))
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();
    state.doc.assets.add(ImageAsset::new("tall", bytes, 16, 24));
    let Attachment::Region(before) = state
        .doc
        .skeleton
        .resolve(royal, slot, "coiffe")
        .unwrap()
        .clone()
    else {
        panic!()
    };
    assign(&mut state, royal, slot, "tall");
    let Attachment::Region(after) = state.doc.skeleton.resolve(royal, slot, "coiffe").unwrap()
    else {
        panic!()
    };
    assert_eq!(before.local_corners(), after.local_corners());
    assert_eq!(
        serde_json::to_value(&before.uv_rect).unwrap(),
        serde_json::to_value(&after.uv_rect).unwrap()
    );
}

#[test]
fn skin_image_save_reopen_and_native_atlas_keep_both_images_without_touching_sources() {
    let (mut state, royal, slot) = fixture();
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("original.png");
    let bytes = state
        .doc
        .assets
        .get(state.doc.assets.by_name("coiffe_blue").unwrap())
        .unwrap()
        .bytes
        .clone();
    std::fs::write(&source, &bytes).unwrap();
    let original = state.doc.assets.by_name("coiffe_blue").unwrap();
    state.doc.assets.images[original].source_path = Some(source.to_string_lossy().into_owned());
    assign(&mut state, royal, slot, "coiffe_royal");
    let saved = dir.path().join("rig.ankh");
    ankhimate_formats::save(&saved, &state.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&saved).unwrap();
    assert!(loaded.report.is_clean());
    let reopened = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
        snapshot(&state)
    );
    let project = ankhimate_formats::convert::to_schema(&reopened.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &project,
        &reopened.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let json: serde_json::Value = serde_json::from_str(
        &plan
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents,
    )
    .unwrap();
    assert_eq!(json["format_version"], 2);
    let skins = json["skins"].as_array().unwrap();
    for (skin, image) in [("default", "coiffe_blue"), ("royal", "coiffe_royal")] {
        let entry = &skins.iter().find(|s| s["name"] == skin).unwrap()["attachments"][0];
        assert_eq!(entry["name"], "coiffe");
        assert_eq!(entry["slot"], "coiffe_slot");
        assert_eq!(entry["attachment"]["texture"], image);
        assert!(
            json["atlas"]["regions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["name"] == image)
        );
    }
    assert!(!plan.binaries.is_empty());
    assert_eq!(std::fs::read(source).unwrap(), bytes);
}

fn frame(
    ctx: &egui::Context,
    state: &mut AppState,
    slot: SlotId,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(650.0, 1600.0),
            )),
            events,
            ..Default::default()
        },
        |ui| attachment_inspector(ui, state, slot),
    )
}

fn click(
    ctx: &egui::Context,
    state: &mut AppState,
    slot: SlotId,
    pos: egui::Pos2,
) -> egui::FullOutput {
    frame(ctx, state, slot, vec![egui::Event::PointerMoved(pos)]);
    frame(
        ctx,
        state,
        slot,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Default::default(),
        }],
    );
    frame(
        ctx,
        state,
        slot,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    )
}

#[test]
fn skin_image_real_inspector_pointer_choice_dispatches_to_the_active_skin() {
    let (mut state, royal, slot) = fixture();
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    let out = frame(&ctx, &mut state, slot, vec![]);
    let tree = out.platform_output.accesskit_update.unwrap();
    let bounds = tree
        .nodes
        .iter()
        .find(|(_, n)| n.role() == egui::accesskit::Role::ComboBox)
        .unwrap()
        .1
        .bounds()
        .unwrap();
    click(
        &ctx,
        &mut state,
        slot,
        egui::pos2(
            (bounds.x0 + bounds.x1) as f32 / 2.0,
            (bounds.y0 + bounds.y1) as f32 / 2.0,
        ),
    );
    let out = frame(&ctx, &mut state, slot, vec![]);
    let position = out
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) if t.galley.text() == "coiffe_royal" => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            _ => None,
        })
        .expect("royal image option is visible");
    click(&ctx, &mut state, slot, position);
    assert_eq!(texture(&state, royal, slot), "coiffe_royal");
    assert_eq!(
        texture(&state, state.doc.skeleton.default_skin, slot),
        "coiffe_blue"
    );
    assert_eq!(state.history.undo_label(), Some("Assign Skin Image"));
}

#[test]
fn skin_image_missing_source_has_a_visible_diagnostic_and_can_be_repaired() {
    let (mut state, royal, slot) = fixture();
    let image = state.doc.assets.by_name("coiffe_blue").unwrap();
    state.doc.assets.remove(image);
    let out = frame(&egui::Context::default(), &mut state, slot, vec![]);
    assert!(out.shapes.iter().any(|s| matches!(&s.shape,egui::epaint::Shape::Text(t) if t.galley.text().contains("missing from the asset library"))));
    assign(&mut state, royal, slot, "coiffe_royal");
    assert_eq!(texture(&state, royal, slot), "coiffe_royal");
    state.undo();
    assert_eq!(texture(&state, royal, slot), "coiffe_blue");
}
