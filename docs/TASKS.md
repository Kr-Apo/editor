# Ankhimate — operational tasks and evidence

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

## 1. Current state

The editor can import layered artwork, rig, animate, save/recover and export
native v2/atlas or CPU images/video. Public PixiJS/Phaser support a documented
common subset, **not** every Rust constraint or attachment combination. AI is
optional and bounded by native approval. Windows x64 portable alpha is unsigned;
MCP/community tools are source/optional packages rather than ZIP executables.

Accepted T-816 baseline: **1596 Rust tests passed, 4 opt-in ignored**, fmt/Clippy/
workspace build/core WASM/docs green. T-817 documentation-only validation is
recorded separately; no new binary or model/GPU test is produced. Read the
[full inventory](FEATURE_INVENTORY.md) before proposing a new implementation.

## 2. Three independent dimensions

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

## 3. Complete historical task matrix

Every identifier in the exact T-816 TASKS is represented, including renumbered
aliases and subtask IDs. T-804/T-812/T-817 are extra context rows. Each link opens
the exact scope, native entry, existing code/tests, limits and recommended action.
The matrix is an operational summary; the archive retains all original criteria.

### Foundations

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-001](FEATURE_INVENTORY.md#t-001) — License + repo hygiene | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not a blanket audit of all transitive dependency licenses |
| [T-002](FEATURE_INVENTORY.md#t-002) — CI skeleton | PARTIAL | NOT_TESTED | SOURCE_ONLY | Actions disabled; no fresh remote green run; actual Clippy job lacks historical -D warnings |
| [T-003](FEATURE_INVENTORY.md#t-003) — ADR seed | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Some historical format statements are superseded by format-spec; not every old design sketch is current |
| [T-101](FEATURE_INVENTORY.md#t-101) — Stable entity IDs | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Names cross disk boundaries; unresolved foreign references still need diagnostics |
| [T-102](FEATURE_INVENTORY.md#t-102) — Affine2 transform math | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Singular transforms remain noninvertible; no universal arbitrary-float geometry guarantee |
| [T-103](FEATURE_INVENTORY.md#t-103) — Pose extraction + evaluate skeleton | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Stateful physics requires its separate accumulator; direct seek is not a simulation |
| [T-104](FEATURE_INVENTORY.md#t-104) — Constraint system + correct IK blending | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Runtime common alpha excludes constraints; geometric singularities are not repaired |
| [T-105](FEATURE_INVENTORY.md#t-105) — Skin layer | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not arbitrary game-runtime skin composition or linked deformation qualification |
| [T-106](FEATURE_INVENTORY.md#t-106) — Animation model + sampling | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Extreme finite handles can overflow sampler; no universal f32-range guarantee |
| [T-107](FEATURE_INVENTORY.md#t-107) — Editor Document/Session split + command undo | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Undo history is not persisted; not every historical UI metadata assignment uses a command (B02) |
| [T-108](FEATURE_INVENTORY.md#t-108) — `formats` crate + .ankh v1 save/load | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Prototype ZIP unsupported; no authentic v0 migration; complete scope is current v1 only |
| [T-707](FEATURE_INVENTORY.md#t-707) — Crash recovery, update check, feedback | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No panic-time live-document dump, recovery browser or complete update/feedback package |
| [T-813](FEATURE_INVENTORY.md#t-813) — Format Compatibility & Project Reliability | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE | No arbitrary structural repair, hostile-resource budget or power-loss certificate; GUI pending |

### Rigging

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-206](FEATURE_INVENTORY.md#t-206) — Hierarchy panel UX | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Original hierarchy/reparent/lock scope exists; separate bone-name context-menu usability issue B03… |
| [T-307](FEATURE_INVENTORY.md#t-307) — Attachment inspector — per-attachment properties | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Mesh/sequence image replacement is a separate unimplemented extension; not an absent region inspector. |
| [T-501](FEATURE_INVENTORY.md#t-501) — Transform constraint | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not part of common Pixi/Phaser alpha contract |
| [T-502](FEATURE_INVENTORY.md#t-502) — Path attachment + path constraint | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Static polyline paths/constraints exist; Bezier path handles and PathPosition/PathSpacing authoring… |
| [T-503](FEATURE_INVENTORY.md#t-503) — Physics constraint | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Rust fixed-step contract is complete in CAP-08; original export/playback parity not fulfilled: native… |
| [T-504](FEATURE_INVENTORY.md#t-504) — IK completeness | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA | No unique-solution guarantee or new IK key dots; runtime IK excluded |
| [T-505](FEATURE_INVENTORY.md#t-505) — Slot & bone presentation depth | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Not every presentation variant or animated dark tint; viewport acceptance incomplete |
| [T-506](FEATURE_INVENTORY.md#t-506) — Event timeline | COMPLETE | AUTOMATED | PUBLIC_ALPHA | t=0 ignored by current runtime policy; audio metadata is not audio playback |
| [T-507](FEATURE_INVENTORY.md#t-507) — Skin manager + multi-skin composition | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Game adapters use one compatible skin; arbitrary multi-skin/mesh-image workflows unqualified |
| [T-508](FEATURE_INVENTORY.md#t-508) — Former performance task renumbered into T-706 | SUPERSEDED | AUTOMATED | SOURCE_ONLY | Historical alias only, not another performance implementation |
| [T-816](FEATURE_INVENTORY.md#t-816) — Rigging Discoverability & First-Run Experience | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE | Artist discovery exercise pending; no new solver/runtime capability |
| [T-904](FEATURE_INVENTORY.md#t-904) — Saved selection sets (not hierarchy folders) | ABSENT | NOT_TESTED | NOT_DISTRIBUTED | Stale SelectionSet comments are not an implementation; new optional schema needed if later authorized |
| [T-907](FEATURE_INVENTORY.md#t-907) — Bone length edit carries its children | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Remembered carry preference/full canvas-length interaction not identified |
| [T-908](FEATURE_INVENTORY.md#t-908) — Make the existing advantages discoverable | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | New user's no-source exercise pending; not an absent IK solver/weight brush |
| [T-909](FEATURE_INVENTORY.md#t-909) — Rig transfer between skeletons | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Opening/replacing a document resets session clipboard; BoneClip carries attachment metadata and bone… |
| [T-911](FEATURE_INVENTORY.md#t-911) — Physics stability harness | PARTIAL | AUTOMATED | PUBLIC_ALPHA | No complete debug overlay or broad unrelated-edit interaction qualification |

### Animation

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-201](FEATURE_INVENTORY.md#t-201) — Dopesheet panel | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA | Complex track addresses belong to the separately partial T-209/FFD workflow; local T-814 selection… |
| [T-202](FEATURE_INVENTORY.md#t-202) — Playback + auto-key | COMPLETE | AUTOMATED | PUBLIC_ALPHA | FPS direct-mutation finding B02 is not fixed; pending preview is distinct from stored key |
| [T-203](FEATURE_INVENTORY.md#t-203) — Bezier interpolation UI | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA | Complete scope excludes linked handles, normalize overlay and graph marquee (T-704) |
| [T-204](FEATURE_INVENTORY.md#t-204) — Draw-order animation UI | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Generic dopesheet order-key clipboard remains excluded |
| [T-205](FEATURE_INVENTORY.md#t-205) — Slot timelines UI | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Static dark tint is separate; no animated-dark channel is invented |
| [T-207](FEATURE_INVENTORY.md#t-207) — Setup mode / Animate mode | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Every future edit surface still needs mode semantics; legacy metadata writes audited separately |
| [T-208](FEATURE_INVENTORY.md#t-208) — Animation manager | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Original animation-reorder UI not clearly qualified; do not recreate manager |
| [T-209](FEATURE_INVENTORY.md#t-209) — Clipboard: copy / paste / duplicate | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Complex rows/events excluded; no empty-row destination selection or automatic retargeting |
| [T-209.2](FEATURE_INVENTORY.md#t-209-2) — Validated normal/mirrored multibone pose paste in… | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Auto-key-off retains preview policy; not a retargeter |
| [T-209.3](FEATURE_INVENTORY.md#t-209-3) — Explicit bone-row mirrored offsets; signs | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Offsets, not asymmetric absolute setup poses; no slot/FFD/event mirror |
| [T-210](FEATURE_INVENTORY.md#t-210) — Keyed-value affordances in the inspector | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Does not add IK key dots or every future property; no whole-app accessibility certificate |
| [T-211](FEATURE_INVENTORY.md#t-211) — Pose tools | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Arbitrary numeric retime dialog and broader original UI acceptance remain |
| [T-703](FEATURE_INVENTORY.md#t-703) — Onion skinning | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Tint/alpha hardcoded; key-mode/boundary coverage, theme controls, performance and desktop qualification… |
| [T-704](FEATURE_INVENTORY.md#t-704) — Curve editor refinement | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No aligned/mirrored handle linking, normalized overlay or graph marquee |
| [T-814](FEATURE_INVENTORY.md#t-814) — Shared key selection | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE | Broader T-704 stays partial; combined graph edits retain existing granularity; GUI pending |
| [T-905](FEATURE_INVENTORY.md#t-905) — Non-destructive timeline offset | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Not arbitrary per-track/per-selection offsets for slots/constraints/deforms |
| [T-906](FEATURE_INVENTORY.md#t-906) — Timeline markers | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not runtime event callbacks; no fresh desktop certification |
| [T-912](FEATURE_INVENTORY.md#t-912) — Graph editor ergonomics review | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No tied handles/normalize/marquee; extreme sampling overflow shown as gaps; GUI pending |

### Meshes

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-401](FEATURE_INVENTORY.md#t-401) — Mesh attachment editing UI | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No universal snapping or arbitrary linked-source/topology edit safety |
| [T-402](FEATURE_INVENTORY.md#t-402) — Auto mesh tracing | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Tracing/refinement exists, but topology replacement clears weights rather than promised remapping;… |
| [T-403](FEATURE_INVENTORY.md#t-403) — Weight painting + bind compensation | COMPLETE | PENDING_MANUAL | PUBLIC_ALPHA | Complete native scoped brush; not autorigging or every-project visual fidelity; T-816 GUI pending |
| [T-404](FEATURE_INVENTORY.md#t-404) — Deform timelines | PARTIAL | AUTOMATED | PUBLIC_ALPHA | FFD dopesheet/clipboard incomplete; topology changes require care |
| [T-405](FEATURE_INVENTORY.md#t-405) — Clipping attachments | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Common adapters support bounded sequential masks, not arbitrary nested intersections |
| [T-815](FEATURE_INVENTORY.md#t-815) — Viewport Precision & Numeric Geometry Editing | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE | Not every handle/expression/snap or linked-source edit; GUI pending |
| [T-902](FEATURE_INVENTORY.md#t-902) — Numeric entry for every vertex and handle | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | Not every draggable point, expression parser or snap-to-neighbour |

### Assets / PSD

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-301](FEATURE_INVENTORY.md#t-301) — Asset database + image drop-import | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Project portability requires sidecar; broad texture-eviction/performance remains T-706 |
| [T-302](FEATURE_INVENTORY.md#t-302) — PSD import | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Renames/reparents become added+missing; automatic add/delete and changed shared/mesh/linked assets… |
| [T-303](FEATURE_INVENTORY.md#t-303) — foreign armature importer | SUPERSEDED | AUTOMATED | SOURCE_ONLY | Do not restore a second built-in importer; package licensing/availability remain external |
| [T-304](FEATURE_INVENTORY.md#t-304) — Startup window | COMPLETE | AUTOMATED | PUBLIC_ALPHA | No first-user study of local T-816; unknown structural input limits remain |
| [T-305](FEATURE_INVENTORY.md#t-305) — Atlas / spritesheet import modal | COMPLETE | NOT_TESTED | PUBLIC_ALPHA | No guarantee for every vendor atlas variant |
| [T-306](FEATURE_INVENTORY.md#t-306) — Asset relink + external-source workflow | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Moving only project does not move sidecar; missing assets need explicit resolution |
| [T-1003](FEATURE_INVENTORY.md#t-1003) — Move foreign formats to community JavaScript plugins | COMPLETE | AUTOMATED | SOURCE_ONLY | Missing package differs from broken builtin; no generic Godot/foreign fidelity promise |

### Formats / exports

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-601](FEATURE_INVENTORY.md#t-601) — Offscreen render + image / spritesheet / sequence… | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Default skin/Normal/no physics; original background/performance/viewport criteria not all closed |
| [T-601.1](FEATURE_INVENTORY.md#t-601-1) — PNG retained | COMPLETE | AUTOMATED | PUBLIC_ALPHA | JPEG lossy tolerance/no alpha; no runtime format change |
| [T-601.2](FEATURE_INVENTORY.md#t-601-2) — Opt-in common camera/global bounds | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Skin/blend/physics exclusions persist; not arbitrary renderer fidelity |
| [T-601.3](FEATURE_INVENTORY.md#t-601-3) — Selected-clip batches | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not crash-atomic multi-file commit or unrestricted memory/performance guarantee |
| [T-602](FEATURE_INVENTORY.md#t-602) — Video export via ffmpeg | COMPLETE | MIXED | PUBLIC_ALPHA | No audio/GIF alpha/physics or every-player guarantee; external encoder required |
| [T-602.1](FEATURE_INVENTORY.md#t-602-1) — Raw RGB CPU video to H.264 | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Opaque background, no bundled encoder; hard-link filesystem requirement |
| [T-602.2](FEATURE_INVENTORY.md#t-602-2) — Two-pass GIF palette | COMPLETE | AUTOMATED | PUBLIC_ALPHA | No GIF alpha or arbitrary exact FPS promise |
| [T-602.3](FEATURE_INVENTORY.md#t-602-3) — CRF/bitrate | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Native opener injected in tests; RGB requires player consent; not hardware encoding |
| [T-603](FEATURE_INVENTORY.md#t-603) — Original hardcoded-exporter plan replaced by… | SUPERSEDED | AUTOMATED | SOURCE_ONLY | Do not create another hardcoded exporter; aggregate library still partial |
| [T-603a](FEATURE_INVENTORY.md#t-603a) — Atlas bake | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Common runtime excludes rotated atlases even if custom presets differ |
| [T-603b](FEATURE_INVENTORY.md#t-603b) — Template engine + export runner | COMPLETE | AUTOMATED | PUBLIC_ALPHA | No filesystem-wide atomicity or guaranteed power-loss durability |
| [T-603c](FEATURE_INVENTORY.md#t-603c) — Native runtime format, authored as a template | COMPLETE | AUTOMATED | PUBLIC_ALPHA | No new .ankh.runtime binary; serializable fields do not imply adapter support |
| [T-603d](FEATURE_INVENTORY.md#t-603d) — Export panel | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not a continuously updated global diagnostics dock |
| [T-603e](FEATURE_INVENTORY.md#t-603e) — Preset library | PARTIAL | AUTOMATED | PUBLIC_ALPHA | No qualified Godot preset; foreign-format plugins remain optional |
| [T-605](FEATURE_INVENTORY.md#t-605) — Format spec finalization + migration policy | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No authentic v0 migration/prototype converter or comprehensive hostile-file resource budget |

### PixiJS / Phaser

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-604](FEATURE_INVENTORY.md#t-604) — Runtime integration | PARTIAL | AUTOMATED | PUBLIC_ALPHA | In-workspace Rust player plan superseded; no universal runtime parity |
| [T-810](FEATURE_INVENTORY.md#t-810) — Phaser common-profile corrections and public… | COMPLETE | AUTOMATED | PUBLIC_ALPHA | No new all-feature parity; detailed external runtime rows below |

### IA

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-805](FEATURE_INVENTORY.md#t-805) — Native conversational assistant | COMPLETE | MIXED | PUBLIC_ALPHA | Not autonomous autorigging; model prose can be wrong; no generic cloud/model certificate |
| [T-805.1](FEATURE_INVENTORY.md#t-805-1) — Fixed composer | COMPLETE | MIXED | PUBLIC_ALPHA | Historical Qwen real run is binary/model-specific; no new T-816 inference qualification |
| [T-806](FEATURE_INVENTORY.md#t-806) — Viewport vision and first native rigging tools | PARTIAL | MIXED | PUBLIC_ALPHA | Targeted historical maintainer report only; no general visual placement or every-model certificate |
| [T-807](FEATURE_INVENTORY.md#t-807) — Controlled native character assembly | COMPLETE | MIXED | PUBLIC_ALPHA | Weighted meshes are not regions; unrepresentable/constrained cases refused |
| [T-808](FEATURE_INVENTORY.md#t-808) — Guarded mesh weight assistance | COMPLETE | MIXED | PUBLIC_ALPHA | No new influences, geometry/UV/bind/FFD edits; linked/shared/constrained cases protected |
| [T-809](FEATURE_INVENTORY.md#t-809) — Multimodal provider transport and bounded context | COMPLETE | MIXED | PUBLIC_ALPHA | Wire support is not concrete model compatibility; context exhaustion remains possible |

### UX / tools

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-701](FEATURE_INVENTORY.md#t-701) — Settings, keymap, autosave | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Original exhaustive settings list/restart acceptance incomplete; unsaved-doc autosave policy retained |
| [T-702](FEATURE_INVENTORY.md#t-702) — Diagnostics / warnings | PARTIAL | PENDING_MANUAL | LOCAL_CANDIDATE | T-818 adds read-only global rules, status badge/list, revision/metadata debounce and safe selection; camera framing and GUI/broad performance qualification remain open. Prior export scope remains PUBLIC_ALPHA |
| [T-705](FEATURE_INVENTORY.md#t-705) — Localization | DEFERRED | NOT_TESTED | NOT_DISTRIBUTED | i18n implementation absent and deliberately deferred; no locale certificate |
| [T-706](FEATURE_INVENTORY.md#t-706) — Performance pass | PARTIAL | AUTOMATED | SOURCE_ONLY | No dedicated core benchmark target/xtask bench/CI trend or general 500-bone/iGPU 60-FPS qualification… |
| [T-708](FEATURE_INVENTORY.md#t-708) — Viewport interaction polish | PARTIAL | PENDING_MANUAL | PUBLIC_ALPHA | No universal snapping/expression-on-drag/pivot editing for every attachment; framing acceptance… |
| [T-709](FEATURE_INVENTORY.md#t-709) — Accessibility & window UX | PARTIAL | AUTOMATED | PUBLIC_ALPHA | No whole workflow keyboard/contrast/colorblind audit or exhaustive icon-tooltip certificate |
| [T-901](FEATURE_INVENTORY.md#t-901) — Bulk rename with sequential numbering | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Not slot/attachment/constraint rename, regex or all path-safety variants |
| [T-903](FEATURE_INVENTORY.md#t-903) — Isolation mode | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Optional hierarchy/dopesheet filters/configurable opacity not fully qualified |
| [T-910](FEATURE_INVENTORY.md#t-910) — Multi-window / multi-monitor | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Second viewport absent; per-window positions/multimonitor acceptance not broadly established |
| [T-913](FEATURE_INVENTORY.md#t-913) — Name the thing under the cursor | COMPLETE | NOT_TESTED | PUBLIC_ALPHA | Overlap/DPI/delay/accessibility not exhaustively qualified No dedicated hover-interaction test found;… |

### Distribution

| ID / scope | Implementation | Validation | Availability | Remainder / priority |
|---|---|---|---|---|
| [T-801](FEATURE_INVENTORY.md#t-801) — Samples + documentation | PARTIAL | AUTOMATED | PUBLIC_ALPHA | Not all 3 CC0 sample/PSD/GIF criteria; new user's exercise pending |
| [T-802](FEATURE_INVENTORY.md#t-802) — Release packaging | PARTIAL | AUTOMATED | PUBLIC_ALPHA | No signed installer, Linux/macOS artifacts or broad hardware certification |
| [T-803](FEATURE_INVENTORY.md#t-803) — Web build | DEFERRED | NOT_TESTED | NOT_DISTRIBUTED | No browser authoring frontend/distribution; WASM core is not a web editor |
| [T-810.5](FEATURE_INVENTORY.md#t-810-5) — Simple Windows MP4 action reusing supervised CPU… | COMPLETE | MIXED | PUBLIC_ALPHA | External FFmpeg, opaque output/new-file only; no bundled encoder |
| [T-811](FEATURE_INVENTORY.md#t-811) — Next Alpha release candidate | COMPLETE | MIXED | PUBLIC_ALPHA | Historical candidate-pending prose superseded only for the exact published artifacts |
| [T-1001](FEATURE_INVENTORY.md#t-1001) — Plugin host and declarative panels | COMPLETE | AUTOMATED | PUBLIC_ALPHA | Not a general external package/security endorsement |
| [T-1002](FEATURE_INVENTORY.md#t-1002) — MCP stdio server | COMPLETE | AUTOMATED | SOURCE_ONLY | Not autonomous AI access or browser authoring; permissions remain caller-specific |
| [T-804](FEATURE_INVENTORY.md#t-804) — Unofficial community fork provenance | COMPLETE | MIXED | PUBLIC_ALPHA | No upstream endorsement or npm publication implied |
| [T-812](FEATURE_INVENTORY.md#t-812) — Exact T-811 artifacts published as v0.2.0-alpha.1… | COMPLETE | MIXED | PUBLIC_ALPHA | Local T-813–T-816 not included; unsigned Windows, no new platform certificate |
| [T-817](FEATURE_INVENTORY.md#t-817) — Operational inventory | COMPLETE | AUTOMATED | LOCAL_CANDIDATE | Documentary audit only; no product or graphical acceptance added |
| [T-818](FEATURE_INVENTORY.md#t-818) — Bounded global diagnostics | COMPLETE | PENDING_MANUAL | LOCAL_CANDIDATE | Read-only rules/cache/badge/list/navigation implemented; larger T-702 framing and latency criteria remain open |

## 4. Completed bounded scopes — retain, do not redevelop

- Native IDs/affine/evaluation/Document-Session/history and current author v1 profiles.
- Bone/slot dopesheet, auto-key/setup return, slot timelines, per-property key dots,
  scalar key blocks, atomic multibone pose paste and explicit mirrored bone offsets.
- Existing region authoring, assets/relink, native mesh/weight brush, 1/2/3+ IK core.
- Native template export, safe staged publisher/preflight, image formats/framing/
  trim/padding/batch, MP4/GIF and simple Windows MP4 entry.
- Bounded native AI tools/approval/transactions and six transport profiles.
- Local T-813 project safeguards, T-814 tangent drafts, T-815 geometry drafts and
  T-816 discoverability: implemented/tested; manual qualification is separate.

These are deliberately bounded completions. The inventory's contradictory checks
exclude unqualified runtimes, platforms, artist quality and extended task promises.

## 5. Partial work — only the remainder is active

| Family | Already delivered | Remaining work / dependencies / risk |
|---|---|---|
| T-209 / T-404 | Owned scalar/slot clipboard, poses and bone-offset mirror | Editable complex/FFD addresses and curves; native index/topology/lock validation before UI; high reference risk |
| T-302 | Nested import/preview and exact-path region refresh | Explicit path-change resolution / simple new-layer policy, not guessed image matching; preserve rig and animations |
| T-402 | Alpha tracing/refinement | Weight/topology/FFD implications and original performance budget; do not retrace the subsystem |
| T-502 / T-503 | Static path constraints and fixed-step Rust physics | Path animation tracks; export/runtime physics compatibility; deliberately excluded from alpha, separate future contracts |
| T-505 / T-507 | Presentation/skins/default fallback | Advanced blend fidelity and composition/image-assignment boundaries; never infer adapter parity |
| T-601 | CPU images/pages and deterministic multi-clip exports | General backgrounds/physics stepping/performance/viewport acceptance; reuse current renderer |
| T-605 / T-701 / T-707 | Profiles/specification, safe refusal, keymap/autosave/recovery | Authentic legacy migration, hostile resource limits, original settings/panic/update/feedback criteria; maintain writer guarantees |
| T-702 | Export integrity/fidelity preflight plus local T-818 diagnostics | Preserve publication checks. Global selection is implemented; universal framing, bounded worst-case latency and GUI qualification remain open |
| T-703 | Ghost passes, selected-only, frame/key steps | Theme/opacity controls, more boundary coverage and visual/performance qualification |
| T-704 / T-912 | Graph selection/framing/overshoot and exact numeric tangents | Handle linking/normalize/marquee; T-814 numeric fields are already done |
| T-708 / T-902 | Gizmos, region pivots, exact mesh/UV/path/polygon fields | Universal point coverage, expressions and neighbour snapping; T-815 fields are already done |
| T-901 / T-903 / T-905 / T-907 | Bone rename, session isolation, per-bone offsets, carry-children | Other entity rename/regex, optional filters, per-track offset scope, remembered carry preference |
| T-909 | Subtree slots/constraints/bone animation copy | Cross-document clipboard/asset ownership, conflict preview and explicit transfer; no AssetDb bytes in BoneClip |
| T-910 / T-911 | Detached noncanvas panes; deterministic physics tests | Second render viewport/debug overlay/broader physics interaction acceptance, not a missing window/solver system |
| T-801 / T-802 / T-908 | Published Jade, portable Windows, existing Tentacle and first-rig docs | Original samples/GIF/signing/platform breadth; new-user exercise in manual queue |

## 6. Absent, replaced or deferred

- **ABSENT:** saved selection sets; global diagnostics dock; tied tangent modes/
  graph marquee; second viewport; panic-time document dump; neighbour snapping;
  onion theme/opacity controls. See scoped CAP rows rather than reopening aggregates.
- **DEFERRED:** localization and browser editor. WASM core is not browser authoring.
- **SUPERSEDED:** T-303 built-in importer → optional community packages;
  T-508 → T-706; T-603 hardcoded-export plan → strict user-authored templates.
- **UNKNOWN:** authenticated v0 migration qualification and general production
  performance budgets. Absence of evidence is not a measured incompatibility.

## 7. Prioritized development backlog (proposals only)

See [ROADMAP_NEXT.md](ROADMAP_NEXT.md): review local T-818 diagnostics; next proposed T-819 bounded
cross-document transfer, T-820 tangent linking, T-821 FFD-key authoring, T-822
explicit PSD path-change resolution. They are five **new proposed IDs**, not
implemented checkpoints or publication authorization. T-817 Diagnostics was only
an old proposal; this documentary inventory is the sole T-817 checkpoint.

P0 = demonstrated integrity/security incident; P1 = central workflow gap;
P2 = production usability; P3 = extension. No new P0 incident is established here.
FPS history bypass B02 and bone-name menu B03 stay recorded for focused triage;
no spontaneous fix is authorized by this audit.

## 8. Pending qualification — separate from development

[MANUAL_QA_PENDING.md](MANUAL_QA_PENDING.md) references the existing T-813–T-816
checklists and cumulative T-816 executable. The maintainer will choose when to
run them; no Windows action is requested now. Missing GUI evidence is not an
absent capability or a reason to recreate it. Historical accepted T-811/T-812
results cover their exact artifacts only.

## 9. Compatibility notes

Author schema/envelope v1, native runtime format v2 and package/app versions are
independent. `.ankh.runtime` binary/postcard and the old macroquad player were
planning targets, not shipped artifacts. Current native output is JSON + atlas.
Core TS reads v1/v2; common qualified adapters use explicit v2. PixiJS 8.17.0 uses
WebGL2; Phaser 3.90.0 used actual WebGL1 in accepted T-811 tests. Native constraints,
physics, linked meshes, rotated atlases and advanced blending remain outside the
common profile. Images/video retain default skin/Normal/no physics restrictions.
Spritesheet JSON is Ankhimate metadata, not native Phaser JSON. No format change.

## 10. History and operating rules

[Exact TASKS before T-817](TASKS-before-T817.md) retains all criteria/IDs/PLAN links.
Architecture sketches, old `docs/alpha-scope.md` and `docs/alpha/` remain historical;
use this inventory, the [format spec](format-spec.md), and protected release records
for actual behavior/availability. Existing licenses/third-party credit remain intact.

Before any future mission: read the row's code/tests and explicit remainder, verify
its exact checkpoint, reuse native commands/mode gates, preserve originals and
immutable releases. Do not use old checkmarks or internal version strings alone.
No current push, merge, tag, release, Actions, package or product change is authorized.

## T-818 checkpoint (local, not released)

**COMPLETE bounded implementation / PENDING_MANUAL / LOCAL_CANDIDATE**.
Read-only document inspection reuses export integrity checks without consumer
fidelity warnings. Native supplements cover weight sums/binds, fallback, IK,
track addressing and draw-order duplicates. Results are revision/document-scoped;
rapid edits coalesce for 200 ms and stale results cannot navigate. Native
commands, AI, Undo/Redo and project replacement retain their original paths.
Metadata name/FPS are observed cheaply; B02's undoable metadata repair is excluded.
Badge/list and navigation use egui/session selection. Camera stays unchanged.
Tests: `document/src/diagnostics/tests.rs`, `editor/src/diagnostics/tests.rs`,
actual dirty-state and preflight integration regressions. No new Save blockers.
See the [manual register](MANUAL_QA_PENDING.md); T-819 is proposed, not started.
