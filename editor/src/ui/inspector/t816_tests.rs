use super::*;
use ankhimate_core::{
    constraints::{Constraint, IkConstraint},
    ids::{BoneId, ConstraintId},
    skeleton::Bone,
};
fn fixture(n: usize) -> (AppState, Vec<BoneId>, BoneId, ConstraintId) {
    let mut s = AppState::default();
    let mut chain = Vec::new();
    for i in 0..n {
        chain.push(s.doc.skeleton.add_bone(Bone {
            name: format!("segment-{i}"),
            parent: chain.last().copied(),
            length: 10.0,
            local_transform: ankhimate_core::math::Transform {
                position: glam::vec2(if i == 0 { 0.0 } else { 10.0 }, 0.0),
                ..Default::default()
            },
            inherit: Default::default(),
            color: Bone::default_color(),
        }));
    }
    let t = s.doc.skeleton.add_bone(Bone {
        name: "IK-handle".into(),
        parent: None,
        length: 0.0,
        local_transform: ankhimate_core::math::Transform {
            position: glam::vec2(12.0, 8.0),
            ..Default::default()
        },
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let c = s
        .doc
        .skeleton
        .add_constraint(Constraint::Ik(IkConstraint::chain(
            "reach",
            t,
            chain.clone(),
        )));
    s.refresh_pose();
    s.session.select_constraint(c);
    (s, chain, t, c)
}
fn draw(s: &mut AppState, c: ConstraintId) -> Vec<String> {
    egui::Context::default()
        .run_ui(Default::default(), |ui| focused_constraint(ui, s, c))
        .shapes
        .iter()
        .filter_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                Some(t.galley.text().to_string())
            } else {
                None
            }
        })
        .collect()
}
#[test]
fn t816_ik_inspector_exposes_root_span_and_solver() {
    let (mut s, _, _, c) = fixture(4);
    let texts = draw(&mut s, c);
    for label in ["Chain root", "Evaluated span", "Solver"] {
        assert!(
            texts.iter().any(|t| t == label),
            "missing {label}: {texts:?}"
        );
    }
}
#[test]
fn t816_empty_selected_ik_explains_invalid_chain() {
    let (mut s, _, _, c) = fixture(0);
    let texts = draw(&mut s, c);
    assert!(
        texts.iter().any(|t| t.contains("empty")),
        "No diagnostic for empty chain: {texts:?}"
    );
}

