# T840A — Architecture decision proposal

Status: recommendation for review; not implemented. Preserve ADR0007's
caller-owned state and pure evaluate boundary. Its intention of display-rate
independence is valuable; the current implementation is not proof of it.

## Evidence changing the decision

- Genuine T839 exports still activate Physics from target; their bytes are
  preserved. At 30/60/120Hz the tested trajectories are close across languages.
  At 144Hz, fractional time, hitches and lifecycle boundaries they diverge.
- Force-only, no-parent-motion probe, 144 calls at 1/144: isolated native springs
  both end at rotation/x .2465458065. Sharing Rust PhysicsState gives .2423238158
  and .2684547305. Constraint calls consume one shared remainder. This difference
  is independent of legitimate parent/child dynamics and is not a good contract.
- Actual headless AppState replacement retains old PhysicsState when reloaded
  IDs match; its first simulated pose differs from a fresh instance. Seek/pause
  show pure rest while offsets remain stored. These are now tested observations,
  not the T839 inspection-only risk.
- TS direct zero-dt evaluation drains hitch backlog. Mix zero freezes state and
  anchors; the Rust path continues. TS low-level seek retains offsets; play
  clears them. Rust stateful entry points cannot accept active skin membership.
- A test-only common 120-tick schedule with priming at zero produces identical
  TS tick histories at all four display rates and matches canonical Rust reference
  within .01 translation/.001 rad/.0001 coefficient thresholds. Max observed
  world-translation error is .000004192. Catch-up of 60 requested ticks after a
  .5s hitch equals the uninterrupted tick prefix. No solver equation changed.

## Options

| Criterion | A: copy current Rust into TS | B: correct shared state design in both | C: small replay controller in consumers |
|---|---|---|---|
| fidelity | reproduces shared-remainder/transport quirks, still frame-end sampling | makes ownership/clock correct; equations need not be replaced | improves cadence/input sampling; shadow experiment supports the bounded case |
| determinism | same cadence still affected by f32 vs double and caller order | explicit ticks, epochs, input log and controlled ordering | strong only if every consumer uses it and supplies identical historical inputs |
| performance | low initial change, cap still hides work debt | bounded one-instance clock; changes state layout and safety work | per-tick animation/constraint evaluation costs more than once per display frame |
| compatibility | deceptively close to Rust, changes current TS behavior anyway | opt-in versioned profile, legacy retained; safety fixes independent | can start opt-in, but cannot silently qualify bypass/raw paths |
| regression risk | inherits demonstrated design weakness | moderate, changes require isolated tests at each step | low solver risk, higher integration risk across editor/player/export |
| maintenance | duplicates quirks and embeds another implicit contract | one documented state/clock contract, two small implementations | reusable orchestration concept; must prevent duplicated ownership/time accumulation |
| future exports | copying raw frame cadence still inadequate for offline replay | clean source for reset/replay/checkpoint metadata | fresh independent controller can power cancellable offline sampling |
| multiple constraints | known undesired coupling reproduced | clock consumed once, records independent except authored order | safe with one tick per whole instance; inner legacy remainder still needs a guarded boundary |

## Recommendation: B at the state boundary, delivered through a small C

Do not choose A. Do not rewrite the spring equations. Define a versioned opt-in
ReplayV1 controller with one requested/completed integer tick clock per instance,
per-constraint numerical records and explicit context epochs. Evaluate authored
inputs at each tick. First expose a bounded rational/tick bridge, then qualify a
seconds bridge. Keep raw LegacyT839 behavior recognizable until reviewed.

The common cadence experiment is **a test model**, not a new shipping adapter.
It avoids old cap/remainder differences for its exact 1/120 calls, but does not
repair raw entry points, invalid-state recovery, inactive skins, real transports,
long-session input logs, snapshot formats or arbitrary external root motion.
Simply wrapping some calls while leaving others unguarded would be insufficient.

Move time ownership rather than copying a per-bone remainder everywhere blindly:
one instance determines N due ticks once; each tick traverses all constraints in
authored order, each state record receives exactly one step. If a temporary
per-record remainder is used during migration, it must not be the authoritative
instance clock or be consumed by observational calls. The old raw API needs a
compatibility boundary, not invisible semantic replacement.

Keep the existing interactive budget of eight as a controller work limit, retain
integer backlog, freeze on zero dt, and provide explicit offline draining. These
are chosen policies, not claims that the current Rust cap drops backlog. Both
current implementations retain debt, with different rates and zero-dt handling.
When catch-up costs exceed real-time budget, show lag; do not claim exact replay,
no latency, and bounded work simultaneously for arbitrary hitches.

## Compatibility and risk management

No .ankh/native-v1/v2 migration in T840A. Keep the target reference correction,
ambiguous-reference rejection and render/Pixi/PathPosition guardrails. ReplayV1
selection is initially an explicit consumer option; persisting/exporting a
required profile needs a separate compatibility review. Old projects are not
silently reinterpreted. Capture old traces as characterization evidence, not
goldens to overwrite whenever trajectories change.

Safety fixes for document/rig replacement can precede opt-in cadence changes:
derived state must not leak to another document. Each needs RED before GREEN,
including unchanged author document/assets and Undo/Redo/save guarantees.
Fault transactions, replay checkpoints and per-tick evaluation add cost; measure
CPU work and memory separately when implemented. No performance benchmark or
universal fidelity claim is made from this small experiment.

Decision depends on maintainer review. See T840A-PHYSICS-REPLAY-CONTRACT.md for
normative proposal and T840A-NEXT.md for independently reviewable changes.
