use super::*;
use ankhimate_core::{skeleton::Bone, slot::Slot};
fn fixture() -> (AppState, BoneId, SlotId) {
    let mut s = AppState::default();
    let b = s.doc.skeleton.add_bone(Bone {
        name: "paint-influence".into(),
        parent: None,
        length: 10.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = s.doc.skeleton.add_slot(Slot {
        attachment: Some("mesh".into()),
        ..Slot::new("mesh-slot".into(), b)
    });
    let skin = s.doc.skeleton.default_skin;
    s.doc.skeleton.skins[skin].set(
        slot,
        "mesh",
        Attachment::Mesh(MeshAttachment {
            setup_vertices: vec![
                glam::vec2(0.0, 0.0),
                glam::vec2(10.0, 0.0),
                glam::vec2(0.0, 10.0),
            ],
            uvs: vec![glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y],
            triangles: vec![[0, 1, 2]],
            weights: vec![
                vec![VertexWeight {
                    bone: b,
                    weight: 1.0
                }];
                3
            ],
            ..Default::default()
        }),
    );
    s.session.select_slot(Some(slot));
    s.refresh_pose();
    s.rebind_meshes();
    (s, b, slot)
}
fn snapshot(s: &AppState) -> String {
    ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
}
fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(480.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| super::ui(ui, s),
    )
}
fn text_rect(o: &egui::FullOutput, text: &str) -> egui::Rect {
    o.shapes
        .iter()
        .find_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                (t.galley.text() == text).then_some(t.visual_bounding_rect())
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("missing UI: {text}"))
}
fn click(ctx: &egui::Context, s: &mut AppState, pos: egui::Pos2) {
    frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
    frame(
        ctx,
        s,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Default::default(),
        }],
    );
    frame(
        ctx,
        s,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    );
}
#[test]
fn t816_selecting_influence_keeps_mesh_target_via_real_egui_row() {
    let (mut s, b, slot) = fixture();
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, vec![]);
    let rect = text_rect(&out, "paint-influence");
    click(&ctx, &mut s, rect.center());
    assert_eq!(
        s.session.active_bone(),
        Some(b),
        "row must actually be clicked"
    );
    assert_eq!(
        s.session.active_slot(),
        Some(slot),
        "choosing influence loses brush mesh"
    );
    assert_eq!(snapshot(&s), before);
    assert!(!s.history.can_undo());
}
#[test]
fn t816_panel_identifies_radius_and_activation_without_mutation() {
    let (mut s, _, _) = fixture();
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, vec![]);
    text_rect(&out, "Radius");
    text_rect(&out, "Activate Weight Paint");
    assert_eq!(snapshot(&s), before);
}

