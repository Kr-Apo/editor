# T840A — Parity measurements

2026-10-23. Characterization of the accepted T839 implementations; no solver
change. A passing characterization test proves repeatability/observed behavior,
not conformance of the current product to the proposed ReplayV1 profile.

## Declared thresholds and method

Thresholds were written before measurements in Runtime
`scripts/t840a/thresholds.json` and copied to the initial evidence receipt:
.01 local translation units; .01 world translation units; .001 wrapped radians;
.0001 absolute dimensionless world a/b/c/d coefficient. Every category must pass.
Nonfinite values always fail; no null-to-zero conversion or post-hoc relaxation.
Local rotation error uses atan2(sin(delta),cos(delta)). Translation and matrix
coefficients are not presented as one interchangeable unit or pixel error.

Three original T839 author JSON files are copied byte-for-byte into isolated
`export/tests/fixtures/t840a/`; their hashes are recorded in FIXTURE-INPUTS.json.
Each is loaded by formats, converted natively and exported by the shipped preset.
The TS driver asserts the baseline native bytes match unchanged T839 fixtures.
Derived context fixtures are clearly separate genuine native exports: an identical
second clip and an empty inactive skin owning no constraints, with Physics owned
by default. They isolate state/activation policy, not artistic crossfade quality.
No manually patched native JSON or historical oracle is used.

Rust `export/tests/t840a_physics.rs` emits the explicit input plan, two native
variants and complete per-step local/world arrays. It runs each scenario twice
from fresh state and asserts exact within-language repeatability; TS independently
does the same. Each bone row stores x,y,radians,a,b,c,d,tx,ty. No render/GPU input.
The final dataset is reference-v3 / measurements-v3 in the external evidence
folder. Earlier runs are preserved and do not replace the final dataset.

Context transport driver conventions are explicit, not claimed common native API:
Rust uses the actual core, paused flag, pure seek/play/replacement preview, stored
state and explicitly supplied crossfade layers. TS uses its actual evaluator,
freeze-last-pose pause, reset-on-play and retained-state seek/crossfade policies.
Other clip is identical and changes state policy only. Rust has no stateful
active-skin argument, so the inactive scenario intentionally shows that API gap;
it does not emulate missing gating and pretend it exists. Actual AppState and
RigPlayer/RigPlayback tests separately verify the policies used by the drivers.

The long case is 10,000 calls at 60Hz (~166.67s); core timestamps run beyond the
two-second clip and hold its last keys. It is not a five-minute interactive game
or a looped-transport graphics campaign. Fractional deltas cycle .001/.002/.003/
.004; variable deltas cycle .003/.017/.011/.027/.008/.034. The hitch contains
zero dt before/after .5s, and repeated hitches are ten .5s calls. Context cases
have twenty moving steps, action, twenty continuation/paused steps, resume and
twenty further steps. Repeated reset has three equal reset+20-step histories.
Two-instance case interleaves another character's .5s steps and asserts this
character exactly matches its unperturbed 144Hz trace in each language.

**60 comparisons / 34,539 sampled poses**: 29 within all declared thresholds; **31 DIVERGENCE**. Zero nonfinite values in these valid-input traces. The mismatches are recorded, not counted as parity PASS.

