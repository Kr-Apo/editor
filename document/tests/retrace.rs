use ankhimate_core::{
    attachment::{Attachment, MeshAttachment, VertexWeight},
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::{Document, History, retrace::Plan};
use glam::Vec2;
fn fixture() -> (
    Document,
    ankhimate_core::ids::SkinId,
    ankhimate_core::ids::SlotId,
) {
    let mut d = Document::new();
    let b = d.skeleton.add_bone(Bone {
        name: "bone".into(),
        parent: None,
        length: 10.,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let slot = d.skeleton.add_slot(Slot {
        attachment: Some("mesh".into()),
        ..Slot::new("slot".into(), b)
    });
    let skin = d.skeleton.default_skin;
    let m = MeshAttachment {
        texture: "art".into(),
        setup_vertices: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        triangles: vec![[0, 1, 2]],
        weights: vec![
            vec![VertexWeight {
                bone: b,
                weight: 1.
            }];
            3
        ],
        ..Default::default()
    };
    d.skeleton.skins[skin].set(slot, "mesh", Attachment::Mesh(m));
    ankhimate_document::rebind::rebind_meshes(&mut d);
    (d, skin, slot)
}
#[test]
fn t823_retracing_a_weighted_mesh_must_not_clear_its_influences() {
    let (mut d, skin, slot) = fixture();
    let p = Plan::prepare(
        &d,
        skin,
        slot,
        "mesh",
        vec![Vec2::ZERO, Vec2::X * 2., Vec2::Y * 2.],
        vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        vec![[0, 1, 2]],
    )
    .unwrap();
    History::default().push(Box::new(p.into_command(&d).unwrap().unwrap()), &mut d);
    let Attachment::Mesh(m) = d.skeleton.skins[skin].get(slot, "mesh").unwrap() else {
        panic!()
    };
    assert!(
        !m.weights.is_empty(),
        "T823 confirmed: TraceMesh clears every weight row"
    );
}

#[path = "support/retrace_fixture.rs"]
mod support;
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    attachment::{FfdKeyframe, LinkedMesh, Sequence},
};
use ankhimate_document::snapshot::snapshot;
fn refuse(d: &Document, skin: ankhimate_core::ids::SkinId, slot: ankhimate_core::ids::SlotId) {
    let before = snapshot(d);
    assert!(support::changed(d, skin, slot).is_err());
    assert!(snapshot(d) == before);
}
#[test]
fn t823_nearest_rows_ties_and_bone_ids_are_deterministic() {
    let (d, sk, sl) = support::fixture(true);
    let a = support::changed(&d, sk, sl).unwrap();
    let b = support::changed(&d, sk, sl).unwrap();
    assert_eq!(a.summary.source_indices, vec![0, 1, 2, 3, 0]);
    assert_eq!(a.summary.source_indices, b.summary.source_indices);
    let old = support::mesh(&d, sk, sl);
    for (i, row) in a.result().weights.iter().enumerate() {
        assert_eq!(
            serde_json::to_value(row).unwrap(),
            serde_json::to_value(&old.weights[a.summary.source_indices[i]]).unwrap()
        );
        assert!((row.iter().map(|w| w.weight).sum::<f32>() - 1.).abs() < 0.0001);
    }
    assert_eq!(a.result().inverse_bind_matrices, old.inverse_bind_matrices);
    assert_eq!(a.summary.old_influences, 8);
    assert_eq!(a.summary.new_influences, 10);
}
#[test]
fn t823_exact_three_cycles_restore_all_fields_and_keep_prior_undo() {
    let (mut d, sk, sl) = support::fixture(true);
    let mut h = History::default();
    let bone = d.skeleton.slots[sl].bone;
    h.push(
        Box::new(ankhimate_document::commands::bone_cmds::RenameBone::new(
            bone,
            "authored root",
        )),
        &mut d,
    );
    let before = snapshot(&d);
    let p = support::changed(&d, sk, sl).unwrap();
    h.push(Box::new(p.into_command(&d).unwrap().unwrap()), &mut d);
    let after = snapshot(&d);
    assert_eq!(h.undo_depth(), 2);
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert!(snapshot(&d) == before);
        assert!(h.redo(&mut d));
        assert!(snapshot(&d) == after);
    }
}
#[test]
fn t823_rigid_mesh_retracing_preserves_historical_behavior() {
    let (mut d, sk, sl) = support::fixture(false);
    let p = support::changed(&d, sk, sl).unwrap();
    assert_eq!(p.summary.new_influences, 0);
    let old = snapshot(&d);
    let mut h = History::default();
    h.push(Box::new(p.into_command(&d).unwrap().unwrap()), &mut d);
    assert_eq!(support::mesh(&d, sk, sl).setup_vertices.len(), 5);
    assert!(support::mesh(&d, sk, sl).weights.is_empty());
    h.undo(&mut d);
    assert!(snapshot(&d) == old);
}
#[test]
fn t823_noop_cancel_and_failure_do_not_consume_redo() {
    let (mut d, sk, sl) = support::fixture(true);
    let mut h = History::default();
    let p = support::changed(&d, sk, sl).unwrap();
    h.push(Box::new(p.into_command(&d).unwrap().unwrap()), &mut d);
    h.undo(&mut d);
    let before = snapshot(&d);
    drop(support::changed(&d, sk, sl).unwrap());
    let m = support::mesh(&d, sk, sl);
    assert!(
        Plan::prepare(
            &d,
            sk,
            sl,
            "mesh",
            m.setup_vertices.clone(),
            m.uvs.clone(),
            m.triangles.clone()
        )
        .unwrap()
        .into_command(&d)
        .unwrap()
        .is_none()
    );
    assert!(Plan::prepare(&d, sk, sl, "mesh", vec![], vec![], vec![]).is_err());
    assert!(h.can_redo());
    assert!(snapshot(&d) == before);
    h.redo(&mut d);
}
#[test]
fn t823_stale_document_snapshot_cannot_materialize_command() {
    let (mut d, sk, sl) = support::fixture(true);
    let p = support::changed(&d, sk, sl).unwrap();
    d.meta.name = "newer".into();
    let before = snapshot(&d);
    assert!(p.into_command(&d).is_err());
    assert!(snapshot(&d) == before);
}
#[test]
fn t823_inline_ffd_refuses_rigid_and_weighted_even_if_empty_reset() {
    for weighted in [false, true] {
        let (mut d, sk, sl) = support::fixture(weighted);
        support::mesh_mut(&mut d, sk, sl)
            .ffd_keyframes
            .push(FfdKeyframe {
                time: 1.,
                vertex_offsets: vec![],
            });
        refuse(&d, sk, sl);
    }
}
#[test]
fn t823_three_deform_keys_and_malformed_offsets_are_never_deleted() {
    for weighted in [false, true] {
        for bad in [false, true] {
            let (mut d, sk, sl) = support::fixture(weighted);
            let count = if bad {
                7
            } else if weighted {
                8
            } else {
                4
            };
            let mut a = Animation::new("protected FFD", 3.);
            a.timelines.push(Timeline::Deform {
                slot: sl,
                attachment: "mesh".into(),
                keys: vec![
                    Key::linear(0., vec![]),
                    Key::linear(1., vec![Vec2::X; count]),
                    Key::stepped(3., vec![]),
                ],
            });
            d.animations.insert(a);
            refuse(&d, sk, sl);
        }
    }
}
#[test]
fn t823_linked_target_and_shared_source_are_refused() {
    for target in [false, true] {
        let (mut d, sk, sl) = support::fixture(true);
        let link = LinkedMesh {
            skin: None,
            slot: "slot".into(),
            attachment: "mesh".into(),
            inherit_deform: true,
        };
        if target {
            support::mesh_mut(&mut d, sk, sl).linked = Some(link);
        } else {
            d.skeleton.skins[sk].set(
                sl,
                "linked",
                Attachment::Mesh(MeshAttachment {
                    linked: Some(link),
                    ..Default::default()
                }),
            );
        }
        refuse(&d, sk, sl);
    }
}
#[test]
fn t823_ambiguous_skin_owner_and_sequence_are_refused() {
    let (mut d, sk, sl) = support::fixture(true);
    let other = d
        .skeleton
        .add_skin(ankhimate_core::skin::Skin::new("override"));
    let copy = support::mesh(&d, sk, sl).clone();
    d.skeleton.skins[other].set(sl, "mesh", Attachment::Mesh(copy));
    refuse(&d, sk, sl);
    let (mut d, sk, sl) = support::fixture(true);
    support::mesh_mut(&mut d, sk, sl).sequence = Some(Sequence {
        frames: vec!["art".into()],
        ..Default::default()
    });
    refuse(&d, sk, sl);
}
#[test]
fn t823_pinned_edges_are_not_left_pointing_at_new_indices() {
    let (mut d, sk, sl) = support::fixture(false);
    support::mesh_mut(&mut d, sk, sl).edges = vec![[0, 2]];
    refuse(&d, sk, sl);
}
#[test]
fn t823_invalid_weight_rows_bones_and_numbers_refuse_without_normalizing() {
    for case in 0..7 {
        let (mut d, sk, sl) = support::fixture(true);
        let m = support::mesh_mut(&mut d, sk, sl);
        match case {
            0 => m.weights[0][0].weight = -0.2,
            1 => m.weights[0][0].weight = f32::NAN,
            2 => m.weights[0][0].weight = f32::INFINITY,
            3 => m.weights[0][0].weight = 0.9,
            4 => m.weights[0][0].bone = Default::default(),
            5 => m.weights[0][1].bone = m.weights[0][0].bone,
            _ => {
                m.weights.pop();
            }
        }
        refuse(&d, sk, sl);
    }
}
#[test]
fn t823_bind_missing_singular_nonfinite_or_noncanonical_is_refused() {
    for case in 0..4 {
        let (mut d, sk, sl) = support::fixture(true);
        let m = support::mesh_mut(&mut d, sk, sl);
        let bone = m.weights[0][0].bone;
        match case {
            0 => {
                m.inverse_bind_matrices.remove(&bone);
            }
            1 => m.inverse_bind_matrices.get_mut(&bone).unwrap().a = 0.,
            2 => m.inverse_bind_matrices.get_mut(&bone).unwrap().tx = f32::NAN,
            _ => m.inverse_bind_matrices.get_mut(&bone).unwrap().tx += 1.,
        }
        refuse(&d, sk, sl);
    }
}
#[test]
fn t823_invalid_old_or_new_topology_is_refused() {
    let (d, sk, sl) = support::fixture(true);
    let m = support::mesh(&d, sk, sl);
    for triangles in [vec![], vec![[0, 1, 99]], vec![[0, 0, 1]]] {
        assert!(
            Plan::prepare(
                &d,
                sk,
                sl,
                "mesh",
                m.setup_vertices.clone(),
                m.uvs.clone(),
                triangles
            )
            .is_err()
        );
    }
    for bad in [false, true] {
        let (mut d, sk, sl) = support::fixture(true);
        let m = support::mesh_mut(&mut d, sk, sl);
        if bad {
            m.uvs.pop();
        } else {
            m.triangles[0][0] = 999;
        }
        refuse(&d, sk, sl);
    }
    assert!(
        Plan::prepare(
            &d,
            sk,
            sl,
            "mesh",
            vec![Vec2::ZERO, Vec2::X, Vec2::X * 2.],
            vec![Vec2::ZERO; 3],
            vec![[0, 1, 2]]
        )
        .is_err()
    );
}
#[test]
fn t823_non_affine_weighted_uv_frame_is_refused_instead_of_approximated() {
    let (mut d, sk, sl) = support::fixture(true);
    support::mesh_mut(&mut d, sk, sl).setup_vertices[3].x += 2.;
    refuse(&d, sk, sl);
}
#[test]
fn t823_whole_bone_influence_cannot_disappear_through_nearest_sampling() {
    let (mut d, sk, sl) = support::fixture(true);
    let m = support::mesh_mut(&mut d, sk, sl);
    let b = m.weights[0][0].bone;
    let c = m.weights[0][1].bone;
    // Give only the top-left source the first bone; all proposed points choose bottom-left.
    m.weights = vec![
        vec![VertexWeight {
            bone: c,
            weight: 1.
        }];
        4
    ];
    m.weights[0] = vec![VertexWeight {
        bone: b,
        weight: 1.,
    }];
    let before = snapshot(&d);
    assert!(
        Plan::prepare(
            &d,
            sk,
            sl,
            "mesh",
            vec![
                glam::vec2(-9., -9.),
                glam::vec2(-8., -9.),
                glam::vec2(-9., -8.)
            ],
            vec![Vec2::ZERO, Vec2::X, Vec2::Y],
            vec![[0, 1, 2]]
        )
        .is_err()
    );
    assert!(snapshot(&d) == before);
}
#[test]
fn t823_shared_texture_does_not_mutate_independent_mesh_or_other_ffd() {
    let (mut d, sk, sl) = support::fixture(true);
    let other_slot = d.skeleton.add_slot(Slot {
        attachment: Some("other".into()),
        ..Slot::new("other-slot".into(), d.skeleton.slots[sl].bone)
    });
    let mut other = support::mesh(&d, sk, sl).clone();
    other.ffd_keyframes = vec![FfdKeyframe {
        time: 1.,
        vertex_offsets: vec![Vec2::Y; 8],
    }];
    d.skeleton.skins[sk].set(other_slot, "other", Attachment::Mesh(other));
    let before = snapshot(&d);
    let mut expected = d.clone();
    let plan = support::changed(&d, sk, sl).unwrap();
    // Only the exact target attachment may differ after the actual native command.
    expected.skeleton.skins[sk].set(sl, "mesh", Attachment::Mesh(plan.result().clone()));
    History::default().push(Box::new(plan.into_command(&d).unwrap().unwrap()), &mut d);
    assert!(snapshot(&d) == snapshot(&expected));
    assert!(snapshot(&d) != before);
}
#[test]
fn t823_setup_mode_guard_does_not_consume_redo() {
    let (mut d, sk, sl) = support::fixture(true);
    let mut h = History::default();
    h.push(
        Box::new(
            support::changed(&d, sk, sl)
                .unwrap()
                .into_command(&d)
                .unwrap()
                .unwrap(),
        ),
        &mut d,
    );
    h.undo(&mut d);
    let before = snapshot(&d);
    let c = support::changed(&d, sk, sl)
        .unwrap()
        .into_command(&d)
        .unwrap()
        .unwrap();
    assert!(!h.push_in_mode(Box::new(c), &mut d, ankhimate_document::WorkMode::Animate));
    assert!(h.can_redo());
    assert!(snapshot(&d) == before);
}
#[test]
fn t823_real_trace_hole_and_transparency_use_existing_generator() {
    let (d, sk, sl) = support::fixture(true);
    assert!(
        ankhimate_document::meshgen::trace(&image::RgbaImage::new(64, 64), Default::default())
            .is_none()
    );
    let mut img = image::RgbaImage::from_pixel(64, 64, image::Rgba([200, 100, 20, 255]));
    for y in 20..44 {
        for x in 20..44 {
            img.get_pixel_mut(x, y).0[3] = 0;
        }
    }
    let t = ankhimate_document::meshgen::trace(&img, Default::default()).unwrap();
    assert!(t.contours.len() >= 2);
    let p = Plan::from_trace(&d, sk, sl, "mesh", &t).unwrap();
    assert!(!p.result().triangles.is_empty());
    assert_eq!(p.result().uvs.len(), p.result().setup_vertices.len());
}

#[test]
fn t823_excess_influences_are_refused_not_pruned() {
    let (mut d, sk, sl) = support::fixture(true);
    let mut bones: Vec<_> = d.skeleton.bones.keys().collect();
    for i in 0..3 {
        bones.push(d.skeleton.add_bone(Bone {
            name: format!("extra{i}"),
            parent: None,
            length: 1.,
            local_transform: Default::default(),
            inherit: Default::default(),
            color: Bone::default_color(),
        }));
    }
    let m = support::mesh_mut(&mut d, sk, sl);
    m.weights = vec![
        bones
            .iter()
            .map(|bone| VertexWeight {
                bone: *bone,
                weight: 0.2
            })
            .collect();
        4
    ];
    m.inverse_bind_matrices.clear();
    ankhimate_document::rebind::rebind_meshes(&mut d);
    refuse(&d, sk, sl);
}
#[test]
fn t823_comparison_budget_is_checked_before_quadratic_search() {
    let (mut d, sk, sl) = support::fixture(true);
    let m = support::mesh_mut(&mut d, sk, sl);
    m.setup_vertices.resize(4097, Vec2::ZERO);
    m.uvs.resize(4097, Vec2::splat(0.5));
    m.weights.resize(4097, m.weights[0].clone());
    let (v, u, t) = (m.setup_vertices.clone(), m.uvs.clone(), m.triangles.clone());
    let before = snapshot(&d);
    let p = Plan::prepare(&d, sk, sl, "mesh", v, u, t);
    assert!(p.err().unwrap().contains("budget"));
    assert!(snapshot(&d) == before);
}
fn approx(a: &serde_json::Value, b: &serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Number(x), serde_json::Value::Number(y)) => {
            assert!((x.as_f64().unwrap() - y.as_f64().unwrap()).abs() < 0.0001)
        }
        (serde_json::Value::Array(x), serde_json::Value::Array(y)) => {
            assert_eq!(x.len(), y.len());
            for (a, b) in x.iter().zip(y) {
                approx(a, b);
            }
        }
        (serde_json::Value::Object(x), serde_json::Value::Object(y)) => {
            assert_eq!(x.len(), y.len());
            for (k, v) in x {
                approx(v, &y[k]);
            }
        }
        _ => assert_eq!(a, b),
    }
}
#[test]
fn t823_save_reload_rebuilds_native_binds_and_preserves_weights_and_animation() {
    let (mut d, sk, sl) = support::fixture(true);
    let p = support::changed(&d, sk, sl).unwrap();
    History::default().push(Box::new(p.into_command(&d).unwrap().unwrap()), &mut d);
    let path =
        std::env::temp_dir().join(format!("ankhimate-t823-save-{}.ankh", std::process::id()));
    assert!(!path.exists());
    ankhimate_formats::save(&path, &d.as_project_ref(), &[]).unwrap();
    let p = ankhimate_formats::load(&path).unwrap().0;
    assert!(p.report.dangling.is_empty() && p.report.lossy.is_empty());
    let mut loaded = Document::from_loaded(p);
    ankhimate_document::rebind::rebind_meshes(&mut loaded);
    approx(
        &serde_json::to_value(ankhimate_formats::convert::to_schema(&d.as_project_ref())).unwrap(),
        &serde_json::to_value(ankhimate_formats::convert::to_schema(
            &loaded.as_project_ref(),
        ))
        .unwrap(),
    );
    let rsk = loaded.skeleton.default_skin;
    let rsl = loaded.skeleton.draw_order[0];
    assert_eq!(
        serde_json::to_value(&support::mesh(&d, sk, sl).weights).unwrap(),
        serde_json::to_value(&support::mesh(&loaded, rsk, rsl).weights).unwrap()
    );
    for (bone, bind) in &support::mesh(&d, sk, sl).inverse_bind_matrices {
        let other = &support::mesh(&loaded, rsk, rsl).inverse_bind_matrices[bone];
        for (x, y) in [bind.a, bind.b, bind.c, bind.d, bind.tx, bind.ty]
            .into_iter()
            .zip([other.a, other.b, other.c, other.d, other.tx, other.ty])
        {
            assert!((x - y).abs() < 0.0001);
        }
    }
    ankhimate_document::retrace::source(&loaded, rsk, rsl, "mesh").unwrap();
    let schema = ankhimate_formats::convert::to_schema(&loaded.as_project_ref());
    let export = ankhimate_export::run::plan(
        &schema,
        &loaded.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    assert!(!export.files.is_empty());
    std::fs::remove_file(&path).unwrap();
    std::fs::remove_dir_all(ankhimate_formats::container::asset_root(&path)).unwrap();
}
#[test]
fn t823_performance_report_512_release_and_disposable_native_fixtures() {
    use std::time::Instant;
    let (mut d, sk, sl) = support::fixture(true);
    let base = support::mesh(&d, sk, sl).weights[0].clone();
    let m = support::mesh_mut(&mut d, sk, sl);
    m.setup_vertices.clear();
    m.uvs.clear();
    m.weights.clear();
    for y in 0..32 {
        for x in 0..32 {
            let uv = glam::vec2(x as f32 / 31., y as f32 / 31.);
            m.uvs.push(uv);
            m.setup_vertices
                .push(glam::vec2((uv.x - 0.5) * 512., (0.5 - uv.y) * 512.));
            m.weights.push(base.clone());
        }
    }
    ankhimate_document::meshgen::retriangulate(m);
    let mut image = image::RgbaImage::new(512, 512);
    for (y, row) in image.rows_mut().enumerate() {
        for (x, p) in row.enumerate() {
            let r = (x as f32 - 256.).hypot(y as f32 - 256.);
            if (90. ..210.).contains(&r) {
                *p = image::Rgba([220, 160, 40, 255]);
            }
        }
    }
    let mut encoded = std::io::Cursor::new(vec![]);
    image::DynamicImage::ImageRgba8(image.clone())
        .write_to(&mut encoded, image::ImageFormat::Png)
        .unwrap();
    let asset = d.assets.by_name("art").unwrap();
    d.assets.images[asset].bytes = encoded.into_inner();
    d.assets.images[asset].width = 512;
    d.assets.images[asset].height = 512;
    let mut tracing = vec![];
    let mut remapping = vec![];
    let mut outlines = vec![];
    let mut geometry = vec![];
    let mut count = 0;
    for i in 0..11 {
        let start = Instant::now();
        let traced = ankhimate_document::meshgen::trace(&image, Default::default()).unwrap();
        let ms = start.elapsed().as_secs_f64() * 1000.;
        let geometry_start = Instant::now();
        let (v, u, t) = ankhimate_document::meshgen::mesh_from_trace_on_mesh(
            &traced,
            support::mesh(&d, sk, sl),
        );
        let geometry_ms = geometry_start.elapsed().as_secs_f64() * 1000.;
        count = v.len();
        let start = Instant::now();
        let plan = Plan::prepare(&d, sk, sl, "mesh", v, u, t).unwrap();
        let extra = start.elapsed().as_secs_f64() * 1000.;
        assert_eq!(plan.result().weights.len(), count);
        if i > 0 {
            tracing.push(ms + geometry_ms);
            outlines.push(ms);
            geometry.push(geometry_ms);
            remapping.push(extra);
        }
    }
    outlines.sort_by(f64::total_cmp);
    geometry.sort_by(f64::total_cmp);
    tracing.sort_by(f64::total_cmp);
    remapping.sort_by(f64::total_cmp);
    let result = serde_json::json!({"release":!cfg!(debug_assertions),"image":[512,512],"sourceVertices":1024,"newVertices":count,"iterations":10,"outlineMedianMs":outlines[5],"geometryMedianMs":geometry[5],"traceMedianMs":tracing[5],"traceMaxMs":tracing[9],"safePrepareRemapMedianMs":remapping[5],"safePrepareRemapMaxMs":remapping[9],"historicalTraceBudgetMs":100,"CIHasTimingAssertion":false});
    println!("T823_PERFORMANCE={result}");
    if let Ok(dest) = std::env::var("ANKHIMATE_T823_ARTIFACTS") {
        let root = std::path::PathBuf::from(dest);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("PERFORMANCE.json"),
            serde_json::to_vec_pretty(&result).unwrap(),
        )
        .unwrap();
        let fixtures = root.join("Fixtures");
        std::fs::create_dir_all(&fixtures).unwrap();
        ankhimate_formats::save(&fixtures.join("Original.ankh"), &d.as_project_ref(), &[]).unwrap();
        let traced = ankhimate_document::meshgen::trace(&image, Default::default()).unwrap();
        let c = Plan::from_trace(&d, sk, sl, "mesh", &traced)
            .unwrap()
            .into_command(&d)
            .unwrap()
            .unwrap();
        History::default().push(Box::new(c), &mut d);
        ankhimate_formats::save(&fixtures.join("Retraced.ankh"), &d.as_project_ref(), &[]).unwrap();
    }
}

