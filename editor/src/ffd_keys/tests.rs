use super::*;
use crate::app_state::AppState;
use ankhimate_core::{
    animation::{Animation, Interp, Key, Timeline},
    attachment::{Attachment, MeshAttachment, VertexWeight},
    ids::{AnimationId, SkinId, SlotId},
    math::Transform,
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::{Document, commands::History, snapshot::snapshot};
use glam::{Vec2, vec2};
fn reference(slot: SlotId, skin: SkinId, index: usize) -> KeyRef {
    KeyRef {
        addr: TimelineAddr::Deform {
            slot,
            attachment: "mesh".into(),
            owner: Some(skin),
        },
        index,
    }
}
fn select(s: &mut AppState, a: AnimationId, slot: SlotId, skin: SkinId, index: usize) -> Lease {
    key_selection::set(s, a, vec![reference(slot, skin, index)]);
    selected(s, a).unwrap()
}
fn keys(s: &AppState, a: AnimationId) -> Vec<Key<Vec<Vec2>>> {
    match &s.doc.animations[a].timelines[0] {
        Timeline::Deform { keys, .. } => keys.clone(),
        _ => panic!("Missing Deform"),
    }
}
pub(crate) fn fixture(weighted: bool) -> (AppState, AnimationId, SlotId, SkinId) {
    let mut s = AppState::default();
    let b = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 5.,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut m = MeshAttachment {
        texture: "image".into(),
        setup_vertices: vec![vec2(0., 0.), vec2(2., 0.), vec2(0., 2.)],
        uvs: vec![vec2(0., 0.), vec2(1., 0.), vec2(0., 1.)],
        triangles: vec![[0, 1, 2]],
        ..Default::default()
    };
    if weighted {
        let other = s.doc.skeleton.add_bone(Bone {
            name: "influence".into(),
            parent: Some(b),
            length: 2.,
            local_transform: Transform::default(),
            inherit: Default::default(),
            color: Bone::default_color(),
        });
        m.weights = vec![
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
        for id in [b, other] {
            m.inverse_bind_matrices
                .insert(id, ankhimate_core::transforms::Affine2::IDENTITY);
        }
    }
    let skin = s.doc.skeleton.default_skin;
    let slot = s.doc.skeleton.add_slot(Slot::new("body".into(), b));
    s.doc.skeleton.slots[slot].attachment = Some("mesh".into());
    s.doc.skeleton.skins[skin].set(slot, "mesh", Attachment::Mesh(m));
    let n = if weighted { 6 } else { 3 };
    let mut a = Animation::new("impact", 2.);
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![
            Key::linear(0., vec![Vec2::ZERO; n]),
            Key {
                time: 1.,
                value: (0..n).map(|i| vec2(i as f32 + 1., -(i as f32))).collect(),
                interp: Interp::Linear,
            },
            Key::linear(2., vec![Vec2::ZERO; n]),
        ],
    });
    a.timelines.push(Timeline::BoneRotate {
        bone: b,
        keys: vec![Key::linear(0., 0.), Key::linear(2., 20.)],
    });
    let anim = s.doc.animations.insert(a);
    s.session.active_animation = Some(anim);
    s.session.select_slot(Some(slot));
    s.set_work_mode(crate::session::WorkMode::Animate);
    s.refresh_pose();
    (s, anim, slot, skin)
}

