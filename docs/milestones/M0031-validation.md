# M003.1 — Fiabilisation du parcours d’import PSD

Validation du 14 octobre 2026, sur Windows, branche `develop`.
Base vérifiée : `fdbf339e022d8b3a08fe0441d91e08e1d9c42082`.
Les deux dépôts étaient propres avant modification. Le runtime reste sur
`dev/three-webgpu`, à `f478463cc431550cc8faa9788e52ca5fef667c95`, sans modification.

## Causes et corrections

**B01 : Redo sans restauration.** `ImportPsd::apply` consommait `imported`, puis
`revert` consommait `before`. Le prochain `apply` revenait immédiatement alors
que `History::redo` annonçait une commande exécutée.

La commande garde maintenant le document inactif et échange les deux états
avec `std::mem::swap`, comme `ReplaceDocument`. Le premier import est calculé
une seule fois. Undo/Redo ne consultent ni le fichier PSD, ni son décodeur.
Le document complet est conservé : squelette, images et leurs bytes, identités
des entités, animations, métadonnées du projet, provenance PSD et presets.
L’historique existant est utilisé sans modification de son architecture.

Deux incohérences supplémentaires dans cette même commande ont été reproduites
par des tests avant correction :

- En remplacement, les animations de l’ancien squelette restaient présentes.
  Le PSD ne contient pas de clips : les animations sont maintenant vidées lors
  du remplacement, et intégralement restaurées par Undo. En fusion, les clips
  existants sont conservés.
- En fusion, `AssetDb::add` uniquifiait les noms, mais les attachments et la
  provenance continuaient de référencer les noms initiaux. La commande remappe
  désormais les textures des regions/meshes, les frames de séquences et les
  chemins de couches PSD. Les pixels et la provenance des anciens assets ne
  sont pas écrasés.

**B03 : startup masquant un import réussi.** Le résultat `Opened` désactivait
`show_startup`, contrairement au résultat `Imported`. Ce dernier le désactive
maintenant uniquement après le succès et l’adoption du document. Le rapport
d’import demeure disponible. Les branches Error/Cancelled et les protections
de chargement asynchrone sont inchangées.

## Fichiers

| Fichier | Modification |
|---|---|
| `document/src/commands/psd_cmds.rs` | Échange des états, animations en remplacement, références d’assets en fusion |
| `document/tests/psd_history.rs` | 5 tests d’intégration sur le vrai PSD et un document animé existant |
| `editor/src/app.rs` | Affichage du workspace après un import réussi |
| `editor/src/app_lifecycle_tests.rs` | 6 tests de parcours et de conservation du document |
| `docs/milestones/M0031-validation.md` | Ce rapport |

Aucune dépendance ajoutée, aucun format d’export changé, aucune fixture
originale modifiée. Aucun travail Phaser, M003.2, M004 ou IA.

## Tests ajoutés

Les quatre régressions initiales de la commande échouaient avant modification :
Redo en remplacement, Redo en fusion, conservation de timelines étrangères en
remplacement et références d’images en collision. La régression startup
échouait également avant la correction du résultat `Imported`.

Les tests de document comparent le schéma complet, les pixels, les identités et
références en mémoire, les ordres de bones/slots/contraintes/groupes, les skins
et les animations. Ils exécutent trois cycles Undo/Redo en remplacement et en
fusion, y compris avec des collisions d’assets et des frames de séquence.
Une region de séquence est également convertie en mesh en mémoire dans le test
de collision, afin de couvrir les deux types d’attachments sans changer le PSD.
L’ancien projet est un `walker.ankh` animé, enrichi de métadonnées et de presets.
Une commande de transformation antérieure reste undoable après l’import.
Un cinquième test vérifie aussi trois traversées de l’import et d’une ancienne
création de bone, dont le Redo régénère normalement l’identifiant.

Les six tests d’éditeur couvrent :

1. File Import asynchrone depuis startup : workspace, rapport, dirty et absence
   de chemin actif après succès.
2. PSD invalide et résultat Cancelled : document, sélection, historique,
   révision et startup conservés.
3. Cancel de la confirmation de remplacement : travail et opération en attente
   conservés, aucun chargement lancé.
4. Modification pendant un import : le résultat tardif ne remplace pas le
   document plus récent.
5. Commande PSD en remplacement/fusion : dirty, sélection de bones existants en
   fusion, pruning des bones importés retirés par Undo, trois cycles,
   Save/Undo/Redo, réouverture et conservation des pixels.
6. Événement egui de fichier PSD déposé : le gestionnaire d’images existant le
   rejette proprement, sans remplacer le travail ni modifier les assets,
   la sélection ou l’historique.

Les sélections restent de l’état de session, conformément à l’architecture :
Undo ne réinstalle pas une ancienne sélection. Les tests vérifient le maintien
des sélections existantes en fusion et le pruning des bones retirés.
Open démarre une nouvelle session et vide l’historique, comme auparavant.

## Résultats automatisés

