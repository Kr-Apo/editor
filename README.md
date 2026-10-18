# Ankhimate — Kr-Apo Community Fork (Unofficial)

This is a community-developed derivative maintained independently by Kr-Apo.
The original projects are [Ankhimate/editor](https://github.com/Ankhimate/editor)
and [Ankhimate/runtime](https://github.com/Ankhimate/runtime).
This derivative is not officially endorsed or maintained by the upstream
Ankhimate organization. Original MIT OR Apache-2.0 licensing, copyrights and
third-party attribution requirements are preserved. Naming identifies upstream
origin; no upstream trademark license or endorsement is claimed.

Ankhimate is an open-source editor for 2D skeletal animation, with a separate
TypeScript runtime for games. This first experimental alpha distributes a
**Windows x64 editor and a PixiJS 8 / WebGL2 runtime**.

**[Download the Windows + PixiJS alpha](https://github.com/Kr-Apo/ankhimate/releases/tag/alpha-2026.10.18-rc2)** ·
**[Get started](docs/alpha/GETTING_STARTED.md)** ·
**[Runtime repository](https://github.com/Kr-Apo/ankhimate-runtime)**

The distributed Windows executables are unsigned. Compatibility with every
project, GPU or browser is not guaranteed. Source features and runtime features
have different scopes; check the [compatibility matrix](docs/alpha/RUNTIME_COMPATIBILITY.md)
before exporting a rig.

## Try Jade Sentinel

1. Download and extract the complete RC2 ZIP. Verify its SHA-256 against the release.
2. Run `Editor/Ankhimate.exe` and open `Examples/Jade-Sentinel/Sentinelle.ankh`.
   Keep its adjacent `.assets` folder. Save a separate copy before editing.
3. In Animate, play `idle` or `attack`. Use Data / atlas and the Ankhimate runtime
   preset to export native v2 into a new folder inside the extracted archive.
4. Review the export preflight, then publish only if its diagnostics are acceptable.
5. Run `Start-Demo.exe`, keep its console open, and load your export in the browser.

The bundled demo needs a desktop WebGL2 browser, but no Node.js, npm, Rust or
internet download. The launcher serves only the extracted folder on loopback.
See [Getting Started](docs/alpha/GETTING_STARTED.md) for the exact workflow.

Jade Sentinel is the **only official alpha sample**. It uses original procedural
art, weighted meshes, FFD, two clips and events. Older examples and the historical
Crapaud demonstrator remain development/regression material, not additional
official samples or claims of runtime qualification.

## Qualified game-runtime subset

Native runtime v2, bones, regions, rigid/weighted meshes, compatible FFD,
Normal blending, light/static dark tint, compatible skins, sequential polygon
clipping, looping, events and distinct-clip crossfades are qualified within the
documented limits. Interrupted crossfades can produce a visible pose jump.

Constraints, linked meshes, Add/Multiply/Screen, WebGPU, WebGL1 and mobile are
outside the qualified PixiJS alpha profile. Phaser remains available in the
[runtime sources](https://github.com/Kr-Apo/ankhimate-runtime/tree/main/phaser) with its historical tests; it is
not an officially qualified distributed runtime for this alpha.

## Build the editor from source

Install the pinned Rust toolchain (`rust-toolchain.toml`: 1.95.0) and the native
build tools for your host. Windows builds need the MSVC C++ build tools and SDK.

```sh
git clone https://github.com/Kr-Apo/ankhimate.git
cd ankhimate
cargo build --locked --release -p ankhimate-editor
cargo run --locked -p ankhimate-editor
```

Builds on other hosts do not establish a qualified Linux or macOS distribution.
The separately versioned distribution is `alpha-2026.10.18-rc2`; the editor crates remain 0.1.0.
See the runtime repository for TypeScript builds. No npm publication is implied.

## Documentation

- [Alpha documentation index](docs/alpha/README.md)
- [PixiJS integration and public APIs](docs/alpha/PIXIJS_INTEGRATION.md)
- [Native runtime v2](docs/alpha/FORMAT_V2.md)
- [Troubleshooting](docs/alpha/TROUBLESHOOTING.md)
- [Release notes](docs/alpha/RELEASE_NOTES.md)
- [AI integration context](docs/alpha/AI_INTEGRATION_CONTEXT.md) and [llms.txt](llms.txt)
- [Authoring architecture](docs/ARCHITECTURE_PLAN.md) and [task status](docs/TASKS.md)

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
