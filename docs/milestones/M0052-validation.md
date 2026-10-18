# M005.2 — Diagnostics et prévalidation des exports

Validation du 15 octobre 2026. Périmètre : éditeur et bibliothèque d'export,
sans modification du runtime, des templates livrés, des formats v1/v2, du renderer
ou de l'évaluation des animations. Aucun nouveau package.

## Intégration locale préalable

- `develop` était propre sur `b0995586`.
- L'ascendance vérifiée contenait exactement B04 `8022196a76b2989aab536d387143817b9ba5dbb0`
  puis B05 `096658fdd01d67f372f5319536b792c8abea955a`.
- Fast-forward local de `develop` vers B05, puis création de
  `feat/m0052-export-preflight` depuis cette base propre.
- `develop` reste sur B05 pendant M005.2. `main` reste sur
  `9ab4fa439c3690ca99325e24f57b24dd7399b90b`. Branches conservées, aucun push.
- Runtime propre, HEAD conservé sur `6d8f3454aacd6b69fbc3c242b54ba98e19c23f79`
  (`fix/m0032-phaser-alpha`), sans changement de branche ni écriture produit.

## Constats et causes

1. **Publication immédiate après choix du dossier.** Le panneau Data appelait
   `run::export` ou `run::write` directement ; les workers rendus démarraient aussi
   immédiatement. `Plan::survey` existait, mais n'était pas une revue préalable.
   Il manquait un consentement explicite aux remplacements/pertes de fidélité.
2. **Attachment absent silencieusement ignoré.** Le parcours CPU peut ignorer
   une référence qui ne résout aucune pièce et exporter une animation transparente
   avec succès. La régression reproduit ce succès incorrect avant correction.
3. **Inventaire Windows incohérent.** Le publisher normalisait les séparateurs,
   contrairement à `survey`. Un chemin avec backslashes pouvait être aussi classé
   comme orphelin. Sur Windows, une différence de casse pouvait annoncer le même
   fichier comme remplacé **et** préservé. Les deux régressions échouent avant fix.
4. **Limites de cibles non présentées.** La teinte sombre est absente du template
   runtime natif ; Generic JSON est un exemple partiel ; Phaser atlas ne décrit
   que sa première page et n'est pas une animation squelettique. Ces limites sont
   diagnostiquées, pas implémentées. Les templates restent inchangés.
5. **Déformation héritée d'un mesh lié.** Le CPU résout sa géométrie, mais cherche
   le deform sur le nom de l'attachment courant, sans suivre le deform animé de
   la source. Ce cas est bloqué pour les images, sans modifier le renderer.
   Preuve : lecture du code et régression de classification ; aucune nouvelle
   qualification visuelle de cette fonctionnalité complexe n'est revendiquée.

## Réalisation

`export::preflight` inspecte le schéma existant et les bytes des assets. Il classe
les résultats en Error / Warning / Info. Les références incohérentes, nombres et
durées invalides, images absentes/corrompues, géométries et deforms incohérents
bloquent. Un clip vide de durée positive est une pose statique valide signalée.

Le panneau prépare le **vrai Plan** existant, puis ouvre le dialogue modal usuel.
Il montre créations, remplacements, orphelins préservés et diagnostics. Les cases
de consentement à la fidélité et aux remplacements sont distinctes et décochées.
Les erreurs ne sont jamais contournables. PNG/Spritesheet refusent toujours toute
collision et les projets à physique ou blending non-Normal.

`run::inspect` réutilise les validateurs M001 sans création de fichiers. Au clic
Publish, révision/document, chemins et inventaire sont revérifiés. Un inventaire
modifié réinitialise les consentements. `write_reviewed` ajoute une précondition
sur la liste approuvée au publisher existant, avant toute écriture. Staging,
verrous, sauvegardes, rollback et rétention des backups restent ceux de M001b.

Les plans rendus de noms proviennent des mêmes helpers que les exports effectifs.
Les workers M004, `Sequence::prepare`, caméra, sampling, layout, metadata et
consommation mémoire par frame/page sont conservés. Les plans de plugins sont
préparés sur un snapshot ; le document actif n'est pas remplacé pendant la revue.
Les raccourcis globaux sont suspendus pendant le modal. Le thème et le mécanisme
de dialogue existants sont réutilisés.

Le reçu copiable conserve en session les diagnostics, noms et consentements.
Un lancement de worker n'est pas présenté comme une publication terminée.
Voir la [matrice complète et les garanties](../export-preflight.md).

## Tests avant / après

