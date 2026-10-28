# Proposed T840B4 — Canonical input sampling and precision

Planning only. Wait for B3 review before implementation.

B3 establishes an executable, Physics-only instance boundary and eliminates
accidental time consumption across independent constraints. It does not sample
real clips, fades, external roots or user transport history, and it does not
activate ReplayV1 in any current graphical consumer.

B4 should specify and test:

1. A canonical source of pre-Physics input locals at each integer tick N/120,
   including setup offsets, animation ordering, fades and external root motion.
   Never replay several historical ticks using the final transform of one frame.
2. Exact timestamp construction and the f32/double boundary. Retain B2's exact
   APIs; separately qualify any seconds/nanoseconds bridge and its overflow,
   rounding and long-session policy before accepting actual game deltas.
3. Pose-pure preparation without duplicate Physics application; authored
   constraint-order integration for mixed rigs needs a deliberate extension of
   B3's present refusal, not silent support for unqualified IK/Transform/Path.
4. Inactive membership, dynamic context changes and explicit invalidation.
   Preserve B1 replacement isolation and instance-bound tokens. Do not reset
   every simulation for every key edit, and do not infer continuity from IDs.
5. Per-tick traces at all canonical cadences, hitches/backlog and real clip
   boundaries. Preserve B3's predeclared thresholds and Legacy evidence. Report
   the exact supported surface, rather than claim universal engine parity.
6. Cost of per-tick pose preparation and defensive snapshots. B3's descriptive
   measurements show measurable overhead, especially TypeScript frame copies;
   optimize only with state-isolation and Legacy regression proofs intact.

Numerical transaction/rollback remains B6. Do not retry a faulted B3 instance as
though its private spring records were restored. No schema change, rendered
Physics, GPU/adapter activation or release follows automatically from B4.
