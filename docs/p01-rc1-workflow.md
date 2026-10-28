# Classic P01-RC1: corrective workflow and compatibility

Local candidate based on A10 `cca30f97c2dd28b49da1d42a6c2cb31a779666ac`.
Public v0.1.5, frozen Alpha RC2 and this candidate are separate artifacts.
v0.1.6 is proposed; this document does not announce a tag or Windows release.
This is the unofficial Kr-Apo community fork; original licenses and credits remain.

## Rigging, meshes and weights

Create/select bones in Setup, import images through Assets/File, associate slots
and attachments, convert a region to a mesh and review Trace/Refine before applying.
Review holes and limb boundaries: a mathematical trace is not artistic acceptance.

Auto Weight measures distance along mesh edges. It does not jump automatically
between disconnected islands. If a selected vertex is unreachable from every
selected bone seed, the entire Auto operation is refused before dispatch, with
skin/slot/mesh and vertex indices. Existing weights, binds, document and Redo are
preserved. Paint the island with an explicitly chosen influence or repair its
topology. Connected selections still use one undoable native command.

Native-export preflight identifies every faulty weight row: vertex index,
stored influence names/coefficients, sum, reason and corrective instruction.
A rigid mesh without a weight table is distinct from a weighted mesh with empty
rows. Diagnostics do not weaken the safe publisher.

A historical “Degenerate mesh triangle” failure was not reproduced in 216
configurations of the current Frog Knight copy. Degenerate topology is still
refused. The error now includes triangle/index/Setup points/area. This is a
diagnostic improvement, not a demonstrated triangulator fix or universal Trace
qualification.

## Animated FFD

1. Select the displayed mesh attachment and a clip; switch to Animate.
2. Enable **Edit vertices** in Properties > Mesh. Dragging now keys FFD. Insertion,
   deletion, UV changes and retriangulation remain Setup-only.
3. Use **Key setup FFD** at the start for a complete zero-offset key. At the impact
   time, select/drag vertices. **Key FFD pose** explicitly records or replaces the
   full evaluated pose at the playhead.
4. Record an explicit rest return. One held nonzero key affects the whole clip.
   A full zero key is not an empty vector and does not modify the rig Setup pose.
5. The existing Dopesheet Deform row supports owned-key timing, interpolation,
   delete and bounded clipboard operations. Bone Graph Editor curves/tangents do
   not become a general FFD component curve editor.
6. Undo/Redo, save, close, reopen and export the isolated copy. Check interpolation
   and ownership after reopening.

The release evidence distinguishes physical-user assistance from controlled egui
tests. Native API tests alone are not GUI qualification. Hierarchy name-hit and
small-panel discoverability issues are not declared fixed here.

## Other existing workflows

Timeline/Graph retain independent scalar bone axes, numeric timing, selections
and Broken/Aligned/Mirrored tangent tools with their documented limits.
Setup changes the rig; Animate creates animation data.

Native AI keeps API/local providers, consent, typed proposals and explicit native
approval. P01-RC1 adds paged stored FFD reads, complete FFD key creation/replacement/
deletion, exact bone-key deletion and named default-payload events. See
[artistic operations](ai/ARTISTIC.md). No real provider/inference or artistic
quality is certified by the deterministic tests. Arbitrary topology, influence
membership and attachment rename remain manual or future work.

Video retains the existing CPU/FFmpeg workflow and requires compatible external
FFmpeg. PNG/JPEG/WebP, spritesheet and video exports retain their restrictions;
ReplayV1 does not make these rendered exports Physics-capable.

Native v2 keeps independent curves, preflight inventory, explicit replacement/
fidelity consent, staging and rollback. Ordinary failures preserve prior outputs;
multi-file publication is not fully crash-atomic or power-loss durable.
Project save/recovery protections are unchanged.

## Runtime admission is separate

See [Editor/runtime compatibility](editor-runtime-compatibility.md) and the
[runtime ReplayV1 guide](https://github.com/Kr-Apo/runtime).
A17 admits native v2, fixed default skin, Normal, exactly Idle/Attack/Hit/Death,
one stable always-visible weighted FFD owner, immutable topology/binds and the
qualified scale/Physics domain. Hard limits: 64 transition commands/session and
1024 pending events; no silent eviction. Empty/full FFD intervals remain HOLD.
Features qualified in separate earlier pilots are not automatically composable.

Current Frog Knight has three clips and its cape has 616 scalar FFD components;
it does not satisfy that integrated domain. No data is stripped or clips renamed
automatically to manufacture admission. Jade Sentinel remains the historical
distributed sample. Frog Knight is a private qualification input, not a new
publicly licensed sample.

Source repositories: [Classic](https://github.com/Kr-Apo/editor) and
[runtime](https://github.com/Kr-Apo/runtime). Product/package versions and project/
runtime format versions are independent.