Baseline : **1 061 tests** Rust. Résultat final Windows : **1 084 tests**, tous
réussis, soit **23 nouvelles régressions** (9 éditeur, 14 export).

| Groupe | Preuve avant | Résultat final |
| --- | --- | --- |
| Choix de destination sans publication | Échec : répertoire/fichiers créés immédiatement | Revue seule, aucun fichier/répertoire créé |
| Attachment inconnu en PNG | Échec : export transparent déclaré réussi | Erreur avant toute écriture |
| Diagnostics d'intégrité et fidélité, 9 tests | API écrite en tests avant module : compilation impossible initialement ; ce n'est pas une preuve d'exécution | Images, références, clips, meshes, deforms, courbes, dark tint, physique, blends, skins, templates validés |
| Deform hérité / références de contrainte et draw order | Deux échecs attendus de classification avant correction | Errors attendues ; runtime non bloqué pour un deform valide |
| Consentements, annulation, inventaire/stale document, workers | Contrats testés séparément de la génération | Annulation sans I/O, Warning accepté explicitement, Error non contournable, deux workers à 60 frames |
| Échec tardif par verrou | Nouveau scénario d'intégrité | Document, assets, révision, sélection et historique conservés ; Undo/Redo restent utilisables |
| Remplacements revérifiés par publisher | Nouvelle API absente au premier lancement de test | Inventaire non approuvé refusé avant écriture ; ancien fichier préservé |
| Chemins/JSON, verrou existant, normalisation Windows | Échecs séparateurs/casse enregistrés avant correction | Pas de faux orphelin ; JSON/chemins invalides et verrou refusés sans écriture |
| Pages Phaser atlas | Régression de développement : un stem sous `images/` était compté à tort comme source copiée | Comptage du vrai Plan ; 1 page valide, plusieurs pages bloquées, copies exclues du nombre |

Les tests de rollback, backups retenus, erreurs Windows, JSON escaping, presets,
déterminisme, import PSD, autosave, récupération, raccourcis et M004 existants
font partie du workspace complet et restent verts. Les refus historiques
physique/blending/collisions restent testés séparément de l'accord aux warnings.

## Contrôles automatisés

| Contrôle | Résultat |
| --- | --- |
| `cargo test --workspace --all-targets` | 1 084 passés, 0 échec |
| `cargo fmt --all -- --check` | OK |
| `cargo clippy --workspace --all-targets` | OK ; mêmes 25 diagnostics de warning que la baseline, aucune nouvelle catégorie/occurrence |
| `cargo build --workspace` | OK ; 3 avertissements de dépréciation existants |
| `cargo check -p ankhimate-core -p ankhimate-render --target wasm32-unknown-unknown` | OK ; contrôle de compilation, pas exécution navigateur de l'éditeur |
| Compilation fraîche TS core/tests hors runtime, puis tests Node | 26/26 |
| Typecheck core et Phaser `--noEmit` | OK ; runtime non modifié |
| Référence Rust/TypeScript M001a | 128 cas, tolérance existante 0,01, snapshot Rust actuel identique |
| Strings JSON Rust/TypeScript M001b | 4 round trips, snapshot actuel identique |
| Fixtures réelles Phaser/CPU | 19 fichiers conformes aux SHA-256 acceptés |

Les sorties de diagnostic sont dans `work/m0052` du dossier de mission externe.
Les anciennes preuves d'échecs attendus sont conservées séparément des gates.

## Fidélité sur projets réels

Références non modifiées : `samples/walker.ankh`, clips `walk` et `leg_ik`, et le
projet `tagged-audit.ankh.json` issu du PSD et déjà qualifié dans M002/M004.
Pour chaque clip : 60 PNG de 512×512 à 60 FPS et spritesheet de 7 pages.

- 180 frames comparées aux PNG M004.1 acceptés : **pixels identiques**.
- 180 frames reconstruites des pages spritesheet : **pixels identiques aux PNG**.
- Ces trois projets ne reçoivent aucune Warning runtime injustifiée.
- Les exports GUI walker produisent les mêmes bytes que le banc headless :
  61 fichiers PNG/metadata et 8 fichiers spritesheet/metadata.
- PSD original SHA-256 inchangé :
  `8C0CB6C1E1491C40A417E467F171584F9CD2FCEB947A07532355F73117A85314`.

## Validation graphique Windows réelle

Binaire local DX12, copies isolées sous `work/m0052/gui`. Computer Use a piloté
l'éditeur. L'utilisateur a confirmé les boutons de sélection des dossiers natifs
que l'outil expose comme une fenêtre de 16×16 non utilisable ; cette intervention
est distincte du contrôle automatisé du modal egui et de la vérification des fichiers.