| Fixture | Scenario | Local units max | Angle rad max | World coefficients max | World units max | Result |
|---|---|---:|---:|---:|---:|---|
| single | canonical-120 | 5.047937e-07 | 1.576116e-07 | 2.507449e-07 | 2.606241e-06 | WITHIN |
| single | rate-30 | 4.923696e-07 | 1.554946e-07 | 2.342086e-07 | 1.61787e-06 | WITHIN |
| single | rate-60 | 5.432379e-07 | 1.810746e-07 | 2.449719e-07 | 2.248853e-06 | WITHIN |
| single | rate-120 | 5.097694e-07 | 1.891401e-07 | 2.656048e-07 | 2.484207e-06 | WITHIN |
| single | rate-144 | 0.007077688 | 0.006018407 | 0.005984383 | 0.0527662 | DIVERGENCE |
| single | fractional | 0.007097644 | 0.005721165 | 0.005706513 | 0.05027371 | DIVERGENCE |
| single | variable | 9.536743e-07 | 1.763006e-07 | 2.778957e-07 | 4.520531e-06 | WITHIN |
| single | hitch-zero | 0.263697 | 0.2489626 | 0.246708 | 2.233918 | DIVERGENCE |
| single | repeated-hitches | 0.2467441 | 0.2467441 | 0.2435147 | 2.446009 | DIVERGENCE |
| single | long-10000 | 9.536743e-07 | 2.909038e-07 | 4.136272e-07 | 3.996586e-06 | WITHIN |
| single | mix-off | 0.05973163 | 0.04622794 | 0.04523742 | 0.3926412 | DIVERGENCE |
| single | inactive | 0.2888703 | 0.2358782 | 0.2336971 | 2.0481 | DIVERGENCE |
| single | pause | 5.432379e-07 | 1.810746e-07 | 2.449719e-07 | 2.248853e-06 | WITHIN |
| single | seek | 0.2225917 | 0.1838589 | 0.1828247 | 1.605656 | DIVERGENCE |
| single | play | 0.2370535 | 0.2318565 | 0.2287584 | 2.059631 | DIVERGENCE |
| single | crossfade | 1.931801e-06 | 1.825187e-07 | 2.584817e-07 | 2.985487e-06 | WITHIN |
| single | reset | 5.123101e-07 | 1.73391e-07 | 2.449719e-07 | 2.248853e-06 | WITHIN |
| single | replace | 0.2370535 | 0.2318565 | 0.2287584 | 2.059631 | DIVERGENCE |
| single | instances-interleaved | 0.007077688 | 0.006018407 | 0.005984383 | 0.0527662 | DIVERGENCE |
| single | repeated-reset | 4.730858e-07 | 1.257529e-07 | 2.221858e-07 | 1.666708e-06 | WITHIN |
| multiple | canonical-120 | 5.535959e-07 | 2.017805e-07 | 3.175888e-07 | 2.436195e-06 | WITHIN |
| multiple | rate-30 | 5.104504e-07 | 1.772153e-07 | 2.886396e-07 | 2.189071e-06 | WITHIN |
| multiple | rate-60 | 5.432379e-07 | 2.127942e-07 | 3.264105e-07 | 2.351515e-06 | WITHIN |
| multiple | rate-120 | 5.260118e-07 | 1.891401e-07 | 3.540449e-07 | 3.523553e-06 | WITHIN |
| multiple | rate-144 | 0.04778906 | 0.03520414 | 0.03162883 | 0.314059 | DIVERGENCE |
| multiple | fractional | 0.01574712 | 0.01562476 | 0.01264107 | 0.07516583 | DIVERGENCE |
| multiple | variable | 0.006980942 | 0.006673295 | 0.006655298 | 0.05995977 | DIVERGENCE |
| multiple | hitch-zero | 0.3012192 | 0.2489626 | 0.4282838 | 2.028194 | DIVERGENCE |
| multiple | repeated-hitches | 0.2593858 | 0.2467441 | 0.4085222 | 2.454956 | DIVERGENCE |
| multiple | long-10000 | 9.536743e-07 | 2.909038e-07 | 4.439567e-07 | 4.505423e-06 | WITHIN |
| multiple | mix-off | 0.05973163 | 0.05486551 | 0.09311313 | 0.3604011 | DIVERGENCE |
| multiple | inactive | 0.3238688 | 0.2358782 | 0.4079402 | 1.778795 | DIVERGENCE |
| multiple | pause | 5.432379e-07 | 2.127942e-07 | 3.120917e-07 | 2.216917e-06 | WITHIN |
| multiple | seek | 0.2325579 | 0.1838589 | 0.3414423 | 1.407109 | DIVERGENCE |
| multiple | play | 0.2590835 | 0.2318565 | 0.41062 | 1.872228 | DIVERGENCE |
| multiple | crossfade | 1.931801e-06 | 2.128021e-07 | 3.120917e-07 | 3.003924e-06 | WITHIN |
| multiple | reset | 5.123101e-07 | 1.73391e-07 | 3.120917e-07 | 2.33851e-06 | WITHIN |
| multiple | replace | 0.2590835 | 0.2318565 | 0.41062 | 1.872228 | DIVERGENCE |
| multiple | instances-interleaved | 0.04778906 | 0.03520414 | 0.03162883 | 0.314059 | DIVERGENCE |
| multiple | repeated-reset | 5.018378e-07 | 1.548046e-07 | 2.721141e-07 | 1.862622e-06 | WITHIN |
| parent | canonical-120 | 4.967623e-07 | 1.514052e-07 | 3.223999e-07 | 4.191733e-06 | WITHIN |
| parent | rate-30 | 4.824727e-07 | 1.364364e-07 | 2.965439e-07 | 3.465606e-06 | WITHIN |
| parent | rate-60 | 4.884296e-07 | 1.483368e-07 | 3.06637e-07 | 4.159428e-06 | WITHIN |
| parent | rate-120 | 5.0363e-07 | 1.433981e-07 | 3.311827e-07 | 4.639283e-06 | WITHIN |
| parent | rate-144 | 0.01218488 | 0.001115451 | 0.00144038 | 0.007077487 | DIVERGENCE |
| parent | fractional | 0.01186976 | 0.001960693 | 0.002498485 | 0.01183021 | DIVERGENCE |
| parent | variable | 9.536743e-07 | 1.532147e-07 | 3.155847e-07 | 5.004134e-06 | WITHIN |
| parent | hitch-zero | 0.4704662 | 0.06964641 | 0.09016291 | 0.2636974 | DIVERGENCE |
| parent | repeated-hitches | 0.382968 | 0.1460786 | 0.1895784 | 0.9585378 | DIVERGENCE |
| parent | long-10000 | 9.536743e-07 | 1.483368e-07 | 3.280709e-07 | 6.431983e-06 | WITHIN |
| parent | mix-off | 0.09979075 | 0.006779278 | 0.008711388 | 0.05973213 | DIVERGENCE |
| parent | inactive | 0.4908108 | 0.03991222 | 0.05153525 | 0.2888703 | DIVERGENCE |
| parent | pause | 4.884296e-07 | 1.483368e-07 | 3.06637e-07 | 3.261862e-06 | WITHIN |
| parent | seek | 0.3795993 | 0.03345462 | 0.04267785 | 0.2225917 | DIVERGENCE |
| parent | play | 0.4244562 | 0.05999409 | 0.07698379 | 0.2370534 | DIVERGENCE |
| parent | crossfade | 1.931801e-06 | 1.546317e-07 | 3.06637e-07 | 3.54759e-06 | WITHIN |
| parent | reset | 5.060401e-07 | 1.412619e-07 | 3.06637e-07 | 4.285653e-06 | WITHIN |
| parent | replace | 0.4244562 | 0.05999409 | 0.07698379 | 0.2370534 | DIVERGENCE |
| parent | instances-interleaved | 0.01218488 | 0.001115451 | 0.00144038 | 0.007077487 | DIVERGENCE |
| parent | repeated-reset | 4.70493e-07 | 1.263564e-07 | 2.849532e-07 | 4.191764e-06 | WITHIN |

