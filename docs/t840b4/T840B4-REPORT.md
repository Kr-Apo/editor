# T-840B4 — Canonical Animation Sampling & Physics Precision

Verdict: PASS within the explicit single-clip, bone-transform, Physics-only numerical
scope. STOP for maintainer review. No B5–B8 implementation or reader activation.

## Git and preservation

Both isolated work/t839 checkouts started clean at the exact accepted B3 checkpoints:
editor 8c69c18977163b62da12b1e8385f31efad07647c and runtime
9c081e24348904af1a2c7772f9a4b396e105cde8. Both use new local branch
dev/t840b4-canonical-animation-sampling. Origins remain Kr-Apo/editor and Kr-Apo/runtime.
All pre-existing refs and config hashes are recorded in INITIAL-GIT.json and verified
after local commits. Final commit receipts are appended to the delivery copy after
commit, without trying to embed a commit's own SHA in its tree.

1,195 pre-existing evidence/reference/archive files were hash-checked. The original
D: repositories remain clean at their previous heads and remotes; historical projects,
assets, autosaves, 207 graphical references, RC2 and v0.1.5 archives are unchanged.
No push, fetch, merge, tag, release, version/npm/Actions change or packaging operation.

## Architecture and files

New Rust core/replay_animation.rs and TS core/replay-animation.ts own the immutable
clip/rig and delegate scheduling/simulation to B3. No standalone interpolation engine.
Rust pose.rs adds a small local-only sampling seam; replay_physics.rs only exposes
existing validation helpers within the crate. TS evaluate.ts extracts its unchanged
bone sampling loop; replay-physics.ts shares its existing rig validator internally.
Core index/lib expose the new opt-in API. Full diff and exact changed-file inventory
are supplied with the delivery.

Tests/fixtures: export/tests/t840b4_animation.rs, nine author .ankh.json fixtures,
core/test/replay-animation.test.ts and nine genuine native exports, plus
scripts/t840b4/compare.mjs, thresholds.json and performance.mjs. Five source reports
live in docs/t840b4; the runtime includes the sampling contract and a README link.

## RED / GREEN and validation

Before implementation, Rust failed on the absent replay_animation module and TS on
the missing ReplayAnimationInstance export (RED-rust.log / RED-typescript.log).
These are missing-capability RED tests, not a pre-existing Legacy solver bug.
The green tests exercise per-executed-tick clip sampling, canonical boundaries,
independence, reset/repeat, rejection and fault guards. Source snapshots preserve
the pre-optimization provider and the first 40-case parity result.

Final workspace: **2,200 passed, 4 ignored**, locked/all-targets/offline, four test
threads. Added five Rust test entries (one opt-in performance entry does no timing
without its explicit output environment variable). Runtime: **254 passed**
(core 66, Phaser 69, Pixi 113, policy 6), plus **11 characterization tests**.
Build, typecheck, syntax, Rust fmt/Clippy/workspace build/WASM core/docs-check pass.
No new product warnings: actual warning messages and counts match B3, 8 test,
34 Clippy and 4 build lines including generated-summary lines.

The initial sandboxed Rust workspace run failed on PermissionDenied in an unchanged
Unicode asset safe-save test; rerunning outside that temporary-directory restriction
passed. The first final Clippy pass identified a new test-only range-index warning;
it was corrected, then all Rust gates were rerun. Incremental hardlink warnings were
environment/cache messages; the authoritative final nonincremental run removes them
and matches the baseline messages exactly. Earlier logs are retained, not erased.

B4: **45 scenarios, 21,645 frames per stage**, all within unchanged thresholds.
B3: 90 scenarios/9,810 frames rerun and raw traces identical to accepted B3.
B2: 461 shared clock operations retained/exercised by full suites.
Legacy A: all 60 comparisons reproduced, retaining 29 within / 31 divergent rows;
Rust output files and raw TS traces are byte-identical to B3. Existing refusal,
document lifecycle, exports and golden-reference tests run in the workspace suite.
GUI/WebGL: **NOT_RUN**; no new rendering qualification is claimed.

## Findings and limits

The fractional duration's shortest JSON decimal needs f32 recovery before canonical
modulo. That correction affects only the new wrapper. No solver equation or original
interpolation algorithm changed. The local-only seam avoids discarding a redundant
full FK/slot pose, with reproducible bounded timing gains. Detailed stage timings,
raw traces and maxima are in the companion reports and delivery JSON.

Only seven bone channels from one clip are qualified. V1 readers remain intact but
the new runtime clip wrapper requires v2. Author bone offsets, attachment sequences,
events, slots, FFD, draw order, mixed constraints, skin transitions and transports
remain refused or explicitly unqualified as detailed in the sampling contract.
Sampling has a conservative 8,388,607-tick horizon. Public last-pose/fault guarantees
remain B3; complete private numerical rollback is deferred to B6. No arbitrary game
input history, clip mixing or gameplay event emission is synthesized.

See [sampling contract](T840B4-SAMPLING-CONTRACT.md),
[parity](T840B4-ANIMATION-PARITY.md), [performance](T840B4-PERFORMANCE.md),
and [next decision](T840B4-NEXT.md).
