> **P01-RC1 local candidate — 28 October 2026.** Read the [current contracts and qualification boundary](p01-rc1-workflow.md) before the historical notes below. Proposed Classic v0.1.6 is not tagged/released here. Runtime package versions are unchanged; ReplayV1 A17 is explicit and separate from Legacy playback.

# Known Limitations — v0.1.6 Corrective Candidate (P01-RC1)

- This is an **unsigned, local-only Windows x64 candidate**. The P01-RC1 evidence qualifies the isolated Frog Knight Auto Weight/FFD/save/export workflow, not every artistic workflow or Windows configuration. Public v0.1.5 and frozen RC2 remain separate artifacts.
- Save a copy of your project **together with its assets**. No writer guarantees a durable multi-file transaction after a power failure. Windows may remove temporary files; autosave is not a permanent backup.
- **Ignore** preserves recovery data. If its registry is inaccessible, corrupted, or lost, a recovery copy may be offered again; a persistence failure is reported. Valid copies older than a confirmed primary project are filtered using timestamps. To review copies belonging to a different named project, open that project's path first, then use **File > Review Recovery Copies…**. This is not a global browser for every recovery file on your computer.
- Recovery does not restore the prior session's Undo history, selections, or necessarily every last edit before a crash. Damaged files are not automatically repaired.
- Complex PSD effects, topology or ownership changes, linked meshes, and FFD migration have exclusions. Protective refusals help avoid breaking existing references.
- CPU export is limited to the **default skin**, **Normal** blending, and **no physics**. Spritesheet JSON is not a native Phaser atlas. MP4/GIF require separately installed FFmpeg; it is not bundled.
- **PathPosition** works in the editor and CPU rendering, but older project readers reject it and the game runtimes do not support it. Advanced constraints, linked meshes, and all blending modes do not have universal runtime parity.
- **T-208:** displaying animation duration in frames in the manager and automatically entering Animate mode when selecting a clip in the manager remain incomplete. Some key types are excluded from group operations, the Graph Editor, or the clipboard. FFD support is deliberately scoped and does not include a vector Graph Editor.
- Legacy crossfade interruption may be discontinuous. Explicit experimental ReplayV1 has bounded accepted-pose interruption qualification; those guarantees do not transfer to Legacy. P01-RC1 reruns targeted WebGL2 cases on one machine, not broad hardware certification.
- The **AI Assistant** is optional and depends on the provider and the installed/configured model. Mock-based tests and validated native commands do not prove artistic quality or real compatibility with every model. Do not include private projects or secrets in bug reports.

See [Features](FEATURES.md), [Compatibility](COMPATIBILITY.md), and [Getting Started](GETTING-STARTED.md). Future priorities include manual qualification, workflow limitations, and reliability. No publication date is promised.
