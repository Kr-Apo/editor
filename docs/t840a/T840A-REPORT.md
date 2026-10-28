# T840A — Mission report

Date: 2026-10-23. Documentary/experimental mission only. This follows unpublished
local T839, not GitHub main. No new solver, runtime format or product behavior.

## Checkouts and branches

Existing checked paths are the isolated sibling checkouts `work/t839/editor` and
`work/t839/runtime` established in T839; the directory name is not the branch.
Both were clean at the exact requested heads and remotes before work:

| Repository | Starting local branch | Starting HEAD | Remote |
|---|---|---|---|
| Editor | dev/t839-editor-runtime-baseline | ac8ace04cfb34f9ffe812fdde0bb8b9058f90303 | https://github.com/Kr-Apo/editor.git |
| Runtime | dev/t839-runtime-parity | d720d6a21c0381e8417390bcac02f97c481432b5 | https://github.com/Kr-Apo/runtime.git |

Both create `dev/t840a-physics-replay-contract` from those heads. All initial refs
are recorded and checked unchanged, including T839 branches, main, remote refs
and Editor v0.1.5. Final local commit IDs/statuses/diffs are in the external
FINAL-GIT.json delivery receipt; the copy of this report there appends exact
HEADs after committing (avoiding a self-referential commit ID in the source).

## Delivered files

- Five documents here: normative replay proposal, all parity results, architecture
  recommendation, T840B steps and this report.
- `export/tests/t840a_physics.rs`: actual core references/export driver, repeated
  replay/reset, independent instances, force-only shared-clock isolation and fault
  characterization. Optional outputs create new destinations only.
- `export/tests/fixtures/t840a/`: three byte-identical T839 author JSON copies,
  protected against Git newline changes by local .gitattributes.
- `editor/src/t840a_physics_tests.rs`: actual headless AppState lifecycle tests.
  Only edit to an existing product source is its cfg(test) module registration
  in main.rs; it does not compile into the shipping application.
- Runtime `scripts/t840a/`: measurement driver, predeclared thresholds, executable
  calendar specification, clock experiment and eleven characterization tests;
  `docs/t840a/README.md` explains their isolated use.

No `core/src`, renderer, exporter implementation, transport implementation,
dependency manifest/lockfile, version, author schema or preset was modified.
Existing `target` activation, v1/v2 reading, ambiguous-bone rejection and
Physics/Pixi/CPU/PathPosition guardrails stay intact. Historical files/oracles,
projects/assets, archived evidence and distribution ZIPs remain protected.

## Findings and recommendation

60 cross-language rows, 34,539 sampled poses: **29 within limits / 31 divergences**.
All valid-input traces repeat exactly within each language and remain finite.
Metrics have separate units and thresholds declared before execution; no golden
replacement or tolerance relaxation. Tests asserting a known divergence prove
characterization, not product conformance.

Confirmed: shared Rust remainder couples otherwise independent springs; zero-dt
backlog and mix/activation policies differ; real Editor document replacement can
reuse previous physical state when IDs match. Seek/pause/play semantics differ
between real AppState and TS player/playback. Fault handling differs too. These
remain documented, not fixed prematurely.

A test-only common 120Hz calendar, with anchor priming and per-tick input sample,
produces identical TS tick histories at 30/60/120/144 display Hz and agrees with
canonical Rust under all declared thresholds for the three one-second fixtures.
Its .5s hitch drains to the same 60-tick prefix. This does not qualify arbitrary
external motion, full transports, continuous looping, long-run bridge precision
or numerical fault recovery. It is no new product adapter.

Recommendation: option B at state/time ownership boundaries, implemented through
a small option C controller behind an explicit future ReplayV1 profile. Keep
LegacyT839 recognizable; do not copy its proven clock-design weakness as option A.
First T840B step: targeted context/instance safety, then guarded clock/scheduling.

## Actual validation and limits

Environment: Windows 11 x64 MSVC, pinned Rust/cargo 1.95.0, Node 24.18.0,
npm 11.16.0, Git 2.55.0.windows.3. Rust build cache reused only as a cache; source
comes from the accepted local branches. Node dependencies already installed and
locked by T839; no dependency change or claim of a fresh npm install here.

- Baseline actually rerun before additions: Rust **2162 passed, 4 ignored**;
  Runtime **231 passed**, build/typecheck passed. No historical result relabeled.
- Final workspace `cargo test --workspace --locked --all-targets --offline --
  --test-threads=4`: **2168 passed, 0 failed, 4 historical ignores** (six new Rust
  tests: three export/core characterizations and three real AppState tests).
- `cargo fmt --all -- --check`; workspace Clippy locked/all-targets/offline;
  workspace build locked/offline; core wasm32-unknown-unknown check; xtask
  docs-check: PASS. Documentation is rechecked after all five files are written.
- Runtime `npm run build`, `npm run typecheck`, `npm test`: PASS, historical
  **231 tests** unchanged. Syntax check and separate characterization run: PASS,
  **11 additional tests**, not silently inserted into the historical count.
- Native export references and both numeric comparison tools executed; source
  bytes checked; complete input plans and all trace arrays retained.
- Existing warnings preserved. Two new useless-Vec warnings in the test data
  builder were removed using equivalent arrays; targeted tests and Clippy rerun.
  No historical warning cleanup. Final warning comparison is in delivery evidence.
- T839 WebGL125, physical Windows GUI, clean-PC, graphical/artist tests and real
  AI/model/provider calls: **NOT_RUN in T840A**. Runtime/render product code and
  old pixel fixtures are unchanged; that is preservation, not new GPU PASS.

Evidence directory: isolated `outputs/T840A-Physics-Replay-Contract`, including
initial/final Git, exact commands/exit codes, all attempts, initial thresholds,
input hashes, reference-v3, measurements-v3, clock experiment, logs, patches and
checksums. No source project's PhysicsState is saved or autosaved by these tests.

No push, merge, tag, release, GitHub Actions change, npm publication, packaging or
solver fix occurred. Stop for review before T840B.
