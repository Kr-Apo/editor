# État local T837 — 22 octobre 2026

Ce checkout contient le correctif ciblé de persistance Recovery T837 sur T836 `1d3cc13bd5fda93c541dc8698918dedcdd146290`, avec documentation utilisateur et deux livraisons locales séparées. Aucun format, runtime, numéro de version ou artefact public n’est modifié. Les tests réellement exécutés et le commit final figurent dans la livraison T837. L’Alpha publique v0.2.0-alpha.1 et RC2 conservent leurs propres preuves.

Les **444 contrôles manuels historiques restent PENDING MANUAL REVIEW** ; T837 ajoute 15 contrôles distincts, eux aussi en attente. Consultez [le catalogue utilisateur](FEATURES.md), [le démarrage](GETTING-STARTED.md), [la compatibilité](COMPATIBILITY.md) et [la récupération](project-recovery.md). Les 101 IDs conservent 62 COMPLETE, 33 PARTIAL, 1 ABSENT, 3 SUPERSEDED, 2 DEFERRED. T208/T209/T211/T404/T502 restent PARTIAL, sans reclassement par effet d’une mission récente.

<details>
<summary>Référence antérieure conservée — état daté T836 ou avant</summary>

<details>
<summary>Retained historical campaign instructions (source artifact requirements)</summary>

# Ankhimate — Cumulative manual validation pending

T-834 local addendum: **Safe FFD Key Clipboard — COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** in its same-document/exact-owned-mesh scope. See [the contract](ffd-key-clipboard.md). T-209/T-404 remain PARTIAL. The T-832 101-ID matrix/categories, its 352 manual scenarios and the 28 T-833 controls stay unchanged; T-834 adds 37 pending controls. No GUI/inference/publication.

T-833 local addendum: selected-bone offsets are implemented; see [the contract](selected-bone-key-offset.md). T-211 and T-905 remain PARTIAL; the frozen T-832 101-ID matrix and original 352 manual scenarios stay unchanged. T-833 adds 28 pending controls. No GUI/inference/public operation.

T-832 reconciliation baseline (retained): **2026-10-21**. Product audited at T-831
`4e4d50b0aa5b7ac160b204540afa878f66fa4d61`, parent T-830
`12252e476b95f9d2ac250d5d0746c589c10269b3`. **T-832 changes documentation only.**

Public Alpha `v0.2.0-alpha.1` remains the exact T-811/T-812 artifact: editor
`9233b8e888c811e7c71300101f865a9e74b3f489`, runtime
`c4c38df269cfcb30f941221454b7202f40d3fa5b`. T-813–T-831 refinements are local
candidates, even though their internal version string remains 0.2.0-alpha.1.
No GUI/inference/public operation in T-832. No local Windows control is marked PASS.
Accepted **T-831** evidence is 2,058 Rust passes / 0 failures / 4 existing ignores;
**T-832 does not rerun that suite** and adds no tests or executable. Relative to
T-816/T-817's 1,596 passes, the historical registration increase is 462, not 462
new tests executed by this documentation mission.

There are **352 pending control instances**, including 351 numbered original steps from 18 checklists and inherited B03. None is PASS. No current action requested.

| Family | Pending instances |
|---|---|
| Project / Save / Recovery | 40 |
| PSD and assets | 22 |
| Rigging | 23 |
| Meshes / weights / FFD | 38 |
| Animations | 58 |
| Graph Editor and Dopesheet | 58 |
| Path constraints | 23 |
| CPU export | 11 |
| Diagnostics and safety | 42 |
| Perceived performance | 35 |
| Compatibility and publication | 2 |

The [full self-contained map](cumulative-manual-qa-map.md) retains source IDs, exact original scenarios, fixtures, required cumulative binary, prerequisites, expected outcomes, status and possible grouping. Repeated Save/Undo/open steps may share setup later, but no source coverage is removed. T-816 master QA routes to the individual checks and is not counted again; T-817 introduced no product checklist.

Use only the exact T-831 cumulative EXE,35,416,576 bytes, SHA-256 `0ba98d6dac970f305f4691f938d8978db37fd034b4a647805e4efb266cdaa3af`. The internal0.2.0-alpha.1 number is not identity. Retain earlier binaries for comparisons on separate copies. Use complete copied Jade/Crapaud/technical fixtures with assets; preserve originals, backups and recovery sidecars. Some closed/index/invalid-layout scenarios need an explicitly prepared compatible technical copy before execution.

No model inference or provider qualification is performed or requested now. T-806/807/808/809 historical narrow reports do not establish general visual/model acceptance. B03 stays pending physical target/focus reproduction. A future campaign must record OS/GPU/driver/DPI/theme, artifact/fixture hashes and expected/actual PASS/FAIL/NOT RUN. See [priorities](roadmap-priorities.md) and [exchange compatibility](format-release-compatibility.md).

## T-833 — 28 additional controls

All are **PENDING MANUAL REVIEW**. The 352 baseline controls remain verbatim in [the retained map](cumulative-manual-qa-map.md). The new list accompanies the T-833 binary; no Windows/GUI test or inference has been executed. It covers explicit selection, preview, Cancel, Apply, history, collisions, locks, FPS changes, Path preservation and Save/reopen. The combined population is 380 scenario instances, not unique test cases or a completion percentage.

## T-834 — 37 additional pending controls

All are **PENDING MANUAL REVIEW** in the T-834 delivery checklist. Prior 352 T-832 +28 T-833 controls remain unchanged and pending. Cumulative population:417 scenario instances, not unique tests or a completion percentage. Exercise Copy/Paste, exact mesh/row, collisions, topology/shared refusals, cancellation, three history cycles, Save/reopen and scalar/pose non-regression only on disposable fixtures. No manual execution requested during this mission.

## T-835 — Animation ordering

27 additional controls, all **PENDING MANUAL REVIEW**, are listed in the local T-835 delivery. Prior 417 instances through T-834 remain pending and intact; cumulative population **444 scenario instances** (not unique tests or a completion percentage). Test disposable projects, exact candidate binary and assets, native row context actions, boundaries, active clip, interleaved history, FFD/PathPosition and all author save profiles. No GUI or inference execution was performed for T-835. See [manager steps](animation-manager.md).

</details>

</details>
