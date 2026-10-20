# Pending manual qualification register

All **local T-813–T-816 GUI results remain PENDING_MANUAL / PENDING MANUAL REVIEW**.
This register requests no action now. The maintainer chose to continue development
before grouped tests. No editor/browser/inference session was run by T-817.

Use the **T-819 cumulative executable** for the grouped campaign; older binaries
are retained for targeted comparisons, not substitutes. Internal version remains
0.2.0-alpha.1 and cannot identify a candidate by itself. Check the SHA before use.

| Phase | Expected original artifact / SHA-256 | Existing checklist (delivery-relative location) | Unrun scenarios / risks | Human result |
|---|---|---|---|---|
| T-813 | Ankhimate-T813-dev.exe — bb3058589ffb3448d2df755b95c7d43b8c651c7701d845457648ab4396e93a73 | outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md | Save/Save As/relink, unknown inputs, failed writes, idle autosave/recovery; use copies | PENDING_MANUAL |
| T-814 | Ankhimate-T814-dev.exe — 4d1b897c5bc646d068ce3267b3bb5830dd2c58c3452f1cf3abe78fb4624f5320 | outputs/T814-Animation-UX/MANUAL-CHECKLIST.md | Key selection, numeric tangents, Apply/Cancel, graph framing/focus and Undo | PENDING_MANUAL |
| T-815 | Ankhimate-T815-dev.exe — bdff15865d02fc945e1daf2e973e2364f867a190da5f99b45de37e0b05e5f9c2 | outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md | Exact positions/UV/polygons, Set/Add, context invalidation/weighted meshes | PENDING_MANUAL |
| T-816 | Ankhimate-T816-dev.exe — fc8268863ce24118818caafcdc2d91f00b3f91a7ef5203ade03de0ba323f5348 | outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md and MASTER-MANUAL-QA.md | IK target/span/bend/mix, paint targeting/locks, real pointer/dock/themes, beginner exercise, save/reopen | PENDING_MANUAL |

These `outputs/` paths refer to the maintainer's local delivery alongside `work/`,
not files distributed by this source-only documentary commit. Do not create broken
repository-relative links to unpublished evidence. Exact checkpoint/validation
metadata are in each local CHECKPOINT.json/VALIDATION.json and the T-817 report.
The master checklist references the prior ones; this register does not duplicate them.

## Historical human evidence — retain its narrow scope

- T-805.1 recorded an actual Qwen/NInfer conversation/approval and Windows layout
  checks on its own development binary. Six-provider mocks do not qualify cloud models.
- Later maintainer reports recorded targeted Jade vision/bone creation/region
  assembly/mesh smoothing, Undo/Redo and Save/Reload, plus H.264 1080p/60 FPS/2 s/
  120-frame MP4, cancel/no-overwrite. Exact Qwen build was not supplied; modified
  Jade was not saved and is deliberately not distributed. This is MIXED historical
  evidence, not a reproducible final project or blanket current-model certification.
- T-811/T-812 publication authorization accepted the exact integrated artifacts.
  T-812 records public checks and hashes. Older candidate-pending statements are
  temporally stale for those published artifacts, but do not qualify newer binaries.
- RC2's separate-PC six-control maintainer report is retained with RC2 evidence;
  the recorded Windows wording is insufficient for a precise build-number claim.
  It does not certify the newer T-813–T-816 executables or all hardware.
- Other GUI/GPU captures remain historical scenario evidence; T-817 did not rerun them.

## Recording the later campaign

Record actual binary SHA, Windows build, GPU/driver, DPI/theme, disposable sample,
steps, PASS/FAIL/NOT RUN and reproducible expected/actual result. A missing run
stays pending. Keep original projects/assets/recoveries intact. Reproduce B03's
bone-name versus icon context target deliberately; do not infer a fix from a workaround.
No result should authorize an automatic release or close the development roadmap.

## T-818 — cumulative main candidate, PENDING_MANUAL

Artifact: `outputs/T818-Global-Diagnostics/Ankhimate-T818-dev.exe`.
Read its exact SHA-256 in the delivery CHECKPOINT.json / WINDOWS-BUILD.json;
version 0.2.0-alpha.1 alone is insufficient. Earlier executables and all T-813
through T-816 checklists remain intact. No action is requested during development.
Follow the 14-step [diagnostics checklist](document-diagnostics.md#deferred-manual-checklist)
and the prior cumulative campaign. No desktop GUI, browser or inference session
was executed for T-818; in-memory egui tests are automated evidence only.

## T-819 — cumulative main candidate, PENDING_MANUAL

Artifact: `outputs/T819-Cross-Project-Transfer/Ankhimate-T819-dev.exe`.
Use its exact delivery CHECKPOINT.json / WINDOWS-BUILD.json SHA; internal version
0.2.0-alpha.1 alone is insufficient. Follow the 18-step transfer checklist plus
all retained T-813–T-818 checklists. Native egui tests are not desktop qualification.
No test is requested now and no previous manual gate is marked PASS. Previous
binaries, including T-818 SHA dcbc852f0d39670acb82ca0fbc844dd264d683a1d17478d23cf8178bc9a3dec0,
remain available for comparison. Source/destination fixtures are disposable copies.
