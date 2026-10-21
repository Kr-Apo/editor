# Cumulative deferred Windows QA map — T-813–T-831

Audit date: **2026-10-21**. Product frozen at T-831
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

**352 control instances: 351 numbered controls from 18 checklists plus inherited B03. All PENDING MANUAL REVIEW. No current action requested.**

Main cumulative executable: `Ankhimate-T831-dev.exe`, **35,416,576 bytes**, SHA-256 `0ba98d6dac970f305f4691f938d8978db37fd034b4a647805e4efb266cdaa3af`. Verify CHECKPOINT/WINDOWS-BUILD/local SHA, not the unchanged internal version. Older exact binaries are retained only for focused comparison on independent copies. Never open a new PathPosition file in an old binary expecting edits; refusal is the expected forward boundary.

The T-816 master guide routes to its individual checklists; it is not an extra numbered population. T-817 is documentary and adds no product checklist. Older historical acceptance applies only to its exact released artifacts, not to this cumulative candidate. No external model, GPU benchmark or remote operation is requested now.

| Source checkpoint | Numbered controls | Original checklist / SHA256 |
|---|---|---|
| T-813 | 9 | outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md / d7f35e65772f88f3ecf70ae7c79889215fd06bd1b9c68f2dbec3e46290d3952e |
| T-814 | 11 | outputs/T814-Animation-UX/MANUAL-CHECKLIST.md / 7eac99fc172b513f5241de7ecd7359955b8bfe7a5331ca0246e630e0b9795513 |
| T-815 | 15 | outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md / a1ff4df5f0e3929ebceac550956ff18786714501fc13a0d7038461711f024f98 |
| T-816 | 15 | outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md / bf0197f79bf4cdcb3a649bfa7bb639bce167a7bc00970716be080e10181995c7 |
| T-818 | 14 | outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md / 3389b90d139eaa7be40a2864c7c29066c4ee5213d1f7050cfdf2bc9f81fe9daa |
| T-819 | 18 | outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md / 210bbbebb869cd5190ca142cbc16ae06f31698bc6c07efca72db30234f1b2504 |
| T-820 | 20 | outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md / b53dec5cfe3f5aff570d0d37dd1359b4e8d9470785fe9d942cb5de211100bf5f |
| T-821 | 24 | outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md / 2bcb128668dd81d2f6f8a1add247ff067dfa5a677f7be1ac48de3a5310ba6d4b |
| T-822 | 26 | outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md / 0296edb3114660670ce674d2b0684547b24fcd0522ee318013d136d155315206 |
| T-823 | 15 | outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md / 7fd562a3f4c2aff82f1c48df8a69d19db3992b2b43e56e4eb03d736c5cd8d354 |
| T-824 | 20 | outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md / 526aaffbf2a82ca1e123bcdf9269d7380179c5619ed24bfc2d4fa9323d7c6f3e |
| T-825 | 22 | outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md / 480c1ff95f406022465e52c6ca4443184ee3a7f963357eb00c6f28f9e19f4dc3 |
| T-826 | 24 | outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md / e4407539581da2d28afc1aa7726263bec3b6233d5824bd100d7bf30141a7f532 |
| T-827 | 20 | outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md / 21a0c61788c96129e696984e7b0a85a66a212a8e98527242a2cd42b6b033e596 |
| T-828 | 22 | outputs/T828-FPS-History/MANUAL-CHECKLIST.md / 9605fbd2ed4b321b84b3de552f611ba0a0e6e37cf9ef96765d76bbbe888d8ec4 |
| T-829 | 21 | outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md / b1c984ad1cd0b419b834fa6bbf079b5e4034c53b4b7327ecea8b59fb9ad25cf5 |
| T-830 | 27 | outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md / fdecce9f3f270ac3a2d98a15053c53e5c71e4e5a253c993210e4f1381b2d933f |
| T-831 | 28 | outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md / a1d68e583eb2ea912c7264c8643a4244afa33368498bfbede131796a7c6f6f0c |

## Project / Save / Recovery

