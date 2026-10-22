# Build the local v0.1.5 candidate

Use pinned Rust1.95, MSVC/Windows SDK for x64, and the pinned community-plugins submodule. Clone the approved future Editor fork only after its URL exists, checkout the exact source commit, and initialize submodules recursively. Current local source is identified by CHECKPOINT.json in the T838 delivery; no new public clone URL is asserted.

```powershell
cargo fmt --all -- --check
cargo test --workspace --locked --all-targets -- --test-threads=4
cargo clippy --workspace --locked --all-targets
cargo build --workspace --locked
cargo check --locked -p ankhimate-core --target wasm32-unknown-unknown
cargo run --locked -p xtask -- docs-check
$env:ANKHIMATE_SOURCE_COMMIT = (git rev-parse HEAD).Trim()
cargo build --release --locked -p ankhimate-editor
```

Build only from a clean exact commit when assigning that commit identity. If ANKHIMATE_SOURCE_COMMIT is omitted, About honestly says provenance was not recorded; it never invents a hash. The delivered Windows build injects the final committed SHA and validates its presence. The reference packaging build also uses static CRT, path remapping, stripped debug data and no UPX. No PE VERSIONINFO resource existed; Cargo identity/About/manifests are authoritative, not a fabricated Windows resource claim.

Package into a new folder/ZIP, preserve all legal notices and source offers, copy Jade and its five assets unchanged, supply English user guides, checksum every file, extract into a new directory and compare every path/size/hash. No .cmd launcher, source/cache/model/runtime demo in the user folder. The qualification folder retains source references, reports and459 pending historical controls. Release/publication operations are separate and require approval. Do not run release Windows compilation concurrently with the full GIF test suite.
