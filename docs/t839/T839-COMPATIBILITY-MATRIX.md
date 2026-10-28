# T839 — Capability and evidence matrix

2026-10-23. IMPLEMENTED: code for stated scope; PARTIAL: subset or known divergence;
UNSUPPORTED: absent or deliberately refused; UNQUALIFIED: fidelity not established.
Code presence and format support do not certify graphics.

| Feature | Editor | Rust core | TS core | PixiJS 8 | Phaser 3 | CPU sequence/sheet |
|---|---|---|---|---|---|---|
| independent bone axes v2 | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED |
| legacy v1 paired reader | PARTIAL, separate author format | UNSUPPORTED as wire reader | IMPLEMENTED | UNSUPPORTED by adapter | PARTIAL | UNSUPPORTED as wire reader |
| weighted meshes / FFD | IMPLEMENTED, bounded tools | IMPLEMENTED | PARTIAL | PARTIAL | PARTIAL | PARTIAL |
| skins / attachments | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED | PARTIAL | PARTIAL | PARTIAL, default only |
| light RGBA / static dark tint | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED | PARTIAL | PARTIAL | PARTIAL |
| clipping / Normal composition | IMPLEMENTED | PARTIAL, pose/polygons | PARTIAL | PARTIAL | PARTIAL | PARTIAL |
| Add/Multiply/Screen | IMPLEMENTED | PARTIAL via renderer | PARTIAL metadata | UNSUPPORTED in measured subset | UNQUALIFIED | UNSUPPORTED by transparent export |
| IK / Transform constraints | IMPLEMENTED | IMPLEMENTED | PARTIAL | UNSUPPORTED by gate | UNQUALIFIED legacy path | PARTIAL |
| Physics native bone reference | IMPLEMENTED | IMPLEMENTED | IMPLEMENTED after T839 | UNSUPPORTED by gate | PARTIAL through shared player | UNSUPPORTED |
| Physics trajectory/transport parity | UNQUALIFIED cross-target | IMPLEMENTED with limits | PARTIAL, divergence | UNSUPPORTED | UNQUALIFIED | UNSUPPORTED |
| static path | IMPLEMENTED polyline | IMPLEMENTED | PARTIAL | UNSUPPORTED | UNQUALIFIED | PARTIAL |
| animated PathPosition | IMPLEMENTED | IMPLEMENTED | UNSUPPORTED | UNSUPPORTED | UNSUPPORTED | IMPLEMENTED under render restrictions |
| animated PathSpacing | UNSUPPORTED | UNSUPPORTED timeline | UNSUPPORTED | UNSUPPORTED | UNSUPPORTED | UNSUPPORTED |
| weighted transform mixing | PARTIAL, no general layer UI | IMPLEMENTED | IMPLEMENTED | PARTIAL playback | PARTIAL playback | PARTIAL, single clip request |
| arbitrary FFD/color layer stacks | PARTIAL | IMPLEMENTED sequential rules | PARTIAL | UNQUALIFIED | UNQUALIFIED | UNSUPPORTED as stack |
| attachment/visibility winner | PARTIAL | IMPLEMENTED | IMPLEMENTED | PARTIAL | PARTIAL | PARTIAL |
| two keyed draw orders mixed | PARTIAL | PARTIAL | PARTIAL | UNQUALIFIED | UNQUALIFIED | UNSUPPORTED as stack |
| interrupted crossfade | UNSUPPORTED general transport | UNSUPPORTED transport object | PARTIAL, possible jump | PARTIAL | PARTIAL | UNSUPPORTED |
| events | IMPLEMENTED | IMPLEMENTED windows | IMPLEMENTED | IMPLEMENTED scoped playback | IMPLEMENTED scoped playback | PARTIAL metadata |
| dedicated facial controls/sliders/layers | UNSUPPORTED dedicated system | PARTIAL reusable primitives | PARTIAL reusable evaluation | UNSUPPORTED dedicated system | UNSUPPORTED dedicated system | UNSUPPORTED dedicated system |
| safe save/recovery/publication | IMPLEMENTED scoped guarantees | UNSUPPORTED I/O by design | UNSUPPORTED editor lifecycle | UNSUPPORTED editor lifecycle | UNSUPPORTED editor lifecycle | IMPLEMENTED via existing publisher |
| image/sheet/video export | IMPLEMENTED scoped exporters | IMPLEMENTED evaluation primitive | UNSUPPORTED editor exporter | UNSUPPORTED editor exporter | UNSUPPORTED editor exporter | PARTIAL |

Evidence/boundaries:

- Live native transform parity: 128 cases/3328 components, <.01 error; v1/v2,
  dark-tint and FFD timing regressions actually rerun.
- Existing WebGL125 rerun on this machine: weighted/FFD, tints, selected masks,
  skins, pages, transforms and short fades. Arbitrary linked/shared migrations,
  compound masks/blends and broad hardware remain unqualified.
- Physics target activation: authentic native fixture + legacy reader tests.
  Twelve reset replay sequences show high-rate/hitch divergence. No physics/path
  fixture is in the WebGL125 campaign; PixiJS still refuses all constraints.
- Static paths: existing numeric test and inspection; exact-corner direction and
  alternate-skin resolution differ. Animated export refusal remains intact.
- Mixing: real native draw-order probe gives Rust c,b,a versus TS c,a,b. Other
  rules are inspected and supported by existing scoped tests, not newly exhaustive
  cross-target layer conformance. Colors/FFD sequential; no animated dark tint.
- Rendered output: default skin, Normal only, no physics; separate FFmpeg for
  MP4/GIF. Historical image/rollback suites rerun; formats unchanged.
- No new physical Windows, clean-PC, artistic or real AI-provider qualification.

Code: `core/src/{animation,pose,constraints,physics,path}.rs`,
`formats/src/{schema,convert}.rs`, `export/src/{context,preflight,run}.rs`,
`render/src/sequence.rs`, `editor/src/app_state.rs`; Runtime
`core/src/{types,rig,evaluate,constraints,player}.ts`, `core/src/render/playback.ts`,
`pixi/src/validate.ts`, `phaser/src/{phaser,validate}.ts`.

Tests: Editor `export/tests/{t839_physics,t839_mixing,path_position,preflight}.rs`,
core/formats/document PathPosition suites and core physics/pose unit tests;
Runtime `core/test/{physics-target,export-parity,dark-tint,ffd-timing,
playback-boundaries}.test.ts`, adapter unit suites and
`scripts/test-parity-headless.mjs`. See [actual results](T839-REPORT.md).
