# T840A — Physics replay contract

Status: normative **proposal for a future opt-in ReplayV1 profile**, submitted for
review. Not a description of current conformance, not a new public API, and not
an implementation. T840A changes no integrator, runtime format or production
transport. LegacyT839 means the existing behavior at the accepted checkpoints.
MUST/SHOULD below govern the proposed profile, not historical files retroactively.

## 1. Guarantee and units

For identical rig revision, profile, initial state and timestamped input history,
the same completed simulation tick MUST produce the same pose within the declared
cross-language tolerances, independently of how display frames group those ticks.
No bitwise f32/double identity is promised. A different input history is not a
parity test, even if the clip name and final playhead match.

Comparisons declared before measurement: local translation .01 author units;
world translation .01 world units; local rotation .001 radians using wrapped
shortest angular distance; world a/b/c/d coefficients .0001 absolute,
dimensionless. All must pass; nonfinite values always fail. These are initial
fixture qualification limits, not an arbitrary-pixel or universal-rig guarantee.
Large scale/bone length can magnify small angular errors; end-point/world position
tests remain necessary. No tolerance may be enlarged after observing a failure.

## 2. State ownership and identities

One instance owns one clock and its own simulation records. A shared immutable
rig/atlas never owns mutable simulation. An instance has a rig/context epoch,
input-log revision, completed integer tick, requested target tick, backlog and
transport state. Each `(constraint identity, driven bone identity, epoch)` owns
offsets, velocities and initialized parent anchor. The frame clock MUST NOT be
consumed separately by each constraint. All active constraints receive the same
tick duration once per completed tick, in authored constraint order.

Constraint order remains significant when one physically affects another's
parent. That legitimate coupling differs from two independent springs stealing
fractional time through a shared remainder. T840A reproduces the latter in Rust;
it MUST NOT become the common contract merely because it is historical.

| Context operation | Keep | Clear / reinitialize |
|---|---|---|
| create instance | immutable asset references | clock, offsets, velocities, anchors, pending events/fade; new epoch |
| first physical evaluation | pure starting pose at declared epoch time | prime anchors at that pose, zero offsets/velocity before first positive tick |
| explicit reset at current pose | selected clip/time, playback paused/running choice | simulation records, backlog and input checkpoint; prime a new zero-offset epoch there |
| historical replay reset | rig revision/profile + recorded inputs | reset to the declared historical origin/checkpoint, not current screen pose |
| replace document/rig | UI preferences only | all physical state and checkpoints, even if names/slotmap IDs happen to match |
| topology/parent/bind/constraint identity change | unrelated instance data | affected state and dependent replay checkpoints; no migration by matching name alone |
| edit earlier animation keys/forces | author document and Undo/Redo | invalidate replay snapshots dependent on that input revision |
| skin, appearance only | state for constraints whose membership/parent/bind is unchanged | nothing merely for a texture/color change |
| constraint becomes inactive | instance clock, other active constraints | that constraint's velocities/offsets/anchor; no dormant catch-up debt |
| constraint reactivates | other active state | prime its current pure anchor; no impulse from time it was absent |
| destroy instance | shared immutable assets owned elsewhere | all instance records, pending transport/events and references; future updates fail clearly |
| two characters share asset | asset bytes/definitions | never share clocks, velocities, anchors, checkpoint buffers or fault state |

Physics state is derived session/runtime state: do not serialize it into the
author document, add it to Undo or let a preview change dirty state. Save/Reload
does not resurrect velocities; a loaded document starts a fresh epoch.

## 3. Clock, integration and display cadence

The logical step is exactly 1/120 second. The canonical identity of a step is its
integer tick N, with authored sample time `epoch + N/120`, not repeated addition
of a f32 frame delta. Native f32 and TS number still perform their existing
arithmetic inside the solver until a separately reviewed numerical change.

Time ownership MUST move out of the per-constraint call chain: request time once
per instance, determine how many ticks are due, then evaluate complete ticks.
Every tick samples animation, external inputs and transition weights at its own
timestamp; composes the parent pose; applies ordered constraints and updates
children. Passing the final frame's parent transform to several historical
substeps is insufficient. Do not discard parent motion when an incoming delta
is too small to complete a tick.

