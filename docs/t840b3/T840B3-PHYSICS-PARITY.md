# T840B3 — Per-tick Physics parity

## Fixtures and inputs

The campaign uses actual Rust exports, without editing their runtime JSON:

- Historical T839/T840A `single`, `parent`, `multiple`: their native bytes still
  match the accepted Runtime fixtures exactly.
- New author fixture `export/tests/fixtures/t840b3/independent.ankh.json`:
  three sibling bones A/B/C under a stationary root, each with the established
  force-only spring parameters. Author-side subsets/orders A, B, AB, BA, ABC are
  exported through the existing native preset.
- `dependent-reversed`: an author-side copy of multiple with reversed constraint
  order, exported normally. This is a real dependency/order probe, not an
  independence golden.

The nine native outputs are copied verbatim into Runtime's new
`core/test/fixtures/t840b3/`. Historical files are untouched.

Each rig runs static and explicitly varying root-local inputs. Motion inputs
start at (50, −20) and use bounded exact per-tick position changes plus small
rotation changes. The historical parent fixture has rotation and nonuniform
scale. Rust emits its exact f32 local inputs for every tick; the comparison
driver supplies those same values directly to TypeScript. No clip/fade/root
history sampler or repeated display-frame input substitutes for these values.

## Predeclared thresholds

`scripts/t840b3/thresholds.json` was created before the numerical campaign and
was not widened after measurements. Both independent and dependent cases use
the same thresholds:

| Quantity | Maximum allowed | Maximum observed |
|---|---:|---:|
| Local translation / positional offset | 0.01 unit | 0.000000994014 |
| World translation | 0.01 unit | 0.000007487131 |
| Local rotation / rotational offset | 0.001 rad | 0.000000246578 |
| World affine linear coefficient | 0.0001 absolute | 0.000000404154 |
| Angular velocity (additional record check) | 0.001 rad/s | 0.000000732289 |
| Linear velocity (additional record check) | 0.01 unit/s | 0.000000937320 |
| Parent anchor (additional record check) | 0.01 unit | 0.000002771999 |

Nonfinite values always fail. Tick indices, order, completed/requested counters,
budgets consumed and backlog compare exactly, with no numeric tolerance.

## Executed comparison

**90 scenarios pass, 9,810 frame comparisons**, comprising 90 primed tick-zero
frames and 9,720 positive-tick frames. Every intermediate tick is compared:
all local/world bone transforms, offsets, velocities and anchors, with matching
constraint identity and author order. Results are in the isolated delivery's
`reference-v3/` and `measurements-final/` directories.

For each of nine rigs and two input schedules, the campaign runs 30/60/120/144 Hz
exact rational requests and a half-second hitch/drain schedule. Within each
language all cadence groupings have identical per-tick trajectories. Hitch
execution commits eight then leaves 52; equal time does nothing; pump 7 then
45 matches the continuous first 60 ticks. The published frame always identifies
the last completed tick, including while backlog remains.

The decisive sibling test separately checks A and B in isolation, together,
reversed and with C added. Their records, local poses and world transforms
remain **exactly equal within each language** at every tick. In the dependent
parent/child fixture, reversing author order changes results as expected;
cross-language parity still meets the unchanged thresholds in both orders.

## RED/GREEN and Legacy evidence

Before product changes, the desired-independence Rust test failed on the actual
Legacy integrator. At 144 calls/second, shared rotation/x offsets were
0.24232382 and 0.26845473, versus isolated 0.2465458 for each spring. The TS RED
was an absent production-controller export, not an executed assertion failure.
Logs and exact RED sources are retained.

The final tests apply the independence contract to ReplayV1. Legacy continues
to reproduce its known coupling, under the unchanged T840A characterization.
Ten freshly generated Rust reference/plan/native files are byte-identical before
and after equation extraction, as is the shared-remainder coupling result.
Fresh TypeScript traces are byte-identical to the accepted T840A traces (B2's
Legacy sources are verified identical to that source baseline).

The freshly rerun 60 Legacy comparisons still yield **29 within thresholds and
31 divergences**. No old trace, threshold or golden was replaced. Those results
are not failures of the new isolated ReplayV1 campaign, nor corrected Legacy
trajectories.

Initial nonzero-anchor tests also analytically verify exactly one elementary
step and zero steps during priming, fixed default-skin filtering and zero mix.
Failure tests verify completed prefixes and fault/reset protection, not B6
rollback. No graphical suite is part of these results: **GUI/WebGL NOT_RUN**.