### T813-M01

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Dupliquer `Manual-Samples/Jade-Sentinel` dans un dossier de test jetable, avec son dossier `Sentinelle.assets`. Ouvrir `Sentinelle.ankh` dans l'EXE T-813. Vérifier les images.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T813-M02

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: En Setup, modifier légèrement un bone. En Animate, ajouter/modifier une clé d'un clip existant. Faire Undo/Redo trois fois : poses et clés doivent revenir exactement. Vérifier le statut modifié ; ne pas attendre une persistance de l'historique après fermeture.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T813-M03

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: **Save As** vers un autre dossier avec espaces/accents. Le chemin actif ne doit changer qu'après succès. Fermer/réouvrir : vérifier images, poses et animations.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T813-M04

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Modifier la copie, puis demander New, Open et Close : vérifier successivement **Cancel** (document/sélection/historique conservés), **Save** (action après réussite), **Discard** (uniquement sur la copie jetable). Annuler aussi un sélecteur Save As.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T813-M05

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Sur une autre copie fermée, déplacer temporairement **un PNG de son `.assets`** dans un dossier témoin, sans le supprimer. Ouvrir la copie : avertissement d'image manquante. Après une petite modification, Save/Save As doivent refuser, garder le chemin et les changements, sans toucher le dernier fichier sur disque. Garder l'application ouverte.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T813-M06

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: En Setup, dans Assets, retrouver l'image et utiliser l'icône Relink (tooltip « Point this asset at a different file, keeping its name »). Choisir le PNG déplacé. Tester Undo/Redo, puis Save As et réouverture. L'image et ses références doivent être conservées.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T813-M07

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Sur une copie complète, régler Autosave à 30 s, modifier et attendre environ 40 s sans playback ni interaction. Vérifier le sidecar `.ankh.autosave` et son dossier d'assets : le primaire n'a pas été réécrit et le document reste modifié. Ne pas poursuivre une longue session avec une image manquante : l'autosave est alors refusée jusqu'au relink.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T813-M08

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ouvrir `Manual-Samples/Recovery-Sandbox/Interrupted.ankh` **dans une copie jetable** : le primaire est volontairement invalide. Choisir Ignore ; les fichiers doivent rester. Rouvrir et choisir Recover. Vérifier Jade, le message de récupération et l'état modifié. Tester Close → Cancel, puis Save As sous un nouveau nom et réouvrir ce résultat. Le primaire artificiellement corrompu et le sidecar doivent rester intacts.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T814-M10

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Supprimer/Undo/Redo, changer clip et document ; brouillon périmé disparu, aucune commande envoyée à une autre clé. Sauvegarder puis rouvrir la copie.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T814-M11

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Comparer courbe, pose, boucle et sampling visuel avant/après réouverture ; compléter également les contrôles Save/Save As/autosave/recovery de T-813.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T815-M14

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As sur copie, fermer/réouvrir ; géométrie/UV/assets et animations conservés.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T816-M08

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: In Animate select curl. Confirm target animation, scrub; Setup property controls stay disabled. Edit a target key with Auto-key, Undo/Redo, Save As/reopen a copy.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T816-M14

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As/reopen the copy. Confirm weights/mesh/assets; session locks may reset. Switch Animate: weight editing is disabled and does not mutate the project.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T818-M13

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As to a new location, reopen, and check authored data/assets plus recomputed diagnostics.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T819-M15

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As to a fresh Unicode/spaced folder.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T819-M16

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Reopen the destination and check data/images/animation.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T819-M17

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Rename the throwaway source folder out of access; reopen destination again to verify independence.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T819-M18

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify T818 diagnostics refresh and stale navigation protections; rerun the older Save/recovery, keys/tangents, geometry and rigging checklists.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T820-M19

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As a copied project and reopen; original project/assets remain untouched.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T820-M20

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify stored coordinates/curve and default Broken mode after reopen; mode was not saved per key. Complete cumulative T813–T819 qualification too.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T821-M20

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As to a fresh copied folder.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T821-M21

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Close and reopen the saved copy.
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T822-M21

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As to fresh folder
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T822-M22

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Close and reopen with .assets
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T823-M13

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As into a fresh folder; close/reopen with .assets
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T823-M15

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Run cumulative T813–T822 save/recovery/animation/geometry/PSD controls
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T824-M18

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save/reload copy and Undo/Redo: author keys/geometry/assets unchanged
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T825-M20

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As and reopen the copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T826-M21

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As and reopen the copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T826-M22

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify animations after reopen
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T827-M17

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save and reopen a copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T828-M17

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save a copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T828-M18

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Close and reopen the copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T828-M19

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify saved FPS
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T828-M20

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check dirty state across Undo/Redo/Save
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T829-M20

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As/reopen on copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T830-M24

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As disposable copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T830-M25

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Close and reopen
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T831-M23

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Save As on a copy
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

