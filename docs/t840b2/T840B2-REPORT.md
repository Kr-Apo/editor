# T840B2 — Deterministic replay clock

Date: 2026-10-23. Verdict: **GO for B2 review only**. Reusable production clock
implemented in both cores; no current simulation/transport consumes it.

## Exact starting references

Verified clean local checkouts `work/t839/editor` and `work/t839/runtime` on
`dev/t840b1-physics-context-safety`:

- Editor `11dc5f32730dd69538f07156f73272883d12f756`.
- Runtime `21919a6a0f45616b6082519fe0aba6aed15b924d` (unchanged during B1).
- Origins `https://github.com/Kr-Apo/editor.git` and
  `https://github.com/Kr-Apo/runtime.git`.

Historical refs were compared to B1's receipts; T839/T840A fixtures/tests and
the B1 AppState correction are present. Both new branches are
`dev/t840b2-replay-clock`. Old branches/tags/configs are preserved. Final local
commit receipts accompany the external delivery copy of this report.

## Architecture and changed files

The clock has explicit initialization and monotonically increasing caller-chosen
epochs; absolute tick/rational requests; a budget-eight interactive path;
explicit bounded pump; observation-only unchanged time; and success-only tick
commit. All integer counters are u64-domain values, with u128 arithmetic in Rust
and bounded BigInt in TS. No dependency was added. No schema derives or implicit
clock JSON transport exist.

Product files:

- Editor `core/src/replay_clock.rs`, exported by `core/src/lib.rs`.
- Runtime `core/src/replay-clock.ts`, exported by `core/src/index.ts`.

Tests and new data:

- Editor `core/src/replay_clock_tests.rs` (7 unit tests).
- Editor `export/tests/t840b2_clock.rs` (2 tests: hitch and common vectors).
  This harness uses the export crate's existing test JSON facilities, keeping
  JSON/I/O out of the production core and avoiding a new dependency.
- Runtime `core/test/replay-clock.test.ts` (7 new tests).
- Identical new `clock-vectors.json` files in Editor
  `export/tests/fixtures/t840b2/` and Runtime `core/test/fixtures/t840b2/`.
- Editor's four documents in `docs/t840b2/`; Runtime's contract copy and local
  README in `docs/t840b2/`.

The new API distinguishes LegacyT839 and future ReplayV1. It has no implicit
activation path in PhysicsState, evaluate, AppState, RigPlayer or either graphics
adapter. No current callback is replaced. Existing callers remain on their
historical semantics; B1 is unchanged. Future integrations must use one clock
per whole instance, not one independent clock consumption per constraint.

## RED → GREEN and exact comparison

Before implementation, the new hitch/observation tests could not compile:
Rust reported missing `ankhimate_core::replay_clock` (exit 101), and TS reported
missing `ReplayClock` export (exit 2, with a consequent untyped callback error).
These are **missing-API RED proofs**, not claims of executed failing test
assertions. Both original logs are retained. Once implemented, the hitch test
and all added clock tests passed.

Targeted results: **7 Rust core + 2 integration tests**, **7 new TS tests**.
The common-vector integration executes **21 scenarios / 461 operations** in
each language against an independent exact Fraction oracle. Result trees match
exactly, including every tick index, counter, rational time, attempt, stop and
error code. At 30/60/120/144 Hz all four completed orders are precisely 1..120.
There is no floating tolerance. See the cross-language results document for
fixture/result hashes and representative measured transitions.

Tests cover hitch backlog, explicit partial/full pumping, positive requests
during debt, exact same-time observation, cancellation, consumer failure/retry,
epoch reset/stale request, backwards sub-tick time, invalid denominators and
budgets, result overflow, large rational products/counters, and independent
interleaved clocks. JS-specific invalid values, reentrancy and thrown/async
callbacks are tested. Rust privately injects the last-counter boundary to test
completion at u64::MAX in bounded time; no restore/injection API is shipped.

## Final gates actually executed

| Gate | Result |
|---|---|
| Rust workspace `--locked --all-targets --offline -- --test-threads=4` | **2,189 passed, 0 failed, 4 ignored** |
| `cargo fmt --all -- --check` | Pass |
| Clippy workspace, locked/all-targets/offline | Pass |
| Workspace build, locked/offline | Pass |
| WASM core check, locked/offline | Pass |
| `xtask docs-check`, locked/offline | Pass; final documents checked separately |
| Runtime build / typecheck | Pass |
| Runtime package/fixture tests | **238 passed**: core 50, Phaser 69, Pixi 113, pixel-policy 6 |
| Runtime source syntax | Pass |
| T840A additional characterization | **11 passed** |

This is a fresh run, not relabeled B1 results. Rust adds 9 tests to B1's 2,180;
Runtime adds 7 to the 231 historical tests. Historical native animation-parity
fixtures also pass in the Runtime suite (128 cases / 3,328 components; maximum
absolute difference approximately 0.000006438). Those are existing references,
not newly generated Physics replay comparisons.

Warning-message multisets are compared to accepted B1: 8 Rust test warning
lines, 34 Clippy lines and 4 build lines, including summaries; no new warning.
There were no unexpected functional failures in the targeted/final passes.
No WebGL/GUI suite was executed, so there is no new graphical PASS.

## Preserved behavior and evidence

No integrator equation, 1/120 Physics step, cap, remainder policy, seek/play/pause,
crossfade, blending, PathPosition, pure evaluation, file format, renderer,
profile default, version or dependency is modified. Only new core modules and
their exports are product changes. Historical source is checked byte-for-byte
through Git's changed-path inventory, with all other tracked blobs unchanged.

1,003 prior evidence/archive files are hash-checked, covering T839, T840A and B1
deliveries and protected RC2/v0.1.5 archives. Original D: checkouts, old T838
checkouts, historical refs, projects, assets and accepted fixtures remain intact.
RC2 remains `a1c9835fbe83468f6e87bdd5676965a45210c4913305fc18b72c9d426b9553c0`.

The **60 T840A physical traces, 29 within / 31 divergent**, remain untouched.
Their numerical matrix was not rerun; its divergences are not fixed or hidden.
B1's 13 replacement-safety tests remain part of the full Editor suite.

## Limits and next step

The clock commits only counters after consumer success. It does not roll back
partially mutated physics data; B6 owns that transaction work. An epoch identifies
time within one clock, not a global asset/context registry. Rust's borrow checker
and TS's busy guard protect synchronous reentrancy, not arbitrary concurrent
workers. TS exceptions and Rust panic unwinding have documented language-specific
behavior. Pump budget is 1..65,536, with unprocessed backlog retained.

The exact last rational time is retained to reject sub-tick regressions. Positive
sub-tick advancement can process old debt; repeating the same rational cannot.
No seconds-to-tick bridge is supplied. The experimental nanosecond bias from
T840A is not adopted because it differs from exact rational floor near a tick;
floating/rounded time needs separate qualification.

No anchors, real clips, stateful constraints, numerical replay records or current
transports have been integrated. B3 must establish the actual per-instance
state/constraint scheduling boundary. No B3 work was begun.

One local commit per repository; no push, merge, tag, release, npm publication,
Actions change, package/version bump or archive rebuild. Both work branches are
clean after commits. Stop for maintainer review.
