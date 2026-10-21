use super::*;
use ankhimate_core::animation::{Animation, Key, Timeline};
#[test]
fn t824_zero_fps_has_no_invented_sample_rate() {
    assert!(frame_onion_times(1., 2., 0, 5).is_empty());
}
#[test]
fn t824_rounded_frame_times_never_repeat_current_pose() {
    let now = 16_777_216.;
    let times = frame_onion_times(now, now + 10., 240, 5);
    assert!(times.iter().all(|(t, _, _)| *t != now));
}
#[test]
fn t824_key_samples_reject_negative_and_beyond_duration() {
    let mut a = Animation::new("bounds", 1.);
    a.timelines.push(Timeline::DrawOrder {
        keys: vec![
            Key::linear(-1., vec![]),
            Key::linear(0., vec![]),
            Key::linear(1., vec![]),
            Key::linear(2., vec![]),
        ],
    });
    let times = key_onion_times(0.5, &a, 5);
    assert_eq!(times.iter().map(|s| s.0).collect::<Vec<_>>(), vec![0., 1.]);
}

use crate::{session::WorkMode, theme::Theme};
use ankhimate_core::{
    attachment::{Attachment, ClippingAttachment},
    skin::Skin,
    slot::Slot,
};
use ankhimate_document::snapshot::snapshot;
#[allow(
    dead_code,
    reason = "Reuse the accepted native weighted fixture without duplicating mesh construction"
)]
mod support {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
fn fixture() -> AppState {
    let (doc, _, _) = support::fixture(true);
    let mut s = AppState::default();
    s.replace_document(doc);
    s.session.active_animation = s.doc.animations.keys().next();
    s.set_work_mode(WorkMode::Animate);
    s.session.playhead = 0.5;
    s.session.onion_skin.enabled = true;
    s.session.onion_skin.frames = 5;
    prepare_textures(&mut s);
    s
}
fn counted(s: &AppState) -> (Vec<(Pose, [f32; 4])>, usize) {
    let mut count = 0;
    let poses = onion_poses_counted(s, &Theme::default(), || count += 1);
    (poses, count)
}
#[test]
fn t824_invalid_frame_clocks_are_empty() {
    for (now, duration) in [
        (f32::NAN, 1.),
        (0., f32::NAN),
        (f32::INFINITY, 1.),
        (0., f32::INFINITY),
        (-0.1, 1.),
        (2., 1.),
        (0., 0.),
        (0., -1.),
    ] {
        assert!(frame_onion_times(now, duration, 60, 5).is_empty());
    }
}
#[test]
fn t824_frames_bounds_fps_and_step_cap() {
    for fps in [1, 24, 30, 60, 240, u32::MAX] {
        for now in [0., 0.5, 1.] {
            let samples = frame_onion_times(now, 1., fps, 255);
            assert!(samples.len() <= 10);
            for (i, (time, side, step)) in samples.iter().enumerate() {
                assert!(*time >= 0. && *time <= 1. && *time != now && *step <= 5);
                assert_eq!(
                    *side,
                    if *time < now {
                        OnionSide::Past
                    } else {
                        OnionSide::Future
                    }
                );
                assert!(samples[..i].iter().all(|x| x.0 != *time));
            }
        }
    }
    assert_eq!(frame_onion_times(0., 1., 60, 1).len(), 1);
    assert_eq!(frame_onion_times(1., 1., 60, 5).len(), 5);
    assert_eq!(frame_onion_times(0.5, 1., 60, 0).len(), 0);
}
#[test]
fn t824_short_clip_omits_unavailable_steps_without_clamping() {
    assert!(frame_onion_times(0., 0.001, 60, 5).is_empty());
    assert_eq!(
        frame_onion_times(0., 0.1, 10, 5),
        vec![(0.1, OnionSide::Future, 1)]
    );
}
#[test]
fn t824_frames_keep_relative_timing_without_global_frame_quantization() {
    let times = frame_onion_times(0.503, 1., 25, 2);
    assert_eq!(times.len(), 4);
    for (time, side, step) in times {
        let delta = step as f32 / 25.;
        assert!(
            (time
                - (0.503
                    + if side == OnionSide::Past {
                        -delta
                    } else {
                        delta
                    }))
            .abs()
                < 1e-6
        );
    }
}
fn key_clip(times: &[f32]) -> Animation {
    let mut a = Animation::new("keys", 1.);
    a.timelines.push(Timeline::DrawOrder {
        keys: times.iter().map(|t| Key::linear(*t, vec![])).collect(),
    });
    a
}
#[test]
fn t824_keys_irregular_distinct_nearest_order_and_duplicate_tolerance() {
    let mut a = key_clip(&[1., 0.1, 0.2, 0.2, 0.200001, 0.8, 0.4]);
    a.timelines.push(a.timelines[0].clone());
    assert_eq!(
        key_onion_times(0.5, &a, 2),
        vec![
            (0.4, OnionSide::Past, 1),
            (0.2, OnionSide::Past, 2),
            (0.8, OnionSide::Future, 1),
            (1., OnionSide::Future, 2)
        ]
    );
}
#[test]
fn t824_single_key_and_no_previous_next() {
    let a = key_clip(&[0.5]);
    assert!(key_onion_times(0.5, &a, 5).is_empty());
    assert_eq!(
        key_onion_times(0., &a, 5),
        vec![(0.5, OnionSide::Future, 1)]
    );
    assert_eq!(key_onion_times(1., &a, 5), vec![(0.5, OnionSide::Past, 1)]);
}
#[test]
fn t824_invalid_keys_clocks_and_empty_clip_are_safe() {
    let a = key_clip(&[f32::NAN, f32::INFINITY, -1., 2.]);
    assert!(key_onion_times(0.5, &a, 5).is_empty());
    for d in [0., -1., f32::NAN, f32::INFINITY] {
        let mut a = a.clone();
        a.duration = d;
        assert!(key_onion_times(0., &a, 5).is_empty());
    }
    assert!(key_onion_times(f32::NAN, &a, 5).is_empty());
    assert!(key_onion_times(0., &Animation::new("empty", 1.), 5).is_empty());
}
#[test]
fn t824_deform_keys_included_events_and_markers_excluded_as_before() {
    let s = fixture();
    let slot = s.doc.skeleton.draw_order[0];
    let mut a = Animation::new("families", 1.);
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![Key::linear(0.25, vec![glam::Vec2::ZERO; 8])],
    });
    a.events.push(ankhimate_core::animation::EventKey {
        time: 0.75,
        name: "event".into(),
        int_value: 0,
        float_value: 0.,
        string_value: String::new(),
        audio: String::new(),
        volume: 1.,
        balance: 0.,
    });
    a.markers
        .push(ankhimate_core::animation::Marker::new(0.9, "marker"));
    assert_eq!(
        key_onion_times(0.5, &a, 5),
        vec![(0.25, OnionSide::Past, 1)]
    );
}
#[test]
fn t824_opacity_is_finite_bounded_for_all_inputs() {
    let mut settings = crate::session::OnionSkinSettings::default();
    for opacity in [-100., 0., 0.25, 1., 100., f32::NAN, f32::INFINITY] {
        for falloff in [-10., 0., 0.5, 2., 99., f32::NAN, f32::INFINITY] {
            settings.opacity = opacity;
            settings.falloff = falloff;
            for step in [0, 1, 2, 5, 255] {
                let a = settings.alpha(step);
                assert!(a.is_finite() && (0. ..=1.).contains(&a));
            }
        }
    }
}
#[test]
fn t824_falloff_defaults_and_monotonic_distance() {
    let mut settings = crate::session::OnionSkinSettings::default();
    assert_eq!(settings.alpha(1), 0.32);
    assert!((settings.alpha(4) - 0.16).abs() < 1e-7);
    for falloff in [0., 0.5, 1., 2.] {
        settings.falloff = falloff;
        for step in 2..=5 {
            assert!(settings.alpha(step) <= settings.alpha(step - 1));
        }
    }
    settings.falloff = 0.;
    assert_eq!(settings.alpha(5), settings.alpha(1));
}
#[test]
fn t824_theme_past_future_and_custom_light_palette_are_applied() {
    let s = fixture();
    for theme in Theme::load_all().into_iter().chain([Theme {
        panel_fill: "#ffffff".into(),
        channel_shear: "#cc2244".into(),
        channel_rotate: "#117733".into(),
        ..Theme::default()
    }]) {
        let p = onion_poses(&s, &theme);
        assert_eq!(p.len(), 10);
        for (i, channel) in [(0, "shear"), (1, "rotate")] {
            let rgb = theme.channel_color(channel).to_srgba_unmultiplied();
            assert_eq!(
                &p[i].1[..3],
                &rgb[..3]
                    .iter()
                    .map(|n| *n as f32 / 255.)
                    .collect::<Vec<_>>()
            );
        }
        assert_ne!(p[0].1, p[1].1);
    }
}
#[test]
fn t824_disabled_setup_hidden_and_zero_opacity_evaluate_zero_ghosts() {
    for case in 0..5 {
        let mut s = fixture();
        match case {
            0 => s.session.onion_skin.enabled = false,
            1 => s.session.work_mode = WorkMode::Setup,
            2 => s.session.show_artwork = false,
            3 => s.session.onion_skin.opacity = 0.,
            _ => s.session.active_animation = None,
        };
        let (p, c) = counted(&s);
        assert_eq!(c, 0);
        assert!(p.is_empty());
    }
}
#[test]
fn t824_real_evaluation_counts_follow_valid_samples_and_appearance_changes() {
    let mut s = fixture();
    assert_eq!(counted(&s).1, 10);
    s.session.onion_skin.opacity = 0.6;
    assert_eq!(counted(&s).1, 10);
    let theme = Theme {
        channel_shear: "#123456".into(),
        ..Theme::default()
    };
    let mut n = 0;
    onion_poses_counted(&s, &theme, || n += 1);
    assert_eq!(n, 10);
    s.session.onion_skin.frames = 2;
    assert_eq!(counted(&s).1, 4);
    s.session.playhead = 0.;
    assert_eq!(counted(&s).1, 2);
    s.session.playhead = 3.;
    assert_eq!(counted(&s).1, 2);
    s.session.onion_skin.key_steps = true;
    assert_eq!(counted(&s).1, 2);
}
#[test]
fn t824_selected_empty_stale_single_multi_preserve_full_rig_evaluation() {
    let mut s = fixture();
    s.session.onion_skin.selected_only = true;
    assert_eq!(counted(&s).1, 0);
    s.session.selected_bones = vec![Default::default()];
    assert_eq!(counted(&s).1, 0);
    let bones = s.doc.skeleton.update_order.clone();
    s.session.select_bone(Some(bones[0]));
    let (a, n) = counted(&s);
    assert_eq!(n, 10);
    assert!(a.iter().all(|p| p.0.worlds.contains_key(bones[1])));
    s.session.selected_bones = bones;
    assert_eq!(counted(&s).1, 10);
}
#[test]
fn t824_selected_only_is_slot_owner_filter_not_influence_filter() {
    let mut s = fixture();
    let slot = s.doc.skeleton.draw_order[0];
    let owner = s.doc.skeleton.slots[slot].bone;
    let other = s.doc.skeleton.update_order[1];
    s.session.onion_skin.selected_only = true;
    for (bone, expected) in [(owner, 1), (other, 0)] {
        s.session.select_bone(Some(bone));
        let p = onion_poses(&s, &Theme::default());
        let mut draws = vec![];
        append_pose_sprites(&s, &p[0].0, Some(p[0].1), true, &mut draws);
        assert_eq!(draws.len(), expected);
    }
}
#[test]
fn t824_ik_dependencies_remain_in_ghost_worlds() {
    use ankhimate_core::constraints::{Constraint, IkConstraint};
    let mut s = fixture();
    let bones = s.doc.skeleton.update_order.clone();
    let mut target = s.doc.skeleton.bones[bones[0]].clone();
    target.name = "target".into();
    target.local_transform.position = glam::vec2(40., 30.);
    let target = s.doc.skeleton.add_bone(target);
    s.doc
        .skeleton
        .add_constraint(Constraint::Ik(IkConstraint::two_bone(
            "ik",
            target,
            [bones[0], bones[1]],
        )));
    s.session.onion_skin.selected_only = true;
    s.session.select_bone(Some(bones[0]));
    let a = s
        .doc
        .animations
        .get(s.session.active_animation.unwrap())
        .unwrap();
    let mut expected = Pose::new();
    ankhimate_core::pose::evaluate(
        &s.doc.skeleton,
        &[(a, 0.5 - 1. / s.doc.meta.fps as f32, 1.)],
        &mut expected,
    );
    let actual = onion_poses(&s, &Theme::default());
    for b in [bones[0], bones[1], target] {
        assert_eq!(actual[0].0.worlds[b], expected.worlds[b]);
    }
}
#[test]
fn t824_clip_document_and_selection_changes_cannot_reuse_stale_poses() {
    let mut s = fixture();
    let a = s.doc.animations.insert(Animation::new("short", 0.01));
    s.session.active_animation = Some(a);
    s.session.playhead = 0.;
    assert_eq!(counted(&s).1, 0);
    s.session.onion_skin.key_steps = true;
    assert_eq!(counted(&s).1, 0);
    s.replace_document(ankhimate_document::Document::new());
    assert_eq!(counted(&s).1, 0);
}
fn draw_bytes(draws: &[SpriteDraw]) -> Vec<u8> {
    draws
        .iter()
        .flat_map(|d| {
            bytemuck::cast_slice::<_, u8>(&d.vertices)
                .iter()
                .copied()
                .chain(bytemuck::cast_slice::<_, u8>(&d.indices).iter().copied())
        })
        .collect()
}
#[test]
fn t824_current_pose_submission_and_authored_data_are_bit_identical() {
    let mut s = fixture();
    s.refresh_pose();
    let before = snapshot(&s.doc);
    let pose = format!("{:?}", s.pose);
    let mut normal = vec![];
    append_pose_sprites(&s, &s.pose, None, false, &mut normal);
    for opacity in [0., 0.2, 1.] {
        s.session.onion_skin.opacity = opacity;
        let _ = onion_poses(&s, &Theme::default());
        let mut after = vec![];
        append_pose_sprites(&s, &s.pose, None, false, &mut after);
        assert_eq!(draw_bytes(&normal), draw_bytes(&after));
    }
    assert!(snapshot(&s.doc) == before);
    assert_eq!(format!("{:?}", s.pose), pose);
    assert_eq!(s.history.undo_depth(), 0);
}
#[test]
fn t824_weighted_ghost_geometry_matches_untinted_pose_geometry() {
    let s = fixture();
    for (pose, tint) in onion_poses(&s, &Theme::default()) {
        let slot = s.doc.skeleton.draw_order[0];
        let a = sprite_for_slot(&s, &pose, slot, None).unwrap();
        let b = sprite_for_slot(&s, &pose, slot, Some(tint)).unwrap();
        assert_eq!(a.indices, b.indices);
        for (a, b) in a.vertices.iter().zip(&b.vertices) {
            assert_eq!(a.position, b.position);
            assert_eq!(a.uv, b.uv);
            assert_eq!(a.dark, b.dark);
            assert_eq!(b.color[3], a.color[3] * tint[3]);
        }
    }
}
#[test]
fn t824_skin_and_region_resolution_is_preserved() {
    let mut s = fixture();
    let slot = s.doc.skeleton.draw_order[0];
    let mut alternate = Skin::new("alternate");
    let mut mesh = support::mesh(&s.doc, s.doc.skeleton.default_skin, slot).clone();
    mesh.setup_vertices[0].x += 10.;
    alternate.set(slot, "mesh", Attachment::Mesh(mesh));
    let skin = s.doc.skeleton.add_skin(alternate);
    let p = onion_poses(&s, &Theme::default());
    let first = sprite_for_slot(&s, &p[0].0, slot, None).unwrap();
    s.session.active_skin = skin;
    let second = sprite_for_slot(&s, &p[0].0, slot, None).unwrap();
    assert_ne!(first.vertices[0].position, second.vertices[0].position);
    let region = ankhimate_core::attachment::RegionAttachment {
        texture: "art".into(),
        local_offset: glam::Vec2::ZERO,
        local_rotation: 0.,
        local_scale: glam::Vec2::ONE,
        width: 20.,
        height: 20.,
        uv_rect: ankhimate_core::attachment::Rect {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.,
        },
        pivot: glam::Vec2::splat(0.5),
        sequence: None,
    };
    s.doc.skeleton.skins[skin].set(slot, "mesh", Attachment::Region(region));
    assert_eq!(
        sprite_for_slot(&s, &p[0].0, slot, Some(p[0].1))
            .unwrap()
            .vertices
            .len(),
        4
    );
}
#[test]
fn t824_clipping_respects_unselected_clip_and_end_slot() {
    let mut s = fixture();
    let slot = s.doc.skeleton.draw_order[0];
    let bone = s.doc.skeleton.slots[slot].bone;
    let other = s.doc.skeleton.update_order[1];
    let clip = s.doc.skeleton.add_slot(Slot {
        attachment: Some("mask".into()),
        ..Slot::new("clip".into(), other)
    });
    let skin = s.doc.skeleton.default_skin;
    s.doc.skeleton.skins[skin].set(
        clip,
        "mask",
        Attachment::Clipping(ClippingAttachment {
            vertices: vec![
                glam::vec2(-100., -100.),
                glam::vec2(100., -100.),
                glam::vec2(100., 100.),
                glam::vec2(-100., 100.),
            ],
            end_slot: Some("slot".into()),
        }),
    );
    s.doc.skeleton.draw_order = vec![clip, slot];
    s.session.select_bone(Some(bone));
    let p = onion_poses(&s, &Theme::default());
    let mut draws = vec![];
    append_pose_sprites(&s, &p[0].0, Some(p[0].1), true, &mut draws);
    assert_eq!(draws.len(), 1);
    assert!(!draws[0].indices.is_empty());
}
#[test]
fn t824_view_changes_preserve_redo_and_three_cycles() {
    use ankhimate_document::commands::bone_cmds::SetBoneTransform;
    let mut s = fixture();
    s.set_work_mode(WorkMode::Setup);
    let b = s.doc.skeleton.update_order[0];
    let before = snapshot(&s.doc);
    let mut local = s.doc.skeleton.bones[b].local_transform;
    local.position.x += 10.;
    s.dispatch(Box::new(SetBoneTransform::new(b, local)));
    let after = snapshot(&s.doc);
    s.undo();
    for _ in 0..3 {
        s.set_work_mode(WorkMode::Animate);
        let _ = counted(&s);
        assert!(s.history.can_redo());
        assert!(snapshot(&s.doc) == before);
        s.redo();
        let _ = counted(&s);
        assert!(snapshot(&s.doc) == after);
        s.undo();
    }
}
#[test]
fn t824_save_reload_and_native_export_unchanged_by_view_settings() {
    let mut s = fixture();
    let schema = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let preset = ankhimate_export::presets::default_preset();
    let a = ankhimate_export::run::plan(&schema, &s.doc.assets, &preset).unwrap();
    let before = snapshot(&s.doc);
    let dirty = s.history.undo_depth();
    let revision = s.revision;
    s.session.onion_skin.opacity = 0.7;
    s.session.onion_skin.falloff = 1.2;
    s.session.onion_skin.key_steps = true;
    let _ = counted(&s);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), dirty);
    assert_eq!(s.revision, revision);
    let schema2 = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    assert_eq!(
        serde_json::to_value(&schema).unwrap(),
        serde_json::to_value(&schema2).unwrap()
    );
    let b = ankhimate_export::run::plan(&schema2, &s.doc.assets, &preset).unwrap();
    assert_eq!(a.binaries, b.binaries);
    assert_eq!(format!("{:?}", a.files), format!("{:?}", b.files));
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("view.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let loaded = ankhimate_formats::load(&path).unwrap().0;
    assert!(loaded.report.dangling.is_empty() && loaded.report.lossy.is_empty());
    let mut reopened = AppState::default();
    reopened.replace_document(ankhimate_document::Document::from_loaded(loaded));
    assert!(!reopened.session.onion_skin.enabled);
    assert_eq!(reopened.session.onion_skin.opacity, 0.32);
    assert_eq!(reopened.doc.animations.len(), s.doc.animations.len());
    assert_eq!(reopened.doc.assets.images.len(), s.doc.assets.images.len());
}
#[test]
fn t824_cpu_measurement_is_observational_not_a_budget_assertion() {
    let Some(path) = std::env::var_os("T824_PERF_PATH") else {
        return;
    };
    let mut s = fixture();
    let root = s.doc.skeleton.update_order[0];
    let base = s.doc.skeleton.bones[root].clone();
    for i in 2..500 {
        let mut b = base.clone();
        b.name = format!("bone-{i}");
        b.parent = Some(root);
        s.doc.skeleton.add_bone(b);
    }
    let mut rows = vec![];
    for keys in [false, true] {
        s.session.onion_skin.key_steps = keys;
        if keys {
            let a = s
                .doc
                .animations
                .get_mut(s.session.active_animation.unwrap())
                .unwrap();
            a.timelines = vec![Timeline::BoneRotate {
                bone: root,
                keys: (0..=30)
                    .map(|i| Key::linear(i as f32 / 10., i as f32))
                    .collect(),
            }];
        }
        let _ = counted(&s);
        let mut ms = vec![];
        let mut count = 0;
        for _ in 0..20 {
            let start = std::time::Instant::now();
            let (p, c) = counted(std::hint::black_box(&s));
            std::hint::black_box(p);
            count = c;
            ms.push(start.elapsed().as_secs_f64() * 1000.);
        }
        ms.sort_by(f64::total_cmp);
        rows.push(serde_json::json!({"bones":500,"ghosts":count,"mode":if keys {"keys"} else {"frames"},"medianMs":ms[10],"maxMs":ms[19],"samples":ms,"runs":20,"warmups":1,"build":if cfg!(debug_assertions){"debug"}else{"release"},"scope":"planning + pose evaluation + tint + allocation/drop; no GPU or sprite tessellation"}));
    }
    std::fs::write(path, serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}

#[test]
fn t824_draw_order_attachments_and_visibility_follow_each_evaluated_pose() {
    let mut s = fixture();
    let first = s.doc.skeleton.draw_order[0];
    let bone = s.doc.skeleton.slots[first].bone;
    let second = s.doc.skeleton.add_slot(Slot {
        attachment: Some("other".into()),
        ..Slot::new("second".into(), bone)
    });
    let sk = s.doc.skeleton.default_skin;
    let mut m = support::mesh(&s.doc, sk, first).clone();
    m.setup_vertices[0].x += 12.;
    s.doc.skeleton.skins[sk].set(second, "other", Attachment::Mesh(m));
    let id = s.session.active_animation.unwrap();
    let a = s.doc.animations.get_mut(id).unwrap();
    a.timelines.push(Timeline::DrawOrder {
        keys: vec![Key::linear(0., vec![(second, -1)])],
    });
    a.timelines.push(Timeline::SlotVisible {
        slot: first,
        keys: vec![Key::linear(0., false)],
    });
    let poses = onion_poses(&s, &Theme::default());
    for (p, tint) in poses {
        assert_eq!(p.draw_order[0], second);
        let mut draws = vec![];
        append_pose_sprites(&s, &p, Some(tint), false, &mut draws);
        assert_eq!(draws.len(), 1);
        let expected = sprite_for_slot(&s, &p, second, Some(tint)).unwrap();
        assert_eq!(draws[0].vertices[0].position, expected.vertices[0].position);
    }
}
#[test]
fn t824_theme_alpha_multiplies_ghost_opacity_only() {
    let s = fixture();
    let theme = Theme {
        channel_shear: "#ff000080".into(),
        ..Theme::default()
    };
    let p = onion_poses(&s, &theme);
    assert!((p[0].1[3] - 0.32 * 128. / 255.).abs() < 1e-6);
    assert_eq!(
        s.doc.skeleton.slots[s.doc.skeleton.draw_order[0]].color,
        [1.; 4]
    );
}

#[test]
fn t835_onion_and_graph_model_keep_same_logical_clip_after_reorder() {
    use ankhimate_document::commands::animation_order::{Direction, MoveAnimation};
    let mut s = fixture();
    let active = s.session.active_animation.unwrap();
    let other = s.doc.animations.insert(Animation::new("Other", 4.));
    let before = onion_poses(&s, &Theme::default());
    assert!(!before.is_empty());
    let model = |s: &AppState| {
        crate::ui::timeline::model::TimelineModel::build(s, s.session.active_animation.unwrap())
            .groups
            .into_iter()
            .flat_map(|g| {
                g.rows.into_iter().map(|p| {
                    (
                        p.addr,
                        p.keys
                            .iter()
                            .map(|k| (k.index, k.time, k.interp))
                            .collect::<Vec<_>>(),
                        p.channels.into_iter().map(|c| c.values).collect::<Vec<_>>(),
                    )
                })
            })
            .collect::<Vec<_>>()
    };
    let before_model = model(&s);
    let c = MoveAnimation::prepare(&s.doc, active, Direction::Down)
        .unwrap()
        .unwrap();
    s.dispatch(Box::new(c));
    assert_eq!(s.doc.animation_ids(), vec![other, active]);
    assert_eq!(s.session.active_animation, Some(active));
    assert_eq!(model(&s), before_model);
    let after = onion_poses(&s, &Theme::default());
    assert_eq!(before.len(), after.len());
    for ((a, ta), (b, tb)) in before.iter().zip(&after) {
        assert_eq!(ta, tb);
        assert_eq!(a.locals, b.locals);
        assert_eq!(a.worlds, b.worlds);
        assert_eq!(a.deforms, b.deforms);
    }
}
