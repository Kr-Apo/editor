# Build and validate the local T836 candidate

This is source/candidate preparation, not a publication procedure. Select the reviewed clean `chore/t836-prealpha-cleanup` commit from CHECKPOINT.json. Local T813–T836 changes are not assumed available on public main. Do not amend, merge, push, tag or publish as part of validation.

Rust1.95.0 is pinned. Windows x64 needs MSVC C++ tools and Windows SDK. Obtain the committed `community-plugins` submodule at its pinned revision for importer integration fixtures; do not silently substitute another package. Python3 is only needed for optional local evidence scripts; Windows users need no toolchain to open the EXE. No model or FFmpeg is bundled.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --all-targets -- --test-threads=4
cargo clippy --workspace --locked --all-targets
cargo build --workspace --locked
cargo check --locked -p ankhimate-core --target wasm32-unknown-unknown
cargo run --locked -p xtask -- docs-check
```

Run the controlled workspace suite without a concurrent Windows release build. Keep all logs and four historical ignored tests. If GIF Windows error32 recurs, retain the first failure and exact test, compare unchanged source, replay in isolation, then run the controlled full suite without a build in parallel. Never weaken assertions/timeouts/ignores. No fixed root cause or repair is claimed by cleanup.

For a local portable executable in PowerShell (after the suite):

```powershell
$env:RUSTFLAGS='-C target-feature=+crt-static -C strip=symbols -C debuginfo=0'
cargo build --locked --release -p ankhimate-editor
```

The delivery build additionally remaps build paths and validates PE architecture/DLL imports. Plain developer builds do not automatically have those properties. Close a running editor before overwriting its EXE. Build outputs belong in a separate disposable cache; never purge historical deliveries/checkpoints or autosaves. Product code/dependencies are unchanged by T836.

The local Windows-Test-Candidate includes only current EXE, authorized Jade project/assets, licenses and instructions; no PSD sources, Crapaud, models, credentials, caches or browser runtime are required. Source images are already present as project asset bytes; duplicate original PNGs are excluded. Verify every SHA before copying for tests. No new ZIP/alpha packaging is performed. The former M006/M007 packager is [retired source text](../tools/history/package-m0072-alpha.ps1.txt), not the current entry point. See [local workflow](prealpha-workflow.md).
