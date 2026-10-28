# After T840B1 — prerequisites for B2

Status: planning only. T840B2 through B8 are not authorized or implemented here.

B1 establishes context disposal when the Editor successfully exchanges a whole
document/rig, including native history. It does not establish clock ownership,
general invalidation epochs, transport replay or Rust/TypeScript fidelity.

Retain the T840A decision: caller-owned state and pure evaluation, unchanged
spring equations, an explicit opt-in ReplayV1 boundary rather than silently
reinterpreting LegacyT839. One instance owns requested/completed integer ticks;
one tick evaluates authored inputs and traverses its constraints in order. Do
not copy the current global Rust remainder coupling or the TS zero-dt drain.

Before implementing B2, agree and test:

1. The public tick/rational input API and overflow/error bounds. The T840A
   BigInt clock is an experiment, not a shipping seconds bridge. Qualify f32 /
   double conversion boundaries separately before promising arbitrary seconds.
2. One instance clock, not independent time consumption by each constraint.
   Numerical records remain per constraint/bone, with deterministic traversal.
3. Eight-step interactive work budget, retained backlog, observational zero-dt
   calls and an explicit offline drain operation. Expose lag rather than imply
   unbounded catch-up at bounded cost. Keep the old raw API recognizable.
4. A context identity/epoch API covering new/replaced documents and rigs. Reuse
   B1 replacement semantics; define later invalidation for edits, binds, skins
   and removed/reintroduced constraints separately, without blanket resets on
   every key edit. A matching entity ID alone is never continuity evidence.
5. Input priming at tick zero, parent/root motion history, and distinction between
   a pure seek pose and stateful reset/replay. Future checkpoint caches require
   explicit invalidation and bounded memory, not document serialization of
   viewport velocity.
6. Fault/nonfinite transactions and inactive-constraint/mix policies; preserve
   native target serialization, existing consumer refusals and explicit warnings.
   These policies are not solved by B1 and should be staged separately.
7. RED/GREEN cadence and multi-constraint tests at controlled tick schedules;
   independent instances, hitches/backlog, reset and repeated playback. Preserve
   all 60 T840A traces and thresholds. Do not overwrite their 31 divergences to
   manufacture a passing baseline.

The T840A common-120-tick experiment supports further investigation: the tested
tick schedules matched within the declared numeric tolerances. It did not
qualify GUI transport, arbitrary seeks, dynamic skins, historical input logs or
long-session replay. B2 must state exactly which surface is introduced and keep
unqualified profiles guarded. No file-format, adapter or rendered Physics
support follows automatically from a clock implementation.

Wait for the maintainer's B1 review before starting B2.
