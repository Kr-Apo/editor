//! Cross-language clock vectors use existing test-only JSON facilities here.
//! The production core clock has no I/O or JSON dependency.
use ankhimate_core::replay_clock::{ReplayClock, TickResult};

#[test]
fn t840b2_common_exact_vectors() {
    use ankhimate_core::replay_clock::{ClockError, Stop};
    use serde_json::{Value, json};
    let input: Value =
        serde_json::from_str(include_str!("fixtures/t840b2/clock-vectors.json")).unwrap();
    let mut results = Vec::new();
    for case in input["cases"].as_array().unwrap() {
        let mut clock = ReplayClock::new();
        let mut rows = Vec::new();
        let mut entire_order = Vec::new();
        for op in case["operations"].as_array().unwrap() {
            let integer = |key: &str| op[key].as_str().unwrap().parse::<u64>().unwrap();
            let epoch = integer("epoch");
            let mut attempts = Vec::new();
            let mut order = Vec::new();
            let consume = |tick: ankhimate_core::replay_clock::ScheduledTick| {
                assert_eq!(tick.epoch, epoch);
                let index = tick.index.to_string();
                attempts.push(index.clone());
                if op["cancelAt"].as_str() == Some(&index) {
                    return TickResult::Cancelled;
                }
                if op["failAt"].as_str() == Some(&index) {
                    return TickResult::Failed("consumer");
                }
                order.push(index);
                TickResult::Complete
            };
            let before = clock.snapshot();
            let result = match op["op"].as_str().unwrap() {
                "reset" => clock.reset(epoch).map(|()| None),
                "tick" => clock
                    .request_tick(epoch, integer("tick"), consume)
                    .map(Some),
                "time" => clock
                    .request_time(epoch, integer("numerator"), integer("denominator"), consume)
                    .map(Some),
                "pump" => clock
                    .pump(
                        epoch,
                        op["budget"].as_u64().unwrap().try_into().unwrap(),
                        consume,
                    )
                    .map(Some),
                _ => panic!("unknown vector operation"),
            };
            let (processed, attempted, stop, error) = match result {
                Err(error) => {
                    assert_eq!(clock.snapshot(), before, "invalid input mutated state");
                    (0, 0, "error", Some(error))
                }
                Ok(None) => (0, 0, "reset", None),
                Ok(Some(p)) => (
                    p.processed,
                    p.attempted,
                    match p.stop {
                        Stop::TargetReached => "target_reached",
                        Stop::BudgetExhausted => "budget_exhausted",
                        Stop::Unchanged => "unchanged",
                        Stop::Cancelled => "cancelled",
                        Stop::Failed("consumer") => "failed",
                        Stop::Failed(_) => panic!("unexpected consumer error"),
                    },
                    None::<ClockError>,
                ),
            };
            let state = clock.snapshot();
            let row = json!({"state":{
                "initialized":state.initialized,"epoch":state.epoch.to_string(),
                "requested":state.requested.to_string(),"completed":state.completed.to_string(),
                "backlog":state.backlog.to_string(),"next":state.next.map(|n|n.to_string()),
                "time":state.time.map(|t|json!({"numerator":t.numerator.to_string(),"denominator":t.denominator.to_string()}))
            },"processed":processed,"attempted":attempted,"stop":stop,"error":error.map(|e|e.code()),"order":order,"attempts":attempts});
            assert_eq!(
                row,
                op["expected"],
                "case {} operation {}",
                case["name"],
                rows.len()
            );
            entire_order.extend(order);
            rows.push(row);
        }
        if case["name"].as_str().unwrap().starts_with("cadence-") {
            assert_eq!(
                entire_order,
                (1..=120).map(|i| i.to_string()).collect::<Vec<_>>()
            );
        }
        results.push(json!({"name":case["name"],"results":rows}));
    }
    if let Some(path) = std::env::var_os("T840B2_CLOCK_RESULTS") {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .unwrap();
        file.write_all(serde_json::to_string_pretty(&results).unwrap().as_bytes())
            .unwrap();
    }
}

#[test]
fn t840b2_hitch_is_bounded_and_unchanged_time_is_observational() {
    let mut clock = ReplayClock::new();
    clock.reset(1).unwrap();
    let mut ticks = Vec::new();
    let progress = clock
        .request_time(1, 1, 2, |tick| {
            ticks.push(tick.index);
            TickResult::<()>::Complete
        })
        .unwrap();
    assert_eq!(ticks, (1..=8).collect::<Vec<_>>());
    assert_eq!(progress.state.backlog, 52);
    let before = clock.snapshot();
    let observed = clock
        .request_time(1, 2, 4, |_| -> TickResult<()> {
            panic!("observation called consumer")
        })
        .unwrap();
    assert_eq!(observed.processed, 0);
    assert_eq!(clock.snapshot(), before);
}
