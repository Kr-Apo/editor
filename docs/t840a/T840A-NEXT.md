# T840A — Proposed T840B implementation sequence

Not authorization to implement. Stop after T840A review. No stage enables Physics
in PixiJS/CPU or changes runtime v1/v2, PathPosition, blending or release versions.

| Stage | Small change after authorization | RED before GREEN / measurable acceptance |
|---|---|---|
| B1 | context/instance safety | AppState replacement with reused loaded IDs must match a fresh first physical frame; old state absent. Rig replacement/destruction, concurrent shared-asset instances, rejected references and nonfinite deltas cannot mutate another instance/document. Preserve M001 save/recovery and Undo/Redo. No cadence migration yet. |
| B2 | explicit clock/profile skeleton | Preserve LegacyT839; opt-in tick/rational ReplayV1. Prime t=0 once; request/complete counters once per instance; eight-tick work budget, retained backlog, zero dt observational, explicit offline pump. RED counts for 30/60/120/144, tiny fractions, repeated .5s hitches, backwards/invalid/overflow input. No spring equation rewrite. |
| B3 | state/constraint scheduling boundary | Independent force-only springs must no longer affect each other's number of ticks. Test reversed/extra independent constraints, actual parent/child ordering, and no lost parent displacement on sub-tick frames. Keep old raw/profile behavior explicit. |
| B4 | canonical input sampling and precision | Sample animation/parent/fade/root/forces at every completed tick. Compare all local/angle/matrix/world metrics to new approved profile references, not changed old goldens. Same tick history at all rates; long-run and continuous loops. Qualify seconds→tick rounding separately before claiming game delta input independence. |
| B5 | transport and skin adapters | Real AppState + RigPlayer/RigPlayback tests for pause/resume/stop, restart/no-op play, pure seek vs historical replay, crossfade, appearance-only skin vs inactive membership and reactivation priming. Add a stateful skinned Rust route only after design approval. One transition command must not integrate by redraw. Do not fix blending/interrupted-fade semantics under this stage. |
| B6 | numeric safety and parameter changes | Invalid parameters rejected before mutation; injected numerical failure rolls back tick and returns last valid instance pose with explicit error. Test velocities/anchors/counters/other characters unchanged. Mix zero advances active state; inactive clears it. Benchmark transactional overhead when implemented. |
| B7 | replay/checkpoint qualification | Exact input/rig/profile revision validation, repeated replay from origin/checkpoint, backwards seek, external timestamped root motion, loop discontinuity and cancellation. Checkpoint includes all transport/clock/state/anchor data. No fake reconstruction from a frame-end transform. |
| B8 | future output integration review | Separate permission before lifting any CPU/adapter refusal. Fresh export controller, tick-aligned sampling/camera union, warmup policy, cancellation/staging/rollback, and actual poses/pixels/GPU qualification. No reuse of live viewport state. |

At each stage run targeted tests first, then the relevant final workspace/runtime
gates once. Keep the current 60 characterization rows visible. Rename a case to
conformance only when the approved profile and route are actually implemented;
never make a known mismatch green by raising tolerances or editing input assets.

Initial qualification limits: .01 local/world units, .001 radians (wrapped),
.0001 dimensionless affine coefficient. Add end-point tests for longer/scaled
rigs. Determinism means same inputs/ticks within each implementation and declared
cross-language error bounds; not unproved bitwise equivalence.

Recommended first authorization: B1 context/instance safety, then a narrowly
scoped B2/B3 clock prototype behind explicit profile selection. The experiment
shows that a small orchestration layer is promising, not that it is complete.
