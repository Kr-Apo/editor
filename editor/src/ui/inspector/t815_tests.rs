use super::*;
use ankhimate_core::{
    attachment::{Attachment, MeshAttachment},
    skeleton::Bone,
    slot::Slot,
};
pub(super) fn fixture() -> (
    AppState,
    ankhimate_core::ids::SkinId,
    ankhimate_core::ids::SlotId,
) {
    let mut s = AppState::default();
    let bone = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = s.doc.skeleton.add_slot(Slot {
        attachment: Some("art".into()),
        ..Slot::new("slot".into(), bone)
    });
    let skin = s.doc.skeleton.default_skin;
    let mesh = MeshAttachment {
        texture: "image".into(),
        setup_vertices: vec![
            glam::vec2(0.12345679, 2.0),
            glam::vec2(10.0, 2.0),
            glam::vec2(10.0, 10.0),
        ],
        uvs: vec![
            glam::vec2(0.1, 0.2),
            glam::vec2(0.9, 0.2),
            glam::vec2(0.9, 0.9),
        ],
        triangles: vec![[0, 1, 2]],
        ..Default::default()
    };
    s.doc.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(mesh));
    s.session.select_slot(Some(slot));
    s.session.mesh_edit = true;
    s.session.selected_vertices = vec![0];
    s.refresh_pose();
    (s, skin, slot)
}
#[test]
fn t815_numeric_geometry_requires_explicit_apply_and_exact_readout() {
    let (mut s, skin, slot) = fixture();
    let ctx = egui::Context::default();
    let out = ctx.run_ui(Default::default(), |ui| {
        vertex_coordinates(ui, &mut s, skin, slot, "art")
    });
    let text = out
        .shapes
        .iter()
        .filter_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                Some(t.galley.text())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert!(
        text.iter().any(|s| s.contains("Apply coordinates")),
        "Existing coordinate editor commits immediately; no Apply"
    );
    assert!(
        text.contains(&"0.12345679"),
        "Numeric position loses presentation precision"
    );
}
#[test]
fn t815_switch_attachment_cannot_reuse_another_mesh_vertex_indices() {
    let (mut s, skin, slot) = fixture();
    let copy = s.doc.skeleton.skins[skin].get(slot, "art").unwrap().clone();
    s.doc.skeleton.skins[skin].set(slot, "other", copy);
    let bone = s.doc.skeleton.slots[slot].bone;
    s.session.select_attachment(slot, "other", bone);
    assert!(
        s.session.selected_vertices.is_empty(),
        "Old mesh index remains selected on a different attachment"
    );
}

