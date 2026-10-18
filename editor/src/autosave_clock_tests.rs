//! Controlled monotonic time; no sleeps or dependency on animation frames.
use super::*;
use std::time::{Duration, Instant};

#[test]
fn idle_dirty_content_is_due_on_the_deadline_without_intermediate_frames() {
    let now = Instant::now();
    let state = AppState::default();
    let mut autosave = Autosave::new(30);
    assert_eq!(
        autosave.schedule(now, &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(29), &state, true),
        Schedule::Wait(Duration::from_secs(1))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Due
    );
}

#[test]
fn clean_disabled_and_pending_recovery_need_no_timer_wakeup() {
    let now = Instant::now();
    let state = AppState::default();
    let mut autosave = Autosave::new(30);
    assert_eq!(autosave.schedule(now, &state, false), Schedule::Idle);
    autosave.interval_secs = 0;
    assert_eq!(autosave.schedule(now, &state, true), Schedule::Idle);
    autosave.interval_secs = 30;
    assert_eq!(
        autosave.schedule(now, &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(15), &state, false),
        Schedule::Idle
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(100), &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
}

#[test]
fn successful_copy_has_no_periodic_writes_and_next_edit_gets_a_full_interval() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("copy.ankh");
    let now = Instant::now();
    let mut state = AppState::default();
    let mut autosave = Autosave::new(30);
    autosave.schedule(now, &state, true);
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Due
    );
    let copy = autosave.write(&state, Some(&project)).unwrap();
    let bytes = std::fs::read(&copy).unwrap();
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Idle
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(300), &state, true),
        Schedule::Idle
    );
    state.doc.meta.name = "next edit".into();
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(301), &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(331), &state, true),
        Schedule::Due
    );
    assert_eq!(std::fs::read(&copy).unwrap(), bytes);
    autosave.write(&state, Some(&project)).unwrap();
    assert_eq!(ankhimate_formats::load(&copy).unwrap().0.name, "next edit");
    assert!(!project.exists());
}

#[test]
fn playback_and_sparse_frames_reach_the_same_deadline() {
    let state = AppState::default();
    let now = Instant::now();
    let mut sparse = Autosave::new(30);
    let mut playback = Autosave::new(30);
    sparse.schedule(now, &state, true);
    playback.schedule(now, &state, true);
    for frame in 1..1800 {
        assert!(matches!(
            playback.schedule(
                now + Duration::from_secs_f64(frame as f64 / 60.0),
                &state,
                true
            ),
            Schedule::Wait(_)
        ));
    }
    assert_eq!(
        sparse.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Due
    );
    assert_eq!(
        playback.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Due
    );
}

#[test]
fn failed_write_keeps_primary_and_retries_at_the_next_interval_without_spin() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("rig.ankh");
    let copy = path_for(Some(&project)).unwrap();
    std::fs::write(&project, b"primary remains").unwrap();
    std::fs::create_dir(&copy).unwrap();
    let state = AppState::default();
    let now = Instant::now();
    let mut autosave = Autosave::new(30);
    autosave.schedule(now, &state, true);
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Due
    );
    assert!(autosave.write(&state, Some(&project)).is_none());
    assert!(autosave.last_error.is_some());
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(30), &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(59), &state, true),
        Schedule::Wait(Duration::from_secs(1))
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(60), &state, true),
        Schedule::Due
    );
    assert_eq!(std::fs::read(project).unwrap(), b"primary remains");
    assert!(copy.is_dir());
}

#[test]
fn interval_changes_use_elapsed_monotonic_time_and_disable_resets_it() {
    let state = AppState::default();
    let now = Instant::now();
    let mut autosave = Autosave::new(120);
    autosave.schedule(now, &state, true);
    autosave.interval_secs = 30;
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(40), &state, true),
        Schedule::Due
    );
    autosave.interval_secs = 60;
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(50), &state, true),
        Schedule::Wait(Duration::from_secs(50))
    );
    autosave.interval_secs = 0;
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(51), &state, true),
        Schedule::Idle
    );
    autosave.interval_secs = 30;
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(100), &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
}

#[test]
fn missed_deadlines_do_not_catch_up_in_bursts_or_overflow_large_intervals() {
    let state = AppState::default();
    let now = Instant::now();
    let mut autosave = Autosave::new(30);
    autosave.schedule(now, &state, true);
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(1000), &state, true),
        Schedule::Due
    );
    assert_eq!(
        autosave.schedule(now + Duration::from_secs(1000), &state, true),
        Schedule::Wait(Duration::from_secs(30))
    );
    autosave.reset();
    autosave.interval_secs = u64::MAX;
    assert_eq!(
        autosave.schedule(now, &state, true),
        Schedule::Wait(Duration::from_secs(u64::MAX))
    );
}
