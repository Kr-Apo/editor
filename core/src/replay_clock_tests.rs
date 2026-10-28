use super::*;
fn complete(_: ScheduledTick) -> TickResult<()> {
    TickResult::Complete
}

#[test]
fn profiles_are_explicit_and_legacy_is_still_default() {
    assert_eq!(SimulationProfile::default(), SimulationProfile::LegacyT839);
    assert_eq!(ReplayClock::new().profile(), SimulationProfile::ReplayV1);
}
#[test]
fn uninitialized_observation_and_reset_are_explicit() {
    let mut c = ReplayClock::new();
    let before = c.snapshot();
    assert_eq!(
        c.request_tick(1, 0, complete),
        Err(ClockError::Uninitialized)
    );
    assert_eq!(c.pump(1, 1, complete), Err(ClockError::Uninitialized));
    assert_eq!(c.snapshot(), before);
    c.reset(1).unwrap();
    let before = c.snapshot();
    for _ in 0..5 {
        assert_eq!(
            c.request_time(1, 0, 7, complete).unwrap().stop,
            Stop::Unchanged
        );
    }
    assert_eq!(c.snapshot(), before);
}
#[test]
fn invalid_inputs_and_stale_epoch_are_atomic() {
    let mut c = ReplayClock::new();
    c.reset(1).unwrap();
    c.request_tick(1, 60, complete).unwrap();
    let before = c.snapshot();
    assert_eq!(c.reset(0), Err(ClockError::InvalidEpoch));
    assert_eq!(c.reset(1), Err(ClockError::InvalidEpoch));
    assert_eq!(
        c.request_time(1, 1, 0, complete),
        Err(ClockError::InvalidTime)
    );
    assert_eq!(
        c.request_time(1, u64::MAX, 1, complete),
        Err(ClockError::Overflow)
    );
    assert_eq!(
        c.request_tick(1, 59, complete),
        Err(ClockError::BackwardTime)
    );
    assert_eq!(
        c.request_tick(2, 60, complete),
        Err(ClockError::InvalidEpoch)
    );
    assert_eq!(c.pump(1, 0, complete), Err(ClockError::InvalidBudget));
    assert_eq!(
        c.pump(1, MAX_BUDGET + 1, complete),
        Err(ClockError::InvalidBudget)
    );
    assert_eq!(c.snapshot(), before);
    c.reset(2).unwrap();
    assert_eq!(c.snapshot().backlog, 0);
    assert_eq!(c.pump(1, 1, complete), Err(ClockError::InvalidEpoch));
}
#[test]
fn partial_failure_cancel_and_retry_commit_only_completed_ticks() {
    let mut c = ReplayClock::new();
    c.reset(1).unwrap();
    let p = c
        .request_tick(1, 20, |t| {
            if t.index == 3 {
                TickResult::Failed("failure")
            } else {
                TickResult::Complete
            }
        })
        .unwrap();
    assert_eq!(
        (p.processed, p.attempted, p.state.completed, p.state.backlog),
        (2, 3, 2, 18)
    );
    assert_eq!(p.stop, Stop::Failed("failure"));
    assert_eq!(
        c.pump(1, 1, |_| TickResult::<()>::Cancelled)
            .unwrap()
            .processed,
        0
    );
    assert_eq!(c.snapshot().next, Some(3));
    let p = c.pump(1, 18, complete).unwrap();
    assert_eq!(p.stop, Stop::TargetReached);
    assert_eq!((p.state.completed, p.state.backlog), (20, 0));
}
#[test]
fn multiple_instances_on_one_input_are_independent() {
    let common_schedule = [(1, 2), (3, 4), (1, 1)];
    let mut a = ReplayClock::new();
    let mut b = ReplayClock::new();
    a.reset(1).unwrap();
    b.reset(1).unwrap();
    for (n, d) in common_schedule {
        a.request_time(1, n, d, complete).unwrap();
        b.request_time(1, n, d, complete).unwrap();
        assert_eq!(a.snapshot(), b.snapshot());
    }
    let before = b.snapshot();
    a.reset(2).unwrap();
    assert_eq!(b.snapshot(), before);
    b.pump(1, MAX_BUDGET, complete).unwrap();
    assert_eq!(b.snapshot().completed, 120);
    assert_eq!(a.snapshot().completed, 0);
}
#[test]
fn last_representable_tick_commits_without_counter_wrap() {
    // Private state injection only to reach the terminal boundary in bounded time.
    let mut c = ReplayClock {
        epoch: 1,
        requested: u64::MAX,
        completed: u64::MAX - 1,
        time: Some(RationalTime::checked(u64::MAX, 120).unwrap().0),
    };
    let p = c.pump(1, 8, complete).unwrap();
    assert_eq!(p.processed, 1);
    assert_eq!(p.state.completed, u64::MAX);
    assert_eq!(p.state.next, None);
    assert_eq!(c.pump(1, 8, complete).unwrap().processed, 0);
    c.reset(u64::MAX).unwrap();
    let before = c.snapshot();
    assert_eq!(c.reset(u64::MAX), Err(ClockError::InvalidEpoch));
    assert_eq!(c.snapshot(), before);
}
#[test]
fn callback_panic_never_commits_the_failed_tick() {
    let mut c = ReplayClock::new();
    c.reset(1).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        c.request_tick(1, 10, |t| {
            if t.index == 2 {
                panic!("consumer panic")
            };
            complete(t)
        })
    }));
    assert!(result.is_err());
    assert_eq!(c.snapshot().completed, 1);
    assert_eq!(c.pump(1, 9, complete).unwrap().state.completed, 10);
}
