# Known limitations — local T-811 candidate

## Local T-828 update — 2026-10-21

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for project FPS history.
The transport now commits `SetProjectFps` through native history once per finished
gesture. Drag release, Enter or field blur apply; Escape, lost window focus,
lost pointer during drag, stale document/revision or hidden-control gap cancel.
Typing is a transient draft. Separate gestures remain separate Undo entries.
The existing editing range is 1–240 whole FPS; invalid text refuses rather than
silently clamping. Displaying legacy out-of-range values does not alter the file.
Undo restores the exact prior u32. No key, curve or clip is retimed.
Existing dirty snapshots, safe save, recovery and coalesced T-818 diagnostics
are preserved; metadata fallback remains. B02 is resolved only for transport FPS.
T-202 retains its historical COMPLETE status; B03 awaits desktop reproduction.
T-827 indexes and graph Raw/Normalize behavior remain unchanged. All cumulative
T-813–T-828 desktop controls are **PENDING MANUAL REVIEW**. No GUI, inference,
runtime change or public operation. See [FPS transport](animator/animation.md#fps-is-a-grid-not-a-retime-command).
Earlier notes below retain their historical scope.


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


- This is an unsigned Windows x64 alpha candidate. New archive launch on the
  maintainer's machine is pending. macOS/Linux distribution, mobile, WebGPU and
  broad GPU support are not qualified. No guaranteed frame rate or memory budget.
- Runtime parity covers the documented native v2 common subset. Constraints/IK,
  physics, linked meshes, rotated atlases and Add/Multiply/Screen are outside it.
  Phaser legacy paths may accept some extras; that is not a fidelity certificate.
  PixiJS requires WebGL2; Phaser used its real WebGL1 context in T-811.
- T-811: 125 framebuffer comparisons passed predeclared tolerances, 104 exact.
  The transformed clipping case has one contour pixel with alpha delta 64
  (0.0198% of foreground, below the 2% edge bound). No uniform pixel exceeded
  tolerance. This measured rasterization difference is preserved, not hidden.
  Five global fractional-alpha cases were exact; arbitrary alpha compositions
  and float precision on every Phaser container/camera are not certified.
- Interrupted crossfades can jump. Events at time zero are ignored at startup
  and on each loop, matching the accepted Rust policy. Skin switches require
  compatible attachments/binds/topology. Core v1 reading does not certify v1
  adapter pixel parity; the common profile is explicit native v2.
- Image/video exports retain default-skin, Normal-only and no-physics restrictions.
  MP4 is opaque, lossy H.264/yuv420p without audio; FFmpeg is separate. Existing
  output videos are refused, even after a native Save dialog confirmation.
  Publication needs hard links (local NTFS recommended). Crash durability and
  fully atomic multi-file exports are not promised; retained staging is reported.
- AI has no artistic correctness guarantee. Provider wire-format support does
  not prove every model supports tools or vision. Long sessions can exhaust
  context: use targeted reads and a shorter conversation. Small local profiles
  reduce budgets but do not certify an 8 GB GPU. No model is supplied/downloaded.
- Qwen/NInfer Jade rigging/assembly/weight tests are maintainer-reported manual
  evidence. Their modified project was not saved and is deliberately not shipped.
  The official sample is unchanged RC2 Jade Sentinel; it has no `qwen_left_arm_test`.
- Alternate-image skin authoring is region-focused; FFD timeline editing and
  complex PSD reconciliation retain their documented limits. Full localization,
  a web editor, a second viewport, autonomous autorigging, complete Spine parity,
  advanced tangent modes and advanced runtime constraints remain open.

No GUI, desktop automation, real inference or GPU benchmark was run for T-811.
Headless WebGL used an installed browser, actual contexts and local assets only;
final interactive and clean-machine checks remain separate.


## T-813 local project-reliability candidate

T-812's published binaries/archives remain immutable. T-813 is a separate local build
awaiting maintainer GUI qualification. Unknown author members/values and unsupported
versions/flags are refused; opaque presets remain preserved. Missing image bytes block
Save/Save As/autosave until relink or explicit asset removal. Non-reopenable typed
serialization is refused before any writes. Structural repair and hostile-file resource
budgets are not comprehensively validated. No authentic v0/prototype converter, panic-time
dump, permanent Undo history, recovery browser, automatic recovery purge or power-loss
certification was added. Read [project recovery](project-recovery.md). The accepted T-811
manual review was subsequently declared complete by the maintainer for public T-812;
that declaration does not qualify this new T-813 build.

## T-814 local animation-UX candidate

Numeric Bezier controls and shared session key selection are implemented and
covered by automated tests; **PENDING MANUAL REVIEW**, together with T-813.
Selection clears conservatively on external document edits/Undo/Redo. Colliding
key moves are refused, and a graph drag stops when sorting changes its key index.
Restart the gesture to continue. Combined time/value graph drags retain separate
commands. Multi-key numeric Apply copies all four displayed handles to every
selected Bezier key. No aligned/mirrored tangent mode, normalized overlay, graph
marquee or general history redesign is included. Extreme finite values may still
overflow the core sampler; the graph reports gaps without modifying the data.
See [graph editor](graph-editor.md). No keymap, project format, evaluator or runtime
change; published releases and the separate T-813 executable remain unchanged.

## T-815 local viewport-precision candidate

Mesh positions/UVs and path/clipping/bounding-box points use exact numeric drafts
and native undoable commands. **PENDING MANUAL REVIEW** with T-813/T-814. Editing
is Setup-only; numeric FFD, linked geometry and linked parents are excluded.
Only the currently displayed attachment in one slot is a geometry target.
UV numbers refer to source texture [0,1] top-left and reject out-of-range results.
Expressions, neighbour snap, universal rig-fit/Frame Selection, new pivot modes
and complete keyboard/DPI/accessibility acceptance remain deferred. Existing
camera/gizmos/hover and drag merging remain in place. No GPU/visual/performance
qualification is claimed for dense rigs. See [numeric geometry](geometry-editing.md).


## T-816 local rigging discovery checkpoint

GUI qualification for T-813/T-814/T-815/T-816 is **PENDING MANUAL REVIEW**.
The local candidate clarifies existing IK and weight controls and retains the
selected mesh during weight-influence picking; solver/brush/bind math is unchanged.
Evaluated span describes current joint geometry, not a maximum or natural-reach
prediction. Bend sign does not guarantee a unique solution; editing locks do not
freeze constraint evaluation. The existing Tentacle example is bones-only and
contains runtime-excluded IK: exporting constraints does not bake their poses.
Weight locks are session-only. There is no Normalize brush mode. No new graphic,
GPU, inference or artistic qualification is claimed. See
[First rig](first-rig-in-10-minutes.md) and [capabilities](rigging-capabilities.md).

## T-820 local Bezier-linking candidate

Single scalar junction only; Broken/Aligned/Mirrored are session editing modes,
not saved per-key constraints. Both neighbors must already be Bezier. The metric
is seconds/native scalar value units, not zoom-dependent screen lengths. Flat
vertical targets, impossible X/overflow, endpoints and shortest-arc rotation
joins with raw adjacent delta >=180 degrees refuse. Multi-key numeric Broken
editing remains; multi-junction linking, normalized overlay and graph marquee
remain outside this candidate. No new sampler, format or runtime field.
Cumulative T-813–T-820 manual qualification is pending; no GUI/GPU/inference run.
See [contract](bezier-tangent-linking.md) and [graph rules](graph-editor.md).

## T-821 local FFD-key candidate

One existing owned Deform key may be selected, retimed, deleted or assigned a
native Linear/Stepped/Bezier interpolation. Weighted offsets count influences;
empty resets are retained with the native sampler's asymmetric between-key
behavior. Unique owner and track required; linked/shared sources, ambiguous skin
overrides, invalid topology/binds/offsets and time collisions refuse. No implicit
rebind/repair, multi-key FFD, vector clipboard, graph average, offset table editor
or scalar tangent linking. Locks use existing slot/influence bone locks; no new
per-mesh/slot lock model. Complete-mesh/track captures have data-proportional cost,
not a claimed interactive performance budget. See [FFD key editing](ffd-key-editing.md).
Manual T-813–T-821 qualification stays pending; old runtimes/releases unchanged.

## Local T-822 PSD mapping boundary

[Explicit path resolution](psd-path-resolution.md) handles one unique simple region,
with exactly one unresolved missing image path in the source inventory. Multiple
missing image paths are conservatively refused because legacy provenance lacks
PSD file/layer UUIDs. A renamed group with multiple missing images is not resolved
in bulk. The user explicitly selects both sides; pixel similarity is not identity.
Changed tags/legacy semantics, skin overrides, shared art, meshes/links, target FFD,
sequences, flattened/hidden/undecodable art and conflicting provenance refuse.
The entire existing exact-path update batch is reviewed and included atomically.
No Photoshop effects/masks, universal persistent source identity or GUI certification
is claimed. An identical-content file replacement is accepted as the same snapshot;
there is no filesystem reservation against edits occurring after the final read.
T-302 remains PARTIAL; all T-813–T-822 manual checklists remain pending.

## Local T-823 retracing boundary

[Safe retracing](safe-mesh-retracing.md) supports owned rigid meshes and compatible
weighted meshes with an affine UV-to-Setup frame and canonical native binds.
The bounded weighted operation accepts at most four influences per row (the
painting default, not a file-format limit) and 16 million nearest-pair comparisons.
Exceeding these limits refuses; rows are never pruned to fit. Nearest-neighbour
may change the artistic weight field; unsampled source-vertex count is disclosed,
and a whole source BoneId disappearing refuses. Inline/Deform FFD, linked/shared
geometry, skin overrides, sequences, pinned edges and invalid/ambiguous data refuse.
No automatic FFD/topology migration or general retopology. Rigid non-affine tracing
retains its historical projection behavior; it is not newly qualified as distortion
free. Release performance measurements cover one synthetic 512x512 ring fixture,
not a universal image/mesh or GUI budget. T-402 remains PARTIAL. Manual review stays
pending, and no public build, model, renderer, runtime or format is changed.

## T-824 local onion-skin candidate

[Onion skinning](onion-skinning.md) remains PENDING MANUAL REVIEW. Ghosts are
artwork sprites, not a bone-overlay/motion-trail system. Past/future reuse theme
shear/rotate roles, so custom themes must retain contrast; changing those roles
also affects timeline channels. Settings are transient Session state and reset
on new application state, not project preferences. Key stepping uses global
Timeline times (FFD included), not selected-track times, events or markers.
No loop wrapping; unavailable ghosts are omitted. Selected-only means slot-owner
selection, not every influenced vertex. All rig dependencies still evaluate.
Physics ghosts are stateless reference poses, not accumulated playback physics.
No existing safe ghost cache exists: appearance-only changes recompute valid
poses (at most ten per visible render), without scheduling continuous repaints.
Measured CPU timings are configuration-specific; no universal performance claim.
T-823's six historical video/process timeout failures at default concurrency
remain an open environment/concurrency risk. Final gates use four test threads;
no timeout assertion or ignored-test policy was weakened.

## T-825 local Graph marquee

[Rectangular Graph selection](graph-editor.md#7-rectangular-key-selection-t-825)
selects visible scalar key centres only. No normalized overlay, lasso, segment,
FFD/event selection or collective key/handle drag. Ctrl-drag unions; removal is
existing Ctrl-click toggle, not a new subtractive rectangle. Alt/nonprimary and
navigation gestures remain reserved. Leaving the plot, focus/pointer loss,
Escape, view/selection/context changes and hidden-frame gaps cancel; no auto-scroll.
Selection applies only on release and uses native revision/document leases.
Pre-existing KeyClip/selection validation remains quadratic for large blocks;
10,000-key debug native set measured about 0.57 s, without a real-time guarantee.
Graph-specific middle pan remains absent; existing wheel/pinch/shared scrollbar
navigation and Dopesheet middle pan are unchanged. T-704/T-912 stay PARTIAL and
all T-813–T-825 desktop qualification remains PENDING MANUAL REVIEW.