#[test]
fn t816_activation_click_is_session_only_and_setup_gated() {
    use crate::session::{Tool, WorkMode};
    for setup in [true, false] {
        let (mut s, _, slot) = fixture();
        if !setup {
            s.set_work_mode(WorkMode::Animate);
        }
        let before = snapshot(&s);
        let had_undo = s.history.can_undo();
        let ctx = egui::Context::default();
        let out = frame(&ctx, &mut s, vec![]);
        let pos = text_rect(&out, "Activate Weight Paint").center();
        click(&ctx, &mut s, pos);
        assert_eq!(s.session.tool == Tool::WeightPaint, setup);
        assert_eq!(s.session.active_slot(), Some(slot));
        assert_eq!(snapshot(&s), before);
        assert_eq!(s.history.can_undo(), had_undo);
        if !setup {
            text_rect(&out, "Weight editing requires Setup mode");
        }
    }
}
#[test]
fn t816_influence_pick_preserves_current_vertex_scope_binds_and_redo() {
    let (mut s, b, slot) = fixture();
    let (t, m) = target(&s).unwrap();
    dispatch(&mut s, &t, m.weights.clone(), "Test weights");
    s.undo();
    assert!(s.history.can_redo());
    s.session.mesh_edit = true;
    s.session.selected_vertices = vec![1];
    crate::geometry_edit::bind(&mut s);
    let before = snapshot(&s);
    select_influence(&mut s, b);
    assert_eq!(s.session.selected_vertices, vec![1]);
    assert_eq!(s.session.active_slot(), Some(slot));
    assert!(s.history.can_redo());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn t816_influence_pick_without_mesh_does_not_guess_a_target() {
    let (mut s, b, _) = fixture();
    s.session.select_slot(None);
    select_influence(&mut s, b);
    assert!(s.session.active_slot().is_none());
    assert!(s.session.active_bone().is_none());
    assert!(!s.history.can_undo());
}
#[test]
fn t816_locked_influence_is_identified_and_panel_does_not_paint() {
    let (mut s, b, _) = fixture();
    select_influence(&mut s, b);
    s.session.weight_paint_settings.locked.push(b);
    let before = snapshot(&s);
    let ctx = egui::Context::default();
    let out = frame(&ctx, &mut s, vec![]);
    text_rect(&out, "Influence: paint-influence (locked)");
    text_rect(&out, "Feather");
    text_rect(&out, "Weight");
    text_rect(&out, "Mode");
    text_rect(&out, "Add");
    for _ in 0..3 {
        frame(&ctx, &mut s, vec![]);
    }
    assert_eq!(snapshot(&s), before);
    assert!(!s.history.can_undo());
}
#[test]
fn t816_existing_brush_modes_bounds_locking_and_history_are_preserved() {
    let (mut s, b, _) = fixture();
    let other = s.doc.skeleton.add_bone(Bone {
        name: "other".into(),
        parent: None,
        length: 5.0,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let (t, mut mesh) = target(&s).unwrap();
    mesh.weights = vec![
        vec![
            VertexWeight {
                bone: b,
                weight: 0.4
            },
            VertexWeight {
                bone: other,
                weight: 0.6
            }
        ];
        3
    ];
    dispatch(&mut s, &t, mesh.weights.clone(), "Prepare weights");
    let before = snapshot(&s);
    for mode in BrushMode::ALL {
        let locked = weight_cmds::brush(&mesh, b, mode, &[0.0, 10.0, 1000.0], 50.0, 0.8, 0.7, &[b]);
        assert_eq!(
            locked
                .iter()
                .flatten()
                .map(|w| (w.bone, w.weight))
                .collect::<Vec<_>>(),
            mesh.weights
                .iter()
                .flatten()
                .map(|w| (w.bone, w.weight))
                .collect::<Vec<_>>()
        );
        let w = weight_cmds::brush(&mesh, b, mode, &[0.0, 10.0, 1000.0], 50.0, 0.8, 0.7, &[]);
        for v in &w {
            assert!(
                v.iter()
                    .all(|w| w.weight.is_finite() && (0.0..=1.0).contains(&w.weight))
            );
            assert!((v.iter().map(|w| w.weight).sum::<f32>() - 1.0).abs() < 1e-6);
        }
        dispatch(&mut s, &t, w, "Paint test");
        let after = snapshot(&s);
        for _ in 0..3 {
            s.undo();
            assert_eq!(snapshot(&s), before);
            s.redo();
            assert_eq!(snapshot(&s), after);
        }
        s.undo();
    }
}
#[test]
fn t816_controls_keep_existing_parameter_ranges_and_direct_mode() {
    let (mut s, _, _) = fixture();
    let ctx = egui::Context::default();
    for mode in BrushMode::ALL {
        s.session.weight_paint_settings.mode = mode;
        let out = frame(&ctx, &mut s, vec![]);
        text_rect(&out, mode.label());
    }
    s.session.weight_paint_settings.direct = true;
    let out = frame(&ctx, &mut s, vec![]);
    text_rect(&out, "Direct");
    let p = &s.session.weight_paint_settings;
    assert!((4.0..=400.0).contains(&p.radius));
    assert!((0.0..=1.0).contains(&p.feather));
    assert!((0.0..=1.0).contains(&p.strength));
    assert!(!s.history.can_undo());
}

#[test]
fn t816_weight_selection_and_native_edit_save_reload_preserve_geometry_and_asset_bytes() {
    let (mut s, b, slot) = fixture();
    let image=include_bytes!("../../../../samples/walker.assets/0f/0f3c812963629e86b9915a3efe6281aa3793718334376c01bf4d29db4561644b.png").to_vec();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "art",
        image.clone(),
        32,
        32,
    ));
    let skin = s.doc.skeleton.default_skin;
    if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "mesh".into()))
    {
        m.texture = "art".into();
    }
    let (t, m) = target(&s).unwrap();
    let points = m.setup_vertices.clone();
    let uv = m.uvs.clone();
    let triangles = m.triangles.clone();
    select_influence(&mut s, b);
    let w = weight_cmds::brush(
        &m,
        b,
        BrushMode::Replace,
        &[0.0, 100.0, 100.0],
        50.0,
        0.5,
        0.7,
        &[],
    );
    dispatch(&mut s, &t, w, "Test paint");
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("copie-épreuve.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(loaded.report.dangling.is_empty());
    assert_eq!(loaded.assets.images.values().next().unwrap().bytes, image);
    let doc = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(
        snapshot(&s),
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap()
    );
    let Attachment::Mesh(mesh) = doc.skeleton.skins[doc.skeleton.default_skin]
        .entries
        .values()
        .next()
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(mesh.setup_vertices, points);
    assert_eq!(mesh.uvs, uv);
    assert_eq!(mesh.triangles, triangles);
}
