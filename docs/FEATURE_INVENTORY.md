# État local T837 — 22 octobre 2026

Ce checkout contient le correctif ciblé de persistance Recovery T837 sur T836 `1d3cc13bd5fda93c541dc8698918dedcdd146290`, avec documentation utilisateur et deux livraisons locales séparées. Aucun format, runtime, numéro de version ou artefact public n’est modifié. Les tests réellement exécutés et le commit final figurent dans la livraison T837. L’Alpha publique v0.2.0-alpha.1 et RC2 conservent leurs propres preuves.

Les **444 contrôles manuels historiques restent PENDING MANUAL REVIEW** ; T837 ajoute 15 contrôles distincts, eux aussi en attente. Consultez [le catalogue utilisateur](FEATURES.md), [le démarrage](GETTING-STARTED.md), [la compatibilité](COMPATIBILITY.md) et [la récupération](project-recovery.md). Les 101 IDs conservent 62 COMPLETE, 33 PARTIAL, 1 ABSENT, 3 SUPERSEDED, 2 DEFERRED. T208/T209/T211/T404/T502 restent PARTIAL, sans reclassement par effet d’une mission récente.

<details>
<summary>Référence antérieure conservée — état daté T836 ou avant</summary>

<details>
<summary>Historical T832–T835 checkpoint context (not current test results)</summary>

T-835 local addendum: **Animation Clip Reordering — implemented / PENDING_MANUAL / LOCAL_CANDIDATE**. [Manager guide](animation-manager.md). T-208 remains PARTIAL for its original frame-display and manager-selection mode criteria; frozen T-832 matrix/categories remain unchanged. 24 new automated tests; Windows qualification deferred.

T-834 local addendum: **Safe FFD Key Clipboard — COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** in its same-document/exact-owned-mesh scope. See [the contract](ffd-key-clipboard.md). T-209/T-404 remain PARTIAL. The T-832 101-ID matrix/categories, its 352 manual scenarios and the 28 T-833 controls stay unchanged; T-834 adds 37 pending controls. No GUI/inference/publication.

T-833 local addendum: selected-bone offsets are implemented; see [the contract](selected-bone-key-offset.md). T-211 and T-905 remain PARTIAL; the frozen T-832 101-ID matrix and original 352 manual scenarios stay unchanged. T-833 adds 28 pending controls. No GUI/inference/public operation.

T-832 reconciliation baseline (retained): **2026-10-21**. Product audited at T-831
`4e4d50b0aa5b7ac160b204540afa878f66fa4d61`, parent T-830
`12252e476b95f9d2ac250d5d0746c589c10269b3`. **T-832 changes documentation only.**

Public Alpha `v0.2.0-alpha.1` remains the exact T-811/T-812 artifact: editor
`9233b8e888c811e7c71300101f865a9e74b3f489`, runtime
`c4c38df269cfcb30f941221454b7202f40d3fa5b`. T-813–T-831 refinements are local
candidates, even though their internal version string remains 0.2.0-alpha.1.
No GUI/inference/public operation in T-832. No local Windows control is marked PASS.
Accepted **T-831** evidence is 2,058 Rust passes / 0 failures / 4 existing ignores;
**T-832 does not rerun that suite** and adds no tests or executable. Relative to
T-816/T-817's 1,596 passes, the historical registration increase is 462, not 462
new tests executed by this documentation mission.

Implementation, validation and availability are independent axes.
COMPLETE means the explicitly retained historical product scope is implemented;
PARTIAL means a concrete retained product condition remains; ABSENT means no
implementation path; SUPERSEDED is an explicit replaced/renumbered design;
DEFERRED is intentional postponement; UNKNOWN is insufficient evidence.
AUTOMATED/MANUAL/MIXED/PENDING_MANUAL/NOT_TESTED describe validation, not feature
presence. PUBLIC_ALPHA/LOCAL_CANDIDATE/SOURCE_ONLY/NOT_DISTRIBUTED/UNKNOWN identify
availability. A public base may have a local-only refinement: the row's availability
describes its latest stated scope and the Public base column records that distinction.
No lack of manual testing alone keeps an implemented scope PARTIAL.

| Status | T-817 | T-831 audited | Delta |
|---|---|---|---|
| COMPLETE | 58 | 62 | +4 |
| PARTIAL | 37 | 33 | -4 |
| ABSENT | 1 | 1 | +0 |
| SUPERSEDED | 3 | 3 | +0 |
| DEFERRED | 2 | 2 | +0 |
| UNKNOWN | 0 | 0 | +0 |
| Total | 101 | 101 | 0 |

</details>

## Population and evidence method

101 historical tasks below,3 original context IDs and 14 later mission records, plus28 separate refinement records:146 records total, not 146 independent features. Historical category changes are only4. Scopes and evidence overlap. Read [tasks](TASKS.md), [CSV](task-status-matrix.csv), [manual map](cumulative-manual-qa-map.md), [compatibility](format-release-compatibility.md) and [old contemporaneous journal](checkpoint-journal-before-T832.md). Original criteria remain in [the exact archive](TASKS-before-T817.md).

`PENDING_MANUAL` means automated implementation evidence exists but the stated local desktop campaign is not passed. A reference to T-831 tests is accepted historical evidence; T-832 runs documentation/integrity checks only. Confidence HIGH refers to a targeted source/criteria comparison, not every branch or every artistic input; MEDIUM inherits accepted bounded T-817 evidence with current path/source checks.

<a id="foundations"></a>
## Foundations

<a id="t-001"></a>
### T-001 — License + repo hygiene

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Dual MIT/Apache licensing, attribution and repository/toolchain setup.
- Remaining condition / qualification or scope limit: Not a blanket audit of all transitive dependency licenses
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`LICENSE-MIT`](../LICENSE-MIT); [`LICENSE-APACHE`](../LICENSE-APACHE); [`Cargo.toml`](../Cargo.toml).
- Tests: [`Cargo.toml`](../Cargo.toml).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-002"></a>
### T-002 — CI skeleton