The proposed time interface accepts a bounded rational or canonical absolute
time, independently of renderer delta precision. The executable test model uses
integer rational seconds: due tick = floor(numerator×120/denominator), with safe
integer bounds, monotonicity and zero-delta checks. It proves exact schedules at
30/60/120/144 Hz without imitating floating clock errors.

For the proposed seconds bridge, canonical elapsed time is nonnegative integer
nanoseconds relative to an epoch. Convert absolute f64/number seconds by rounding
seconds×1e9 to nearest integer, ties upward. Use integer arithmetic (u128/BigInt
intermediate) for `dueTick = floor((ns×120+60)/1e9)`. The +60 defines a <=1ns
canonical boundary band so rounded timestamps of exact 1/120 instants land on
their intended tick. This is an explicit quantization convention, not an epsilon
added to force failed physics comparisons to pass. Integer bounds/overflow must
be validated before mutation. Tick times themselves remain N/120, not rounded
nanoseconds re-divided repeatedly.

If only deltas are supplied, retain a compensated f64/number elapsed sum and
fractional conversion residue, then quantize the accumulated absolute target;
never round each display period independently. Prefer authoritative absolute
timestamps or integer/rational targets. Never promote an already accumulated
f32 clock and claim lost precision is recovered. Pure test vectors cover the
nanosecond boundary and four one-second schedules; a production bridge, long-run
conversion and resource bounds remain T840B gates. The test model is not that
shipping bridge. Inputs distinguish original versus quantized timestamps.

### Backlog and hitches

- Interactive work budget: at most eight complete instance ticks per positive
  request/pump. This is a scheduling limit, not eight substeps per bone.
- Retain all requested elapsed time as an integer target/backlog. Do not clamp
  dt, clear the remainder or abandon time silently. Backlog counts use bounded
  integers; an unrepresentable request fails before mutation.
- `dt=0`/same timestamp MUST NOT integrate, drain backlog, update anchors or alter
  velocities. Read/render must be observational. A separate explicit offline
  replay pump may process pending ticks while the target stays fixed.
- A .5 s hitch requests 60 ticks. The first budget processes eight; 52 remain.
  After explicit catch-up, the result must match a continuous 60-tick prefix.
- Display the last completed simulation pose and expose lag. Do not combine a
  future parent pose with stale physical offsets and call it synchronized.
  If the machine cannot catch up, it can lag, pause or require explicit reset;
  exact historical simulation cannot also guarantee zero recovery latency.
- Offline/video replay drains in cancellable budgeted chunks until its target
  is complete. Never label an incomplete target frame as complete.
- Pause freezes target and completed tick. Paused wall time is not added on
  resume. Repeated hitches must not overflow counters or cause unbounded event
  allocation; fail a configured resource limit explicitly rather than drop time.

Presentation interpolation is optional and outside this first qualification.
If later added, it uses completed cached poses, never advances simulation, and
has a declared timestamp/latency convention. Existing renderers need not change.
At 144 display Hz some 120-Hz simulation poses may repeat. At 60-FPS export,
frame i requests tick 2i; t=0 is the primed pure pose, and loop end stays exclusive.

## 4. Input history

Animation timelines are sampled at canonical tick times, with explicit clip/lap,
looping, blend weights and constraint order. Physics persists across a loop;
loop-boundary discontinuity in authored motion is an input, not an implicit
reset. Pausing/seek/restart policies must not depend on redraws or `pose()` count.

World/root transforms, forces, skin membership and user controls driven by a game
need timestamped inputs with a declared interpolation policy. A frame-end sample
alone cannot reconstruct motion that happened during a hitch. Determinism is
conditional on identical sampled history, not identical final transforms.
Missing history prevents a certified replay; do not fabricate it silently.

External event ordering at the same tick must be stable and recorded. Simulation
replay does not re-emit gameplay/audio events by default. Keep the existing
incoming-clip event policy; changing animation mixing/events is separate scope.

## 5. Four distinct operations

1. **Pure pose**: evaluate authored transforms/constraints without PhysicsState;
   physics contributes rest. No clock/anchor mutation.
2. **Physical advance**: complete scheduled ticks over an existing epoch/input
   log, committing state only for valid ticks.
