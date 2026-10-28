# T840B2 — Exact cross-language results

The same new clock-vector bytes were executed independently against the Rust
production clock and the TypeScript production clock. An independent Python
`fractions.Fraction` oracle supplied expected states and orders. Both test
runners check those expectations before emitting their actual results; the
delivery then compares the two JSON result trees exactly. No float tolerance,
pose renderer, mock clock or legacy-clock replacement is involved.

Fixture copies: Editor `export/tests/fixtures/t840b2/clock-vectors.json` and
Runtime `core/test/fixtures/t840b2/clock-vectors.json`.

- **21 scenarios, 461 operations; all exact comparisons pass.**
- Fixture SHA-256:
  `d245d133ab0cdcf3750ca34a444da17ab9643ba4c4e6507d615e9cfd40d34f4b`.
- Canonically sorted result-tree SHA-256:
  `ddfcdf5208f14240cda6698f48853631415707911de27caaba08e41c61caee14`.

Each operation compares initialization, epoch, reduced exact time, requested,
completed, next tick, backlog, budget consumed, attempts, completed tick order,
stop reason and expected input-error code. Decimal strings are an explicit
test-only integer transport, not a change to native runtime JSON.

| Schedule/scenario | Observed result |
|---|---|
| 30 Hz exact rational frame times | Exactly ticks 1..120 in order |
| 60 Hz | Exactly ticks 1..120 in order |
| 120 Hz | Exactly ticks 1..120 in order |
| 144 Hz | Exactly ticks 1..120; repeated displayed tick on some frames is expected |
| 0.5-second hitch | 8 processed, requested 60, completed 8, backlog 52 |
| Repeat equivalent time 2/4 | Zero attempts, state unchanged, backlog still 52 |
| Explicit pump 7 | Completed 15, backlog 45 |
| Pump remaining 45 | Completed 60, backlog 0, target reached |
| Repeated hitches and new target during debt | Pending order retained; bounded interactive work, explicit drain |
| Positive sub-tick advance / exact backward sub-tick request | New logical time can process debt; true backward time rejected |
| Fail at tick 3 | Only 1,2 committed; 3 remains pending and is retried explicitly |
| Cancel before tick 4 work | Only 1,2,3 committed; pending suffix retained |
| Reset / stale epoch / repeated invalid reset | New epoch starts at zero; stale requests and invalid reset do not mutate |
| MAX tick target / large rational cross-products | Exact u64-domain counters, no number precision loss |
| Fractions just below, at and above 1/120 | Exact floor boundary, no epsilon adjustment |
| Invalid denominator, result overflow, backward time and budget | Error before mutation |

Additional language-level tests cover independent interleaved instances,
uninitialized reads, TS negative/nonfinite/non-BigInt input, callback reentrancy,
exceptions/invalid async returns, and Rust's terminal completed-counter boundary
(private state injection to test it without iterating 2^64 ticks).

Executable tests: `export/tests/t840b2_clock.rs`, Rust core
`replay_clock_tests.rs`, and `core/test/replay-clock.test.ts` in Runtime.
Set `T840B2_CLOCK_RESULTS` to a **new** file path to emit actual vector results;
writers refuse an existing file. The delivery contains both result files,
comparison receipt, oracle generator, inputs and logs.

These results qualify clock scheduling only. T840A's 60 physical comparisons
(29 within / 31 divergent) and thresholds are preserved, not replaced or
relabeled as passing. That physical matrix and graphical suites were not rerun.
