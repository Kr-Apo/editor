---
title: M001a — Export scalaire et parité Rust/TypeScript
description: Contrat runtime v2, compatibilité v1 et validation numérique des canaux d'os.
---

# M001a — Export des canaux d'os et parité Rust/TypeScript

**Date : 14 octobre 2026.** Branches `dev/premium-pipeline` (éditeur) et
`dev/three-webgpu` (runtime). Aucun commit créé. Les changements préexistants de
`package-lock.json` et `phaser/package.json` sont conservés à l'identique.

## Changement limité

Le preset natif émet désormais `format_version: 2` et les six pistes scalaires
`translate_x`, `translate_y`, `scale_x`, `scale_y`, `shear_x`, `shear_y`.
Une piste n'a plus besoin d'un axe frère pour être exportée. Chaque axe conserve
ses propres temps, valeurs et courbes sortantes. Une piste absente ou vide garde
la valeur setup ; scale reste multiplicative et shear reste un offset en degrés.

`export/src/context.rs` fournissait déjà ces pistes exactes. Son contrat public,
son `CONTEXT_VERSION=1`, sa projection paire de compatibilité et le bake des
offsets restent inchangés. Les presets générique/Phaser et les formats
communautaires ne sont pas modifiés. Le format des presets reste également v1.

Le runtime adapte les types, `parseRig` et l'évaluateur. V2 interdit les pistes
paires et le champ `offset`, car l'export a déjà décalé les temps des clés. Les
pistes scalaires sont validées : tableaux, temps triés et finis, valeurs finies,
courbes et handles Bézier finis, X des handles dans `[0,1]`, Y non borné pour
permettre l'overshoot.

La comparaison réelle a révélé une frontière de temps : `1.6` en Rust f32 vaut
environ `1.600000023841858`. V2 arrondit le playhead d'os avec `Math.fround` avant
sampling, afin d'atteindre la même clé stepped au même instant. Cette adaptation
ne change pas le chemin legacy v1, ni les canaux de slots/FFD/contraintes.

## Versionnement et compatibilité

| Export | Lecteur adapté | Comportement |
|---|---|---|
| Sans `format_version` | Accepté comme v1 | Anciennes pistes paires et offset conservés |
| `format_version: 1` | Accepté | Même comportement que v1 implicite |
| `format_version: 2` | Accepté | Pistes scalaires indépendantes, temps déjà décalés |
| Version inconnue ou `null` | Refusé | `RigFormatError` explicite |
| V2 avec pistes paires/offset, ou v1 avec pistes scalaires | Refusé | Contrat ambigu signalé, pas de perte silencieuse dans le lecteur adapté |

V2 nécessite le lecteur modifié dans cette branche. Les anciennes versions du
runtime ne connaissant pas `format_version` peuvent accepter le JSON puis ignorer
les pistes scalaires : elles ne sont **pas compatibles** avec v2. Le numéro npm
`0.1.1` n'est pas changé ni publié dans cette mission ; il ne suffit pas à lui
seul à identifier le lecteur adapté. Une release devra attribuer une version de
package appropriée.

Les presets natifs déjà sauvegardés dans un document conservent leur ancien corps
v1 ; ils ne sont pas migrés silencieusement. Sélectionner/copier le preset natif
mis à jour pour produire v2. Des canaux déjà perdus dans un ancien fichier exporté
ne sont pas récupérables : réexporter depuis le projet auteur.

## Tests écrits avant correction

Trois nouveaux tests d'export Rust échouaient sur le preset original : axe seul,
temps/courbes indépendants, offsets exportés. L'ancien test de rendu du preset
restait vert. Six nouveaux tests TS échouaient avant l'adaptation : pistes sans
frère, stepped asynchrone, Bézier indépendant, temps baked/axes vides, versions
et validation des clés. Les 17 anciens tests TS restaient verts.

Une septième régression TS a ensuite été ajoutée et exécutée rouge avant la
correction du playhead f32. La comparaison intégrée sur les références Rust
échouait aussi à cette frontière, sans augmenter la tolérance ni supprimer le
cas de test.

## Fixtures et gate inter-dépôts

Source canonique : `export/tests/fixtures/runtime_axes.ankh.json`. Dix clips
couvrent les six axes seuls, Y stepped, courbes indépendantes et temps
asynchrones pour Translate/Scale/Shear, overshoot, offsets positifs/négatifs.
Deux os avec rotation, shear et échelles setup non uniformes permettent de tester
la propagation hiérarchique, pas seulement les valeurs de clés.

`export/examples/runtime_axes_parity.rs` utilise le vrai preset natif et
`core::pose::evaluate` sur le même projet. Il produit l'export et les références
locales/mondiales ; aucune valeur attendue n'est calculée par TypeScript.
Le snapshot dans le runtime est
`core/test/fixtures/runtime_axes_parity.json` ; il est vérifié par la suite
officielle `core/test/export-parity.test.ts`.

Le gate compare **128 cas, 3 328 composantes** : sept valeurs locales et six
composantes d'affine mondiale par os, aux frontières et entre les clés, avant et
après leur plage, ainsi qu'avec alpha partiel et deux couches.
Tolérance absolue : **0,01**. Écart maximal observé :
**0,00000643730700744527** (environ 6,44 × 10⁻⁶).

