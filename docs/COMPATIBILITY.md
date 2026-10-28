> **P01-RC1 local candidate — 28 October 2026.** Read the [current contracts and qualification boundary](p01-rc1-workflow.md) before the historical notes below. Proposed Classic v0.1.6 is not tagged/released here. Runtime package versions are unchanged; ReplayV1 A17 is explicit and separate from Legacy playback.

# Compatibility — v0.1.6 Corrective Candidate (P01-RC1)

This candidate is neither RC2 nor the historical v0.2.0-alpha.1 release. Its internal product version is 0.1.6; identify the exact build using its checksum and delivery checkpoint. This deliberately resets numbering below the former 0.2.0-alpha.1; version comparators may treat it as older. Author schema/envelope 1 and runtime format v2 do not change.

| Area | Compatibility contract |
|---|---|
| Existing projects | The current reader accepts the tested, authentic Ankh v1 profiles (binary, compact JSON, and readable JSON), with assets stored separately. This is not a promise to migrate every historical prototype. |
| Newer or unknown files | Unrepresentable versions, flags, and fields are rejected. Do not edit a version label by hand. Keep the original file. |
| PathPosition | This adds a variant to a closed set, although the author format and envelope still identify as version 1. Older readers reject projects containing this new track; native/runtime and script exports that cannot represent it are blocked. No PixiJS or Phaser runtime support is claimed. |
| Animation clip order | The authored order is stored in the animation array. Older editor versions may lose that order when they save the project again. Clip identities and keyframes are not renamed. |
| Runtime v1/v2 | T838 does not change either runtime. External readers support their own defined subsets; the native exporter produces v2, not fully baked animation for every constraint. |
| PixiJS / Phaser | These are separate runtimes with narrowly qualified compatibility profiles. FFD, skins, dark tint, clipping, and blending support depend on the case and adapter. Advanced constraints, physics, and linked meshes are not guaranteed. No new WebGL qualification was carried out in T838. |
| Rendered exports | Existing Rust CPU rendering supports the default skin and Normal blending; physics is refused. Spritesheet JSON uses Ankhimate's format, not Phaser's native atlas format. MP4/GIF require a separately installed compatible FFmpeg executable. |
| Recovery | **Ignore** decisions are stored in a separate registry, never in `.ankh`. Older versions do not understand this registry and may offer the same recovery copy again. The project file format is unchanged. |

Native Windows x64 is the built target. The EXE is unsigned. There is no broad certification for mobile, WebGPU, WebGL 1, macOS/Linux, drivers, or every graphics card. Keep backup copies before trying a local development candidate. The matching source code is retained in the repository and identified by the delivery README; T838 / v0.1.5 has not been published.

## Source and runtime availability

The original upstream projects are Ankhimate/editor and Ankhimate/runtime. New community forks have not been created. Their future URLs remain explicitly unset; do not use the deleted Kr-Apo repository links as installation endpoints. The prepared compatibility guide records the protected runtime checkpoint and unchanged package versions. Publication and physical smoke tests require separate maintainer approval.