use crate::geometry_edit::{self as geometry, Domain, Draft};
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    attachment::{BoundingBoxAttachment, PathAttachment, VertexWeight},
    math::Transform,
    transforms::Affine2,
};
use ankhimate_document::commands::mesh_cmds::{EditMesh, MeshEdit};
fn snapshot(s: &AppState) -> String {
    ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
}
fn draft(
    s: &mut AppState,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
    domain: Domain,
) -> Draft {
    Draft::new(s, skin, slot, "art", domain).unwrap()
}
fn mesh(
    s: &AppState,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
) -> &MeshAttachment {
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin].get(slot, "art").unwrap() else {
        panic!()
    };
    m
}
fn weighted() -> (
    AppState,
    ankhimate_core::ids::SkinId,
    ankhimate_core::ids::SlotId,
) {
    let (mut s, skin, slot) = fixture();
    let root = s.doc.skeleton.slots[slot].bone;
    let child = s.doc.skeleton.add_bone(Bone {
        name: "arm".into(),
        parent: Some(root),
        length: 4.0,
        local_transform: Transform {
            position: glam::vec2(3.0, -2.0),
            rotation: 0.3,
            scale: glam::vec2(1.2, 0.8),
            ..Default::default()
        },
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "art".into()))
        .unwrap()
    else {
        panic!()
    };
    m.weights = vec![
        vec![
            VertexWeight {
                bone: root,
                weight: 0.3
            },
            VertexWeight {
                bone: child,
                weight: 0.7
            }
        ];
        3
    ];
    m.ffd_keyframes
        .push(ankhimate_core::attachment::FfdKeyframe {
            time: 0.5,
            vertex_offsets: vec![glam::vec2(0.25, 0.75); 6],
        });
    let mut clip = Animation::new("move", 1.0);
    clip.timelines.push(Timeline::BoneRotate {
        bone: child,
        keys: vec![Key::linear(0.0, 0.0), Key::linear(1.0, 40.0)],
    });
    clip.timelines.push(Timeline::Deform {
        slot,
        attachment: "art".into(),
        keys: vec![
            Key::linear(0.0, vec![glam::vec2(0.2, 0.3); 6]),
            Key::linear(1.0, vec![glam::vec2(0.6, 0.8); 6]),
        ],
    });
    s.doc.animations.insert(clip);
    let mut bytes = Vec::new();
    image::RgbaImage::from_pixel(4, 4, image::Rgba([100, 120, 140, 255]))
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "image", bytes, 4, 4,
    ));
    s.rebind_meshes();
    s.refresh_pose();
    (s, skin, slot)
}
fn shape_fingerprint(m: &MeshAttachment) -> String {
    let mut x = m.clone();
    x.setup_vertices.clear();
    serde_json::to_string(&x).unwrap()
}
fn bind_fingerprint(m: &MeshAttachment) -> Vec<(String, [f32; 6])> {
    let mut out = m
        .inverse_bind_matrices
        .iter()
        .map(|(b, m)| (format!("{b:?}"), [m.a, m.b, m.c, m.d, m.tx, m.ty]))
        .collect::<Vec<_>>();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}