#[test]
fn t821_select_exact_skin_slot_attachment_and_not_scalar() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    assert_eq!(l.capture.key, reference(slot, skin, 1));
    assert_eq!(
        key_selection::read(&mut s, a),
        vec![reference(slot, skin, 1)]
    );
    assert!(
        ankhimate_document::clipboard::KeyClip::capture(&s.doc, a, &[reference(slot, skin, 1)])
            .is_err()
    );
    assert!(crate::tangent_linking::selected(&mut s).is_err());
}
#[test]
fn t821_rigid_move_changes_only_time_and_preserves_full_document_on_undo() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    let before = snapshot(&s.doc);
    let old = keys(&s, a);
    apply(&mut s, &l, Action::Move(1.5), None).unwrap();
    let mut expected = old;
    expected[1].time = 1.5;
    assert_eq!(keys(&s, a), expected);
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 1);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t821_weighted_influence_count_not_visible_vertex_count() {
    let (mut s, a, slot, skin) = fixture(true);
    let l = select(&mut s, a, slot, skin, 1);
    assert_eq!(l.capture.keys[1].value.len(), 6);
    apply(&mut s, &l, Action::Move(1.2), None).unwrap();
    assert_eq!(keys(&s, a)[1].value, l.capture.keys[1].value);
    if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
        keys[1].value.truncate(3);
    }
    assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
}
#[test]
fn t821_invalid_times_collisions_and_overflow_preserve_document_redo() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Move(1.5), None).unwrap();
    s.undo();
    let l = select(&mut s, a, slot, skin, 1);
    let before = snapshot(&s.doc);
    for t in [-1., f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 2.] {
        assert!(apply(&mut s, &l, Action::Move(t), None).is_err());
        assert!(snapshot(&s.doc) == before);
        assert!(s.history.can_redo());
    }
    assert!(move_delta(&mut s, a, f32::MAX, Some(2)).is_ok());
    let l = selected(&mut s, a).unwrap();
    assert!(move_delta(&mut s, a, f32::MAX, Some(2)).is_err());
    assert!(l.matches(&s));
}
#[test]
fn t821_duration_growth_and_undo_restore_exact_prior_duration() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Move(5.), None).unwrap();
    assert_eq!(s.doc.animations[a].duration, 5.);
    s.undo();
    assert_eq!(s.doc.animations[a].duration, 2.);
    s.redo();
    assert_eq!(s.doc.animations[a].duration, 5.);
}
#[test]
fn t821_delete_restores_exact_offsets_and_track_order_three_cycles() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    let before = snapshot(&s.doc);
    let old = keys(&s, a);
    apply(&mut s, &l, Action::Delete, None).unwrap();
    assert_eq!(keys(&s, a), vec![old[0].clone(), old[2].clone()]);
    let after = snapshot(&s.doc);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t821_delete_last_key_removes_track_and_reopens() {
    let (mut s, a, slot, skin) = fixture(false);
    if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
        keys.truncate(1);
    }
    let l = select(&mut s, a, slot, skin, 0);
    let before = snapshot(&s.doc);
    apply(&mut s, &l, Action::Delete, None).unwrap();
    assert!(
        !s.doc.animations[a]
            .timelines
            .iter()
            .any(|t| matches!(t, Timeline::Deform { .. }))
    );
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("last.ankh");
    ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
    let d = Document::from_loaded(ankhimate_formats::load(&path).unwrap().0);
    assert!(
        !d.animations
            .values()
            .flat_map(|a| &a.timelines)
            .any(|t| matches!(t, Timeline::Deform { .. }))
    );
    s.undo();
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t821_linear_stepped_bezier_sampling_uses_ending_key() {
    for interp in [
        Interp::Linear,
        Interp::Stepped,
        Interp::Bezier {
            out_handle: vec2(0.25, 0.),
            in_handle: vec2(0.75, 0.),
        },
    ] {
        let (mut s, a, slot, skin) = fixture(false);
        let l = select(&mut s, a, slot, skin, 1);
        let values = keys(&s, a)
            .iter()
            .map(|k| k.value.clone())
            .collect::<Vec<_>>();
        apply(&mut s, &l, Action::Interpolation(interp), None).unwrap();
        let k = keys(&s, a);
        assert_eq!(
            k.iter().map(|k| k.value.clone()).collect::<Vec<_>>(),
            values
        );
        assert_eq!(k[1].interp, interp);
        assert_eq!(k[2].interp, Interp::Linear);
        for time in [0., 0.25, 0.5, 0.75, 1., 2.] {
            let v = ankhimate_core::animation::sample(&k, time, &mut 0).unwrap();
            if time == 1. {
                assert_eq!(v, k[1].value);
            }
            if time == 0.5 {
                let factor = match interp {
                    Interp::Linear => 0.5,
                    Interp::Stepped => 0.,
                    _ => 0.125,
                };
                for (i, p) in v.iter().enumerate() {
                    assert!((*p - k[1].value[i] * factor).length() < 1e-5);
                }
            }
        }
        let mut p = ankhimate_core::pose::Pose::default();
        ankhimate_core::pose::evaluate(&s.doc.skeleton, &[(&s.doc.animations[a], 0.5, 1.)], &mut p);
        assert!(p.deforms.contains_key(&(slot, "mesh".into())));
    }
}
#[test]
fn t821_empty_reset_vectors_preserved_with_native_asymmetric_sampling() {
    let (mut s, a, slot, skin) = fixture(false);
    if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
        keys[0].value.clear();
        keys[2].value.clear();
    }
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Move(1.5), None).unwrap();
    let k = keys(&s, a);
    assert!(k[0].value.is_empty() && k[2].value.is_empty());
    assert!(
        ankhimate_core::animation::sample(&k, 0.75, &mut 0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        ankhimate_core::animation::sample(&k, 1.75, &mut 0).unwrap(),
        k[1].value
    );
    assert!(
        ankhimate_core::animation::sample(&k, 2., &mut 0)
            .unwrap()
            .is_empty()
    );
}
#[test]
fn t821_invalid_interpolation_refuses_without_clearing_redo() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Delete, None).unwrap();
    s.undo();
    let l = select(&mut s, a, slot, skin, 1);
    for h in [
        vec2(-0.1, 0.),
        vec2(1.1, 0.),
        vec2(0.5, f32::NAN),
        vec2(f32::INFINITY, 0.),
    ] {
        assert!(
            apply(
                &mut s,
                &l,
                Action::Interpolation(Interp::Bezier {
                    out_handle: h,
                    in_handle: Vec2::ONE
                }),
                None
            )
            .is_err()
        );
        assert!(s.history.can_redo());
    }
}
#[test]
fn t821_noop_and_refused_mixed_selection_never_dispatch() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    let before = snapshot(&s.doc);
    apply(&mut s, &l, Action::Move(1.), None).unwrap();
    apply(&mut s, &l, Action::Interpolation(Interp::Linear), None).unwrap();
    key_selection::set(
        &mut s,
        a,
        vec![reference(slot, skin, 0), reference(slot, skin, 1)],
    );
    assert_eq!(
        key_selection::read(&mut s, a),
        vec![reference(slot, skin, 1)]
    );
    assert!(snapshot(&s.doc) == before);
    assert!(!s.history.can_undo());
}
#[test]
fn t821_stale_document_clip_revision_topology_and_attachment_refuse() {
    for case in 0..7 {
        let (mut s, a, slot, skin) = fixture(false);
        let l = select(&mut s, a, slot, skin, 1);
        match case {
            0 => s.revision += 1,
            1 => {
                s.session.active_animation =
                    Some(s.doc.animations.insert(Animation::new("other", 2.)));
            }
            2 => s.replace_document(s.doc.clone()),
            3 => {
                if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
                    .entries
                    .get_mut(&(slot, "mesh".into()))
                {
                    m.setup_vertices.swap(0, 1);
                }
            }
            4 => {
                s.doc.skeleton.skins[skin]
                    .entries
                    .remove(&(slot, "mesh".into()));
            }
            5 => {
                if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
                    keys.swap(0, 1);
                }
            }
            _ => {
                s.doc.animations[a].timelines.remove(0);
            }
        }
        let before = snapshot(&s.doc);
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
        assert!(snapshot(&s.doc) == before);
    }
}
#[test]
fn t821_hidden_panel_skin_and_mode_roundtrips_clear_selection() {
    for case in 0..2 {
        let (mut s, a, slot, skin) = fixture(false);
        let l = select(&mut s, a, slot, skin, 1);
        if case == 0 {
            let other = s
                .doc
                .skeleton
                .skins
                .insert(ankhimate_core::skin::Skin::new("alternate"));
            s.session.set_active_skin(other);
            s.session.set_active_skin(skin);
        } else {
            s.set_work_mode(crate::session::WorkMode::Setup);
            s.set_work_mode(crate::session::WorkMode::Animate);
        }
        assert!(key_selection::read(&mut s, a).is_empty());
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
    }
}
#[test]
fn t821_setup_and_slot_influence_locks_refuse() {
    for weighted in [false, true] {
        let (mut s, a, slot, skin) = fixture(weighted);
        let l = select(&mut s, a, slot, skin, 1);
        let bone = s.doc.skeleton.slots[slot].bone;
        s.session.locked_bones.insert(bone, true);
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
        s.session.locked_bones.remove(bone);
        s.set_work_mode(crate::session::WorkMode::Setup);
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
    }
}
#[test]
fn t821_skin_overrides_duplicate_tracks_and_wrong_owner_refuse() {
    let (mut s, a, slot, skin) = fixture(false);
    let mesh = s.doc.skeleton.skins[skin]
        .get(slot, "mesh")
        .unwrap()
        .clone();
    let mut alt = ankhimate_core::skin::Skin::new("alt");
    alt.set(slot, "mesh", mesh);
    s.doc.skeleton.skins.insert(alt);
    assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
    let (mut s, a, slot, skin) = fixture(false);
    let track = s.doc.animations[a].timelines[0].clone();
    s.doc.animations[a].timelines.push(track);
    assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
}
#[test]
fn t821_fallback_to_default_owner_is_explicit_and_valid() {
    let (mut s, a, slot, skin) = fixture(false);
    let alt = s
        .doc
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("alt"));
    s.session.set_active_skin(alt);
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Move(1.25), None).unwrap();
    assert_eq!(keys(&s, a)[1].time, 1.25);
}
#[test]
fn t821_linked_target_and_shared_source_refuse() {
    for source in [false, true] {
        let (mut s, a, slot, skin) = fixture(false);
        let link = ankhimate_core::attachment::LinkedMesh {
            skin: None,
            slot: "body".into(),
            attachment: "mesh".into(),
            inherit_deform: true,
        };
        if source {
            s.doc.skeleton.skins[skin].set(
                slot,
                "linked",
                Attachment::Mesh(MeshAttachment {
                    linked: Some(link),
                    ..Default::default()
                }),
            );
        } else {
            if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
                .entries
                .get_mut(&(slot, "mesh".into()))
            {
                m.linked = Some(link);
            }
        }
        assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
    }
}
#[test]
fn t821_invalid_offset_size_nan_topology_and_bind_refuse() {
    for case in 0..6 {
        let (mut s, a, slot, skin) = fixture(true);
        match case {
            0 => {
                if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
                    keys[1].value.pop();
                }
            }
            1 => {
                if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
                    keys[1].value[0].x = f32::NAN;
                }
            }
            _ => {
                if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
                    .entries
                    .get_mut(&(slot, "mesh".into()))
                {
                    match case {
                        2 => m.triangles[0][0] = 999,
                        3 => m.uvs.pop().map(|_| ()).unwrap(),
                        4 => m.inverse_bind_matrices.clear(),
                        _ => m.weights[0][0].weight = -1.,
                    }
                }
            }
        }
        assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
    }
}
#[test]
fn t821_native_animate_guard_and_drag_merge_tokens() {
    let (mut s, a, slot, skin) = fixture(false);
    let l = select(&mut s, a, slot, skin, 1);
    let mut h = History::default();
    let cmd = l
        .capture
        .command(&s.doc, Action::Delete, None)
        .unwrap()
        .unwrap();
    assert!(!h.push_in_mode(
        Box::new(cmd),
        &mut s.doc,
        ankhimate_document::WorkMode::Setup
    ));
    for (token, time) in [(31, 1.1), (31, 1.2), (32, 1.3)] {
        let l = selected(&mut s, a).unwrap();
        apply(&mut s, &l, Action::Move(time), Some(token)).unwrap();
    }
    assert_eq!(s.history.undo_depth(), 2);
}
#[test]
fn t821_move_crossing_rebinds_by_time_and_keeps_single_undo() {
    let (mut s, a, slot, skin) = fixture(false);
    select(&mut s, a, slot, skin, 1);
    move_delta(&mut s, a, 1.5, Some(81)).unwrap();
    assert_eq!(key_selection::read(&mut s, a)[0].index, 2);
    move_delta(&mut s, a, 0.5, Some(81)).unwrap();
    assert_eq!(keys(&s, a)[2].time, 3.);
    assert_eq!(s.history.undo_depth(), 1);
    s.undo();
    assert_eq!(keys(&s, a)[1].time, 1.);
    assert_eq!(s.doc.animations[a].duration, 2.);
}

