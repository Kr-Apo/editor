# T839 — Proposed future PathPosition contract (not implemented)

## Current native contract

`Timeline::PathPosition` targets a ConstraintId and scalar keys. Author disk data:
`kind: path_position`, constraint name, time/value/curve keys. Value is an
**absolute start fraction**, not a setup offset; it is not an angle or spacing.
Native author schema version remains 1, which does not imply older readers know
this variant. Angles in Pose remain radians.

`formats/src/{schema,convert,path_position}.rs` preserve names on disk and IDs in
memory, finite sorted keys, unique track identity and an exact valid Path target.
Every evaluation clears `Pose.path_position`. Valid timelines sequentially lerp
from setup/current override by alpha before the ordered constraint pass.
The solver resolves the **default-skin setup attachment**, samples the polyline
in the host bone's world transform, writes local transforms and updates children.
Constant speed selects arc length versus vertex index; open paths clamp, closed
paths wrap. Static spacing/mixes and skin ownership remain explicit.

TS currently has static PathConstraint.path.position, no animated track/override.
Its selected-skin lookup and exact-corner segment search differ from Rust.
PixiJS rejects constraints. Native/template planning and preflight deliberately
reject PathPosition before writes. No guard is unlocked in T839.

## Candidate consumer contract

A future `PathPositionTrack { constraint: string; keys: ScalarKey[] }` collection
on RuntimeAnimation is clearer than overloading IK channels. Reuse existing
Linear/Stepped/normalized Bézier curves; finite fractional values may overshoot.
Reset overrides per evaluation, sample at native f32 boundaries, sequentially
lerp from the Path setup position, apply in authored constraint order.

**Additive v2 alone is unsafe:** current old readers accept unknown animation
properties and silently ignore them. Before shipping use a version old readers
reject (candidate v3), or an explicit required-capability mechanism enforced by
every permitted reader. This is a proposal; no version, field or capability key
was introduced here. No format change is required for T839's Physics fix.

Validate target name/type, one track per target, finite times/values/curves,
sorted keys, constraint order, path slot/attachment identity and supported skin
resolution. Refuse missing/ambiguous/non-path targets. Do not clamp fractions to
[0,1] before solving; open/closed path semantics determine out-of-range behavior.
Absent tracks preserve static setup semantics.

## Tests and acceptance before future enablement

Already rerun: core PathPosition tests cover five bones at 0/.25/.5/.75/1,
static-reference equality, curves/overshoot, open/closed endpoints, spacing,
stale override reset, wrong/duplicate targets and parent transforms. Document
tests cover commands/Undo/Redo/retiming; formats tests serialization/validation;
export tests native/generic/custom/plugin refusal, rendered validation and static
availability. These are Rust and guard tests, not a TS implementation or GPU proof.

Future red→green plan:

1. Generate genuine native static-path references: unequal segments, exact corners,
   open/closed ends, both spacing modes, default/alternate skins, rotated/scaled
   parents. Align static solver behavior first.
2. Introduce the guarded wire contract and TS override only after that decision.
   Compare positions/affines at 0/.25/.5/.75 and stepped boundaries, .01 units;
   explicitly declare angular tolerance in radians.
3. Test absent track, reused Pose/reset, bad numbers/IDs/duplicates, first-key holds,
   looping, alpha blends and other constraints before/after Path.
4. Prove old v1/v2 unchanged and old consumers reject rather than drop new tracks;
   maintain pre-write diagnostics and staged rollback tests.
5. Qualify each adapter separately. PixiJS needs an explicit capability decision.
   Compare CPU and real WebGL after numeric parity, never infer visual fidelity.

Animated PathSpacing, cubic geometry authoring, physics/path composition and
arbitrary skin-dependent topology remain separate scope.
