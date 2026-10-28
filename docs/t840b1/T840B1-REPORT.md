# T840B1 — Physics context safety

Date: 2026-10-23. Status: validated locally; no publication. Scope is replacement
safety in the Editor, not ReplayV1 or a new Physics fidelity qualification.

## Confirmed defect and correction

T840A's actual AppState test reproduced retained simulation across a document
replacement with recycled bone/constraint IDs. `replace_document` reseated the
session through `reset_rig_session`, but neither discarded `AppState.physics`.
The new rig could inherit offsets, velocities, anchors, remainder and pause.
`PhysicsState::reset()` alone is insufficient: it deliberately retains `paused`.

The successful rig-session handoff now assigns a fresh `PhysicsState::new()`.
This covers File New, successful native/foreign Open, recovery and successful
PSD Replace. It preserves the existing cancelled export worker until its staged
publication transaction unwinds. It does not alter the document or history.

The caller audit also found whole-rig exchanges inside native commands:
`ReplaceDocument` and `ImportPsd(replace=true)`, including their undo/redo paths.
They bypass the session reset when executed from history. A small, default-false
`EditCommand::replaces_rig()` query identifies only these exchanges; `Group`
propagates it. AppState clears Physics after successful dispatch, when recording
an already-applied exchange, and after successful undo/redo of such an exchange.
This avoids clearing UI sessions or adding an artificial history entry on undo.
PSD Merge, ordinary bone/key edits and empty/refused history operations retain
their previous behavior. No equality or continuity inference is made from IDs.

No integrator, time cap/remainder rule, pause/seek/clip policy, schema, renderer,
runtime, version, asset or export data changed. The command query is additive
with a default implementation; it is not a serialized field.

## Modified files

Product (four files, 43 added lines):

- `editor/src/app_state.rs`: fresh state at successful context boundaries.
- `document/src/commands/mod.rs`: context query and Group propagation.
- `document/src/commands/document_cmds.rs`: whole-document exchange annotation.
- `document/src/commands/psd_cmds.rs`: PSD Replace annotation; Merge stays false.

Regression coverage and registration:

- `editor/src/t840a_physics_tests.rs`: convert the accepted leakage
  characterization into the required fresh-state regression; preserve the other
  two T840A characterization tests unchanged.
- `editor/src/t840b1_physics_tests.rs`: state, continuation and command/history tests.
- `editor/src/app_t840b1_tests.rs`: real app lifecycle handoffs, without native dialogs.
- `editor/src/main.rs`, `editor/src/app.rs`: test-only module registration.

Documentation: the three files in `docs/t840b1/`. Complete staged patch, old/new
Git receipts, logs and integrity manifests are in the isolated delivery folder
`outputs/T840B1-Physics-Context-Safety` outside the repositories.

## RED → GREEN evidence

Before product changes, `cargo test --locked --offline -p ankhimate-editor
t840b1 -- --test-threads=4 --nocapture` compiled and ran nine tests: **8 failed,
1 passed**, exit 101. Retained rotation/x offset was **0.18385881** and y offset
**−0.22259165**, versus required zeros. Pause also survived replacement.
`RED.log`, `RED-tracked.patch` and two `RED-*.rs` files retain the exact proof.
The unchanged T840A branch preserves the original bug-characterization test.

After the fix, those nine tests passed. Four further boundary/non-regression
tests were added; **all 13 targeted tests pass** in `GREEN-final.log`.
This is **12 new tests plus one converted existing test**, not 13 new tests.
An intermediate test compilation attempt used an incorrect field name
(`translation` instead of native `position`); it was corrected in the test and
its log is retained as `GREEN.log`. This was not a product failure.

Executed checks include:

- Active and paused replacement, recycled IDs, no-Physics New and a distinct
  parented Physics rig. Offsets/velocities become exactly zero, old anchor absent,
  pause false. All evaluated local/world bone transforms match fresh AppState
  exactly across fractional and ordinary follow-up timesteps. Continuation
  probes also detect a retained private remainder without exposing a new API.
- Native command/group exchanges and pre-applied groups; three undo/redo cycles
  with full native document snapshots. No extra history entry; refused dispatch
  retains simulation and Redo. Normal bone edits retain Physics through history.
