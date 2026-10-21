use ankhimate_core::{animation::Animation, ids::AnimationId};
use ankhimate_document::{
    Document, History, WorkMode,
    commands::{
        animation_order::{Direction, MoveAnimation},
        key_cmds::{CreateAnimation, DeleteAnimation, DuplicateAnimation, RenameAnimation},
    },
    snapshot::snapshot,
};
#[allow(dead_code, reason = "Shared rich native fixture")]
#[path = "support/t835_fixture.rs"]
mod fixture;
fn move_it(
    d: &mut Document,
    h: &mut History,
    id: AnimationId,
    dir: Direction,
) -> Result<bool, String> {
    let Some(c) = MoveAnimation::prepare(d, id, dir)? else {
        return Ok(false);
    };
    h.push(Box::new(c), d);
    Ok(true)
}
#[test]
fn t835_first_last_middle_and_successive_adjacent_swaps() {
    let (mut d, ids) = fixture::fixture();
    let mut h = History::default();
    let p = fixture::payloads(&d);
    for (id, dir, want) in [
        (ids[0], Direction::Down, vec![ids[1], ids[0], ids[2]]),
        (ids[2], Direction::Up, vec![ids[1], ids[2], ids[0]]),
        (ids[2], Direction::Up, vec![ids[2], ids[1], ids[0]]),
        (ids[1], Direction::Down, vec![ids[2], ids[0], ids[1]]),
    ] {
        assert!(move_it(&mut d, &mut h, id, dir).unwrap());
        assert_eq!(d.animation_ids(), want);
        assert_eq!(fixture::payloads(&d), p);
    }
    assert_eq!(h.undo_depth(), 4);
}
#[test]
fn t835_three_exact_snapshot_cycles() {
    let (mut d, ids) = fixture::fixture();
    let before = snapshot(&d);
    let mut h = History::default();
    move_it(&mut d, &mut h, ids[1], Direction::Up).unwrap();
    let after = snapshot(&d);
    for _ in 0..3 {
        assert!(h.undo(&mut d));
        assert!(snapshot(&d) == before);
        assert!(h.redo(&mut d));
        assert!(snapshot(&d) == after);
    }
    assert_eq!(h.undo_depth(), 1);
}
#[test]
fn t835_empty_singleton_boundaries_and_missing_preserve_redo() {
    let mut d = Document::new();
    let mut h = History::default();
    let mut temp = Document::new();
    let absent = temp.animations.insert(Animation::new("absent", 1.));
    assert!(move_it(&mut d, &mut h, absent, Direction::Up).is_err());
    let a = d.animations.insert(Animation::new("a", 1.));
    for dir in [Direction::Up, Direction::Down] {
        assert!(!move_it(&mut d, &mut h, a, dir).unwrap());
    }
    let b = d.animations.insert(Animation::new("b", 1.));
    move_it(&mut d, &mut h, b, Direction::Up).unwrap();
    h.undo(&mut d);
    let stale = d.animations.insert(Animation::new("removed", 1.));
    d.animations.remove(stale);
    let before = snapshot(&d);
    assert!(!move_it(&mut d, &mut h, a, Direction::Up).unwrap());
    assert!(!move_it(&mut d, &mut h, b, Direction::Down).unwrap());
    assert!(move_it(&mut d, &mut h, stale, Direction::Up).is_err());
    assert!(h.can_redo());
    assert_eq!(h.undo_depth(), 0);
    assert!(snapshot(&d) == before);
}
#[test]
fn t835_invalid_order_and_ambiguous_names_refuse_without_mutation() {
    let (mut d, ids) = fixture::fixture();
    let mut h = History::default();
    d.animation_order = vec![ids[0], ids[0]];
    let before = snapshot(&d);
    assert!(move_it(&mut d, &mut h, ids[0], Direction::Down).is_err());
    assert!(snapshot(&d) == before);
    d.animation_order.clear();
    d.animations[ids[1]].name = d.animations[ids[0]].name.clone();
    assert!(move_it(&mut d, &mut h, ids[1], Direction::Up).is_err());
    assert_eq!(h.undo_depth(), 0);
}
#[test]
fn t835_metadata_modes_are_both_allowed() {
    for mode in [WorkMode::Setup, WorkMode::Animate] {
        let (mut d, ids) = fixture::fixture();
        let c = MoveAnimation::prepare(&d, ids[1], Direction::Up)
            .unwrap()
            .unwrap();
        assert!(History::default().push_in_mode(Box::new(c), &mut d, mode));
        assert_eq!(d.animation_ids()[0], ids[1]);
    }
}
#[test]
fn t835_rename_delete_undo_restores_ids_and_earlier_order_command() {
    let (mut d, ids) = fixture::fixture();
    let before = snapshot(&d);
    let mut h = History::default();
    move_it(&mut d, &mut h, ids[1], Direction::Up).unwrap();
    h.push(Box::new(RenameAnimation::new(ids[1], "Royal")), &mut d);
    h.push(Box::new(DeleteAnimation::new(ids[1])), &mut d);
    let after = snapshot(&d);
    for _ in 0..3 {
        for _ in 0..3 {
            assert!(h.undo(&mut d));
        }
        assert!(snapshot(&d) == before);
        for _ in 0..3 {
            assert!(h.redo(&mut d));
        }
        assert!(snapshot(&d) == after);
    }
}
#[test]
fn t835_duplicate_then_move_keeps_identity_across_three_history_cycles() {
    let (mut d, ids) = fixture::fixture();
    let before = snapshot(&d);
    let mut h = History::default();
    h.push(Box::new(DuplicateAnimation::new(ids[1])), &mut d);
    let copy = *d.animation_ids().last().unwrap();
    move_it(&mut d, &mut h, copy, Direction::Up).unwrap();
    let after = snapshot(&d);
    for _ in 0..3 {
        h.undo(&mut d);
        h.undo(&mut d);
        assert!(snapshot(&d) == before);
        h.redo(&mut d);
        h.redo(&mut d);
        assert!(snapshot(&d) == after);
        assert!(d.animations.contains_key(copy));
    }
}
#[test]
fn t835_create_then_move_and_reused_slot_append() {
    let (mut d, ids) = fixture::fixture();
    d.animations.remove(ids[0]);
    let before = snapshot(&d);
    let mut h = History::default();
    h.push(Box::new(CreateAnimation::new("new", 2.)), &mut d);
    let new = *d.animation_ids().last().unwrap();
    assert_eq!(d.animations[new].name, "new");
    move_it(&mut d, &mut h, new, Direction::Up).unwrap();
    let after = snapshot(&d);
    for _ in 0..3 {
        h.undo(&mut d);
        h.undo(&mut d);
        assert!(snapshot(&d) == before);
        h.redo(&mut d);
        h.redo(&mut d);
        assert!(snapshot(&d) == after);
    }
}
#[test]
fn t835_all_author_profiles_preserve_order_and_complete_clip_payloads() {
    let (mut d, ids) = fixture::fixture();
    let mut h = History::default();
    move_it(&mut d, &mut h, ids[1], Direction::Up).unwrap();
    let expected = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    let tmp = std::env::temp_dir().join(format!(
        "ankhimate-t835-profiles-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&tmp).unwrap();
    for ext in ["ankh", "ankh.json", "ankh.min.json"] {
        let path = tmp.join(format!("Ordre-鍵.{ext}"));
        ankhimate_formats::save(&path, &d.as_project_ref(), &[]).unwrap();
        let (loaded, _) = ankhimate_formats::load(&path).unwrap();
        let mut reopened = Document::from_loaded(loaded);
        let actual = ankhimate_formats::convert::to_schema(&reopened.as_project_ref());
        assert_eq!(
            serde_json::to_value(&actual).unwrap(),
            serde_json::to_value(&expected).unwrap()
        );
        let order = fixture::names(&reopened);
        let id = reopened.animation_ids()[1];
        let mut rh = History::default();
        move_it(&mut reopened, &mut rh, id, Direction::Up).unwrap();
        rh.undo(&mut reopened);
        assert_eq!(fixture::names(&reopened), order);
        rh.redo(&mut reopened);
        ankhimate_formats::save(&path, &reopened.as_project_ref(), &[]).unwrap();
        let (again, _) = ankhimate_formats::load(&path).unwrap();
        assert_eq!(
            fixture::names(&Document::from_loaded(again)),
            fixture::names(&reopened)
        );
    }
    assert!(tmp.starts_with(std::env::temp_dir()));
    std::fs::remove_dir_all(&tmp).unwrap();
}
#[test]
fn t835_native_export_preserves_list_order_and_path_refusal() {
    let (mut d, ids) = fixture::fixture();
    move_it(&mut d, &mut History::default(), ids[1], Direction::Up).unwrap();
    let schema = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    assert!(ankhimate_export::run::plan(&schema, &d.assets, &preset).is_err());
    for a in d.animations.values_mut() {
        a.timelines
            .retain(|t| !matches!(t, ankhimate_core::animation::Timeline::PathPosition { .. }));
    }
    let project = ankhimate_formats::convert::to_schema(&d.as_project_ref());
    let plan = ankhimate_export::run::plan(&project, &d.assets, &preset).unwrap();
    let native: serde_json::Value = serde_json::from_str(
        &plan
            .files
            .iter()
            .find(|f| f.path.ends_with(".json"))
            .unwrap()
            .contents,
    )
    .unwrap();
    assert_eq!(native["format_version"], 2);
    // JSON object storage may canonicalize keys; template order is witnessed by
    // the unchanged context array, rather than serde_json object enumeration.
    let context = ankhimate_export::context::Context::build(
        &project,
        None,
        ankhimate_export::context::ExportInfo {
            output_dir: "".into(),
            preset_name: "test".into(),
            template_name: "test".into(),
            atlas_stem: "atlas".into(),
        },
    );
    assert_eq!(
        context
            .animations
            .iter()
            .map(|v| v["name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        fixture::names(&d)
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn t835_real_v2_payloads_and_all_nonorder_native_data_are_identical() {
    let (mut d, ids) = fixture::fixture();
    let original = d.clone();
    let preset = ankhimate_export::presets::builtin()
        .into_iter()
        .find(|p| p.name.contains("Ankhimate"))
        .unwrap();
    let native = |d: &Document| {
        let mut export = d.clone();
        for a in export.animations.values_mut() {
            a.timelines
                .retain(|t| !matches!(t, ankhimate_core::animation::Timeline::PathPosition { .. }));
        }
        let p = ankhimate_export::run::plan(
            &ankhimate_formats::convert::to_schema(&export.as_project_ref()),
            &export.assets,
            &preset,
        )
        .unwrap();
        let json = serde_json::from_str::<serde_json::Value>(
            &p.files
                .iter()
                .find(|f| f.path.ends_with(".json"))
                .unwrap()
                .contents,
        )
        .unwrap();
        (json, p.binaries)
    };
    let before = native(&d);
    move_it(&mut d, &mut History::default(), ids[1], Direction::Up).unwrap();
    assert_eq!(native(&d), before);
    let mut reset = d.clone();
    reset.animation_order = original.animation_order.clone();
    assert!(snapshot(&reset) == snapshot(&original));
}