fn add_image(s: &mut AppState) {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([30, 160, 210, 190]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    s.doc.assets.add(ankhimate_core::assets::ImageAsset::new(
        "image",
        bytes.into_inner(),
        2,
        2,
    ));
}
#[test]
fn t821_save_reload_and_real_v2_export_preserve_unaffected_fields() {
    for weighted in [false, true] {
        let (mut s, a, slot, skin) = fixture(weighted);
        add_image(&mut s);
        let tmp = tempfile::tempdir().unwrap();
        let original = s.doc.clone();
        let export = |doc: &Document, name: &str| {
            let project = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
            let plan = ankhimate_export::run::plan(
                &project,
                &doc.assets,
                &ankhimate_export::presets::default_preset(),
            )
            .unwrap();
            let dir = tmp.path().join(name);
            ankhimate_export::run::write(&plan, &dir).unwrap();
            serde_json::from_slice::<serde_json::Value>(
                &std::fs::read(dir.join("skeleton.json")).unwrap(),
            )
            .unwrap()
        };
        let mut before = export(&s.doc, "before");
        let l = select(&mut s, a, slot, skin, 1);
        apply(&mut s, &l, Action::Move(1.25), None).unwrap();
        let l = selected(&mut s, a).unwrap();
        apply(
            &mut s,
            &l,
            Action::Interpolation(Interp::Bezier {
                out_handle: vec2(0.25, -0.5),
                in_handle: vec2(0.75, 1.5),
            }),
            None,
        )
        .unwrap();
        let expected = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
        let path = tmp.path().join("Déformation copie.ankh");
        ankhimate_formats::save(&path, &s.doc.as_project_ref(), &[]).unwrap();
        let loaded = ankhimate_formats::load(&path).unwrap().0;
        assert!(loaded.report.is_clean());
        let reopened = Document::from_loaded(loaded);
        assert_eq!(
            expected,
            ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap()
        );
        let mut after = export(&reopened, "after");
        assert_eq!(after["format_version"], 2);
        let deform = &after["animations"]["impact"]["deform"][0];
        assert_eq!(deform["keys"][1]["time"], 1.25);
        assert_eq!(
            deform["keys"][1]["offsets"],
            before["animations"]["impact"]["deform"][0]["keys"][1]["offsets"]
        );
        assert_ne!(
            deform["keys"][0]["curve"],
            before["animations"]["impact"]["deform"][0]["keys"][0]["curve"]
        );
        before["animations"]["impact"]
            .as_object_mut()
            .unwrap()
            .remove("deform");
        after["animations"]["impact"]
            .as_object_mut()
            .unwrap()
            .remove("deform");
        assert_eq!(before, after);
        assert_eq!(
            original.assets.images.values().next().unwrap().bytes,
            reopened.assets.images.values().next().unwrap().bytes
        );
        if let Some(output) = std::env::var_os("ANKHIMATE_T821_FIXTURES") {
            let dir =
                std::path::PathBuf::from(output).join(if weighted { "Weighted" } else { "Rigid" });
            assert!(!dir.exists());
            std::fs::create_dir_all(&dir).unwrap();
            for (name, d) in [("Original", &original), ("Edited", &reopened)] {
                ankhimate_formats::save(
                    &dir.join(format!("{name}.ankh")),
                    &d.as_project_ref(),
                    &[],
                )
                .unwrap();
            }
        }
    }
}
#[test]
fn t821_recreated_track_and_displayed_attachment_change_invalidate() {
    for case in 0..2 {
        let (mut s, a, slot, skin) = fixture(false);
        let l = select(&mut s, a, slot, skin, 1);
        if case == 0 {
            if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
                keys[1].value[0] = vec2(9., 9.);
            }
        } else {
            s.pose.slot_attachments.insert(slot, Some("other".into()));
        }
        let before = snapshot(&s.doc);
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
        assert!(snapshot(&s.doc) == before);
    }
}
#[test]
fn t821_offsets_bit_patterns_and_inline_ffd_are_untouched() {
    let (mut s, a, slot, skin) = fixture(false);
    if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
        keys[1].value[0].x = -0.;
    }
    if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "mesh".into()))
    {
        m.ffd_keyframes
            .push(ankhimate_core::attachment::FfdKeyframe {
                time: 0.5,
                vertex_offsets: vec![vec2(3., 4.); 3],
            });
    }
    let l = select(&mut s, a, slot, skin, 1);
    let bits = l
        .capture
        .keys
        .iter()
        .flat_map(|k| k.value.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits()]))
        .collect::<Vec<_>>();
    let mesh = l.capture.mesh.clone();
    apply(&mut s, &l, Action::Move(1.25), None).unwrap();
    let next = selected(&mut s, a).unwrap();
    assert_eq!(next.capture.mesh, mesh);
    assert_eq!(
        next.capture
            .keys
            .iter()
            .flat_map(|k| k.value.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits()]))
            .collect::<Vec<_>>(),
        bits
    );
}
#[test]
fn t821_same_name_on_different_slot_does_not_modify_other_mesh() {
    let (mut s, a, slot, skin) = fixture(false);
    let other = s
        .doc
        .skeleton
        .add_slot(Slot::new("other".into(), s.doc.skeleton.slots[slot].bone));
    let mesh = s.doc.skeleton.skins[skin]
        .get(slot, "mesh")
        .unwrap()
        .clone();
    s.doc.skeleton.skins[skin].set(other, "mesh", mesh);
    let mut track = s.doc.animations[a].timelines[0].clone();
    if let Timeline::Deform { slot, .. } = &mut track {
        *slot = other;
    }
    s.doc.animations[a].timelines.push(track.clone());
    let l = select(&mut s, a, slot, skin, 1);
    apply(&mut s, &l, Action::Delete, None).unwrap();
    assert_eq!(s.doc.animations[a].timelines.last().unwrap(), &track);
    assert_ne!(
        reference(slot, skin, 1).addr.stable_id(),
        reference(other, skin, 1).addr.stable_id()
    );
}
#[test]
fn t821_invalid_native_times_and_owner_are_refused() {
    for time in [-1., f32::NAN, f32::INFINITY, 0.] {
        let (mut s, a, slot, skin) = fixture(false);
        if let Timeline::Deform { keys, .. } = &mut s.doc.animations[a].timelines[0] {
            keys[1].time = time;
        }
        assert!(Lease::new(&s, a, reference(slot, skin, 1)).is_err());
    }
    let (mut s, a, slot, skin) = fixture(false);
    let wrong = s
        .doc
        .skeleton
        .skins
        .insert(ankhimate_core::skin::Skin::new("wrong"));
    assert!(Lease::new(&s, a, reference(slot, wrong, 1)).is_err());
    assert!(Lease::new(&s, a, reference(slot, skin, 99)).is_err());
}

