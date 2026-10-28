# Artistic native operations — P01-RC1

Read exact mesh identity/influence order first. ankhimate_read_deform returns
1–32 stored offset pairs, never inferred poses or images. deform.key records a
FULL native offset array (rigid vertex order; weighted vertex/influence order,
max512 pairs), in Animate; no sparse/linked/locked or invalid-bind target.
Same time replaces; full zero offsets mean rest. deform.key_delete and key.delete
require one exact stored key. event.add uses a name/time and default payload.
Propose, review before/after, wait for approval; native revision/locks are rechecked.
No topology/bind changes or artistic guarantee. Runtime admission is separate.

## Detailed reference

# Native artistic edits — P01-RC1 candidate

Existing weights: `ankhimate_read_mesh`, `mesh.set_weights`, `mesh.normalize_weights`, `mesh.smooth_weights`. Same ordered existing influences only; no new binds or topology. Constrained/shared/linked or invalid weight targets remain refused.

FFD: read with `ankhimate_read_deform` (animation, skin, slot, attachment, key_index, start, count=1..32). This returns stored keys, not interpolated poses; absent tracks are reported, never fabricated. Only requested offset pages are transmitted; no image bytes or local paths.

Use `ankhimate_animate` with a nonempty summary and actions:

- `deform.key`: animation, skin, slot, attachment, time, offsets. Full native array of [x,y] pairs, at most 512 pairs. Rigid meshes use vertex order; weighted meshes use vertex then existing ordered influence. Values are native author-space offsets, not world positions or skinned GPU vertices. All values finite, magnitude <=1e6. Same native time replaces a key, preserving its interpolation. New keys are Linear. No sparse/empty arrays, linked/shared owners, inline FFD, invalid binds or locked targets.

- `deform.key_delete`: same identity/time, without offsets; requires one exact stored key.

- `key.delete`: animation, bone, channel, time; one exact bone transformation key. Existing channel spellings/units apply.

- `event.add`: animation, name, time; native default payload only. Unique name/time pair, <=256 events/clip; duplicate requests are refused.

FFD requires Animate. A proposal validates an isolated document, shows before/after, waits for approval, then rechecks revision, locks and mesh identity. A rejected/invalid lot preserves document and Redo. One approved lot is one Undo entry. Save explicitly afterwards.

Example (rigid mesh with exactly three vertices; do not reuse these offsets on another mesh):

```json

{"summary":"Return the triangle mesh to rest","actions":[{"op":"deform.key","animation":"Idle","skin":"default","slot":"body","attachment":"triangle","time":0,"offsets":[[0,0],[0,0],[0,0]]}]}

```

Invalid: one pair for a mesh requiring three; a missing skin/bone/key; a NaN value; an out-of-duration time. No coercion or implicit topology/bind repair.

The GUI also offers Key FFD pose and Key setup FFD; the latter records a full zero-offset key and does not change Setup. Key rest at the beginning and end explicitly; one held key affects the whole clip.

Limitations: no arbitrary vertex/UV/topology changes, influence membership changes, attachment rename, custom event payload editing or constraint authoring through these new operations. They remain manual or future work. A17 runtime admission is separate (512 *scalar* FFD components, whereas this authoring tool caps 512 *pairs*). No actual model/provider or artistic qualification is claimed by deterministic tests.