Pour vérifier que le snapshot correspond au code Rust actuel, depuis le runtime :

```powershell
node scripts/check-rust-parity.mjs C:/path/to/editor C:/path/to/cargo.exe
npm test
```

Le premier contrôle lance l'exporter Rust et compare structurellement sa sortie
au snapshot ; il ne réécrit aucun fichier. `npm test` compare ensuite les valeurs
TypeScript aux références Rust. Pour régénérer volontairement le snapshot après
une évolution validée, depuis l'éditeur :

```powershell
cargo run --quiet --locked --offline -p ankhimate-export --example runtime_axes_parity > C:/path/to/runtime/core/test/fixtures/runtime_axes_parity.json
```

Revoir le diff numérique avant d'accepter un nouveau snapshot ; ne pas actualiser
des attentes simplement pour faire passer un test. Sous Windows, ce gate suppose
une redirection UTF-8 (PowerShell 7). Les deux dépôts ne sont pas requis pour la
suite npm ordinaire ; ils le sont pour le contrôle anti-obsolescence.

## Contrôles finaux

| Contrôle exécuté | Résultat définitif |
|---|---|
| `cargo test --workspace --all-targets --locked --offline` | **936 réussis**, 0 échec, 0 ignoré ; baseline 933 + 3 régressions |
| `cargo fmt --all --check` | Code 0 |
| `cargo clippy --workspace --all-targets --locked --offline` | Code 0 ; mêmes diagnostics préexistants qu'à M000, aucun nouveau warning |
| `cargo build --workspace --locked --offline` | Code 0, build dev |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown --locked --offline` | Code 0 |
| Runtime `npm test` | **25 tests core réussis**, 0 échec ; Phaser build/typecheck réussis |
| Runtime `npm run build` | Deux workspaces réussis |
| Runtime `npm run typecheck` | Deux workspaces réussis |
| `node scripts/check-rust-parity.mjs ...` | Snapshot identique aux **128 références du code Rust actuel** |
| `git diff --check` dans les deux dépôts | Réussi |
| SHA-256 des deux fichiers de dépendance runtime préexistants | Identiques aux hashes relevés avant cette mission |
| `cargo run --locked --offline -p xtask -- docs-check` (contrôle supplémentaire) | Échec préexistant : références CRLF signalées stale ; un essai en LF révèle aussi le lien `docs/formats/index.md → ../format-spec/` non résolu |

Les warnings de M000 ne sont pas supprimés ; Clippy n'est pas lancé avec
`-D warnings`. Le runtime n'a pas de formatter/linter configuré : le formatage
TypeScript a été revu dans le diff et le contrôle de whitespace Git est vert.
Aucun résultat graphique, performance GPU ou stabilité de session MCP n'est
revendiqué par ces tests numériques.

Le contrôle documentaire signale les fins de ligne CRLF des références générées.
Un essai de resynchronisation LF n'a changé aucun contenu normalisé ni API ;
les fichiers ont ensuite été restaurés octet pour octet. L'échec de lien révélé
pendant cet essai est documenté dans M000 ; le checker et la page concernée sont
inchangés. Les nouveaux liens relatifs de ces deux rapports
pointent vers des fichiers existants. Les suites comportementales n'ont aucune
régression.

## Fichiers modifiés ou ajoutés par M001a

Dans `D:\Ankhimate\editor` :

- `export/src/presets/ankhimate_runtime.json` : format natif v2 et six axes ;
- `export/tests/export.rs` : trois régressions natives ;
- `export/tests/fixtures/runtime_axes.ankh.json` : fixture auteur canonique ;
- `export/examples/runtime_axes_parity.rs` : export et références Rust ;
- `docs/formats/export-runtime.md` : contrat et compatibilité ;
- `docs/milestones/M000-baseline.md` : bilan initial complet ;
- `docs/milestones/M001a-validation.md` : présent rapport.

Dans `D:\Ankhimate\runtime` :

- `core/src/types.ts` : version de format et six champs scalaires ;
- `core/src/rig.ts` : lecture v1/v2 et validation du nouveau contrat ;
- `core/src/evaluate.ts` : sampling indépendant, playhead v2 f32 ;
- `core/test/evaluate.test.ts` : sept régressions v2/compatibilité ;
- `core/test/export-parity.test.ts` : comparaison numérique avec Rust ;
- `core/test/fixtures/runtime_axes_parity.json` : références générées ;
- `scripts/check-rust-parity.mjs` : contrôle anti-obsolescence inter-dépôts ;
- `core/README.md` : contrat et utilisation du gate.

`runtime/package-lock.json` et `runtime/phaser/package.json` restent modifiés par
les changements **préexistants** de l'utilisateur ; ils ne sont pas des edits de
M001a. Les HEAD et branches n'ont pas changé. Les fichiers livrés restent non
committés pour revue. M001a est validé sur le périmètre décrit.

## Hors périmètre maintenu

Pas de M001b : ni escaping général des noms, publication transactionnelle,
sauvegarde/recovery, physique, fondus interrompus ou couleur des slots corrigés.
Pas de M002 : aucun renderer Three.js ou API graphique modifié.
Pas de modification de l'interface, du MCP, des autres formats ou des dépendances.
