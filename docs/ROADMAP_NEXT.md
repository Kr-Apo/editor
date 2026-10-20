# Next roadmap — proposals after T-817 review

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

The original five bounded development proposals below balance project diagnostics, real game-asset
reuse, animation authoring and PSD workflow. T-818 was subsequently authorized and implemented locally; T-819 and later remain **not authorized** by this audit. Manual T-813–T-816 QA is a parallel qualification queue, not a missing-feature
development task, and is not an artificial prerequisite imposed on the maintainer.

No demonstrated new P0 incident was found. P0 prevention/triage remains first when
an actual reproducible issue appears. These proposals are P1 central workflow or
P2 production usability; P3 platforms/localization/second viewport stay deferred.

## T-818 — Document diagnostics and offender navigation

Historical IDs: **T-702** · Priority **P1** · Effort: Medium, bounded rule set first.

- **T-818 implementation:** Local bounded diagnostics, revision/metadata debounce, badge/list and stale-safe navigation now exist. This replaces the historical absence finding for the local candidate only.
- **Already done — preserve:** Reuse export preflight schema/rules and native document reads; do not create another export pipeline.
- **Bounded work:** Read-only structured rules for missing asset/attachment, invalid clip/time, stale deform vertex counts, draw-order and missing IK refs; debounced revision-keyed results; badge/dock and select offender without authoring changes.
- **Dependencies:** Current T-816 document/session and existing export preflight; no need to wait artificially for GUI campaign.
- **Regression risk:** Medium: false positives/stale async results and accidental duplication of export safety rules.
- **Measurable acceptance:** Each rule has headless valid/invalid tests; diagnostics cannot mutate data/history or repair implicitly; old results cannot select a newer document; no unjustified warning on approved samples; existing export staging/consent unchanged; manual dock QA explicitly pending.
- **Runtime boundary:** Diagnostics can explain adapter exclusions but cannot certify an unknown consumer.
- **Migration boundary:** No format migration; session-derived results only.

## T-819 — Safe bounded cross-document subtree transfer

Historical IDs: **T-909 / T-209** · Priority **P1** · Effort: Medium/high; split before adding mesh transfer.

- **T-819 local implementation:** Explicit read-only source loading and reviewed region/image/bone-key transfer now address bounded B01. File/Open and existing clipboard behavior are retained.
- **Already done — preserve:** Keep PasteBones, internal constraint/bone-key capture and existing one-step history.
- **Bounded work:** Explicit reviewed copy-to-project operation for regions first, named conflict/asset ownership policy, image bytes and native target transaction. Refuse unresolved weighted/linked/FFD or external-reference cases until representable.
- **Dependencies:** T-818 diagnostics useful; existing PasteBones/asset commands and target identity checks.
- **Regression risk:** High: asset names, shared references, ids/remaps and partial transfer; narrow first scope required.
- **Measurable acceptance:** Two isolated documents with real region image and bone keys transfer coherently; conflict report before writes; refusal keeps full document/Redo; three Undo/Redo cycles; Save/Reload and native v2 export retain pixels/keys; no guessed external constraints or silent image loss.
- **Runtime boundary:** Result only uses already-supported native region/bone tracks; no new adapter capability.
- **Migration boundary:** No new project fields required for bounded region transfer; broader meshes need separate review.

## T-820 — Linked Bezier handle editing modes

Historical IDs: **T-704 / T-912** · Priority **P2** · Effort: Medium.

- **Verified gap:** CAP-02: independent/numeric handles exist; no aligned/mirrored link behavior.
- **Already done — preserve:** Reuse T-814 selection/drafts/SetInterp and existing free handles; do not rebuild numeric tangents.
- **Bounded work:** Explicit independent/aligned/mirrored editing policy for suitable scalar segments, using correct normalized units and ending-key ownership; handle flat/zero-duration cases by refusal or explicit semantics.
- **Dependencies:** T-814 native SetInterp, current context leases and exact fields.
- **Regression risk:** Medium: normalized handles do not directly encode geometric slope for unequal time/value spans.
- **Measurable acceptance:** Red tests for missing link behavior; unequal segment durations/value ranges/flat segments and overshoots covered; changing one handle respects chosen mathematical policy; no sampler change by default; one gesture/Apply Undo, stale-context refusal; saved curves preserve current wire format.
- **Runtime boundary:** Existing Bezier handles only; validate sampled exported curves without promising new runtime features.
- **Migration boundary:** Prefer session-only edit policy; stop for review if persistent link semantics are required.

## T-821 — Editable existing FFD keys in the dopesheet

Historical IDs: **T-209 / T-404** · Priority **P1** · Effort: Medium/high; small staged acceptance.