### T831-M24

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Close and reopen
- Expected: Saved copy reopens with matching authored data/assets; failures/Cancel preserve the current document and valid files.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: SAVE-REOPEN / history context differs; retain every source condition.

## PSD and assets

### T822-M01

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable PSD-derived project copy
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T822-M02

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect existing images
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M03

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect rig and animation clips
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M04

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Load modified PSD
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M05

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Choose Update existing PSD images (keep rig)
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M06

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect full added/missing paths
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M07

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select old missing image explicitly
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M08

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select actual new added path explicitly
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M09

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Prepare; review mapping, dimensions, provenance and whole batch
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M10

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cancel mapping/preview
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M11

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify no document, selection or history mutation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M12

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat Prepare and confirm Apply
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M13

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify displayed image
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M14

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify bones and slots
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M15

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify transforms and pivots
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M16

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify animations and curves
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M17

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test compatible image dimension change
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M18

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test duplicate/ambiguous/unsupported target
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M20

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perform three full Undo/Redo cycles
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T822-M23

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Preview same PSD again
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M24

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify new path recognized without unresolved rename
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M26

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Execute cumulative T-813–T-821 manual controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Rigging

### T816-M01

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Copy Samples/Tentacle into a writable practice directory; open tentacle.ankh.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T816-M02

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: In Setup, inspect root, tentacle1…8 and separate tentacle-target in Hierarchy.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M03

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select a chain bone, then target, then constraint; View → Properties if hidden.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M04

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify 8 bones, root, ordered names, FABRIK, current evaluated world span and target.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M05

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Translate the target near/far with T. Observe the chain; no NaN/jump/crash.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M06

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo the target edit; confirm the exact prior pose. Repeat three times.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T816-M07

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try Mix 0/0.5/1, Flip bend, Stiffness and existing Softness/Stretch in Setup. Judge results, not a unique-solution promise. Undo and restore defaults.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T816-M09

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Copy Samples/Jade-Sentinel including Sentinelle.assets. Open the copy.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M10

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Setup → select a mesh slot → View → Weights → Activate Weight Paint (also try W).
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M11

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check active tool, influence name, Radius world units, Feather, Weight and modes.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M13

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Lightly paint Add/Subtract/Replace/Smooth on a disposable copy; verify no unwanted change to geometry/UV/binds. Check Direct selection separately. Undo/Redo each stroke.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T816-M15

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Follow docs/first-rig-in-10-minutes.md without reading source. Record obstacles, completion time (not guaranteed), theme/dock clipping, keyboard focus and clarity.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M01

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a copied Destination-B project.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T819-M02

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: File → Import from Ankhimate Project… → choose disposable Source-A.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M03

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Choose the source root `source`, and destination parent / New root deliberately.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M05

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cancel, verifying dirty state, selection, existing data and history unchanged.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M06

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat and Confirm reviewed transfer.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M07

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify new bones and hierarchy, with the expected parent and local transforms.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M08

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify default-skin slots/regions and relative draw order.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M09

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify both textures display and existing destination images remain intact.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M10

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select the distinct transferred `move_2` clip; check its axes, keys and Bezier behavior.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M13

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try Missing-Image-C (intentionally missing its sidecar).
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### B03-M01

- Introduced: T-817 record of earlier observation; original source: `work/t831/editor/docs/FEATURE_INVENTORY.md#bugs/gaps`.
- Fixture: Copied rig / Jade; no original edits.
- Required binary: Ankhimate-T831-dev.exe, retain older comparison candidate.
- Prerequisites: Physical Windows mouse; known hierarchy target/name/icon and keyboard focus; record DPI/theme.
- Scenario: Attempt the bone-name context menu, compare name versus icon/viewport target and focus; preserve the original observation without assuming reproduction.
- Expected: Record a precise reproduction or non-reproduction; icon workaround alone does not close B03.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: B03 inherited manual queue, not a newly reproduced defect.

