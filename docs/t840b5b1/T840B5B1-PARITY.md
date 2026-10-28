# T-840B5B1 — Numerical qualification

PASS: 60 scenarios, 13210 command snapshots, 32400 positive completed tick comparisons.
Limits were copied byte-for-byte from B4/B5A before measurement; thresholds.json
and raw Rust/TS traces are supplied in the independent delivery folder.

Five genuine author fixtures: independent (A/C detachable, B always kept), dependent
(C child of A, author order A/B/C), zero-mix (A numerics active/output zero),
default-only (B belongs only to default; native special membership rule), control
(default contains all constraints, identical B5A/fixed-calendar baseline).
Their on/visual skins contain distinct native clipping polygons; no rendered
appearance or clipping validation is claimed. Bone membership references are real.
All are serialized/reopened without load losses and exported with the untouched
native preset. Runtime fixture bytes equal those exports; no runtime JSON patches.

Each fixture has visual-only and membership calendars (0 on,120 visual/off,240 on,
300 partial,330 on). Per-cadence trajectories at 30/60/120/144 Hz and half-second
hitches with pause/query/resume/52-tick drains are exactly identical within each
language. Ten additional lifecycle scenarios seek to 180 without Physics history,
replay 360 three times with pause before tick one, reset, reject stale context and
dispose. Integer clock counters, ordering, modes, selected skins and record
identities are exact. Pre/post locals, affine worlds, 300-unit tips, spring offsets,
velocities and anchors are compared separately. Every measured value must be finite.

| Stage | Maximum errors (observed) |
|---|---|
| pure | localTranslation=2.74630039e-06, worldTranslation=2.8834554e-05, rotation=3.46848513e-07, worldLinear=7.0133405e-07, scaleShear=1.44407152e-07, tip=0.000239425094 |
| spring | localTranslation=2.56039744e-07, rotation=8.68335901e-08, angularVelocity=8.95401029e-07, linearVelocity=3.27440405e-06, anchor=1.14828525e-05 |
| post | localTranslation=2.74630039e-06, worldTranslation=3.1253361e-05, rotation=3.46848513e-07, worldLinear=7.488281e-07, scaleShear=1.44407152e-07, tip=0.000261913762 |

Limits: locals/world translation .01; rotation .001 rad; affine coefficients .0001;
scale/shear .0001; angular velocity .001; linear velocity .01; anchor .01; tip .01.
Complete repeated Rust exports and TS traces match byte-for-byte. Additional
headless tests verify fresh reactivation against a separately constructed/primed
B3 owner at the exact current pose, unaffected independent B, zero-mix numerical
evolution, invalid calendars/references/rigs, observer fault, private input copies,
foreign/stale contexts and disposal. No past-edit API exists: all setSkin calls
refuse; a new calendar requires a new owner/history.

Actual historical reruns: B3 90 cases, B4 45 cases, B5A 50 cases. All native inputs
and raw per-tick TS traces equal the accepted B5A evidence. Legacy A: 29 within
thresholds, 31 historical divergences unchanged; not silently relabeled PASS.
B1/B2 and original Physics/T839 tests also execute in the full Rust/Runtime suites.
GUI, GPU/WebGL, PixiJS/Phaser visual qualification: NOT_RUN. Full workspace headless
tests are not a substitute for those campaigns.
