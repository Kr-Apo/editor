# T-840B5B2 — Consumer bridge validation report

Implemented opt-in exact headless consumer bridge, pending maintainer review.
No graphics integration, profile activation, dependency, format or version change.

## Baseline

Both isolated work/t839 checkouts were clean on dev/t840b5b1-replay-skin-membership:
Editor7473358ae9d3ecae23e22e92456e0403621e114d;
Runtime9a898d8aece70586b0437d128cfdce55522d73cc.
New branches: dev/t840b5b2-consumer-bridge. Origins remain Kr-Apo/editor and runtime.
Historical refs/configs preserved; no original D: repository mutation or upstream
history rewrite.2173 older protected evidence/project/archive files hash-checked.
No push, merge, tag, release, Actions/npm or package version operation.

## Architecture and scope

New replay_consumer/replay-consumer core modules own one existing transport and
produce only qualified bone locals/worlds and Physics records. Additional read-only
transport helpers distinguish committed simulation from pure display preview and
reuse canonical clip time. The bridge delegates exact progression/control with no
second clock, evaluator, solver, skin system or host delta accumulator.

currentFrame always uses completed tick for bones,records,skin,poseTime,clipTime.
Requested/backlog are labeled intent metadata. Pure seek has a distinct preview
type; it cannot masquerade as committed historical Physics. Presentation callbacks
read owned snapshots outside simulation; their errors do not fault/advance it.
An optional inherited precommit diagnostic observer can fault numerics under B3;
it is not a renderer. Reset/context/dispose and immutable schedule rules remain.

Host float policy: analysis demonstrates ambiguous tick boundaries and accumulated
float drift. Exact tick/rational APIs are shipped; no arbitrary epsilon or biased
nanosecond adapter is shipped. RigPlayer deltaSeconds remains unchanged. See
CONSUMER-CONTRACT for semantics and ADAPTER-READINESS for concrete integration map.
No complete RigPose/slot/FFD/clipping rendering is invented or implied.

## RED/GREEN and validation

RED-rust/RED-typescript demonstrate absence of the new consumer API (not Legacy bugs).
Initial green minimal hitch test passes; final campaign replaces it with genuine
multi-size native fixtures and shared vectors. A transient helper-import error in
targeted-rust.log is preserved, fixed in targeted-rust-green.log; it is not hidden.
Final warning audit, not this intermediate compiler diagnostic, defines the baseline.

- Rust workspace locked/offline/all-targets:2214 passed,4 ignored,0 failures.
- New Rust tests: consumer campaign, independent/owned/error semantics, opt-in bench.
- Runtime273 passed: core85,Phaser69,Pixi113,pixel-policy6, plus11 characterizations.
- fmt,Clippy,workspace build,WASM core,docs-check PASS.
- Warning lines/messages unchanged: tests8/Clippy34/build4 including summaries.
- New parity18 scenarios/3456 command snapshots/7044 positive per-tick comparisons,
  all within unchanged thresholds. Repeat/reference and historical receipts supplied.
- Full historical B1/B2 tests and461 clock operations retained; actual B3/B4/B5A/
  B5B1 regeneration/comparison and Legacy reference preservation are documented.
- Graphics/GUI/WebGL qualification NOT_RUN. No historical pixel/oracle update.

Changes are limited to new core bridge, read-only transport views, module/type
exports, three author/native fixture sizes, shared vectors, tests, scripts,
.gitattributes and documentation. Complete diff and exact allowlisted inventory
accompany this report. Commits/status are recorded in LOCAL-COMMITS/FINAL-GIT and
the delivery receipt; no broad refactor or integration of consumer code elsewhere.

## Performance and open limits

PERFORMANCE contains30 rows per language, warmup/sample/dispersion, several sizes,
B5B1 controls, scheduling/read/copy/combined Physics/multiple-owner/backlog costs,
and memory limits. Copy cost scales with bones; reuse one retained snapshot per
update, rather than cloning on every redraw. No safety reduction or general claim.
Rust initial late-size run drift and differing8-owner residency are explicitly noted.

Exact-only game time, immutable skin schedule, bone-only publication and no events/
constraints mixing/FFD/slot animation remain limitations. Cancel catch-up between
bounded pumps (budget1 for single-tick granularity); no asynchronous mid-tick abort.
B6 numerical rollback is NOT implemented: failed private mutations may exist, last
committed frame is retained, progression is blocked and reset/reconstruction required.
No GPU/CPU Physics/PathPosition refusal is lifted. NEXT is planning only. STOP here.

After the full suite, coherence assertions were strengthened without production changes: each published snapshot must exactly match the last successful precommit audit frame (or an independent origin-zero B4 reference). This catches a future pose mislabeled with the correct completed tick, even if both bridges made the same error. The targeted Rust suite and18-case comparison passed again; raw repeat traces are unchanged.
