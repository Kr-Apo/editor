# T839 — Recommended short next missions

Recommendations require separate authorization; none is started here.

| Order | Mission | Dependencies and measurable acceptance | Format impact |
|---|---|---|---|
| 1 | Physics replay contract | Decide initialization/reset/pause/seek/clip, cap/backlog and accumulator ownership. Reuse genuine T839 fixtures at 60/144Hz, hitch/zero dt; add 30/120Hz, long-run, off/on, multi-instance and constraint order. Require <=.01 unit geometry under declared cadence, explicit angular tolerance, finite state and reset determinism. | Prefer state/consumer changes only; new tuning needs explicit contract. |
| 2 | Bounded Physics synchronization | After #1, fix demonstrated differences one at a time with independent Rust/TS red tests. Preserve pure evaluate, caller-owned simulation, current Pixi gate and rendered refusal until qualified. | No migration expected for existing target reference. |
| 3 | Discrete mixing fidelity | Decide winning draw-order semantics; reproduce c,b,a versus c,a,b and repair in a targeted checkpoint. Cover attachment/visibility ties, two keyed clips, incoming-only events and interrupted fade policy. | No wire change expected; behavior requires review. |
| 4 | Static path parity | Exact corners, endpoints, arc-length/index, default skin, host/parent transforms and constraint order. Authentic references; .01 units plus explicit angles. | No new track; keep animated export refused. |
| 5 | Guarded PathPosition consumer | After #4 and version decision, real export/read parity, old-reader rejection, reset/interpolation tests and per-adapter qualification. | Candidate v3 or genuinely enforced required capability; additive v2 insufficient. |
| 6 | Bounded pose/facial controls | Specify authored parameter→transform/FFD mapping, masks, ownership, ranges and native Undo/Redo. Reuse existing editor/evaluator; Save/Reload and safe export refusal until consumer exists. | New author/runtime data likely; separate migration review. |
| 7 | Animation Layers | After discrete policy, define additive/override, masks, scale/color/FFD/discrete/events/interruption rules. A two-track fade is not a general layer system. | Likely new authored metadata/guarded runtime contract. |
| 8 | Simulated rendered exports | After deterministic replay, specify warmup/start/reset, fixed-step sampling, camera union, cancellation and staging. Compare poses then pixels before relaxing physics refusal. | Separate rendered metadata if needed; retain old pixels/safety. |

Immediate recommendation: **specify and regression-test the Physics replay
contract**, then authorize a bounded synchronization fix. Changing MAX_STEPS
alone conceals accumulator and transport differences.

Ordinary PSD→rig→animate→native/image export remains available within documented
limits. Advanced physics/paths/layers can remain explicitly excluded. Preserve
T833/834/835 rather than recreate them. No AI, packaging, release or broad refactor
belongs in these short fixes. Any future candidate needs its own manual/clean-PC
qualification and explicit publication permission.