#[test]
fn t815_set_x_y_xy_and_negative_decimal_values_preserve_other_fields() {
    for fields in [["-5.1234565", ""], ["", "9.876543"], ["-8.25", "7.5"]] {
        let (mut s, skin, slot) = weighted();
        let before = mesh(&s, skin, slot).clone();
        let binds = bind_fingerprint(&before);
        let original = snapshot(&s);
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.fields = fields.map(str::to_owned);
        d.apply(&mut s).unwrap();
        let after = mesh(&s, skin, slot);
        assert_eq!(shape_fingerprint(after), shape_fingerprint(&before));
        assert_eq!(bind_fingerprint(after), binds);
        assert_eq!(&after.setup_vertices[1..], &before.setup_vertices[1..]);
        for (axis, field) in fields.iter().enumerate() {
            let expected = if field.is_empty() {
                before.setup_vertices[0][axis]
            } else {
                field.parse().unwrap()
            };
            assert_eq!(after.setup_vertices[0][axis], expected);
        }
        let result = snapshot(&s);
        for _ in 0..3 {
            s.undo();
            assert_eq!(snapshot(&s), original);
            assert_eq!(bind_fingerprint(mesh(&s, skin, slot)), binds);
            s.redo();
            assert_eq!(snapshot(&s), result);
        }
    }
}
#[test]
fn t815_uv_exact_values_do_not_change_geometry_weights_or_bind() {
    let (mut s, skin, slot) = weighted();
    let before = mesh(&s, skin, slot).clone();
    let bind = bind_fingerprint(&before);
    let mut d = draft(&mut s, skin, slot, Domain::Uv);
    d.fields = ["0.12345679".into(), "0.8765432".into()];
    d.apply(&mut s).unwrap();
    let m = mesh(&s, skin, slot);
    assert_eq!(m.uvs[0], glam::vec2(0.12345679, 0.8765432));
    assert_eq!(&m.uvs[1..], &before.uvs[1..]);
    assert_eq!(m.setup_vertices, before.setup_vertices);
    assert_eq!(bind_fingerprint(m), bind);
    let mut normalized = m.clone();
    normalized.uvs = before.uvs.clone();
    assert_eq!(
        serde_json::to_value(&normalized).unwrap(),
        serde_json::to_value(before).unwrap()
    );
}
#[test]
fn t815_invalid_values_are_atomic_and_preserve_redo() {
    for domain in [Domain::Position, Domain::Uv] {
        for value in ["NaN", "inf", "-inf", "1e100", "bad", "120/2"] {
            let (mut s, skin, slot) = fixture();
            let mut good = draft(&mut s, skin, slot, Domain::Position);
            good.fields = ["4".into(), "5".into()];
            good.apply(&mut s).unwrap();
            let expected = snapshot(&s);
            s.undo();
            s.session.selected_vertices = vec![0];
            geometry::bind(&mut s);
            let before = snapshot(&s);
            let mut d = draft(&mut s, skin, slot, domain);
            d.fields = [value.into(), "0.4".into()];
            assert!(d.apply(&mut s).is_err(), "{value:?}");
            assert_eq!(snapshot(&s), before);
            s.redo();
            assert_eq!(snapshot(&s), expected);
        }
    }
}
#[test]
fn t815_uv_outside_range_is_refused_without_clamping() {
    for x in ["-0.0001", "1.00001"] {
        let (mut s, skin, slot) = fixture();
        let before = snapshot(&s);
        let mut d = draft(&mut s, skin, slot, Domain::Uv);
        d.fields = [x.into(), "0.4".into()];
        assert!(d.apply(&mut s).is_err());
        assert_eq!(snapshot(&s), before);
    }
}
#[test]
fn t815_multi_set_and_add_are_explicit_and_one_undo() {
    for relative in [false, true] {
        let (mut s, skin, slot) = weighted();
        s.session.selected_vertices = vec![0, 1];
        let before = snapshot(&s);
        let old = mesh(&s, skin, slot).setup_vertices.clone();
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.relative = relative;
        d.fields = ["5.5".into(), String::new()];
        d.apply(&mut s).unwrap();
        for (i, old_point) in old.iter().enumerate().take(2) {
            assert_eq!(
                mesh(&s, skin, slot).setup_vertices[i].x,
                if relative { old_point.x + 5.5 } else { 5.5 }
            );
            assert_eq!(mesh(&s, skin, slot).setup_vertices[i].y, old_point.y);
        }
        s.undo();
        assert_eq!(snapshot(&s), before);
    }
}
#[test]
fn t815_apply_operations_do_not_merge_with_drag_or_each_other() {
    let (mut s, skin, slot) = fixture();
    let original = snapshot(&s);
    s.dispatch(Box::new(EditMesh::new(
        skin,
        slot,
        "art",
        MeshEdit::MoveVertices(vec![(0, glam::vec2(3.0, 4.0))]),
    )));
    geometry::sync(&mut s);
    s.session.selected_vertices = vec![0];
    geometry::bind(&mut s);
    let drag = snapshot(&s);
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields = ["5".into(), "6".into()];
    d.apply(&mut s).unwrap();
    let one = snapshot(&s);
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "7".into();
    d.apply(&mut s).unwrap();
    s.undo();
    assert_eq!(snapshot(&s), one);
    s.undo();
    assert_eq!(snapshot(&s), drag);
    s.undo();
    assert_eq!(snapshot(&s), original);
}
#[test]
fn t815_cancel_and_reading_do_not_mutate_document_or_history() {
    let (mut s, skin, slot) = fixture();
    let before = snapshot(&s);
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields = ["999".into(), "-888".into()];
    let _cancelled = draft(&mut s, skin, slot, Domain::Position);
    assert_eq!(snapshot(&s), before);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_stale_revision_topology_external_command_and_redo_cannot_retarget_indices() {
    let (mut s, skin, slot) = fixture();
    let d = draft(&mut s, skin, slot, Domain::Position);
    s.dispatch(Box::new(EditMesh::new(
        skin,
        slot,
        "art",
        MeshEdit::AddVertex(glam::vec2(7.0, 8.0)),
    )));
    let after = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert!(s.session.selected_vertices.is_empty());
    assert_eq!(snapshot(&s), after);
    s.undo();
    geometry::sync(&mut s);
    assert!(s.session.selected_vertices.is_empty());
    s.redo();
    geometry::sync(&mut s);
    assert!(s.session.selected_vertices.is_empty());
}
#[test]
fn t815_context_changes_invalidate_selection_and_active_geometry_gestures() {
    for which in 0..5 {
        let (mut s, skin, slot) = fixture();
        geometry::bind(&mut s);
        s.session.dragging_vertex = Some(0);
        s.session.vertex_box_start = Some(glam::Vec2::ZERO);
        match which {
            0 => s.session.tool = crate::session::Tool::WeightPaint,
            1 => s.session.mesh_edit = false,
            2 => s.set_work_mode(crate::session::WorkMode::Animate),
            3 => {
                let alt = s
                    .doc
                    .skeleton
                    .skins
                    .insert(ankhimate_core::skin::Skin::new("alt"));
                s.session.active_skin = alt;
            }
            _ => {
                s.doc.skeleton.skins[skin].remove(slot, "art");
            }
        }
        geometry::sync(&mut s);
        assert!(s.session.selected_vertices.is_empty(), "context {which}");
        assert!(s.session.dragging_vertex.is_none());
        assert!(s.session.vertex_box_start.is_none());
    }
}
#[test]
fn t815_project_replace_and_attachment_switch_invalidate_old_drafts() {
    let (mut s, skin, slot) = fixture();
    let d = draft(&mut s, skin, slot, Domain::Position);
    s.replace_document(Default::default());
    assert!(d.apply(&mut s).is_err());
    assert!(s.session.selected_vertices.is_empty());
}
#[test]
fn t815_setup_only_numeric_and_native_command_boundary() {
    let (mut s, skin, slot) = fixture();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "8".into();
    s.set_work_mode(crate::session::WorkMode::Animate);
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert!(
        !s.dispatch(Box::new(
            EditMesh::new(
                skin,
                slot,
                "art",
                MeshEdit::MoveVertices(vec![(0, glam::Vec2::ZERO)])
            )
            .isolated()
        ))
    );
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_missing_or_locked_vertices_and_bones_are_refused() {
    let (mut s, skin, slot) = weighted();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "8".into();
    let bone = s.doc.skeleton.slots[slot].bone;
    s.session.locked_bones.insert(bone, true);
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    s.session.locked_bones.insert(bone, false);
    s.session.selected_vertices = vec![999];
    assert!(Draft::new(&mut s, skin, slot, "art", Domain::Position).is_err());
}
#[test]
fn t815_invalid_mesh_topology_or_binds_are_refused() {
    for which in 0..3 {
        let (mut s, skin, slot) = weighted();
        let Attachment::Mesh(m) = s.doc.skeleton.skins[skin]
            .entries
            .get_mut(&(slot, "art".into()))
            .unwrap()
        else {
            panic!()
        };
        match which {
            0 => m.triangles.push([0, 1, 999]),
            1 => {
                m.uvs.pop();
            }
            _ => m.inverse_bind_matrices.clear(),
        };
        let before = snapshot(&s);
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.fields[0] = "3".into();
        assert!(d.apply(&mut s).is_err());
        assert_eq!(snapshot(&s), before);
    }
}
#[test]
fn t815_relative_result_overflow_and_world_overflow_are_refused() {
    let (mut s, skin, slot) = fixture();
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "art".into()))
        .unwrap()
    else {
        panic!()
    };
    m.setup_vertices[0].x = f32::MAX;
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.relative = true;
    d.fields = [f32::MAX.to_string(), String::new()];
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    let (mut s, skin, slot) = fixture();
    let bone = s.doc.skeleton.slots[slot].bone;
    s.doc.skeleton.bones[bone].local_transform.scale = glam::vec2(2.0, 1.0);
    s.refresh_pose();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = f32::MAX.to_string();
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_path_bounding_box_and_clip_use_existing_commands_atomically() {
    for kind in 0..3 {
        let (mut s, skin, slot) = fixture();
        let points = mesh(&s, skin, slot).setup_vertices.clone();
        let a = match kind {
            0 => Attachment::Path(PathAttachment {
                vertices: points,
                closed: false,
                constant_speed: true,
            }),
            1 => Attachment::BoundingBox(BoundingBoxAttachment {
                vertices: points,
                weights: Vec::new(),
            }),
            _ => Attachment::Clipping(ankhimate_core::attachment::ClippingAttachment {
                vertices: points,
                end_slot: Some("slot".into()),
            }),
        };
        s.doc.skeleton.skins[skin].set(slot, "art", a);
        geometry::bind(&mut s);
        s.session.selected_vertices = vec![0, 1];
        let before = snapshot(&s);
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.fields = ["-3.25".into(), String::new()];
        d.apply(&mut s).unwrap();
        let first = snapshot(&s);
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.fields = ["6.75".into(), String::new()];
        d.apply(&mut s).unwrap();
        s.undo();
        assert_eq!(snapshot(&s), first);
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), first);
    }
}
#[test]
fn t815_save_as_reload_retains_weighted_geometry_assets_and_ffd() {
    let (mut s, skin, slot) = weighted();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields = ["-8.123457".into(), "4.765432".into()];
    d.apply(&mut s).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("géométrie 保存.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    let mut reopened = AppState::default();
    reopened.replace_document(ankhimate_document::Document::from_loaded(loaded));
    reopened.rebind_meshes();
    let loaded_slot = reopened
        .doc
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "slot")
        .unwrap()
        .0;
    let loaded_skin = reopened.doc.skeleton.default_skin;
    assert_eq!(
        mesh(&s, skin, slot).setup_vertices,
        mesh(&reopened, loaded_skin, loaded_slot).setup_vertices
    );
    assert_eq!(
        mesh(&s, skin, slot).uvs,
        mesh(&reopened, loaded_skin, loaded_slot).uvs
    );
    assert_eq!(
        mesh(&s, skin, slot).triangles,
        mesh(&reopened, loaded_skin, loaded_slot).triangles
    );
    assert_eq!(
        s.doc.assets.images.values().next().unwrap().bytes,
        reopened.doc.assets.images.values().next().unwrap().bytes
    );
    assert_eq!(s.doc.animations.len(), reopened.doc.animations.len());
    assert!(reopened.session.selected_vertices.is_empty());
    for (a, b) in weighted_poses(&s, skin, slot).iter().zip(weighted_poses(
        &reopened,
        loaded_skin,
        loaded_slot,
    )) {
        assert!((*a - b).length() < 0.0001);
    }
}
#[test]
fn t815_camera_local_world_screen_and_zoom_mapping_are_consistent() {
    use crate::ui::canvas::camera::{Camera2D, MAX_ZOOM, MIN_ZOOM, t_to_zoom, zoom_to_t};
    let transform = Transform {
        position: glam::vec2(12.0, -8.0),
        rotation: 0.73,
        scale: glam::vec2(1.3, 0.8),
        shear: glam::vec2(0.2, -0.1),
    };
    let affine = Affine2::compose(&transform);
    let inverse = affine.invert().unwrap();
    for size in [
        glam::Vec2::ZERO,
        glam::vec2(800.0, 600.0),
        glam::vec2(4096.0, 2160.0),
    ] {
        for zoom in [MIN_ZOOM, 1.0, MAX_ZOOM] {
            let camera = Camera2D {
                position: glam::vec2(7.0, -9.0),
                zoom,
            };
            let p = glam::vec2(4.0, 5.0);
            let world = affine.transform_point(p);
            let restored = inverse
                .transform_point(camera.screen_to_world(camera.world_to_screen(world, size), size));
            assert!((restored - p).length() < 0.01);
            let t = zoom_to_t(zoom);
            assert!((t_to_zoom(t) - zoom).abs() < 0.0001);
        }
    }
}
#[test]
fn t815_selection_away_and_back_discards_an_old_draft_without_document_changes() {
    let (mut s, skin, slot) = fixture();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "5".into();
    let before = snapshot(&s);
    s.session.selected_vertices = vec![1];
    geometry::sync(&mut s);
    s.session.selected_vertices = vec![0];
    geometry::sync(&mut s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_linked_mesh_and_its_source_are_read_only() {
    let (mut s, skin, slot) = fixture();
    let mut linked = mesh(&s, skin, slot).clone();
    linked.linked = Some(ankhimate_core::attachment::LinkedMesh {
        skin: None,
        slot: "slot".into(),
        attachment: "art".into(),
        inherit_deform: true,
    });
    s.doc.skeleton.skins[skin].set(slot, "linked", Attachment::Mesh(linked));
    let before = snapshot(&s);
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "8".into();
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    s.doc.skeleton.slots[slot].attachment = Some("linked".into());
    geometry::sync(&mut s);
    s.session.selected_vertices = vec![0];
    let mut d = Draft::new(&mut s, skin, slot, "linked", Domain::Position).unwrap();
    d.fields[0] = "4".into();
    let before = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}
fn weighted_poses(
    s: &AppState,
    skin: ankhimate_core::ids::SkinId,
    slot: ankhimate_core::ids::SlotId,
) -> Vec<glam::Vec2> {
    let clip = s.doc.animations.values().next().unwrap();
    let m = mesh(s, skin, slot);
    let mut all = Vec::new();
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let mut pose = ankhimate_core::pose::Pose::new();
        ankhimate_core::pose::evaluate(&s.doc.skeleton, &[(clip, t, 1.0)], &mut pose);
        let world = pose.world(s.doc.skeleton.slots[slot].bone);
        let deform = pose.deforms.get(&(slot, "art".into())).unwrap();
        for i in 0..m.setup_vertices.len() {
            all.push(m.skin_vertex_with_deform(
                i,
                deform.get(m.influence_range(i)).unwrap(),
                &pose,
                &world,
            ));
        }
    }
    all
}
#[test]
fn t815_weighted_ffd_deformation_is_exact_after_three_undo_redo_cycles() {
    let (mut s, skin, slot) = weighted();
    let before = weighted_poses(&s, skin, slot);
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields = ["5.25".into(), "6.75".into()];
    d.apply(&mut s).unwrap();
    let after = weighted_poses(&s, skin, slot);
    assert_ne!(before, after);
    for _ in 0..3 {
        s.undo();
        assert_eq!(weighted_poses(&s, skin, slot), before);
        s.redo();
        assert_eq!(weighted_poses(&s, skin, slot), after);
    }
}
#[test]
fn t815_valid_approval_cannot_apply_twice() {
    let (mut s, skin, slot) = fixture();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.relative = true;
    d.fields = ["1".into(), String::new()];
    d.apply(&mut s).unwrap();
    let after = snapshot(&s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), after);
}
#[test]
fn t815_nearly_equal_coordinates_are_not_displayed_as_one_rounded_value() {
    let (mut s, skin, slot) = fixture();
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "art".into()))
        .unwrap()
    else {
        panic!()
    };
    m.setup_vertices[0].x = 1.000001;
    m.setup_vertices[1].x = 1.000002;
    s.session.selected_vertices = vec![0, 1];
    let d = draft(&mut s, skin, slot, Domain::Position);
    assert!(d.fields[0].is_empty());
    assert_eq!(d.fields[1], "2");
}
#[test]
fn t815_headless_numeric_apply_and_keyboard_draft_are_connected() {
    let (mut s, skin, slot) = fixture();
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let frame = |s: &mut AppState, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| geometry::ui(ui, s, skin, slot, "art", Domain::Position),
        )
    };
    let find = |o: &egui::FullOutput, text: &str| {
        o.shapes
            .iter()
            .find_map(|x| {
                if let egui::Shape::Text(t) = &x.shape
                    && t.galley.text() == text
                {
                    Some(t.pos + t.galley.size() * 0.5)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Missing {text}"))
    };
    frame(&mut s, vec![]);
    let out = frame(&mut s, vec![]);
    let pos = find(&out, "0.12345679");
    frame(&mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    frame(
        &mut s,
        vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            },
            egui::Event::Text("-12.345679".into()),
        ],
    );
    assert_eq!(snapshot(&s), before);
    let out = frame(&mut s, vec![]);
    let pos = find(&out, "Apply coordinates");
    frame(&mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    assert_eq!(mesh(&s, skin, slot).setup_vertices[0].x, -12.345679);
    s.undo();
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_uv_pane_click_shares_vertex_selection_with_numeric_inspector() {
    let (mut s, skin, slot) = fixture();
    geometry::bind(&mut s);
    s.session.uv_pane = Some(crate::ui::uv::UvPane::new(skin, slot, "art".into()));
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let frame = |s: &mut AppState, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| crate::ui::uv::ui(ui, s, &crate::theme::Theme::default()),
        )
    };
    frame(&mut s, vec![]);
    let out = frame(&mut s, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| {
            if let egui::Shape::Circle(c) = &s.shape
                && c.radius == 3.0
            {
                Some(c.center)
            } else {
                None
            }
        })
        .unwrap();
    frame(&mut s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    assert_eq!(s.session.selected_vertices, vec![1]);
    let d = draft(&mut s, skin, slot, Domain::Uv);
    assert_eq!(d.fields, ["0.9", "0.2"]);
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_uv_pane_target_is_cleared_after_a_skin_or_attachment_switch() {
    let (mut s, skin, slot) = fixture();
    geometry::bind(&mut s);
    s.session.uv_pane = Some(crate::ui::uv::UvPane::new(skin, slot, "art".into()));
    s.doc.skeleton.slots[slot].attachment = None;
    geometry::sync(&mut s);
    assert!(s.session.uv_pane.is_none());
    assert!(s.session.selected_vertices.is_empty());
}
#[test]
fn t815_one_invalid_vertex_prevents_the_entire_numeric_batch() {
    let (mut s, skin, slot) = fixture();
    geometry::bind(&mut s);
    s.session.selected_vertices = vec![0, 999];
    let before = snapshot(&s);
    assert!(Draft::new(&mut s, skin, slot, "art", Domain::Position).is_err());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t815_hidden_geometry_panel_skin_and_tool_roundtrips_discard_drafts() {
    for which in 0..2 {
        let (mut s, skin, slot) = fixture();
        let mut d = draft(&mut s, skin, slot, Domain::Position);
        d.fields[0] = "5".into();
        let before = snapshot(&s);
        if which == 0 {
            s.session.set_tool(crate::session::Tool::WeightPaint);
            s.session.set_tool(crate::session::Tool::Select);
        } else {
            let alt = s
                .doc
                .skeleton
                .skins
                .insert(ankhimate_core::skin::Skin::new("alt"));
            let before_switch = snapshot(&s);
            s.session.set_active_skin(alt);
            s.session.set_active_skin(skin);
            assert_eq!(snapshot(&s), before_switch);
        }
        s.session.selected_vertices = vec![0];
        geometry::bind(&mut s);
        assert!(d.apply(&mut s).is_err());
        if which == 0 {
            assert_eq!(snapshot(&s), before);
        }
    }
}
#[test]
fn t815_hidden_panel_mode_roundtrip_disallows_an_old_draft() {
    let (mut s, skin, slot) = weighted();
    let mut d = draft(&mut s, skin, slot, Domain::Position);
    d.fields[0] = "5".into();
    let before = snapshot(&s);
    s.set_work_mode(crate::session::WorkMode::Animate);
    s.set_work_mode(crate::session::WorkMode::Setup);
    s.session.selected_vertices = vec![0];
    geometry::bind(&mut s);
    assert!(d.apply(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}
