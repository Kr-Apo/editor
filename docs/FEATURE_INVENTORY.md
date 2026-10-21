# Feature inventory — exact T-816 product, T-817 documentary audit

## Local T-827 update — 2026-10-21

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for native large-key selection
validation. KeyClip remains the authority: larger requests use operation-local
full-identity deduplication, timeline/name indexes and lazy per-track resolution.
Every distinct key still passes native value/time/interpolation checks, in the
original request order. First occurrences, exact errors, session duplicates,
scalar invalid-set clearing and FFD refusal policies remain unchanged. Small
requests retain the original path; this is an algorithm choice, not a key limit.
No persistent cache, document edit, dirty revision, Undo entry or Redo loss.

Paired debug CPU observations and exact T-826 comparisons demonstrate the scoped
gain; local delivery `outputs/T827-Key-Selection-Performance/` records methods and
raw results. This does **not** certify interactive responsiveness: graph/sheet
linear membership highlighting, native move/resolve scans and other consumer
costs remain. T-825 gestures and T-826 Raw/Normalize View Only are unchanged.
T-704/T-912 remain PARTIAL. Cumulative T-813–T-827 desktop checks remain
**PENDING MANUAL REVIEW**; no GUI/GPU/inference or public operation. Earlier notes
retain their historical scope, including the now-addressed native capture cost.


