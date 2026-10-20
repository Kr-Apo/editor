# T-815 — Numeric geometry editing contract

Local candidate based on T-814; **PENDING MANUAL REVIEW**. No published release,
project/export format, evaluator, renderer, runtime or AI tool is changed.

## Existing mechanisms retained

Canvas mesh and polygon tools own `Session.selected_vertices`, hit-testing,
Shift toggles/Ctrl box selection and interactive commands. Inspector coordinate
fields already existed for meshes, UVs, clipping and bounding boxes. T-815
replaces immediate numeric writes with validated drafts; it adds the missing
Path inspector route using the already-present EditPath/PolygonKind selection.
The UV pane now shares its existing pick result with the same vertex selection.
No second hit-test or independent persistent geometry selection is introduced.

## Address and units

A target is an existing displayed attachment: exact owning skin, active skin,
slot, attachment name and kind. The active skin can resolve to the default owner.
Only one slot/attachment can be edited at once. A focused attachment different
from the slot's displayed setup attachment is not treated as a canvas selection.

- **Position:** stored Setup vertices in the slot bone's local frame, X right,
  Y up. Region-to-mesh conversion already bakes region offset/pivot/rotation/
  scale into these coordinates. These are neither posed world coordinates nor
  screen pixels. Weighted meshes still store the same setup positions; the
  native evaluator applies the unchanged influences and inverse binds.
- **UV:** normalized coordinates of the **source texture**, origin top-left,
  U right and V down, independent of geometric vertices. The existing numeric
  [0,1] policy is explicit: out-of-range results are refused rather than clamped.
  Source UVs are not coordinates of a baked output atlas. The existing exporter
  handles region/trim/page remapping; no atlas or texture orientation is changed.
- Paths, clipping and bounding boxes use their existing stored local point arrays.
  No path interpolation/constraint or clipping end-slot/ordering logic is changed.

## Draft, Set/Add and precision

Select points using existing tools. Position and UV each have a separate draft
and Apply. The section displays kind/name, selected count, first eight indices,
coordinate space and policy. Text uses round-trippable f32 decimal values, not
rounded two/four-decimal presentations. Mixed axes are blank unless bit-identical.

**Set** writes an entered axis to every selected point (alignment). **Add** adds
that axis to each captured point. A blank axis leaves it unchanged. Switching
Set/Add clears the fields so previously displayed coordinates cannot silently
become offsets. Both X/Y may be applied in the same command; position and UV
remain independent. The number of affected vertices is explicit. No geometry
snapping is applied to typed values. Numeric point-to-neighbour snap is deferred.

Decimal/scientific numbers, Unicode minus and whitespace follow the syntax of
the compiled egui DragValue. Contrary to the historical task's assumption, that
widget has no expression evaluator: `120/2` is refused with a clear message.
No new parser, dependency or arbitrary-code evaluation is added. Empty drafts,
NaN/Infinity, unrepresentable f32 numbers and additive overflow are refused.
Finite points whose evaluated setup transform overflows are also refused.

Typing and Cancel write nothing. **Apply coordinates** / **Apply UV coordinates**
validate both axes and all selected indices before one existing native command.
A no-op creates no history entry. Separate applies do not merge with each other
or preceding drags. Existing drag merging is unchanged. Undo/Redo retain complete
native attachment snapshots; three cycles and evaluated weighted FFD positions
are regression-tested. Project Save/Save As and recovery still use T-813 writers.

## Safe selection and validation

Selection remains session-only. A lease scopes existing indices by target,
active skin, tool, edit mode, mesh-edit state and revision. A selection epoch
invalidates drafts even after selecting away and returning to the same indices.
Unrecognized document changes (including approved native AI edits), topology
changes, deletion, Undo/Redo and project replacement invalidate stale selection.
Native gesture wrappers rebind only indices produced by their own existing tool.
Skin/attachment switches cancel vertex/box/deform/UV captures.
Tool/skin setters and the work-mode boundary invalidate immediately, including
round trips while the geometry panes are hidden. UV targets cease
to be editable when they no longer match the current selected attachment.
Legacy callers' first selection binds on initial observation; normal selection
methods explicitly clear geometry when their target changes.

Before Apply, revision, selection epoch, complete attachment snapshot and inverse
bind values are rechecked. No late draft may edit another point with the same index.
Missing indices, multi-slot ambiguity, missing/locked bones, invalid UV/weight
array alignment or triangle indices, missing/non-finite/non-invertible inverse
binds and invalid influence references/weights are refused before dispatch.
Linked meshes and sources referenced by linked meshes are read-only in this
numeric path pending a separate sharing contract. No implicit weight pruning,
normalization, influence replacement, triangulation or bind compensation occurs.

All geometry/UV/path/polygon numeric commands require **Setup** at their native
command boundary. Animate keeps the existing interactive FFD tool and T-814 key/
tangent authoring. This is not numeric FFD editing. No new AI authoring capability
is granted; T-808 geometry/FFD read-only rules remain unchanged.

## Scope and evidence

Camera cursor anchoring, log zoom/UV fit, gizmos, hover labels and multi-bone
feedback are retained. No universal Frame Selection/rig-fit, X/Y/rotation snap,
new pivot compensation, numeric-on-drag or viewport refactor is added. These
historical T-708/T-902 criteria remain open. T-709 work is limited to labelled
TextEdits, ordinary keyboard focus and explicit Apply/Cancel; no complete keyboard,
screen-reader, DPI or visual accessibility qualification is claimed.

Tests use synthetic documents/temp directories and in-memory egui only. No GUI
Windows, browser, inference, capture or GPU test was performed. T-813/T-814/T-815
manual status remains **PENDING MANUAL REVIEW**.
