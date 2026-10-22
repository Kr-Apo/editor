---
title: Rigging capabilities and qualification
description: Existing Rust authoring tools, exports and alpha runtime boundaries, without conflating availability with visual qualification.
---

# Rigging capabilities and qualification

Ankhimate — Kr-Apo Community Fork (Unofficial) builds on the original
[Ankhimate editor](https://github.com/Ankhimate/editor) and
[runtime](https://github.com/Ankhimate/runtime), under the existing licenses and
credits. No endorsement or comparative parity claim is implied.

| Capability | Rust core / editor | Native v2 | Game adapters / evidence boundary |
|---|---|---|---|
| Bone hierarchy, FK | Local transforms, derived worlds; Setup / Animate tools | Bones and scalar tracks | Qualified subset at the protected alpha checkpoint; no new GPU run in T-816 |
| IK | Aim (1), analytic (2), FABRIK (3+); native target creation, mix/bend/softness/stretch and 3+ stiffness | Constraint data can be serialized; not baked animation | IK excluded in alpha PixiJS/Phaser; exported data is not runtime support |
| Weighted meshes | Per-vertex influences and inverse binds; existing brush and native weight commands | Mesh geometry and weights | Previously qualified subset, subject to adapter limits; no new visual claim |
| Weight brush | Weights dock: Add/Subtract/Replace/Smooth, Direct input, radius/feather, session locks | Resulting weights, not brush/session state | Authoring operation, not game-side painting |
| Skins / attachments | Named skin entries and default fallback, native assignments | Named data | Adapter-specific support and rejection policy; no promise of arbitrary multi-skin composition |
| Keys / curves | Scalar axes, Linear/Stepped/Bézier, graph editor and native history | Independent scalar tracks | Existing contracts and accepted parity evidence; T-814 manual review pending |
| FFD | Native deformation tracks over existing topology | Deformations | Qualified weighted/deform subset from historical work; do not infer linked-mesh support |
| Physics / paths / advanced constraints | Existing Rust capabilities | Data may be representable | Not certified by the alpha adapters; preflight warnings/refusals remain authoritative |
| CPU rendered exports | Existing PNG/JPEG/WebP sequences, PNG spritesheets and external FFmpeg workflows | Separate rendered output contracts | Default skin, Normal blending and physics exclusions retained; no new rendered-output qualification |
| Optional AI assistant | Bounded reads and approved native transactions, model/provider dependent | Not required to rig/export | No inference, vision, artistic or GPU qualification in T-816 |

The public **v0.2.0-alpha.1** and the local T-813–T-816 development candidates are
different checkpoints. T-816 neither changes that release nor packages runtimes.
Consult [known alpha limitations](KNOWN_LIMITATIONS.md),
[format specification](format-spec.md) and the external
[runtime documentation](editor-runtime-compatibility.md).
The older [RC2 compatibility guide](alpha/RUNTIME_COMPATIBILITY.md) is explicitly
an RC2 contract, not a new certification of T-816.

## Start with existing examples

Follow [First rig in ten minutes](first-rig-in-10-minutes.md). The unchanged
[`tentacle.ankh`](../samples/tentacle.ankh) demonstrates an eight-bone IK chain and
`curl` clip without external art. It is tested through the current loader,
Save/Load and numerical evaluation. The original procedural generator remains
in `formats/examples/gen_tentacle.rs`; running generators is unnecessary.
Use a disposable copy of the approved Jade Sentinel for weight painting.
Jade remains the sole official alpha sample; Tentacle is an existing source-tree
rigging example, not a newly qualified game-runtime sample.

## Evidence

T-816 adds headless egui discovery/selection tests and native numerical/sample
checks. It does not change solver, brush arithmetic, normalization, binds,
project schema, export templates or runtimes. The small weight-specific selection
fix retains the selected mesh when re-aiming its influence; ordinary bone picking
elsewhere keeps its existing exclusive behavior.

Desktop usability, theme contrast, real input, artistic results and the grouped
T-813–T-816 campaign remain **PENDING MANUAL REVIEW**. Headless tests do not close
T-801/T-908 acceptance requiring a new user's actual successful exercise.
