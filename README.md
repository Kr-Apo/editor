# Ankhimate

**A native 2D skeletal animation editor built with Rust.** Rig illustrated characters, animate with bones and weighted meshes, refine free-form deformations, and export your work for video or compatible game runtimes.

**[Download v0.1.6 — Windows x64](https://github.com/Kr-Apo/editor/releases/tag/v0.1.6)** · **[Getting Started](docs/GETTING-STARTED.md)** · **[PixiJS / Phaser runtimes](https://github.com/Kr-Apo/runtime)**

> **Status:** v0.1.6 public **pre-release**. The Windows executable is unsigned. Advanced workflows, hardware coverage and runtime compatibility are still evolving.

<img width="2559" height="1390" alt="frog-knight-editor" src="https://github.com/user-attachments/assets/13d147de-3a85-49b7-af7e-c374050f0a23" />

## Download & install

1. Download the Windows ZIP from the **[v0.1.6 release](https://github.com/Kr-Apo/editor/releases/tag/v0.1.6)**.
2. Extract the complete ZIP to a writable folder and run `Ankhimate.exe`. No Rust installation is needed.
3. Open `Examples/Jade-Sentinel/Sentinelle.ankh` via **File > Open…**. Keep `Sentinelle.assets` alongside the project.
4. Use **File > Save As…** before editing the sample.

The installer-free archive contains no AI model or FFmpeg executable. See [Getting Started](docs/GETTING-STARTED.md) and [Known Limitations](docs/KNOWN-LIMITATIONS.md).

## Features

- **Skeletal rigging:** bones, hierarchies, skins, slots, attachments, supported IK/Transform/Path constraints and Physics workflows.
- **Meshes & weights:** UVs, mesh tracing, weighted skinning, manual weight painting and guarded Auto Weight.
- **Animation:** Dopesheet, Graph Editor, Linear/Stepped/Bézier interpolation, onion skinning, events and timeline editing.
- **Free-form deformation (FFD):** compatible animated mesh deformations, full-pose/rest keys, Undo/Redo and Save/Reopen.
- **Artwork import:** PNG, JPEG, WebP and supported layered PSD workflows.
- **Export:** native v2 data/atlas, PNG/JPEG/WebP frames, spritesheets and MP4/GIF with a separate FFmpeg installation.
- **Project recovery:** `.ankh` projects with accompanying `.assets`, autosave and recovery-copy review.

Read the [full feature guide](docs/FEATURES.md), [rigging walkthrough](docs/rigging-walkthrough.md) and [FFD/weights guide](docs/p01-rc1-workflow.md).

## Optional native AI Assistant

Ankhimate supports configurable **OpenAI, Anthropic, DeepSeek, Ollama, LM Studio and NInfer** provider profiles, including local AI servers. It can inspect supported project data and propose reviewed native edits, including selected animation, weight, FFD and event operations. **Every native modification requires explicit approval and validation**, with Undo/Redo support. No model is bundled, and artistic quality is not guaranteed.

See the [AI artistic tools](docs/ai/ARTISTIC.md) and [command contracts](docs/ai/TOOLS.md).

## PixiJS & Phaser runtimes

The separate **[Ankhimate runtime repository](https://github.com/Kr-Apo/runtime)** provides a TypeScript animation core with **PixiJS 8** and **Phaser 3** adapters. Export compatible characters using the **Data / atlas** preset; the runtime consumes native export data, **not `.ankh` author files**.

Experimental opt-in **ReplayV1** has a qualified **A17** pilot combining bones, weighted FFD, animated slots, events, crossfades, interruptions and continuous Physics under **strict admission limits**. It is not a general guarantee for every rig and does not extend automatically to Legacy playback.

**[ReplayV1 contract & limits](https://github.com/Kr-Apo/runtime/blob/main/docs/replay-v1.md)** · **[Editor/runtime compatibility](docs/editor-runtime-compatibility.md)**

The integrated A17 pilot uses a fixed default skin, one stable weighted FFD owner, exactly `Idle`/`Attack`/`Hit`/`Death` and bounded transition/event queues. Linked FFD, composed clipping and dynamic skins are not part of that integrated scope. No new npm package publication is implied.

## What’s new in v0.1.6

- Auto Weight now **refuses incomplete results** on disconnected geometry without changing existing hand-painted weights.
- Export preflight identifies faulty weighted vertices and their reasons.
- **Animate** exposes FFD vertex editing and explicit **Key FFD pose / Key setup FFD** controls.
- Native AI commands gain paged FFD reads, supported FFD key write/delete, exact bone-key deletion and default-payload animation events.
- Updated Classic/ReplayV1 compatibility documentation.

The reported Trace Mesh triangulation failure was **not reproduced**; diagnostics improved, but a general fix is **not claimed**. See the [v0.1.6 release notes](https://github.com/Kr-Apo/editor/releases/tag/v0.1.6).

## Important limitations

- This is an **early public pre-release**, not a general production or mobile certification.
- CPU-rendered exports use the **default skin**, **Normal** blending and **no Physics**. Native spritesheet metadata is not directly a Phaser atlas.
- Some advanced editing, linked/shared FFD, topology migration, constraints and runtime combinations remain unqualified.
- **PathPosition** is not supported by current game-runtime exports.
- Keep backups of each `.ankh` project **and its `.assets` folder**; autosave is not a full backup.

See [Compatibility](docs/COMPATIBILITY.md) and [Known Limitations](docs/KNOWN-LIMITATIONS.md).

## Build from source

Requires Rust **1.95** (pinned in `rust-toolchain.toml`), Git with submodules and supported graphics/build tools. Windows requires MSVC and the Windows SDK.

```bash
git clone --recurse-submodules https://github.com/Kr-Apo/editor.git
cd editor
cargo run --locked -p ankhimate-editor
cargo test --workspace --locked --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-targets
cargo run --locked -p xtask -- docs-check
```

To inspect the exact tagged source, check out [`v0.1.6`](https://github.com/Kr-Apo/editor/tree/v0.1.6). The Windows binary was built from an **identical-tree local source checkpoint**, not rebuilt from the public merge SHA; the [release page](https://github.com/Kr-Apo/editor/releases/tag/v0.1.6) records its provenance.

## Documentation & contributing

[Getting Started](docs/GETTING-STARTED.md) · [Features](docs/FEATURES.md) · [Graph Editor](docs/graph-editor.md) · [Export compatibility](docs/COMPATIBILITY.md) · [AI Assistant](docs/ai/ARTISTIC.md) · [Tasks](docs/TASKS.md) · [Contributing](CONTRIBUTING.md)

Bug reports and focused contributions are welcome. Include version, reproduction steps and error messages without posting private artwork or credentials.

## License & origin

Ankhimate is open source under **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**. This is the **unofficial Kr-Apo community fork** of [Ankhimate/editor](https://github.com/Ankhimate/editor), not endorsed by upstream maintainers. Original credits and third-party notices are preserved in [the source notices](docs/THIRD_PARTY_SOURCE_NOTICE.md).<img width="512" height="512" alt="frog-knight-demo" src="https://github.com/user-attachments/assets/fee7498d-7a3c-406f-b2fb-a2b620a8d640" />

