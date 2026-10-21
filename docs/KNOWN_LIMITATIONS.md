# Ankhimate — Current known limitations after T-831

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

## Current product gaps / evidence boundaries

| Historical task | Current implementation | Remaining product or qualification condition |
|---|---|---|
| T-002 | Linux/Windows test/fmt/Clippy and core WASM workflow definitions | Actions disabled; no fresh remote green run; actual Clippy job lacks historical -D warnings |
| T-208 | Clip create/rename/duplicate/delete/duration/loop manager retained | T-835 adds persistent adjacent Move Up/Down with native Undo/Redo. Original manager duration-in-frames display and switching to Animate on manager-row selection remain unimplemented; no claim of full T-208 completion. |
| T-209 | Existing owned key blocks/pose/mirror plus faster capture; separate reviewed region transfer | T-834 supports owned same-mesh FFD key blocks on an explicit existing row. Cross-project/cross-mesh, linked/shared/overridden sources, empty-row creation, constraints, draw-order and event blocks remain excluded. Scalar replacement semantics remain unchanged. |
| T-211 | Bake/reset/clear and fixed retime retained; checked numeric whole-clip retime | T-833 adds explicit selected-bone transform-key offset, separate from whole-clip Retime. K keys pending poses; no dedicated Shift+K key-all-changed binding identified. Arbitrary KeyRefs/constraint/slot offsets and selected time scaling remain out of scope. |
| T-302 | Initial PSD import plus exact-path refresh and one explicit path-change mapping | Bulk/multiple missing path mapping, complex/shared/mesh/FFD/sequence refresh, automatic added/deleted-layer reconciliation and Photoshop effects/masks remain outside qualified workflow. |
| T-401 | Mesh vertices/triangles, constrained retriangulation, UV and exact local numeric drafts | No universal snapping or arbitrary linked-source/topology edit safety |
| T-402 | Original tracer retained; safe nearest-vertex weight-row remap and bind preservation | FFD-dependent, linked/shared/overridden or pinned retrace is refused; no topology/FFD migration. General <100 ms release/artist preservation budget remains unqualified beyond one ring fixture. |
| T-404 | Stable FFD drag plus typed owned-mesh Dopesheet selection/time/delete/interpolation | Stale topology is reported by live diagnostics and editing refuses it; the author loader still directly constructs Deform rather than invalidating its track at load. No general topology/FFD remap or ambiguous/shared ownership edit. Vector clipboard belongs to T-209 and alone is not the reason for this status. |
| T-502 | Static polylines/native constraint plus native PathPosition keys/evaluation/author files | Animated PathSpacing and geometric Bezier handles remain absent; data/runtime exports refuse PathPosition. Graph/group/clipboard Path keys are deferred. |
| T-503 | Rust caller-owned fixed-step physics state, reset and deterministic advance | Rust fixed-step contract is complete in CAP-08; original export/playback parity not fulfilled: native TS physics activation incompatible and rendered exports explicitly refuse it. |
| T-505 | Bone/slot colors, visibility, locks and native static dark tint | Not every presentation variant or animated dark tint; viewport acceptance incomplete |
| T-507 | Named skins, default fallback and editor composition controls | Game adapters use one compatible skin; arbitrary multi-skin/mesh-image workflows unqualified |
| T-601 | CPU deterministic PNG/JPEG/WebP sequences, spritesheets, framing/trim/padding/batch | Default skin/Normal/no physics; original background/performance/viewport criteria not all closed |
| T-603e | Native/generic/Phaser-atlas presets and custom templates | No qualified Godot preset; foreign-format plugins remain optional |
| T-604 | External TypeScript core and two packaged adapters with qualified common profile | In-workspace Rust player plan superseded; no universal runtime parity |
| T-605 | Existing v1 spec/gates/refusal plus documented additive PathPosition closed enum | Authentic v0 migration still unqualified; no universal hostile-resource budget. T-831 keeps author/envelope 1 but older readers reject the new variant; future release must communicate feature-level compatibility. |
| T-701 | JSON preferences, registered keymap and monotonic isolated idle autosave | Original exhaustive settings list/restart acceptance incomplete; unsaved-doc autosave policy retained |
| T-702 | Global shared integrity rules, debounced badge/list and exact safe offender selection | Automatic select-and-frame is absent: camera intentionally unchanged. Large image scans remain synchronous without a general 500-bone/asset completion budget. Export preflight remains independently authoritative. |
| T-705 | English UI literals; no localization catalog/macro/picker identified | i18n implementation absent and deliberately deferred; no locale certificate |
| T-706 | Bounded native capture/indexing and selected-view membership optimizations, measured headlessly | No general xtask bench/trend or integrated-GPU 500-bone/300-row 60 fps certificate. Large commands/model/payload work remains; debug medians are not budgets. |
| T-707 | Safe save, sidecar recovery, candidate checks and recovery offers | No panic-time live-document dump, recovery browser or complete update/feedback package |
| T-708 | Gizmos, region pivot editing, camera and precise geometry/UV entry | No universal snapping/expression-on-drag/pivot editing for every attachment; framing acceptance incomplete |
| T-709 | Control sizing, scaling, partial keyboard/AccessKit, close guard and docking | No whole workflow keyboard/contrast/colorblind audit or exhaustive icon-tooltip certificate |
| T-801 | Approved Jade alpha sample, source examples and first-rig guide | Not all 3 CC0 sample/PSD/GIF criteria; new user's exercise pending |
| T-802 | Published unsigned portable Windows ZIP, manifests and matching packages | No signed installer, Linux/macOS artifacts or broad hardware certification |
| T-803 | WASM core compiles; native desktop editor remains product | No browser authoring frontend/distribution; WASM core is not a web editor |
| T-901 | Bone numbering/substring replacement, preview and native batch rename | Not slot/attachment/constraint rename, regex or all path-safety variants |
| T-902 | Exact mesh/UV/path/polygon Set/Add and numeric normalized Bezier handles | Not every draggable point, expression parser or snap-to-neighbour |
| T-903 | Session-only descendant isolation and exit affordance | Optional hierarchy/dopesheet filters/configurable opacity not fully qualified |
| T-904 | Hierarchy folders exist, but no saved overlapping selection-set model/schema/panel found | Stale SelectionSet comments are not an implementation; new optional schema needed if later authorized |
| T-905 | Persisted per-bone static sampling offsets and header UI | Not arbitrary per-track/per-selection offsets for slots/constraints/deforms |
| T-907 | Length edit carries tip children; Alt preserves positions; one native Undo | Remembered carry preference/full canvas-length interaction not identified |
| T-909 | Reviewed native cross-project region/asset/bone-key copy through separate import workflow | Explicit Move, general meshes/FFD/path/clipping/physics/alternate-skin or advanced animation transfer remain unavailable/refused. Local transforms are preserved, not arbitrary world pose; this is not retargeting. |
| T-910 | Noncanvas panels detach over one AppState; detached set persisted | Second viewport absent; per-window positions/multimonitor acceptance not broadly established |
| T-911 | 10k-step soak, energy decay and 30/60/144 physics tests; velocity API | No complete debug overlay or broad unrelated-edit interaction qualification |
| T-912 | Auto-fit/readouts/overshoot/gap notices, numerics, shared selection and newer scalar Graph refinements | The literal historical opt-in snapping criterion is still not met: Raw frame snapping is the default, Alt temporarily frees it, and the badge is visible. This policy gap is distinct from pending Windows readability/usability. Linked/normalize/marquee are no longer absent. |