## Diagnoses, strength of evidence

- Caps 8/32 and current zero-dt backlog consumption: code plus TS state-record
  assertions and hitch traces. Both retain backlog; no claim Rust drops it.
- Shared Rust remainder: direct force-only probe removes parent motion and all
  hierarchy effects. At 144 calls, independent states both give rotation/x
  .2465458065. Shared state produces .2423238158 and .2684547305. This is a
  demonstrated clock-ownership weakness, not a desired Rust reference rule.
- High-rate/fractional differences: f32/double floor/remainder boundaries and
  frame-end anchor sampling are probable causes from the code and traces. Their
  individual contributions are not claimed independently isolated. A common
  tick experiment avoids both; it does not prove one constant change fixes all.
- Mix zero: actual Rust advances an active simulation then suppresses output;
  TS skips it and leaves old anchors. Inactive skin: TS gates constraints while
  Rust stateful public entry points lack an active-skins parameter.
- Real AppState tests prove seek/pause displays pure rest while stored state
  persists; clip selection also keeps state. Replacing a document with the same
  loaded IDs leaks that state and yields a different first simulated pose than a
  fresh AppState. This is newly reproduced P1 context safety, not repaired here.
- Actual RigPlayer seek retains offsets, play clears, crossfade preserves. Actual
  RigPlayback pauses with last pose, resumes, stops to setup and rejects updates
  after destroy. The proposed stop/seek policy intentionally differs and is opt-in.
- Deliberate in-memory nonfinite force injection: current Rust reset guard clears
  offsets; current direct TS evaluation produces nonfinite pose. High-level TS
  update rejects NaN delta before advancing. A separate healthy instance remains
  unchanged. Fault injection is not presented as a valid exported fixture.
- Three reset histories and separate instances are repeatable. That proves
  isolation under the stated valid input sequence, not universal lifecycle safety.

## Test-only common clock experiment

The test model calls the existing kernels, primes at zero and samples each integer
N/120 tick. At 30/60/120/144 display rates it creates the same 120-tick TS history,
and compares to the native canonical-120 reference at every tick. Twelve rate/rig
comparisons meet all thresholds. Max errors across them: local units .000000554,
angle .000000202 rad, coefficients .000000323, world units .000004192.
For each rig, a .5s request processes eight ticks, leaves 52 pending, does not
process on zero dt, then explicit drain exactly matches the continuous 60-tick
prefix. Three hitch schedule checks pass. Pure canonical-nanosecond vectors
also test boundaries/four schedules; no production seconds bridge is qualified.

This is evidence for option C's orchestration idea, not a shipping replay adapter
or a repair of raw kernels. It covers one second and simple input history, not
full long-run transport/skin/fault behavior. Existing interactive renderers and
all Physics/Pixi/CPU refusals remain unchanged. No new WebGL/GUI/manual/clean-PC
results; T839's 125 graphics scenarios remain historical here.

## Reproduction

From the Editor root, set T840A_REFERENCE_DIR to a nonexistent directory and
T840A_COUPLING_FILE to a new file, then run:

```
cargo test --locked --offline -p ankhimate-export --test t840a_physics
cargo test --locked --offline -p ankhimate-editor t840a_
```

Unset output variables before workspace tests. In Runtime after build:

```
node scripts/t840a/compare.mjs <reference-dir> <new-results-dir>
node scripts/t840a/clock-experiment.mjs <reference-dir> <new-clock-result.json>
node --test scripts/t840a/characterization.test.mjs
```

Emission refuses an existing destination; it does not overwrite evidence. Full
trace arrays, maxima, first divergent component, thresholds and both exports are
retained. The comparison exits successfully when characterization completes;
**its result rows, not its exit code, determine parity**. A corrupted/missing
fixture, changed source bytes or nondeterminism throws. Known differences are not
added as ignored failing tests to the main suite, and no assertion is weakened.