## Meshes / weights / FFD

### T815-M01

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ouvrir une copie de Jade Sentinel dans Ankhimate-T815-dev.exe ; vérifier assets/poses.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T815-M02

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Setup : sélectionner un mesh réellement affiché dans son slot et Edit vertices.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M03

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cliquer un vertex ; vérifier type/nom/index/count et espace slot-bone local.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M04

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Lire les nombres exacts ; changer Local X puis Cancel, vérifier aucune mutation.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M05

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Changer X/Y puis Apply coordinates ; vérifier déplacement visuel attendu et UV intacts.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M06

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Vérifier les autres vertices, poids, influences, triangles et animation.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M07

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Faire trois Undo/Redo ; vérifier retour exact et absence de fusion avec un drag voisin.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T815-M08

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ctrl box/Shift toggle : Set X aligne tous, Add X décale tous ; nombre d'éléments explicite.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M11

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Path/bbox/clipping de test : sélectionner les points et Apply local ; Undo/Redo ; vérifier end-slot/closed/poids et ordre inchangés. Ne pas utiliser de projet original.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T815-M13

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Zoom au pointeur, pan/zoom bar et UV fit restent cohérents ; pas de recadrage pendant le geste. Confirmer que « No geometry snapping » est clair, sans promettre un snapping voisin, F universel ou pivot non livré.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M15

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Comparer les déformations à T-814 ; lisibilité à plusieurs tailles/DPI et clavier.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M01

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable FFD project (supplied Rigid/Weighted fixtures are technical samples).
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T821-M02

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select its compatible displayed mesh attachment.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M03

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Switch to Animate.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M04

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Choose impact with three Deform keys.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M06

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Click the middle key.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M07

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check shared selection feedback.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M08

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Drag horizontally to change its time.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M09

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Observe timing change during later human playback.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M10

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check other stored offsets remain unchanged.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M11

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Right-click; change a compatible FFD interpolation.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M12

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check intermediate behavior, including the documented empty-reset case.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M13

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Delete a key through Delete FFD key.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M14

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perform three Undo/Redo cycles and inspect the mesh/offsets.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T821-M17

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Switch skin or clip, including away/back.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M18

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ensure an old selection cannot edit a new key/index.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M22

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect mesh, keys, real image assets and influence data.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M24

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Complete cumulative T813–T820 checklists too.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M01

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Trace a simple Region converted to Mesh on a disposable copy
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T823-M02

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Retrace a rigid mesh without deformations
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M03

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Retrace a compatible weighted owned mesh
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M04

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Review old/new topology, influence/bind counts, unsampled source vertices and policy
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M05

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify bones, slots, rig placement and animation identity
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M06

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try inline and three-key Deform FFD dependencies
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M08

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try linked target and shared source, including a nondefault skin
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M10

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cancel preview and verify unchanged document, selection and Redo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T823-M11

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Run three Undo/Redo cycles and compare complete mesh state
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T823-M12

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Paint weights using the existing brush after retracing
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Animations

### T824-M01

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Onion disabled: current artwork unchanged and no ghosts
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M02

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Enable onion in Animate with an active clip
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M03

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Distinguish past/current/future poses
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M04

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Light and dark themes; check Past/Future labels and palette contrast
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M05

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Minimum and maximum opacity, including zero
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M06

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Falloff 0, .5 and 2; observe distance rank attenuation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M07

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: One through five steps per side
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M08

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Beginning/end: no repeated endpoint/current ghost and no wrap
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M09

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Frame mode at several FPS
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M10

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Key mode with authored timeline keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M11

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Irregular/duplicate/single/absent previous and next keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M12

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Selected-only with no, one and multiple bones
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M13

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: IK chain and weighted mesh: retain parent/influence motion
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M14

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Continuous scrubbing; no unexpected pose lag
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M16

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Setup/Animate: no ghost evaluation in Setup
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M17

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Disable onion again: normal current pose colors, alpha and order unchanged
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M19

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perceived responsiveness on a representative dense rig
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M20

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cumulative T-813–T-823 checklists on disposable project copies
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M01

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable project copy
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T828-M02

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Note its current FPS
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M03

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change FPS in the transport
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M04

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify the new value
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M05

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T828-M06

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify the previous value
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M07

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Redo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T828-M08

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify the new value again
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M09

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Edit an animation key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M10

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo key and FPS separately
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T828-M11

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perform two distinct FPS gestures
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M12

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cancel a numeric edit
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M13

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check frame stepping
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M14

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check snapping
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M15

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Onion Skinning by frames
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M16

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check key seconds and clip durations
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M22

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Replay relevant cumulative T813–T827 controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M01

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable animated project
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T830-M02

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Switch to Animate
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M03

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open Pose → Retime
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M04

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check initial values
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M05

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Enter Time Scale 2
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M06

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Review preview
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M07

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Cancel
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M08

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify no document mutation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M09

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat and Apply
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M10

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify key times and duration
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M11

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T830-M12

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Redo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T830-M13

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Time Scale 0.5
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M14

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Time Scale 1.25
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M15

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test positive frame offset
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M16

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test negative frame offset
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M18

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change FPS with draft open where the existing UI allows; otherwise record modal restriction
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M20

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test bone selection
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M21

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify excluded elements
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M22

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify curves
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M23

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Three Undo/Redo cycles
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T830-M26

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify times and animations
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M27

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat cumulative T813–T829 controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Graph Editor and Dopesheet

