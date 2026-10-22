# Ankhimate — Kr-Apo Community Fork (Unofficial)

État local **T837** : correction Recovery, documentation utilisateur et dossier Windows simplifié. Ce checkpoint n’est pas publié ; aucun changement de l’Alpha publique ni de RC2. [Fonctionnalités](docs/FEATURES.md), [premiers pas](docs/GETTING-STARTED.md), [compatibilité](docs/COMPATIBILITY.md), [développement et tâches](docs/TASKS.md).

Pour la livraison Windows, double-cliquez sur Ankhimate.exe, puis File > Open… et Examples/Jade-Sentinel/Sentinelle.ankh. Faites Save As avant modification. Les sources Rust, fixtures et procédures de build restent ici ; le ZIP utilisateur ne les remplace pas. Licences MIT OR Apache-2.0 et crédits amont conservés.

<details>
<summary>Documentation de référence antérieure — T836 et publication précédente</summary>

# Ankhimate — Kr-Apo Community Fork (Unofficial)

Native Windows authoring for2D skeletal animation, with separate TypeScript game adapters. Maintained independently by Kr-Apo; derived from [Ankhimate/editor](https://github.com/Ankhimate/editor) and [Ankhimate/runtime](https://github.com/Ankhimate/runtime). No upstream endorsement or trademark license is claimed. Preserve original MIT OR Apache-2.0 licenses, copyrights and third-party attributions.

## Choose the right artifact

- Accepted public [v0.2.0-alpha.1](https://github.com/Kr-Apo/ankhimate/releases/tag/v0.2.0-alpha.1): the T811/T812 release, with its own frozen documentation/evidence.
- This checkout: **T836 local pre-alpha preparation**, based on accepted T835. It is not a new release. Internal version0.2.0-alpha.1 is unchanged; use the exact local commit and binary SHA. No publication is authorized.
- Historical [RC2](https://github.com/Kr-Apo/ankhimate/releases/tag/alpha-2026.10.18-rc2): unchanged and separate. Do not replace it with this build.

## Start with Jade Sentinel

[Jade Sentinel](samples/jade-sentinel/README.md) is the only active user character. Keep `Sentinelle.ankh` and `Sentinelle.assets` together. The local Windows-Test-Candidate contains its unchanged authorized copy, the current unsigned x64 executable, notices and a quickstart. Save As to a disposable working directory before editing; keep backups/recovery sidecars. It needs no Rust, Node, model or FFmpeg simply to open/play/save. Actual physical Windows acceptance remains pending.

[Jade workflow](docs/jade-sentinel.md): inspect Setup bones/attachments/weights; switch Animate, play `idle`/`attack`; make a small change, Undo/Redo, Save/Reload; preflight a fresh native or rendered export. Jade has five procedural PNG assets and no identified PSD. Historical technical samples/PSDs remain for specific tests. Crapaud is historical-only and excluded from the test candidate.

## Existing tools and honest boundaries

The local editor includes save/recovery safeguards, diagnostics/navigation, bounded region rig transfer, linked Bézier tangents, FFD key editing/clipboard, guarded PSD reimport, compatible weighted retracing, onion skinning, Graph marquee/normalized view, faster selection validation/highlighting, native Undo FPS, whole-clip numeric retime, selected-bone offsets and saved clip ordering. [Inventory](docs/FEATURE_INVENTORY.md) separates implementation, evidence and availability; the historical101-task counts stay62/33/1/3/2. T208/T209/T211/T404/T502 remain partial.

Rendered sequences support PNG/JPEG/WebP; PNG spritesheets and batch/framing/trim/padding share the CPU renderer. Default skin/Normal blending only; physics is refused. JPEG needs an explicit opaque background. MP4/GIF require separately installed FFmpeg; none is bundled. Native JSON/atlas uses runtime v2 and preflight/staging/rollback, without crash-atomic multi-file promises.

**PathPosition is native-editor/CPU only**: recent compatible reader required, old readers reject its tag, game exporters refuse it. Old editors are not certified to preserve T835 clip order on resave. Read [compatibility](docs/release-compatibility.md) before sharing author files. PixiJS/Phaser support only their documented subsets, not every editor feature.

The opt-in AI panel provides bounded native reads and approved transactions. [AI guide](docs/AI_ASSISTANT.md). No model/backend/credential is included. A proposal is not an applied edit; capture and changes need their own explicit consent. No new inference or visual certification in T836.

## Build and validate

Install pinned Rust1.95.0 and Windows MSVC C++ build tools/SDK. Clone this source with its pinned `community-plugins` submodule for integration tests. Select the reviewed local branch/commit; do not assume remote main contains local T813–T836 work.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --all-targets -- --test-threads=4
cargo clippy --workspace --locked --all-targets
cargo build --workspace --locked
cargo build --locked --release -p ankhimate-editor
cargo check --locked -p ankhimate-core --target wasm32-unknown-unknown
cargo run --locked -p xtask -- docs-check
```

[Build details](docs/CANDIDATE_BUILD.md), [contributing](CONTRIBUTING.md), [architecture](docs/ARCHITECTURE_PLAN.md), [current tasks](docs/TASKS.md), [roadmap](docs/ROADMAP_NEXT.md), [manual campaign](docs/MANUAL_QA_PENDING.md), [llms.txt](llms.txt). Do not run an editor build concurrently with the controlled GIF regression pass. Historical Windows sharing-lock intermittence is not fixed by this cleanup. Preserve all logs and assertions.

## License and reporting

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE); Jade has its own preserved MIT asset grant. Dependency/font notices remain separate. No source-art right is inferred from conversion. Use [SECURITY](SECURITY.md) and minimal reproducible bug reports with exact artifact/OS information. Do not upload private projects, credentials or unredacted personal paths. [Known limitations](docs/KNOWN_LIMITATIONS.md).

</details>