## Local T-826 update — 2026-10-21

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for the bounded **Normalize View
Only** scalar overlay. Raw remains the default native-unit editor. Normalize
uses each displayed channel's finite authored keys across the whole active clip,
with f64 intermediates, a constant/single-key level of 0.5 and fixed relative
Y ticks 0 / 0.5 / 1. Key points, sampled curves and selection share the projection.
Bezier overshoots remain unclamped relative values and may leave the comparison
frame. Hover the relative-mode badge for native target/unit/range legends.
Graph key/time drags, handles and graph-local K insertion are disabled in this
mode; switch to Raw to edit. Click, Ctrl-click and T-825 replace/Ctrl-union marquee
remain available. Switching modes invalidates pending captures, even with equal
bounds. The option follows clips/projects in the same egui session, resets on
application restart, and is never stored in .ankh or Config.
See [Normalize contract](graph-editor.md#8-normalized-comparison-view-t-826).
T-704/T-912 remain PARTIAL: normalized editing and broader desktop qualification
are not delivered. Existing quadratic large native selection validation remains
a separate future scoped task; normalization adds no selection validation loop.
Cumulative T-813–T-826 Windows tests remain **PENDING MANUAL REVIEW**. Older notes
retain their historical scope. No GUI, GPU, inference or public operation ran.


## Local T-823 update — 2026-10-21

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for safe retracing of compatible
owned meshes. [Safe mesh retracing](safe-mesh-retracing.md) reuses the existing
tracer and copies nearest source weight rows in slot-bone-local Setup space;
exact ties use the lowest source index. No silent pruning, normalization or FFD
remapping. All source BoneIds must remain represented. Inline/track FFD, linked
or ambiguous ownership, pinned edges, invalid/noncanonical binds and unsupported
weighted UV frames refuse. T-402 stays PARTIAL; nearest-neighbour is not artistic
fidelity. Cumulative T-813–T-823 manual qualification remains deferred. Prior
checkpoint sections retain their historical scope. No runtime, renderer, format,
writer, version or public-release change; no next phase is authorized.

## Local T-822 update — 2026-10-20

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for one explicit missing-image
→ added-layer mapping on a unique simple region. No default pairing or fuzzy
matching. See [PSD path resolution](psd-path-resolution.md). The existing exact-path
batch is disclosed and remains all-or-nothing. Other missing image provenance,
shared/overridden/mesh/linked/FFD/sequence/flattened targets and semantic tag changes
refuse. T-302 stays PARTIAL. T-813–T-822 manual qualification is deferred; this
local checkpoint does not certify Windows interaction, art or Photoshop effects.
Earlier checkpoint sections retain their historical scope. No public operation,
runtime, renderer, format, writer or version change.

## Local T-821 update — 2026-10-20

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for editing one existing owned
Deform key in the Dopesheet: select, move time, delete and native interpolation.
See [FFD key editing](ffd-key-editing.md). Weighted arrays use influence count;
linked/shared sources, ambiguous skins and invalid topology refuse. No vector
clipboard, graph averaging or T-820 scalar tangent linking. T-404/T-209 remain
PARTIAL. Cumulative T-813–T-821 manual qualification is deferred, not certified.
No runtime, sampler, format, version or public release change. Earlier sections
below describe their historical checkpoint scope. T-822 is not started.


## Local T-820 update — 2026-10-20

**T-820: COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for bounded single-junction
Bezier linking. Broken/Aligned/Mirrored are session editing modes, not new project
metadata. See [contract](bezier-tangent-linking.md). T-704/T-912 remain PARTIAL:
normalized overlay, graph marquee and broader manual qualification remain open.
T-813–T-820 manual Windows qualification is deferred; no GUI/GPU/inference run.
This is not a public release, and no subsequent development task is authorized.
Older checkpoint sections below retain their historical scope.


## Local T-819 update — 2026-10-20

**LOCAL_CANDIDATE / PENDING_MANUAL**. Explicit reviewed cross-project region
subtree transfer is now implemented locally; same-document clipboard behavior
and File/Open remain unchanged. Read [Cross-project transfer](cross-project-transfer.md).
T-909 remains PARTIAL for advanced types and retargeting. No GUI, inference,
runtime change, publication or B02/B03 correction. Earlier checkpoint notes below
describe their historical scopes; T-818/T-819 are not public releases.


## Local T-818 update — 2026-10-20

T-818 is a **LOCAL_CANDIDATE / PENDING_MANUAL** refinement of the T-817
inventory. The prior T-817 audit below remains historical evidence; it is no
longer a statement that global diagnostics are absent in this local candidate.
Read [Document diagnostics](document-diagnostics.md) for the bounded contract.
No GUI/inference test, publication, runtime change or B01/B02/B03 correction.


Audit date: **2026-10-20**. Product source frozen at T-816
`009e67898317d779d943c23a0210bd48ed9d63bd`. T-817 changes documentation only.
Public editor: `9233b8e888c811e7c71300101f865a9e74b3f489`; public external runtime:
`c4c38df269cfcb30f941221454b7202f40d3fa5b`; released tag **v0.2.0-alpha.1**.
The exact T-811 archives were published by T-812 on 2026-10-19. Historical candidate
labels in FEATURES/RELEASE_READINESS, old alpha-scope and docs/alpha/ do not override
that publication record. They also do not qualify the newer local candidates.
T-813–T-816: **PENDING MANUAL REVIEW**. No GUI, inference, browser or GPU run in T-817.

## Reading this inventory

136 **entries**, consisting of 108 ID records (101 historical
IDs plus T-804/T-812/T-817/T-818/T-819/T-820/T-821 context) and 28 linked scope refinements.
These are not 136 independent nonoverlapping product features. Aggregates
and their refinements overlap; counts are always reported separately.

Implementation: **COMPLETE** = the explicitly bounded functional scope is implemented;
**PARTIAL** = usable subset with original criteria left; **ABSENT** = no implementation
path found; **SUPERSEDED** = named historical design replaced/renumbered;
**DEFERRED** = intentionally postponed; **UNKNOWN** = insufficient evidence.

Validation: **NOT_TESTED**, **AUTOMATED**, **MANUAL**, **MIXED**, **PENDING_MANUAL**.
PENDING_MANUAL retains automated evidence and means the local GUI gate is still
open, never that the functionality is absent. MIXED may combine automated tests
with a historical maintainer report for the specified binary/scenario; it is not
a new manual qualification of T-816. No row claims a universal GUI certificate.

Availability: **PUBLIC_ALPHA**, **LOCAL_CANDIDATE**, **SOURCE_ONLY**,
**NOT_DISTRIBUTED**, **UNKNOWN**. PUBLIC_ALPHA means present in the exact released
artifact scope, not every original acceptance criterion. An existing public base
can have a local-only refinement; the local-delta note identifies that boundary.
SOURCE_ONLY includes the separately built MCP server and optional community formats,
which are absent from the portable Windows ZIP. Internal version strings are not
artifact provenance.

Code and test links were checked against the audited local trees. Runtime links
are pinned to the protected commit, inspected locally; remote page availability
is not a substitute for inspection. Test references name suites, not universal
coverage. T-816's executed 1596/4 baseline and T-817 rerun results are distinguished
in the delivery VALIDATION.json. That T-817 documentary audit changed no tests, thresholds or fixtures. T-818 adds dedicated tests without weakening historical oracles.

The original acceptance text and all historical references remain byte-exact in
[TASKS-before-T817.md](TASKS-before-T817.md). That root location deliberately keeps
its original relative links valid. The archive is historical/normative context,
not the current availability matrix. The current [tasks](TASKS.md),
[next roadmap](ROADMAP_NEXT.md), and [manual register](MANUAL_QA_PENDING.md) are operational.

## Find a family

- [Foundations](#foundations)
- [Rigging](#rigging)
- [Animation](#animation)
- [Meshes](#meshes)
- [Assets / PSD](#assets-psd)
- [Formats / exports](#formats-exports)
- [PixiJS / Phaser](#pixijs-phaser)
- [IA](#ia)
- [UX / tools](#ux-tools)
- [Distribution](#distribution)

<a id="foundations"></a>
## Foundations

<a id="t-001"></a>
### T-001 — License + repo hygiene

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-001.

- Delivered scope: Dual MIT/Apache licensing, attribution and repository/toolchain setup
- Entry point: README / CONTRIBUTING.
- Implementation: [`LICENSE-MIT`](../LICENSE-MIT), [`LICENSE-APACHE`](../LICENSE-APACHE), [`Cargo.toml`](../Cargo.toml).
- Test/reference suites: [`Cargo.toml`](../Cargo.toml)
- Contradictory check / remaining historical criteria: Not a blanket audit of all transitive dependency licenses
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-002"></a>
### T-002 — CI skeleton

**PARTIAL / NOT_TESTED / SOURCE_ONLY** · Origin: T-002.

- Delivered scope: Linux/Windows test/fmt/Clippy and core WASM workflow definitions
- Entry point: CI source.
- Implementation: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml).
- Test/reference suites: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)
- Contradictory check / remaining historical criteria: Actions disabled; no fresh remote green run; actual Clippy job lacks historical -D warnings
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-003"></a>
### T-003 — ADR seed

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-003.

- Delivered scope: Accepted architectural decision records and source references
- Entry point: ADR documents.
- Implementation: [`docs/adr/0001-slotmap-ids.md`](../docs/adr/0001-slotmap-ids.md), [`docs/adr/0007-physics-determinism.md`](../docs/adr/0007-physics-determinism.md).
- Test/reference suites: [`docs/ARCHITECTURE_PLAN.md`](../docs/ARCHITECTURE_PLAN.md)
- Contradictory check / remaining historical criteria: Some historical format statements are superseded by format-spec; not every old design sketch is current
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-101"></a>
### T-101 — Stable entity IDs

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-101.

- Delivered scope: Slotmap entity identity, topological order and native Undo ID remapping
- Entry point: Native document operators.
- Implementation: [`core/src/ids.rs`](../core/src/ids.rs), [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Test/reference suites: [`core/src/skeleton.rs`](../core/src/skeleton.rs), [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `delete_middle_bone_keeps_identity_and_reparents`; `delete_bone_undo_restores_it_and_readopts_children`.
- Contradictory check / remaining historical criteria: Names cross disk boundaries; unresolved foreign references still need diagnostics
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-102"></a>
### T-102 — Affine2 transform math

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-102.

- Delivered scope: Native 2D affine composition and inherited transforms
- Entry point: FK / transform fields.
- Implementation: [`core/src/transforms.rs`](../core/src/transforms.rs).
- Test/reference suites: [`core/src/transforms.rs`](../core/src/transforms.rs)
- Example test symbols (suite references, not a coverage claim): `shear_axes_follow_the_spine_convention`.
- Contradictory check / remaining historical criteria: Singular transforms remain noninvertible; no universal arbitrary-float geometry guarantee
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-103"></a>
### T-103 — Pose extraction + evaluate skeleton

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-103.

- Delivered scope: Deterministic finite-input pose evaluation with derived worlds
- Entry point: refresh_pose.
- Implementation: [`core/src/pose.rs`](../core/src/pose.rs), [`editor/src/app_state.rs`](../editor/src/app_state.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `evaluate_is_deterministic`.
- Contradictory check / remaining historical criteria: Stateful physics requires its separate accumulator; direct seek is not a simulation
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-104"></a>
### T-104 — Constraint system + correct IK blending

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-104.

- Delivered scope: Ordered IK, shortest-angle local mixing and 1/2/3+ solvers
- Entry point: Properties / Add driver.
- Implementation: [`core/src/constraints.rs`](../core/src/constraints.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs), [`core/src/constraints.rs`](../core/src/constraints.rs)
- Example test symbols (suite references, not a coverage claim): `ik_mix_across_pi_boundary_does_not_flip`.
- Contradictory check / remaining historical criteria: Runtime common alpha excludes constraints; geometric singularities are not repaired
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-105"></a>
### T-105 — Skin layer

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-105.

- Delivered scope: Named skins and default fallback resolution
- Entry point: Skins.
- Implementation: [`core/src/skin.rs`](../core/src/skin.rs), [`core/src/skeleton.rs`](../core/src/skeleton.rs).
- Test/reference suites: [`core/src/skin.rs`](../core/src/skin.rs), [`core/src/skeleton.rs`](../core/src/skeleton.rs)
- Example test symbols (suite references, not a coverage claim): `new_skeleton_has_a_default_skin`.
- Contradictory check / remaining historical criteria: Not arbitrary game-runtime skin composition or linked deformation qualification
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-106"></a>
### T-106 — Animation model + sampling

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-106.

- Delivered scope: Typed scalar/slot/deform tracks, Linear/Stepped/Bezier for valid authored inputs
- Entry point: Timeline / evaluation.
- Implementation: [`core/src/animation.rs`](../core/src/animation.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/animation.rs`](../core/src/animation.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `empty_timeline_samples_to_none`; `slot_attachment_is_stepped_and_highest_alpha_wins`.
- Contradictory check / remaining historical criteria: Extreme finite handles can overflow sampler; no universal f32-range guarantee
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-107"></a>
### T-107 — Editor Document/Session split + command undo

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-107.

- Delivered scope: Native document commands, Undo/Redo and separate session state
- Entry point: Edit Undo/Redo.
- Implementation: [`document/src/doc.rs`](../document/src/doc.rs), [`document/src/commands/mod.rs`](../document/src/commands/mod.rs), [`editor/src/session.rs`](../editor/src/session.rs).
- Test/reference suites: [`document/src/commands/mod.rs`](../document/src/commands/mod.rs), [`editor/src/app_state.rs`](../editor/src/app_state.rs)
- Example test symbols (suite references, not a coverage claim): `undo_redo_roundtrips_a_create`; `ease_in_out_changes_motion_and_undo_reverts`.
- Contradictory check / remaining historical criteria: Undo history is not persisted; not every historical UI metadata assignment uses a command (B02)
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-108"></a>
### T-108 — `formats` crate + .ankh v1 save/load

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-108.

- Delivered scope: Author v1 ANKH envelope, MessagePack and readable/compact JSON profiles
- Entry point: File Open/Save.
- Implementation: [`formats/src/lib.rs`](../formats/src/lib.rs), [`formats/src/container.rs`](../formats/src/container.rs), [`formats/src/convert.rs`](../formats/src/convert.rs).
- Test/reference suites: [`formats/tests/reliability.rs`](../formats/tests/reliability.rs), [`formats/tests/migrations.rs`](../formats/tests/migrations.rs)
- Example test symbols (suite references, not a coverage claim): `t813_missing_image_save_refuses_without_changing_primary_or_save_as`; `authentic_v1_fixtures_save_reopen_all_profiles_without_touching_original_assets`.
- Contradictory check / remaining historical criteria: Prototype ZIP unsupported; no authentic v0 migration; complete scope is current v1 only
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-707"></a>
### T-707 — Crash recovery, update check, feedback

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-707.

- Delivered scope: Safe save, sidecar recovery, candidate checks and recovery offers
- Entry point: Recovery prompt / File lifecycle.
- Implementation: [`editor/src/autosave.rs`](../editor/src/autosave.rs), [`editor/src/app.rs`](../editor/src/app.rs), [`formats/src/safe_write.rs`](../formats/src/safe_write.rs).
- Test/reference suites: [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs), [`formats/tests/reliability.rs`](../formats/tests/reliability.rs)
- Example test symbols (suite references, not a coverage claim): `close_waits_for_png_cancellation_before_issuing_the_close_command`; `t813_missing_image_save_refuses_without_changing_primary_or_save_as`.
- Contradictory check / remaining historical criteria: No panic-time live-document dump, recovery browser or complete update/feedback package
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-813"></a>
### T-813 — Format Compatibility & Project Reliability

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-813.

- Delivered scope: Local unknown/lossy input and missing-byte save refusal; recovery hardening
- Entry point: Local File / recovery.
- Implementation: [`formats/src/compatibility.rs`](../formats/src/compatibility.rs), [`formats/src/lib.rs`](../formats/src/lib.rs), [`editor/src/app.rs`](../editor/src/app.rs).
- Test/reference suites: [`formats/tests/reliability.rs`](../formats/tests/reliability.rs), [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t813_missing_image_save_refuses_without_changing_primary_or_save_as`; `close_waits_for_png_cancellation_before_issuing_the_close_command`.
- Contradictory check / remaining historical criteria: No arbitrary structural repair, hostile-resource budget or power-loss certificate; GUI pending
- Latest relevant checkpoint: T-813.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Qualification queue only; do not redevelop.

<a id="cap-05"></a>
### CAP-05 — Panic-time emergency project dump

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-707.

- Delivered scope: No panic-time in-memory project dump/recovery package identified
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/main.rs`](../editor/src/main.rs), [`editor/src/autosave.rs`](../editor/src/autosave.rs).
- Test/reference suites: No dedicated executed test identified for this absent/unknown scope; static search only.
- Contradictory check / remaining historical criteria: Safe sidecar autosave/recovery is already present; do not serialize arbitrarily during panic
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="rigging"></a>
## Rigging

<a id="t-206"></a>
### T-206 — Hierarchy panel UX

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-206.

- Delivered scope: Hierarchy, folders, reparenting and native rename/delete
- Entry point: Hierarchy.
- Implementation: [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs), [`document/src/commands/group_cmds.rs`](../document/src/commands/group_cmds.rs).
- Test/reference suites: [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs), [`document/src/commands/group_cmds.rs`](../document/src/commands/group_cmds.rs), [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `reparent_refuses_to_create_a_cycle`.
- Contradictory check / remaining historical criteria: Original hierarchy/reparent/lock scope exists; separate bone-name context-menu usability issue B03 remains, not an absent reparent system.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-307"></a>
### T-307 — Attachment inspector — per-attachment properties

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-307.

- Delivered scope: Original region attachment property/action and on-canvas authoring scope, via native commands.
- Entry point: Properties / Slot Editor.
- Implementation: [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs), [`document/src/commands/attachment_cmds.rs`](../document/src/commands/attachment_cmds.rs).
- Test/reference suites: [`editor/src/ui/inspector/image_tests.rs`](../editor/src/ui/inspector/image_tests.rs), [`document/src/commands/attachment_cmds.rs`](../document/src/commands/attachment_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `skin_image_inherited_region_creates_an_override_and_undo_restores_fallback`; `renaming_an_attachment_leaves_its_texture_alone`.
- Contradictory check / remaining historical criteria: Mesh/sequence image replacement is a separate unimplemented extension; not an absent region inspector.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-501"></a>
### T-501 — Transform constraint

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-501.

- Delivered scope: Native transform constraints and mixes
- Entry point: Add driver / Transform.
- Implementation: [`core/src/pose.rs`](../core/src/pose.rs), [`document/src/commands/constraint_cmds.rs`](../document/src/commands/constraint_cmds.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs), [`document/src/commands/constraint_cmds.rs`](../document/src/commands/constraint_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `a_rotation_constraint_at_half_mix_lands_halfway`.
- Contradictory check / remaining historical criteria: Not part of common Pixi/Phaser alpha contract
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-502"></a>
### T-502 — Path attachment + path constraint

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-502.

- Delivered scope: Native path geometry/constraint and exact local point editing
- Entry point: Path attachment / constraint.
- Implementation: [`core/src/path.rs`](../core/src/path.rs), [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/path.rs`](../core/src/path.rs), [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs)
- Example test symbols (suite references, not a coverage claim): `a_closed_path_wraps_instead_of_clamping`; `t815_path_bounding_box_and_clip_use_existing_commands_atomically`.
- Contradictory check / remaining historical criteria: Static polyline paths/constraints exist; Bezier path handles and PathPosition/PathSpacing authoring timelines remain explicitly deferred; runtime junction/fallback parity excluded.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-503"></a>
### T-503 — Physics constraint

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-503.

- Delivered scope: Rust caller-owned fixed-step physics state, reset and deterministic advance
- Entry point: Add driver / Physics.
- Implementation: [`core/src/physics.rs`](../core/src/physics.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/physics.rs`](../core/src/physics.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `the_same_dt_sequence_produces_the_same_trajectory`.
- Contradictory check / remaining historical criteria: Rust fixed-step contract is complete in CAP-08; original export/playback parity not fulfilled: native TS physics activation incompatible and rendered exports explicitly refuse it.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-504"></a>
### T-504 — IK completeness

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-504.

- Delivered scope: Aim/analytic/FABRIK, bend, softness/stretch, 3+ stiffness and target creation
- Entry point: Add driver / IK.
- Implementation: [`core/src/constraints.rs`](../core/src/constraints.rs), [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs), [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs)
- Example test symbols (suite references, not a coverage claim): `ik_mix_across_pi_boundary_does_not_flip`; `t816_ik_inspector_exposes_root_span_and_solver`.
- Contradictory check / remaining historical criteria: No unique-solution guarantee or new IK key dots; runtime IK excluded
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-505"></a>
### T-505 — Slot & bone presentation depth

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-505.

- Delivered scope: Bone/slot colors, visibility, locks and native static dark tint
- Entry point: Hierarchy / Properties.
- Implementation: [`core/src/slot.rs`](../core/src/slot.rs), [`core/src/skeleton.rs`](../core/src/skeleton.rs), [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs), [`editor/src/session.rs`](../editor/src/session.rs)
- Example test symbols (suite references, not a coverage claim): `slot_color_blends_toward_the_keyed_value`; `both_layers_start_visible`.
- Contradictory check / remaining historical criteria: Not every presentation variant or animated dark tint; viewport acceptance incomplete
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-506"></a>
### T-506 — Event timeline

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-506.

- Delivered scope: Typed event authoring and native payload export
- Entry point: Events lane.
- Implementation: [`core/src/animation.rs`](../core/src/animation.rs), [`editor/src/ui/timeline/events.rs`](../editor/src/ui/timeline/events.rs), [`document/src/commands/event_cmds.rs`](../document/src/commands/event_cmds.rs).
- Test/reference suites: [`document/src/commands/event_cmds.rs`](../document/src/commands/event_cmds.rs), [`core/src/animation.rs`](../core/src/animation.rs)
- Example test symbols (suite references, not a coverage claim): `events_are_kept_in_time_order`; `an_event_fires_once_in_the_step_that_crosses_it`.
- Contradictory check / remaining historical criteria: t=0 ignored by current runtime policy; audio metadata is not audio playback
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-507"></a>
### T-507 — Skin manager + multi-skin composition

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-507.

- Delivered scope: Named skins, default fallback and editor composition controls
- Entry point: Skins.
- Implementation: [`core/src/skin.rs`](../core/src/skin.rs), [`editor/src/ui/skins.rs`](../editor/src/ui/skins.rs), [`document/src/commands/skin_cmds.rs`](../document/src/commands/skin_cmds.rs).
- Test/reference suites: [`core/src/skin.rs`](../core/src/skin.rs), [`document/src/commands/skin_cmds.rs`](../document/src/commands/skin_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `a_duplicate_skin_name_is_refused`.
- Contradictory check / remaining historical criteria: Game adapters use one compatible skin; arbitrary multi-skin/mesh-image workflows unqualified
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-508"></a>
### T-508 — Former performance task renumbered into T-706

**SUPERSEDED / AUTOMATED / SOURCE_ONLY** · Origin: T-508.

- Delivered scope: Former performance task renumbered into T-706
- Entry point: Renumbered T-706.
- Implementation: [`docs/TASKS-before-T817.md`](../docs/TASKS-before-T817.md), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `evaluate_is_deterministic`.
- Contradictory check / remaining historical criteria: Historical alias only, not another performance implementation
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-816"></a>
### T-816 — Rigging Discoverability & First-Run Experience

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-816.

- Delivered scope: IK chain/span/solver diagnostics, brush target fix and unchanged Tentacle discovery example
- Entry point: Local IK/Weights discovery.
- Implementation: [`editor/src/ui/inspector/ik_info.rs`](../editor/src/ui/inspector/ik_info.rs), [`editor/src/ui/weights.rs`](../editor/src/ui/weights.rs).
- Test/reference suites: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs), [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t816_ik_inspector_exposes_root_span_and_solver`; `t816_selecting_influence_keeps_mesh_target_via_real_egui_row`.
- Contradictory check / remaining historical criteria: Artist discovery exercise pending; no new solver/runtime capability
- Latest relevant checkpoint: T-816.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Qualification queue only; do not redevelop.

<a id="t-904"></a>
### T-904 — Saved selection sets (not hierarchy folders)

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-904.

- Delivered scope: Hierarchy folders exist, but no saved overlapping selection-set model/schema/panel found
- Entry point: No saved set panel/model found.
- Implementation: [`core/src/skeleton.rs`](../core/src/skeleton.rs), [`formats/src/schema.rs`](../formats/src/schema.rs), [`editor/src/ui/tree.rs`](../editor/src/ui/tree.rs).
- Test/reference suites: [`core/src/skeleton.rs`](../core/src/skeleton.rs)
- Example test symbols (suite references, not a coverage claim): `group_membership_is_exclusive`.
- Contradictory check / remaining historical criteria: Stale SelectionSet comments are not an implementation; new optional schema needed if later authorized
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P3 extension, not next core workflow.

<a id="t-907"></a>
### T-907 — Bone length edit carries its children

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-907.

- Delivered scope: Length edit carries tip children; Alt preserves positions; one native Undo
- Entry point: Properties Length / Alt.
- Implementation: [`editor/src/ui/inspector.rs`](../editor/src/ui/inspector.rs), [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Test/reference suites: [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `setting_a_length_carries_the_child_at_the_tip`.
- Contradictory check / remaining historical criteria: Remembered carry preference/full canvas-length interaction not identified
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-908"></a>
### T-908 — Make the existing advantages discoverable

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-908.

- Delivered scope: T-816 explains existing IK/brush and validates an 8-bone source sample
- Entry point: First-rig exercise.
- Implementation: [`docs/rigging-capabilities.md`](../docs/rigging-capabilities.md), [`docs/first-rig-in-10-minutes.md`](../docs/first-rig-in-10-minutes.md).
- Test/reference suites: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs), [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t816_ik_inspector_exposes_root_span_and_solver`; `t816_selecting_influence_keeps_mesh_target_via_real_egui_row`.
- Contradictory check / remaining historical criteria: New user's no-source exercise pending; not an absent IK solver/weight brush
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-909"></a>
### T-909 — Rig transfer between skeletons

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-909 / T-819.

- Prior PUBLIC_ALPHA scope: same-document subtree Copy/Paste, slots, internal constraints and bone tracks remains unchanged.
- Local T-819 scope: explicit source-project reader, region/sequence image bytes, deterministic collision/reuse preview, unique new clips and one validated native transaction.
- Entry: File → Import from Ankhimate Project… / Prepare / Confirm reviewed transfer.
- Implementation: [`document/src/transfer.rs`](../document/src/transfer.rs), [`editor/src/transfer.rs`](../editor/src/transfer.rs), shared [`clipboard.rs`](../document/src/clipboard.rs) and native commands.
- Tests: [`document/src/transfer/tests.rs`](../document/src/transfer/tests.rs), [`editor/src/transfer/tests.rs`](../editor/src/transfer/tests.rs); historical Copy/Paste suites retained.
- Limits: default-skin regions/sequences, local coordinates, internal IK/Transform and bone keys only. Mesh/linked/FFD/clipping/path/physics, relevant alternate skins/external constraints or unsupported tracks refuse. No arbitrary timeline fusion, world-pose retargeting or global project merge.
- Latest checkpoint: T-819 local candidate, parent exact T-818; GUI/manual campaign remains pending.
- Next: review the bounded delivery; do not recreate existing clipboard/key/pose work.

<a id="t-911"></a>
### T-911 — Physics stability harness

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-911.

- Delivered scope: 10k-step soak, energy decay and 30/60/144 physics tests; velocity API
- Entry point: Physics stability tests.
- Implementation: [`core/src/physics.rs`](../core/src/physics.rs), [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs).
- Test/reference suites: [`core/src/physics.rs`](../core/src/physics.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `the_same_dt_sequence_produces_the_same_trajectory`.
- Contradictory check / remaining historical criteria: No complete debug overlay or broad unrelated-edit interaction qualification
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="cap-08"></a>
### CAP-08 — Rust fixed-step physics core

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-503 / T-911.

- Delivered scope: Caller-owned deterministic state/reset, fixed-step accumulator, decay and 10k soak tests
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`core/src/physics.rs`](../core/src/physics.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`core/src/physics.rs`](../core/src/physics.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `the_same_dt_sequence_produces_the_same_trajectory`; `evaluate_is_deterministic`.
- Contradictory check / remaining historical criteria: Not native TS activation or rendered-export stepping; do not recreate physics core
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="animation"></a>
## Animation

<a id="t-201"></a>
### T-201 — Dopesheet panel

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-201.

- Delivered scope: Original bone/slot scalar dopesheet, synced rows/ruler, selection and native key interactions.
- Entry point: View Dopesheet.
- Implementation: [`editor/src/ui/timeline/sheet.rs`](../editor/src/ui/timeline/sheet.rs), [`editor/src/ui/timeline/model.rs`](../editor/src/ui/timeline/model.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs), [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `t814_dopesheet_selection_reaches_numeric_inspector`; `duplicate_is_a_deep_copy`.
- Contradictory check / remaining historical criteria: Complex track addresses belong to the separately partial T-209/FFD workflow; local T-814 selection refinements still await GUI review.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-202"></a>
### T-202 — Playback + auto-key

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-202.

- Delivered scope: Playback, auto-key and exact setup-return key routing
- Entry point: Transport / auto-key.
- Implementation: [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs), [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Test/reference suites: [`editor/src/edit_router.rs`](../editor/src/edit_router.rs), [`editor/src/app_state.rs`](../editor/src/app_state.rs)
- Example test symbols (suite references, not a coverage claim): `auto_key_off_defers_the_pose_until_k`.
- Contradictory check / remaining historical criteria: FPS direct-mutation finding B02 is not fixed; pending preview is distinct from stored key
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-203"></a>
### T-203 — Bezier interpolation UI

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-203.

- Delivered scope: Independent Bezier presets, handle drags and local precise numeric tangent drafts
- Entry point: Graph / Properties tangents.
- Implementation: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs), [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t814_interpolation_command_cannot_edit_in_setup`.
- Contradictory check / remaining historical criteria: Complete scope excludes linked handles, normalize overlay and graph marquee (T-704)
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-204"></a>
### T-204 — Draw-order animation UI

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-204.

- Delivered scope: Setup draw order and animated offsets through dedicated native surface
- Entry point: Draw Order.
- Implementation: [`editor/src/ui/draw_order.rs`](../editor/src/ui/draw_order.rs), [`document/src/commands/slot_cmds.rs`](../document/src/commands/slot_cmds.rs).
- Test/reference suites: [`document/src/commands/slot_cmds.rs`](../document/src/commands/slot_cmds.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `create_slot_undo_removes_it_from_draw_order_too`; `draw_order_includes_unordered_slots`.
- Contradictory check / remaining historical criteria: Generic dopesheet order-key clipboard remains excluded
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-205"></a>
### T-205 — Slot timelines UI

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-205.

- Delivered scope: Slot light RGBA, visibility and attachment switching
- Entry point: Slot Editor / Properties.
- Implementation: [`editor/src/ui/slot_editor.rs`](../editor/src/ui/slot_editor.rs), [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Test/reference suites: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `inspector_slot_colour_pending_revert_manual_key_and_reset_contract`; `draw_order_includes_unordered_slots`.
- Contradictory check / remaining historical criteria: Static dark tint is separate; no animated-dark channel is invented
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-207"></a>
### T-207 — Setup mode / Animate mode

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-207.

- Delivered scope: Native Setup/Animate structural gates, router and unkeyed previews
- Entry point: Setup / Animate.
- Implementation: [`document/src/work_mode.rs`](../document/src/work_mode.rs), [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Test/reference suites: [`document/src/lib.rs`](../document/src/lib.rs), [`editor/src/edit_router.rs`](../editor/src/edit_router.rs)
- Example test symbols (suite references, not a coverage claim): `a_structural_command_declares_its_mode_here`.
- Contradictory check / remaining historical criteria: Every future edit surface still needs mode semantics; legacy metadata writes audited separately
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-208"></a>
### T-208 — Animation manager

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-208.

- Delivered scope: Create/rename/duplicate/delete clips, duration and loop settings
- Entry point: Animations.
- Implementation: [`editor/src/ui/animations.rs`](../editor/src/ui/animations.rs), [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs).
- Test/reference suites: [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs), [`editor/src/ui/animations.rs`](../editor/src/ui/animations.rs)
- Example test symbols (suite references, not a coverage claim): `duplicate_is_a_deep_copy`.
- Contradictory check / remaining historical criteria: Original animation-reorder UI not clearly qualified; do not recreate manager
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-209"></a>
### T-209 — Clipboard: copy / paste / duplicate

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-209.

- Delivered scope: Bones, atomic multibone poses, owned key blocks and mirrored bone offsets
- Entry point: Edit / Dopesheet menu.
- Implementation: [`document/src/clipboard.rs`](../document/src/clipboard.rs), [`editor/src/key_clipboard.rs`](../editor/src/key_clipboard.rs), [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs).
- Test/reference suites: [`editor/src/ui/timeline/sheet/clipboard_tests.rs`](../editor/src/ui/timeline/sheet/clipboard_tests.rs), [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs)
- Example test symbols (suite references, not a coverage claim): `selected_key_menu_exposes_copy_and_paste`.
- Contradictory check / remaining historical criteria: Complex rows/events excluded; no empty-row destination selection or automatic retargeting
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P1/P2 targeted remainder; see roadmap.

<a id="t-209-2"></a>
### T-209.2 — Validated normal/mirrored multibone pose paste in one native transaction

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-209.2.

- Delivered scope: Validated normal/mirrored multibone pose paste in one native transaction
- Entry point: Paste Pose.
- Implementation: [`editor/src/pose_clipboard.rs`](../editor/src/pose_clipboard.rs), [`document/src/commands/pose_cmds.rs`](../document/src/commands/pose_cmds.rs).
- Test/reference suites: [`editor/src/paste_tests/pose_tests.rs`](../editor/src/paste_tests/pose_tests.rs), [`editor/src/paste_tests/menu_tests.rs`](../editor/src/paste_tests/menu_tests.rs)
- Example test symbols (suite references, not a coverage claim): `four_bone_pose_paste_is_one_undo`; `edit_menu_buttons_invoke_one_pose_transaction`.
- Contradictory check / remaining historical criteria: Auto-key-off retains preview policy; not a retargeter
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-209-3"></a>
### T-209.3 — Explicit bone-row mirrored offsets; signs

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-209.3.

- Delivered scope: Explicit bone-row mirrored offsets; signs, curves, overshoots and relative times preserved
- Entry point: Paste Keys Mirrored (Offsets).
- Implementation: [`document/src/clipboard/keys.rs`](../document/src/clipboard/keys.rs), [`editor/src/key_clipboard.rs`](../editor/src/key_clipboard.rs).
- Test/reference suites: [`editor/src/ui/timeline/sheet/mirrored_clipboard_tests.rs`](../editor/src/ui/timeline/sheet/mirrored_clipboard_tests.rs)
- Example test symbols (suite references, not a coverage claim): `context_menu_offers_mirrored_bone_key_offsets`.
- Contradictory check / remaining historical criteria: Offsets, not asymmetric absolute setup poses; no slot/FFD/event mirror
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-210"></a>
### T-210 — Keyed-value affordances in the inspector

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-210.

- Delivered scope: Per-property key dots, setup/animated reverts and Setup resets
- Entry point: Properties key dots.
- Implementation: [`editor/src/ui/inspector/key_controls.rs`](../editor/src/ui/inspector/key_controls.rs), [`editor/src/edit_router.rs`](../editor/src/edit_router.rs).
- Test/reference suites: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs)
- Example test symbols (suite references, not a coverage claim): `inspector_key_context_actions_are_reachable_by_secondary_click`.
- Contradictory check / remaining historical criteria: Does not add IK key dots or every future property; no whole-app accessibility certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-211"></a>
### T-211 — Pose tools

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-211.

- Delivered scope: Bake pose as setup, reset, fixed retime and clear operations
- Entry point: Pose menu.
- Implementation: [`editor/src/app_state.rs`](../editor/src/app_state.rs), [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs), [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs).
- Test/reference suites: [`editor/src/app_state.rs`](../editor/src/app_state.rs), [`document/src/commands/key_cmds.rs`](../document/src/commands/key_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `set_pose_as_setup_bakes_what_is_on_screen`.
- Contradictory check / remaining historical criteria: Arbitrary numeric retime dialog and broader original UI acceptance remain
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-703"></a>
### T-703 — Onion skinning

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-703.

- Public baseline: earlier ghost implementation remains PUBLIC_ALPHA; this refinement is local only.
- Delivered scope: Ghost passes, 1–5 steps, selected-only, frame/key stepping and disabled early return
- Entry point: Onion toggle and context menu.
- Implementation: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs).
- Test/reference suites: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs)
- Example test symbols (suite references, not a coverage claim): `disabled_onion_skin_does_not_evaluate_extra_poses`.
- Local T-824: Theme channel shear/rotate tints, Session opacity/falloff, bounded frame/key times and deterministic counters; [contract](onion-skinning.md). Historical scrubbing/desktop acceptance remains pending.
- Latest relevant checkpoint: T-824 LOCAL_CANDIDATE, automated refinement; PUBLIC_ALPHA baseline remains separate.
- Recommended action: Deferred manual onion qualification.

<a id="t-704"></a>
### T-704 — Curve editor refinement

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-704.

- Delivered scope: Curve framing, snap/free badge, shared selection and exact numeric tangents
- Entry point: Graph / Properties tangents.
- Implementation: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs), [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t814_dopesheet_selection_reaches_numeric_inspector`.
- Contradictory check / remaining historical criteria: T-825 adds local scalar marquee and T-820 bounded linking; normalized overlay and global qualification remain
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P1/P2 targeted remainder; see roadmap.

<a id="t-814"></a>
### T-814 — Shared key selection

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-814.

- Delivered scope: Shared key selection, numeric tangent Apply/Cancel and safe graph gesture framing
- Entry point: Local Graph / tangents.
- Implementation: [`editor/src/key_selection.rs`](../editor/src/key_selection.rs), [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t814_dopesheet_selection_reaches_numeric_inspector`.
- Contradictory check / remaining historical criteria: Broader T-704 stays partial; combined graph edits retain existing granularity; GUI pending
- Latest relevant checkpoint: T-814.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Qualification queue only; do not redevelop.

<a id="t-905"></a>
### T-905 — Non-destructive timeline offset

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-905.

- Delivered scope: Persisted per-bone static sampling offsets and header UI
- Entry point: Bone header offset.
- Implementation: [`core/src/animation.rs`](../core/src/animation.rs), [`editor/src/ui/timeline/tree.rs`](../editor/src/ui/timeline/tree.rs), [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs).
- Test/reference suites: [`core/src/pose.rs`](../core/src/pose.rs), [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `bone_keys_are_offsets_from_the_setup_pose`.
- Contradictory check / remaining historical criteria: Not arbitrary per-track/per-selection offsets for slots/constraints/deforms
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-906"></a>
### T-906 — Timeline markers

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-906.

- Delivered scope: Named colored serialized timeline markers and snapping, omitted from runtime export
- Entry point: Ruler markers.
- Implementation: [`editor/src/ui/timeline/ruler.rs`](../editor/src/ui/timeline/ruler.rs), [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs), [`export/src/context.rs`](../export/src/context.rs).
- Test/reference suites: [`document/src/commands/marker_cmds.rs`](../document/src/commands/marker_cmds.rs), [`export/src/context.rs`](../export/src/context.rs)
- Example test symbols (suite references, not a coverage claim): `markers_are_added_in_time_order_whatever_order_they_arrive`.
- Contradictory check / remaining historical criteria: Not runtime event callbacks; no fresh desktop certification
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-912"></a>
### T-912 — Graph editor ergonomics review

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-912.

- Delivered scope: Framing, explicit snapping, overshoot flags and T-814 numeric tangents
- Entry point: Graph / Properties.
- Implementation: [`editor/src/ui/timeline/graph.rs`](../editor/src/ui/timeline/graph.rs), [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t814_dopesheet_selection_reaches_numeric_inspector`.
- Contradictory check / remaining historical criteria: T-825 adds local scalar marquee; normalized overlay and manual qualification remain; extreme sampling gaps unchanged
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="cap-02"></a>
### CAP-02 — Linked tangent modes

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-704 / T-912 / T-820.

- Delivered scope: Broken plus Aligned/Mirrored on one selected scalar junction with two compatible Bezier segments; numeric driver and graph drag share geometry.
- Entry point: Existing Properties Bezier tangents and Graph Editor.
- Implementation: [native math](../document/src/tangent_linking.rs), [native command](../document/src/commands/tangent_cmds.rs), [session policy](../editor/src/tangent_linking.rs).
- Tests: [native regression suite](../document/src/tangent_linking/tests.rs), [editor regression suite](../editor/src/tangent_linking/tests.rs), [graph gestures](../editor/src/ui/timeline/graph/t820_tests.rs).
- Contract: [seconds/native-value metric](bezier-tangent-linking.md); flat representability and shortest-arc rotation limitations are explicit. No automatic conversion, multi-junction linking or persistent per-key mode.
- Latest checkpoint: T-820 local candidate; GUI remains pending. T-704/T-912 broader criteria stay PARTIAL.

<a id="cap-03"></a>
### CAP-03 — Graph marquee and normalized overlay

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-704 / T-912 / T-825 / T-826.

- Delivered scope: Visible scalar key marquee through T-814 shared selection, plus per-channel normalized comparison (View Only). Plain replace / Ctrl-union; Ctrl-click remains a toggle.
- Entry point: Graph header Normalize, badge native-range tooltip and empty-area primary drag. [Interaction/normalization contract](graph-editor.md#8-normalized-comparison-view-t-826).
- Implementation: [graph](../editor/src/ui/timeline/graph.rs), [projection](../editor/src/ui/timeline/graph/normalization.rs), [marquee](../editor/src/ui/timeline/graph/marquee.rs).
- Tests: [T-826 headless coordinate/safety/reference tests](../editor/src/ui/timeline/graph/t826_tests.rs), existing T-814/T-820/T-825 suites.
- Remaining: Normalized value/handle editing is deliberately unavailable. No lasso, FFD graph, grouped key drag or event selection. Desktop qualification PENDING; T-704/T-912 remain PARTIAL.
- Latest checkpoint: T-826 local candidate. Public baseline is unchanged.

<a id="cap-11"></a>
### CAP-11 — Existing onion ghost generation

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-703.

- Delivered scope: Frame/key time generation, ghost passes, selected-bone filtering and off early return
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`editor/src/ui/timeline/transport.rs`](../editor/src/ui/timeline/transport.rs).
- Test/reference suites: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs)
- Example test symbols (suite references, not a coverage claim): `disabled_onion_skin_does_not_evaluate_extra_poses`.
- Contradictory check / remaining historical criteria: Tests cover off/boundaries; not every key-mode/selected-only visual case, theme or performance qualification
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-12"></a>
### CAP-12 — Precise numeric Bezier drafts

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-814 / T-704.

- Delivered scope: T-814 exact f32 normalized handles, Apply/Cancel and one native command
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs), [`editor/src/key_selection.rs`](../editor/src/key_selection.rs).
- Test/reference suites: [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t814_dopesheet_selection_reaches_numeric_inspector`.
- Contradictory check / remaining historical criteria: No tied handles; Windows interaction/artistic behavior pending
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-16"></a>
### CAP-16 — Onion theme/opacity controls

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-703 / T-824.

- Delivered scope: existing theme shear/rotate colors; Session opacity 0..1 and falloff exponent 0..2 in existing context menu.
- Contract: [Onion skinning](onion-skinning.md); no author data or new .ankh fields.
- Implementation: [`renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`transport.rs`](../editor/src/ui/timeline/transport.rs), [`session.rs`](../editor/src/session.rs).
- Tests: [`render/time/counter tests`](../editor/src/ui/canvas/renderer/t824_tests.rs), [`egui controls`](../editor/src/ui/timeline/transport/t824_tests.rs).
- Limits: uses shared channel palette roles; custom themes can remove past/future contrast. Theme/colorblindness, real scrubbing and perceived performance remain pending.
- Historical T-703 stays PARTIAL until desktop acceptance; no new public release.


<a id="meshes"></a>
## Meshes

<a id="t-401"></a>
### T-401 — Mesh attachment editing UI

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-401.

- Delivered scope: Mesh vertices/triangles, constrained retriangulation, UV and exact local numeric drafts
- Entry point: Mesh edit / UV Editor.
- Implementation: [`editor/src/ui/canvas/tools/mesh_edit.rs`](../editor/src/ui/canvas/tools/mesh_edit.rs), [`editor/src/ui/uv.rs`](../editor/src/ui/uv.rs), [`document/src/commands/mesh_cmds.rs`](../document/src/commands/mesh_cmds.rs).
- Test/reference suites: [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs), [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t815_switch_attachment_cannot_reuse_another_mesh_vertex_indices`.
- Contradictory check / remaining historical criteria: No universal snapping or arbitrary linked-source/topology edit safety
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-402"></a>
### T-402 — Auto mesh tracing

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-402.

- Delivered scope: Native alpha tracing/refinement with undoable topology replacement
- Entry point: Trace from image.
- Implementation: [`editor/src/ui/trace.rs`](../editor/src/ui/trace.rs), [`document/src/meshgen.rs`](../document/src/meshgen.rs).
- Test/reference suites: [`document/src/meshgen.rs`](../document/src/meshgen.rs)
- Example test symbols (suite references, not a coverage claim): `a_hole_gets_its_own_contour`.
- Contradictory check / remaining historical criteria: Tracing/refinement exists, but topology replacement clears weights rather than promised remapping; original <100 ms budget and general FFD preservation not qualified.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-403"></a>
### T-403 — Weight painting + bind compensation

**COMPLETE / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-403.

- Delivered scope: Add/Subtract/Replace/Smooth/Direct weights, radius/feather/session locks, normalization and history
- Entry point: Weights / W.
- Implementation: [`editor/src/ui/weights.rs`](../editor/src/ui/weights.rs), [`document/src/commands/weight_cmds.rs`](../document/src/commands/weight_cmds.rs).
- Test/reference suites: [`editor/src/ui/weights/t816_tests.rs`](../editor/src/ui/weights/t816_tests.rs), [`document/src/commands/weight_cmds.rs`](../document/src/commands/weight_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `t816_selecting_influence_keeps_mesh_target_via_real_egui_row`; `add_never_lowers_a_stronger_weight`.
- Contradictory check / remaining historical criteria: Complete native scoped brush; not autorigging or every-project visual fidelity; T-816 GUI pending
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-404"></a>
### T-404 — Deform timelines

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-404.

- Delivered scope: Native FFD evaluation and corrected stable Animate vertex drag
- Entry point: Animate vertex drag.
- Implementation: [`editor/src/edit_router.rs`](../editor/src/edit_router.rs), [`editor/src/ui/canvas/tools/mesh_edit.rs`](../editor/src/ui/canvas/tools/mesh_edit.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs), [`core/src/pose.rs`](../core/src/pose.rs)
- Example test symbols (suite references, not a coverage claim): `ffd_drag_does_not_accumulate_mouse_updates`; `deform_offsets_land_in_the_pose`.
- Contradictory check / remaining historical criteria: T-821 adds local single-key FFD editing; vector clipboard and broader topology/authoring qualification remain
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-405"></a>
### T-405 — Clipping attachments

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-405.

- Delivered scope: Clipping polygon/end-slot authoring, ordering and native commands
- Entry point: Clipping attachments.
- Implementation: [`core/src/clipping.rs`](../core/src/clipping.rs), [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs).
- Test/reference suites: [`core/src/clipping.rs`](../core/src/clipping.rs), [`document/src/commands/clip_cmds.rs`](../document/src/commands/clip_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `a_square_clipped_by_its_own_half_keeps_half_the_area`; `adding_a_clip_gives_it_a_polygon_and_points_the_slot_at_it`.
- Contradictory check / remaining historical criteria: Common adapters support bounded sequential masks, not arbitrary nested intersections
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-815"></a>
### T-815 — Viewport Precision & Numeric Geometry Editing

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-815.

- Delivered scope: Scoped exact Set/Add drafts for mesh/UV/path/polygons with Apply/Cancel
- Entry point: Local numeric mesh/UV/polygons.
- Implementation: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs), [`editor/src/ui/uv.rs`](../editor/src/ui/uv.rs).
- Test/reference suites: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t815_numeric_geometry_requires_explicit_apply_and_exact_readout`.
- Contradictory check / remaining historical criteria: Not every handle/expression/snap or linked-source edit; GUI pending
- Latest relevant checkpoint: T-815.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Qualification queue only; do not redevelop.

<a id="t-902"></a>
### T-902 — Numeric entry for every vertex and handle

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-902.

- Delivered scope: Exact mesh/UV/path/polygon Set/Add and numeric normalized Bezier handles
- Entry point: Local Properties / UV.
- Implementation: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs), [`editor/src/ui/inspector/tangents.rs`](../editor/src/ui/inspector/tangents.rs).
- Test/reference suites: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs), [`editor/src/ui/timeline/t814_tests.rs`](../editor/src/ui/timeline/t814_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t815_numeric_geometry_requires_explicit_apply_and_exact_readout`; `t814_dopesheet_selection_reaches_numeric_inspector`.
- Contradictory check / remaining historical criteria: Not every draggable point, expression parser or snap-to-neighbour
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="cap-07"></a>
### CAP-07 — Point-to-neighbour numeric snapping

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-708 / T-902.

- Delivered scope: No point-to-neighbour snapping path identified
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs), [`editor/src/ui/canvas/tools/mesh_edit.rs`](../editor/src/ui/canvas/tools/mesh_edit.rs).
- Test/reference suites: No dedicated executed test identified for this absent/unknown scope; static search only.
- Contradictory check / remaining historical criteria: Exact Set/Add numerical positions/UV/polygons already exist locally
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-13"></a>
### CAP-13 — Exact numeric geometry drafts

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-815 / T-902.

- Delivered scope: Scoped mesh/UV/path/polygon Set/Add, Apply/Cancel and context invalidation
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs), [`editor/src/ui/uv.rs`](../editor/src/ui/uv.rs).
- Test/reference suites: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t815_numeric_geometry_requires_explicit_apply_and_exact_readout`.
- Contradictory check / remaining historical criteria: Not every draggable point, expression or neighbour snapping
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="assets-psd"></a>
## Assets / PSD

<a id="t-301"></a>
### T-301 — Asset database + image drop-import

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-301.

- Delivered scope: Native encoded image bytes, image import and content-keyed upload pipeline
- Entry point: Assets / image drop.
- Implementation: [`core/src/assets.rs`](../core/src/assets.rs), [`editor/src/ui/assets.rs`](../editor/src/ui/assets.rs), [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs).
- Test/reference suites: [`core/src/assets.rs`](../core/src/assets.rs), [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `import_creates_asset_slot_and_attachment_in_one_step`.
- Contradictory check / remaining historical criteria: Project portability requires sidecar; broad texture-eviction/performance remains T-706
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-302"></a>
### T-302 — PSD import

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-302.

- Delivered scope: Nested PSD import without duplicates, Replace/Merge preview and exact-path simple-region image refresh
- Entry point: File Import / Assets / PSD drop.
- Implementation: [`formats/src/psd_read.rs`](../formats/src/psd_read.rs), [`editor/src/ui/psd_import.rs`](../editor/src/ui/psd_import.rs), [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs).
- Test/reference suites: [`formats/tests/psd_nested.rs`](../formats/tests/psd_nested.rs), [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs)
- Example test symbols (suite references, not a coverage claim): `nested_parser_ranges_include_descendants_but_parent_ids_identify_direct_children`.
- Contradictory check / remaining historical criteria: Renames/reparents become added+missing; automatic add/delete and changed shared/mesh/linked assets refused
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P1/P2 targeted remainder; see roadmap.

<a id="t-303"></a>
### T-303 — foreign armature importer

**SUPERSEDED / AUTOMATED / SOURCE_ONLY** · Origin: T-303.

- Delivered scope: Removed built-in foreign importer replaced by optional community JavaScript packages
- Entry point: Optional community packages.
- Implementation: [`formats/src/importer.rs`](../formats/src/importer.rs), [`plugins/src/importer.rs`](../plugins/src/importer.rs).
- Test/reference suites: [`formats/tests/importer_registry.rs`](../formats/tests/importer_registry.rs)
- Example test symbols (suite references, not a coverage claim): `only_first_party_formats_are_built_in`.
- Contradictory check / remaining historical criteria: Do not restore a second built-in importer; package licensing/availability remain external
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-304"></a>
### T-304 — Startup window

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-304.

- Delivered scope: Startup New/Open/recent/sample path with safe document replacement
- Entry point: Startup / File.
- Implementation: [`editor/src/ui/startup.rs`](../editor/src/ui/startup.rs), [`editor/src/app.rs`](../editor/src/app.rs).
- Test/reference suites: [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs), [`editor/src/app_psd_workflow_tests.rs`](../editor/src/app_psd_workflow_tests.rs)
- Example test symbols (suite references, not a coverage claim): `close_waits_for_png_cancellation_before_issuing_the_close_command`; `file_psd_import_stages_options_without_touching_the_document_or_history`.
- Contradictory check / remaining historical criteria: No first-user study of local T-816; unknown structural input limits remain
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-305"></a>
### T-305 — Atlas / spritesheet import modal

**COMPLETE / NOT_TESTED / PUBLIC_ALPHA** · Origin: T-305.

- Delivered scope: Existing atlas modal and native atlas import mapping
- Entry point: Atlas import.
- Implementation: [`editor/src/ui/atlas.rs`](../editor/src/ui/atlas.rs), [`document/src/import_ops.rs`](../document/src/import_ops.rs).
- Test/reference suites: [`document/src/import_ops.rs`](../document/src/import_ops.rs)
- Contradictory check / remaining historical criteria: No guarantee for every vendor atlas variant
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-306"></a>
### T-306 — Asset relink + external-source workflow

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-306.

- Delivered scope: Image relink/reload; safe save refuses missing image bytes
- Entry point: Assets Relink / Reload.
- Implementation: [`editor/src/ui/assets.rs`](../editor/src/ui/assets.rs), [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs).
- Test/reference suites: [`document/src/commands/asset_cmds.rs`](../document/src/commands/asset_cmds.rs), [`formats/tests/reliability.rs`](../formats/tests/reliability.rs)
- Example test symbols (suite references, not a coverage claim): `t813_missing_image_save_refuses_without_changing_primary_or_save_as`.
- Contradictory check / remaining historical criteria: Moving only project does not move sidecar; missing assets need explicit resolution
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-1003"></a>
### T-1003 — Move foreign formats to community JavaScript plugins

**COMPLETE / AUTOMATED / SOURCE_ONLY** · Origin: T-1003.

- Delivered scope: Foreign formats moved to optional pinned JavaScript packages
- Entry point: Optional community packages.
- Implementation: [`formats/src/importer.rs`](../formats/src/importer.rs), [`plugins/src/importer.rs`](../plugins/src/importer.rs), [`plugins/src/exporter.rs`](../plugins/src/exporter.rs), [`.gitmodules`](../.gitmodules).
- Test/reference suites: [`formats/tests/importer_registry.rs`](../formats/tests/importer_registry.rs)
- Example test symbols (suite references, not a coverage claim): `only_first_party_formats_are_built_in`.
- Contradictory check / remaining historical criteria: Missing package differs from broken builtin; no generic Godot/foreign fidelity promise
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="cap-09"></a>
### CAP-09 — Exact-path region PSD refresh

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-302.

- Delivered scope: Existing unique exact-path region refresh with native atomic Undo/Redo and save/reload tests
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs), [`formats/src/reimport_guard.rs`](../formats/src/reimport_guard.rs).
- Test/reference suites: [`document/src/psd_reimport.rs`](../document/src/psd_reimport.rs), [`formats/tests/reimport_guard.rs`](../formats/tests/reimport_guard.rs)
- Example test symbols (suite references, not a coverage claim): `canonical_author_fixtures_are_read_only_and_safe_to_review`.
- Contradictory check / remaining historical criteria: Added/missing/renamed/shared/mesh cases remain guarded or manual
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="formats-exports"></a>
## Formats / exports

<a id="t-601"></a>
### T-601 — Offscreen render + image / spritesheet / sequence export

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-601.

- Delivered scope: CPU deterministic PNG/JPEG/WebP sequences, spritesheets, framing/trim/padding/batch
- Entry point: Export sequences / sheets.
- Implementation: [`render/src/sequence.rs`](../render/src/sequence.rs), [`render/src/spritesheet.rs`](../render/src/spritesheet.rs), [`editor/src/png_export.rs`](../editor/src/png_export.rs).
- Test/reference suites: [`render/src/lib.rs`](../render/src/lib.rs), [`export/tests/generated_images.rs`](../export/tests/generated_images.rs)
- Example test symbols (suite references, not a coverage claim): `sequence_is_exclusive_deterministic_and_has_one_camera`; `generated_images_use_the_existing_transaction_and_match_their_extensions`.
- Contradictory check / remaining historical criteria: Default skin/Normal/no physics; original background/performance/viewport criteria not all closed
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-601-1"></a>
### T-601.1 — PNG retained

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-601.1.

- Delivered scope: PNG retained, explicit opaque JPEG background and lossless RGBA WebP
- Entry point: Sequence format choice.
- Implementation: [`render/src/encoding.rs`](../render/src/encoding.rs), [`editor/src/png_export.rs`](../editor/src/png_export.rs).
- Test/reference suites: [`render/src/encoding.rs`](../render/src/encoding.rs), [`export/tests/generated_images.rs`](../export/tests/generated_images.rs)
- Example test symbols (suite references, not a coverage claim): `lossless_formats_preserve_every_rgba_byte_even_beneath_zero_alpha`; `generated_images_use_the_existing_transaction_and_match_their_extensions`.
- Contradictory check / remaining historical criteria: JPEG lossy tolerance/no alpha; no runtime format change
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-601-2"></a>
### T-601.2 — Opt-in common camera/global bounds

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-601.2.

- Delivered scope: Opt-in common camera/global bounds, trim reconstruction and padding
- Entry point: Framing options.
- Implementation: [`render/src/framing.rs`](../render/src/framing.rs), [`editor/src/ui/png_sequence.rs`](../editor/src/ui/png_sequence.rs).
- Test/reference suites: [`render/src/framing.rs`](../render/src/framing.rs), [`export/tests/generated_images.rs`](../export/tests/generated_images.rs)
- Example test symbols (suite references, not a coverage claim): `static_margins_partial_alpha_and_empty_frames_have_unambiguous_offsets`; `generated_images_use_the_existing_transaction_and_match_their_extensions`.
- Contradictory check / remaining historical criteria: Skin/blend/physics exclusions persist; not arbitrary renderer fidelity
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-601-3"></a>
### T-601.3 — Selected-clip batches

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-601.3.

- Delivered scope: Selected-clip batches, deterministic plans and new-destination publication
- Entry point: Multiple clip checkboxes.
- Implementation: [`export/src/batch.rs`](../export/src/batch.rs), [`editor/src/ui/png_sequence.rs`](../editor/src/ui/png_sequence.rs).
- Test/reference suites: [`export/tests/batch.rs`](../export/tests/batch.rs)
- Example test symbols (suite references, not a coverage claim): `batch_directory_is_published_only_after_its_complete_plan_is_ready`.
- Contradictory check / remaining historical criteria: Not crash-atomic multi-file commit or unrestricted memory/performance guarantee
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-602"></a>
### T-602 — Video export via ffmpeg

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-602.

- Delivered scope: CPU MP4/GIF through supervised external FFmpeg, progress/cancel and safe publication
- Entry point: Export Video.
- Implementation: [`editor/src/video_export.rs`](../editor/src/video_export.rs), [`editor/src/video_export/gif.rs`](../editor/src/video_export/gif.rs), [`editor/src/ui/video.rs`](../editor/src/ui/video.rs).
- Test/reference suites: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs), [`editor/src/video_export/gif/tests.rs`](../editor/src/video_export/gif/tests.rs)
- Example test symbols (suite references, not a coverage claim): `detection_missing_custom_path_and_empty_path_never_fall_back`; `gif_capabilities_do_not_require_h264_and_report_missing_filters`.
- Contradictory check / remaining historical criteria: No audio/GIF alpha/physics or every-player guarantee; external encoder required
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-602-1"></a>
### T-602.1 — Raw RGB CPU video to H.264

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-602.1.

- Delivered scope: Raw RGB CPU video to H.264, no-replace publication
- Entry point: Advanced MP4.
- Implementation: [`editor/src/video_export.rs`](../editor/src/video_export.rs), [`export/src/new_file.rs`](../export/src/new_file.rs).
- Test/reference suites: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs)
- Example test symbols (suite references, not a coverage claim): `detection_missing_custom_path_and_empty_path_never_fall_back`.
- Contradictory check / remaining historical criteria: Opaque background, no bundled encoder; hard-link filesystem requirement
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-602-2"></a>
### T-602.2 — Two-pass GIF palette

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-602.2.

- Delivered scope: Two-pass GIF palette, repeat policy and measured centisecond timing
- Entry point: GIF export.
- Implementation: [`editor/src/video_export/gif.rs`](../editor/src/video_export/gif.rs).
- Test/reference suites: [`editor/src/video_export/gif/tests.rs`](../editor/src/video_export/gif/tests.rs)
- Example test symbols (suite references, not a coverage claim): `gif_capabilities_do_not_require_h264_and_report_missing_filters`.
- Contradictory check / remaining historical criteria: No GIF alpha or arbitrary exact FPS promise
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-602-3"></a>
### T-602.3 — CRF/bitrate

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-602.3.

- Delivered scope: CRF/bitrate, probed libx264/libx264rgb and Windows opt-in association opening
- Entry point: Encoding options.
- Implementation: [`editor/src/video_export.rs`](../editor/src/video_export.rs), [`editor/src/ui/video.rs`](../editor/src/ui/video.rs).
- Test/reference suites: [`editor/src/video_export/tests.rs`](../editor/src/video_export/tests.rs)
- Example test symbols (suite references, not a coverage claim): `detection_missing_custom_path_and_empty_path_never_fall_back`.
- Contradictory check / remaining historical criteria: Native opener injected in tests; RGB requires player consent; not hardware encoding
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-603"></a>
### T-603 — Original hardcoded-exporter plan replaced by user-authored templates/preset

**SUPERSEDED / AUTOMATED / SOURCE_ONLY** · Origin: T-603.

- Delivered scope: Original hardcoded-exporter plan replaced by user-authored templates/presets
- Entry point: Template engine replaces old plan.
- Implementation: [`export/src/template.rs`](../export/src/template.rs), [`docs/export-plan.md`](../docs/export-plan.md).
- Test/reference suites: [`export/tests/template.rs`](../export/tests/template.rs)
- Example test symbols (suite references, not a coverage claim): `a_missing_field_is_an_error_not_an_empty_string`.
- Contradictory check / remaining historical criteria: Do not create another hardcoded exporter; aggregate library still partial
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-603a"></a>
### T-603a — Atlas bake

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-603a.

- Delivered scope: Deterministic atlas pages, trim/padding and mappings
- Entry point: Atlas settings.
- Implementation: [`export/src/atlas.rs`](../export/src/atlas.rs).
- Test/reference suites: [`export/tests/atlas.rs`](../export/tests/atlas.rs)
- Example test symbols (suite references, not a coverage claim): `the_same_assets_pack_to_identical_bytes`.
- Contradictory check / remaining historical criteria: Common runtime excludes rotated atlases even if custom presets differ
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-603b"></a>
### T-603b — Template engine + export runner

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-603b.

- Delivered scope: Strict template runner, path validation, staging/backups/rollback on ordinary failure
- Entry point: Publish after preflight.
- Implementation: [`export/src/template.rs`](../export/src/template.rs), [`export/src/run.rs`](../export/src/run.rs), [`export/src/staged.rs`](../export/src/staged.rs).
- Test/reference suites: [`export/tests/template.rs`](../export/tests/template.rs), [`export/tests/safety.rs`](../export/tests/safety.rs)
- Example test symbols (suite references, not a coverage claim): `a_missing_field_is_an_error_not_an_empty_string`; `an_empty_output_directory_is_refused`.
- Contradictory check / remaining historical criteria: No filesystem-wide atomicity or guaranteed power-loss durability
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-603c"></a>
### T-603c — Native runtime format, authored as a template

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-603c.

- Delivered scope: Native v2 template: independent axes, curves, strings, FFD mapping and static dark RGBA
- Entry point: Native v2 preset.
- Implementation: [`export/src/presets/ankhimate_runtime.json`](../export/src/presets/ankhimate_runtime.json), [`export/src/context.rs`](../export/src/context.rs).
- Test/reference suites: [`export/tests/export.rs`](../export/tests/export.rs), [`export/tests/ffd_curves.rs`](../export/tests/ffd_curves.rs), [`export/tests/dark_tint.rs`](../export/tests/dark_tint.rs)
- Example test symbols (suite references, not a coverage claim): `native_ffd_stepped_uses_the_destination_keys_curve`; `static_dark_rgba_survives_native_json_exactly_including_zero_alpha`.
- Contradictory check / remaining historical criteria: No new .ankh.runtime binary; serializable fields do not imply adapter support
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-603d"></a>
### T-603d — Export panel

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-603d.

- Delivered scope: Existing data/rendered export preflight, collision inventory/consent and safety rechecks
- Entry point: View Export.
- Implementation: [`editor/src/ui/export.rs`](../editor/src/ui/export.rs), [`editor/src/export_preflight.rs`](../editor/src/export_preflight.rs), [`export/src/preflight.rs`](../export/src/preflight.rs).
- Test/reference suites: [`export/tests/preflight.rs`](../export/tests/preflight.rs), [`export/tests/preflight_paths.rs`](../export/tests/preflight_paths.rs)
- Example test symbols (suite references, not a coverage claim): `valid_runtime_has_no_unjustified_warning_and_preserves_its_input`; `inspection_reuses_portable_path_and_json_checks_without_creating_a_directory`.
- Contradictory check / remaining historical criteria: Not a continuously updated global diagnostics dock
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-603e"></a>
### T-603e — Preset library

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-603e.

- Delivered scope: Native/generic/Phaser-atlas presets and custom templates
- Entry point: Preset list.
- Implementation: [`export/src/presets/mod.rs`](../export/src/presets/mod.rs), [`export/src/preset.rs`](../export/src/preset.rs).
- Test/reference suites: [`export/tests/template.rs`](../export/tests/template.rs)
- Example test symbols (suite references, not a coverage claim): `a_missing_field_is_an_error_not_an_empty_string`.
- Contradictory check / remaining historical criteria: No qualified Godot preset; foreign-format plugins remain optional
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-605"></a>
### T-605 — Format spec finalization + migration policy

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-605.

- Delivered scope: v1 profiles/specification, version gate and local T-813 unknown-field refusal
- Entry point: File reader / format spec.
- Implementation: [`docs/format-spec.md`](../docs/format-spec.md), [`formats/src/compatibility.rs`](../formats/src/compatibility.rs).
- Test/reference suites: [`formats/tests/schema_contract.rs`](../formats/tests/schema_contract.rs), [`formats/tests/migrations.rs`](../formats/tests/migrations.rs), [`formats/tests/reliability.rs`](../formats/tests/reliability.rs)
- Example test symbols (suite references, not a coverage claim): `every_schema_field_and_variant_matches_the_documented_inventory`; `authentic_v1_fixtures_save_reopen_all_profiles_without_touching_original_assets`.
- Contradictory check / remaining historical criteria: No authentic v0 migration/prototype converter or comprehensive hostile-file resource budget
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="cap-04"></a>
### CAP-04 — Authentic legacy migration qualification

**UNKNOWN / NOT_TESTED / UNKNOWN** · Origin: T-605.

- Delivered scope: No authenticated author-v0 input/conversion proof available
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`formats/tests/migrations.rs`](../formats/tests/migrations.rs), [`formats/tests/fixtures/format-versions.json`](../formats/tests/fixtures/format-versions.json).
- Test/reference suites: No dedicated executed test identified for this absent/unknown scope; static search only.
- Contradictory check / remaining historical criteria: Current v1 profiles qualified; prototype ZIP rejected; do not fabricate a migration fixture
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-10"></a>
### CAP-10 — Export preflight and consent

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-702 / T-603d.

- Delivered scope: Existing integrity/fidelity diagnostics, file/collision review, explicit consent and publisher rechecks
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`export/src/preflight.rs`](../export/src/preflight.rs), [`editor/src/export_preflight.rs`](../editor/src/export_preflight.rs).
- Test/reference suites: [`export/tests/preflight.rs`](../export/tests/preflight.rs), [`export/tests/preflight_paths.rs`](../export/tests/preflight_paths.rs)
- Example test symbols (suite references, not a coverage claim): `valid_runtime_has_no_unjustified_warning_and_preserves_its_input`; `inspection_reuses_portable_path_and_json_checks_without_creating_a_directory`.
- Contradictory check / remaining historical criteria: Not global live document diagnostics; no new filesystem guarantees
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-14"></a>
### CAP-14 — Unknown-author-member safe refusal

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-813 / T-605.

- Delivered scope: Unsupported/dropped author members or missing image bytes prevent unsafe save
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`formats/src/compatibility.rs`](../formats/src/compatibility.rs), [`formats/src/lib.rs`](../formats/src/lib.rs).
- Test/reference suites: [`formats/tests/reliability.rs`](../formats/tests/reliability.rs)
- Example test symbols (suite references, not a coverage claim): `t813_missing_image_save_refuses_without_changing_primary_or_save_as`.
- Contradictory check / remaining historical criteria: Not lossless opaque-author-extra storage or complete hostile-file validation
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="pixijs-phaser"></a>
## PixiJS / Phaser

<a id="t-604"></a>
### T-604 — Runtime integration

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604.

- Delivered scope: External TypeScript core and two packaged adapters with qualified common profile
- Entry point: External runtime integration.
- Implementation: [`docs/rigging-capabilities.md`](../docs/rigging-capabilities.md), [`export/src/presets/ankhimate_runtime.json`](../export/src/presets/ankhimate_runtime.json).
- Test/reference suites: [`export/tests/export.rs`](../export/tests/export.rs)
- Example test symbols (suite references, not a coverage claim): `spine_is_not_a_first_party_export_preset`.
- Contradictory check / remaining historical criteria: In-workspace Rust player plan superseded; no universal runtime parity
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-810"></a>
### T-810 — Phaser common-profile corrections and public playback API aligned to shared

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-810.

- Delivered scope: Phaser common-profile corrections and public playback API aligned to shared renderer helpers
- Entry point: External PhaserRig API.
- Implementation: [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts), [`runtime/core/src/render/mesh.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/mesh.ts).
- Test/reference suites: [`runtime/phaser/test/parity.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/test/parity.test.mjs)
- Contradictory check / remaining historical criteria: No new all-feature parity; detailed external runtime rows below
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="rt-01"></a>
### RT-01 — Shared native v1/v2 parser

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: v1/v2 read and separate author-file adapter exist
- Entry point: External public adapter/core API.
- Implementation: [`runtime/core/src/rig.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/rig.ts), [`runtime/core/src/ankh.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/ankh.ts).
- Test/reference suites: [`runtime/core/test/runtime.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/runtime.test.ts)
- Contradictory check / remaining historical criteria: Core v1 reading does not certify v1 adapter pixel parity
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-02"></a>
### RT-02 — Common animation and crossfade

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Independent axes/curves, playback and distinct-clip crossfade
- Entry point: External public adapter/core API.
- Implementation: [`runtime/core/src/evaluate.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/evaluate.ts), [`runtime/core/src/player.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/player.ts), [`runtime/core/src/render/playback.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/playback.ts).
- Test/reference suites: [`runtime/core/test/evaluate.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/evaluate.test.ts), [`runtime/core/test/playback-boundaries.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/playback-boundaries.test.ts)
- Contradictory check / remaining historical criteria: Interrupted crossfades may jump; seek does not emit traversal events
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-03"></a>
### RT-03 — Runtime events

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Positive-time events in deterministic tie order, incoming-only fades
- Entry point: External public adapter/core API.
- Implementation: [`runtime/core/src/player.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/player.ts), [`runtime/core/src/render/playback.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/playback.ts).
- Test/reference suites: [`runtime/core/test/playback-boundaries.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/playback-boundaries.test.ts)
- Contradictory check / remaining historical criteria: t=0 ignored at every loop; no automatic sound
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-04"></a>
### RT-04 — PixiJS 8 WebGL2 common profile

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Regions, rigid/weighted meshes, FFD, trim and multipage pages
- Entry point: External public adapter/core API.
- Implementation: [`runtime/pixi/src/rig.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/rig.ts), [`runtime/pixi/src/validate.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/validate.ts), [`runtime/pixi/src/load.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/load.ts).
- Test/reference suites: [`runtime/pixi/test/regions.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/regions.test.mjs), [`runtime/pixi/test/meshes.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/meshes.test.mjs)
- Contradictory check / remaining historical criteria: Explicit v2/WebGL2; constraints/linked/rotation rejected; not WebGL1/WebGPU/mobile certificate
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-05"></a>
### RT-05 — Phaser 3 common profile

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Phaser 3.90.0 public playback, skin UV refresh, dark tint and shared geometry
- Entry point: External public adapter/core API.
- Implementation: [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts), [`runtime/core/src/render/mesh.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/mesh.ts).
- Test/reference suites: [`runtime/phaser/test/parity.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/test/parity.test.mjs)
- Contradictory check / remaining historical criteria: Actual T-811 GL context WebGL1; legacy extras warn, not parity-certified
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-06"></a>
### RT-06 — Weighted meshes and FFD

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Qualified finite native v2 weighted/FFD subset with Rust references
- Entry point: External public adapter/core API.
- Implementation: [`runtime/core/src/render/mesh.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/mesh.ts), [`runtime/pixi/src/mesh.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/mesh.ts), [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts).
- Test/reference suites: [`runtime/pixi/test/meshes.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/meshes.test.mjs), [`runtime/core/test/ffd-timing.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/ffd-timing.test.ts)
- Contradictory check / remaining historical criteria: Topology/bind compatibility required; not arbitrary linked FFD inheritance
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-07"></a>
### RT-07 — Skins, light and static dark tint

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Compatible alternate skins, light animation and static dark RGBA
- Entry point: External public adapter/core API.
- Implementation: [`runtime/pixi/src/tint.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/tint.ts), [`runtime/pixi/src/rig.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/rig.ts), [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts).
- Test/reference suites: [`runtime/pixi/test/appearance.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/appearance.test.mjs), [`runtime/core/test/dark-tint.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/dark-tint.test.ts)
- Contradictory check / remaining historical criteria: No animated dark channel or arbitrary multi-skin composition
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-08"></a>
### RT-08 — Sequential clipping

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Qualified slot/end-slot sequential masks, transformed cases
- Entry point: External public adapter/core API.
- Implementation: [`runtime/pixi/src/clipping.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/clipping.ts), [`runtime/core/src/render/clipping.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/clipping.ts).
- Test/reference suites: [`runtime/pixi/test/clipping.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/clipping.test.mjs), [`runtime/phaser/test/parity.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/test/parity.test.mjs)
- Contradictory check / remaining historical criteria: Nested replacement is not intersecting mask composition; bounded concave qualification
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-09"></a>
### RT-09 — Constraints and physics outside common profile

**PARTIAL / AUTOMATED / SOURCE_ONLY** · Origin: T-604 / T-810.

- Delivered scope: Some legacy TypeScript constraint code exists; Phaser warns and Pixi rejects
- Entry point: External public adapter/core API.
- Implementation: [`runtime/core/src/constraints.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/constraints.ts), [`runtime/pixi/src/validate.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/validate.ts), [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts).
- Test/reference suites: [`runtime/core/test/runtime.test.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/test/runtime.test.ts)
- Contradictory check / remaining historical criteria: Native physics target/bones mismatch not activated; no Rust/TS stateful fidelity claim
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-10"></a>
### RT-10 — Advanced blending and linked meshes

**PARTIAL / AUTOMATED / SOURCE_ONLY** · Origin: T-604 / T-810.

- Delivered scope: Normal is common qualified composition; legacy Phaser extra paths exist
- Entry point: External public adapter/core API.
- Implementation: [`runtime/pixi/src/validate.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/validate.ts), [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts).
- Test/reference suites: [`runtime/pixi/test/appearance.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/appearance.test.mjs)
- Contradictory check / remaining historical criteria: Add/Multiply/Screen and linked/rotated cases excluded, even if a core type exists
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-11"></a>
### RT-11 — Runtime resource and performance qualification

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Ownership/cleanup tests and bounded historical instance/soak observations
- Entry point: External public adapter/core API.
- Implementation: [`runtime/pixi/src/rig.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/src/rig.ts), [`runtime/phaser/src/phaser.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/src/phaser.ts), [`runtime/core/src/render/playback.ts`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/core/src/render/playback.ts).
- Test/reference suites: [`runtime/pixi/test/regions.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/pixi/test/regions.test.mjs), [`runtime/phaser/test/parity.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/phaser/test/parity.test.mjs)
- Contradictory check / remaining historical criteria: Not production-scale/iGPU/mobile or universal frame-time budget
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.

<a id="rt-12"></a>
### RT-12 — Reproducible game examples and packages

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-604 / T-810.

- Delivered scope: Matching GitHub tarballs, offline portable Pixi demo and standalone comparison example
- Entry point: External public adapter/core API.
- Implementation: [`runtime/examples/phaser-pixi-parity/server.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/examples/phaser-pixi-parity/server.mjs), [`runtime/docs/RUNTIME_INTEGRATION.md`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/docs/RUNTIME_INTEGRATION.md), [`runtime/package-lock.json`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/package-lock.json).
- Test/reference suites: [`runtime/scripts/parity-pixels.test.mjs`](https://github.com/Kr-Apo/ankhimate-runtime/blob/c4c38df269cfcb30f941221454b7202f40d3fa5b/scripts/parity-pixels.test.mjs)
- Contradictory check / remaining historical criteria: No npm publication; browser modules need HTTP and matching core/render import map
- Latest relevant checkpoint: c4c38df; historical T-811 tests, no T-817 runtime rerun.
- Recommended action: Retain qualified common profile; do not infer Rust feature support. Priority: Respect common alpha contract.


<a id="ia"></a>
## IA

<a id="t-805"></a>
### T-805 — Native conversational assistant

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-805.

- Delivered scope: Opt-in conversation, bounded native proposals, explicit approval and atomic history
- Entry point: AI Assistant.
- Implementation: [`editor/src/ai/mod.rs`](../editor/src/ai/mod.rs), [`editor/src/ai/transport.rs`](../editor/src/ai/transport.rs), [`editor/src/ai/ui.rs`](../editor/src/ai/ui.rs).
- Test/reference suites: [`editor/src/ai/tests.rs`](../editor/src/ai/tests.rs), [`editor/src/ai/expert_tests.rs`](../editor/src/ai/expert_tests.rs)
- Example test symbols (suite references, not a coverage claim): `expert_result_never_rewrites_an_older_call_with_the_same_provider_id`; `send_stop_widget_identity_survives_proposals_and_native_result_banners`.
- Contradictory check / remaining historical criteria: Not autonomous autorigging; model prose can be wrong; no generic cloud/model certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-805-1"></a>
### T-805.1 — Fixed composer

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-805.1.

- Delivered scope: Fixed composer, phases, native receipts, bounded correction and NInfer profile
- Entry point: AI panel / NInfer settings.
- Implementation: [`editor/src/ai/activity.rs`](../editor/src/ai/activity.rs), [`editor/src/ai/manual.rs`](../editor/src/ai/manual.rs), [`editor/src/ai/ui.rs`](../editor/src/ai/ui.rs).
- Test/reference suites: [`editor/src/ai/expert_tests.rs`](../editor/src/ai/expert_tests.rs)
- Example test symbols (suite references, not a coverage claim): `send_stop_widget_identity_survives_proposals_and_native_result_banners`.
- Contradictory check / remaining historical criteria: Historical Qwen real run is binary/model-specific; no new T-816 inference qualification
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-806"></a>
### T-806 — Viewport vision and first native rigging tools

**PARTIAL / MIXED / PUBLIC_ALPHA** · Origin: T-806.

- Delivered scope: Bounded viewport preview/one-send consent and typed bone rigging tools
- Entry point: AI consent / native proposal.
- Implementation: [`editor/src/ai/vision.rs`](../editor/src/ai/vision.rs), [`editor/src/ai/rig.rs`](../editor/src/ai/rig.rs).
- Test/reference suites: [`editor/src/ai/t806_tests.rs`](../editor/src/ai/t806_tests.rs)
- Example test symbols (suite references, not a coverage claim): `rigging_tool_is_advertised_and_create_bone_is_typed`.
- Contradictory check / remaining historical criteria: Targeted historical maintainer report only; no general visual placement or every-model certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-807"></a>
### T-807 — Controlled native character assembly

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-807.

- Delivered scope: Guarded region/slot/bone/order assembly preserving representable FK Setup
- Entry point: ankhimate_assemble / approval.
- Implementation: [`editor/src/ai/assembly.rs`](../editor/src/ai/assembly.rs), [`editor/src/ai/tools.rs`](../editor/src/ai/tools.rs), [`document/src/part_ops.rs`](../document/src/part_ops.rs).
- Test/reference suites: [`editor/src/ai/t807_tests.rs`](../editor/src/ai/t807_tests.rs), [`document/src/part_ops.rs`](../document/src/part_ops.rs)
- Example test symbols (suite references, not a coverage claim): `assembly_tool_and_region_transform_are_available`.
- Contradictory check / remaining historical criteria: Weighted meshes are not regions; unrepresentable/constrained cases refused
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-808"></a>
### T-808 — Guarded mesh weight assistance

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-808.

- Delivered scope: Paged mesh reads and bounded set/normalize/smooth existing weights after approval
- Entry point: ankhimate_read_mesh / ankhimate_weights.
- Implementation: [`editor/src/ai/weights.rs`](../editor/src/ai/weights.rs), [`editor/src/ai/tools.rs`](../editor/src/ai/tools.rs).
- Test/reference suites: [`editor/src/ai/t808_tests.rs`](../editor/src/ai/t808_tests.rs)
- Example test symbols (suite references, not a coverage claim): `existing_weighted_mesh_can_receive_an_approved_native_weight_correction`.
- Contradictory check / remaining historical criteria: No new influences, geometry/UV/bind/FFD edits; linked/shared/constrained cases protected
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-809"></a>
### T-809 — Multimodal provider transport and bounded context

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-809.

- Delivered scope: Six multimodal envelopes, destination-bound declarations and image/context budgets
- Entry point: AI model declarations / budgets.
- Implementation: [`editor/src/ai/protocol.rs`](../editor/src/ai/protocol.rs), [`editor/src/ai/capabilities.rs`](../editor/src/ai/capabilities.rs), [`editor/src/ai/transport.rs`](../editor/src/ai/transport.rs).
- Test/reference suites: [`editor/src/ai/t809_tests.rs`](../editor/src/ai/t809_tests.rs)
- Example test symbols (suite references, not a coverage claim): `all_six_providers_encode_real_image_in_native_wire_contract`.
- Contradictory check / remaining historical criteria: Wire support is not concrete model compatibility; context exhaustion remains possible
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.


<a id="ux-tools"></a>
## UX / tools

<a id="t-701"></a>
### T-701 — Settings, keymap, autosave

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-701.

- Delivered scope: JSON preferences, registered keymap and monotonic isolated idle autosave
- Entry point: Settings / autosave.
- Implementation: [`editor/src/config.rs`](../editor/src/config.rs), [`editor/src/keymap.rs`](../editor/src/keymap.rs), [`editor/src/autosave.rs`](../editor/src/autosave.rs), [`editor/src/ui/settings.rs`](../editor/src/ui/settings.rs).
- Test/reference suites: [`editor/src/config.rs`](../editor/src/config.rs), [`editor/src/keymap.rs`](../editor/src/keymap.rs), [`editor/src/autosave_clock_tests.rs`](../editor/src/autosave_clock_tests.rs)
- Example test symbols (suite references, not a coverage claim): `shift_h_and_h_are_different_bindings`.
- Contradictory check / remaining historical criteria: Original exhaustive settings list/restart acceptance incomplete; unsaved-doc autosave policy retained
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-702"></a>
### T-702 — Diagnostics / warnings

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-702, refined T-818.

- Delivered scope: Prior PUBLIC_ALPHA export/load diagnostics, plus LOCAL_CANDIDATE global author-document rules, debounce, badge/list and safe offender selection.
- Entry point: Bottom Diagnostics badge / Document diagnostics window; export preflight remains independent.
- Implementation: [`document/src/diagnostics.rs`](../document/src/diagnostics.rs), [`editor/src/diagnostics.rs`](../editor/src/diagnostics.rs), [`export/src/preflight.rs`](../export/src/preflight.rs).
- Tests: [`document/src/diagnostics/tests.rs`](../document/src/diagnostics/tests.rs), [`editor/src/diagnostics/tests.rs`](../editor/src/diagnostics/tests.rs); original publication tests retained.
- Remainder: No universal frame-offender camera, no historical numeric debounce budget, no worst-case resource/latency certification. GUI remains pending. Unsupported runtime features remain export fidelity concerns.
- Latest relevant checkpoint: T-818 local candidate, direct parent T-817. No release/version change.
- Recommended action: Review T-818 and run deferred cumulative manual campaign; do not rebuild the existing export pipeline.

<a id="t-705"></a>
### T-705 — Localization

**DEFERRED / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-705.

- Delivered scope: English UI literals; no localization catalog/macro/picker identified
- Entry point: No catalog/picker found.
- Implementation: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs), [`docs/TASKS-before-T817.md`](../docs/TASKS-before-T817.md).
- Test/reference suites: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs)
- Example test symbols (suite references, not a coverage claim): `sibling_cards_are_separated_by_the_gap`.
- Contradictory check / remaining historical criteria: i18n implementation absent and deliberately deferred; no locale certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P3 extension, not next core workflow.

<a id="t-706"></a>
### T-706 — Performance pass

**PARTIAL / AUTOMATED / SOURCE_ONLY** · Origin: T-706.

- Delivered scope: Renderer/cache reuse and physics microbench tests exist; no dedicated core benchmark target identified in this checkout.
- Entry point: Developer performance work.
- Implementation: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`editor/src/ui/timeline/sheet.rs`](../editor/src/ui/timeline/sheet.rs).
- Test/reference suites: [`core/src/physics.rs`](../core/src/physics.rs)
- Example test symbols (suite references, not a coverage claim): `the_same_dt_sequence_produces_the_same_trajectory`.
- Contradictory check / remaining historical criteria: No dedicated core benchmark target/xtask bench/CI trend or general 500-bone/iGPU 60-FPS qualification identified; small deterministic tests are not performance budgets.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-708"></a>
### T-708 — Viewport interaction polish

**PARTIAL / PENDING_MANUAL / PUBLIC_ALPHA** · Origin: T-708.

- Delivered scope: Gizmos, region pivot editing, camera and precise geometry/UV entry
- Entry point: Viewport / numeric Properties.
- Implementation: [`editor/src/ui/canvas/tools/select.rs`](../editor/src/ui/canvas/tools/select.rs), [`editor/src/geometry_edit.rs`](../editor/src/geometry_edit.rs).
- Test/reference suites: [`editor/src/ui/inspector/t815_tests.rs`](../editor/src/ui/inspector/t815_tests.rs), [`editor/src/ui/canvas/tools/mesh_edit/tests.rs`](../editor/src/ui/canvas/tools/mesh_edit/tests.rs)
- Example test symbols (suite references, not a coverage claim): `t815_numeric_geometry_requires_explicit_apply_and_exact_readout`; `ffd_drag_does_not_accumulate_mouse_updates`.
- Contradictory check / remaining historical criteria: No universal snapping/expression-on-drag/pivot editing for every attachment; framing acceptance incomplete
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Availability split: Scoped T-813/T-814/T-815/T-816 improvements are LOCAL_CANDIDATE; the earlier public baseline is separate.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-709"></a>
### T-709 — Accessibility & window UX

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-709.

- Delivered scope: Control sizing, scaling, partial keyboard/AccessKit, close guard and docking
- Entry point: View / keyboard focus.
- Implementation: [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs), [`editor/src/ui/settings.rs`](../editor/src/ui/settings.rs), [`editor/src/app.rs`](../editor/src/app.rs).
- Test/reference suites: [`editor/src/ui/inspector/key_control_tests.rs`](../editor/src/ui/inspector/key_control_tests.rs), [`editor/src/app_lifecycle_tests.rs`](../editor/src/app_lifecycle_tests.rs)
- Example test symbols (suite references, not a coverage claim): `inspector_key_context_actions_are_reachable_by_secondary_click`; `close_waits_for_png_cancellation_before_issuing_the_close_command`.
- Contradictory check / remaining historical criteria: No whole workflow keyboard/contrast/colorblind audit or exhaustive icon-tooltip certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-901"></a>
### T-901 — Bulk rename with sequential numbering

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-901.

- Delivered scope: Bone numbering/substring replacement, preview and native batch rename
- Entry point: Rename bones.
- Implementation: [`editor/src/ui/rename.rs`](../editor/src/ui/rename.rs), [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs).
- Test/reference suites: [`document/src/commands/bone_cmds.rs`](../document/src/commands/bone_cmds.rs)
- Example test symbols (suite references, not a coverage claim): `a_shifted_batch_rename_does_not_collide_with_itself`.
- Contradictory check / remaining historical criteria: Not slot/attachment/constraint rename, regex or all path-safety variants
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-903"></a>
### T-903 — Isolation mode

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-903.

- Delivered scope: Session-only descendant isolation and exit affordance
- Entry point: Isolate / exit badge.
- Implementation: [`editor/src/session.rs`](../editor/src/session.rs), [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`editor/src/operators.rs`](../editor/src/operators.rs).
- Test/reference suites: [`editor/src/session.rs`](../editor/src/session.rs)
- Example test symbols (suite references, not a coverage claim): `isolation_takes_the_whole_subtree`.
- Contradictory check / remaining historical criteria: Optional hierarchy/dopesheet filters/configurable opacity not fully qualified
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-910"></a>
### T-910 — Multi-window / multi-monitor

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-910.

- Delivered scope: Noncanvas panels detach over one AppState; detached set persisted
- Entry point: View Open in a window.
- Implementation: [`editor/src/app.rs`](../editor/src/app.rs), [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs), [`editor/src/config.rs`](../editor/src/config.rs).
- Test/reference suites: [`editor/src/app.rs`](../editor/src/app.rs), [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs)
- Example test symbols (suite references, not a coverage claim): `open_loads_in_background_then_hands_off_atomically`; `sibling_cards_are_separated_by_the_gap`.
- Contradictory check / remaining historical criteria: Second viewport absent; per-window positions/multimonitor acceptance not broadly established
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P3 extension, not next core workflow.

<a id="t-913"></a>
### T-913 — Name the thing under the cursor

**COMPLETE / NOT_TESTED / PUBLIC_ALPHA** · Origin: T-913.

- Delivered scope: Entity hover labels, preference and Alt reveal
- Entry point: Hover / Settings.
- Implementation: [`editor/src/ui/canvas/hover_label.rs`](../editor/src/ui/canvas/hover_label.rs), [`editor/src/config.rs`](../editor/src/config.rs).
- Test/reference suites: [`editor/src/config.rs`](../editor/src/config.rs)
- Example test symbols (suite references, not a coverage claim): `recent_files_are_deduplicated_and_capped`.
- Contradictory check / remaining historical criteria: Overlap/DPI/delay/accessibility not exhaustively qualified No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences.
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="cap-01"></a>
### CAP-01 — Global document diagnostics

**PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-702 / T-818.

- Delivered scope: Deterministic read-only global inspection, typed locations, severity/code/context, cache/document tickets, coalesced refresh and navigation with stale refusal.
- Entry point: Diagnostics badge and closable egui list; no automatic popup.
- Implementation: [`document/src/diagnostics.rs`](../document/src/diagnostics.rs), [`editor/src/diagnostics.rs`](../editor/src/diagnostics.rs).
- Test suites: [`document/src/diagnostics/tests.rs`](../document/src/diagnostics/tests.rs), [`editor/src/diagnostics/tests.rs`](../editor/src/diagnostics/tests.rs), native lifecycle/preflight tests. Includes egui clicks in memory and 500-bone CPU counters.
- Remaining: Explicit selection without camera reframing; assets/document-level and ambiguous locations are report-only. Full scans are synchronous and decode in-memory images once per coalesced revision. No broad latency or GUI guarantee.
- Latest checkpoint: T-818 local cumulative executable, internal version 0.2.0-alpha.1 unchanged.
- Recommended action: Deferred manual qualification; publication preflight remains authoritative.

<a id="cap-06"></a>
### CAP-06 — Second render viewport

**ABSENT / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-910.

- Delivered scope: Canvas is excluded from detached panes; second viewport not implemented
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/app.rs`](../editor/src/app.rs), [`editor/src/ui/mod.rs`](../editor/src/ui/mod.rs).
- Test/reference suites: No dedicated executed test identified for this absent/unknown scope; static search only.
- Contradictory check / remaining historical criteria: Noncanvas windows already share one AppState and must not be redeveloped
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.

<a id="cap-15"></a>
### CAP-15 — Production performance budget

**UNKNOWN / NOT_TESTED / UNKNOWN** · Origin: T-706.

- Delivered scope: No broad integrated-GPU/500-bone/300-row frame-time acceptance available
- Entry point: Existing surface described in code, or no entry point for absent scope.
- Implementation: [`editor/src/ui/canvas/renderer.rs`](../editor/src/ui/canvas/renderer.rs), [`core/src/pose.rs`](../core/src/pose.rs).
- Test/reference suites: No dedicated executed test identified for this absent/unknown scope; static search only.
- Contradictory check / remaining historical criteria: Historical bounded runtime/fixture runs are not a universal budget
- Latest relevant checkpoint: Exact T-816 source / accepted historical evidence.
- Recommended action: Retain the existing subset; no implementation in this audit. Priority: See linked historical task remainder.


<a id="distribution"></a>
## Distribution

<a id="t-801"></a>
### T-801 — Samples + documentation

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-801.

- Delivered scope: Approved Jade alpha sample, source examples and first-rig guide
- Entry point: Samples / first-rig guide.
- Implementation: [`samples/tentacle.ankh`](../samples/tentacle.ankh), [`samples/walker.ankh`](../samples/walker.ankh), [`docs/first-rig-in-10-minutes.md`](../docs/first-rig-in-10-minutes.md).
- Test/reference suites: [`editor/src/ui/inspector/t816_tests.rs`](../editor/src/ui/inspector/t816_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t816_ik_inspector_exposes_root_span_and_solver`.
- Contradictory check / remaining historical criteria: Not all 3 CC0 sample/PSD/GIF criteria; new user's exercise pending
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-802"></a>
### T-802 — Release packaging

**PARTIAL / AUTOMATED / PUBLIC_ALPHA** · Origin: T-802.

- Delivered scope: Published unsigned portable Windows ZIP, manifests and matching packages
- Entry point: Public portable ZIP.
- Implementation: [`docs/CANDIDATE_BUILD.md`](../docs/CANDIDATE_BUILD.md), [`docs/ALPHA-LICENSES.md`](../docs/ALPHA-LICENSES.md).
- Test/reference suites: [`docs/RELEASE_READINESS.md`](../docs/RELEASE_READINESS.md)
- Contradictory check / remaining historical criteria: No signed installer, Linux/macOS artifacts or broad hardware certification
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: Retain scoped implementation.

<a id="t-803"></a>
### T-803 — Web build

**DEFERRED / NOT_TESTED / NOT_DISTRIBUTED** · Origin: T-803.

- Delivered scope: WASM core compiles; native desktop editor remains product
- Entry point: No browser editor.
- Implementation: [`core/src/lib.rs`](../core/src/lib.rs), [`editor/src/main.rs`](../editor/src/main.rs).
- Test/reference suites: [`core/src/lib.rs`](../core/src/lib.rs)
- Contradictory check / remaining historical criteria: No browser authoring frontend/distribution; WASM core is not a web editor
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Work only on the explicit remainder; preserve existing paths. Priority: P3 extension, not next core workflow.

<a id="t-810-5"></a>
### T-810.5 — Simple Windows MP4 action reusing supervised CPU export worker

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-810.5.

- Delivered scope: Simple Windows MP4 action reusing supervised CPU export worker
- Entry point: File Export Video MP4.
- Implementation: [`editor/src/ui/video.rs`](../editor/src/ui/video.rs), [`editor/src/operators.rs`](../editor/src/operators.rs).
- Test/reference suites: [`editor/src/ui/video_t8105_tests.rs`](../editor/src/ui/video_t8105_tests.rs), [`editor/src/video_export/t8105_tests.rs`](../editor/src/video_export/t8105_tests.rs)
- Example test symbols (suite references, not a coverage claim): `t8105_mp4_entry_uses_selected_clip_and_safe_presets_without_editing_project`; `t8105_absolute_sampling_counts_endpoints_and_repeats`.
- Contradictory check / remaining historical criteria: External FFmpeg, opaque output/new-file only; no bundled encoder
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-811"></a>
### T-811 — Next Alpha release candidate

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-811.

- Delivered scope: Integrated accepted artifacts later published unchanged through T-812
- Entry point: Accepted integrated archive.
- Implementation: [`docs/CANDIDATE_BUILD.md`](../docs/CANDIDATE_BUILD.md), [`docs/FEATURES.md`](../docs/FEATURES.md).
- Test/reference suites: [`docs/RELEASE_READINESS.md`](../docs/RELEASE_READINESS.md)
- Contradictory check / remaining historical criteria: Historical candidate-pending prose superseded only for the exact published artifacts
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-1001"></a>
### T-1001 — Plugin host and declarative panels

**COMPLETE / AUTOMATED / PUBLIC_ALPHA** · Origin: T-1001.

- Delivered scope: Sandboxed QuickJS, named verbs and declarative native panels
- Entry point: Community panels.
- Implementation: [`plugins/src/lib.rs`](../plugins/src/lib.rs), [`plugins/src/panel.rs`](../plugins/src/panel.rs), [`editor/src/ui/plugin_panel.rs`](../editor/src/ui/plugin_panel.rs).
- Test/reference suites: [`plugins/src/lib.rs`](../plugins/src/lib.rs), [`plugins/src/panel.rs`](../plugins/src/panel.rs)
- Example test symbols (suite references, not a coverage claim): `a_plugin_builds_a_rig_through_the_same_verbs_a_menu_uses`; `the_shape_is_the_tag`.
- Contradictory check / remaining historical criteria: Not a general external package/security endorsement
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-1002"></a>
### T-1002 — MCP stdio server

**COMPLETE / AUTOMATED / SOURCE_ONLY** · Origin: T-1002.

- Delivered scope: Headless MCP stdio consumer reusing document/export/render
- Entry point: MCP stdio.
- Implementation: [`mcp/src/server.rs`](../mcp/src/server.rs), [`mcp/src/tools.rs`](../mcp/src/tools.rs), [`mcp/src/session.rs`](../mcp/src/session.rs).
- Test/reference suites: [`mcp/src/session.rs`](../mcp/src/session.rs), [`mcp/src/tools.rs`](../mcp/src/tools.rs)
- Example test symbols (suite references, not a coverage claim): `a_tool_with_nothing_open_says_what_to_do_about_it`; `the_tool_list_is_small_enough_to_read`.
- Contradictory check / remaining historical criteria: Not autonomous AI access or browser authoring; permissions remain caller-specific
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-804"></a>
### T-804 — Unofficial community fork provenance

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-804.

- Delivered scope: Unofficial community fork provenance, licenses and historical RC2 publication
- Entry point: Public unofficial forks.
- Implementation: [`README.md`](../README.md), [`LICENSE-MIT`](../LICENSE-MIT), [`docs/THIRD_PARTY_SOURCE_NOTICE.md`](../docs/THIRD_PARTY_SOURCE_NOTICE.md).
- Test/reference suites: [`README.md`](../README.md)
- Contradictory check / remaining historical criteria: No upstream endorsement or npm publication implied
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-812"></a>
### T-812 — Exact T-811 artifacts published as v0.2.0-alpha.1 with recorded hashes

**COMPLETE / MIXED / PUBLIC_ALPHA** · Origin: T-812.

- Delivered scope: Exact T-811 artifacts published as v0.2.0-alpha.1 with recorded hashes
- Entry point: Public v0.2.0-alpha.1 assets.
- Implementation: [`README.md`](../README.md), [`docs/FEATURES.md`](../docs/FEATURES.md).
- Test/reference suites: [`docs/RELEASE_READINESS.md`](../docs/RELEASE_READINESS.md)
- Contradictory check / remaining historical criteria: Local T-813–T-816 not included; unsigned Windows, no new platform certificate
- Latest relevant checkpoint: T-812 integration; audited exact T-816 source.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

<a id="t-817"></a>
### T-817 — Operational inventory

**COMPLETE / AUTOMATED / LOCAL_CANDIDATE** · Origin: T-817.

- Delivered scope: Operational inventory, exact historical archive and evidence register
- Entry point: Current documentation.
- Implementation: [`docs/TASKS.md`](../docs/TASKS.md), [`docs/FEATURE_INVENTORY.md`](../docs/FEATURE_INVENTORY.md), [`docs/ROADMAP_NEXT.md`](../docs/ROADMAP_NEXT.md).
- Test/reference suites: [`docs/TASKS.md`](../docs/TASKS.md)
- Contradictory check / remaining historical criteria: Documentary audit only; no product or graphical acceptance added
- Latest relevant checkpoint: T-817.
- Recommended action: Retain tested scoped capability; do not redevelop. Priority: Retain scoped implementation.

## Important source/report contradictions

1. The original T-811 pending-publication prose conflicts with T-812's verified
   publication record. Exact public artifacts are released; no local T-813–T-816
   binary was published. Public labels do not inherit local fixes.
2. `core/src/skeleton.rs` mentions SelectionSet in comments, but current structs,
   schema and UI implement hierarchy folders, not saved overlapping selection sets.
3. Onion skinning is implemented: renderer `onion_poses`/ghost passes and transport
   controls exist. T-824 supplies local theme/opacity settings; broad visual acceptance
   are not complete. The disabled path returns before additional pose evaluation;
   the existing test is not an instrumented universal performance counter.
4. Native physics code/fields exist in both languages. That does not activate the
   native export correctly in TypeScript or satisfy the adapters' alpha contract.
5. `.ankh.runtime` and the macroquad player are old planning targets, not files or
   tools users should expect. Current native export is JSON plus atlas PNG pages.
6. T-813 refusals protect unsupported unknown author data; they do not implement
   arbitrary opaque author-field preservation or a v0 migration.
7. T-816's example already existed. Do not generate another multibone sample merely
   because the first-rig GUI exercise is waiting for the maintainer.

## Bugs/gaps recorded without correction

**B01 / P1 — incomplete cross-document subtree transfer (T-817 finding).**
Addressed by the bounded T-819 explicit region transfer workflow. BoneClip remains
asset-free and File/Open intentionally resets its session clipboard; the new
reviewed import transports required bytes separately using native asset commands.
Original same-document Copy/Paste is retained. Advanced universal transfer is
still partial and explicitly refused, not claimed complete or silently dropped.

**B02 / P2 — transport FPS bypasses native history.** `ui/timeline/transport.rs`
writes `state.doc.meta.fps` directly on change, without a native command. Source
reproduction: change only project FPS in transport; no history command is dispatched.
`App::is_dirty` compares document snapshots, so this audit does **not** claim that
dirty tracking/autosave/close safety lose the change. Undo/revision consistency needs
a targeted headless regression in a separately authorized correction.

**B03 / P2 — bone-name context-menu usability.** Previously observed physically
and retained in T-816 evidence; T-817 did not retest Windows. The icon workaround
does not close that report. Keep it in the manual queue for precise target/focus
reproduction; do not treat hierarchy authoring as absent.

**P0:** no new demonstrated corruption/crash/security incident established by this
documentary audit. Resource-budget/hostile-file behavior remains unqualified; no
claim of absence of all P0 bugs. All findings remain unmodified.

## Provider qualification

| Profile | Native transport inspected | Actually qualified evidence | Boundary |
|---|---|---|---|
| OpenAI | Responses SSE, text/image/tool envelopes | Synthetic local HTTP tests | No new cloud/model run |
| Anthropic | Messages SSE, base64 image/tool use | Synthetic local HTTP tests | Not every Claude model certified |
| DeepSeek | Compatible streaming/image envelope | Synthetic local HTTP tests | Transport support does not establish actual endpoint/model vision |
| Ollama | Native chat NDJSON and images array | Synthetic local HTTP tests | No local model launched by T-817 |
| LM Studio | Compatible streaming envelope | Synthetic local HTTP tests | Per-destination declarations required |
| NInfer / Qwen | Compatible profile, bounded native tools | Mocks plus historical targeted real/maintainer-reported Jade scenario | Exact Qwen build not recorded; unsaved edited project excluded; no general 8-GB/artist certificate |

All six use independent metadata/image/edit consent, destination-bound capability
declarations and bounded proposals/approval. See [multimodal](native-ai-multimodal.md),
[weights](native-ai-weights.md), [assembly](native-ai-assembly.md). Their early
"pending" records are not erased; later narrow maintainer reports are time-scoped
in [manual QA](MANUAL_QA_PENDING.md).

<a id="t-818"></a>
### T-818 — Bounded global document diagnostics

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-702 / CAP-01.

- Delivered bounded scope: read-only rules, coalesced revision/metadata cache, badge/list, guarded selection and native tests.
- Implementation and test paths: see [Document diagnostics](document-diagnostics.md) and CAP-01 above.
- Remaining broader T-702 scope: universal camera framing, worst-case resource/latency and deferred manual qualification; no blanket completeness of all historical production diagnostics.
- Parent: exact T-817 `acc3f065c98856a8e734a22341637f29d2e38d9c`; local cumulative Windows build only, internal version unchanged.
- Next step: maintainer review; T-819 remains a proposal.

<a id="t-819"></a>
### T-819 — Safe cross-project region transfer

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · bounded refinement of T-909.

- Scope, commands, source/destination guards, native transaction and limitations:
  [Cross-project transfer](cross-project-transfer.md).
- Parent exact T-818 `6e139b6a5deb65fa6922704fc7bb3127f2a21cde`.
- Default-skin region/sequence bytes and compatible bone animation are supported;
  advanced rig/project merge is not. GUI qualification stays deferred.
- Prior source branches, binaries, historical oracles and releases remain intact.

<a id="t-820"></a>
### T-820 — Bezier tangent linking

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-704 / T-912 / CAP-02.

Bounded single-key junction linking, exact native Undo/Redo and unchanged
Save/Reload fields. [Contract](bezier-tangent-linking.md), [graph rules](graph-editor.md).
No public distribution or GUI claim. Preserve T-813–T-819; T-821 not started.

<a id="t-821"></a>
### T-821 — Editable existing FFD keys

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** · Origin: T-404 / T-209.

- Scope: one existing Deform key, exact owned mesh, native time/delete/interpolation;
  rigid and weighted influence-count arrays plus preserved empty resets.
- Entry: Dopesheet Deform row, click / horizontal drag / right-click FFD actions.
- Implementation: [native capture](../document/src/ffd_keys.rs), [command](../document/src/commands/ffd_cmds.rs), [session](../editor/src/ffd_keys.rs), [shared selection](../editor/src/key_selection.rs).
- Tests: [native/editor regression scenarios](../editor/src/ffd_keys/tests.rs), [real in-memory egui actions](../editor/src/ui/timeline/sheet/t821_tests.rs).
- Limits: [contract](ffd-key-editing.md); one key, no general clipboard or vector graph,
  linked/shared-source and ambiguous owner refusal. T-404/T-209 remain PARTIAL.
- GUI qualification is deferred cumulatively through T-821; no public release.

<a id="t-827"></a>
### T-827 — Large key selection validation performance

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for batch native capture.

- Delivered scope: Same ordered KeyClip/KeySelection results, fewer repeated dedup/timeline/name validations within one immutable capture.
- Implementation: [native capture](../document/src/clipboard/keys.rs), [operation-local indexes](../document/src/clipboard/keys/capture_batch.rs). Shared editor selection functions are unchanged.
- Tests: [exact differential and operation-count tests](../document/src/clipboard/keys/t827_tests.rs), [history/clipboard/FFD/I/O consumers](../editor/src/key_selection/t827_tests.rs), [headless large marquee](../editor/src/ui/timeline/t827_tests.rs).
- Limits: Existing graph/sheet membership and native edit/resolve scans remain; no general frame budget. Same single-FFD/mixed-selection policies and command mode/lock guards. 20 deferred manual checks; cumulative QA pending.
