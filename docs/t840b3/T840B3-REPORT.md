# T840B3 — Physics state and constraint scheduling

Date: 2026-10-23. Verdict: **GO for B3 review, qualified core surface only**.
No default consumer activation, graphics qualification or publication.

## Starting checkpoints

Both isolated checkouts were clean on `dev/t840b2-replay-clock`:

- Editor: `b3dff4f095f601fc28f9030b33f19643712c0f3d`.
- Runtime: `a8c934ace7948605548c9d00731e0d0c24f62531`.
- Origins: `https://github.com/Kr-Apo/editor.git` and
  `https://github.com/Kr-Apo/runtime.git`.

Historical refs, B1 disposal correction, B2 clocks and T839/T840A fixtures/tests
were verified. Both new branches are `dev/t840b3-physics-instance-scheduling`.
Final local commit/status receipts accompany the external delivery report.

## Implementation

Each explicit ReplayPhysicsInstance owns one private B2 clock, an immutable rig
snapshot, an instance-bound context token, separate numerical constraint records
and the last successfully completed frame. The constructor primes actual parent
anchors at tick zero without advancing. Each positive tick obtains its own
pre-Physics inputs, prepares a fresh pose and advances each active spring once
in author order, updating FK before dependents run. Records have no accumulator.

The qualified B3 surface is Physics-only, with fixed default-skin membership and
explicit local inputs. Mixed constraint contexts are rejected before simulation.
This avoids broadening the mission to a mixed-constraint evaluator/clip sampler.
No AppState, player, adapter, export or transport path is connected to the class.

The existing equations are shared through small elementary step helpers; Legacy
keeps its old step counting, remainders, caps, impulse division, activation and
nonfinite policy. Only the new path bypasses those accumulators. Pose/FK helpers
are reused. No schema, dependency or version changed.

Product changes:

- Editor `core/src/replay_physics.rs`, export in `core/src/lib.rs`.
- Editor `core/src/physics.rs`: shared elementary step, private-to-crate numerical
  record access; existing Legacy guard and scheduler retained.
- Editor `core/src/pose.rs`: shared offset application and crate-visible FK helper.
- Runtime `core/src/replay-physics.ts`, export in `core/src/index.ts`.
- Runtime `core/src/spring-step.ts`: shared elementary equations.
- Runtime `core/src/constraints.ts`: call the shared step and add internal
  Replay-only geometry/priming helpers; Legacy scheduling retained.

Tests/data/measurement changes:

- Editor `export/tests/t840b3_physics.rs` and new authored independent fixture
  `export/tests/fixtures/t840b3/independent.ankh.json`.
- Runtime `core/test/replay-physics.test.ts`, nine genuine native fixtures in
  `core/test/fixtures/t840b3/`, and `scripts/t840b3/{compare.mjs,thresholds.json}`.
- Four Editor documents in `docs/t840b3/`; Runtime contract copy and README.

## RED/GREEN and validation

The actual Legacy independent-spring test failed before changes: shared offsets
0.24232382/0.26845473 versus isolated 0.2465458. TS initially lacked the exported
controller. Exact RED sources/logs are archived. Final ReplayV1 independence
tests pass, with exact equality within each implementation and every intermediate
pose/record checked. See the parity document for the 90-scenario campaign and
predeclared thresholds: **9,810 frames pass Rust/TS comparison**.

Development errors are retained rather than hidden: one Rust test compilation
attempt used leading-dot float literals; another new analytic expectation used
world wind rather than parent-local wind for a rotated parent. The test inputs/
expectation were corrected, without changing the solver or widening thresholds.
Final trajectories include nonzero initial anchors, not only origin anchors.

| Fresh final gate | Result |
|---|---|
| Rust workspace locked/all-targets/offline, 4 test threads | **2,195 passed, 0 failed, 4 ignored** |
| Targeted Replay tests | **6 Rust, 9 TS passed** |
| Runtime complete packages/fixture checks | **247 passed**: core 59, Phaser 69, Pixi 113, pixel-policy 6 |
| T840A additional characterization | **11 passed** |
| Format, Clippy workspace, workspace build | Pass |
| WASM core, docs-check | Pass; final documents checked again |
| Runtime build, typecheck, syntax | Pass |
| Fresh Legacy numerical campaign | Same 29 within / 31 divergent; raw traces preserved |
| GUI / WebGL | **NOT_RUN** |

This adds six Rust tests and nine TS tests to B2. The optional descriptive Rust
timing test was actually run with its output path during targeted validation;
without that option the ordinary suite does no timing measurement. B1's 13
tests and B2's 461-operation vectors remain in the full fresh runs. Historical
warnings are compared to B2; no warning was fixed as unrelated cleanup.

## Descriptive CPU cost

50 measured runs after five warmups, 120 ticks/run, stationary inputs. Timings
include construction/priming, per-tick pose preparation and, for Replay, defensive
snapshots/observer calls. Rust uses the test/debug build; TS uses the local Node
runtime. These are local CPU observations, not release/GPU benchmarks or a claim
that the languages/builds are directly comparable.

| Engine / rig | Legacy median ms / 120 ticks | Replay median ms / 120 ticks |
|---|---:|---:|
| Rust / single spring | 0.2671 | 0.4740 |
| Rust / three independent springs | 0.5127 | 0.8388 |
| TypeScript / single spring | 0.1605 | 1.3183 |
| TypeScript / three independent springs | 0.2729 | 2.0444 |

Replay adds measurable cost (roughly 1.6–1.8x in this Rust probe, 7.5–8.2x in
this TS probe), notably input/pose rebuilding, finite checks and detached copies.
No timing acceptance threshold was imposed or changed. Large projects and real
consumer frame budgets remain unqualified; eight ticks bounds tick count, not
arbitrary provider execution time. Per-tick input/pose cost needs B4 attention.

## Error safety and limits

Context tokens bind to the instance, not a numeric epoch shared by another rig.
Old tokens fail after reset/recreate. A bad clock request cannot advance state.
Tick failure leaves its clock uncommitted and public frame at the last completed
prefix, marks the instance faulted and blocks continuation until explicit reset.
Other instances remain isolated. **Private records may be partially modified;
B6 rollback is not implemented.** The observer receives a candidate result before
clock commit and must not assume external side effects can be rolled back.

Dynamic topology/constraint/skin changes, real animation/crossfade sampling,
float-delta conversion and mixed-constraint scheduling are not qualified. There
is no clip/event/seek/play/pause policy and no graphical integration. The 31
Legacy divergences remain measured historical behavior, not repaired trajectories.

## Preservation and delivery

All 1,057 protected evidence/archive files are hash-checked. Historical refs,
original D: checkouts, old T838 checkouts, projects/assets/oracles and remotes
remain intact. RC2 stays
`a1c9835fbe83468f6e87bdd5676965a45210c4913305fc18b72c9d426b9553c0`.
No historical fixture or threshold was rewritten. Fresh Legacy traces are saved
in new directories; old delivery data remains untouched.

One local commit per repository, explicit file staging, clean new branches.
No push, merge, tag, package/version bump, publication, release, Actions change
or archive rebuild. Complete patches, traces, logs, timing data and preservation
receipts are in `outputs/T840B3-Physics-Instance-Scheduling` outside the repos.
Stop for maintainer review before B4; the next document is planning only.