- Actual asynchronous Open success; missing, malformed, unsupported-version and
  stale-result failures. Cancel and failed Save preserve the pending action/live
  context; confirmed Discard/New resets it.
- Recovery success resets; corrupted recovery and Ignore retain the active
  paused context and recovery bytes. No recovery copy is silently deleted.
- Real tagged PSD decoding through the existing preview import seam: mode and
  decoding failures retain state; Replace resets, Merge retains. Three history
  cycles compare complete snapshots including assets and references.
- Save/reopen, foreign adoption, saved/dirty tracking, serialized author JSON
  unchanged by simulation, and another independent instance unaffected.

These are deterministic headless tests of production seams on Windows. Native
picker interactions and visual Physics tuning were **not run** in B1.

## Final validation actually executed

| Check | Result |
|---|---|
| Rust workspace `--locked --all-targets --offline -- --test-threads=4` | **2,180 passed, 0 failed, 4 ignored** |
| `cargo fmt --all -- --check` | Pass |
| Clippy workspace, locked/all-targets/offline | Pass; existing warnings unchanged |
| Workspace build, locked/offline | Pass |
| WASM core check, locked/offline | Pass |
| `xtask docs-check`, locked/offline | Pass; checked again after these documents |
| Runtime build/typecheck | Pass; no source change |
| Historical runtime tests | **231 passed**: core 43, Phaser 69, Pixi 113, pixel-policy 6 |
| T840A additional TS characterization | **11 passed** |
| Runtime source syntax / native fixture checks | Pass |

Warning-message multisets match the accepted T840A logs exactly: 8 test warning
lines, 34 Clippy warning lines and 4 build warning lines (including summaries).
No new warning was introduced or old warning fixed.

The full Rust run includes existing save/recovery, PSD, export refusal/staging,
cancelled export worker, rollback and native-target regressions. This does not
claim new visual or numerical Rust/TS parity. The historical **29 within / 31
divergent of 60 comparisons** remain T840A evidence; the comparison matrix was
not rerun. Its scripts, fixtures, thresholds and measurement files are preserved.
No graphical WebGL suite was rerun.

## Git and preservation

Initial Editor: clean `dev/t840a-physics-replay-contract` at
`011885b260e9477fbd4790e92b35f4ad590029c5`, descended from accepted T839
`ac8ace04cfb34f9ffe812fdde0bb8b9058f90303`.
Initial Runtime: clean same-named source branch at
`21919a6a0f45616b6082519fe0aba6aed15b924d`, descended from T839
`d720d6a21c0381e8417390bcac02f97c481432b5`.
Origins verified as `Kr-Apo/editor` and `Kr-Apo/runtime` respectively.

Both new branches are `dev/t840b1-physics-context-safety`. One local Editor
commit contains only this correction, tests and documents. Runtime has no new
commit and remains at its initial SHA. Final SHA/status receipts accompany the
external report; all old refs/tags/configs are compared to `INITIAL-GIT.json`.
Original D: and T838 source checkouts remain clean at their accepted HEADs.

965 protected evidence/archive files were hash-verified, including all T839 and
T840A delivery data. RC2 remains
`a1c9835fbe83468f6e87bdd5676965a45210c4913305fc18b72c9d426b9553c0`;
the v0.1.5 candidate remains
`3c8b2b75c4fcd2d2d480009b9c4ba25237b206988e5ae1eea498154b341e0b21`.
Historical projects, assets, backups and autosaves were not edited. No push,
merge, tag, publication, package/version or Actions change was performed.

## Limits and verdict

**GO for review of B1 only.** A successful context replacement starts fresh;
Undo restores authored data, not a previous running simulation. This is derived
state disposal, not a replay checkpoint implementation. Existing pause/seek,
mixing, cadence, numerical coupling, active-skin and TS differences are outside
this change. PSD Merge and other incremental structural edits keep their old
invalidation policy; broader revision/epoch invalidation remains future work.
New whole-rig command types must declare the context query, with tests. Direct
unsupported mutation bypassing AppState is not made safe by a global registry.

No new crash-durability guarantee is claimed. Existing save/export staging and
rollback guarantees and their documented multi-file crash limitations remain.
See the lifecycle matrix and B2 prerequisites; no B2 implementation was begun.
