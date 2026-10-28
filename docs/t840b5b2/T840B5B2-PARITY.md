# T-840B5B2 — Consumer parity

18 scenarios,3456 post-command snapshots,7044 positive per-tick audit comparisons.
Three genuine saved/reopened author projects, standard native preset exports:
rig-5,rig-32,rig-128, each with the same three Physics constraints and immutable
0:on/120:off/240:on calendar. Extra bones are real static siblings; no artificially
added solvers or patched native JSON. Identical vector JSON is committed in both
repositories and compared against exported trace commands. Native bytes match the
committed runtime fixtures. No graphic oracle is changed.

15 exact-cadence runs at 30/60/120/144 Hz and half-second hitches; three lifecycle
runs include capture/stale token, equal rational time, repeated observations,
pause/cancel-between-pumps, drain through tick120, pure seek180, replay360,
failed presentation, precommit audit failure, fault refusal/reset, invalid times/
budgets/horizon, maximum-horizon request (only eight executed ticks), stop/dispose.
Each cadence has the same complete per-tick trajectory within each language.
100 reads/presentations in observation commands do no work. Seek separates
committed0/on from pure180/off; no simulated skin is taken from the host target.

Published and diagnostic-candidate paths are compared separately. Metadata, exact
rational pose times, requested/completed/backlog, modes, health, targetReached,
logical epoch/tick identities, skin and record order are exact. Opaque tokens are
not serialized as global IDs: local tests verify actual identity/foreign rejection.
The precommit audit is explicitly NOT published/rendered. Its failure tests B3's
real fault path; it does not claim B6 rollback or inject future valid frames.

Thresholds copied unchanged from B5B1 before measurement: locals/world .01, rotation
.001 rad, affine coefficients .0001, scale/shear .0001, angular velocity .001,
linear velocity/anchors/tips .01; clip times match exactly. All values finite.

| Field/stage | Largest observed absolute error |
|---|---:|
| published/clipTime | 0 |
| published/localTranslation | 2.74630039e-06 |
| published/rotation | 3.46848513e-07 |
| published/scaleShear | 1.44407152e-07 |
| published/worldLinear | 7.24644393e-07 |
| published/worldTranslation | 2.38189378e-05 |
| published/angularVelocity | 6.28617583e-07 |
| published/linearVelocity | 1.46524732e-06 |
| published/anchor | 6.38225618e-06 |
| audit/clipTime | 0 |
| audit/localTranslation | 2.74630039e-06 |
| audit/rotation | 3.46848513e-07 |
| audit/scaleShear | 1.44407152e-07 |
| audit/worldLinear | 7.24644393e-07 |
| audit/worldTranslation | 2.38189378e-05 |
| audit/tip | 0.000244417167 |
| audit/angularVelocity | 6.28617583e-07 |
| audit/linearVelocity | 1.46524732e-06 |
| audit/anchor | 6.38225618e-06 |

Headless integration also checks detached snapshot mutation, presentation reentrancy,
multiple live consumers with shared input rig, preserved retained frames, invalid
initial clip (no initialized frame produced), and independent reset/destruction.
Historical comparisons are regenerated separately: B2 461 exact operations in full
suites, B3 90 cases, B4 45, B5A 50, B5B1 60. Their raw traces and native inputs must
remain byte-identical; preservation receipts accompany the report. Legacy retains
29 within thresholds /31 divergences. GPU/GUI/PixiJS/Phaser visual campaigns NOT_RUN.

After the full suite, coherence assertions were strengthened without production changes: each published snapshot must exactly match the last successful precommit audit frame (or an independent origin-zero B4 reference). This catches a future pose mislabeled with the correct completed tick, even if both bridges made the same error. The targeted Rust suite and18-case comparison passed again; raw repeat traces are unchanged.
