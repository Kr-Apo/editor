//! Headless AppState characterization, not physical GUI qualification.
use crate::{app_state::AppState, session::WorkMode};
use ankhimate_core::{
    constraints::Constraint,
    ids::{BoneId, ConstraintId},
};
use ankhimate_document::Document;

fn document() -> Document {
    Document::from_loaded(
        ankhimate_formats::from_json(include_str!(
            "../../export/tests/fixtures/t840a/single.ankh.json"
        ))
        .unwrap(),
    )
}
fn state() -> AppState {
    let mut s = AppState::default();
    s.replace_document(document());
    s.set_work_mode(WorkMode::Animate);
    s.session.playing = true;
    s
}
fn ids(s: &AppState) -> (ConstraintId, BoneId) {
    let (id, c) = s.doc.skeleton.constraints.iter().next().unwrap();
    let Constraint::Physics(p) = c else { panic!() };
    (id, p.bone)
}
fn warmed() -> AppState {
    let mut s = state();
    for _ in 0..20 {
        s.advance_playback(1.0 / 60.0);
    }
    s
}

#[test]
fn t840a_editor_seek_displays_pure_pose_but_keeps_simulation_offsets() {
    let mut s = warmed();
    let (c, b) = ids(&s);
    let before = s.physics.offsets(c, b);
    assert_ne!(before.0, 0.0);
    s.set_playhead(0.0);
    assert_eq!(s.pose.locals[b].rotation, 0.0);
    assert_eq!(
        s.physics.offsets(c, b),
        before,
        "seek is not replay or reset"
    );
}
#[test]
fn t840b1_editor_replacement_starts_fresh_when_loaded_ids_are_reused() {
    let mut s = warmed();
    let (c, b) = ids(&s);
    s.replace_document(document());
    assert_eq!(ids(&s), (c, b));
    assert_eq!(
        s.physics.offsets(c, b),
        (0.0, glam::Vec2::ZERO),
        "replacement must start from rest"
    );
    s.set_work_mode(WorkMode::Animate);
    s.session.playing = true;
    s.advance_playback(1.0 / 60.0);
    let mut fresh = state();
    fresh.advance_playback(1.0 / 60.0);
    assert_eq!(
        s.pose.locals[b], fresh.pose.locals[b],
        "replacement must match a fresh AppState"
    );
}
#[test]
fn t840a_editor_pause_and_clip_change_keep_offsets_until_explicit_reset() {
    let mut s = warmed();
    let (c, b) = ids(&s);
    let before = s.physics.offsets(c, b);
    s.session.playing = false;
    s.refresh_pose();
    assert_eq!(s.pose.locals[b].rotation, 0.0);
    assert!(!s.advance_playback(0.5));
    assert_eq!(s.physics.offsets(c, b), before);
    let mut clip = s.doc.animations.values().next().unwrap().clone();
    clip.name = "other".into();
    let id = s.doc.animations.insert(clip);
    s.session.active_animation = Some(id);
    s.set_playhead(0.0);
    assert_eq!(s.physics.offsets(c, b), before);
    s.physics.reset();
    assert_eq!(s.physics.offsets(c, b), (0.0, glam::Vec2::ZERO));
}
