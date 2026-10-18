# M005.3a — Transmission Rust des teintes sombres

Validation du 15 octobre 2026. Travail limité au preset natif, à la validation
de ses valeurs sombres, aux diagnostics, tests et documentation. Aucun changement
du runtime, de l'évaluateur ou du renderer produit. Aucune dépendance ajoutée.

## Base et intégration locale

- Éditeur initial propre sur M005.2 `13f62df50bc6c603d8580129fcda0e373fa03b76`.
- `develop` était sur `096658fdd01d67f372f5319536b792c8abea955a` ; son unique
  descendant jusqu'au checkpoint accepté était M005.2, dont le parent est B05.
- Fast-forward local de `develop` vers M005.2, puis création de
  `fix/m0053a-dark-tint-export` depuis cette base propre.
- `main` reste sur `9ab4fa439c3690ca99325e24f57b24dd7399b90b`. Branches conservées.
- Runtime propre sur `6d8f3454aacd6b69fbc3c242b54ba98e19c23f79`, branche
  `fix/m0032-phaser-alpha`, sans changement produit ni changement de branche.
- Aucun push, merge distant, amend ou release.

## Audit et causes

| Étape | Preuve et état |
| --- | --- |
| Modèle auteur | `core/src/slot.rs` porte déjà `dark_color: Option<[f32; 4]>` |
| Schéma et conversion | `formats/src/schema.rs` accepte le tableau optionnel ; `formats/src/convert.rs` le conserve dans les deux sens |
| Contexte | `export/src/context.rs` transmet déjà `slot.dark_color` sans arrondi ; aucun changement du contrat de contexte nécessaire |
| Template natif | `export/src/presets/ankhimate_runtime.json` omettait la propriété : cause directe de la perte |
| JSON intermédiaire | serde convertit les f32 non finis en null dans un `Value` : risque évité avant construction du contexte pour le preset natif actualisé |
| Lecteur existant | `core/src/rig.ts` du runtime accepte v1/v2 et rend l'objet original : propriété inconnue conservée |
| Évaluation/Phaser | `core/src/types.ts`, `core/src/evaluate.ts` et `phaser/src/phaser.ts` ne déclarent/appliquent pas le sombre : perte visuelle persistante, explicitement avertie |

L'audit et la proposition additive ont été présentés avant modification produit.
L'extension minimale reste dans `format_version: 2`, `context_version: 1`,
`preset_version: 1` : propriété optionnelle `slots[].dark_color`, quatre nombres
finis RGBA, sans modification de sens des champs précédents. `None` omet la clé ;
`Some([0;4])` l'émet. Aucune nouvelle piste d'animation. Les nombres f32 sont
préservés, sans encodage hexadécimal ni arrondi décimal.

Le CPU copie le sombre statique du setup dans la pose. Les pistes SlotColor
animent uniquement la teinte claire. Le rasterizer utilise
`T.rgb*L.rgb + (1-T.rgb)*D.rgb*D.a` et `T.a*L.a`, puis la composition Normal.
L'alpha sombre est une intensité de contribution, pas l'opacité du slot.
Trois nouveaux tests CPU vérifient intensité 0/.5/1, extrêmes RGB, transparence,
interaction avec l'alpha clair et indépendance vis-à-vis de son animation.
Ils n'ajoutent aucune capacité de rendu.

## Réalisation et compatibilité

- Le template utilise le helper JSON existant sur le tableau optionnel.
- Le planificateur natif actualisé refuse NaN/±Inf dans les quatre canaux avant
  bake/contexte, pour les appels directs comme pour le preview. Aucun accès disque
  supplémentaire ; même erreur de rendu compréhensible, aucune écriture partielle.
- Le préflight conserve le consentement obligatoire avec le code
  `dark-tint-runtime-unapplied`, sans annoncer de fidélité Phaser.
- Les anciens presets sauvegardés ne sont pas réécrits. Leur template exact est
  reconnu comme `LegacyRuntime` et conserve `dark-tint-loss`, avec instruction
  d'ajouter le preset natif courant. Une simple étiquette ne qualifie toujours pas
  un preset modifié ; le diagnostic custom reste applicable.
- Les autres avertissements runtime restent appliqués aux anciens presets.
- Les exports sans sombre, y compris un atlas non vide, sont comparés octet par
  octet à une copie du template accepté M005.2. Les presets Generic/Phaser atlas,
  le publisher, le staging/rollback, les formats rendus et M001 restent inchangés.
- Les valeurs finies hors 0–1 sont conservées, conformément au modèle actuel.
  Le contrôle direct ciblé n'est pas un validateur universel de templates custom ;
  le préflight conserve son contrôle global des nombres et sa politique de consentement.

## Tests avant/après

`export/tests/dark_tint.rs` ajoute sept tests :

1. Transmission exacte des quatre f32, fractions non triviales, extrêmes, alpha
   nul et intermédiaire, tableau nul et valeurs finies hors 0–1 ; document inchangé.
2. Sans sombre : identité du JSON et des vrais bytes d'atlas avec l'ancien preset.
3. Guillemets, backslashes, contrôles, Unicode et caractères spéciaux dans le nom ;
   couleur claire/blend inchangés, export déterministe.
4. Ancien preset sauvegardé préservé, reconnu, averti ; pas de fausse alerte sans
   sombre ; preset édité toujours custom.
5. Nouveau diagnostic, consentement non acquis par défaut, inspection/annulation
   sans création du dossier de destination.
6. NaN/±Inf dans chaque composante : erreur bloquante au préflight et au planificateur
   Encoded/Named, source préservée même à l'échelle des bits.
