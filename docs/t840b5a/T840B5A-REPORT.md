# T-840B5A — ReplayV1 Transport Lifecycle

Verdict: PASS for the explicitly bounded, headless author-only transport. STOP for
review before B5B. No graphical player, renderer, export or Legacy activation.

## Baseline and architecture

Exact clean B4 sources: editor 38c9f15d82671154e2102cfd6b16b27ac93c08fc;
runtime 3c4767175d25dc61198272da30fa940ff55d1f7c. Both isolated work/t839 checkouts
use new local branch dev/t840b5a-replay-transport, unchanged Kr-Apo/editor/runtime
origins. Initial refs/config hashes are recorded; old refs/configs remain unchanged.
Commit receipts are appended to the delivery copy after local commit.

New replay_transport.rs / replay-transport.ts own transport state and immutable input
snapshots around B4. No second advancing clock, animation interpolator or spring solver.
The only small B2 change shares its existing rational validator internally. New B3/B4
queue_time/queueTime delegates to B2's pre-work cancellation offer so a fresh historical
replay can be interrupted before tick one and pumped in explicitly bounded batches.
Existing request/pump behavior and equations remain unchanged.

Play, pause, resume, stop, canonical pure seek, origin replay, reset, terminal disposal
and explicit refusals are implemented in both cores. Current simulated poses correspond
only to completed ticks. Validation failures do not replace state; numerical/observer
failures preserve the last good public frame and require reset. B6 rollback is absent.

## RED/GREEN and tests

The initial RED established a missing API (unresolved Rust module / missing TS export),
not a bug in an existing player. That test is green with the full .5s hitch/pause/debt
cycle. Tests cover 600-tick historical replay, arbitrary inter-tick interruption via
one-tick pumps, full-frame equality, malformed commands, epoch exhaustion, stale/foreign
tokens, independent instances, pure seek and failed sampler/Physics/observer paths.

Final Rust workspace: **2,206 passed, 4 ignored**, locked/all-targets/offline, four
threads. Six new Rust test entries include an opt-in performance test that is a no-op
without its explicit output variable. Runtime: **261 passed** (core 73, Phaser 69,
Pixi 113, policy 6) plus **11 characterization tests**. Format, Clippy, workspace
build, WASM core, runtime build/typecheck/syntax and docs-check pass.

Actual Rust warning messages/counts match B4: 8 test, 34 Clippy and 4 build warning
lines including summaries. No new product warning. Runtime checks were rerun after
requiring an explicit restart boolean for Play and adding malformed-argument tests.
Final documentation checks are rerun after these five reports are written.

Parity: **50 scenarios, 16,600 command snapshots, 28,850 positive ticks**, all within
unchanged B4 thresholds; transport states, exact times, counters and order match exactly.
Nine real author/native rigs, five cadences each, plus 130 shared lifecycle/refusal
operations. Multiple runs reproduce raw traces. B3 90 cases/9,810 frames and B4
45 cases/21,645 frames per stage rerun with byte-identical historical outputs/traces.
B2 461 shared clock operations are retained in the suites. Legacy's 60 comparisons
retain their 29 within / 31 divergent split and unchanged traces. Known Legacy
differences are not claimed fixed. GUI/WebGL: NOT_RUN.

## Performance and preservation

Same-rig B4/B5A controls measure 120/600/1,200 tick advances and replays, command cost,
pause/observation, pure pose reconstruction, defensive frame copies and epoch resets.
Five warmups, 50 measured runs, percentiles and memory snapshots are retained. The
transport adds little steady tick overhead on these five-bone fixtures; short TS
replay/reset costs expose defensive candidate copying. No new speedup or large-rig
latency budget is claimed. Detailed values and limitations are in the performance report.

1,816 old evidence/reference/archive files are hash-preserved. Original D: repositories,
projects, assets, archives, autosaves, old branches/remotes/tags and 207 graphical
references remain untouched. New technical fixtures are isolated. No package manifest,
version, .ankh/native v1/v2 schema, old reader or product UI changes. No public operation.

Changed groups: new core transport modules/exports; shared rational validator visibility;
new scheduling-only B3/B4 helpers; deterministic tests/common vectors and real native
fixtures; B5A-only Git fixture byte attributes; five source reports plus runtime
contract/README link. Complete patches and exact inventories accompany the delivery.

## Limits requiring explicit review

Seek is on the 120Hz canonical grid; exact rational requests are retained but sub-tick
preview is not implemented. Seek invalidates old physical continuation. Historical
replay is origin-zero and author-only. Nonzero rest-start-from-preview is explicitly
refused; play(restart=true) explicitly restarts at zero. Nonloop clips hold their last
author pose while further requested Physics ticks may settle; no automatic stop.

Fixed default skin, one compatible bone-transform clip and Physics-only constraints
remain B4 restrictions. Skins, crossfade/layers, events, external inputs, arbitrary
checkpoints, mixed constraints and graphical/export readers are not qualified or enabled.
Synchronous work is interruptible between bounded calls, not preemptively during a tick.
The sampling horizon and B3 fault/last-frame guarantees remain; private numerical state
may have changed on failure, and reset is mandatory.

See [contract](T840B5A-TRANSPORT-CONTRACT.md), [parity](T840B5A-TRANSPORT-PARITY.md),
[performance](T840B5A-PERFORMANCE.md) and [next decisions](T840B5A-NEXT.md).
