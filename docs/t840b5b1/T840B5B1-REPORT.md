# T-840B5B1 — Validation report

Implemented and qualified for maintainer review: opt-in immutable native skin
calendars in ReplayTransport -> ReplayAnimation -> ReplayPhysics, one B2 clock.
No adapter, editor UI, exporter, numerical solver equation, format or version change.

## Git baseline and preservation

Isolated copies work/t839/editor and work/t839/runtime; source branches
`dev/t840b5a-replay-transport`, exact accepted heads:
- Editor b711901504c3cd9881fdb6bd158c04db6b45c02a
- Runtime 4bb6c8469a524d3699a8c15cc54455a5b02b932b

New branch in both: `dev/t840b5b1-replay-skin-membership`. Originals on D:, earlier
branches/tags/remotes/configs, original projects/assets, 207 golden references,
RC2/v0.1.5 archives and earlier evidence are preserved. 1982 previously protected
files hash-verified. FINAL-GIT/LOCAL-COMMITS and complete patches are delivered.
No push, merge, tag, publication, Actions/version change or dependency addition.

## Audit and change rationale

See SKIN-CONTRACT for exact native default special case, attachment fallback and
bone-gating limits. Existing formats already represent the memberships; no migration
is required. A frozen calendar avoids an unsafe mutable input journal. Events apply
only at their executed historical tick, including pending backlog. Visual-only
changes keep compatible records; exits erase state; reentries prime zero state using
the parent AFTER earlier author-ordered constraints, then step once at activation.
Active mix zero continues numerics; parameter mutation/animation is not introduced.

Changes: new core replay_skin/replay-skin module, additive constructors and pure
skin queries in three existing controllers, module/type exports, five native author
fixtures and their genuine TS export copies, targeted suites, comparison/benchmark
scripts, scoped .gitattributes and documentation. Exact file inventory and full diffs
are delivered separately. Old fixed-default constructors remain the default.

## RED/GREEN and gates

RED-rust.log: missing module/API. RED-typescript-compile.log: missing withSkins.
This demonstrates missing opt-in capability, NOT a new Legacy defect. An earlier
raw strip-types attempt fails module resolution and is retained only as diagnostic,
not the RED proof. Both authoritative regressions compile and pass after addition.

- Rust workspace --locked --offline --all-targets: 2211 passed, 0 failed, 4 ignored.
- Five new Rust tests: campaign, lifecycle/errors, fresh activation, mix/invalid
  membership and opt-in benchmark (benchmark is a no-op absent its explicit env).
- Runtime full suites: 267 passed (core79, Phaser69, Pixi113, pixel-policy6), plus
  11 T-840A characterization tests. No browser/GPU campaign implied.
- fmt, Clippy, workspace build, WASM core: PASS.
- docs-check: initial new-report Windows encoding failure corrected to UTF-8;
  final rerun PASS (documentation checks passed), preserved in docs-final.log.
- Rust warning lines/messages unchanged: tests8, Clippy34, build4 including summaries;
  zero added/removed warnings. Runtime no new warnings.
- New parity: 60 scenarios,13210 command states,32400 positive ticks; thresholds
  unchanged, no failures. Repeat results/exports byte-identical.
- B3/B4/B5A genuine reference regeneration and comparisons PASS, raw traces unchanged.
  Legacy 31 divergences remain historical; detailed counts in PARITY/PRESERVATION.

No GUI, WebGL, Phaser/Pixi graphical qualification, inference or model download.
Complete historical full-suite headless tests ran; no existing graphic oracle updated.

## Measurements and limitations

See PERFORMANCE for balanced controls, environmental timing variance, boundary costs,
query/copy costs and limited memory observations. Small rigs only, no broad promise.
No skin layering, mutable schedule, mix-animation, new channels or mixed solvers.
B3/B4 legacy reset signatures still select fixed-default contexts; use a new opt-in
constructor for their calendar. Transport resets preserve its own frozen schedule.
No B6 rollback: a failed tick faults after possible private mutation, keeps last
committed frame and requires reset. No graphical/attachment qualification is implied.
B5B2 planning only is in NEXT; STOP after these local commits for review.
