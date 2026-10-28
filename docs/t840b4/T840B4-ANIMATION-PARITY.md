# T-840B4 — Animation parity

## Evidence and scope

45 scenarios, 21,645 frame comparisons **per stage** (including 45 primed frames),
21,600 positive simulated ticks. These are comparisons across two engines, not
21,645 independent artistic rigs. Nine minimal author projects: idle, parent, loop,
A, B, AB, ABC, CBA, angles. Each is serialized/reopened as an author project and
exported through the real default native preset. Runtime fixture bytes must equal
these actual exports. No native JSON repair or format change.

Idle animates the torso while root stays fixed; parent includes animated root,
rotated torso and nonuniform parent scales; loop uses a non-tick-multiple 0.83 s
duration over four seconds; A/B/AB/ABC/CBA compare isolated and combined springs in
author/reversed order. Angles exercises signed half-turns -180/+180 degrees. Bones
are 300 units long; the parent scenario includes root scale (3,1.5) and torso scale
(1.75,0.6). All seven channels, asynchronous axis key times, linear/stepped/Bezier,
overshoot-capable handles and Setup retention are present.

Each rig runs 30/60/120/144 Hz exact rational requests plus 0.5 s hitches, an equal
time request that must not drain debt, and explicit pumps. Frames within each engine
are bit-identical across these groupings. Reset/repeat, instance isolation, unsupported
channels and numerical/context failure behavior have targeted tests.

Every tick compares pure local/world transforms first, then spring offsets, velocities
and anchors, then simulated local/world transforms and tips. Clock counters, backlog,
processed counts, identity order and tick numbers compare exactly. Angular distance
uses shortest-arc radians; world matrices and tips additionally prevent orientation
equivalences from hiding geometric errors. Scale/shear use absolute 0.0001.

## Fixed thresholds and maxima

Thresholds were committed to the experiment configuration before measurements and
were never widened. The tip limit is 0.01 units. Rows retain the first divergent tick
and both values if any threshold fails; this campaign has no failing row.

| Stage | Metric | Maximum error | Limit | Scenario | Tick |
|---|---|---|---|---|---|
| pure | localTranslation | 2.74630039e-06 | 0.01 | idle/30 | 14 |
| pure | worldTranslation | 4.58300498e-05 | 0.01 | parent/30 | 91 |
| pure | rotation | 4.58159615e-07 | 0.001 | angles/30 | 96 |
| pure | worldLinear | 1.37589118e-06 | 0.0001 | parent/30 | 29 |
| pure | scaleShear | 1.6072442e-07 | 0.0001 | loop/30 | 152 |
| pure | tip | 0.000471268175 | 0.01 | parent/30 | 13 |
| spring | localTranslation | 7.14244273e-07 | 0.01 | angles/30 | 444 |
| spring | rotation | 7.14244273e-07 | 0.001 | angles/30 | 444 |
| spring | angularVelocity | 5.53261699e-06 | 0.001 | angles/30 | 479 |
| spring | linearVelocity | 5.74905397e-06 | 0.01 | angles/30 | 191 |
| spring | anchor | 1.02082412e-05 | 0.01 | parent/30 | 14 |
| post | localTranslation | 2.74630039e-06 | 0.01 | idle/30 | 14 |
| post | worldTranslation | 4.58300498e-05 | 0.01 | parent/30 | 91 |
| post | rotation | 7.82444179e-07 | 0.001 | angles/30 | 449 |
| post | worldLinear | 1.58434728e-06 | 0.0001 | angles/30 | 392 |
| post | scaleShear | 1.6072442e-07 | 0.0001 | loop/30 | 152 |
| post | tip | 0.000470983296 | 0.01 | parent/30 | 48 |

Raw evidence: reference-final/cases.json, parity-final/results.json and
parity-final/typescript-traces.json in the isolated delivery. Each raw frame contains
local x/y/rotation/scales/shears, affine coefficients/translation and world tip;
spring records contain offset, velocities and parent anchor. Results identify the
case, tick, subject and component of every maximum.

## Precision findings

The first fractional-duration probe rejected the serialized decimal 0.83 as though
it had to be exactly representable as JS number=f32. The source export was valid:
duration uses a shortest f32 decimal, while key times contain promoted values.
The wrapper now recovers f32 duration before f64 modulo. This is confined to the new
opt-in time conversion, and neither evaluator's interpolation algorithm changed.
Rust still uses its native Newton/bisection Bezier solver and f32 arithmetic;
TypeScript still uses its 20-step bisection and double arithmetic. No blanket f32
rounding of transforms, solver outputs or spring state was introduced.

## Historical preservation

B3: 90 scenarios / 9,810 frames rerun, all within the original limits; actual raw TS
traces exactly match accepted B3. B2's 461 shared clock operations run in the complete
suites; clock sources and vectors remain untouched. Legacy: all ten Rust output files
and three raw TS trace files match B3 byte-for-byte; 29 within / 31 divergent rows are
preserved on 60 comparisons. Those known Legacy differences are not claimed fixed.

The initial 40-case B4 campaign passed before optimization. Its TS traces are exactly
identical after the local-sampling extraction. The final 45-case campaign adds signed
half-turn cases. In each engine, all nine rigs also compare every wrapper frame
against B3 fed from the complete pure evaluator. This independently checks that
the optimized provider supplies exactly the locals exposed by the pure query. The extracted TS bone sampling loop is textually identical after
whitespace normalization (EXTRACTION.json).

GUI and WebGL: NOT_RUN. Unit/numerical tests do not qualify new PixiJS/Phaser rendering,
arbitrary production rigs, external inputs, mixed constraints or transports.
