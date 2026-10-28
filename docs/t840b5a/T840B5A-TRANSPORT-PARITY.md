# T-840B5A — Transport parity

50 shared Rust/TS scenarios: 45 cadence/rig cases plus five lifecycle/refusal vectors.
16,600 command observations and 28,850 positive executed-tick comparisons. The five
explicit vectors contain 130 operations. These counts are repeated measurements of
minimal rigs, not 50 independently qualified artistic characters.

Nine genuine B4 author rigs (idle, parent, loop, A/B/AB/ABC/CBA, angles) run through
author serialization/reopen and the unchanged default native export preset. The loop
fixture adds two real author clips: held (nonlooping) and with-event (intentionally
refused by B4). Runtime fixture bytes must match these exports; no patched native JSON.
The source author multiclip project is included separately; old fixtures are unchanged.

Each rig runs five seconds to tick 600 through exact 30/60/120/144Hz schedules or
0.5s hitches with pause/observation/resume and explicit drainage. Within each engine,
every executed-tick full pose/record is bit-identical across groupings. Signed half
turns, independent scalar axes, Bezier handles/overshoot-capable curves, stepped bounds,
parent rotation, nonuniform scale, long-bone tips and fractional 0.83s loops are reused.

Shared command vectors exercise creation, repeated play, restart, abrupt clip change,
pause/resume, preserved debt, exact equal/backward/invalid time, invalid budget, stop,
seek around loops, rejected implicit resume/rest-start, origin replay, interruption,
replay-zero completion, stale tokens, u64 epoch exhaustion, unsupported event clip,
crossfade/skins and terminal disposal. Rejected commands preserve complete snapshot
and current frame. All transport state/time/tick/order/counters compare exactly.

Every observed positive tick separately compares pure author bone locals/worlds,
spring offsets/velocities/anchors, and final simulated bone locals/worlds/tips. Every
command also compares current pure/simulated view plus the exact state snapshot.
No future simulated frame may have a tick different from clock.completed.

## Thresholds fixed before measurement

The threshold file is copied unchanged from B4 before measurements. Translation .01
units, angular distance .001 radians, affine coefficients .0001, scale/shear .0001,
angular velocity .001 rad/s, linear velocity .01 units/s, anchor .01 and tip .01 units.
Nonfinite values always fail. Angular distance is shortest-arc; affine/tip checks
independently detect geometric disagreement. No threshold was widened.

| Stage | Metric | Maximum error | Limit | Scenario | Tick/observation |
|---|---|---|---|---|---|
| pure | localTranslation | 2.74630039e-06 | 0.01 | idle/30 | 14 |
| pure | worldTranslation | 4.58300498e-05 | 0.01 | parent/30 | 91 |
| pure | rotation | 4.58159615e-07 | 0.001 | angles/30 | command-24 |
| pure | worldLinear | 1.37589118e-06 | 0.0001 | parent/30 | 29 |
| pure | scaleShear | 1.6072442e-07 | 0.0001 | loop/30 | command-38 |
| pure | tip | 0.000471268175 | 0.01 | parent/30 | 13 |
| spring | localTranslation | 7.3110072e-07 | 0.01 | angles/30 | 507 |
| spring | rotation | 7.3110072e-07 | 0.001 | angles/30 | 507 |
| spring | angularVelocity | 5.53261699e-06 | 0.001 | angles/30 | 479 |
| spring | linearVelocity | 5.74905397e-06 | 0.01 | angles/30 | 191 |
| spring | anchor | 1.02082412e-05 | 0.01 | parent/30 | 14 |
| post | localTranslation | 2.74630039e-06 | 0.01 | idle/30 | 14 |
| post | worldTranslation | 4.58300498e-05 | 0.01 | parent/30 | 91 |
| post | rotation | 7.89689917e-07 | 0.001 | angles/30 | command-127 |
| post | worldLinear | 1.58434728e-06 | 0.0001 | angles/30 | command-98 |
| post | scaleShear | 1.6072442e-07 | 0.0001 | loop/30 | command-38 |
| post | tip | 0.000470983296 | 0.01 | parent/30 | command-12 |

## Independent checks and historical evidence

Separate Rust and TS unit tests simulate 600 continuous ticks, rebuild from zero,
pause before the first replay tick and at tick 137, drain one tick at a time, and
compare the complete frame sequence. After completion an explicit resume advances
to 601 without reintegrating a previous tick. Isolation tests compare a second
instance with an untouched standalone owner while the first pauses, faults, resets
and disposes. TS also checks defensive frame/source copies and reentrant refusal.

Sampler overflow, Physics overflow and observer failure have language-specific
failure tests. They retain the last good frame/completed prefix and require reset.
Those overflow inputs are not a shared cross-language numerical-overflow contract;
f32 and double overflow domains differ. No B6 private-state rollback is claimed.

B3: all 90 cases / 9,810 frames rerun; raw TS traces and Rust reference files are
byte-identical to B4's accepted historical evidence. B4: all 45 cases / 21,645 frames
per stage rerun; full raw TS traces and 19 Rust reference files are byte-identical.
B2: all 461 shared clock operations remain in the full suites, vectors untouched.
Legacy: all 60 comparisons rerun, preserving 29 within / 31 divergent results, ten
Rust output files and three TS trace files byte-for-byte. No old oracle was replaced.

Raw vectors, cases, command snapshots, per-tick traces, first-failure fields and
maximum-error locations are in the isolated delivery (parity-final and reference-final).
Two B5A runs produce identical raw traces. GUI/WebGL: NOT_RUN. No sub-tick seek,
nonzero rest-start, skin switching, mixed constraints, external histories or graphical
consumer is qualified by this numerical campaign.
