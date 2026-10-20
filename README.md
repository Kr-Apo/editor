# Ankhimate — Kr-Apo Community Fork (Unofficial)

This is a community-developed derivative maintained independently by Kr-Apo.
The original projects are [Ankhimate/editor](https://github.com/Ankhimate/editor)
and [Ankhimate/runtime](https://github.com/Ankhimate/runtime).
This derivative is not officially endorsed or maintained by the upstream
Ankhimate organization. Original MIT OR Apache-2.0 licensing, copyrights and
third-party attribution requirements are preserved. Naming identifies upstream
origin; no upstream trademark license or endorsement is claimed.

Ankhimate is a native Windows editor for 2D skeletal animation, with separate
TypeScript game adapters. **T-811 is a local next-alpha candidate, not a release.**
Proposed versions: Editor/core/Phaser/PixiJS **0.2.0-alpha.1**, subject to review.
The [existing public RC2](https://github.com/Kr-Apo/ankhimate/releases/tag/alpha-2026.10.18-rc2)
is unchanged and does not contain the new AI or simplified MP4 workflow.

Explore the existing tools with [First rig in ten minutes](docs/first-rig-in-10-minutes.md)
and the factual [rigging capability matrix](docs/rigging-capabilities.md).

## Create, animate, export

- Rig bones and image parts; use Setup/Animate, curves, slots, skins, meshes,
  weight painting and FFD, with native Undo/Redo and save/recovery safeguards.
- Import PSD/images; export native v2/atlas, PNG/JPEG/WebP sequences and transparent
  PNG spritesheets using the existing deterministic CPU renderer.
- **File → Export → Video MP4…**: clip, 720p/1080p, 24/30/60 FPS, 1–3 cycles,
  opaque background, progress and cancel. Requires separately installed FFmpeg
  with libx264. Existing videos are refused. Advanced MP4/GIF remain available.
- Use the opt-in native AI assistant for bounded reads and approved animation,
  Setup rigging, region assembly and weight corrections. Vision requires explicit
  capture consent; actual provider/model capabilities and context limits matter.

## Try the candidate

Extract the complete portable Windows ZIP after receiving it from the maintainer.
Run `Editor/Ankhimate.exe`, open a **copy** of `Examples/Jade-Sentinel/Sentinelle.ankh`
with its assets, and play `idle` or `attack`. Save As before editing. Export native
v2 into a new directory, then run `Start-Demo.exe` to view it through the bundled
local PixiJS demonstration, without Node/Rust or internet downloads.

Jade Sentinel is the only official alpha sample: procedural art, weighted meshes,
FFD, two clips and events. The maintainer's unsaved Qwen test rig is not included.
The binary is unsigned. New-archive Windows and interactive GPU checks remain
required; no macOS/Linux/mobile, universal GPU or performance certification.

## Runtime and evidence

[Core, PixiJS 8.17.0 and Phaser 3.90.0](https://github.com/Kr-Apo/ankhimate-runtime)
share the documented native v2 subset. T-811 adds 125 bounded headless WebGL pixel
comparisons (104 exact) to the Rust/Node numeric evidence, on one ANGLE/NVIDIA
configuration. One measured clipping contour difference remains inside the frozen
edge tolerance. Review [features](docs/FEATURES.md) and
[limitations](docs/KNOWN_LIMITATIONS.md); constraints, physics, linked meshes,
rotated atlases and non-Normal composition are outside the common profile.

Editor screenshots: final authorized captures are pending maintainer approval.
No generated screenshot is presented as the application.

## Build the editor from source

Install the pinned Rust toolchain (`rust-toolchain.toml`: 1.95.0) and the native
build tools for your host. Windows builds need the MSVC C++ build tools and SDK.

```sh
git clone --recurse-submodules https://github.com/Kr-Apo/ankhimate.git
cd ankhimate
cargo build --locked --release -p ankhimate-editor
cargo run --locked -p ankhimate-editor
```

Builds on other hosts do not establish a qualified Linux or macOS distribution.
This local candidate proposes editor 0.2.0-alpha.1; supporting Rust crates retain their existing versions.
See the runtime repository for TypeScript builds. No npm publication is implied.

## Documentation

- [Features](docs/FEATURES.md) and [known limitations](docs/KNOWN_LIMITATIONS.md)
- [AI assistant, consent and manual evidence](docs/AI_ASSISTANT.md)
- [MP4 guide](docs/EXPORT-MP4.md)
- [Release readiness](docs/RELEASE_READINESS.md) and [changelog](CHANGELOG.md)
- [Historical RC2 getting started](docs/alpha/GETTING_STARTED.md)
- [Architecture](docs/ARCHITECTURE_PLAN.md), [tasks](docs/TASKS.md) and [llms.txt](llms.txt)

## Contribute and report problems

Read [CONTRIBUTING](CONTRIBUTING.md) and [SECURITY](SECURITY.md). Include the exact
release/commit, Windows/browser/GPU versions and a minimal reproduction you may
share publicly. Never attach private projects, credentials or personal paths.
There is no automatic bug-report upload in the supplied demo.

## License

Ankhimate code is **MIT OR Apache-2.0**, at your choice:
[MIT](LICENSE-MIT), [Apache-2.0](LICENSE-APACHE). Bundled dependencies/fonts keep
their own notices. The ZIP includes a complete notices collection and Jade's MIT
asset grant; these are not replaced by the source-code license.