### T814-M01

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ouvrir une copie animée dans Ankhimate-T814-dev.exe ; vérifier pose/assets.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T814-M03

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Afficher le Graph Editor ; vérifier le même point sélectionné (anneau accent).
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T814-M05

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Dans Bezier tangents, modifier Out/In X/Y puis Apply ; relire les valeurs. X [0,1], Y négatif et >1 autorisés. Vérifier Cancel sans commande.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T814-M06

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Observer la courbe et le playback ; tester overshoot et handles épinglés, constantes, première clé et Linear/Stepped sans conversion implicite.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T814-M07

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo trois fois ; Apply successifs séparés, un drag de poignée = un Undo, deux drags distincts = deux Undo. Multiclés : consentement lisible aux quatre handles.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T814-M08

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cadrage stable pendant le geste, refit au relâchement ; zoom/scroll conservés. Badge snap: frames ; Alt = free, déplacement libre non recadré à une frame.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M01

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a copied project with three scalar keys and two Bezier segments.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T820-M02

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: In Animate select the central key in the existing shared selection.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M03

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Find textual Broken / Aligned / Mirrored in the existing tangents inspector.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M04

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Broken edits one handle independently; mode choice alone changes nothing.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M05

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Choose Aligned and drag the central incoming or outgoing handle in Graph.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M06

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Observe the opposite handle on the OTHER segment, not its segment sibling.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M07

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check alignment, accounting for anisotropic viewport scaling.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M08

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Choose Mirrored; verify opposite equal lengths in documented time/value units.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M09

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat with unequal durations, for example t=0,.2,2.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M12

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify overshoot Y survives valid edits; no silent clamp to [0,1].
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M13

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Edit numeric Incoming/Outgoing with an explicit Driver; opposite preview read-only.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M14

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Cancel draft edits; no document/history/Redo change.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T820-M15

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Apply numeric edits; one isolated Undo, no command per typed character.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T820-M16

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perform three Undo/Redo cycles; inspect BOTH segments and preserved selection policy.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T825-M01

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open an isolated animated project copy
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T825-M02

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Switch to Animate
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M03

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open the Graph Editor
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M04

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify existing single-key selection
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M05

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Drag a rectangle around several scalar key points
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M06

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify the intended keys/rings and untouched other keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M08

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ctrl-drag to add to the shared selection
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M09

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ctrl-click to toggle/remove; no subtractive marquee promised
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M10

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Drag in all four directions
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M11

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Empty replace clears / empty Ctrl-union preserves; click/jitter preserves
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M12

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Zoom and wheel/pinch/scroll navigation; view change cancels an active band
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M13

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Overlaid properties and keys at identical times/positions
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M14

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Grab an existing Bezier handle
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M15

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify handles and key drags win over the background
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M18

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change clip during/after a gesture; no old-index targeting
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M19

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo a key edit; selection creates no history or Redo loss
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T825-M21

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: When only selection changed, verify animation/tangents/assets unchanged
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M22

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Run relevant cumulative T-813–T-824 controls; include focus loss/outside release and light/dark contrast
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M01

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open an isolated copy of an animated project
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T826-M02

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open Graph Editor
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M03

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Display a translation and a rotation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M04

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check native Raw scales
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M05

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Enable Normalize
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M06

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check overlaid relative profiles
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M07

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Hover and read native ranges/units
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M08

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test a constant channel
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M09

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test large amplitudes
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M10

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test negative values
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M11

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Bezier overshoot and outside-frame notice
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M12

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test time zoom and pan
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M13

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test time Fit and stable relative framing
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M14

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Click a normalized key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M15

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Draw a normalized replace/Ctrl-union marquee
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M18

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Return to Raw
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M19

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify authored data stayed unchanged
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M20

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo after an actual Raw edit
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T826-M23

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Compare loaded-clip responsiveness
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M24

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Replay cumulative T-813–T-825 controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Path constraints

