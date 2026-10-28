# T-840B4 — Next decision (planning only)

B4 stops at an opt-in single-clip numerical wrapper. No B5 code is implemented.

Before enabling an editor or game player, approve an explicit B5 policy covering
clip changes/restart, seek/backward time, pause/resume, speed, interrupted fades,
skin membership replacement and external input histories. Specify which actions
create a new epoch/reset and which replay from a known initial context; ordinary
loop wrapping currently preserves Physics state. Do not infer missing past root
inputs from the most recent frame. The general float-delta bridge is still absent.

Keep mixed IK/Transform/Path contexts refused until combined scheduling and ownership
are defined. Qualify any skin transition before changing fixed default-skin membership.
Keep unsupported clip channels explicit, including events and attachment sequences.
Qualify author bone offsets versus baked v2 key boundaries separately.

Proposed small checkpoints, subject to approval: first one transport with exact
requests and lifecycle tests; then explicit clip/reset policy; then skin contexts;
then constraints/mixing only with a separate contract and parity evidence. Preserve
Legacy by default until an opt-in application integration is reviewed.

Performance follow-up may consider a validated prepared-pose seam or alternate
observer contract, with before/after measurements and defensive-isolation tests.
Do not fold it into renderer work. B6 numerical rollback remains distinct: a failed
tick currently retains the last valid public pose but can mutate private spring state.

No GUI/WebGL compliance is inferred from B4. Request dedicated graphical qualification
only when a reader actually activates this surface. No policy is implicitly approved
by this planning document.
