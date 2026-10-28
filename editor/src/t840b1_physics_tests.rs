//! Headless lifecycle regressions. No changes to the Physics integration policy.
use crate::{app_state::AppState, session::WorkMode};
use ankhimate_core::{
    constraints::Constraint,
    ids::{BoneId, ConstraintId},
    physics::PhysicsState,
};
use ankhimate_document::{
    Document,
    commands::{Group, document_cmds::ReplaceDocument},
};

pub(crate) fn document() -> Document {
    Document::from_loaded(
        ankhimate_formats::from_json(include_str!(
            "../../export/tests/fixtures/t840a/single.ankh.json"
        ))
        .unwrap(),
    )
}
pub(crate) fn ids(s: &AppState) -> (ConstraintId, BoneId) {
    let (c, constraint) = s.doc.skeleton.constraints.iter().next().unwrap();
    let Constraint::Physics(p) = constraint else {
        panic!()
    };
    (c, p.bone)
}
pub(crate) fn play(s: &mut AppState) {
    s.set_work_mode(WorkMode::Animate);
    s.session.playing = true;
}
pub(crate) fn warmed() -> AppState {
    let mut s = AppState::default();
    s.replace_document(document());
    play(&mut s);
    for _ in 0..20 {
        s.advance_playback(1.0 / 60.0);
    }
    // Deliberately leave a non-integral remainder and a conspicuously old anchor.
    s.advance_playback(0.003);
    let (c, b) = ids(&s);
    s.physics.set_anchor(c, b, glam::Vec2::new(700.0, -300.0));
    assert_ne!(s.physics.offsets(c, b).0, 0.0);
    assert_ne!(s.physics.velocity(c, b).0, 0.0);
    s
}
pub(crate) fn assert_fresh(s: &AppState, old: (ConstraintId, BoneId)) {
    assert!(!s.physics.paused, "a replacement must not inherit pause");
    assert_eq!(s.physics.offsets(old.0, old.1), (0.0, glam::Vec2::ZERO));
    assert_eq!(s.physics.velocity(old.0, old.1), (0.0, glam::Vec2::ZERO));
    assert_eq!(s.physics.last_anchor(old.0, old.1), None);
}
// Check continuation too: a zero first pose alone cannot expose leftover remainder.
pub(crate) fn assert_same_physics(s: &AppState, expected: &PhysicsState) {
    let (c, b) = ids(s);
    assert_eq!(s.physics.paused, expected.paused);
    assert_eq!(s.physics.offsets(c, b), expected.offsets(c, b));
    assert_eq!(s.physics.velocity(c, b), expected.velocity(c, b));
    assert_eq!(s.physics.last_anchor(c, b), expected.last_anchor(c, b));
    let mut a = s.physics.clone();
    let mut e = expected.clone();
    for dt in [0.002, 0.004, 0.009, 0.031, 1.0 / 144.0] {
        let mut pa = ankhimate_core::pose::Pose::new();
        let mut pe = ankhimate_core::pose::Pose::new();
        ankhimate_core::pose::evaluate_with(&s.doc.skeleton, &[], &mut a, dt, &mut pa);
        ankhimate_core::pose::evaluate_with(&s.doc.skeleton, &[], &mut e, dt, &mut pe);
        assert_eq!(pa.locals, pe.locals);
        assert_eq!(pa.worlds, pe.worlds);
    }
}

#[test]
fn t840b1_active_and_paused_replacements_match_fresh_full_poses_and_continuation() {
    for paused in [false, true] {
        let mut reused = warmed();
        let old = ids(&reused);
        reused.physics.paused = paused;
        let incoming = document();
        let before = ankhimate_document::snapshot::snapshot(&incoming);
        reused.replace_document(incoming);
        assert_fresh(&reused, old);
        assert!(before == ankhimate_document::snapshot::snapshot(&reused.doc));
        assert!(!reused.history.can_undo() && !reused.history.can_redo());
        let mut fresh = AppState::default();
        fresh.replace_document(document());
        play(&mut reused);
        play(&mut fresh);
        for dt in [0.002, 0.004, 0.009, 1.0 / 60.0, 0.03, 0.003, 0.02] {
            reused.advance_playback(dt);
            fresh.advance_playback(dt);
            // This fixture has no slots/deforms; compare every evaluated bone, not one.
            assert_eq!(reused.pose.locals, fresh.pose.locals);
            assert_eq!(reused.pose.worlds, fresh.pose.worlds);
            assert_eq!(reused.pose.draw_order, fresh.pose.draw_order);
            assert_same_physics(&reused, &fresh.physics);
        }
        assert!(before == ankhimate_document::snapshot::snapshot(&reused.doc));
    }
}

#[test]
fn t840b1_no_physics_new_and_rig_session_reset_clear_all_state_without_an_undo_step() {
    let mut s = warmed();
    let old = ids(&s);
    s.physics.paused = true;
    crate::fileops::new_document(&mut s);
    assert_fresh(&s, old);
    assert!(s.doc.skeleton.constraints.is_empty());
    s.replace_document(document());
    play(&mut s);
    s.advance_playback(0.1);
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let depth = s.history.undo_depth();
    let rev = s.revision;
    s.physics.paused = true;
    s.reset_rig_session();
    assert_fresh(&s, old);
    assert!(before == ankhimate_document::snapshot::snapshot(&s.doc));
    assert_eq!(s.history.undo_depth(), depth);
    assert_eq!(s.revision, rev);
}

