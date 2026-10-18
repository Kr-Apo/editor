# M004.1 — Export de séquences PNG

Validation locale Windows du 15 octobre 2026. Base propre vérifiée sur `develop` :
`25f3a301920a4139db16a7820c42bf5d7c15c822`. Travail isolé sur
`feat/m0041-png-sequence`. Le runtime reste propre à
`6d8f3454aacd6b69fbc3c242b54ba98e19c23f79`, branche
`fix/m0032-phaser-alpha`. Aucun push, merge, format runtime ou original de fixture
n'est modifié. Le commit local M004.1 contient ce rapport.

## Audit et choix

- `render/src/lib.rs::render_frame` et `render_contact_sheet` fournissaient déjà
  l'évaluation CPU, les draws, les textures et le calcul d'une caméra commune.
  Il manquait une séquence temporelle avec fin exclusive et publication.
- `export/src/run.rs` et `staged.rs` fournissaient déjà confinement Windows,
  staging, flush, verrou, manifeste, sauvegardes et rollback. Leur `Plan` tenait
  toutes les images en mémoire ; la variante générée prépare une image à la fois
  avec la même transaction, sans changer le comportement des presets existants.
- `editor/src/ui/export.rs` ne proposait que les données/atlas. Le nouveau choix
  PNG est un état de session, avec un worker sur un snapshot du document.
- `core::evaluate` traite les contraintes stateless mais ignore la simulation
  physique sans `PhysicsState`. Le refus explicite de tout projet avec physique
  évite un rendu silencieusement faux. Pas de nouveau moteur ni de simulation.
- Le schéma de dépendances interdit `export → render` car `render → document →
  export`. L'orchestration des fichiers reste donc dans l'éditeur ; `render`
  reste sans I/O. La seule dépendance ajoutée est le crate local existant
  `ankhimate-render` dans l'éditeur. Aucun nouveau paquet tiers.

L'implémentation minimale, la caméra commune et l'exclusion physique ont été
annoncées avant modification du code.

## Défauts confirmés avant correction

Trois tests ont échoué sur le renderer initial avant leur correction :

1. Rouge à 50% sur fond transparent : `[128,0,0,128]` au lieu de
   `[255,0,0,128]`. La composition ne tenait pas compte de l'alpha destination
   et produisait des couleurs prémultipliées dans un PNG straight-alpha.
2. Deux triangles d'un quad couvraient tous deux leur diagonale : alpha 192 au
   lieu de 128. Une règle de bord semi-ouvert évite la double composition,
   également avec un winding inversé.
3. Filtrage entre rouge opaque et noir transparent : rouge 0,5 au lieu de 1
   pour alpha 0,5. Le filtrage passe maintenant par des couleurs prémultipliées
   et revient à straight-alpha après interpolation.

Les régressions de séquence et de publication générée ont été écrites avant les
API correspondantes ; elles échouaient alors à la compilation, faute de ces API.
Le test graphique a aussi révélé la coupure des contrôles dans un panneau étroit :
le panneau PNG est défilable et l'annulation est à côté de la progression.

## Fichiers modifiés

| Fichiers | Rôle |
|---|---|
| `render/src/sequence.rs` | Échantillonnage, prévalidation, caméra commune, textures cachées, rendu RGBA |
| `render/src/lib.rs` | Alpha, filtrage, bords de triangles, sept nouvelles régressions |
| `export/src/run.rs`, `export/src/staged.rs` | Génération des PNG pendant le staging ; refus de remplacement ; annulation et deux tests de sûreté |
| `editor/src/png_export.rs` | Assemblage, noms stables, métadonnées, trois tests sur walker et PSD |
| `editor/src/ui/png_sequence.rs` | Sélection, paramètres, worker, progression, annulation, deux tests de concurrence |
| `editor/src/ui/export.rs`, `editor/src/ui/mod.rs`, `editor/src/main.rs` | Branchement du panneau |
| `editor/src/app_state.rs`, `editor/src/app.rs` | Conserver le worker annulé lors de New/Open ; attendre son rollback avant Close |
| `editor/src/app_lifecycle_tests.rs` | Close/Cancel et attente de terminaison du worker |
| `editor/Cargo.toml`, `Cargo.lock` | Dépendance locale existante, sans paquet tiers ajouté |
| `docs/png-sequences.md`, `docs/export-safety.md`, ce rapport | Utilisation, contrat et garanties effectives |

## Résultats automatisés

Rust/Cargo 1.99.0, Windows natif. Gates exécutés hors sandbox pour éviter les
effets de permissions déjà identifiés dans M001. Les logs détaillés sont dans
le dossier externe de preuves `work/m0041`, avec les résultats JSON.

| Contrôle | Résultat |
|---|---|
| `cargo test --workspace --all-targets` | **1 039 réussis**, 0 échec, 0 ignoré ; baseline 1 024 + 15 |
| `cargo fmt --all --check` | Réussi |
| `cargo clippy --workspace --all-targets` | Réussi, 25 diagnostics préexistants ; aucun nouveau |
| `cargo build --workspace` | Réussi ; trois dépréciations préexistantes de l'éditeur |
| `cargo check -p ankhimate-core -p ankhimate-render --target wasm32-unknown-unknown` | Réussi |
| Tests TypeScript existants en lecture seule | **26 réussis**, 0 échec |
| Typecheck core et Phaser, `--noEmit` | Réussi, aucun fichier runtime écrit |
| Référence Rust vs fixture des axes M001a | 128 cas identiques |
| Référence Rust vs fixture des chaînes M001b | 4 cas identiques |