#[test]
fn t823_unpinned_link_in_nondefault_skin_protects_its_real_source() {
    let (mut d, sk, sl) = support::fixture(true);
    let attachment = d.skeleton.skins[sk]
        .entries
        .remove(&(sl, "mesh".into()))
        .unwrap();
    let royal = d
        .skeleton
        .add_skin(ankhimate_core::skin::Skin::new("royal"));
    d.skeleton.skins[royal].set(sl, "mesh", attachment);
    d.skeleton.skins[royal].set(
        sl,
        "linked",
        Attachment::Mesh(MeshAttachment {
            linked: Some(LinkedMesh {
                skin: None,
                slot: "slot".into(),
                attachment: "mesh".into(),
                inherit_deform: true,
            }),
            ..Default::default()
        }),
    );
    refuse(&d, royal, sl);
}

#[test]
fn t823_setup_world_positions_and_retained_vertices_animation_are_coherent() {
    let (d, sk, sl) = support::fixture(true);
    let old = support::mesh(&d, sk, sl);
    let mut vertices = old.setup_vertices.clone();
    let mut uvs = old.uvs.clone();
    vertices.push(Vec2::ZERO);
    uvs.push(Vec2::splat(0.5));
    let plan = Plan::prepare(
        &d,
        sk,
        sl,
        "mesh",
        vertices,
        uvs,
        vec![[0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4]],
    )
    .unwrap();
    let mesh = plan.result();
    let mut pose = ankhimate_core::pose::Pose::new();
    ankhimate_core::pose::evaluate(&d.skeleton, &[], &mut pose);
    let space = pose.world(d.skeleton.slots[sl].bone);
    let mut max: f32 = 0.;
    for (i, p) in mesh.setup_vertices.iter().enumerate() {
        let actual = mesh.skin_vertex_with_ffd(i, Vec2::ZERO, &pose, &space);
        max = max.max(actual.distance(space.transform_point(*p)));
    }
    assert!(max < 0.01);
    let a = d.animations.values().next().unwrap();
    for time in [0., 0.25, 0.5, 0.75, 1., 3.] {
        ankhimate_core::pose::evaluate(&d.skeleton, &[(a, time, 1.)], &mut pose);
        let space = pose.world(d.skeleton.slots[sl].bone);
        for i in 0..4 {
            assert!(
                mesh.skin_vertex_with_ffd(i, Vec2::ZERO, &pose, &space)
                    .distance(old.skin_vertex_with_ffd(i, Vec2::ZERO, &pose, &space))
                    < 0.01
            );
        }
    }
    println!("T823_SETUP_MAX_ERROR={max}");
}

#[test]
fn t823_near_canonical_bind_is_refused_without_a_reload_drift_window() {
    let (mut d, sk, sl) = support::fixture(true);
    let m = support::mesh_mut(&mut d, sk, sl);
    let bone = m.weights[0][0].bone;
    m.inverse_bind_matrices.get_mut(&bone).unwrap().a += 0.00001;
    refuse(&d, sk, sl);
}
