---
title: Install and make the first save
description: Distinguish the public alpha from the local test candidate and start safely with Jade Sentinel.
---

# Install and make the first save

The accepted public v0.2.0-alpha.1 has its own frozen archive. The local T836 Windows-Test-Candidate is a separate unsigned development folder, identified by commit and SHA, not its unchanged internal version. It includes the EXE, authorized Jade project/assets, notices and README; no Rust/Node/model is needed to open/play/save. Keep the full folder, work on a copy, then **File → Save As** before edits.

[Jade workflow](../jade-sentinel.md), [compatibility](../release-compatibility.md), [source build](../CANDIDATE_BUILD.md). Existing technical samples/PSDs remain for focused tests, not additional primary character demos. Preserve `.ankh` plus adjacent `.assets`, autosaves and backups. Current physical Windows testing remains pending.

From a reviewed source checkout with pinned Rust1.95/MSVC and pinned community plugins: `cargo run --locked -p ankhimate-editor`. Close the app before replacing its EXE. Website builds remain `bun install --frozen-lockfile`, `bun run check`, `bun run build`; these are development commands, not certification of an editor release.
