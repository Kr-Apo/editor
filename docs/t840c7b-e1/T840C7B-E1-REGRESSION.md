# T-840C7B-E1 — Regression and preservation

## Executed

Full Rust workspace --locked --all-targets: 2,231 passed, four ignored. No real inference or application GUI was launched. The ignored opt-in/manual tests remain ignored. Fmt, workspace/all-targets Clippy, workspace build, core wasm32 check and xtask docs-check pass. Nine added tests are included. Export safety, Windows locks/rollback, old preset, v1/v2 compatibility, animation/parity and CPU PNG/spritesheet reference suites run as part of the workspace gate. No goldens were regenerated.

Baseline Clippy on the exact B6 archive plus its existing community-plugin fixtures and current Clippy emit identical multisets of 25 warning messages; WARNING-COMPARISON.json lists them. Existing deprecations/test/style warnings are retained. Build reports the same three existing editor deprecations. None is fixed outside E1 scope.

Runtime remains C7A fa4e6078ddff8c85ca5a6b91f7e5dae9174476b1. Existing 102 C2/C6A/C7A targeted tests and 105 core compiled tests pass. Its tracked source, compiled protected evidence, configuration, branches and remotes are preserved. Broad historical graphical suites, 752 Pixi cases, 125 modern comparisons, Phaser campaigns, C7B WebGL/performance and artistic acceptance are NOT_RUN here; their old accepted evidence is preserved, not represented as a fresh pass.

## Before/after native inventory

HISTORICAL-INVENTORY.json enumerates every changed JSON leaf and all file identities for eight pairs. Four unweighted projects (minimal, walker, C7A simple/mixed) have byte-identical native JSON. The four weighted projects (C7B simple/mixed/near-singular and the real Sentinel reference) differ only in derived influence/flat LOCAL coordinates and, where applicable, full-precision bone numbers. Root/member shapes, source vertices, UVs, indices, counts, bone references/order and coefficients are unchanged. Sentinel is an export-only weighted reference, not a new Replay qualification.

All eight saved author files, image assets and atlas files remain byte-identical. On the three measured before/after bind traces, bones, Physics records and native skinned reference points remain identical. Only export-derived results change. Simple/mixed differences are the expected removal of f32 conversion quantization; no previously valid reference fails. Singular/nonfinite binds and materially lossy old presets now fail explicitly instead of emitting misleading data; those error semantics are documented for review.

## Protected inventory

PRESERVATION-FINAL.json verifies all 27,950 previously sealed files, including prior mission reports/raw oracles, the 97 C7B prototype files with their unvalidated diff/patch, historical 207 reference set and active/protected projects. The original D: repositories stay clean at their initial checkpoints. Original local refs/remotes/configurations are preserved; only the new Editor branch gains a local commit. RC2 SHA-256 remains a1c9835fbe83468f6e87bdd5676965a45210c4913305fc18b72c9d426b9553c0.

Delivery includes full binary Git diff, format-patch, changed-file hashes, commands and raw results. No code/format migration, reference-oracle replacement, dependency/version change, runtime modification, push, merge, tag or publication occurs. The guarded source digests in docs/format-schema.json change solely because the reviewed context/template source changed, not because a historical expected result was rebaselined.