## Implemented scopes with pending manual acceptance

| Scope | Implemented | Pending / explicit limit |
|---|---|---|
| T-703 | Native artwork ghosts with themed past/future tint, opacity/falloff, 1–5 frame/key steps and selected-owner filter | No product gap in retained T-703 scope. Manual visual contrast/scrubbing acceptance pending; no stateful physics ghosts or preference persistence promised. |
| T-704 | Existing scalar Graph plus numeric tangents, single-junction linking, marquee and Normalize View Only | No remaining product bullet in retained scalar scope. Windows ergonomics pending. Normalized editing, multi-junction linking, FFD/Path Graph and middle-button pan are separate extensions, not retroactively added requirements. |
| T-806 | Bounded viewport preview/one-send consent, coordinate declarations and typed bone proposal tools | No product gap in retained native tool/consent scope; real vision/placement/model qualification remains pending and is not generalized from old narrow feedback. |
| T-908 | Visible N-bone IK and brush guidance plus documented unchanged Tentacle exercise | No product gap in retained discovery surfaces/sample scope. New-user no-source Windows exercise remains pending; not a universal discoverability certificate. |

## Export and file exchange


This is a documentary finding at T-831, not a migration or new format change.
Author schema 1, binary envelope 1, native runtime 2 and app/package 0.2.0-alpha.1 are
independent identifiers. A shared author version alone does not imply identical
supported enum variants across binaries.

| Input/output | T-831 behavior | Historical reader/consumer | Evidence / risk |
|---|---|---|---|
| Old valid project without PathPosition | Existing fields, assets and animations retained; no manufactured track | Existing old profile | Historical migrations/reliability plus T-831 old-project witness |
| New save without PathPosition | Existing profile/field semantics retained | Compatible only within the old supported feature subset; not every arbitrary new project | No T-831 track added implicitly |
| New project with PathPosition | Named exact target/scalar keys/curves persist in all 3 profiles | Frozen T-830 rejects unknown `path_position` before returning an editable document | Forward opening is refused, not lossy downgrade |
| Readable/compact JSON/binary | Both conversion directions and strict references qualified | Existing compact field tags unchanged; new kind carried literally | Schema inventory/Serde case and actual-file old-reader probes |
| Native runtime v2, legacy, generic/custom/template export | Blocking preflight and independent plan guard | No new runtime field or runtime support | Explicit failure before atlas/writes |
| Script/plugin exporter | Refuses before executing script, returns original document | Current plugin context cannot encode the track | Native snapshot regression |
| CPU sequence and spritesheet | Existing Sequence/evaluator, common camera and exact reconstructed RGBA | Raster result does not require runtime Path support | Native pose/static reference at 0/.25/.5/.75; sheet pixels equal |
| Video/GIF | Same Sequence input and existing supervised encoders | Not newly visually qualified for PathPosition | Historical process/encoder gates only; do not claim a new video test |
| Whole-clip numeric retime | T-831 extends T-830 walkers to 14th timeline | Preserves old time/duration/collision semantics | New Path retime/lock/history regressions |