- **Verified gap:** FFD authoring/evaluation exists but complex rows have no editable key addresses or block clipboard path.
- **Already done — preserve:** Reuse Deform tracks, existing edit router, KeyClip/PasteKeys transactions where applicable; preserve T-209.2/.3.
- **Bounded work:** First expose select/move/delete/interpolation for keys of one existing exact mesh topology; no vertex generation/retopology, no mirrored FFD or automatic target guessing.
- **Dependencies:** Existing FFD and key selection/history; define typed deform address and vertex-count validation before UI.
- **Regression risk:** High: index/topology/linked ownership and curves; separate selection/edit phase from cross-mesh paste if needed.
- **Measurable acceptance:** Single-mesh FFD keys editable and undoable; mismatched topology/skin/linked cases refuse without data/Redo loss; three cycles and Save/Reload; scalar/pose/mirrored bone clipboard regressions retained; no format/evaluator changes.
- **Runtime boundary:** Existing native v2 deform encoding only; runtime render qualification remains separately scoped.
- **Migration boundary:** None for editing existing tracks; any new payload/address architecture needs design review.

## T-822 — Explicit PSD path-change resolution for simple regions

Historical IDs: **T-302** · Priority **P2** · Effort: Medium.

- **Verified gap:** Exact-path refresh exists; a renamed/reparented layer becomes added+missing without a user-approved mapping.
- **Already done — preserve:** Reuse existing PSD preview/diff/reimport guarded transaction; do not implement another importer.
- **Bounded work:** Optional explicit old-path to new-path mapping for unique unshared region images; display before/after and preserve manually authored transforms/animation. Do not infer by leaf name or visual resemblance.
- **Dependencies:** Existing psd_reimport/reimport_guard and origin/revision validation; T-818 diagnostics helps explain refusal.
- **Regression risk:** High: provenance ambiguity and image reuse; retain current refusals for mesh/sequence/linked/shared/flattened cases.
- **Measurable acceptance:** Renamed/reparented unique layer can be explicitly matched; no mapping means existing behavior unchanged; ambiguous paths and late revisions refuse atomically; no PSD source write; three Undo/Redo and Save/Reload preserve rig/keys/art; exact-path and nested-import regressions retained.
- **Runtime boundary:** Authoring provenance change only; no adapter or native format extension.
- **Migration boundary:** Reuse psd_layer_paths; no stable Photoshop UUID invented.

## Contradictory review of every proposal

| Question | T-818 | T-819 | T-820 | T-821 | T-822 |
|---|---|---|---|---|---|
| Real missing/incomplete scope? | Global live diagnostics absent | Cross-document asset workflow incomplete | Link modes absent | FFD-key addresses/editing incomplete | Explicit path-change match absent |
| Prior work already solved it? | Export preflight only | Intra-document paste only | Free/numeric tangents only | FFD drag/evaluation only | Exact-path refresh only |
| Exists under another name? | Load/IK warnings are narrower | PasteBones is useful substrate | SetInterp stores handles, not linked editing | Deform keys exist, read-only row is not editable UI | Reimport guards exist, no user mapping |
| Original criterion still suitable? | Narrow rule set, no universal validator | Start regions; broad move/retarget deferred | Define normalized geometry explicitly | Existing topology only | No speculative automatic reconciliation |
| Already automatically tested? | Subrules/preflight, not whole dock | Subtree/key commands, not full assets | Independent handles yes | Core FFD yes | Exact-path yes |
| Only manual QA missing? | No | No | No | No | No |
| Runtime scope? | Informational limits | Existing region subset | Existing curves | Existing FFD | Provenance only |
| Migration? | No | None for first subset | Prefer session-only policy | None for existing tracks | Existing path table |
| Can ship locally without public changes? | Yes | Yes | Yes | Yes | Yes |
| Why this priority? | Prevent silent workflow ambiguity and explain actual defects | Reuse rigged game assets safely | Daily animation control | Finish existing deform authoring | Avoid re-rigging renamed simple artwork |

## Why T-818 first

A live, read-only diagnostic view fills a proven T-702 gap and gives artists
actionable errors while preserving current save/export safeguards. A standalone
implementation prompt should pin exact T-817 commit, reuse export/preflight rule
building blocks, keep derived results session-only, limit the first rule set above,
require red/green headless stale-result/history tests and deliver a separate local
candidate. No repair, serializer/runtime change or new export pipeline belongs in it.

## Not another development queue

Retain the existing key dots/clipboard/pose/mirror, video/GIF, PSD nesting/exact-path
refresh, onion renderer, tangent numerics, geometry numerics, IK/weight brush and
recovery code. Performance budgets, localization, browser editor, second viewport,
advanced runtime constraints/physics/blends and broad platform certification remain
explicit later decisions, not hidden commitments in one of these five missions.
See [manual register](MANUAL_QA_PENDING.md) and the [inventory](FEATURE_INVENTORY.md).

## After T-818

Review the local diagnostics candidate and preserve the pending manual campaign.
T-819 was subsequently authorized and is now a local bounded transfer candidate.
T-818 alone did not close B01, B02, B03 or every T-702
camera/performance criterion. The 200 ms coalescing interval is a scheduling
choice, not a guarantee that arbitrary scans complete within that duration.

## After T-819

Review the local transfer candidate; cumulative manual T-813–T-819 qualification
remains deferred. T-820 linked Bezier handles is the next proposed development
mission, not started or authorized by this checkpoint. Advanced T-909 transfer
requires separate scope review; no universal retargeting claim is added.
