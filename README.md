> **P01-RC1 local candidate — 28 October 2026.** Read the [current contracts and qualification boundary](docs/p01-rc1-workflow.md) before the historical notes below. Proposed Classic v0.1.6 is not tagged/released here. Runtime package versions are unchanged; ReplayV1 A17 is explicit and separate from Legacy playback.

# Ankhimate

**A native 2D skeletal animation editor written in Rust** — bones, skins, meshes, weights, keyframes and an extensible export pipeline.

Rig image parts into a skeleton, animate them on a timeline, and export rendered frames or data for game runtimes. Ankhimate is free and open source under **MIT OR Apache-2.0**.

> **Status: v0.1.5 early public pre-release — Windows x64.** The editor works, but file compatibility, advanced workflows and hardware qualification are still evolving. This is not a production-stability guarantee. See [Known Limitations](docs/KNOWN-LIMITATIONS.md) and [current tasks](docs/TASKS.md).

![Ankhimate editor — native animation workspace](https://github.com/user-attachments/assets/34d2ad47-68e5-4218-ab15-415cb7d00a5a)

## Download & install

**[Download Ankhimate v0.1.5 for Windows](https://github.com/Kr-Apo/editor/releases/tag/v0.1.5)**

1. Download `Ankhimate-v0.1.5-Windows-Candidate.zip` from the release assets.
2. Extract the entire ZIP into a folder.
3. Double-click `Ankhimate.exe`. No Rust toolchain or launcher script is required.
4. To explore a sample, choose **File > Open…** and select `Examples/Jade-Sentinel/Sentinelle.ankh`. Keep its `Sentinelle.assets` folder next to the project.

The published v0.1.5 Windows executable is **unsigned**; its About dialog identifies that version. The P01-RC1 candidate instead reports **0.1.6** and its exact source commit. No v0.1.6 download or tag is announced before final approval.

The release is tagged at **[`f436474ace8580742113bf3447b430bbbd74153f`](https://github.com/Kr-Apo/editor/tree/v0.1.5)**. That tag preserves the corresponding source even when the `main` branch receives further documentation changes.

## What works today

- **Setup and Animate modes** — separate structural rigging edits from animated poses.
- **Bones and constraints** — hierarchy, FK transforms, multibone IK, Transform and Path constraints, plus bounded physics workflows.
- **Slots, attachments and skins** — assign visuals through slots, manage variations, draw order, clipping and supported attachment types.
- **Meshes and weights** — vertex and UV editing, alpha-silhouette tracing, weighted meshes, weight painting and validated mesh operations.
- **Animation** — Dopesheet, Graph Editor, Linear/Stepped/Bezier interpolation, onion skinning, events, clip reordering and retiming.
- **Free-form deformation (FFD)** — edit compatible mesh deformation keys with native Undo/Redo.
- **Import** — PNG, JPEG and WebP images; layered PSD files and supported PSD update/relink workflows; atlas-related tools.
- **Editing workflows** — hierarchy tools, selection, numeric editing, document commands and undoable operations.
- **Save and recovery** — `.ankh` author project profiles, sidecar assets, Save As, autosave/recovery review, and persistent **Ignore** for unchanged recovery copies.
- **Export** — native data/atlas, CPU-rendered PNG/JPEG/WebP sequences and Ankhimate spritesheets; MP4/GIF through **separately installed FFmpeg**.
- **Optional AI Assistant** — provider profiles for OpenAI, Anthropic, DeepSeek, Ollama, LM Studio and NInfer; native edits require explicit review and approval. No model is bundled.

See [Features](docs/FEATURES.md) for detailed capabilities and scope.

## Not there yet

The published v0.1.5 and proposed v0.1.6 remain early builds with important limits:

- **Runtime compatibility:** the separate TypeScript core and PixiJS/Phaser adapters support selected exported features, not every editor constraint, attachment, blend mode or deformation. **PathPosition is not supported in current game-runtime exports.**
- **CPU-rendered export:** currently uses the default skin and Normal blending, with no physics. Ankhimate spritesheet JSON is not a native Phaser atlas format.
- **Advanced editing:** vector Graph operations, linked/shared-mesh or arbitrary FFD migration, and some animation-manager interactions remain scoped or incomplete.
- **Project safety:** save backups with their `.assets` directories. Multi-file writes are not guaranteed power-loss atomic; recovery cannot restore every session detail.
- **Qualification:** broad Windows GPU/DPI checks, independent browser/adapter conformance and real AI-provider workflows remain pending. Automated test results are not full production certification.

Consult [Compatibility](docs/COMPATIBILITY.md), [Known Limitations](docs/KNOWN-LIMITATIONS.md), and [Tasks](docs/TASKS.md).

## Workspace

| Crate | Responsibility |
|---|---|
| [`core`](core) — `ankhimate-core` | Framework-independent skeleton, constraints, animation and pose evaluation; WASM-compatible core. |
| [`document`](document) — `ankhimate-document` | Document state, validation, native editing commands and Undo/Redo history. |
| [`editor`](editor) — `ankhimate-editor` | Desktop app using egui/eframe and a wgpu viewport. |
| [`formats`](formats) — `ankhimate-formats` | `.ankh` formats, supported conversions, image and PSD import. |
| [`plugins`](plugins) — `ankhimate-plugins` | Plugin and guarded extension interfaces. |
| [`render`](render) — `ankhimate-render` | Headless CPU rendering and compatible exports/tests. |
| [`export`](export) — `ankhimate-export` | Data/atlas export, templates, preflight and rendered output. |
| [`mcp`](mcp) — `ankhimate-mcp` | Headless tooling over supported document and rendering operations. |
| [`xtask`](xtask) | Development, documentation and validation helpers. |

Game-side code lives in the separate **[Kr-Apo/runtime](https://github.com/Kr-Apo/runtime)** fork (TypeScript core and PixiJS/Phaser adapters), derived from [Ankhimate/runtime](https://github.com/Ankhimate/runtime). **Editor v0.1.5 does not imply runtime npm package version 0.1.5.** Check the [editor/runtime compatibility contract](docs/editor-runtime-compatibility.md).

## Build & run

Requirements: **Rust 1.95** (pinned in `rust-toolchain.toml`), Git and its submodules, and a compatible graphics backend. Windows builds require MSVC C++ build tools and the Windows SDK.

```bash
git clone --recurse-submodules https://github.com/Kr-Apo/editor.git
cd editor
cargo run --locked -p ankhimate-editor
cargo run --locked -p ankhimate-mcp
cargo test --workspace --locked --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-targets
cargo run --locked -p xtask -- docs-check
```

To reproduce the **exact release source**, check out `v0.1.5` before building and update submodules for that tag. See [build instructions](docs/CANDIDATE_BUILD.md) for release-specific settings.

## Try it

The Windows package includes **Jade Sentinel**, a sample project with five image assets and `idle`/`attack` animation clips. Open it via **File > Open…**, explore its timeline and hierarchy, and use **File > Save As…** before editing.

More examples and guidance: [Rigging Walkthrough](docs/rigging-walkthrough.md) and [Jade Sentinel](docs/jade-sentinel.md).

## Contributing

Contributions and focused bug reports are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/TASKS.md](docs/TASKS.md). Include the version, commit, steps to reproduce and relevant logs, but never share private projects or credentials.

**Clean-room policy:** implement observed behavior independently; do not copy source code from other animation editors. Preserve upstream credit and third-party licenses.

## License & origin

This is the **unofficial Kr-Apo community fork** of [Ankhimate/editor](https://github.com/Ankhimate/editor). It is not endorsed by upstream maintainers.

The editor is dual-licensed, at your option, under [MIT](LICENSE-MIT) **OR** [Apache-2.0](LICENSE-APACHE). Bundled assets and dependencies have their own requirements. See [third-party source information](docs/THIRD_PARTY_SOURCE_NOTICE.md) and [Jade Sentinel's asset license](samples/jade-sentinel/ASSET-LICENSE.txt).

## Documentation

- [Getting Started](docs/GETTING-STARTED.md) — installation, Jade, Save As, recovery.
- [Features](docs/FEATURES.md) — supported authoring workflows.
- [Compatibility](docs/COMPATIBILITY.md) — project formats and game-runtime boundaries.
- [Known Limitations](docs/KNOWN-LIMITATIONS.md) — restrictions and risk areas.
- [Project Recovery](docs/project-recovery.md) — recover/Ignore behavior.
- [Graph Editor](docs/graph-editor.md) — editing curves and keys.
- [Export Context](docs/export-context.md) — templates and export data.
- [Rigging Walkthrough](docs/rigging-walkthrough.md) — building a rig.
- [Tasks](docs/TASKS.md) — history and outstanding acceptance criteria.
- [v0.1.5 Release](https://github.com/Kr-Apo/editor/releases/tag/v0.1.5) — download and checksum.

> **Versioning:** `v0.1.5` is intentionally numerically lower than historical `v0.2.0-alpha.1` candidates. The editor product version, author project schema, game-runtime format and npm package versions are separate identifiers.
