# Ankhimate v0.1.5

**Kr-Apo Community Fork (Unofficial) — local release candidate, not published.** Native Windows 2D skeletal animation editor, derived from [Ankhimate/editor](https://github.com/Ankhimate/editor). MIT OR Apache-2.0; original credits and third-party obligations are retained. No upstream endorsement is claimed.

Import layered PSD/images, build bones/slots/skins and compatible weighted meshes, animate with the Dopesheet/Graph and FFD tools, save safely, and export native data/atlas or rendered image/video outputs. Optional approved native AI operations use six configured providers; no model or FFmpeg is bundled. Implementation is distinct from artistic or physical qualification.

## Windows candidate

Extract the entire supplied ZIP, double-click **Ankhimate.exe**, then **File > Open…** → `Examples/Jade-Sentinel/Sentinelle.ankh`. Keep all five PNG assets beside the project. Use **Save As…** before editing. Welcome intentionally has no version; **Help > About Ankhimate** shows0.1.5, local build status and source commit. Ignore recovery copies without deleting them; revisit them with **File > Review Recovery Copies…**.

Read [Getting Started](docs/GETTING-STARTED.md), [Features](docs/FEATURES.md), [Compatibility](docs/COMPATIBILITY.md), [Known Limitations](docs/KNOWN-LIMITATIONS.md), [Recovery](docs/project-recovery.md) and the [runtime contract](docs/editor-runtime-compatibility.md). CPU exports retain default skin/Normal/no physics; PathPosition is editor/CPU-only. The runtime is a separate project; its new fork URL is not yet created or configured.

## Source and development

Use the pinned Rust toolchain and initialize the pinned community-plugins submodule. [Build/package instructions](docs/CANDIDATE_BUILD.md), [task status](docs/TASKS.md), [release preparation](docs/t838-release-preparation.md). Current candidate commits are local; future public clone URLs remain unset. Do not substitute deleted old fork URLs. Product0.1.5 deliberately resets the earlier0.2.0-alpha.1 numbering and is not semver-newer; project/runtime formats and npm package versions do not follow this reset.

Historical RC2/Alpha archives and source checkpoints remain preserved evidence, not current download endpoints. [MIT](LICENSE-MIT), [Apache-2.0](LICENSE-APACHE), [third-party source availability](docs/THIRD_PARTY_SOURCE_NOTICE.md), [Jade asset license](samples/jade-sentinel/ASSET-LICENSE.txt). Physical testing and release authorization are pending. No fork creation, push, tag or release is part of this preparation.
