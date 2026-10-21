# Contributing

Small, reproducible changes are welcome. Explain the user-visible problem,
the affected release/commit and the verification performed. Discuss larger work
before implementation; do not treat every editor feature as a runtime promise.

## Safety and scope

Contribute only code and assets you may redistribute. Do not copy, translate or
closely paraphrase another animation editor's source. Describe behavior and use
original implementations. Preserve existing copyright and dependency notices.
If you have read the source of a GPL-licensed animation editor, do not contribute
to the matching Ankhimate feature; choose a different task.
For this repository's code, contributions are MIT OR Apache-2.0 unless explicitly
agreed otherwise. Dependency and sample-asset licenses remain separate.

Never attach private projects, credentials, access tokens, local configuration or
unredacted personal paths. Construct a minimal synthetic rig where possible.
Keep `.ankh` assets alongside any shareable project. Include only the files needed
to reproduce; verify ownership before posting screenshots or sample artwork.

## Changes and validation

Keep changes focused and add a regression before a confirmed bug fix. Explain
which checks were run and which historical evidence was retained. UI changes
need relevant interaction checks; rendering changes need numerical/pixel evidence,
not just screenshots. Do not update reference outputs simply to make tests pass.

Use explicit staged paths and review the complete diff. Do not include generated
build/cache files, backups, private recordings or dependency directories.
Do not introduce a new format version, package publication or release asset as an
incidental part of a fix. Read SECURITY.md for vulnerability reporting.

## Editor commands and architecture

Use the pinned Rust 1.95.0 toolchain. Windows builds need MSVC build tools/SDK.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --all-targets -- --test-threads=4
cargo clippy --workspace --locked --all-targets
cargo build --workspace --locked
cargo check --locked -p ankhimate-core --target wasm32-unknown-unknown
cargo run --locked -p xtask -- docs-check
```

Consult [architecture](docs/ARCHITECTURE_PLAN.md), [tasks](docs/TASKS.md),
[clean-room policy](docs/adr/0005-license-and-clean-room-policy.md) and
[work modes](docs/adr/0006-work-modes.md). Keep core independent of the UI,
mutations in undoable document commands, and Setup/Animate semantics distinct.
Do not weaken staged publication, rollback or project recovery protections.
There are three accepted baseline editor deprecation warnings; identify them
explicitly and introduce none. This is not permission to hide new warnings.

## Current local preparation

See [T836 workflow](docs/prealpha-workflow.md) and [Jade](docs/jade-sentinel.md). Keep controlled test runs separate from Windows release builds; preserve the first GIF sharing-lock failure if it recurs. Do not discard old evidence/caches/unknown autosaves in a cleanup. Public v0.2.0-alpha.1 is accepted historical publication; local T813–T836 features are not implicitly public. No new release/version is authorized by validation.
