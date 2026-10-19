# Guarded weights v1
ankhimate_read_mesh: skin, slot, attachment, start, count (1–32). Exact owned
mesh page: vertex-local coords/UV, named weights/sums, locks, binds, FFD and
limits. Replies <=32 KiB; truncation means incomplete knowledge. No auto geometry.
ankhimate_weights: summary + actions, Setup, approval, one Undo lot.
- mesh.set_weights: op, skin, slot, attachment, vertices [{index,influences:
  [{bone,weight}]}]. ALL existing influences required. Finite coefficients [0,1],
  normalized natively. No added/removed/reordered IDs; pruning REJECTS.
- mesh.normalize_weights: op, skin, slot, attachment, vertices [indices].
- mesh.smooth_weights: same + bone, strength (0,1], iterations 1–4. Native
  triangle-neighbour brush on one EXISTING influence; others share the remainder.
32 distinct vertices/action, 16 actions/128 vertices per separate weights lot.
Mesh <=4096 vertices/8192 triangles, 1–4 influences/vertex (tool budget, not engine limit). Binds must match native Setup; kept bit-identical. No implicit
rebind. Linked targets/sources, shared slots, constraints, skin-dependent chains,
locks, ambiguity or bad topology/binds REFUSE. Geometry/UV/FFD read-only; indices
have shared meaning. Read a small joint zone, propose mild smoothing, inspect
before/after and await approval. No artistic guarantee. Images
need T-806 consent; distinguish facts from observations/estimates.
[Examples](weights-examples.json); Rust schema/validation is authoritative.
