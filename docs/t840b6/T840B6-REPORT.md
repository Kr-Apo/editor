# T-840B6 — Numerical fault transactions and Physics rollback

## Verdict and scope

PASS for the numerical ReplayV1 core contract, pending maintainer review. No graphical
consumer activation, GUI/WebGL qualification, publication or version/format change.
Local source branches in both isolated T-839 repositories: `dev/t840b6-physics-transactions`.
Parents are editor `e8395b8e5da57778643274c2a2b0c1933194c1c5` and runtime
`fab7f7ca891cb5b57279a25554f762b466b6b7ad`. Original D: checkouts, old branches/tags,
Git configuration/remotes, assets, projects and accepted output directories are protected.
Final commit identifiers and preservation receipts accompany the delivery, outside
this source report (a commit cannot contain its own final hash).

## Root cause and change

B5B2 protected the public frame and completed clock tick but integrated directly into
private numerical records. When A succeeded and B rejected a singular parent, A's
rotation, velocities, position and anchor remained changed. Skin deletion/priming had
the same ownership problem. Observers could reject an already-mutated private state.

B6 computes on an owned candidate map and active vector, using fresh per-tick FK.
Only after integration, finite checks, frame creation and synchronous audit succeed
are records/membership/frame accepted in a callback-free block, immediately followed
by B2's Complete increment. Errors/exceptions/unwinds drop candidates; successful
prefix ticks and accepted temporal intention remain. No solver, clock, animation
sampler, format, Legacy policy or renderer is changed. `numerical_state()` /
`numericalState()` is a detached read-only inspection view, never restoration authority.

Production edits: editor `core/src/replay_physics.rs`; runtime
`core/src/replay-physics.ts`. Remaining edits are isolated B6 tests, genuine native
fixtures, reproduction/measurement scripts, scoped JSON byte attributes, documentation
and one README contract link. Full binary-capable diffs and explicit file/hash inventory
are in `editor-complete.diff`, `runtime-complete.diff`, `CHANGED-FILES.json`.

## RED then GREEN

`RED-rust.log` and `RED-typescript.log` show the real original numerical leak, after
adding only a read-only probe. A rotation changes from zero to about 0.00020362007 rad
and angular velocity to about 0.024434408 before B fails; the previously protected
frame and completed tick do not change. This is not a missing-API RED. Exact pre-fix
sources/hashes are retained in `BASELINE-PROVENANCE.json`, `baseline-*-source.txt` and
`BASELINE-SOURCE-RECOVERY.json`. Both recovered sources match the pre-measurement hashes.
The same regression is GREEN after candidate ownership, with exact private restoration.

## Executed quality and regression results

| Check | Actual result |
|---|---|
| Rust workspace, locked/offline/all-targets | 2,222 passed; 0 failed; 4 ignored (baseline 2,214) |
| New Rust tests | 3 core + 5 export; opt-in measurement/export helpers do no work without their environment flags |
| Runtime package/pixel-policy tests | 278 passed; 0 failed: core 90, Phaser 69, PixiJS 113, pixel policy 6 (baseline 273) |
| Separate historical TS characterizations | 11 passed |
| Rust fmt / Clippy / workspace build / WASM core / docs-check | PASS |
| Runtime build / typecheck / syntax | PASS; B6 nested scripts checked separately |
| New cross-language private rollback campaign | 33/33 PASS, five real native fixtures |
| Rust private injections | 84 returned-error/unwind cases, plus genuine finite-force overflow |
| TS private sidecar | 54/54 PASS; nested observer mutation and real integration failures also tested |
| Same-request committed-prefix test | 3 successful ticks retained, failing fourth, request 60, backlog 57 |
| Observable commit boundary | 100 reads during skin audit retain old committed pose/skin/records; new request intent visible; new pose and completed tick agree after return |
| Warning comparison | No added/removed Rust warning messages or counts: tests 8, Clippy 34, build 4; no new runtime warning |

Historic native exports and comparison campaigns re-executed: B3 90, B4 45, B5A 50,
B5B1 60, B5B2 18 scenarios. T-840A keeps its accepted 29 within-threshold / 31 Legacy
divergence results. B1 lifecycle, B2 clock and T-839 native tests run in full suites.
B5B2's 18 transport/consumer scenarios separately cover exact public health/epoch,
clock/state/skin, cancellation, audit fault and reset. B6 private injection does not
pretend to serialize every transport state itself. No historical tolerance is raised.

The B6 cross-language corpus checks 1,958 successful warm-prefix instance ticks and
23,760 reset/control instance ticks (33 x 360 x 2), total 25,718. Each reconstruction
is checked tick-by-tick against its fresh same-language control; only before, rejected
and final reconstructed snapshots are emitted for cross-language comparison.
Counts of scenarios/assertions are not additional unit-test counts.

## Evidence and remaining limits

See the five companion reports for ownership/boundary, fault matrix, parity,
performance and separately proposed graphical qualification. Gates and raw logs are
listed in `RUST-FINAL.json`, `RUNTIME-FINAL.json`, `RUNTIME-VERIFIED.json`.
First failures are retained: RED logs, the initial test-helper compilation correction
(`green-rust.log`, followed by `green-rust-retry.log`), missing-threshold and reused-output
comparison attempts, followed by successful fresh `parity-green` and `b6-typescript`.
These failed attempts are not counted as PASS runs or silently removed.

A fault still requires explicit reset/reconstruction. No intermediate retry, durable
checkpoint, async mid-tick interruption or external observer side-effect rollback.
Rust unwind is covered, abort/OOM/process kill is not. TS synchronous exceptions and
detached nested arrays are covered; fatal VM failure and hostile globals are not.
Per-tick candidate allocation has a measured cost; no full rig clone is added per tick.
Memory snapshots are not allocator totals or a leak/GC certification. No GUI/GPU,
PixiJS/Phaser ReplayV1 activation, authoring changes or new compatibility claim.
STOP after these local B6 checkpoints for maintainer review.
