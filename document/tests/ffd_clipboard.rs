use ankhimate_core::{
    animation::{Interp, Timeline, sample},
    attachment::{Attachment, LinkedMesh},
    ids::{SkinId, SlotId},
    skin::Skin,
};
use ankhimate_document::{
    WorkMode,
    clipboard::deform::{DeformClip, PreparedDeformPaste},
    commands::{History, document_cmds::SetProjectFps, key_cmds::TimelineAddr},
    snapshot::snapshot,
};
use std::sync::Arc;
#[allow(
    dead_code,
    reason = "Shared synthetic fixture helpers across native/editor tests"
)]
mod rig {
    include!("support/t834_fixture.rs");
}
use rig::*;
fn clip(r: &Rig, indices: &[usize]) -> Arc<DeformClip> {
    Arc::new(DeformClip::capture(&r.doc, 7, r.source, &r.refs(indices)).unwrap())
}
fn plan(r: &Rig, p: Arc<DeformClip>, time: f32) -> Result<PreparedDeformPaste, String> {
    PreparedDeformPaste::prepare(&r.doc, 7, r.destination, p, &r.addr(), time)
}
#[test]
fn t834_owned_vectors_curves_source_deletion_and_exact_three_cycles() {
    for weighted in [false, true] {
        let mut r = fixture(weighted);
        let payload = clip(&r, &[2, 0, 1]);
        assert_eq!(payload.keys(), deform(&r.doc, r.source));
        let source_snapshot = r.doc.animations[r.source].clone();
        r.doc.animations.remove(r.source);
        let before = snapshot(&r.doc);
        let mut expected = r.doc.clone();
        let mut keys = deform(&r.doc, r.destination).to_vec();
        for k in payload.keys() {
            let mut k = k.clone();
            k.time = 2. + (k.time - 0.25);
            keys.push(k);
        }
        if let Timeline::Deform { keys: dst, .. } =
            &mut expected.animations[r.destination].timelines[0]
        {
            *dst = keys;
        }
        let p = plan(&r, payload.clone(), 2.).unwrap();
        assert_eq!(p.summary.keys, 3);
        assert_eq!((p.summary.first, p.summary.last), (2., 3.));
        let c = p.into_command(&r.doc, 7, &payload).unwrap();
        let mut h = History::default();
        assert!(h.push_in_mode(c, &mut r.doc, WorkMode::Animate));
        assert_eq!(h.undo_depth(), 1);
        assert!(snapshot(&r.doc) == snapshot(&expected));
        assert_eq!(source_snapshot.timelines.len(), 1);
        let after = snapshot(&r.doc);
        drop(payload);
        for _ in 0..3 {
            assert!(h.undo(&mut r.doc));
            assert!(snapshot(&r.doc) == before);
            assert!(h.redo(&mut r.doc));
            assert!(snapshot(&r.doc) == after);
        }
    }
}
#[test]
fn t834_sampling_preserved_at_equivalent_shifted_times() {
    for weighted in [false, true] {
        let mut r = fixture(weighted);
        let source = deform(&r.doc, r.source).to_vec();
        let p = clip(&r, &[0, 1, 2]);
        let cmd = plan(&r, p.clone(), 2.)
            .unwrap()
            .into_command(&r.doc, 7, &p)
            .unwrap();
        let mut h = History::default();
        h.push_in_mode(cmd, &mut r.doc, WorkMode::Animate);
        for t in [0.25, 0.375, 0.5, 0.75, 0.875, 1., 1.125, 1.25] {
            let a = sample(&source, t, &mut 0).unwrap();
            let b = sample(deform(&r.doc, r.destination), t + 1.75, &mut 0).unwrap();
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                assert!((*a - b).length() <= 0.00001, "t={t} {a:?} {b:?}");
            }
        }
    }
}
#[test]
fn t834_copy_one_or_many_no_document_mutation_and_negative_zero_extremes() {
    let mut r = fixture(false);
    let n = r.mesh().setup_vertices.len();
    for (i, v) in [-1e8, -1e-20, 0., 1e-20, 1e8].iter().enumerate() {
        deform_mut(&mut r.doc, r.source)[1].value =
            vec![glam::vec2(*v, if i % 2 == 0 { 0. } else { *v }); n];
        let before = snapshot(&r.doc);
        let p = clip(&r, &[1]);
        assert_eq!(p.keys()[0].value, deform(&r.doc, r.source)[1].value);
        assert!(snapshot(&r.doc) == before);
    }
    let p = clip(&r, &[0, 1, 2]);
    assert_eq!(p.keys().len(), 3);
}
#[test]
fn t834_copy_rejects_mixed_tracks_stale_indices_and_duplicate_refs() {
    let r = fixture(false);
    for refs in [
        vec![],
        r.refs(&[1, 1]),
        r.refs(&[99]),
        vec![
            r.refs(&[0])[0].clone(),
            ankhimate_document::commands::key_cmds::KeyRef {
                addr: TimelineAddr::Bone {
                    bone: r.doc.skeleton.bones.keys().next().unwrap(),
                    property: ankhimate_document::commands::key_cmds::BoneProperty::Rotate,
                    axis: None,
                },
                index: 0,
            },
        ],
    ] {
        assert!(DeformClip::capture(&r.doc, 7, r.source, &refs).is_err());
    }
    let mut other = r.refs(&[0]);
    other[0].addr = TimelineAddr::Deform {
        slot: r.slot,
        attachment: "other".into(),
        owner: Some(r.skin),
    };
    let mut refs = r.refs(&[0]);
    refs.extend(other);
    assert!(DeformClip::capture(&r.doc, 7, r.source, &refs).is_err());
}
#[test]
fn t834_exact_owner_not_names_or_equal_vertex_count() {
    let mut r = fixture(false);
    let p = clip(&r, &[0, 1]);
    let other = another_mesh(&mut r);
    let before = snapshot(&r.doc);
    assert!(PreparedDeformPaste::prepare(&r.doc, 7, r.destination, p.clone(), &other, 2.).is_err());
    assert!(
        PreparedDeformPaste::prepare(&r.doc, 8, r.destination, p.clone(), &r.addr(), 2.).is_err()
    );
    for target in [
        TimelineAddr::Deform {
            slot: SlotId::default(),
            attachment: "mesh".into(),
            owner: Some(r.skin),
        },
        TimelineAddr::Deform {
            slot: r.slot,
            attachment: "mesh".into(),
            owner: Some(SkinId::default()),
        },
    ] {
        assert!(
            PreparedDeformPaste::prepare(&r.doc, 7, r.destination, p.clone(), &target, 2.).is_err()
        );
    }
    assert!(snapshot(&r.doc) == before);
    let skin = r.doc.skeleton.skins.insert(Skin {
        name: "alternate".into(),
        ..Default::default()
    });
    let m = r.mesh().clone();
    r.doc.skeleton.skins[skin].set(r.slot, "mesh", Attachment::Mesh(m));
    assert!(plan(&r, p, 2.).is_err());
}
#[test]
fn t834_topology_structure_order_uv_weights_binds_and_setup_changes_refuse() {
    for change in 0..10 {
        let mut r = fixture(true);
        let p = clip(&r, &[0, 1]);
        match change {
            0 => r.mesh_mut().setup_vertices.swap(0, 1),
            1 => r.mesh_mut().uvs.swap(0, 1),
            2 => r.mesh_mut().triangles.swap(0, 1),
            3 => r.mesh_mut().weights[0].swap(0, 1),
            4 => r.mesh_mut().weights[0][0].weight = 0.9,
            5 => {
                let id = *r.mesh().inverse_bind_matrices.keys().next().unwrap();
                r.mesh_mut().inverse_bind_matrices.get_mut(&id).unwrap().tx += 1.;
            }
            6 => {
                let b = r.doc.skeleton.slots[r.slot].bone;
                r.doc.skeleton.bones[b].local_transform.rotation += 0.1;
            }
            7 => r.mesh_mut().edges.push([0, 2]),
            8 => r.mesh_mut().texture = "other art".into(),
            _ => {
                r.doc.skeleton.skins[r.skin]
                    .entries
                    .remove(&(r.slot, "mesh".into()));
            }
        }
        let before = snapshot(&r.doc);
        assert!(plan(&r, p, 2.).is_err(), "change{change}");
        assert!(snapshot(&r.doc) == before);
    }
}
#[test]
fn t834_linked_shared_and_sequence_refuse() {
    for shared in [false, true] {
        let mut r = fixture(false);
        let p = clip(&r, &[0, 1]);
        let link = LinkedMesh {
            skin: None,
            slot: r.doc.skeleton.slots[r.slot].name.clone(),
            attachment: "mesh".into(),
            inherit_deform: true,
        };
        if shared {
            let mut other = r.mesh().clone();
            other.linked = Some(link);
            r.doc.skeleton.skins[r.skin].set(r.slot, "borrower", Attachment::Mesh(other));
        } else {
            r.mesh_mut().linked = Some(link);
        }
        assert!(plan(&r, p, 2.).is_err());
        assert!(DeformClip::capture(&r.doc, 7, r.source, &r.refs(&[0])).is_err());
    }
}
#[test]
fn t834_invalid_vectors_times_handles_and_topology_refused_locally() {
    for change in 0..10 {
        let mut r = fixture(false);
        match change {
            0 => deform_mut(&mut r.doc, r.source)[1]
                .value
                .pop()
                .map(|_| ())
                .unwrap(),
            1 => deform_mut(&mut r.doc, r.source)[1].value[0].x = f32::NAN,
            2 => deform_mut(&mut r.doc, r.source)[1].time = f32::INFINITY,
            3 => deform_mut(&mut r.doc, r.source)[1].time = 0.25,
            4 => {
                deform_mut(&mut r.doc, r.source)[1].interp = Interp::Bezier {
                    out_handle: glam::vec2(2., 0.),
                    in_handle: glam::Vec2::ZERO,
                }
            }
            5 => r.mesh_mut().triangles[0][0] = 99,
            6 => r.mesh_mut().triangles[0] = [0, 0, 1],
            7 => r.mesh_mut().setup_vertices[0] = r.mesh().setup_vertices[1],
            8 => r.mesh_mut().uvs.pop().map(|_| ()).unwrap(),
            _ => {
                deform_mut(&mut r.doc, r.source)[1].time = -0.1;
            }
        }
        assert!(
            DeformClip::capture(&r.doc, 7, r.source, &r.refs(&[0, 1, 2])).is_err(),
            "change{change}"
        );
    }
}
#[test]
fn t834_empty_native_zero_vectors_are_preserved() {
    let mut r = fixture(true);
    deform_mut(&mut r.doc, r.source)[0].value.clear();
    let p = clip(&r, &[0]);
    assert!(p.keys()[0].value.is_empty());
    let c = plan(&r, p.clone(), 2.)
        .unwrap()
        .into_command(&r.doc, 7, &p)
        .unwrap();
    let mut h = History::default();
    h.push_in_mode(c, &mut r.doc, WorkMode::Animate);
    assert!(
        deform(&r.doc, r.destination)
            .last()
            .unwrap()
            .value
            .is_empty()
    );
}
#[test]
fn t834_collisions_invalid_playhead_overflow_and_rounding_preserve_redo() {
    let mut r = fixture(false);
    let mut h = History::default();
    h.push_in_mode(
        Box::new(SetProjectFps::new(&r.doc, 60).unwrap().unwrap()),
        &mut r.doc,
        WorkMode::Animate,
    );
    h.undo(&mut r.doc);
    let p = clip(&r, &[0, 1, 2]);
    for time in [0., f32::NAN, f32::INFINITY, -1., 1e20] {
        let before = snapshot(&r.doc);
        assert!(plan(&r, p.clone(), time).is_err(), "{time}");
        assert!(snapshot(&r.doc) == before);
        assert!(h.can_redo());
    }
    deform_mut(&mut r.doc, r.source)[0].time = 0.;
    deform_mut(&mut r.doc, r.source)[1].time = f32::MAX / 2.;
    deform_mut(&mut r.doc, r.source)[2].time = f32::MAX;
    let p = clip(&r, &[0, 1, 2]);
    assert!(plan(&r, p, f32::MAX).is_err());
}
#[test]
fn t834_stale_plan_payload_target_and_owner_refused() {
    for change in 0..5 {
        let mut r = fixture(false);
        let p = clip(&r, &[0, 1]);
        let prepared = plan(&r, p.clone(), 2.).unwrap();
        let mut actual = p.clone();
        match change {
            0 => actual = clip(&r, &[0, 1]),
            1 => deform_mut(&mut r.doc, r.destination)[0].value[0].x += 1.,
            2 => r.doc.animations[r.destination].duration = 7.,
            3 => r.mesh_mut().setup_vertices.swap(0, 1),
            _ => {
                r.doc.animations.remove(r.destination);
            }
        }
        let before = snapshot(&r.doc);
        assert!(prepared.into_command(&r.doc, 7, &actual).is_err());
        assert!(snapshot(&r.doc) == before);
    }
}
#[test]
fn t834_outside_keys_preserve_duration_and_setup_refusal() {
    let mut r = fixture(false);
    let p = clip(&r, &[0, 1, 2]);
    let prepared = plan(&r, p.clone(), 5.).unwrap();
    assert_eq!(prepared.summary.outside_duration, 3);
    assert_eq!(prepared.summary.duration, 4.);
    let before = snapshot(&r.doc);
    let c = prepared.into_command(&r.doc, 7, &p).unwrap();
    let mut h = History::default();
    assert!(!h.push_in_mode(c, &mut r.doc, WorkMode::Setup));
    assert!(snapshot(&r.doc) == before);
    let c = plan(&r, p.clone(), 5.)
        .unwrap()
        .into_command(&r.doc, 7, &p)
        .unwrap();
    assert!(h.push_in_mode(c, &mut r.doc, WorkMode::Animate));
    assert_eq!(r.doc.animations[r.destination].duration, 4.);
}
#[test]
fn t834_destination_missing_empty_or_invalid_is_not_repaired() {
    for change in 0..5 {
        let mut r = fixture(false);
        let p = clip(&r, &[0, 1]);
        match change {
            0 => {
                r.doc.animations[r.destination].timelines.clear();
            }
            1 => deform_mut(&mut r.doc, r.destination).clear(),
            2 => deform_mut(&mut r.doc, r.destination)[0]
                .value
                .pop()
                .map(|_| ())
                .unwrap(),
            3 => deform_mut(&mut r.doc, r.destination)[0].value[0].y = f32::INFINITY,
            _ => {
                let duplicate = r.doc.animations[r.destination].timelines[0].clone();
                r.doc.animations[r.destination].timelines.push(duplicate);
            }
        }
        let before = snapshot(&r.doc);
        assert!(plan(&r, p, 2.).is_err());
        assert!(snapshot(&r.doc) == before);
    }
}
#[test]
fn t834_sequence_inline_weights_and_bind_errors_refuse() {
    for change in 0..5 {
        let mut r = fixture(true);
        match change {
            0 => r.mesh_mut().sequence = Some(Default::default()),
            1 => r
                .mesh_mut()
                .ffd_keyframes
                .push(ankhimate_core::attachment::FfdKeyframe {
                    time: 0.,
                    vertex_offsets: vec![glam::Vec2::ONE],
                }),
            2 => {
                let b = r.mesh().weights[0][0].bone;
                r.mesh_mut().weights[0][1].bone = b;
            }
            3 => {
                let b = *r.mesh().inverse_bind_matrices.keys().next().unwrap();
                r.mesh_mut().inverse_bind_matrices.get_mut(&b).unwrap().a = f32::NAN;
            }
            _ => {
                r.mesh_mut().weights[0] = vec![
                    ankhimate_core::attachment::VertexWeight {
                        bone: r.doc.skeleton.bones.keys().next().unwrap(),
                        weight: 0.2
                    };
                    5
                ];
            }
        }
        assert!(DeformClip::capture(&r.doc, 7, r.source, &r.refs(&[0])).is_err());
    }
}
#[test]
fn t834_distinct_pastes_do_not_merge_and_other_clips_stay_exact() {
    let mut r = fixture(false);
    let source = r.doc.animations[r.source].clone();
    let p = clip(&r, &[0, 1]);
    let mut h = History::default();
    let c = plan(&r, p.clone(), 2.)
        .unwrap()
        .into_command(&r.doc, 7, &p)
        .unwrap();
    h.push_in_mode(c, &mut r.doc, WorkMode::Animate);
    let middle = snapshot(&r.doc);
    let c = plan(&r, p.clone(), 3.)
        .unwrap()
        .into_command(&r.doc, 7, &p)
        .unwrap();
    h.push_in_mode(c, &mut r.doc, WorkMode::Animate);
    assert_eq!(h.undo_depth(), 2);
    assert_eq!(r.doc.animations[r.source], source);
    h.undo(&mut r.doc);
    assert!(snapshot(&r.doc) == middle);
}
#[test]
fn t834_cpu_capture_prepare_validation_apply_undo_redo_observations() {
    for vertices in [10, 100, 1000] {
        for count in [1, 10, 100] {
            let mut r = fixture(false);
            let m = r.mesh_mut();
            m.setup_vertices = (0..vertices)
                .map(|i| {
                    let a = i as f32 / vertices as f32 * std::f32::consts::TAU;
                    glam::vec2(a.cos() * 100., a.sin() * 100.)
                })
                .collect();
            m.uvs = m
                .setup_vertices
                .iter()
                .map(|p| *p / 200. + glam::Vec2::splat(0.5))
                .collect();
            m.triangles = (1..vertices - 1)
                .map(|i| [0, i as u32, (i + 1) as u32])
                .collect();
            *deform_mut(&mut r.doc, r.source) = (0..count)
                .map(|i| {
                    ankhimate_core::animation::Key::linear(
                        i as f32 * 0.25,
                        vec![glam::vec2(i as f32, -0.1); vertices],
                    )
                })
                .collect();
            deform_mut(&mut r.doc, r.destination)[0].value = vec![glam::Vec2::ZERO; vertices];
            r.doc.animations[r.destination].duration = 100.;
            let refs = r.refs(&(0..count).collect::<Vec<_>>());
            let t = std::time::Instant::now();
            let p = Arc::new(DeformClip::capture(&r.doc, 7, r.source, &refs).unwrap());
            let capture = t.elapsed();
            let t = std::time::Instant::now();
            let prepared = plan(&r, p.clone(), 50.).unwrap();
            let prepare = t.elapsed();
            let t = std::time::Instant::now();
            let c = prepared.into_command(&r.doc, 7, &p).unwrap();
            let validate = t.elapsed();
            let mut h = History::default();
            let t = std::time::Instant::now();
            h.push_in_mode(c, &mut r.doc, WorkMode::Animate);
            let apply = t.elapsed();
            let t = std::time::Instant::now();
            h.undo(&mut r.doc);
            let undo = t.elapsed();
            let t = std::time::Instant::now();
            h.redo(&mut r.doc);
            eprintln!(
                "T834 CPU vertices={vertices} keys={count}: capture+payload={capture:?} prepare={prepare:?} final_validation={validate:?} apply={apply:?} undo={undo:?} redo={:?}",
                t.elapsed()
            );
        }
    }
}