#[test]
fn t840b1_native_undoable_replacements_and_groups_reset_only_on_success() {
    for grouped in [false, true] {
        let mut s = warmed();
        let old = ids(&s);
        let physics = s.physics.clone();
        let original = ankhimate_document::snapshot::snapshot(&s.doc);
        // Setup-only replacement refused in Animate: keep the live context.
        assert!(!s.dispatch(Box::new(ReplaceDocument::new(document()))));
        assert_same_physics(&s, &physics);
        s.set_work_mode(WorkMode::Setup);
        let mut incoming = document();
        incoming.meta.name = "replacement".into();
        let replacement = ankhimate_document::snapshot::snapshot(&incoming);
        let cmd: Box<dyn ankhimate_document::commands::EditCommand> =
            Box::new(ReplaceDocument::new(incoming));
        let cmd = if grouped {
            Box::new(Group::new(vec![cmd], "Replace group"))
                as Box<dyn ankhimate_document::commands::EditCommand>
        } else {
            cmd
        };
        assert!(s.dispatch(cmd));
        assert_fresh(&s, old);
        assert_eq!(s.history.undo_depth(), 1);
        for _ in 0..3 {
            // Simulate both contexts, including paused state, before traversing history.
            s.session.simulate_in_setup = true;
            s.advance_playback(0.05);
            s.physics.paused = true;
            s.undo();
            assert_fresh(&s, old);
            assert!(original == ankhimate_document::snapshot::snapshot(&s.doc));
            assert_eq!(s.history.undo_depth(), 0);
            assert!(s.history.can_redo());
            s.advance_playback(0.05);
            s.physics.paused = true;
            s.redo();
            assert_fresh(&s, old);
            assert!(replacement == ankhimate_document::snapshot::snapshot(&s.doc));
            assert_eq!(s.history.undo_depth(), 1);
        }
    }
}

#[test]
fn t840b1_simulation_is_unsaved_derived_state_and_instances_are_independent() {
    let mut a = warmed();
    let mut b = warmed();
    let retained = b.physics.clone();
    let before = ankhimate_document::snapshot::snapshot(&b.doc);
    let json = ankhimate_formats::to_json(&b.doc.as_project_ref()).unwrap();
    a.replace_document(Document::new());
    assert_same_physics(&b, &retained);
    for _ in 0..30 {
        b.advance_playback(1.0 / 60.0);
    }
    assert!(before == ankhimate_document::snapshot::snapshot(&b.doc));
    assert_eq!(
        json,
        ankhimate_formats::to_json(&b.doc.as_project_ref()).unwrap()
    );
    assert_eq!(b.history.undo_depth(), 0);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("physics.ankh");
    assert!(matches!(
        crate::fileops::write_to(&b, &path),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let old = ids(&b);
    b.physics.paused = true;
    assert!(matches!(
        crate::fileops::open_path(&mut b, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    assert_fresh(&b, old);
}

#[test]
fn t840b1_normal_edits_and_empty_or_refused_history_do_not_reset_physics() {
    use ankhimate_document::commands::bone_cmds::SetBoneTransform;
    let mut s = warmed();
    let physics = s.physics.clone();
    s.undo();
    s.redo(); // Empty stacks are observational.
    assert_same_physics(&s, &physics);
    s.set_work_mode(WorkMode::Setup);
    let bone = s.doc.skeleton.update_order[0];
    let mut transform = s.doc.skeleton.bones[bone].local_transform;
    transform.position.x += 5.0;
    assert!(s.dispatch(Box::new(SetBoneTransform::new(bone, transform))));
    assert_same_physics(&s, &physics);
    s.undo();
    assert_same_physics(&s, &physics);
    assert!(s.history.can_redo());
    s.set_work_mode(WorkMode::Animate);
    assert!(!s.dispatch(Box::new(ReplaceDocument::new(Document::new()))));
    assert!(s.history.can_redo());
    assert_same_physics(&s, &physics);
    s.redo();
    assert_same_physics(&s, &physics);
}

#[test]
fn t840b1_preapplied_group_replacement_resets_context_without_reapplying() {
    use ankhimate_document::commands::EditCommand;
    let mut s = warmed();
    let old = ids(&s);
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    let mut cmd = ReplaceDocument::new(Document::new());
    cmd.apply(&mut s.doc);
    s.dispatch_applied(Box::new(Group::new(
        vec![Box::new(cmd)],
        "Applied replacement",
    )));
    assert_fresh(&s, old);
    assert!(s.doc.skeleton.bones.is_empty());
    assert_eq!(s.history.undo_depth(), 1);
    s.undo();
    assert_fresh(&s, old);
    assert!(before == ankhimate_document::snapshot::snapshot(&s.doc));
    s.redo();
    assert_fresh(&s, old);
    assert!(s.doc.skeleton.bones.is_empty());
}

#[test]
fn t840b1_different_parented_rig_matches_new_instance_at_every_step() {
    let incoming = Document::from_loaded(
        ankhimate_formats::from_json(include_str!(
            "../../export/tests/fixtures/t840a/parent.ankh.json"
        ))
        .unwrap(),
    );
    let mut reused = warmed();
    let mut fresh = AppState::default();
    reused.physics.paused = true;
    reused.replace_document(incoming.clone());
    fresh.replace_document(incoming);
    play(&mut reused);
    play(&mut fresh);
    for dt in [0.001, 0.008, 0.015, 0.007, 0.1, 0.004, 1.0 / 144.0] {
        reused.advance_playback(dt);
        fresh.advance_playback(dt);
        assert_eq!(reused.pose.locals, fresh.pose.locals);
        assert_eq!(reused.pose.worlds, fresh.pose.worlds);
        assert_same_physics(&reused, &fresh.physics);
    }
}