Descending compatibility (new reader, old valid files) and forward compatibility
(old reader, new feature) are different. Actual delivered T-831 files were rejected
unchanged by frozen T-830 in all 3 profiles. The older public reader also has the
closed enum without this tag; its inability follows from that source contract,
not a new public-binary GUI run. Never suggest removing the tag manually, baking
to a static path silently or saving an unsupported project through an older app.

Before a future alpha, decide and document the minimum feature-capable reader,
exchange warnings and candidate/release identifiers. A feature/version/migration
policy review is a release gate; **T-832 does not authorize or implement a bump,
migration, runtime change or repackaging**. Keep a separate complete copy for old
readers and retain originals. Author1 forward refusal is deliberate and prevents
silent loss, but can surprise collaborators using the public alpha.

Existing Save/staging/recovery guarantees and limitations are unchanged: ordinary
failure protection/rollback is tested, multi-file crash atomicity and power-loss
durability are not promised. No new corruption incident is demonstrated by this audit.

## Performance and diagnostics

T-827 removes repeated native key capture/resolution scans and T-829 removes idle selected-view membership scans in the measured bounded cases. A10k selection is no longer truthfully described as systematically quadratic end-to-end. Payload cloning, model/layout, active gestures and command application still cost time; no universal60 fps budget is certified. Historical default-concurrency video/process timeout sensitivity remains recorded; accepted final suites use4 threads without weaker assertions. T-818 diagnostics/navigation exists; automatic camera framing and general synchronous scan completion budgets do not.

No current claim of absent onion theme/opacity, Graph marquee, normalized view, linked tangents, FPS Undo, numeric retime or PathPosition is valid. Their scoped limitations are documented in [current tasks](TASKS.md). [Older notes](checkpoint-journal-before-T832.md) retain their original dates and claims only as provenance. [Manual qualification](cumulative-manual-qa-map.md) remains pending and [priorities](roadmap-priorities.md) starts no development.

## Selected-bone offset limits (T-833)

Only direct bone transformation tracks; no descendants, arbitrary KeyRefs, time scaling, slot/constraint/FFD/Path retiming or evaluation offsets. Explicit synchronous preparation/commit uses memory proportional to targeted key payload and scans timeline headers; no large-project Windows latency guarantee. Stale context requires Prepare again. A safe clamp or keys outside fixed clip duration require consent. Existing fixed shortcuts retain their historical weaker checks. See [contract](selected-bone-key-offset.md).

## T-834 FFD clipboard bounds

Only one unique owning skin/slot/attachment and its exact native mesh structure. Destination must already have a valid nonempty Deform track explicitly selected through its keys. No name/count-based matching, empty-row creation, cross-project or cross-mesh transfer, remapping, linked/shared/overridden or sequence mesh support. Missing/invalid derived binds refuse; no implicit repair. Full structural comparisons are conservative, including relevant Setup ancestry. Native identity is the owner entry key plus structural equality, not an unavailable heap-generation identifier. Existing T-821 time/delete controls remain single-key. Copy/Paste has no new keyboard binding; use dedicated context actions.

The new clipboard modal never re-captures vector data on repaint; group/single sheet leases compare borrowed native data. The existing single-key T-821 action menu may still capture its native edit snapshot while open. CPU measurements are not physical Windows responsiveness certification.

## T-835 animation order boundaries

Native author order is durable in the existing animation array; old editor versions may sort their UI or resave from SlotMap order. No new minimum-reader flag or schema is added. This candidate removes the manager alphabetical sort only; batch export keeps its existing occupied-slot/source-index folder convention, independently of author order. Single-clip rendered exports remain selected by exact clip name/ID. Runtime-v2 object member order is not a portable playback-selection API.

Create/Duplicate/Delete retain animation-collection snapshots to preserve generation IDs through interleaved history. This copies clip data, not skeletons/assets; the bounded History still retains up to its existing capacity. Large many-clip delete/create/duplicate memory use is not newly certified. Reorder itself stores only two ID vectors and never clones a clip. No drag/drop reorder or global shortcut is added. Windows usability is PENDING MANUAL REVIEW; historical GIF sharing-lock intermittence remains recorded without video changes.