### T831-M01

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable project
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T831-M02

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Create/open a native polyline
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M03

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Create a five-bone chain
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M04

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Add a Path constraint
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M05

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect static pose
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M06

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Switch to Animate
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M07

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select the constraint
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M08

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Set first PathPosition key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M09

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Advance the playhead
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M10

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Set second key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M11

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Set third key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M12

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Scrub between keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M13

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check chain progression
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M14

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check spacing
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M15

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check bone rotation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M16

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check an open path
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M17

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check a closed path
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M18

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check negative position
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M19

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check out-of-range position
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M21

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Setup versus Animate
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M22

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat three Undo/Redo cycles
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T831-M25

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Inspect restored animation
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M28

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat relevant cumulative T-813–T-830 controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## CPU export

### T814-M02

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Animate : choisir une clé terminant un segment Bézier dans la dopesheet.
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T814-M04

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Sélectionner aussi depuis le graphe, Ctrl-clic et marquee dopesheet ; vérifier l'inspecteur et que le changement de vue n'altère ni pose ni historique.
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T818-M12

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open Export preflight; errors, warnings, consent, Cancel and destination checks must retain their behavior.
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T818-M14

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check no implicit correction, normalization, history entry or changed export; run earlier cumulative safety/animation/geometry/rigging checklists.
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T821-M05

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Find deform under the slot group in the Dopesheet.
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T825-M07

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify the same selection in Dopesheet
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T825-M17

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change selection in Dopesheet and check Graph/Properties
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T826-M16

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check shared Dopesheet selection
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T827-M10

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check shared Dopesheet selection
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T829-M04

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Dopesheet highlighting
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T829-M15

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change selection from Dopesheet
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

## Diagnostics and safety

### T813-M09

- Introduced: T-813; original source: `outputs/T813-Project-Reliability/MANUAL-CHECKLIST.md`.
- Fixture: Copy of T813 Manual-Samples/Jade-Sentinel and Recovery-Sandbox with assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T813-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Noter toute erreur avec l'action précise, la version Windows et le message affiché. Ne partager ni projet privé, ni credential, ni transcription IA. Aucun modèle requis.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T814-M09

