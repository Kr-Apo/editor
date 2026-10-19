# Native mesh weight assistance (T-808 development)

T-808 extends the accepted T-807 branch locally. No GUI, visual placement,
real inference or runtime graphics qualification was performed. Maintainer
acceptance of T-806/T-807 vision/assembly and T-808 weights remains pending.

## Read and propose

The embedded [manual](ai/WEIGHTS.md) and [examples](ai/weights-examples.json)
describe the typed Rust tools. Enable metadata/tools, editing explicitly, Setup.
`ankhimate_read_mesh` addresses an exact owned skin, slot, attachment and a
contiguous vertex page (`start`, `count` 1–32). It reports local positions/UVs,
existing influence names and sums, normalization, bind matrices and locks, FFD
presence and edit limitations. Meshes are not treated as regions. No geometry
is automatically inserted into the initial context, no image/source paths/bytes
are read by this tool. Replies cap at 32 KiB; oversized replies require a smaller
page. Invalid rows expose at most eight influences with a truncation flag.
Linked geometry is reported as raw owned data, never falsely resolved geometry.

`ankhimate_weights` proposes three existing-command adapters:

| Typed operation | Native mechanism | Scope |
|---|---|---|
| `mesh.set_weights` | `SetWeights` + `normalize` | Explicit existing rows, all their existing named influences |
| `mesh.normalize_weights` | `SetWeights` + `normalize` | Explicit selected indices |
| `mesh.smooth_weights` | `brush` / `BrushMode::Smooth` + `SetWeights` | One already-influencing bone, explicit vertices, strength (0,1], 1–4 iterations |

These names follow the native mesh/SetWeights and brush vocabulary; no second
painting algorithm is introduced. Smoothing uses native triangle-neighbour
averages, computed from the previous iteration, and proportional redistribution
among the remaining influences. Results are deterministic, not artistic proof.
The caller supplies all coefficients for direct edits; they are normalized by
the native rule. The proposal displays actual normalized values, not guesses.

## Preservation boundary

One separate weight-only Setup lot: <=16 actions, <=32 distinct existing vertices
per action and <=128 selected vertices total. Writable meshes: 3–4096 vertices,
1–8192 valid triangles, valid UV/weight arrays, 1–4 influences per vertex. Four
is the native **default budget**, not a universal engine limit. Larger/over-budget
meshes remain inspectable by pages; this tool does not prune them automatically.
All values finite; positive existing weights >1e-4 and <=1. Direct input allows
[0,1] coefficients but an output which removes an influence is refused.

Influence IDs and order per vertex must remain unchanged; unknown/new/removed
bones or native normalization that prunes influences are rejected. Bones and
weight-paint locks are rechecked at approval; an affected row containing any
locked influence is refused as a whole. No redistribution around a lock occurs.
Exact-owned meshes only, no ambiguous names/fallback. Shared slots, linked
meshes and sources referenced by linked meshes are protected. Constrained rigs
and skin-dependent bone chains are excluded pending separate bind qualification.

Inverse binds must be finite/invertible and match native FK Setup within 1e-4
per affine coefficient. The new opt-in SetWeights `preserving_bindings` path
keeps these existing matrices bit-identical on apply/Undo/Redo. Its documented
precondition is unchanged geometry/setup/ordered influence IDs; the AI validator
checks these before use. Ordinary manual SetWeights calls retain their prior
bind invalidation/rebuild behavior. SetWeights also observes native BoneId remaps.
Approval detects changed bind values and reviewed before/after rows even if
document revision was not bumped. Native DeleteBone Undo also restores surviving
weight and inverse-bind references to the recreated bone ID; no coefficients or
FFD indices are rewritten.
No implicit rebind or bind-pose editing is exposed to the model.

FFD tracks and inline FFD arrays are checked for finite, supported indexed data.
Their payloads, times, curves and indices are never modified. Geometry/vertex,
UV, topology and FFD editing remain read-only: native commands exist, but Setup
vertex edits can change bind/FFD meaning and topology edits can clear weights;
exposing them safely would need a separate contract. No new mesh generation.

## Confirmation and history

Existing proposal cards show identity, operation, selected count, warnings and
collapsible before/after named weights. Explicit native approval/refusal remains
required. Validation runs on an isolated document. One accepted lot uses the
existing transaction, one Undo; no merging with brush strokes. Invalid lots,
refusals, late locks/references and stale revisions do not mutate the project or
discard Redo. Applied receipts remain authoritative; Save stays manual. The
existing bounded correction mechanism handles structured errors. T-806 image
consent/protocol and T-807 region/bone assembly operations are unchanged.

Use the delivered QUICKSTART and isolated weighted project for manual A/B/C/D.
Only the maintainer will test Qwen/NInfer and visual/artistic behavior. Tests of
native weights, history, Save/Reload and v2 export are headless, not runtime
rendering acceptance. No renderer, runtime or project/export format changes.

The delivery-copy test is headless and explicitly selected with
`ANKHIMATE_T808_DEMO`; it stays ignored in ordinary checkout tests, which must
not depend on external delivery files. The real inference tests stay ignored.
