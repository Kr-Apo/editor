# Current task status through T835 / T836 preparation

This checkout is the **T836 local pre-alpha preparation**, based on accepted T835 `514c6aa8a6eeae8bb9dcb8c22e0dc35c48bb2cc2`. No product feature, version, format, runtime or public artifact is changed. The accepted public Alpha is **v0.2.0-alpha.1**, editor `9233b8e888c811e7c71300101f865a9e74b3f489`, runtime `c4c38df269cfcb30f941221454b7202f40d3fa5b`, as recorded by T812; this is not a fresh remote audit. RC2 remains a different, frozen archive.

Use [Jade Sentinel](jade-sentinel.md) as the only active user character. [Current local workflow](prealpha-workflow.md), [compatibility](release-compatibility.md), and [manual campaign](MANUAL_QA_PENDING.md) distinguish implemented features from physical Windows acceptance. **444 manual scenario instances remain PENDING MANUAL REVIEW**. Baseline T835:2,133 Rust passes/4 historical ignores; T836 results are in its delivery, not invented here. No real inference, paid call, GUI or GPU certification.

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

</details>

## Fixed historical population and current result

| Status | T-817 | T-831 audited | Delta |
|---|---|---|---|
| COMPLETE | 58 | 62 | +4 |
| PARTIAL | 37 | 33 | -4 |
| ABSENT | 1 | 1 | +0 |
| SUPERSEDED | 3 | 3 | +0 |
| DEFERRED | 2 | 2 | +0 |
| UNKNOWN | 0 | 0 | +0 |
| Total | 101 | 101 | 0 |

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

There are 101 historical IDs,3 original context IDs,28 original refinement records and 14 later mission records. They are separate, overlapping populations. [Historical archive](TASKS-before-T817.md) is byte-identical; [older notes](checkpoint-journal-before-T832.md) are a provenance journal, not current absences.

## Changes since T-817

| Historical ID | Implementation change | Reason |
|---|---|---|
| T-703 | PARTIAL → COMPLETE | Category change justified individually: The original ghost-generation/tint/falloff/step/selection scope is implemented. T-824 closes theme/opacity and boundary gaps; only Windows appearance/scrubbing acceptance remains. Session persistence and physics ghost simulation were not original T-703 requirements. |
| T-704 | PARTIAL → COMPLETE | Category change justified individually: All retained scalar-curve feature bullets exist: views, property/time/value curves, handles/history, colors/value axis/fit, normalized overlay, tie modes and rectangle selection. Normalize View Only fulfills the requested overlay toggle; normalized author editing and multi-junction linking are not added historical requirements. Real usability stays pending. |
| T-806 | PARTIAL → COMPLETE | Category change justified individually: The retained scope is consented viewport preview/one-send observation plus typed first bone tools. Capture, bounded coordinates/transports, proposals, approval/refusal and atomic history exist. General artistic placement or every-model certification is validation/qualification, not an additional implemented tool. Historical narrow maintainer feedback does not pass the local campaign. |
| T-908 | PARTIAL → COMPLETE | Category change justified individually: Source already provides N-bone IK chain/span/solver explanations, visible brush controls/help, first-rig/advantages documentation and the 8-bone Tentacle sample. T-817 conflated the pending new-user exercise with missing implementation. The exercise remains pending; no new product work is credited to T-832. |

## Historical task matrix — exactly 101 IDs

