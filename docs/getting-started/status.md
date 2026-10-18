---
title: Status and support
description: Understand Ankhimate's pre-alpha maturity, status labels, supported targets, and bug-reporting expectations.
---

# Status and support

Status labels have precise meanings throughout this book:

| Label | Meaning |
|---|---|
| **Current** | Implemented and covered by the current repository. |
| **Partial** | Useful behavior exists, but the stated workflow has known gaps. |
| **Experimental** | Implemented, but its contract or UX may change substantially. |
| **Planned** | Recorded work; not available to users. |
| **Not supported** | Deliberately unavailable or outside this repository. |

**Current:** local Windows x64 alpha candidate, not a public release or a 1.0.
The core compiles for `wasm32`; that is not a browser editor. Other desktop
targets are intended, not newly packaged or qualified here.

**Partial:** production polish. Settings/keymap persistence, safe saving,
isolated autosave/recovery and export preflight are implemented. Full
localization, panic-time emergency dumps, accessibility and performance gates
remain open. Autosave is recovery assistance, not a backup.

**Current within scope:** PNG sequences and spritesheets use the shared CPU
renderer and staged publication (default skin, Normal blending, no physics).
**Experimental:** advanced combinations beyond the qualified alpha cases.
**Planned:** video export and the browser editor, deferred after alpha.
Read [alpha-scope](../alpha-scope.md) for target-specific limitations and
[alpha-validation](../alpha-validation.md) for evidence.

**Not supported here:** game-side runtime implementations. They live in the
[separate runtime repository](https://github.com/Ankhimate/runtime).

Report reproducible bugs at the [editor issue tracker](https://github.com/Ankhimate/editor/issues).
Include the OS, GPU/backend, commit or release, steps, expected and observed
behavior, and a minimal `.ankh` file when it is safe to share. Remove personal paths
and private artwork first.