- Introduced: T-814; original source: `outputs/T814-Animation-UX/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated scalar Bezier project / Jade.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T814-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: NaN/Inf/texte/X hors [0,1] refusés ; bone verrouillé et Setup non éditables. Déplacer une clé au-delà d'une autre : arrêt prudent du drag puis resélection correcte. Collision refusée ; vérifier aucun voisin modifié.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M09

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Champs mixtes blancs, axe laissé vide, négatif/décimal, NaN/Inf/texte refusés.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M10

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: UV editor : cliquer un UV, inspector suit ; Set/Add U/V [0,1], Cancel, Apply ; tester refus hors plage et absence de déplacement géométrique. Source texture, pas atlas bake.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T815-M12

- Introduced: T-815; original source: `outputs/T815-Viewport-Precision/MANUAL-CHECKLIST.md`.
- Fixture: Copied Jade mesh/UV/polygon/path project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T815-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Panneau masqué : changer outil/skin/mode puis revenir ; ancien brouillon/drag invalidé. Changer d'attachment/mesh, supprimer/Undo : aucune ancienne sélection sur un autre index.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T816-M12

- Introduced: T-816; original source: `outputs/T816-Rigging-Discoverability/MANUAL-CHECKLIST.md`.
- Fixture: Copied T816 Samples/Tentacle + Jade-Sentinel.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T816-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Pick several bound-bone rows and canvas bones; mesh stays targeted. Hover a lock, toggle it, verify locked/unlocked text and that a locked influence is unchanged.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M01

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a valid copied project.
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T818-M02

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Locate the bottom Diagnostics badge and its counts.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M03

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open and close the list; verify no dirty-state change.
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M04

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Shorten a copied clip so an existing key lies beyond its duration (reversible).
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M05

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Warning / `key-outside-clip`, clip, time and retained key.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M06

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Click the finding and check selection/playhead without a new Undo entry.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T818-M07

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Restore the duration or Undo the change.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T818-M08

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Wait for the warning to disappear; verify no automatic key deletion.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M09

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change clip, make an edit, then Undo/Redo.
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T818-M10

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify refresh/stale indication and updated counts without a camera-induced scan.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T818-M11

- Introduced: T-818; original source: `outputs/T818-Global-Diagnostics/MANUAL-CHECKLIST.md`.
- Fixture: Copied valid project + reversible induced diagnostic.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T818-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Keep the list open, delete an offender then Undo or open another copy; stale navigation must not select a reused object.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M04

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Prepare; inspect counts, LOCAL-coordinate warning, image and clip mappings.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M11

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Perform three Undo/Redo cycles; no half-import, unexpected existing-object change or stale focused selection.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M12

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat with an asset-name collision and inspect the proposed rename/reuse explicitly.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T819-M14

- Introduced: T-819; original source: `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md`.
- Fixture: Copied T819 Source-A / Destination-B fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T819-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify explicit refusal without mutation or lost Redo.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M10

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat with unequal amplitudes, e.g. values=0,100,101; impossible X must refuse.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M11

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check flat-horizontal joins and explicit refusal of unrepresentable vertical ones.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M17

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check first/last key, multi-selection, Linear/Stepped, extreme/invalid values, locked targets and raw rotation deltas >=180°: clear disabled/refusal reasons.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T820-M18

- Introduced: T-820; original source: `outputs/T820-Bezier-Linking/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar keys t=0,.2,2 with two Bezier segments.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T820-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Setup refusal versus Animate; interrupt a drag by Undo or clip change.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M15

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try an occupied exact time; verify explicit refusal and preserved Redo.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M16

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Try linked/shared/ambiguous or invalid topology; verify disabled reason/refusal.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M19

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Setup refuses while setup geometry tools retain their existing behavior.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T821-M23

- Introduced: T-821; original source: `outputs/T821-FFD-Key-Editing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T821 Rigid/Weighted FFD fixtures.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T821-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check T818 diagnostics refresh with no stale navigation.
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M19

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify refusal and intact Redo
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T822-M25

- Introduced: T-822; original source: `outputs/T822-PSD-Path-Resolution/MANUAL-CHECKLIST.md`.
- Fixture: Copied T822 Fixtures/PSDs original and renamed/moved/resized PSDs.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T822-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check refreshed T-818 diagnostics and stale ticket refusal
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M07

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify refusal without deleting/zeroing data or consuming Redo
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M09

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Lock slot/influence bones and verify refusal
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T823-M14

- Introduced: T-823; original source: `outputs/T823-Safe-Mesh-Retracing/MANUAL-CHECKLIST.md`.
- Fixture: Copied T823 Fixtures/Original.ankh + assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T823-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check diagnostics refresh and stale navigation refusal
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T824-M15

- Introduced: T-824; original source: `outputs/T824-Onion-Skin-Refinement/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated artwork with several known poses.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T824-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change clip, selection and document; no stale ghosts
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T825-M16

- Introduced: T-825; original source: `outputs/T825-Graph-Marquee/MANUAL-CHECKLIST.md`.
- Fixture: Copied scalar animated project.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T825-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Broken/Aligned/Mirrored and multi-key disabled states; stale draft cancellation
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T826-M17

- Introduced: T-826; original source: `outputs/T826-Normalized-Curve-Overlay/MANUAL-CHECKLIST.md`.
- Fixture: Copied mixed translation/rotation/scale fixture.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T826-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check permitted gestures and explicit blocked key/handle drags
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M15

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check stale-target refusal
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T828-M21