| Contrôle | Résultat |
|---|---|
| `cargo test --workspace --all-targets` | **1 024 réussis**, 0 échec, 0 ignoré : baseline M002 de 1 013 + 11 nouveaux |
| `cargo fmt --all -- --check` | Réussi |
| `cargo clippy --workspace --all-targets` | Réussi ; diagnostics identiques à M002 |
| `cargo build --workspace` | Réussi, éditeur fermé |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown` | Réussi |
| Tests TypeScript existants, exécutés en lecture seule | **26 réussis**, 0 échec |
| Typecheck core et Phaser, `--noEmit` | Réussi, aucun fichier runtime écrit |
| Référence Rust des axes/export vs fixtures TS | 128 cas identiques à la référence M001a |
| Référence Rust des chaînes JSON vs fixtures TS | 4 cas identiques à la référence M001b |
| Sources d’export/runtime protégées | 61 fichiers vérifiés, aucun changement de hash |

Clippy contient 25 diagnostics préexistants, soit 34 lignes `warning:` en
incluant les résumés de compilation. La comparaison des messages avec M002
ne trouve aucun ajout. Les premières exécutions dans le sandbox Windows ont
produit un avertissement de cache de hardlinks et une erreur de sauvegarde dans
un ancien test ; les gates définitifs utilisent le système Windows natif et
ne reproduisent pas ces effets du sandbox.

Les suites existantes d’atlas, presets, déterminisme, escaping JSON,
publication staged/rollback, sauvegarde sécurisée, verrous Windows, Save As,
chargements concurrents, autosave monotone et récupération sont incluses dans
la suite Rust complète ; aucune ne régresse.

## Essais graphiques Windows

Exécutable construit depuis le correctif, mode réactif, contrôle par Computer
Use. Copies exclusivement dans un répertoire d’essai extérieur au dépôt.

| Essai réellement exécuté | Résultat |
|---|---|
| Startup → File > Import > Photoshop PSD | Workspace visible immédiatement après succès et rapport montrant les deux couches ignorées |
| Save As de cet import en JSON lisible | Succès ; 4 bones, 9 slots, 11 images |
| New après sauvegarde | Projet vide, pas de confirmation inutile |
| Assets → preview PSD → remplacement | Succès ; 4 bones, 9 slots, 11 images |
| Trois Undo/Redo du remplacement | Bibliothèque 11 → 0 → 11, artwork restauré à chaque cycle |
| Preview PSD → fusion du même PSD | Succès ; 9 bones, 18 slots, 22 images, noms d’assets suffixés |
| Trois Undo/Redo de la fusion | Bibliothèque 22 → 11 → 22, artwork restauré à chaque cycle |
| Close sur le document modifié | Save / Discard / Cancel visibles |
| Cancel de Close | Fenêtre ouverte et projet fusionné conservé |
| Save As puis Open du projet fusionné | Réouverture réussie, 22 images et artwork conservés |
| Close du projet réouvert propre | Fermeture réussie sans confirmation inutile |

Le JSON fusionné a été analysé séparément : 9 bones, 18 slots, 22 assets,
1 skin, 0 animation. Son hash est inchangé après Open et Close.
Les préférences ont été restaurées byte pour byte après fermeture de l’éditeur.
Le PSD original conserve son SHA-256
`8c0cb6c1e1491c40a417e467f171584f9cd2fceb947a07532355f73117a85314`.

**Limite explicite du glisser-déposer graphique :** l’outil de contrôle Windows
refuse un drag hors des bounds de la fenêtre source ; le déplacement Explorateur
→ viewport n’a pas été exécuté. Le test egui et la lecture du code montrent que
le parcours actuel est celui de PNG/JPG/WebP, pas celui d’un import PSD avec
preview. Il rejette donc le PSD sans remplacement. Sur startup, le canvas et
son gestionnaire de drop ne sont pas exécutés. Ce parcours PSD absent reste dans
B05, hors de cette mission ; aucune connexion nouvelle à la preview n’a été
ajoutée. Ce rapport ne le déclare pas validé graphiquement.

## Garanties et limites

M001 reste intact : mêmes writers sécurisés, même séparation autosave/primary,
mêmes confirmations et barrières de chargement, même récupération. Aucun
changement de garantie d’atomicité ou de durabilité des fichiers n’est revendiqué.
Les erreurs de sauvegarde et la récupération sont revérifiées par les tests
automatisés M001 ; aucun nouveau crash forcé ni test de coupure électrique n’a
été réalisé pendant cet essai graphique.

L’import conserve un document inactif en mémoire, y compris ses pixels. Son coût
mémoire dépend de la taille du projet ; aucune borne de taille n’est revendiquée.
La preview et File Import demeurent deux parcours existants distincts : File
adopte un nouveau document et réinitialise l’historique ; Assets/preview exécute
la commande undoable. Leur unification relève de B05, pas de M003.1.

Après le checkpoint local, arrêter et attendre la revue avant M003.2.
Aucun push, merge, amend ou release.