7. Schéma auteur : quatre composantes numériques obligatoires, anciens documents
   sans champ toujours acceptés.

Avant correction, cinq de ces sept tests échouaient pour les comportements attendus
(champ absent, nouveau diagnostic absent, ancien template non distingué et nombres
non finis acceptés par le planificateur). Les deux autres conservaient les invariants
existants. Journal : `work/m0053a/red-dark-tint.log`. Le premier essai avait une
fixture incomplète (`length` absent) ; elle a été corrigée avant cette vraie preuve
rouge, sans modification produit. Après correction, les sept tests passent.
Le test M005.2 de perte sombre est adapté à la transmission et au refus de fidélité
visuelle ; la perte de l'ancien template reste couverte séparément.

## Résultats automatisés

| Contrôle | Résultat |
| --- | --- |
| Baseline Rust M005.2 | 1 084 tests réussis |
| `cargo test --workspace --all-targets` | 1 094 réussis, 0 échec : +7 export, +3 CPU |
| `cargo fmt --all -- --check` | Réussi |
| `cargo clippy --workspace --all-targets` | Réussi, mêmes 25 avertissements qu'avant ; aucun ajouté |
| `cargo build --workspace` | Réussi, éditeur fermé avant build |
| WASM core/render, `wasm32-unknown-unknown` | Réussi |
| Tests TypeScript, recompilés hors runtime | 26 réussis |
| Core/Phaser typecheck et build hors runtime | Réussis |
| Référence Rust/TypeScript M001a | 128 cas conservés |
| Référence JSON/échappement Rust/TS | 4 cas conservés |
| Fixtures Phaser/CPU acceptées | 19 fichiers, SHA-256 inchangés |
| Lecteur TS sur vrai export sombre Rust | Champ RGBA f32 exact conservé ; 8 poses de deux clips identiques sans/avec champ car le sombre est toujours ignoré |
| PNG/spritesheet | 180 frames, 3 clips ; pixels identiques aux PNG M004.1 et à leur reconstitution depuis les pages |

Le banc isolé utilise `walker.ankh` (`walk`, `leg_ik`) et le projet issu de
`tagged_rig.psd` (`animation_1`). Chaque clip produit 60 frames à 60 FPS,
512×512. Aucun original modifié. Il vérifie aussi document → schéma → JSON →
modèle auteur pour le sombre avant d'exporter le vrai JSON natif et son atlas.
Les builds TypeScript écrivent uniquement sous `work/m0053a`, jamais dans le runtime.

Les 25 avertissements Clippy sont comparés aux messages de la baseline ; les
trois dépréciations compilateur existantes restent également présentes. Aucun
nouveau warning lié à ce milestone. Journaux sous `work/m0053a` hors dépôt produit.

## Contrôle Windows

Contrôle réel réussi sur une copie isolée de walker avec sombre statique :

1. Ouverture de `work/m0053a/gui/dark.ankh` dans le binaire construit. L'offre de
   récupération préexistante est ignorée sans supprimer sa copie.
2. Panneau Export / Data-atlas : choix de `work/m0053a/gui/output`. Le sélecteur
   natif est renseigné par l'outil, puis confirmé par l'utilisateur car les clics
   automatisés n'activaient pas ce dialogue.
3. Préflight : deux créations, zéro remplacement/orphelin. Avertissement
   `dark-tint-runtime-unapplied` visible, consentement décoché, Publish désactivé.
   Le dossier ne contient alors aucun fichier exporté.
4. Acceptation explicite : Publish activé. Publication effective du JSON et de
   l'atlas, puis message « Wrote 2 files ... after preflight » dans l'interface.
5. Les deux fichiers sont identiques **octet par octet** à l'export headless du
   même projet. Le slot `arm_far_slot` contient exactement les quatre valeurs
   f32 `[0.2,0.1,0.3,0.5]` élargies en JSON ; aucune teinte sombre visible dans
   Phaser n'est revendiquée.
6. Ctrl+S enregistre uniquement la copie isolée. Relecture avec le chargeur réel
   pour vérifier la conservation de la valeur sombre, des images et du preset.
   Fermeture normale de l'éditeur ; configuration utilisateur restaurée à l'octet
   près (SHA-256 `0938A37338D6A8E3F42A4484CF22E844A8FAEB316318C7E663D7FCC25E4070AC`).

Cinq captures et leurs arbres d'accessibilité, ainsi que le résultat vérifié,
sont conservés sous `work/m0053a/windows`. Aucun original PSD/projet n'a été édité.

## Fichiers et limites

- Produit : `export/src/presets/ankhimate_runtime.json`, `export/src/presets/mod.rs`,
  `export/src/preflight.rs`, `export/src/run.rs`.
- Tests : `export/tests/dark_tint.rs`, `export/tests/preflight.rs`,
  `export/tests/fixtures/runtime_v2_before_dark_tint.json`, bloc `cfg(test)` de
  `render/src/lib.rs` exclusivement.
- Documentation : `docs/export-context.md`, `docs/export-preflight.md`,
  `docs/runtime-dark-tint.md` et ce rapport.

Le contrat proposé pour M005.3b et ses critères CPU/WebGL sont détaillés dans
[runtime-dark-tint.md](../runtime-dark-tint.md). M005.3b n'est pas commencé.
Pas de garantie de two-color tint universelle, de fidélité sombre Phaser, de
physique, de blending complexe ou de skins rendus supplémentaires. Les avertissements
PNG/spritesheet restent prudents. Les limites d'atomicité/durabilité multi-fichier
de [M001b](../export-safety.md) restent inchangées. Attendre la revue avant toute
modification du runtime ou toute suite.