### Foundations

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-001](FEATURE_INVENTORY.md#t-001) — License + repo hygiene | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not a blanket audit of all transitive dependency licenses |
| [T-002](FEATURE_INVENTORY.md#t-002) — CI skeleton | PARTIAL | NOT_TESTED | SOURCE_ONLY / SOURCE_ONLY | Actions disabled; no fresh remote green run; actual Clippy job lacks historical -D warnings |
| [T-003](FEATURE_INVENTORY.md#t-003) — ADR seed | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Some historical format statements are superseded by format-spec; not every old design sketch is current |
| [T-101](FEATURE_INVENTORY.md#t-101) — Stable entity IDs | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Names cross disk boundaries; unresolved foreign references still need diagnostics |
| [T-102](FEATURE_INVENTORY.md#t-102) — Affine2 transform math | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Singular transforms remain noninvertible; no universal arbitrary-float geometry guarantee |
| [T-103](FEATURE_INVENTORY.md#t-103) — Pose extraction + evaluate skeleton | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Stateful physics requires its separate accumulator; direct seek is not a simulation |
| [T-104](FEATURE_INVENTORY.md#t-104) — Constraint system + correct IK blending | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Runtime common alpha excludes constraints; geometric singularities are not repaired |
| [T-105](FEATURE_INVENTORY.md#t-105) — Skin layer | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not arbitrary game-runtime skin composition or linked deformation qualification |
| [T-106](FEATURE_INVENTORY.md#t-106) — Animation model + sampling | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Extreme finite handles can overflow sampler; no universal f32-range guarantee |
| [T-107](FEATURE_INVENTORY.md#t-107) — Editor Document/Session split + command undo | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Undo history is not persisted; this closes only the identified FPS bypass, not every metadata editing surface. |
| [T-108](FEATURE_INVENTORY.md#t-108) — `formats` crate + .ankh v1 save/load | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Prototype ZIP unsupported; no authentic v0 migration; complete scope is current v1 only |
| [T-707](FEATURE_INVENTORY.md#t-707) — Crash recovery, update check, feedback | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA / PUBLIC_ALPHA | No panic-time live-document dump, recovery browser or complete update/feedback package |
| [T-813](FEATURE_INVENTORY.md#t-813) — Format Compatibility & Project Reliability | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / LOCAL_CANDIDATE | No arbitrary structural repair, hostile-resource budget or power-loss certificate; GUI pending |

### Animation

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-201](FEATURE_INVENTORY.md#t-201) — Dopesheet panel | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Complex track addresses belong to the separately partial T-209/FFD workflow; local T-814 selection refinements still await GUI review. |
| [T-202](FEATURE_INVENTORY.md#t-202) — Playback + auto-key | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Pending previews are not keys; Windows playback/focus qualification remains pending. |
| [T-203](FEATURE_INVENTORY.md#t-203) — Bezier interpolation UI | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | T-704 scalar feature list now implemented locally; new property/FFD/normalized editing is not universal. |
| [T-204](FEATURE_INVENTORY.md#t-204) — Draw-order animation UI | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Generic dopesheet order-key clipboard remains excluded |
| [T-205](FEATURE_INVENTORY.md#t-205) — Slot timelines UI | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Static dark tint is separate; no animated-dark channel is invented |
| [T-207](FEATURE_INVENTORY.md#t-207) — Setup mode / Animate mode | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Every future edit surface still needs mode semantics; legacy metadata writes audited separately |
| [T-208](FEATURE_INVENTORY.md#t-208) — Animation manager | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | T-835 adds persistent adjacent Move Up/Down with native Undo/Redo. Original manager duration-in-frames display and switching to Animate on manager-row selection remain unimplemented; no claim of full T-208 completion. |
| [T-209](FEATURE_INVENTORY.md#t-209) — Clipboard: copy / paste / duplicate | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | T-834 supports owned same-mesh FFD key blocks on an explicit existing row. Cross-project/cross-mesh, linked/shared/overridden sources, empty-row creation, constraints, draw-order and event blocks remain excluded. Scalar replacement semantics remain unchanged. |
| [T-209.2](FEATURE_INVENTORY.md#t-209-2) — Validated normal/mirrored multibone pose paste in one native transaction | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Auto-key-off retains preview policy; not a retargeter |
| [T-209.3](FEATURE_INVENTORY.md#t-209-3) — Explicit bone-row mirrored offsets; signs | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Offsets, not asymmetric absolute setup poses; no slot/FFD/event mirror |
| [T-210](FEATURE_INVENTORY.md#t-210) — Keyed-value affordances in the inspector | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Does not add IK key dots or every future property; no whole-app accessibility certificate |
| [T-211](FEATURE_INVENTORY.md#t-211) — Pose tools | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | T-833 adds explicit selected-bone transform-key offset, separate from whole-clip Retime. K keys pending poses; no dedicated Shift+K key-all-changed binding identified. Arbitrary KeyRefs/constraint/slot offsets and selected time scaling remain out of scope. |
| [T-703](FEATURE_INVENTORY.md#t-703) — Onion skinning | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | No product gap in retained T-703 scope. Manual visual contrast/scrubbing acceptance pending; no stateful physics ghosts or preference persistence promised. |
| [T-704](FEATURE_INVENTORY.md#t-704) — Curve editor refinement | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | No remaining product bullet in retained scalar scope. Windows ergonomics pending. Normalized editing, multi-junction linking, FFD/Path Graph and middle-button pan are separate extensions, not retroactively added requirements. |
| [T-814](FEATURE_INVENTORY.md#t-814) — Shared key selection | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / LOCAL_CANDIDATE | Broader T-704 stays partial; combined graph edits retain existing granularity; GUI pending |
| [T-905](FEATURE_INVENTORY.md#t-905) — Non-destructive timeline offset | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not arbitrary per-track/per-selection offsets for slots/constraints/deforms |
| [T-906](FEATURE_INVENTORY.md#t-906) — Timeline markers | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not runtime event callbacks; no fresh desktop certification |
| [T-912](FEATURE_INVENTORY.md#t-912) — Graph editor ergonomics review | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | The literal historical opt-in snapping criterion is still not met: Raw frame snapping is the default, Alt temporarily frees it, and the badge is visible. This policy gap is distinct from pending Windows readability/usability. Linked/normalize/marquee are no longer absent. |

### Rigging

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-206](FEATURE_INVENTORY.md#t-206) — Hierarchy panel UX | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Original hierarchy/reparent/lock scope exists; separate bone-name context-menu usability issue B03 remains, not an absent reparent system. |
| [T-307](FEATURE_INVENTORY.md#t-307) — Attachment inspector — per-attachment properties | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Mesh/sequence image replacement is a separate unimplemented extension; not an absent region inspector. |
| [T-501](FEATURE_INVENTORY.md#t-501) — Transform constraint | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not part of common Pixi/Phaser alpha contract |
| [T-502](FEATURE_INVENTORY.md#t-502) — Path attachment + path constraint | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Animated PathSpacing and geometric Bezier handles remain absent; data/runtime exports refuse PathPosition. Graph/group/clipboard Path keys are deferred. |
| [T-503](FEATURE_INVENTORY.md#t-503) — Physics constraint | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Rust fixed-step contract is complete in CAP-08; original export/playback parity not fulfilled: native TS physics activation incompatible and rendered exports explicitly refuse it. |
| [T-504](FEATURE_INVENTORY.md#t-504) — IK completeness | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA / PUBLIC_ALPHA | No unique-solution guarantee or new IK key dots; runtime IK excluded |
| [T-505](FEATURE_INVENTORY.md#t-505) — Slot & bone presentation depth | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not every presentation variant or animated dark tint; viewport acceptance incomplete |
| [T-506](FEATURE_INVENTORY.md#t-506) — Event timeline | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | t=0 ignored by current runtime policy; audio metadata is not audio playback |
| [T-507](FEATURE_INVENTORY.md#t-507) — Skin manager + multi-skin composition | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Game adapters use one compatible skin; arbitrary multi-skin/mesh-image workflows unqualified |
| [T-508](FEATURE_INVENTORY.md#t-508) — Former performance task renumbered into T-706 | SUPERSEDED | AUTOMATED | SOURCE_ONLY / SOURCE_ONLY | Historical alias only, not another performance implementation |
| [T-816](FEATURE_INVENTORY.md#t-816) — Rigging Discoverability & First-Run Experience | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / LOCAL_CANDIDATE | Artist discovery exercise pending; no new solver/runtime capability |
| [T-904](FEATURE_INVENTORY.md#t-904) — Saved selection sets (not hierarchy folders) | ABSENT | NOT_TESTED | NOT_DISTRIBUTED / NOT_DISTRIBUTED | Stale SelectionSet comments are not an implementation; new optional schema needed if later authorized |
| [T-907](FEATURE_INVENTORY.md#t-907) — Bone length edit carries its children | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Remembered carry preference/full canvas-length interaction not identified |
| [T-908](FEATURE_INVENTORY.md#t-908) — Make the existing advantages discoverable | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | No product gap in retained discovery surfaces/sample scope. New-user no-source Windows exercise remains pending; not a universal discoverability certificate. |
| [T-909](FEATURE_INVENTORY.md#t-909) — Rig transfer between skeletons | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Explicit Move, general meshes/FFD/path/clipping/physics/alternate-skin or advanced animation transfer remain unavailable/refused. Local transforms are preserved, not arbitrary world pose; this is not retargeting. |
| [T-911](FEATURE_INVENTORY.md#t-911) — Physics stability harness | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No complete debug overlay or broad unrelated-edit interaction qualification |

### Assets / PSD

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-301](FEATURE_INVENTORY.md#t-301) — Asset database + image drop-import | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Project portability requires sidecar; broad texture-eviction/performance remains T-706 |
| [T-302](FEATURE_INVENTORY.md#t-302) — PSD import | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Bulk/multiple missing path mapping, complex/shared/mesh/FFD/sequence refresh, automatic added/deleted-layer reconciliation and Photoshop effects/masks remain outside qualified workflow. |
| [T-303](FEATURE_INVENTORY.md#t-303) — foreign armature importer | SUPERSEDED | AUTOMATED | SOURCE_ONLY / SOURCE_ONLY | Do not restore a second built-in importer; package licensing/availability remain external |
| [T-304](FEATURE_INVENTORY.md#t-304) — Startup window | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No first-user study of local T-816; unknown structural input limits remain |
| [T-305](FEATURE_INVENTORY.md#t-305) — Atlas / spritesheet import modal | COMPLETE | NOT_TESTED | PUBLIC_ALPHA / PUBLIC_ALPHA | No guarantee for every vendor atlas variant |
| [T-306](FEATURE_INVENTORY.md#t-306) — Asset relink + external-source workflow | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Moving only project does not move sidecar; missing assets need explicit resolution |
| [T-1003](FEATURE_INVENTORY.md#t-1003) — Move foreign formats to community JavaScript plugins | COMPLETE | AUTOMATED | SOURCE_ONLY / SOURCE_ONLY | Missing package differs from broken builtin; no generic Godot/foreign fidelity promise |

### Meshes

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-401](FEATURE_INVENTORY.md#t-401) — Mesh attachment editing UI | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA / PUBLIC_ALPHA | No universal snapping or arbitrary linked-source/topology edit safety |
| [T-402](FEATURE_INVENTORY.md#t-402) — Auto mesh tracing | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | FFD-dependent, linked/shared/overridden or pinned retrace is refused; no topology/FFD migration. General <100 ms release/artist preservation budget remains unqualified beyond one ring fixture. |
| [T-403](FEATURE_INVENTORY.md#t-403) — Weight painting + bind compensation | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA / PUBLIC_ALPHA | Complete native scoped brush; not autorigging or every-project visual fidelity; T-816 GUI pending |
| [T-404](FEATURE_INVENTORY.md#t-404) — Deform timelines | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Stale topology is reported by live diagnostics and editing refuses it; the author loader still directly constructs Deform rather than invalidating its track at load. No general topology/FFD remap or ambiguous/shared ownership edit. Vector clipboard belongs to T-209 and alone is not the reason for this status. |
| [T-405](FEATURE_INVENTORY.md#t-405) — Clipping attachments | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Common adapters support bounded sequential masks, not arbitrary nested intersections |
| [T-815](FEATURE_INVENTORY.md#t-815) — Viewport Precision & Numeric Geometry Editing | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / LOCAL_CANDIDATE | Not every handle/expression/snap or linked-source edit; GUI pending |
| [T-902](FEATURE_INVENTORY.md#t-902) — Numeric entry for every vertex and handle | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Not every draggable point, expression parser or snap-to-neighbour |

### Formats / exports

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-601](FEATURE_INVENTORY.md#t-601) — Offscreen render + image / spritesheet / sequence export | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Default skin/Normal/no physics; original background/performance/viewport criteria not all closed |
| [T-601.1](FEATURE_INVENTORY.md#t-601-1) — PNG retained | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | JPEG lossy tolerance/no alpha; no runtime format change |
| [T-601.2](FEATURE_INVENTORY.md#t-601-2) — Opt-in common camera/global bounds | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Skin/blend/physics exclusions persist; not arbitrary renderer fidelity |
| [T-601.3](FEATURE_INVENTORY.md#t-601-3) — Selected-clip batches | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not crash-atomic multi-file commit or unrestricted memory/performance guarantee |
| [T-602](FEATURE_INVENTORY.md#t-602) — Video export via ffmpeg | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | No audio/GIF alpha/physics or every-player guarantee; external encoder required |
| [T-602.1](FEATURE_INVENTORY.md#t-602-1) — Raw RGB CPU video to H.264 | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Opaque background, no bundled encoder; hard-link filesystem requirement |
| [T-602.2](FEATURE_INVENTORY.md#t-602-2) — Two-pass GIF palette | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No GIF alpha or arbitrary exact FPS promise |
| [T-602.3](FEATURE_INVENTORY.md#t-602-3) — CRF/bitrate | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Native opener injected in tests; RGB requires player consent; not hardware encoding |
| [T-603](FEATURE_INVENTORY.md#t-603) — Original hardcoded-exporter plan replaced by user-authored templates/preset | SUPERSEDED | AUTOMATED | SOURCE_ONLY / SOURCE_ONLY | Do not create another hardcoded exporter; aggregate library still partial |
| [T-603a](FEATURE_INVENTORY.md#t-603a) — Atlas bake | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Common runtime excludes rotated atlases even if custom presets differ |
| [T-603b](FEATURE_INVENTORY.md#t-603b) — Template engine + export runner | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No filesystem-wide atomicity or guaranteed power-loss durability |
| [T-603c](FEATURE_INVENTORY.md#t-603c) — Native runtime format, authored as a template | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Native v2 does not encode animated PathPosition; no new runtime field or compatibility claim. Existing supported profile remains unchanged. |
| [T-603d](FEATURE_INVENTORY.md#t-603d) — Export panel | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not a continuously updated global diagnostics dock |
| [T-603e](FEATURE_INVENTORY.md#t-603e) — Preset library | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No qualified Godot preset; foreign-format plugins remain optional |
| [T-605](FEATURE_INVENTORY.md#t-605) — Format spec finalization + migration policy | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Authentic v0 migration still unqualified; no universal hostile-resource budget. T-831 keeps author/envelope 1 but older readers reject the new variant; future release must communicate feature-level compatibility. |

### PixiJS / Phaser

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-604](FEATURE_INVENTORY.md#t-604) — Runtime integration | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | In-workspace Rust player plan superseded; no universal runtime parity |
| [T-810](FEATURE_INVENTORY.md#t-810) — Phaser common-profile corrections and public playback API aligned to shared | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No new all-feature parity; detailed external runtime rows below |

### UX / tools

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-701](FEATURE_INVENTORY.md#t-701) — Settings, keymap, autosave | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Original exhaustive settings list/restart acceptance incomplete; unsaved-doc autosave policy retained |
| [T-702](FEATURE_INVENTORY.md#t-702) — Diagnostics / warnings | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | Automatic select-and-frame is absent: camera intentionally unchanged. Large image scans remain synchronous without a general 500-bone/asset completion budget. Export preflight remains independently authoritative. |
| [T-705](FEATURE_INVENTORY.md#t-705) — Localization | DEFERRED | NOT_TESTED | NOT_DISTRIBUTED / NOT_DISTRIBUTED | i18n implementation absent and deliberately deferred; no locale certificate |
| [T-706](FEATURE_INVENTORY.md#t-706) — Performance pass | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / SOURCE_ONLY | No general xtask bench/trend or integrated-GPU 500-bone/300-row 60 fps certificate. Large commands/model/payload work remains; debug medians are not budgets. |
| [T-708](FEATURE_INVENTORY.md#t-708) — Viewport interaction polish | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE / PUBLIC_ALPHA | No universal snapping/expression-on-drag/pivot editing for every attachment; framing acceptance incomplete |
| [T-709](FEATURE_INVENTORY.md#t-709) — Accessibility & window UX | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No whole workflow keyboard/contrast/colorblind audit or exhaustive icon-tooltip certificate |
| [T-901](FEATURE_INVENTORY.md#t-901) — Bulk rename with sequential numbering | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not slot/attachment/constraint rename, regex or all path-safety variants |
| [T-903](FEATURE_INVENTORY.md#t-903) — Isolation mode | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Optional hierarchy/dopesheet filters/configurable opacity not fully qualified |
| [T-910](FEATURE_INVENTORY.md#t-910) — Multi-window / multi-monitor | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Second viewport absent; per-window positions/multimonitor acceptance not broadly established |
| [T-913](FEATURE_INVENTORY.md#t-913) — Name the thing under the cursor | COMPLETE | NOT_TESTED | PUBLIC_ALPHA / PUBLIC_ALPHA | Overlap/DPI/delay/accessibility not exhaustively qualified No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences. No dedicated hover-interaction test found; Config tests cover only preferences. |

### Distribution

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-801](FEATURE_INVENTORY.md#t-801) — Samples + documentation | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not all 3 CC0 sample/PSD/GIF criteria; new user's exercise pending |
| [T-802](FEATURE_INVENTORY.md#t-802) — Release packaging | PARTIAL | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | No signed installer, Linux/macOS artifacts or broad hardware certification |
| [T-803](FEATURE_INVENTORY.md#t-803) — Web build | DEFERRED | NOT_TESTED | NOT_DISTRIBUTED / NOT_DISTRIBUTED | No browser authoring frontend/distribution; WASM core is not a web editor |
| [T-810.5](FEATURE_INVENTORY.md#t-810-5) — Simple Windows MP4 action reusing supervised CPU export worker | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | External FFmpeg, opaque output/new-file only; no bundled encoder |
| [T-811](FEATURE_INVENTORY.md#t-811) — Next Alpha release candidate | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | Historical candidate-pending prose superseded only for the exact published artifacts |
| [T-1001](FEATURE_INVENTORY.md#t-1001) — Plugin host and declarative panels | COMPLETE | AUTOMATED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not a general external package/security endorsement |
| [T-1002](FEATURE_INVENTORY.md#t-1002) — MCP stdio server | COMPLETE | AUTOMATED | SOURCE_ONLY / SOURCE_ONLY | Not autonomous AI access or browser authoring; permissions remain caller-specific |

### IA

| ID / scope | Implementation | Validation | Availability / public base | Actual remainder / limit |
|---|---|---|---|---|
| [T-805](FEATURE_INVENTORY.md#t-805) — Native conversational assistant | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | Not autonomous autorigging; model prose can be wrong; no generic cloud/model certificate |
| [T-805.1](FEATURE_INVENTORY.md#t-805-1) — Fixed composer | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | Historical Qwen real run is binary/model-specific; no new T-816 inference qualification |
| [T-806](FEATURE_INVENTORY.md#t-806) — Viewport vision and first native rigging tools | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA / PUBLIC_ALPHA | No product gap in retained native tool/consent scope; real vision/placement/model qualification remains pending and is not generalized from old narrow feedback. |
| [T-807](FEATURE_INVENTORY.md#t-807) — Controlled native character assembly | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | Weighted meshes are not regions; unrepresentable/constrained cases refused |
| [T-808](FEATURE_INVENTORY.md#t-808) — Guarded mesh weight assistance | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | No new influences, geometry/UV/bind/FFD edits; linked/shared/constrained cases protected |
| [T-809](FEATURE_INVENTORY.md#t-809) — Multimodal provider transport and bounded context | COMPLETE | MIXED | PUBLIC_ALPHA / PUBLIC_ALPHA | Wire support is not concrete model compatibility; context exhaustion remains possible |

## Already present — do not recreate

Global diagnostics/navigation (T-818), reviewed region transfer (T-819), linked tangents (T-820), exact single-key FFD controls (T-821), single PSD path mapping (T-822), guarded weighted retrace (T-823), themed onion/opacity (T-824), Graph marquee (T-825), normalized view-only overlay (T-826), native capture/highlight optimizations (T-827/T-829), FPS history (T-828), numeric whole-clip retime (T-830), and native PathPosition (T-831) are local implementations. They are not newly public.

## Remaining / next steps

[Priorities](roadmap-priorities.md) separates release gates, artist features, ergonomic work and deferred platforms. [Manual map](cumulative-manual-qa-map.md) retains all 352 pending instances without requesting execution now. [Compatibility](format-release-compatibility.md) distinguishes backward read compatibility from old-reader forward refusal. No new P0 incident has been established.

B01 is addressed for T-819 region copy only; T-909 remains PARTIAL. B02 is locally closed only for transport FPS. B03 awaits physical reproduction. T-912 still has a literal opt-in snapping policy gap; linked modes/Normalize/marquee are already present. T-404 has live stale-FFD diagnostics and protected editing, not automatic load-time track invalidation.

The only whole historical ABSENT task is T-904 selection sets. T-705 localization and T-803 browser editor remain DEFERRED. T-303 built-in importer, T-508 performance alias and T-603 hardcoded exporter are SUPERSEDED. Authentic v0 qualification and broad performance/resource budgets remain uncertain subordinate criteria; no whole historical ID is UNKNOWN.

[CSV with all evidence columns](task-status-matrix.csv). This is a documentation-only local checkpoint; no product/test/dependency/format/runtime change, no binary, no GUI/inference/remote operation.

## T-835 — Persistent animation ordering (local candidate)

[Animation manager guide](animation-manager.md): right-click a clip row, choose Move Up or Move Down. The native author list is saved through the existing `.ankh` animation array. IDs and complete clip payloads remain unchanged; one click is one Undo. Manager/picker share the order. Boundaries and stale popup contexts refuse before dispatch. No schema, sampler, evaluator, runtime, shortcut or release change.

T-208 stays **PARTIAL / PENDING_MANUAL / LOCAL_CANDIDATE**: auditing its original criteria found the existing seconds display and Setup-preserving manager selection. These are separate from the delivered ordering scope and were not changed. Counts remain **62 COMPLETE /33 PARTIAL /1 ABSENT /3 SUPERSEDED /2 DEFERRED =101**. The byte-identical T-832 CSV is a historical snapshot; its old reorder-gap wording is superseded by this explicit T-835 addendum. T-209/T-404/T-502 remain partial. PathPosition minimum-reader release policy remains pending.