Les nouvelles régressions couvrent : 60 frames/60 FPS, dernière frame 59/60,
indices invalides, deux exports identiques, transparence, caméra unique,
réévaluation identique au rendu individuel, clip manquant, durée zéro/NaN,
FPS/dimensions invalides, clip statique sans clés, refus physique, annulation du
cadrage et du rendu, échec d'écriture, refus de collision, préservation des
sorties antérieures, rollback après début de publication, import PSD en mémoire,
timelines d'attachment/couleur/draw-order, lancement concurrent et Close.

Toutes les suites M001 existantes font partie du gate : atlas/presets,
déterminisme, JSON, staged export, rollback/restauration et backups conservés,
verrous Windows réels, sauvegarde projet, dirty/undo/redo, Save As, chargements
asynchrones, autosave monotone et récupération. Aucun changement de leurs formats.

## Fidélité et exports réels

Un banc externe appelle les API réelles, sans patch de données d'export ni
écriture dans le runtime. Il exporte trois séquences RGBA 512×512, à 60 FPS :

| Projet / clip | Frames | Temps du banc debug, staging compris |
|---|---:|---:|
| `samples/walker.ankh` / `walk` | 60 | 7,076 s |
| `samples/walker.ankh` / `leg_ik` | 60 | 7,550 s |
| Projet M002 issu de `tagged_rig.psd` / `animation_1` | 60 | 5,262 s |

À t=0, 0,25, 0,5 et 0,75, les 12 poses/draw-orders/couleurs ont été confrontés
aux références CPU conservées et utilisées pour la validation runtime M003.2.
L'écart maximal sur affines et sommets est `2,85 × 10^-14`, très inférieur à
0,01 unité. Chaque PNG de séquence comparé est identique au rendu individuel
avec sa caméra fixe. La comparaison des previews opaques avec les anciennes
références trouve au maximum 13 pixels sur 540000 avec un écart supérieur à
8/255 (0,00241%), lié aux corrections de filtrage/bords ; tolérance M003.2
conservée : moins de 2%. La caméra de séquence est calculée sur les 60 poses,
distincte de la caméra de référence M003.2 fixée à [0,0].

## Test graphique Windows réel

Exécuté avec le binaire debug natif et Computer Use, uniquement sur des copies
externes. Les assets associés ont été copiés avec les projets ; les originaux
PSD/walker n'ont pas été modifiés.

- Export → PNG sequence, `walk`, 60 FPS, 512×512 : **60 PNG + sequence.json**.
- Ouverture du projet issu du PSD, choix `animation_1`, mêmes paramètres :
  **60 PNG + sequence.json**. Les 120 PNG de ces deux parcours ont les mêmes
  hashes que les exports automatisés correspondants.
- Annulation pendant le rendu walker : retour d'erreur explicite
  « export cancelled; rollback completed, previous output files preserved » ;
  seul `previous.txt` reste dans le dossier, sans transaction ni PNG partiel.
- Nouvelle tentative sur le dossier tagged déjà exporté : refus explicite,
  **61 hashes inchangés** et aucun staging résiduel.
- Fermeture pendant l'export d'une copie prolongée à 30 s / 1800 frames :
  fermeture observée en 347 ms après le clic, zéro fenêtre éditeur restante,
  seul le fichier antérieur subsiste dans la destination. Aucun export partiel.
- Le test automatisé complémentaire vérifie que Cancel au dialogue Close laisse
  l'export actif et qu'aucune commande Close finale n'est émise avant sa fin.
- Préférences utilisateur restaurées à l'identique après fermeture ; aucun
  enregistrement sur les sources originales. SHA256 du PSD toujours
  `8C0CB6C1E1491C40A417E467F171584F9CD2FCEB947A07532355F73117A85314`.

Les captures Windows, hashes, `fidelity.json`, logs et une archive des vraies
séquences sont remis dans les livrables externes. Les erreurs OS et violations
de partage Windows sont testées automatiquement ; ce rapport ne prétend pas
avoir injecté une panne matérielle ni une coupure électrique via l'interface.

## Garanties et limites

Prévalidation, staging sur le même filesystem et flush avant publication,
refus de tout nom existant pour une séquence, rollback des erreurs ordinaires,
préservation des orphelins, erreurs explicites et sauvegardes conservées si une
restauration échoue. Les anciennes API d'export gardent leur remplacement M001b.

Ce n'est **pas une transaction atomique multi-fichier** : un lecteur concurrent
peut voir une publication partielle. Un arrêt forcé peut laisser verrou,
manifeste, staging et sorties partielles. Pas de garantie de durabilité des
renames de répertoire, pas de journal de crash ni de récupération automatique
d'export. La protection contre les writers non coopératifs garde les limites
M001b, décrites dans `docs/export-safety.md`.

Physique refusée de façon conservatrice ; skin par défaut ; Normal uniquement.
La certification de teintes sombres, déformations/clipping complexes et très
grands projets reste à faire. Annulation coopérative après la frame/encode ou
l'opération filesystem courante ; pas de délai maximal garanti. Le snapshot
initial copie les assets ; la mémoire n'est pas proportionnelle à tous les PNG
mais n'est pas bornée indépendamment de la taille du document et d'une frame.

Pas de spritesheet, vidéo, GPU, IA, M004.2 ou M005 dans ce milestone. Attendre
la revue avant tout travail suivant.
