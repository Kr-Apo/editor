# Known limitations — local T-811 candidate

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
