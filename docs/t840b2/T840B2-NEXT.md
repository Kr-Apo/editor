# Proposed T840B3 — State and constraint scheduling boundary

Planning only; do not begin before B2 review.

B2 supplies exact per-instance time ownership but leaves every existing consumer
and integrator untouched. B3 should connect this clock to actual simulation
behind an explicit opt-in ReplayV1 boundary, retaining LegacyT839 defaults.

Required decisions and tests:

1. Bind one clock and its numerical state to one instance/context. Reuse B1
   successful replacement disposal; reject old-context work. Do not infer
   continuity from a reused bone/constraint ID or an epoch from another clock.
2. Let the controller consume the instance's due time once. For each committed
   tick, traverse all participating constraints in authored order and give each
   numerical record one 1/120 step. Remove accidental shared-remainder coupling
   between otherwise independent constraints without changing spring equations.
3. Prime real anchors at the simulation origin. Specify parent/root-motion
   inputs and active constraint membership; do not silently use display-frame
   samples as historical per-tick inputs. Some consumer sampling work may belong
   to B4 rather than be hidden inside B3.
4. Preserve budget eight, backlog, explicit pump, unchanged-time observation,
   and the distinction between pure seek and replay. Do not claim arbitrary
   floating delta fidelity until a separately tested conversion contract exists.
5. Define consumer failure handling before retrying stateful ticks. B2 commits
   counters after reported success but cannot undo mutated physical records;
   numerical transactional rollback remains B6 work, not an implicit B2 feature.
6. Reuse the exact clock vectors and retain all T839/T840A/B1 evidence. Add
   independent-vs-shared-asset, multi-constraint and parented-rig regressions,
   canonical cadence and hitch/retry tests against actual numerical states.

No .ankh/v1/v2 schema migration, rendered Physics support, graphics adapter,
transport rewrite, mixing correction or broad epoch invalidation follows
automatically. Declare which core surface B3 introduces and keep unsupported
consumers guarded. The remaining 31 T840A divergences must stay traceable.