- Introduced: T-828; original source: `outputs/T828-FPS-History/MANUAL-CHECKLIST.md`.
- Fixture: Copied animated project with known FPS.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T828-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check T818 diagnostics
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M17

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify collisions and refusals
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T830-M19

- Introduced: T-830; original source: `outputs/T830-Numeric-Retime/MANUAL-CHECKLIST.md`.
- Fixture: Copied weighted/FFD animation, plus ordinary scalar clip.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T830-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify stale draft refusal before reprepare
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T831-M20

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check stale-target refusal
- Expected: Expected diagnostic/refusal; no unintended author mutation, stale selection, lost Redo or replaced output.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Perceived performance

### T827-M01

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open a disposable animated project
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T827-M02

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select one key
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M03

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select ten keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M04

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select several hundred keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M05

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select several thousand keys in the synthetic fixture
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M06

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check excessive stalls without assuming a global frame budget
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M07

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test marquee
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M08

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Ctrl-union
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M09

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Ctrl-click toggle
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M11

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check tangent selection/guards
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M12

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test Raw and Normalize View Only
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M13

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change clip
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M14

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Test homonymous tracks/targets
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M16

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Redo remains available
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T827-M18

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify authored animations are unchanged
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M19

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Compare T826/T827 responsiveness on exactly the same fixture
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T827-M20

- Introduced: T-827; original source: `outputs/T827-Key-Selection-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 keys-100/1000/10000.ankh.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T827-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Replay necessary cumulative T813–T826 checks
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M01

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Open synthetic100-key fixture
- Expected: Copied project and its required assets display correctly in the exact cumulative candidate.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: PREPARE-OPEN / same candidate may group this; feature fixture remains distinct.

### T829-M02

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Select a few keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M03

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check Graph highlighting
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M05

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat with 1000 keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M06

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Repeat with 10000 keys
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M07

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Compare retained T828/T829 binaries on identical fixture
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M08

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Partial selection
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M09

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Full selection
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M10

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Ctrl-click
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M11

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Marquee replace
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M12

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Marquee Ctrl-union
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M13

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Raw
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M14

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Normalize
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M16

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change selection from Graph
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M17

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Change clip
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M18

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Undo/Redo
- Expected: Each intended edit undoes/redoes exactly at the declared command granularity, retaining other data.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: HISTORY / full source-specific payload comparison required.

### T829-M19

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Verify curves unchanged
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

### T829-M21

- Introduced: T-829; original source: `outputs/T829-Key-Highlight-Performance/MANUAL-CHECKLIST.md`.
- Fixture: Copied T827 100/1000/10000-key fixtures, identical for comparisons.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T829-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Relevant cumulative T813–T828 controls
- Expected: The stated scenario and its documented native contract hold on the copied project; record actual result.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: No equivalence asserted; retain independently.

## Compatibility and publication

### T831-M26

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check export diagnostics
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

### T831-M27

- Introduced: T-831; original source: `outputs/T831-Path-Position-Animation/MANUAL-CHECKLIST.md`.
- Fixture: Copied T831 Fixtures/AnimatedPath.ankh + AnimatedPath.assets.
- Required binary: Ankhimate-T831-dev.exe (exact SHA below); original Ankhimate-T831-dev.exe only for a controlled comparison.
- Prerequisites: Disposable complete copy; original checklist/contract read; preserve prior binaries/assets/recoveries; stated mode/selection/clip prepared.
- Scenario: Check supported rendered exports
- Expected: Supported CPU output is correct; incompatible data output refuses explicitly before publication. No silent omission.
- Current result: **PENDING MANUAL REVIEW** (no observed result).
- Possible equivalent campaign step: EXPORT-BOUNDARY / retain target-specific constraints.

## Campaign record required later

Record exact executable/fixture hashes, OS build, GPU/driver, DPI/theme, input/focus, expected/actual result and PASS/FAIL/NOT RUN per source ID. Grouping may share setup, Save/reopen or history gestures, but every source-specific postcondition remains represented. A pending or equivalent step is never marked PASS automatically. Prepare compatible closed/index Path and rejected-layout copies before those cases; if a prerequisite fixture is unavailable, record a preparation blocker rather than fabricate a run. No original project/asset/recovery may be altered.