use ankhimate_core::pose::{Pose, evaluate};
#[test]
fn t816_reach_one_two_three_four_bones_and_far_targets() {
    for n in 1..=4 {
        for far in [false, true] {
            let (mut s, chain, t, _) = fixture(n);
            let goal = if far {
                glam::vec2(100.0, 40.0)
            } else {
                glam::vec2(5.0, 7.0)
            };
            s.doc.skeleton.bones[t].local_transform.position = goal;
            s.refresh_pose();
            let tip = s.pose.world_tip(&s.doc.skeleton, *chain.last().unwrap());
            let reach = 10.0 * n as f32;
            assert!(s.pose.worlds.values().all(|w| {
                [w.a, w.b, w.c, w.d, w.tx, w.ty]
                    .iter()
                    .all(|v| v.is_finite())
            }));
            if far || n == 1 {
                assert!(
                    (tip - goal.normalize() * reach).length() < 0.01,
                    "n={n}, far={far}, tip={tip:?}"
                );
            } else {
                assert!((tip - goal).length() < 0.01, "n={n}: {tip:?}");
            }
            let rows = ik_info::summary(
                &s,
                match &s.doc.skeleton.constraints.values().next().unwrap() {
                    Constraint::Ik(ik) => ik,
                    _ => panic!(),
                },
            )
            .unwrap();
            assert!(
                rows.iter()
                    .any(|(k, v)| *k == "Evaluated span" && v.starts_with(&format!("{reach:.3}")))
            );
        }
    }
}
#[test]
fn t816_invalid_chain_diagnostics_do_not_mutate_or_guess_lengths() {
    for case in 0..5 {
        let (mut s, chain, t, c) = fixture(4);
        if case == 0 {
            s.doc.skeleton.bones.remove(t);
        }
        if case == 1 {
            s.doc.skeleton.bones.remove(chain[3]);
        }
        if let Constraint::Ik(ik) = &mut s.doc.skeleton.constraints[c] {
            match case {
                2 => ik.bones.push(chain[0]),
                3 => ik.bones.reverse(),
                4 => ik.target = chain[0],
                _ => {}
            }
        }
        let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
        let text = draw(&mut s, c);
        assert!(
            text.iter().any(|t| t.contains("Invalid IK")),
            "case{case}: {text:?}"
        );
        assert!(!text.iter().any(|t| t == "Evaluated span"));
        assert_eq!(
            before,
            ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap()
        );
        assert!(!s.history.can_undo());
    }
}
#[test]
fn t816_skipped_bone_span_uses_joint_gaps_and_target_selection_shows_constraint() {
    let (mut s, chain, t, c) = fixture(4);
    if let Constraint::Ik(ik) = &mut s.doc.skeleton.constraints[c] {
        ik.bones = vec![chain[0], chain[2], chain[3]];
        ik.mix = 0.0;
    }
    s.refresh_pose();
    let Constraint::Ik(ik) = &s.doc.skeleton.constraints[c] else {
        panic!()
    };
    let rows = ik_info::summary(&s, ik).unwrap();
    assert!(
        rows.iter()
            .any(|(k, v)| *k == "Evaluated span" && v.starts_with("40.000"))
    );
    s.session.select_bone(Some(t));
    let out =
        egui::Context::default().run_ui(Default::default(), |ui| ik_inspector(ui, &mut s, Some(t)));
    assert!(
        out.shapes
            .iter()
            .any(|s| matches!(&s.shape,egui::Shape::Text(t)if t.galley.text()=="IK-handle")),
        "current target must not be labelled missing"
    );
}
#[test]
fn t816_ik_native_properties_undo_redo_modes_and_bend_mix() {
    use ankhimate_document::commands::constraint_cmds::{IkProps, SetIkProps};
    let (mut s, chain, t, c) = fixture(4);
    let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    let Constraint::Ik(ik) = &s.doc.skeleton.constraints[c] else {
        panic!()
    };
    let mut p = IkProps::from_constraint(ik);
    p.bend_direction = -1.0;
    p.mix = 0.5;
    p.stiffness = 0.25;
    assert!(s.dispatch(Box::new(SetIkProps::new(c, p.clone()))));
    let after = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    assert_ne!(after, before);
    for _ in 0..3 {
        s.undo();
        assert_eq!(
            ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
            before
        );
        s.redo();
        assert_eq!(
            ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
            after
        );
    }
    s.set_work_mode(crate::session::WorkMode::Animate);
    let frozen = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
    p.mix = 0.0;
    assert!(!s.dispatch(Box::new(SetIkProps::new(c, p))));
    assert_eq!(
        ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
        frozen
    );
    // The actual core remains the source of truth: zero mix restores FK.
    if let Constraint::Ik(ik) = &mut s.doc.skeleton.constraints[c] {
        ik.mix = 0.0;
    }
    let mut pose = Pose::new();
    evaluate(&s.doc.skeleton, &[], &mut pose);
    assert!((pose.world_tip(&s.doc.skeleton, chain[3]) - glam::vec2(40.0, 0.0)).length() < 1e-5);
    assert_eq!(pose.world_position(t), glam::vec2(12.0, 8.0));
}
#[test]
fn t816_existing_tentacle_sample_strict_roundtrip_and_temporal_ik() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/tentacle.ankh");
    let original = std::fs::read(&source).unwrap();
    let (loaded, images) = ankhimate_formats::load(&source).unwrap();
    assert!(images.is_empty());
    assert!(loaded.report.dangling.is_empty());
    assert!(loaded.report.lossy.is_empty());
    assert!(loaded.assets.images.is_empty());
    let doc = ankhimate_document::Document::from_loaded(loaded);
    assert_eq!(doc.skeleton.bones.len(), 10);
    assert_eq!(doc.animations.len(), 1);
    let Constraint::Ik(ik) = doc.skeleton.constraints.values().next().unwrap() else {
        panic!()
    };
    assert_eq!(ik.bones.len(), 8);
    assert_eq!(doc.skeleton.bones[ik.target].name, "tentacle-target");
    assert_eq!(ik.mix, 1.0);
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("tentacle-copy.ankh");
    ankhimate_formats::save(&path, &doc.as_project_ref(), &[]).unwrap();
    let (reloaded, _) = ankhimate_formats::load(&path).unwrap();
    assert!(reloaded.report.dangling.is_empty());
    let reloaded = ankhimate_document::Document::from_loaded(reloaded);
    assert_eq!(
        ankhimate_formats::to_json(&doc.as_project_ref()).unwrap(),
        ankhimate_formats::to_json(&reloaded.as_project_ref()).unwrap()
    );
    let clip = doc.animations.values().next().unwrap();
    assert_eq!(clip.name, "curl");
    let mut pose = Pose::new();
    for t in [0.0, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0] {
        evaluate(&doc.skeleton, &[(clip, t, 1.0)], &mut pose);
        let error = (pose.world_tip(&doc.skeleton, *ik.bones.last().unwrap())
            - pose.world_position(ik.target))
        .length();
        println!("T816 tentacle t={t}: endpoint error={error:.6}");
        assert!(error < 0.01);
    }
    assert_eq!(std::fs::read(source).unwrap(), original);
}

#[test]
fn t816_chain_creation_orders_hierarchy_and_refuses_unrelated_selection() {
    let (mut s, chain, t, _) = fixture(4);
    s.session.selected_bones = chain.iter().rev().copied().collect();
    assert_eq!(selected_chain(&s), chain);
    s.session.selected_bones.push(t);
    assert!(selected_chain(&s).is_empty());
}
#[test]
fn t816_zero_length_and_unevaluated_span_are_unavailable() {
    for evaluated in [true, false] {
        let (mut s, chain, _, c) = fixture(3);
        if evaluated {
            s.doc.skeleton.bones[chain[2]].length = 0.0;
            s.refresh_pose();
        } else {
            s.pose = Pose::new();
        }
        let Constraint::Ik(ik) = &s.doc.skeleton.constraints[c] else {
            panic!()
        };
        assert!(ik_info::summary(&s, ik).is_err());
    }
}