**PARTIAL / NOT_TESTED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Linux/Windows test/fmt/Clippy and core WASM workflow definitions.
- Remaining condition / qualification or scope limit: Actions disabled; no fresh remote green run; actual Clippy job lacks historical -D warnings
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml).
- Tests: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-003"></a>
### T-003 — ADR seed

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Accepted architectural decision records and source references.
- Remaining condition / qualification or scope limit: Some historical format statements are superseded by format-spec; not every old design sketch is current
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`docs/adr/0001-slotmap-ids.md`](../docs/adr/0001-slotmap-ids.md); [`docs/adr/0007-physics-determinism.md`](../docs/adr/0007-physics-determinism.md).
- Tests: [`docs/ARCHITECTURE_PLAN.md`](../docs/ARCHITECTURE_PLAN.md).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-101"></a>
### T-101 — Stable entity IDs

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Slotmap entity identity, topological order and native Undo ID remapping.
- Remaining condition / qualification or scope limit: Names cross disk boundaries; unresolved foreign references still need diagnostics
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/ids.rs`](../core/src/ids.rs); [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Tests: [`core/src/skeleton.rs`](../core/src/skeleton.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-102"></a>
### T-102 — Affine2 transform math

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native 2D affine composition and inherited transforms.
- Remaining condition / qualification or scope limit: Singular transforms remain noninvertible; no universal arbitrary-float geometry guarantee
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/transforms.rs`](../core/src/transforms.rs).
- Tests: [`core/src/transforms.rs`](../core/src/transforms.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-103"></a>
### T-103 — Pose extraction + evaluate skeleton

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Deterministic finite-input pose evaluation with derived worlds.
- Remaining condition / qualification or scope limit: Stateful physics requires its separate accumulator; direct seek is not a simulation
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/pose.rs`](../core/src/pose.rs); [`editor/src/app_state.rs`](../editor/src/app_state.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-104"></a>
### T-104 — Constraint system + correct IK blending

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Ordered IK, shortest-angle local mixing and 1/2/3+ solvers.
- Remaining condition / qualification or scope limit: Runtime common alpha excludes constraints; geometric singularities are not repaired
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/constraints.rs`](../core/src/constraints.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs); [`core/src/constraints.rs`](../core/src/constraints.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-105"></a>
### T-105 — Skin layer

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Named skins and default fallback resolution.
- Remaining condition / qualification or scope limit: Not arbitrary game-runtime skin composition or linked deformation qualification
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/skin.rs`](../core/src/skin.rs); [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Tests: [`core/src/skin.rs`](../core/src/skin.rs); [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-106"></a>
### T-106 — Animation model + sampling

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Typed scalar/slot/deform tracks, Linear/Stepped/Bezier for valid authored inputs.
- Remaining condition / qualification or scope limit: Extreme finite handles can overflow sampler; no universal f32-range guarantee
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/animation.rs`](../core/src/animation.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Tests: [`core/src/animation.rs`](../core/src/animation.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-107"></a>
### T-107 — Editor Document/Session split + command undo

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native document/history split; transport FPS now uses SetProjectFps.
- Remaining condition / qualification or scope limit: Undo history is not persisted; this closes only the identified FPS bypass, not every metadata editing surface.
- Evidence checkpoints: T-828.
- Code: [`document/src/doc.rs`](../document/src/doc.rs); [`document/src/commands/mod.rs`](../document/src/commands/mod.rs); [`editor/src/session.rs`](../editor/src/session.rs); [`document/src/commands/document_cmds.rs`](../document/src/commands/document_cmds.rs); [`editor/src/ui/timeline/transport/fps.rs`](../editor/src/ui/timeline/transport/fps.rs).
- Tests: [`document/src/commands/mod.rs`](../document/src/commands/mod.rs); [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`document/src/commands/document_cmds/t828_tests.rs`](../document/src/commands/document_cmds/t828_tests.rs); [`editor/src/app_fps_tests.rs`](../editor/src/app_fps_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Undo history is not persisted; this closes only the identified FPS bypass, not every metadata editing surface.
- Confidence: HIGH.

<a id="t-108"></a>
### T-108 — `formats` crate + .ankh v1 save/load

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Author v1 ANKH envelope, MessagePack and readable/compact JSON profiles.
- Remaining condition / qualification or scope limit: Prototype ZIP unsupported; no authentic v0 migration; complete scope is current v1 only
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`formats/src/lib.rs`](../formats/src/lib.rs); [`formats/src/container.rs`](../formats/src/container.rs); [`formats/src/convert.rs`](../formats/src/convert.rs).
- Tests: [`formats/tests/reliability.rs`](../formats/tests/reliability.rs); [`formats/tests/migrations.rs`](../formats/tests/migrations.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-707"></a>
### T-707 — Crash recovery, update check, feedback

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Safe save, sidecar recovery, candidate checks and recovery offers.
- Remaining condition / qualification or scope limit: No panic-time live-document dump, recovery browser or complete update/feedback package
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/autosave.rs`](../editor/src/autosave.rs); [`editor/src/app.rs`](../editor/src/app.rs); [`formats/src/safe_write.rs`](../formats/src/safe_write.rs).
- Tests: [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs); [`formats/tests/reliability.rs`](../formats/tests/reliability.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-813"></a>
### T-813 — Format Compatibility & Project Reliability

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: LOCAL_CANDIDATE. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Local unknown/lossy input and missing-byte save refusal; recovery hardening.
- Remaining condition / qualification or scope limit: No arbitrary structural repair, hostile-resource budget or power-loss certificate; GUI pending
- Evidence checkpoints: T-813.
- Code: [`formats/src/compatibility.rs`](../formats/src/compatibility.rs); [`formats/src/lib.rs`](../formats/src/lib.rs); [`editor/src/app.rs`](../editor/src/app.rs).
- Tests: [`formats/tests/reliability.rs`](../formats/tests/reliability.rs); [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="animation"></a>
## Animation

<a id="t-201"></a>
### T-201 — Dopesheet panel

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Original bone/slot scalar dopesheet, synced rows/ruler, selection and native key interactions..
- Remaining condition / qualification or scope limit: Complex track addresses belong to the separately partial T-209/FFD workflow; local T-814 selection refinements still await GUI review.
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/timeline/sheet.rs`](../editor/src/ui/timeline/sheet.rs); [`editor/src/ui/timeline/model.rs`](../editor/src/ui/timeline/model.rs).
- Tests: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs); [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-202"></a>
### T-202 — Playback + auto-key

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Playback/Auto-key/setup return preserved; native undoable transport FPS.
- Remaining condition / qualification or scope limit: Pending previews are not keys; Windows playback/focus qualification remains pending.
- Evidence checkpoints: T-828.
- Code: [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs); [`editor/src/edit_router.rs`](../editor/src/edit_router.rs); [`editor/src/ui/timeline/transport/fps.rs`](../editor/src/ui/timeline/transport/fps.rs).
- Tests: [`editor/src/edit_router.rs`](../editor/src/edit_router.rs); [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`editor/src/ui/timeline/transport/t828_tests.rs`](../editor/src/ui/timeline/transport/t828_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Pending previews are not keys; Windows playback/focus qualification remains pending.
- Confidence: HIGH.

<a id="t-203"></a>
### T-203 — Bezier interpolation UI

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Existing scalar Bezier editing/numerics plus local linked modes.
- Remaining condition / qualification or scope limit: T-704 scalar feature list now implemented locally; new property/FFD/normalized editing is not universal.
- Evidence checkpoints: T-820.
- Code: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs); [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs); [`document/src/tangent_linking.rs`](../document/src/tangent_linking.rs).
- Tests: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs); [`editor/src/ui/timeline/graph/t820_tests.rs`](../editor/src/ui/timeline/graph/t820_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. T-704 scalar feature list now implemented locally; new property/FFD/normalized editing is not universal.
- Confidence: HIGH.

<a id="t-204"></a>
### T-204 — Draw-order animation UI

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Setup draw order and animated offsets through dedicated native surface.
- Remaining condition / qualification or scope limit: Generic dopesheet order-key clipboard remains excluded
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/draw_order.rs`](../editor/src/ui/draw_order.rs); [`document/src/commands/slot_cmds.rs`](../document/src/commands/slot_cmds.rs).
- Tests: [`document/src/commands/slot_cmds.rs`](../document/src/commands/slot_cmds.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-205"></a>
### T-205 — Slot timelines UI

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Slot light RGBA, visibility and attachment switching.
- Remaining condition / qualification or scope limit: Static dark tint is separate; no animated-dark channel is invented
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/slot_editor.rs`](../editor/src/ui/slot_editor.rs); [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Tests: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-207"></a>
### T-207 — Setup mode / Animate mode

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native Setup/Animate structural gates, router and unkeyed previews.
- Remaining condition / qualification or scope limit: Every future edit surface still needs mode semantics; legacy metadata writes audited separately
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`document/src/work_mode.rs`](../document/src/work_mode.rs); [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Tests: [`document/src/lib.rs`](../document/src/lib.rs); [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-208"></a>
### T-208 — Animation manager

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Clip create/rename/duplicate/delete/duration/loop manager retained.
- Remaining condition / qualification or scope limit: T-835 adds persistent adjacent Move Up/Down with native Undo/Redo. Original manager duration-in-frames display and switching to Animate on manager-row selection remain unimplemented; no claim of full T-208 completion.
- Evidence checkpoints: .
- Code: [`editor/src/ui/animations.rs`](../editor/src/ui/animations.rs); [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs); [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs).
- Tests: [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs); [`editor/src/ui/animations.rs`](../editor/src/ui/animations.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. T-835 adds persistent adjacent Move Up/Down with native Undo/Redo. Original manager duration-in-frames display and switching to Animate on manager-row selection remain unimplemented; no claim of full T-208 completion.
- Confidence: HIGH.

<a id="t-209"></a>
### T-209 — Clipboard: copy / paste / duplicate

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: T-834 adds same-owner native FFD vector blocks with exact topology guard, explicit row, strict collisions and one Undo.  Existing owned key blocks/pose/mirror plus faster capture; separate reviewed region transfer.
- Remaining condition / qualification or scope limit: T-834 supports owned same-mesh FFD key blocks on an explicit existing row. Cross-project/cross-mesh, linked/shared/overridden sources, empty-row creation, constraints, draw-order and event blocks remain excluded. Scalar replacement semantics remain unchanged.
- Evidence checkpoints: T-834; T-819; T-821; T-827.
- Code: [`document/src/clipboard.rs`](../document/src/clipboard.rs); [`editor/src/key_clipboard.rs`](../editor/src/key_clipboard.rs); [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs); [`document/src/clipboard/keys.rs`](../document/src/clipboard/keys.rs); [`document/src/transfer.rs`](../document/src/transfer.rs).
- Tests: [`editor/src/ui/timeline/sheet/clipboard_tests.rs`](../editor/src/ui/timeline/sheet/clipboard_tests.rs); [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs); [`editor/src/ui/timeline/sheet/t821_tests.rs`](../editor/src/ui/timeline/sheet/t821_tests.rs); [`document/src/clipboard/keys/t827_tests.rs`](../document/src/clipboard/keys/t827_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. T-834 supports owned same-mesh FFD key blocks on an explicit existing row. Cross-project/cross-mesh, linked/shared/overridden sources, empty-row creation, constraints, draw-order and event blocks remain excluded. Scalar replacement semantics remain unchanged.
- Confidence: HIGH.


T-834 sources: [`typed native payload/plan`](../document/src/clipboard/deform.rs), [`native tests`](../document/tests/ffd_clipboard.rs), [`editor lease/UI`](../editor/src/ffd_clipboard.rs), [`actual egui tests`](../editor/src/ui/timeline/sheet/t834_tests.rs).

<a id="t-209-2"></a>
### T-209.2 — Validated normal/mirrored multibone pose paste in one native transaction

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Validated normal/mirrored multibone pose paste in one native transaction.
- Remaining condition / qualification or scope limit: Auto-key-off retains preview policy; not a retargeter
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs); [`document/src/commands/pose_cmds.rs`](../document/src/commands/pose_cmds.rs).
- Tests: [`editor/src/paste_tests/pose_tests.rs`](../editor/src/paste_tests/pose_tests.rs); [`editor/src/paste_tests/menu_tests.rs`](../editor/src/paste_tests/menu_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-209-3"></a>
### T-209.3 — Explicit bone-row mirrored offsets; signs

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Explicit bone-row mirrored offsets; signs, curves, overshoots and relative times preserved.
- Remaining condition / qualification or scope limit: Offsets, not asymmetric absolute setup poses; no slot/FFD/event mirror
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`document/src/clipboard/keys.rs`](../document/src/clipboard/keys.rs); [`editor/src/key_clipboard.rs`](../editor/src/key_clipboard.rs).
- Tests: [`editor/src/ui/timeline/sheet/mirrored_clipboard_tests.rs`](../editor/src/ui/timeline/sheet/mirrored_clipboard_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-210"></a>
### T-210 — Keyed-value affordances in the inspector

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Per-property key dots, setup/animated reverts and Setup resets.
- Remaining condition / qualification or scope limit: Does not add IK key dots or every future property; no whole-app accessibility certificate
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/inspector/key_controls.rs`](../editor/src/ui/inspector/key_controls.rs); [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Tests: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-211"></a>
### T-211 — Pose tools

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Bake/reset/clear and fixed retime retained; checked numeric whole-clip retime plus T-833 selected-bone transform offset.
- Remaining condition / qualification or scope limit: T-833 adds explicit selected-bone transform-key offset, separate from whole-clip Retime. K keys pending poses; no dedicated Shift+K key-all-changed binding identified. Arbitrary KeyRefs/constraint/slot offsets and selected time scaling remain out of scope.
- Evidence checkpoints: T-830, T-833.
- Code: [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs); [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs); [`editor/src/ui/retime.rs`](../editor/src/ui/retime.rs); [`document/src/commands/retime_numeric.rs`](../document/src/commands/retime_numeric.rs); [`editor/src/keymap.rs`](../editor/src/keymap.rs).
- Tests: [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs); [`editor/src/ui/retime/tests.rs`](../editor/src/ui/retime/tests.rs); [`document/src/commands/retime_numeric_tests.rs`](../document/src/commands/retime_numeric_tests.rs).
- T-833 source/tests: [`selected_offset.rs`](../document/src/commands/selected_offset.rs), [`native tests`](../document/src/commands/selected_offset_tests.rs), [`UI and lease`](../editor/src/ui/retime/selected.rs), [`egui tests`](../editor/src/ui/retime/selected/tests.rs), [`Save/export tests`](../editor/src/app_selected_offset_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. T-833 adds explicit selected-bone transform-key offset, separate from whole-clip Retime. K keys pending poses; no dedicated Shift+K key-all-changed binding identified. Arbitrary KeyRefs/constraint/slot offsets and selected time scaling remain out of scope.
- Confidence: HIGH.

<a id="t-703"></a>
### T-703 — Onion skinning

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Native artwork ghosts with themed past/future tint, opacity/falloff, 1–5 frame/key steps and selected-owner filter.
- Remaining condition / qualification or scope limit: No product gap in retained T-703 scope. Manual visual contrast/scrubbing acceptance pending; no stateful physics ghosts or preference persistence promised.
- Evidence checkpoints: T-824.
- Code: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs); [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs); [`editor/src/session.rs`](../editor/src/session.rs).
- Tests: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs); [`editor/src/ui/canvas/renderer/t824_tests.rs`](../editor/src/ui/canvas/renderer/t824_tests.rs); [`editor/src/ui/timeline/transport/t824_tests.rs`](../editor/src/ui/timeline/transport/t824_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Category change justified individually: The original ghost-generation/tint/falloff/step/selection scope is implemented. T-824 closes theme/opacity and boundary gaps; only Windows appearance/scrubbing acceptance remains. Session persistence and physics ghost simulation were not original T-703 requirements.
- Confidence: HIGH.

<a id="t-704"></a>
### T-704 — Curve editor refinement

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Existing scalar Graph plus numeric tangents, single-junction linking, marquee and Normalize View Only.
- Remaining condition / qualification or scope limit: No remaining product bullet in retained scalar scope. Windows ergonomics pending. Normalized editing, multi-junction linking, FFD/Path Graph and middle-button pan are separate extensions, not retroactively added requirements.
- Evidence checkpoints: T-814; T-820; T-825; T-826; T-827; T-829.
- Code: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs); [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs); [`editor/src/ui/timeline/graph/marquee.rs`](../editor/src/ui/timeline/graph/marquee.rs); [`editor/src/ui/timeline/graph/normalization.rs`](../editor/src/ui/timeline/graph/normalization.rs); [`document/src/tangent_linking.rs`](../document/src/tangent_linking.rs).
- Tests: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs); [`editor/src/ui/timeline/graph/t820_tests.rs`](../editor/src/ui/timeline/graph/t820_tests.rs); [`editor/src/ui/timeline/graph/t825_tests.rs`](../editor/src/ui/timeline/graph/t825_tests.rs); [`editor/src/ui/timeline/graph/t826_tests.rs`](../editor/src/ui/timeline/graph/t826_tests.rs); [`editor/src/ui/timeline/t829_interactions.rs`](../editor/src/ui/timeline/t829_interactions.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Category change justified individually: All retained scalar-curve feature bullets exist: views, property/time/value curves, handles/history, colors/value axis/fit, normalized overlay, tie modes and rectangle selection. Normalize View Only fulfills the requested overlay toggle; normalized author editing and multi-junction linking are not added historical requirements. Real usability stays pending.
- Confidence: HIGH.

<a id="t-814"></a>
### T-814 — Shared key selection

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: LOCAL_CANDIDATE. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Shared key selection, numeric tangent Apply/Cancel and safe graph gesture framing.
- Remaining condition / qualification or scope limit: Broader T-704 stays partial; combined graph edits retain existing granularity; GUI pending
- Evidence checkpoints: T-814.
- Code: [`editor/src/key_selection.rs`](../editor/src/key_selection.rs); [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Tests: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-905"></a>
### T-905 — Non-destructive timeline offset

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Persisted per-bone static sampling offsets and header UI.
- Remaining condition / qualification or scope limit: Not arbitrary per-track/per-selection offsets for slots/constraints/deforms
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/animation.rs`](../core/src/animation.rs); [`editor/src/ui/timeline/tree.rs`](../editor/src/ui/timeline/tree.rs); [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs); [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-906"></a>
### T-906 — Timeline markers

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Named colored serialized timeline markers and snapping, omitted from runtime export.
- Remaining condition / qualification or scope limit: Not runtime event callbacks; no fresh desktop certification
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/timeline/ruler.rs`](../editor/src/ui/timeline/ruler.rs); [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs); [`export/src/context.rs`](../export/src/context.rs).
- Tests: [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs); [`export/src/context.rs`](../export/src/context.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-912"></a>
### T-912 — Graph editor ergonomics review

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Auto-fit/readouts/overshoot/gap notices, numerics, shared selection and newer scalar Graph refinements.
- Remaining condition / qualification or scope limit: The literal historical opt-in snapping criterion is still not met: Raw frame snapping is the default, Alt temporarily frees it, and the badge is visible. This policy gap is distinct from pending Windows readability/usability. Linked/normalize/marquee are no longer absent.
- Evidence checkpoints: T-814; T-820; T-825; T-826; T-827; T-829.
- Code: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs); [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs); [`docs/graph-editor.md`](../docs/graph-editor.md).
- Tests: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs); [`editor/src/ui/timeline/graph/t826_tests.rs`](../editor/src/ui/timeline/graph/t826_tests.rs); [`editor/src/ui/timeline/t829_interactions.rs`](../editor/src/ui/timeline/t829_interactions.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. The literal historical opt-in snapping criterion is still not met: Raw frame snapping is the default, Alt temporarily frees it, and the badge is visible. This policy gap is distinct from pending Windows readability/usability. Linked/normalize/marquee are no longer absent.
- Confidence: HIGH.

<a id="rigging"></a>
## Rigging

<a id="t-206"></a>
### T-206 — Hierarchy panel UX

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Hierarchy, folders, reparenting and native rename/delete.
- Remaining condition / qualification or scope limit: Original hierarchy/reparent/lock scope exists; separate bone-name context-menu usability issue B03 remains, not an absent reparent system.
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs); [`document/src/commands/group_cmds.rs`](../document/src/commands/group_cmds.rs).
- Tests: [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs); [`document/src/commands/group_cmds.rs`](../document/src/commands/group_cmds.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-307"></a>
### T-307 — Attachment inspector — per-attachment properties

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Original region attachment property/action and on-canvas authoring scope, via native commands..
- Remaining condition / qualification or scope limit: Mesh/sequence image replacement is a separate unimplemented extension; not an absent region inspector.
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs); [`document/src/commands/attachment_cmds.rs`](../document/src/commands/attachment_cmds.rs).
- Tests: [`editor/src/ui/inspector/image_tests.rs`](../editor/src/ui/inspector/image_tests.rs); [`document/src/commands/attachment_cmds.rs`](../document/src/commands/attachment_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-501"></a>
### T-501 — Transform constraint

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native transform constraints and mixes.
- Remaining condition / qualification or scope limit: Not part of common Pixi/Phaser alpha contract
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/pose.rs`](../core/src/pose.rs); [`document/src/commands/constraint_cmds.rs`](../document/src/commands/constraint_cmds.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs); [`document/src/commands/constraint_cmds.rs`](../document/src/commands/constraint_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-502"></a>
### T-502 — Path attachment + path constraint

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Static polylines/native constraint plus native PathPosition keys/evaluation/author files.
- Remaining condition / qualification or scope limit: Animated PathSpacing and geometric Bezier handles remain absent; data/runtime exports refuse PathPosition. Graph/group/clipboard Path keys are deferred.
- Evidence checkpoints: T-831.
- Code: [`core/src/path.rs`](../core/src/path.rs); [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs); [`core/src/pose.rs`](../core/src/pose.rs); [`document/src/path_keys.rs`](../document/src/path_keys.rs); [`editor/src/path_keys.rs`](../editor/src/path_keys.rs); [`formats/src/compatibility.rs`](../formats/src/compatibility.rs).
- Tests: [`core/src/path.rs`](../core/src/path.rs); [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs); [`core/tests/path_position.rs`](../core/tests/path_position.rs); [`document/tests/path_position.rs`](../document/tests/path_position.rs); [`formats/tests/path_position.rs`](../formats/tests/path_position.rs); [`export/tests/path_position.rs`](../export/tests/path_position.rs); [`render/tests/path_position.rs`](../render/tests/path_position.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Animated PathSpacing and geometric Bezier handles remain absent; data/runtime exports refuse PathPosition. Graph/group/clipboard Path keys are deferred.
- Confidence: HIGH.

<a id="t-503"></a>
### T-503 — Physics constraint

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Rust caller-owned fixed-step physics state, reset and deterministic advance.
- Remaining condition / qualification or scope limit: Rust fixed-step contract is complete in CAP-08; original export/playback parity not fulfilled: native TS physics activation incompatible and rendered exports explicitly refuse it.
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/physics.rs`](../core/src/physics.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Tests: [`core/src/physics.rs`](../core/src/physics.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-504"></a>
### T-504 — IK completeness

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Aim/analytic/FABRIK, bend, softness/stretch, 3+ stiffness and target creation.
- Remaining condition / qualification or scope limit: No unique-solution guarantee or new IK key dots; runtime IK excluded
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/constraints.rs`](../core/src/constraints.rs); [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs); [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-505"></a>
### T-505 — Slot & bone presentation depth

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Bone/slot colors, visibility, locks and native static dark tint.
- Remaining condition / qualification or scope limit: Not every presentation variant or animated dark tint; viewport acceptance incomplete
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/slot.rs`](../core/src/slot.rs); [`core/src/skeleton.rs`](../core/src/skeleton.rs); [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs); [`editor/src/session.rs`](../editor/src/session.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-506"></a>
### T-506 — Event timeline

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Typed event authoring and native payload export.
- Remaining condition / qualification or scope limit: t=0 ignored by current runtime policy; audio metadata is not audio playback
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/animation.rs`](../core/src/animation.rs); [`editor/src/ui/timeline/events.rs`](../editor/src/ui/timeline/events.rs); [`document/src/commands/event_cmds.rs`](../document/src/commands/event_cmds.rs).
- Tests: [`document/src/commands/event_cmds.rs`](../document/src/commands/event_cmds.rs); [`core/src/animation.rs`](../core/src/animation.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-507"></a>
### T-507 — Skin manager + multi-skin composition

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Named skins, default fallback and editor composition controls.
- Remaining condition / qualification or scope limit: Game adapters use one compatible skin; arbitrary multi-skin/mesh-image workflows unqualified
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/skin.rs`](../core/src/skin.rs); [`editor/src/ui/skins.rs`](../editor/src/ui/skins.rs); [`document/src/commands/skin_cmds.rs`](../document/src/commands/skin_cmds.rs).
- Tests: [`core/src/skin.rs`](../core/src/skin.rs); [`document/src/commands/skin_cmds.rs`](../document/src/commands/skin_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-508"></a>
### T-508 — Former performance task renumbered into T-706

**SUPERSEDED / AUTOMATED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: SUPERSEDED.

- Historical criteria fulfilled: Former performance task renumbered into T-706.
- Remaining condition / qualification or scope limit: Historical alias only, not another performance implementation
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`docs/TASKS-before-T817.md`](../docs/TASKS-before-T817.md); [`core/src/pose.rs`](../core/src/pose.rs).
- Tests: [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-816"></a>
### T-816 — Rigging Discoverability & First-Run Experience

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: LOCAL_CANDIDATE. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: IK chain/span/solver diagnostics, brush target fix and unchanged Tentacle discovery example.
- Remaining condition / qualification or scope limit: Artist discovery exercise pending; no new solver/runtime capability
- Evidence checkpoints: T-816.
- Code: [`editor/src/ui/inspector/ik_info.rs`](../editor/src/ui/inspector/ik_info.rs); [`editor/src/ui/weights.rs`](../editor/src/ui/weights.rs).
- Tests: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs); [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-904"></a>
### T-904 — Saved selection sets (not hierarchy folders)

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED**. Public base: NOT_DISTRIBUTED. T-817 implementation: ABSENT.

- Historical criteria fulfilled: Hierarchy folders exist, but no saved overlapping selection-set model/schema/panel found.
- Remaining condition / qualification or scope limit: Stale SelectionSet comments are not an implementation; new optional schema needed if later authorized
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/skeleton.rs`](../core/src/skeleton.rs); [`formats/src/schema.rs`](../formats/src/schema.rs); [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs).
- Tests: [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-907"></a>
### T-907 — Bone length edit carries its children

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Length edit carries tip children; Alt preserves positions; one native Undo.
- Remaining condition / qualification or scope limit: Remembered carry preference/full canvas-length interaction not identified
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Tests: [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-908"></a>
### T-908 — Make the existing advantages discoverable

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Visible N-bone IK and brush guidance plus documented unchanged Tentacle exercise.
- Remaining condition / qualification or scope limit: No product gap in retained discovery surfaces/sample scope. New-user no-source Windows exercise remains pending; not a universal discoverability certificate.
- Evidence checkpoints: T-816.
- Code: [`docs/rigging-capabilities.md`](../docs/rigging-capabilities.md); [`docs/first-rig-in-10-minutes.md`](../docs/first-rig-in-10-minutes.md); [`editor/src/ui/inspector/ik_info.rs`](../editor/src/ui/inspector/ik_info.rs); [`editor/src/ui/weights.rs`](../editor/src/ui/weights.rs).
- Tests: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs); [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Category change justified individually: Source already provides N-bone IK chain/span/solver explanations, visible brush controls/help, first-rig/advantages documentation and the 8-bone Tentacle sample. T-817 conflated the pending new-user exercise with missing implementation. The exercise remains pending; no new product work is credited to T-832.
- Confidence: HIGH.

<a id="t-909"></a>
### T-909 — Rig transfer between skeletons

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Reviewed native cross-project region/asset/bone-key copy through separate import workflow.
- Remaining condition / qualification or scope limit: Explicit Move, general meshes/FFD/path/clipping/physics/alternate-skin or advanced animation transfer remain unavailable/refused. Local transforms are preserved, not arbitrary world pose; this is not retargeting.
- Evidence checkpoints: T-819.
- Code: [`document/src/clipboard.rs`](../document/src/clipboard.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs); [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`document/src/transfer.rs`](../document/src/transfer.rs); [`editor/src/transfer.rs`](../editor/src/transfer.rs).
- Tests: [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs); [`editor/src/app_state.rs`](../editor/src/app_state.rs); [`document/src/transfer/tests.rs`](../document/src/transfer/tests.rs); [`editor/src/transfer/tests.rs`](../editor/src/transfer/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Explicit Move, general meshes/FFD/path/clipping/physics/alternate-skin or advanced animation transfer remain unavailable/refused. Local transforms are preserved, not arbitrary world pose; this is not retargeting.
- Confidence: HIGH.

<a id="t-911"></a>
### T-911 — Physics stability harness

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: 10k-step soak, energy decay and 30/60/144 physics tests; velocity API.
- Remaining condition / qualification or scope limit: No complete debug overlay or broad unrelated-edit interaction qualification
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/physics.rs`](../core/src/physics.rs); [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs).
- Tests: [`core/src/physics.rs`](../core/src/physics.rs); [`core/src/pose.rs`](../core/src/pose.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="assets-psd"></a>
## Assets / PSD

<a id="t-301"></a>
### T-301 — Asset database + image drop-import

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native encoded image bytes, image import and content-keyed upload pipeline.
- Remaining condition / qualification or scope limit: Project portability requires sidecar; broad texture-eviction/performance remains T-706
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/assets.rs`](../core/src/assets.rs); [`editor/src/ui/assets.rs`](../editor/src/ui/assets.rs); [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs).
- Tests: [`core/src/assets.rs`](../core/src/assets.rs); [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-302"></a>
### T-302 — PSD import

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Initial PSD import plus exact-path refresh and one explicit path-change mapping.
- Remaining condition / qualification or scope limit: Bulk/multiple missing path mapping, complex/shared/mesh/FFD/sequence refresh, automatic added/deleted-layer reconciliation and Photoshop effects/masks remain outside qualified workflow.
- Evidence checkpoints: T-822.
- Code: [`formats/src/psd_read.rs`](../formats/src/psd_read.rs); [`editor/src/ui/psd_import.rs`](../editor/src/ui/psd_import.rs); [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs); [`document/src/psd_reimport/path_change.rs`](../document/src/psd_reimport/path_change.rs); [`editor/src/ui/psd_import/path_resolution.rs`](../editor/src/ui/psd_import/path_resolution.rs).
- Tests: [`formats/tests/psd_nested.rs`](../formats/tests/psd_nested.rs); [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs); [`document/tests/psd_path_resolution.rs`](../document/tests/psd_path_resolution.rs); [`editor/src/ui/psd_import/path_resolution/tests.rs`](../editor/src/ui/psd_import/path_resolution/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Bulk/multiple missing path mapping, complex/shared/mesh/FFD/sequence refresh, automatic added/deleted-layer reconciliation and Photoshop effects/masks remain outside qualified workflow.
- Confidence: HIGH.

<a id="t-303"></a>
### T-303 — foreign armature importer

**SUPERSEDED / AUTOMATED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: SUPERSEDED.

- Historical criteria fulfilled: Removed built-in foreign importer replaced by optional community JavaScript packages.
- Remaining condition / qualification or scope limit: Do not restore a second built-in importer; package licensing/availability remain external
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`formats/src/importer.rs`](../formats/src/importer.rs); [`plugins/src/importer.rs`](../plugins/src/importer.rs).
- Tests: [`formats/tests/importer_registry.rs`](../formats/tests/importer_registry.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-304"></a>
### T-304 — Startup window

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Startup New/Open/recent/sample path with safe document replacement.
- Remaining condition / qualification or scope limit: No first-user study of local T-816; unknown structural input limits remain
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/startup.rs`](../editor/src/ui/startup.rs); [`editor/src/app.rs`](../editor/src/app.rs).
- Tests: [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs); [`editor/src/app_psd_workflow_tests.rs`](../editor/src/app_psd_workflow_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-305"></a>
### T-305 — Atlas / spritesheet import modal

**COMPLETE / NOT_TESTED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Existing atlas modal and native atlas import mapping.
- Remaining condition / qualification or scope limit: No guarantee for every vendor atlas variant
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/atlas.rs`](../editor/src/ui/atlas.rs); [`document/src/import_ops.rs`](../document/src/import_ops.rs).
- Tests: [`document/src/import_ops.rs`](../document/src/import_ops.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-306"></a>
### T-306 — Asset relink + external-source workflow

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Image relink/reload; safe save refuses missing image bytes.
- Remaining condition / qualification or scope limit: Moving only project does not move sidecar; missing assets need explicit resolution
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/assets.rs`](../editor/src/ui/assets.rs); [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs).
- Tests: [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs); [`formats/tests/reliability.rs`](../formats/tests/reliability.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-1003"></a>
### T-1003 — Move foreign formats to community JavaScript plugins

**COMPLETE / AUTOMATED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Foreign formats moved to optional pinned JavaScript packages.
- Remaining condition / qualification or scope limit: Missing package differs from broken builtin; no generic Godot/foreign fidelity promise
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`formats/src/importer.rs`](../formats/src/importer.rs); [`plugins/src/importer.rs`](../plugins/src/importer.rs); [`plugins/src/exporter.rs`](../plugins/src/exporter.rs); [`.gitmodules`](../.gitmodules).
- Tests: [`formats/tests/importer_registry.rs`](../formats/tests/importer_registry.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="meshes"></a>
## Meshes

<a id="t-401"></a>
### T-401 — Mesh attachment editing UI

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Mesh vertices/triangles, constrained retriangulation, UV and exact local numeric drafts.
- Remaining condition / qualification or scope limit: No universal snapping or arbitrary linked-source/topology edit safety
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/canvas/tools/mesh_edit.rs`](../editor/src/ui/canvas/tools/mesh_edit.rs); [`editor/src/ui/uv.rs`](../editor/src/ui/uv.rs); [`document/src/commands/mesh_cmds.rs`](../document/src/commands/mesh_cmds.rs).
- Tests: [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs); [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-402"></a>
### T-402 — Auto mesh tracing

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Original tracer retained; safe nearest-vertex weight-row remap and bind preservation.
- Remaining condition / qualification or scope limit: FFD-dependent, linked/shared/overridden or pinned retrace is refused; no topology/FFD migration. General <100 ms release/artist preservation budget remains unqualified beyond one ring fixture.
- Evidence checkpoints: T-823.
- Code: [`editor/src/ui/trace.rs`](../editor/src/ui/trace.rs); [`document/src/meshgen.rs`](../document/src/meshgen.rs); [`document/src/retrace.rs`](../document/src/retrace.rs).
- Tests: [`document/src/meshgen.rs`](../document/src/meshgen.rs); [`document/tests/retrace.rs`](../document/tests/retrace.rs); [`editor/src/ui/trace/t823_tests.rs`](../editor/src/ui/trace/t823_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. FFD-dependent, linked/shared/overridden or pinned retrace is refused; no topology/FFD migration. General <100 ms release/artist preservation budget remains unqualified beyond one ring fixture.
- Confidence: HIGH.

<a id="t-403"></a>
### T-403 — Weight painting + bind compensation

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Add/Subtract/Replace/Smooth/Direct weights, radius/feather/session locks, normalization and history.
- Remaining condition / qualification or scope limit: Complete native scoped brush; not autorigging or every-project visual fidelity; T-816 GUI pending
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/weights.rs`](../editor/src/ui/weights.rs); [`document/src/commands/weight_cmds.rs`](../document/src/commands/weight_cmds.rs).
- Tests: [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs); [`document/src/commands/weight_cmds.rs`](../document/src/commands/weight_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-404"></a>
### T-404 — Deform timelines

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Stable FFD drag plus typed owned-mesh Dopesheet selection/time/delete/interpolation.
- Remaining condition / qualification or scope limit: Stale topology is reported by live diagnostics and editing refuses it; the author loader still directly constructs Deform rather than invalidating its track at load. No general topology/FFD remap or ambiguous/shared ownership edit. Vector clipboard belongs to T-209 and alone is not the reason for this status.
- Evidence checkpoints: T-818; T-821.
- Code: [`editor/src/edit_router.rs`](../editor/src/edit_router.rs); [`editor/src/ui/canvas/tools/mesh_edit.rs`](../editor/src/ui/canvas/tools/mesh_edit.rs); [`core/src/pose.rs`](../core/src/pose.rs); [`document/src/ffd_keys.rs`](../document/src/ffd_keys.rs); [`formats/src/convert.rs`](../formats/src/convert.rs); [`document/src/diagnostics.rs`](../document/src/diagnostics.rs).
- Tests: [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs); [`core/src/pose.rs`](../core/src/pose.rs); [`editor/src/ffd_keys/tests.rs`](../editor/src/ffd_keys/tests.rs); [`editor/src/ui/timeline/sheet/t821_tests.rs`](../editor/src/ui/timeline/sheet/t821_tests.rs); [`document/src/diagnostics/tests.rs`](../document/src/diagnostics/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Stale topology is reported by live diagnostics and editing refuses it; the author loader still directly constructs Deform rather than invalidating its track at load. No general topology/FFD remap or ambiguous/shared ownership edit. Vector clipboard belongs to T-209 and alone is not the reason for this status.
- Confidence: HIGH.

<a id="t-405"></a>
### T-405 — Clipping attachments

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Clipping polygon/end-slot authoring, ordering and native commands.
- Remaining condition / qualification or scope limit: Common adapters support bounded sequential masks, not arbitrary nested intersections
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/clipping.rs`](../core/src/clipping.rs); [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs).
- Tests: [`core/src/clipping.rs`](../core/src/clipping.rs); [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-815"></a>
### T-815 — Viewport Precision & Numeric Geometry Editing

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: LOCAL_CANDIDATE. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Scoped exact Set/Add drafts for mesh/UV/path/polygons with Apply/Cancel.
- Remaining condition / qualification or scope limit: Not every handle/expression/snap or linked-source edit; GUI pending
- Evidence checkpoints: T-815.
- Code: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs); [`editor/src/ui/uv.rs`](../editor/src/ui/uv.rs).
- Tests: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-902"></a>
### T-902 — Numeric entry for every vertex and handle

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Exact mesh/UV/path/polygon Set/Add and numeric normalized Bezier handles.
- Remaining condition / qualification or scope limit: Not every draggable point, expression parser or snap-to-neighbour
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs); [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Tests: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs); [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="formats-exports"></a>
## Formats / exports

<a id="t-601"></a>
### T-601 — Offscreen render + image / spritesheet / sequence export

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: CPU deterministic PNG/JPEG/WebP sequences, spritesheets, framing/trim/padding/batch.
- Remaining condition / qualification or scope limit: Default skin/Normal/no physics; original background/performance/viewport criteria not all closed
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`render/src/sequence.rs`](../render/src/sequence.rs); [`render/src/spritesheet.rs`](../render/src/spritesheet.rs); [`editor/src/png_export.rs`](../editor/src/png_export.rs).
- Tests: [`render/src/lib.rs`](../render/src/lib.rs); [`export/tests/generated_images.rs`](../export/tests/generated_images.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-601-1"></a>
### T-601.1 — PNG retained

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: PNG retained, explicit opaque JPEG background and lossless RGBA WebP.
- Remaining condition / qualification or scope limit: JPEG lossy tolerance/no alpha; no runtime format change
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`render/src/encoding.rs`](../render/src/encoding.rs); [`editor/src/png_export.rs`](../editor/src/png_export.rs).
- Tests: [`render/src/encoding.rs`](../render/src/encoding.rs); [`export/tests/generated_images.rs`](../export/tests/generated_images.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-601-2"></a>
### T-601.2 — Opt-in common camera/global bounds

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Opt-in common camera/global bounds, trim reconstruction and padding.
- Remaining condition / qualification or scope limit: Skin/blend/physics exclusions persist; not arbitrary renderer fidelity
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`render/src/framing.rs`](../render/src/framing.rs); [`editor/src/ui/png_sequence.rs`](../editor/src/ui/png_sequence.rs).
- Tests: [`render/src/framing.rs`](../render/src/framing.rs); [`export/tests/generated_images.rs`](../export/tests/generated_images.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-601-3"></a>
### T-601.3 — Selected-clip batches

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Selected-clip batches, deterministic plans and new-destination publication.
- Remaining condition / qualification or scope limit: Not crash-atomic multi-file commit or unrestricted memory/performance guarantee
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`export/src/batch.rs`](../export/src/batch.rs); [`editor/src/ui/png_sequence.rs`](../editor/src/ui/png_sequence.rs).
- Tests: [`export/tests/batch.rs`](../export/tests/batch.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-602"></a>
### T-602 — Video export via ffmpeg

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: CPU MP4/GIF through supervised external FFmpeg, progress/cancel and safe publication.
- Remaining condition / qualification or scope limit: No audio/GIF alpha/physics or every-player guarantee; external encoder required
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/video_export.rs`](../editor/src/video_export.rs); [`editor/src/video_export/gif.rs`](../editor/src/video_export/gif.rs); [`editor/src/ui/video.rs`](../editor/src/ui/video.rs).
- Tests: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs); [`editor/src/video_export/gif/tests.rs`](../editor/src/video_export/gif/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-602-1"></a>
### T-602.1 — Raw RGB CPU video to H.264

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Raw RGB CPU video to H.264, no-replace publication.
- Remaining condition / qualification or scope limit: Opaque background, no bundled encoder; hard-link filesystem requirement
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/video_export.rs`](../editor/src/video_export.rs); [`export/src/new_file.rs`](../export/src/new_file.rs).
- Tests: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-602-2"></a>
### T-602.2 — Two-pass GIF palette

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Two-pass GIF palette, repeat policy and measured centisecond timing.
- Remaining condition / qualification or scope limit: No GIF alpha or arbitrary exact FPS promise
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/video_export/gif.rs`](../editor/src/video_export/gif.rs).
- Tests: [`editor/src/video_export/gif/tests.rs`](../editor/src/video_export/gif/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-602-3"></a>
### T-602.3 — CRF/bitrate

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: CRF/bitrate, probed libx264/libx264rgb and Windows opt-in association opening.
- Remaining condition / qualification or scope limit: Native opener injected in tests; RGB requires player consent; not hardware encoding
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/video_export.rs`](../editor/src/video_export.rs); [`editor/src/ui/video.rs`](../editor/src/ui/video.rs).
- Tests: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-603"></a>
### T-603 — Original hardcoded-exporter plan replaced by user-authored templates/preset

**SUPERSEDED / AUTOMATED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: SUPERSEDED.

- Historical criteria fulfilled: Original hardcoded-exporter plan replaced by user-authored templates/presets.
- Remaining condition / qualification or scope limit: Do not create another hardcoded exporter; aggregate library still partial
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`export/src/template.rs`](../export/src/template.rs); [`docs/export-plan.md`](../docs/export-plan.md).
- Tests: [`export/tests/template.rs`](../export/tests/template.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-603a"></a>
### T-603a — Atlas bake

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Deterministic atlas pages, trim/padding and mappings.
- Remaining condition / qualification or scope limit: Common runtime excludes rotated atlases even if custom presets differ
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`export/src/atlas.rs`](../export/src/atlas.rs).
- Tests: [`export/tests/atlas.rs`](../export/tests/atlas.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-603b"></a>
### T-603b — Template engine + export runner

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Strict template runner, path validation, staging/backups/rollback on ordinary failure.
- Remaining condition / qualification or scope limit: No filesystem-wide atomicity or guaranteed power-loss durability
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`export/src/template.rs`](../export/src/template.rs); [`export/src/run.rs`](../export/src/run.rs); [`export/src/staged.rs`](../export/src/staged.rs).
- Tests: [`export/tests/template.rs`](../export/tests/template.rs); [`export/tests/safety.rs`](../export/tests/safety.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-603c"></a>
### T-603c — Native runtime format, authored as a template

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Native v2 axes/curves/FFD/static dark tint retained; PathPosition explicitly refused.
- Remaining condition / qualification or scope limit: Native v2 does not encode animated PathPosition; no new runtime field or compatibility claim. Existing supported profile remains unchanged.
- Evidence checkpoints: T-831.
- Code: [`export/src/presets/ankhimate_runtime.json`](../export/src/presets/ankhimate_runtime.json); [`export/src/context.rs`](../export/src/context.rs); [`export/src/run.rs`](../export/src/run.rs); [`plugins/src/exporter.rs`](../plugins/src/exporter.rs).
- Tests: [`export/tests/export.rs`](../export/tests/export.rs); [`export/tests/ffd_curves.rs`](../export/tests/ffd_curves.rs); [`export/tests/dark_tint.rs`](../export/tests/dark_tint.rs); [`export/tests/path_position.rs`](../export/tests/path_position.rs); [`editor/src/app_path_tests.rs`](../editor/src/app_path_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Native v2 does not encode animated PathPosition; no new runtime field or compatibility claim. Existing supported profile remains unchanged.
- Confidence: HIGH.

<a id="t-603d"></a>
### T-603d — Export panel

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Existing data/rendered export preflight, collision inventory/consent and safety rechecks.
- Remaining condition / qualification or scope limit: Not a continuously updated global diagnostics dock
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/export.rs`](../editor/src/ui/export.rs); [`editor/src/export_preflight.rs`](../editor/src/export_preflight.rs); [`export/src/preflight.rs`](../export/src/preflight.rs).
- Tests: [`export/tests/preflight.rs`](../export/tests/preflight.rs); [`export/tests/preflight_paths.rs`](../export/tests/preflight_paths.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-603e"></a>
### T-603e — Preset library

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Native/generic/Phaser-atlas presets and custom templates.
- Remaining condition / qualification or scope limit: No qualified Godot preset; foreign-format plugins remain optional
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`export/src/presets/mod.rs`](../export/src/presets/mod.rs); [`export/src/preset.rs`](../export/src/preset.rs).
- Tests: [`export/tests/template.rs`](../export/tests/template.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-605"></a>
### T-605 — Format spec finalization + migration policy

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Existing v1 spec/gates/refusal plus documented additive PathPosition closed enum.
- Remaining condition / qualification or scope limit: Authentic v0 migration still unqualified; no universal hostile-resource budget. T-831 keeps author/envelope 1 but older readers reject the new variant; future release must communicate feature-level compatibility.
- Evidence checkpoints: T-831.
- Code: [`docs/format-spec.md`](../docs/format-spec.md); [`formats/src/compatibility.rs`](../formats/src/compatibility.rs); [`formats/src/schema.rs`](../formats/src/schema.rs); [`formats/src/convert.rs`](../formats/src/convert.rs).
- Tests: [`formats/tests/schema_contract.rs`](../formats/tests/schema_contract.rs); [`formats/tests/migrations.rs`](../formats/tests/migrations.rs); [`formats/tests/reliability.rs`](../formats/tests/reliability.rs); [`formats/tests/path_position.rs`](../formats/tests/path_position.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Authentic v0 migration still unqualified; no universal hostile-resource budget. T-831 keeps author/envelope 1 but older readers reject the new variant; future release must communicate feature-level compatibility.
- Confidence: HIGH.

<a id="pixijs-phaser"></a>
## PixiJS / Phaser

<a id="t-604"></a>
### T-604 — Runtime integration

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: External TypeScript core and two packaged adapters with qualified common profile.
- Remaining condition / qualification or scope limit: In-workspace Rust player plan superseded; no universal runtime parity
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`docs/rigging-capabilities.md`](../docs/rigging-capabilities.md); [`export/src/presets/ankhimate_runtime.json`](../export/src/presets/ankhimate_runtime.json).
- Tests: [`export/tests/export.rs`](../export/tests/export.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-810"></a>
### T-810 — Phaser common-profile corrections and public playback API aligned to shared

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Phaser common-profile corrections and public playback API aligned to shared renderer helpers.
- Remaining condition / qualification or scope limit: No new all-feature parity; detailed external runtime rows below
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: `runtime:phaser/src/phaser.ts` (protected runtime source); `runtime:core/src/render/mesh.ts` (protected runtime source).
- Tests: `runtime:phaser/test/parity.test.mjs` (protected runtime source).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="ux-tools"></a>
## UX / tools

<a id="t-701"></a>
### T-701 — Settings, keymap, autosave

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: JSON preferences, registered keymap and monotonic isolated idle autosave.
- Remaining condition / qualification or scope limit: Original exhaustive settings list/restart acceptance incomplete; unsaved-doc autosave policy retained
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/config.rs`](../editor/src/config.rs); [`editor/src/keymap.rs`](../editor/src/keymap.rs); [`editor/src/autosave.rs`](../editor/src/autosave.rs); [`editor/src/ui/settings.rs`](../editor/src/ui/settings.rs).
- Tests: [`editor/src/config.rs`](../editor/src/config.rs); [`editor/src/keymap.rs`](../editor/src/keymap.rs); [`editor/src/autosave_clock_tests.rs`](../editor/src/autosave_clock_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-702"></a>
### T-702 — Diagnostics / warnings

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Global shared integrity rules, debounced badge/list and exact safe offender selection.
- Remaining condition / qualification or scope limit: Automatic select-and-frame is absent: camera intentionally unchanged. Large image scans remain synchronous without a general 500-bone/asset completion budget. Export preflight remains independently authoritative.
- Evidence checkpoints: T-818.
- Code: [`export/src/preflight.rs`](../export/src/preflight.rs); [`editor/src/export_preflight.rs`](../editor/src/export_preflight.rs); [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs); [`document/src/diagnostics.rs`](../document/src/diagnostics.rs); [`editor/src/diagnostics.rs`](../editor/src/diagnostics.rs).
- Tests: [`export/tests/preflight.rs`](../export/tests/preflight.rs); [`export/tests/preflight_paths.rs`](../export/tests/preflight_paths.rs); [`document/src/diagnostics/tests.rs`](../document/src/diagnostics/tests.rs); [`editor/src/diagnostics/tests.rs`](../editor/src/diagnostics/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. Automatic select-and-frame is absent: camera intentionally unchanged. Large image scans remain synchronous without a general 500-bone/asset completion budget. Export preflight remains independently authoritative.
- Confidence: HIGH.

<a id="t-705"></a>
### T-705 — Localization

**DEFERRED / NOT_TESTED / NOT_DISTRIBUTED**. Public base: NOT_DISTRIBUTED. T-817 implementation: DEFERRED.

- Historical criteria fulfilled: English UI literals; no localization catalog/macro/picker identified.
- Remaining condition / qualification or scope limit: i18n implementation absent and deliberately deferred; no locale certificate
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs); [`docs/TASKS-before-T817.md`](../docs/TASKS-before-T817.md).
- Tests: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-706"></a>
### T-706 — Performance pass

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: SOURCE_ONLY. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Bounded native capture/indexing and selected-view membership optimizations, measured headlessly.
- Remaining condition / qualification or scope limit: No general xtask bench/trend or integrated-GPU 500-bone/300-row 60 fps certificate. Large commands/model/payload work remains; debug medians are not budgets.
- Evidence checkpoints: T-827; T-829.
- Code: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs); [`editor/src/ui/timeline/sheet.rs`](../editor/src/ui/timeline/sheet.rs); [`document/src/clipboard/keys.rs`](../document/src/clipboard/keys.rs); [`editor/src/ui/timeline/membership.rs`](../editor/src/ui/timeline/membership.rs).
- Tests: [`core/src/physics.rs`](../core/src/physics.rs); [`document/src/clipboard/keys/t827_tests.rs`](../document/src/clipboard/keys/t827_tests.rs); [`editor/src/ui/timeline/t829_probe.rs`](../editor/src/ui/timeline/t829_probe.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Inspected the current implementation and named tests; later work closes the stated sub-scope only. No general xtask bench/trend or integrated-GPU 500-bone/300-row 60 fps certificate. Large commands/model/payload work remains; debug medians are not budgets.
- Confidence: HIGH.

<a id="t-708"></a>
### T-708 — Viewport interaction polish

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Gizmos, region pivot editing, camera and precise geometry/UV entry.
- Remaining condition / qualification or scope limit: No universal snapping/expression-on-drag/pivot editing for every attachment; framing acceptance incomplete
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/canvas/tools/select.rs`](../editor/src/ui/canvas/tools/select.rs); [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs).
- Tests: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs); [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-709"></a>
### T-709 — Accessibility & window UX

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Control sizing, scaling, partial keyboard/AccessKit, close guard and docking.
- Remaining condition / qualification or scope limit: No whole workflow keyboard/contrast/colorblind audit or exhaustive icon-tooltip certificate
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs); [`editor/src/ui/settings.rs`](../editor/src/ui/settings.rs); [`editor/src/app.rs`](../editor/src/app.rs).
- Tests: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs); [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-901"></a>
### T-901 — Bulk rename with sequential numbering

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Bone numbering/substring replacement, preview and native batch rename.
- Remaining condition / qualification or scope limit: Not slot/attachment/constraint rename, regex or all path-safety variants
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/rename.rs`](../editor/src/ui/rename.rs); [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Tests: [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-903"></a>
### T-903 — Isolation mode

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Session-only descendant isolation and exit affordance.
- Remaining condition / qualification or scope limit: Optional hierarchy/dopesheet filters/configurable opacity not fully qualified
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/session.rs`](../editor/src/session.rs); [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs); [`editor/src/operators.rs`](../editor/src/operators.rs).
- Tests: [`editor/src/session.rs`](../editor/src/session.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-910"></a>
### T-910 — Multi-window / multi-monitor

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Noncanvas panels detach over one AppState; detached set persisted.
- Remaining condition / qualification or scope limit: Second viewport absent; per-window positions/multimonitor acceptance not broadly established
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/app.rs`](../editor/src/app.rs); [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs); [`editor/src/config.rs`](../editor/src/config.rs).
- Tests: [`editor/src/app.rs`](../editor/src/app.rs); [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-913"></a>
### T-913 — Name the thing under the cursor

**COMPLETE / NOT_TESTED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Entity hover labels, preference and Alt reveal.
- Remaining condition / qualification or scope limit: Overlap/DPI/delay/accessibility not exhaustively qualified No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences.
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/canvas/hover_label.rs`](../editor/src/ui/canvas/hover_label.rs); [`editor/src/config.rs`](../editor/src/config.rs).
- Tests: [`editor/src/config.rs`](../editor/src/config.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="distribution"></a>
## Distribution

<a id="t-801"></a>
### T-801 — Samples + documentation

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Approved Jade alpha sample, source examples and first-rig guide.
- Remaining condition / qualification or scope limit: Not all 3 CC0 sample/PSD/GIF criteria; new user's exercise pending
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`samples/tentacle.ankh`](../samples/tentacle.ankh); [`samples/walker.ankh`](../samples/walker.ankh); [`docs/first-rig-in-10-minutes.md`](../docs/first-rig-in-10-minutes.md).
- Tests: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-802"></a>
### T-802 — Release packaging

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Published unsigned portable Windows ZIP, manifests and matching packages.
- Remaining condition / qualification or scope limit: No signed installer, Linux/macOS artifacts or broad hardware certification
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`docs/CANDIDATE_BUILD.md`](../docs/CANDIDATE_BUILD.md); [`docs/ALPHA-LICENSES.md`](../docs/ALPHA-LICENSES.md).
- Tests: [`docs/RELEASE_READINESS.md`](../docs/RELEASE_READINESS.md).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-803"></a>
### T-803 — Web build

**DEFERRED / NOT_TESTED / NOT_DISTRIBUTED**. Public base: NOT_DISTRIBUTED. T-817 implementation: DEFERRED.

- Historical criteria fulfilled: WASM core compiles; native desktop editor remains product.
- Remaining condition / qualification or scope limit: No browser authoring frontend/distribution; WASM core is not a web editor
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`core/src/lib.rs`](../core/src/lib.rs); [`editor/src/main.rs`](../editor/src/main.rs).
- Tests: [`core/src/lib.rs`](../core/src/lib.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-810-5"></a>
### T-810.5 — Simple Windows MP4 action reusing supervised CPU export worker

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Simple Windows MP4 action reusing supervised CPU export worker.
- Remaining condition / qualification or scope limit: External FFmpeg, opaque output/new-file only; no bundled encoder
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ui/video.rs`](../editor/src/ui/video.rs); [`editor/src/operators.rs`](../editor/src/operators.rs).
- Tests: [`editor/src/ui/video_t8105_tests.rs`](../editor/src/ui/video_t8105_tests.rs); [`editor/src/video_export/t8105_tests.rs`](../editor/src/video_export/t8105_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-811"></a>
### T-811 — Next Alpha release candidate

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Integrated accepted artifacts later published unchanged through T-812.
- Remaining condition / qualification or scope limit: Historical candidate-pending prose superseded only for the exact published artifacts
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`docs/CANDIDATE_BUILD.md`](../docs/CANDIDATE_BUILD.md); [`docs/FEATURES.md`](../docs/FEATURES.md).
- Tests: [`docs/RELEASE_READINESS.md`](../docs/RELEASE_READINESS.md).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-1001"></a>
### T-1001 — Plugin host and declarative panels

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Sandboxed QuickJS, named verbs and declarative native panels.
- Remaining condition / qualification or scope limit: Not a general external package/security endorsement
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`plugins/src/lib.rs`](../plugins/src/lib.rs); [`plugins/src/panel.rs`](../plugins/src/panel.rs); [`editor/src/ui/plugin_panel.rs`](../editor/src/ui/plugin_panel.rs).
- Tests: [`plugins/src/lib.rs`](../plugins/src/lib.rs); [`plugins/src/panel.rs`](../plugins/src/panel.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-1002"></a>
### T-1002 — MCP stdio server

**COMPLETE / AUTOMATED / SOURCE_ONLY**. Public base: SOURCE_ONLY. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Headless MCP stdio consumer reusing document/export/render.
- Remaining condition / qualification or scope limit: Not autonomous AI access or browser authoring; permissions remain caller-specific
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`mcp/src/server.rs`](../mcp/src/server.rs); [`mcp/src/tools.rs`](../mcp/src/tools.rs); [`mcp/src/session.rs`](../mcp/src/session.rs).
- Tests: [`mcp/src/session.rs`](../mcp/src/session.rs); [`mcp/src/tools.rs`](../mcp/src/tools.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: No new T-832 manual test; no universal GUI certificate.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="ia"></a>
## IA

<a id="t-805"></a>
### T-805 — Native conversational assistant

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Opt-in conversation, bounded native proposals, explicit approval and atomic history.
- Remaining condition / qualification or scope limit: Not autonomous autorigging; model prose can be wrong; no generic cloud/model certificate
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ai/mod.rs`](../editor/src/ai/mod.rs); [`editor/src/ai/transport.rs`](../editor/src/ai/transport.rs); [`editor/src/ai/ui.rs`](../editor/src/ai/ui.rs).
- Tests: [`editor/src/ai/tests.rs`](../editor/src/ai/tests.rs); [`editor/src/ai/expert_tests.rs`](../editor/src/ai/expert_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-805-1"></a>
### T-805.1 — Fixed composer

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Fixed composer, phases, native receipts, bounded correction and NInfer profile.
- Remaining condition / qualification or scope limit: Historical Qwen real run is binary/model-specific; no new T-816 inference qualification
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ai/activity.rs`](../editor/src/ai/activity.rs); [`editor/src/ai/manual.rs`](../editor/src/ai/manual.rs); [`editor/src/ai/ui.rs`](../editor/src/ai/ui.rs).
- Tests: [`editor/src/ai/expert_tests.rs`](../editor/src/ai/expert_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-806"></a>
### T-806 — Viewport vision and first native rigging tools

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: PARTIAL.

- Historical criteria fulfilled: Bounded viewport preview/one-send consent, coordinate declarations and typed bone proposal tools.
- Remaining condition / qualification or scope limit: No product gap in retained native tool/consent scope; real vision/placement/model qualification remains pending and is not generalized from old narrow feedback.
- Evidence checkpoints: T-806; T-809.
- Code: [`editor/src/ai/vision.rs`](../editor/src/ai/vision.rs); [`editor/src/ai/rig.rs`](../editor/src/ai/rig.rs); [`editor/src/ai/capabilities.rs`](../editor/src/ai/capabilities.rs).
- Tests: [`editor/src/ai/t806_tests.rs`](../editor/src/ai/t806_tests.rs); [`editor/src/ai/t809_tests.rs`](../editor/src/ai/t809_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: PENDING MANUAL REVIEW.
- Status rationale: Category change justified individually: The retained scope is consented viewport preview/one-send observation plus typed first bone tools. Capture, bounded coordinates/transports, proposals, approval/refusal and atomic history exist. General artistic placement or every-model certification is validation/qualification, not an additional implemented tool. Historical narrow maintainer feedback does not pass the local campaign.
- Confidence: HIGH.

<a id="t-807"></a>
### T-807 — Controlled native character assembly

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Guarded region/slot/bone/order assembly preserving representable FK Setup.
- Remaining condition / qualification or scope limit: Weighted meshes are not regions; unrepresentable/constrained cases refused
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ai/assembly.rs`](../editor/src/ai/assembly.rs); [`editor/src/ai/tools.rs`](../editor/src/ai/tools.rs); [`document/src/part_ops.rs`](../document/src/part_ops.rs).
- Tests: [`editor/src/ai/t807_tests.rs`](../editor/src/ai/t807_tests.rs); [`document/src/part_ops.rs`](../document/src/part_ops.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-808"></a>
### T-808 — Guarded mesh weight assistance

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Paged mesh reads and bounded set/normalize/smooth existing weights after approval.
- Remaining condition / qualification or scope limit: No new influences, geometry/UV/bind/FFD edits; linked/shared/constrained cases protected
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ai/weights.rs`](../editor/src/ai/weights.rs); [`editor/src/ai/tools.rs`](../editor/src/ai/tools.rs).
- Tests: [`editor/src/ai/t808_tests.rs`](../editor/src/ai/t808_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

<a id="t-809"></a>
### T-809 — Multimodal provider transport and bounded context

**COMPLETE / MIXED / PUBLIC_ALPHA**. Public base: PUBLIC_ALPHA. T-817 implementation: COMPLETE.

- Historical criteria fulfilled: Six multimodal envelopes, destination-bound declarations and image/context budgets.
- Remaining condition / qualification or scope limit: Wire support is not concrete model compatibility; context exhaustion remains possible
- Evidence checkpoints: T-812 integration; audited exact T-816 source.
- Code: [`editor/src/ai/protocol.rs`](../editor/src/ai/protocol.rs); [`editor/src/ai/capabilities.rs`](../editor/src/ai/capabilities.rs); [`editor/src/ai/transport.rs`](../editor/src/ai/transport.rs).
- Tests: [`editor/src/ai/t809_tests.rs`](../editor/src/ai/t809_tests.rs).
- Automated validation: T-831 accepted checkpoint: 2058/0/4; referenced suites present (not rerun in T-832).
- Manual validation: Historical exact-artifact evidence only; local campaign not passed.
- Status rationale: Retained T-817 bounded implementation status: no later evidence closes the listed product remainder or disproves the accepted scope.
- Confidence: MEDIUM (accepted bounded T-817 evidence, source/suite references retained).

## Separate original refinements — fixed28 records

<a id="rt-01"></a>
### RT-01 — Shared native v1/v2 parser

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

v1/v2 read and separate author-file adapter exist. Remaining: Core v1 reading does not certify v1 adapter pixel parity Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-02"></a>
### RT-02 — Common animation and crossfade

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Independent axes/curves, playback and distinct-clip crossfade. Remaining: Interrupted crossfades may jump; seek does not emit traversal events Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-03"></a>
### RT-03 — Runtime events

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Positive-time events in deterministic tie order, incoming-only fades. Remaining: t=0 ignored at every loop; no automatic sound Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-04"></a>
### RT-04 — PixiJS 8 WebGL2 common profile

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Regions, rigid/weighted meshes, FFD, trim and multipage pages. Remaining: Explicit v2/WebGL2; constraints/linked/rotation rejected; not WebGL1/WebGPU/mobile certificate Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-05"></a>
### RT-05 — Phaser 3 common profile

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Phaser 3.90.0 public playback, skin UV refresh, dark tint and shared geometry. Remaining: Actual T-811 GL context WebGL1; legacy extras warn, not parity-certified Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-06"></a>
### RT-06 — Weighted meshes and FFD

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Qualified finite native v2 weighted/FFD subset with Rust references. Remaining: Topology/bind compatibility required; not arbitrary linked FFD inheritance Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-07"></a>
### RT-07 — Skins, light and static dark tint

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Compatible alternate skins, light animation and static dark RGBA. Remaining: No animated dark channel or arbitrary multi-skin composition Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-08"></a>
### RT-08 — Sequential clipping

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Qualified slot/end-slot sequential masks, transformed cases. Remaining: Nested replacement is not intersecting mask composition; bounded concave qualification Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-09"></a>
### RT-09 — Constraints and physics outside common profile

**PARTIAL / AUTOMATED / SOURCE_ONLY**. Historical task(s): T-604 / T-810.

Some legacy TypeScript constraint code exists; Phaser warns and Pixi rejects. Remaining: Native physics target/bones mismatch not activated; no Rust/TS stateful fidelity claim Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-10"></a>
### RT-10 — Advanced blending and linked meshes

**PARTIAL / AUTOMATED / SOURCE_ONLY**. Historical task(s): T-604 / T-810.

Normal is common qualified composition; legacy Phaser extra paths exist. Remaining: Add/Multiply/Screen and linked/rotated cases excluded, even if a core type exists Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-11"></a>
### RT-11 — Runtime resource and performance qualification

**PARTIAL / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Ownership/cleanup tests and bounded historical instance/soak observations. Remaining: Not production-scale/iGPU/mobile or universal frame-time budget Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="rt-12"></a>
### RT-12 — Reproducible game examples and packages

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-604 / T-810.

Matching GitHub tarballs, offline portable Pixi demo and standalone comparison example. Remaining: No npm publication; browser modules need HTTP and matching core/render import map Evidence: c4c38df; historical T-811 tests, no T-817 runtime rerun.

<a id="cap-01"></a>
### CAP-01 — Global document diagnostics

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-702.

Global shared integrity rules, debounced badge/list and exact safe offender selection. Remaining: Automatic select-and-frame is absent: camera intentionally unchanged. Large image scans remain synchronous without a general 500-bone/asset completion budget. Export preflight remains independently authoritative. Evidence: T-818.

<a id="cap-02"></a>
### CAP-02 — Linked tangent modes

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-704 / T-912.

No aligned/mirrored/broken handle-link state or UI identified. Remaining: Single compatible scalar junction; Raw angular/flat/nonrepresentable cases refuse; Windows pending. Evidence: T-820.

<a id="cap-03"></a>
### CAP-03 — Graph marquee and normalized overlay

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-704.

No graph marquee/normalize control identified. Remaining: Visible scalar marquee and Normalize View Only; no normalized authoring, vectors or lasso; Windows pending. Evidence: T-825 + T-826.

<a id="cap-04"></a>
### CAP-04 — Authentic legacy migration qualification

**UNKNOWN / NOT_TESTED / UNKNOWN**. Historical task(s): T-605.

No authenticated author-v0 input/conversion proof available. Remaining: Current v1 profiles qualified; prototype ZIP rejected; do not fabricate a migration fixture Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-05"></a>
### CAP-05 — Panic-time emergency project dump

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED**. Historical task(s): T-707.

No panic-time in-memory project dump/recovery package identified. Remaining: Safe sidecar autosave/recovery is already present; do not serialize arbitrarily during panic Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-06"></a>
### CAP-06 — Second render viewport

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED**. Historical task(s): T-910.

Canvas is excluded from detached panes; second viewport not implemented. Remaining: Noncanvas windows already share one AppState and must not be redeveloped Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-07"></a>
### CAP-07 — Point-to-neighbour numeric snapping

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED**. Historical task(s): T-708 / T-902.

No point-to-neighbour snapping path identified. Remaining: Exact Set/Add numerical positions/UV/polygons already exist locally Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-08"></a>
### CAP-08 — Rust fixed-step physics core

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-503 / T-911.

Caller-owned deterministic state/reset, fixed-step accumulator, decay and 10k soak tests. Remaining: Not native TS activation or rendered-export stepping; do not recreate physics core Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-09"></a>
### CAP-09 — Exact-path region PSD refresh

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-302.

Existing unique exact-path region refresh with native atomic Undo/Redo and save/reload tests. Remaining: Added/missing/renamed/shared/mesh cases remain guarded or manual Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-10"></a>
### CAP-10 — Export preflight and consent

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-702 / T-603d.

Existing integrity/fidelity diagnostics, file/collision review, explicit consent and publisher rechecks. Remaining: Not global live document diagnostics; no new filesystem guarantees Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-11"></a>
### CAP-11 — Existing onion ghost generation

**COMPLETE / AUTOMATED / PUBLIC_ALPHA**. Historical task(s): T-703.

Frame/key time generation, ghost passes, selected-bone filtering and off early return. Remaining: Tests cover off/boundaries; not every key-mode/selected-only visual case, theme or performance qualification Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-12"></a>
### CAP-12 — Precise numeric Bezier drafts

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-814 / T-704.

T-814 exact f32 normalized handles, Apply/Cancel and one native command. Remaining: No tied handles; Windows interaction/artistic behavior pending Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-13"></a>
### CAP-13 — Exact numeric geometry drafts

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-815 / T-902.

Scoped mesh/UV/path/polygon Set/Add, Apply/Cancel and context invalidation. Remaining: Not every draggable point, expression or neighbour snapping Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-14"></a>
### CAP-14 — Unknown-author-member safe refusal

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-813 / T-605.

Unsupported/dropped author members or missing image bytes prevent unsafe save. Remaining: Not lossless opaque-author-extra storage or complete hostile-file validation Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-15"></a>
### CAP-15 — Production performance budget

**UNKNOWN / NOT_TESTED / UNKNOWN**. Historical task(s): T-706.

No broad integrated-GPU/500-bone/300-row frame-time acceptance available. Remaining: Historical bounded runtime/fixture runs are not a universal budget Evidence: Exact T-816 source / accepted historical evidence.

<a id="cap-16"></a>
### CAP-16 — Onion theme/opacity controls

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Historical task(s): T-703.

Theme-role past/future tint plus Session opacity/falloff controls. Remaining: Transient controls; real theme contrast/scrubbing pending. Evidence: T-824.

## Context and recent missions — excluded from 101

<a id="t-804"></a>
### T-804 — Historical publication provenance

Original context ID, not an extra historical task. Retained provenance; no new public or manual qualification.

<a id="t-812"></a>
### T-812 — Exact public alpha publication

Original context ID, not an extra historical task. Retained provenance; no new public or manual qualification.

<a id="t-817"></a>
### T-817 — Original101-task reconciliation

Original context ID, not an extra historical task. Retained provenance; no new public or manual qualification.

<a id="t-818"></a>
### T-818 — Global document diagnostics

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Integrity rules, debounced badge/list, exact stale-safe selection; no automatic camera framing, synchronous scans. Historical tasks: T-702. Commit `6e139b6a5deb65fa6922704fc7bb3127f2a21cde`. Accepted historical checkpoint: 1636 passes /4 ignored, not rerun by T-832.

<a id="t-819"></a>
### T-819 — Safe region rig transfer

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Explicit reviewed cross-project copy with image bytes, internal IK/Transform and bone keys; no move/universal mesh transfer. Historical tasks: T-909, T-209. Commit `90dbb71cc5daec8a0cc89aacde865e8eab9a2f85`. Accepted historical checkpoint: 1672 passes /4 ignored, not rerun by T-832.

<a id="t-820"></a>
### T-820 — Linked Bezier tangents

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Broken/Aligned/Mirrored at one compatible scalar junction, numeric and drag; raw wrap/nonrepresentable cases refuse. Historical tasks: T-704, T-912. Commit `fad9150d04d81438ef177b96dd53f793ea11e3bb`. Accepted historical checkpoint: 1716 passes /4 ignored, not rerun by T-832.

<a id="t-821"></a>
### T-821 — Existing FFD key editing

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Exact owned-mesh single-key time/drag/delete/interpolation; vector clipboard, shared/linked ambiguity and topology migration excluded. Historical tasks: T-404, T-209. Commit `73ba3c010eee5f8aef06100658152618aec6a8c6`. Accepted historical checkpoint: 1752 passes /4 ignored, not rerun by T-832.

<a id="t-822"></a>
### T-822 — Explicit PSD path mapping

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. One uniquely missing/added simple-region path with reviewed batch; no bulk/fuzzy or automatic add/delete. Historical tasks: T-302. Commit `c109720f97de670fa8c6c715f5b932a8d2d8dda2`. Accepted historical checkpoint: 1789 passes /4 ignored, not rerun by T-832.

<a id="t-823"></a>
### T-823 — Safe weighted retrace

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Prepared nearest-row remap preserving valid binds, deterministic limits/refusals; FFD-dependent retrace refuses. Historical tasks: T-402. Commit `2718e3fb36d6fa9cf47f5f48e38e0751c910f5c9`. Accepted historical checkpoint: 1827 passes /4 ignored, not rerun by T-832.

<a id="t-824"></a>
### T-824 — Onion theme/opacity and stepping

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Theme roles, opacity/falloff, frame/key bounds, selected-only and zero-evaluation guards; session controls, no stateful physics ghosts. Historical tasks: T-703. Commit `ebd6c6a9eb8170b5458678a55354c6942a19832b`. Accepted historical checkpoint: 1858 passes /4 ignored, not rerun by T-832.

<a id="t-825"></a>
### T-825 — Graph marquee

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Visible scalar centres, replace/Ctrl union and stale cancellation using existing shared selection. Historical tasks: T-704, T-912. Commit `e57bcb4a25443c7dec6f55fc68c54ab4530b60f0`. Accepted historical checkpoint: 1889 passes /4 ignored, not rerun by T-832.

<a id="t-826"></a>
### T-826 — Normalized scalar overlay

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. View-only relative ranges, constants, overshoot notices, Raw return; editing stays in Raw. Historical tasks: T-704, T-912. Commit `074c683125ec7ef0bb50addb20487aa6220d4b44`. Accepted historical checkpoint: 1920 passes /4 ignored, not rerun by T-832.

<a id="t-827"></a>
### T-827 — Large-selection validation

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Indexed native capture with full typed equality and unchanged semantics; no universal edit/display budget. Historical tasks: T-706, T-209, T-704, T-912. Commit `b3af4e802f7ad96bbc8d67b8b33ff96717a3d1ef`. Accepted historical checkpoint: 1947 passes /4 ignored, not rerun by T-832.

<a id="t-828"></a>
### T-828 — FPS native history

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Transport draft/commit and SetProjectFps, gesture bounds, Undo/Redo, dirty/save/recovery; no retime. Historical tasks: T-107, T-202. Commit `e039e4ad275f810783bfedb4bfe5f1dd17592b4c`. Accepted historical checkpoint: 1973 passes /4 ignored, not rerun by T-832.

<a id="t-829"></a>
### T-829 — Large-selection highlighting

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Transient typed membership and idle-handle gates; unchanged native selection and epaint logical outputs. Historical tasks: T-706, T-704, T-912. Commit `21b58bcf8e82efccb6559e40671b7d1bb5d73b43`. Accepted historical checkpoint: 1991 passes /4 ignored, not rerun by T-832.

<a id="t-830"></a>
### T-830 — Numeric whole-clip retime

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Explicit Prepare/Apply with scale/frame offset, strict collisions/stale guards and excluded-metadata consent; no selection-only mode. Historical tasks: T-211. Commit `12252e476b95f9d2ac250d5d0746c589c10269b3`. Accepted historical checkpoint: 2015 passes /4 ignored, not rerun by T-832.

<a id="t-831"></a>
### T-831 — Native PathPosition

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded mission. Exact constraint timeline, native evaluation/authoring/history/persistence, data-export refusal; no PathSpacing/Graph/runtime support. Historical tasks: T-502, T-605, T-603c. Commit `4e4d50b0aa5b7ac160b204540afa878f66fa4d61`. Accepted historical checkpoint: 2058 passes /4 ignored, not rerun by T-832.

## Findings B01 / B02 / B03

B01 region-transfer subset addressed by T-819, advanced transfer not complete. B02 transport FPS bypass fixed locally by T-828; no statement about every metadata write. B03 bone-name context-menu usability remains an inherited physical observation awaiting targeted reproduction, with no new GUI result. No new demonstrated P0 corruption/crash/security issue; format exchange/release policy is a gate, not a fabricated incident.

## T-833 — Selected-bone key offset (local addendum)

Implemented destructive key-time offset for explicitly selected BoneIds, seven direct transform channels, unchanged duration and strict per-track collision refusal. Pure Prepare, leased Apply, consent for safe clamp/outside keys, one native Undo. No selection fallback. See [contract](selected-bone-key-offset.md). Historical 101 categories unchanged; T-211 remains PARTIAL. Windows qualification PENDING MANUAL REVIEW.

## T-834 — Safe FFD Key Clipboard (local addendum)

Same-project, exact unique owner/mesh only; rigid per-vertex and weighted per-influence vectors retained by value. Explicit existing nonempty Deform row, playhead placement, preserved interpolation, full topology/Setup/bind equality and strict collision refusal. Pure Copy/preview, leased consented Apply, one native Undo and exact Save/reopen. Linked/shared/overridden/sequence meshes and cross-project retargeting remain refused. Loader topology migration is still separate T-404 debt. Manual Windows qualification remains pending.

## T-835 local animation ordering evidence

The [manager guide](animation-manager.md) describes native author order, adjacent swaps, persistence, identity-preserving interleaved history and stale menu refusal. Automated in-memory egui/command/Save/Reload/CPU/export checks exercise the implemented scope; no Windows interaction was run. T-208 remains PARTIAL for its original frame-display and manager-mode-selection criteria, rather than promoting from the shorthand T-832 reorder remainder alone. The frozen 101-row T-832 CSV and all 101 anchors remain preserved; counts stay 62/33/1/3/2. T-209, T-404 and T-502 are unchanged.

</details>
