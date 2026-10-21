use super::*;
use crate::commands::key_cmds::{AddKey, BoneProperty, KeyValue, TimelineAddr};
use crate::{edit::Edit, snapshot::snapshot};
use ankhimate_core::{
    animation::{Animation, Interp},
    skeleton::Bone,
};
#[test]
fn t828_native_fps_command_undo_redo_three_cycles() {
    let mut e = Edit::default();
    let before = snapshot(&e.doc);
    let command = SetProjectFps::new(&e.doc, 60).unwrap().unwrap();
    e.dispatch(Box::new(command)).unwrap();
    let after = snapshot(&e.doc);
    assert_eq!(e.doc.meta.fps, 60);
    for _ in 0..3 {
        assert!(e.undo());
        assert!(snapshot(&e.doc) == before);
        assert!(e.redo());
        assert!(snapshot(&e.doc) == after);
    }
}

#[test]
fn t828_bounds_and_refusals_preserve_redo_and_document() {
    let mut e = Edit::default();
    e.dispatch(Box::new(SetProjectFps::new(&e.doc, 60).unwrap().unwrap()))
        .unwrap();
    e.undo();
    let before = snapshot(&e.doc);
    for invalid in [0, 241, u32::MAX] {
        assert!(SetProjectFps::new(&e.doc, invalid).is_err());
        assert!(snapshot(&e.doc) == before);
        assert!(e.history.can_redo());
        assert_eq!(e.history.undo_depth(), 0);
    }
    assert!(SetProjectFps::new(&e.doc, 30).unwrap().is_none());
    assert!(e.history.can_redo());
    for value in [1, 240] {
        e.dispatch(Box::new(
            SetProjectFps::new(&e.doc, value).unwrap().unwrap(),
        ))
        .unwrap();
        assert_eq!(e.doc.meta.fps, value);
    }
    assert_eq!(e.history.undo_depth(), 2);
    e.undo();
    assert_eq!(e.doc.meta.fps, 1);
    e.undo();
    assert_eq!(e.doc.meta.fps, 30);
}

#[test]
fn t828_legacy_fps_is_restored_without_new_format_limits() {
    for original in [0, 1000, u32::MAX] {
        let mut e = Edit::default();
        e.doc.meta.fps = original;
        let before = snapshot(&e.doc);
        e.dispatch(Box::new(SetProjectFps::new(&e.doc, 60).unwrap().unwrap()))
            .unwrap();
        for _ in 0..3 {
            e.undo();
            assert!(snapshot(&e.doc) == before);
            e.redo();
            assert_eq!(e.doc.meta.fps, 60);
        }
    }
}

#[test]
fn t828_fps_interleaves_with_keys_without_merging_or_retiming() {
    let mut e = Edit::default();
    let bone = e.doc.skeleton.bones.insert(Bone {
        name: "b".into(),
        parent: None,
        length: 10.,
        local_transform: Default::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    e.doc.skeleton.rebuild_update_order();
    let anim = e.doc.animations.insert(Animation::new("clip", 2.));
    e.mode = WorkMode::Animate;
    let before = snapshot(&e.doc);
    e.dispatch(Box::new(SetProjectFps::new(&e.doc, 60).unwrap().unwrap()))
        .unwrap();
    let fps_only = snapshot(&e.doc);
    e.dispatch(Box::new(AddKey::new(
        anim,
        TimelineAddr::Bone {
            bone,
            property: BoneProperty::Rotate,
            axis: None,
        },
        0.137,
        KeyValue::Scalar(45.),
        Interp::Stepped,
    )))
    .unwrap();
    let keyed = snapshot(&e.doc);
    assert_eq!(e.history.undo_depth(), 2);
    for _ in 0..3 {
        e.undo();
        assert!(snapshot(&e.doc) == fps_only);
        e.undo();
        assert!(snapshot(&e.doc) == before);
        e.redo();
        assert!(snapshot(&e.doc) == fps_only);
        e.redo();
        assert!(snapshot(&e.doc) == keyed);
    }
}

#[test]
fn t828_capture_on_first_apply_and_both_modes() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        let mut e = Edit {
            mode,
            ..Default::default()
        };
        let cmd = SetProjectFps::new(&e.doc, 60).unwrap().unwrap();
        e.doc.meta.fps = 24; // Preparation is not the apply boundary.
        e.dispatch(Box::new(cmd)).unwrap();
        e.undo();
        assert_eq!(e.doc.meta.fps, 24);
        e.redo();
        assert_eq!(e.doc.meta.fps, 60);
    }
}