| Parcours | Résultat observé |
| --- | --- |
| Data runtime valide | 2 fichiers prévus, aucune Warning ; aucun fichier avant Publish |
| Cancel du préflight | Dossier vide et document toujours présent |
| Fermeture après ajout de preset | Confirmation Save/Discard/Cancel affichée ; Save de la copie et fermeture réussis |
| PNG `walk`, 60 FPS, 512×512 | 61 noms prévus ; progression du worker ; succès 60 PNG, transparence et metadata conservées |
| Spritesheet `walk`, mêmes réglages, page 2048, padding 2 | 8 noms prévus ; progression ; succès 60 frames / 7 pages |
| Copie avec dark tint + sorties existantes | Warning de perte connue, 2 remplacements, 1 orphelin ; Publish désactivé |
| Accord fidélité seul | Publish toujours désactivé |
| Accord fidélité + remplacement | Publish activé, noms `skeleton.json` / `atlas.png` consultables |
| Publication autorisée | Message « Wrote 2 files … after preflight », JSON runtime v2 valide ; orphelin conservé avec SHA-256 identique |

Captures livrées hors dépôt : `M0052_01_runtime_review.jpg` à
`M0052_13_data_published.jpg`. Les scénarios d'erreurs d'intégrité, permissions,
verrous, rollback et collisions ont une preuve automatisée ; ils ne sont pas
présentés comme tous rejoués manuellement dans l'interface.

Les journaux GUI signalent aussi que la surface wgpu active ne fournit pas de
`CompositeAlphaMode` transparent pour la fenêtre. Les réglages de fenêtre/GPU
existants ne sont pas modifiés ; les PNG RGBA sont vérifiés séparément et restent
identiques aux références. Ce message n'est pas confondu avec les 25 diagnostics
Clippy ni présenté comme une erreur de publication.

## Fichiers concernés

- `export/src/preflight.rs`, `export/src/lib.rs` : diagnostics publics en lecture seule.
- `export/src/run.rs`, `export/src/staged.rs` : inspection réutilisant M001,
  inventaire normalisé et précondition des remplacements approuvés.
- `export/tests/preflight.rs`, `export/tests/preflight_paths.rs` : 14 tests.
- `editor/src/export_preflight.rs`, `editor/src/main.rs` : revue de session,
  consentements et tests de publication/intégrité.
- `editor/src/ui/export.rs`, `editor/src/ui/png_sequence.rs`, `editor/src/ui/mod.rs` :
  passage par le préflight, reçu et thème existant.
- `editor/src/png_export.rs`, `editor/src/spritesheet_export.rs` : plans de noms
  partagés et refus d'erreurs d'intégrité avant rendu/publication.
- `editor/src/app.rs` : suspension des raccourcis globaux pendant le préflight.
- `docs/export-preflight.md`, ce rapport : matrice, procédure et garanties.

## Garanties, compatibilité et limites restantes

Le préflight n'effectue aucune publication et ne réserve pas le dossier. Le
publisher reste seul responsable des contrôles finaux. Les échecs ordinaires
utilisent le rollback existant ; une erreur de rollback retient les backups et
la transaction pour inspection. Aucun backup n'est écrasé par la revue.

Aucune atomicité multi-fichier ni durabilité complète après crash/coupure n'est
promise. L'inventaire des orphelins est best effort. Le consentement concerne
les noms, pas un hash des contenus existants ; un tiers peut encore intervenir
entre opérations OS. Voir `docs/export-safety.md` et la matrice de préflight.

Les formats et pixels des exports valides restent compatibles. Des projets
incohérents auparavant partiellement exportés sont désormais refusés dans l'UI
et dans l'orchestration rendue. Des fonctionnalités présentes mais non qualifiées
nécessitent désormais un accord explicite dans l'éditeur. Les consommateurs
headless des API historiques doivent adopter les diagnostics et leur propre
politique d'accord : `run::write` conserve son contrat de sécurité des fichiers.

Dette restante : qualification des fonctionnalités avancées signalées ; omission
de dark tint native ; héritage des deforms liés côté CPU ; limitations des exemples
Generic JSON/Phaser atlas ; planning/bake synchrone sur les gros projets ; reçu
de consentement non durable (session/copie seulement) ; tests GUI exhaustifs des
erreurs OS et fenêtres détachées. Ces points ne sont pas implémentés dans M005.2.

Le hash du nouveau checkpoint local est fourni avec la livraison. Aucun M005.3,
M005.4, M006 ou travail IA n'est commencé. La suite attend la revue.