3. **Seek preview**: show the pure pose at t, pause transport, invalidate stale
   physical continuation. It is explicitly not historical physical replay.
4. **Replay to t**: restore a valid origin/checkpoint and apply every required
   tick and recorded context change up to t, without gameplay-event side effects.

| Proposed transport | Physics rule |
|---|---|
| play(restart=true) | new zero-offset epoch at clip start; clear fade/state/backlog; prime t=0 |
| play(same clip, restart=false) | no reset or extra advance |
| play another clip without a fade | reset; no implicit reuse of the old clip's velocities |
| pause | keep last valid simulated pose and state; no pure/rest pop or catch-up |
| resume after ordinary pause | continue from frozen logical time, excluding paused wall time |
| stop | clear physical state/backlog/fade; retain clip selection; pure selected clip@0 (setup if none); no events |
| seek(t) | pure preview, paused; clear/mark physical continuation invalid; no fake `pose(dt=0)` replay |
| resume after seek | require explicit historical replay or explicitly labeled restart-from-current-pose; do not silently choose |
| crossFade | preserve state only for the same compatible instance/epoch; use blended input at every tick; no extra integration on the command itself |
| reset simulation | clear numerical state/backlog and prime current pose; transport paused/running flag is separate and retained |
| replay from origin/checkpoint | validate rig/profile/input revision; reset/restore exact recorded state, clock and anchors, then drain ticks deterministically |

Replay from an arbitrary nonzero time without its state means a **new rest-start
epoch**, not the pose that would result from simulation since time zero. Checkpoint
content must include clock, all numerical state/anchors, activation membership,
clip/fade state and input-log position. A checkpoint with a different rig/input
revision is rejected. Interrupted crossfades can still expose the existing base
pose jump; this contract does not create a new mixing engine.

Current Editor and TS behavior differs from this table; measurements and actual
AppState/player tests are in T840A-PARITY-RESULTS.md. Nothing here silently changes
legacy controls or introduces a new user flow in T840A.

## 6. Mix, activation and numerical faults

Mix controls output contribution, not the existence of an active simulation.
For an active constraint, mix=0 MUST keep advancing its state/anchors; restoring
mix exposes its current response, not old motion. Inactive membership is different:
clear that record and re-prime on reactivation. A visual-only skin change keeps
compatible records. A caller must not route skinned physics through an API that
cannot receive the active membership (a current Rust stateful-API gap).

Preserve native finite parameter conventions where well-defined: mass floor .01,
inertia/damping/mix clamped to [0,1]. Negative spring strength is invalid for the
new qualified profile. Validate all numbers, transformed forces, references,
domains and time before advancing. Missing driven bone/constraint/context must
invalidate the relevant epoch with a clear diagnostic; never guess another bone.

A tick yielding nonfinite offsets, velocities or pose MUST NOT commit its clock
or corrupt state. Restore the last valid instance state/pose and return an
explicit fault; repair/reset/disable is a deliberate caller action. Invalid
parameters can be rejected before simulation. One invalid character must not
affect another character or mutate its document/asset. Diagnostics should be
deduplicated by fault/context, not silently suppressed. This transactional rule
requires implementation/performance qualification in T840B; neither today's Rust
reset guard nor today's TS direct evaluator alone satisfies it.

## 7. Compatibility and implementation boundary

ReplayV1 must be explicitly selected and versioned outside the existing v1/v2
data semantics initially. LegacyT839 remains the default for unqualified old
projects/exports until a separately approved migration/profile choice exists.
Do not hide changed trajectories behind the same unqualified “physics parity”
claim. Reference failures in this mission remain visible, not updated oracles.
Safety bugs such as state reuse across replaced documents warrant targeted
RED/GREEN fixes, independently of dynamic-profile compatibility decisions.

No new author or runtime field, profile selector, replay adapter or solver is
shipped here. Persisting profile selection or exporting a requirement sidecar
needs explicit design/review; an application can initially pass an explicit
consumer option. Existing v1/v2 bone references, PixiJS constraint refusal, CPU
physics refusal and runtime PathPosition refusal remain unchanged. A future video
export must use its own fresh qualified replay state, never the editor viewport's.
