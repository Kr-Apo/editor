# T-821 — existing FFD keys in the Dopesheet

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** in this bounded scope. T-404/T-209
remain PARTIAL. No desktop Windows, artistic or runtime qualification is claimed.

## Find and edit a key

1. Open a disposable project with an owned mesh and existing Deform animation.
2. Enter Animate and choose the clip/skin displaying that mesh attachment.
3. Expand the slot group in the Dopesheet. Its **deform** row contains the keys.
4. Click one key. It uses the same shared selection feedback as other key rows.
5. Drag horizontally to retime it with existing frame snapping. Different drags
   are separate Undo entries. Exact-time collisions refuse without replacement.
6. Right-click the key. The menu identifies skin / slot / attachment and time.
7. Type Time (s), then **Move FFD key**; typing/menu closure alone changes nothing.
8. Use **Delete FFD key**, or **FFD interpolation** → native preset.
9. Undo/Redo restores exact offsets; Save As/reopen persists the authored changes.

Time/drag/delete/interpolation edits remain one-key only. T-834 adds same-track grouping for the dedicated clipboard; mixed/other-track groups remain refused. See [safe FFD clipboard](ffd-key-clipboard.md).
Scalar/slot multi-selection and existing clipboard remain unchanged. FFD cannot
be copied/pasted/mirrored through the general key clipboard or transferred via
T-819. The Graph has no fabricated scalar average for vector FFD values, and
T-820 Broken/Aligned/Mirrored are not applied to FFD. Use existing presets here.

## Native identity and topology

Deform storage stays `(slot, attachment, Vec<Key<Vec<Vec2>>>)`; it has no skin
discriminator. The transient typed key address adds the resolved owner skin and
never uses a fake bone or color key. Require one globally unique owner for the
slot/name and one matching track. Active skin may inherit that default owner.
Alternate overrides are ambiguous and refused even when their counts coincide.

Meshes must be owned, not linked or a source shared by a linked mesh. Rigid
vectors have one offset per setup vertex; weighted vectors have one per existing
influence entry (`influence_count`), not one per visible vertex. UV/vertex/weight
arrays, triangle/edge indices, finite values, normalized influences and usable
inverse binds are validated. Missing/invalid derived binds refuse this editor
path; it never silently invokes a bind fallback or repairs them.

Offsets and ordering are copied verbatim. Changing time/interpolation does not
change setup vertices, UVs, triangles, weights, binds, inline `ffd_keyframes`,
skins, image bytes or bone keys. Inline attachment FFD is not an animation track
and is neither exposed nor retimed by T-821.

## Timing, interpolation and history

Times must be finite nonnegative f32; overflow and occupied exact-time
destinations refuse. Moves sort and re-resolve by unique resulting time. Duration
uses existing max(content_duration) policy, never truncates keys; Undo restores
the prior duration exactly. Deleting the last key removes that track, matching
native DeleteKeys policy; Undo restores its original position and full contents.

Linear, Stepped and Bezier use the unchanged vector sampler, with interpolation
on the ending key. X handles finite [0,1], Y finite with overshoot allowed.
Offsets are not scalar tangent channels. Empty vectors remain exact native reset
values: between empty→full the sampled left vector remains empty; between full→
empty its left tail is retained until the endpoint reset. No zero expansion,
truncation or new interpolation policy is introduced. Large finite values can
still overflow existing sampling; this is not a new runtime fidelity guarantee.

One targeted Animate command publishes a prepared track and duration together,
with one Undo for an action/continuous drag. Unique tokens prevent distinct drag
merging. No-op/refusal/menu cancellation dispatch nothing and preserve Redo.
Undo/Redo preserve full native data; session selections must be reselected after
history/external edits. The current key's own valid move can rebind its new index.

## Stale contexts and locks

Shared T-814 selection now supports an FFD lease guarded by revision, T-818
document identity, active clip/skin/work mode, displayed attachment, whole track
and mesh snapshot including sorted inverse binds. Topology/UV/influence/bind
changes invalidate even when lengths stay equal. Skin/mode setters clear FFD
leases immediately, including change-away/back while the panel is hidden.
New document/clip, deletion, Undo/Redo and stale captures cannot reuse an old index.

Setup keeps geometry editing and refuses animated-key edits. Existing slot-bone
or influencing-bone locks refuse FFD-key actions. There is no new per-slot lock
type. Incompatible rows are greyed with an explanatory label/tooltip. Complete
track/mesh capture costs scale with data size; performance/desktop comfort is
deferred, not certified by headless egui tests. T-813 Save/autosave/recovery,
T-815 geometry, T-816 rigging and T-818 diagnostics remain independent safeguards.
