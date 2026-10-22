# Known Limitations — v0.1.5 (T838)

- This is an **unsigned, local-only Windows x64 candidate**. Physical Windows interaction and artistic workflows are still pending review. The former public Alpha is a different, preserved artifact; its old fork URL is no longer an active distribution endpoint.
- Save a copy of your project **together with its assets**. No writer guarantees a durable multi-file transaction after a power failure. Windows may remove temporary files; autosave is not a permanent backup.
- **Ignore** preserves recovery data. If its registry is inaccessible, corrupted, or lost, a recovery copy may be offered again; a persistence failure is reported. Valid copies older than a confirmed primary project are filtered using timestamps. To review copies belonging to a different named project, open that project's path first, then use **File > Review Recovery Copies…**. This is not a global browser for every recovery file on your computer.
- Recovery does not restore the prior session's Undo history, selections, or necessarily every last edit before a crash. Damaged files are not automatically repaired.
- Complex PSD effects, topology or ownership changes, linked meshes, and FFD migration have exclusions. Protective refusals help avoid breaking existing references.
- CPU export is limited to the **default skin**, **Normal** blending, and **no physics**. Spritesheet JSON is not a native Phaser atlas. MP4/GIF require separately installed FFmpeg; it is not bundled.
- **PathPosition** works in the editor and CPU rendering, but older project readers reject it and the game runtimes do not support it. Advanced constraints, linked meshes, and all blending modes do not have universal runtime parity.
- **T-208:** displaying animation duration in frames in the manager and automatically entering Animate mode when selecting a clip in the manager remain incomplete. Some key types are excluded from group operations, the Graph Editor, or the clipboard. FFD support is deliberately scoped and does not include a vector Graph Editor.
- Interrupting a crossfade can produce a discontinuity, depending on the runtime. This package includes no new GPU benchmark or hardware qualification.
- The **AI Assistant** is optional and depends on the provider and the installed/configured model. Mock-based tests and validated native commands do not prove artistic quality or real compatibility with every model. Do not include private projects or secrets in bug reports.

See [Features](FEATURES.md), [Compatibility](COMPATIBILITY.md), and [Getting Started](GETTING-STARTED.md). Future priorities include manual qualification, workflow limitations, and reliability. No publication date is promised.