#[test]
fn t821_mesh_uv_weights_and_bind_changes_invalidate_even_without_revision() {
    for case in 0..5 {
        let (mut s, a, slot, skin) = fixture(true);
        let l = select(&mut s, a, slot, skin, 1);
        if let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[skin]
            .entries
            .get_mut(&(slot, "mesh".into()))
        {
            match case {
                0 => m.uvs.swap(0, 1),
                1 => m.triangles[0].swap(0, 1),
                2 => m.weights[0].swap(0, 1),
                3 => {
                    m.inverse_bind_matrices.values_mut().next().unwrap().tx += 1.;
                }
                _ => m
                    .ffd_keyframes
                    .push(ankhimate_core::attachment::FfdKeyframe {
                        time: 0.,
                        vertex_offsets: vec![Vec2::ZERO; 6],
                    }),
            }
        }
        let before = snapshot(&s.doc);
        assert!(apply(&mut s, &l, Action::Delete, None).is_err());
        assert!(snapshot(&s.doc) == before);
    }
}
#[test]
fn t821_non_slot_influence_lock_and_generic_clipboard_destination_refuse() {
    let (mut s, a, slot, skin) = fixture(true);
    let l = select(&mut s, a, slot, skin, 1);
    let other = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "influence")
        .unwrap()
        .0;
    s.session.locked_bones.insert(other, true);
    assert!(apply(&mut s, &l, Action::Delete, None).is_err());
    s.session.locked_bones.remove(other);
    let bone = s.doc.skeleton.slots[slot].bone;
    let scalar = KeyRef {
        addr: TimelineAddr::Bone {
            bone,
            property: ankhimate_document::commands::key_cmds::BoneProperty::Rotate,
            axis: None,
        },
        index: 0,
    };
    let clip = ankhimate_document::clipboard::KeyClip::capture(&s.doc, a, &[scalar]).unwrap();
    assert!(
        clip.resolve(&s.doc, a, 0., &[reference(slot, skin, 1).addr])
            .is_err()
    );
    assert!(!s.history.can_undo());
}
