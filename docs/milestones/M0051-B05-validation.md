# M005.1 — B05, parcours PSD unifié

Validation Windows du 15 octobre 2026. Branche `fix/m0051-editor-workflow`,
après le commit B04 `8022196a76b2989aab536d387143817b9ba5dbb0`.
Base intégrée `develop` : M004.2 `b0995586d28b4278fdd7936cfb1fe186f9fa58b0`.

## Audit et défauts reproduits

- File/Import utilisait directement le lecteur de la registry avec ses options
  par défaut et adoptait le résultat sans preview. Le nouveau test de parcours
  échouait avant correction : aucun `pending_psd`, document déjà remplacé.
- Assets possédait déjà la preview, les tags, l'inférence, la sélection de
  couches, le scale, le filtre hidden et la commande ImportPsd. Ce parcours
  confirmait le remplacement directement dans le panneau, sans passer par les
  confirmations de l'application. Aucun deuxième importeur n'était nécessaire.
- Les drops OS arrivaient dans le décodeur d'images du viewport. Le test M003.1
  documentait explicitement le refus PSD ; il vérifie maintenant le staging.
- Le test Windows a démontré des textures périmées après remplacement d'un rig
  déjà affiché : les IDs d'assets sont réutilisés, mais les clés de contenu
  mémoïsées dans Session restaient celles du rig précédent. Le document et les
  exports étaient corrects, le viewport utilisait les mauvais pixels. Une
  régression a échoué avant correction et couvre maintenant trois cycles.
- Le statut d'import dans Session masquait le résultat Save de l'application.
  L'assertion a échoué avant correction ; le statut et le compte rendu sont
  maintenant gérés par le parcours unifié, sans masquer les actions suivantes.

Les preuves avant correction sont conservées hors dépôt :
`work/m0051/b05-before.log`, `b05-cache-before.log`, `b05-status-before.log`.

## Implémentation et garanties

Les trois entrées demandent la même préparation : lecture et `layer_tree` dans
le mécanisme FileLoad existant, puis PendingPsd et la preview déjà présente.
Le panneau Assets ne lit plus le fichier de façon synchrone. Les drops sont
reconnus avant le décodeur d'images, y compris via la requête de session d'un
viewport détaché. Le routing est partagé et insensible à la casse.

Les options Replace et Merge sont visibles séparément. Choisir un fichier ou
modifier les options ne change ni le document ni l'historique. À la confirmation,
le remplacement d'un document modifié passe par Save/Discard/Cancel M001. Cancel
revient aux mêmes options ; un Save échoué garde le document et la confirmation.
La fusion conserve le projet courant et ses animations, sans demander de
discard. Les deux opérations restent une seule commande ImportPsd existante.
Redo utilise le document conservé par M003.1, sans relire le PSD.

La révision et le snapshot de contenu sont contrôlés à la remise de la preview
et à sa confirmation. Un document édité ou un Open plus récent ne peut pas être
remplacé par un résultat tardif. Les raccourcis de fichier, outils et playback
sont suspendus pendant les options PSD. New/Open/Close, autosave et récupération
conservent leurs protections existantes. La preview est dessinée aussi depuis
startup ; le workspace apparaît seulement après import réussi.

Le remplacement réutilise le reset de Session déjà employé par New/Open, extrait
sans changer leur comportement. L'historique de la commande PSD est conservé.
Sélections, skins actifs et caches ne peuvent plus viser des IDs réutilisés d'un
autre rig. Le worker d'export annulé reste joignable jusqu'à la fin du rollback.
Undo/Redo invalident les caches d'assets de session : clés de contenu, thumbnails,
silhouettes et états de sources. Le GPU conserve son cache par contenu ; le
renderer et ses shaders ne sont pas modifiés. Cette invalidation est faite sur
l'historique et l'import, pas à chaque frame ou déplacement continu de bone.

Le compte rendu modal nomme les couches ignorées, les blends non représentés et
les échecs de trace, en plus des compteurs. Le lecteur et le traitement mesh/weights
étaient déjà présents et sont réutilisés, sans changer les formats ni la registry.

## Tests

Neuf nouveaux tests dans `editor/src/app_psd_workflow_tests.rs` :

- File Import stage les options sans mutation ; une lecture invalide garde le rig.
- Requête Assets, drop principal et fallback viewport passent par la même preview ;
  Escape préserve projet, pixels, sélection, historique et dirty state.
- Cancel, Save échoué et Save avant remplacement conservent les garanties M001.
- Replace et Merge font un undo step ; trois cycles restaurent projet, images,
  métadonnées PSD et IDs de bones/assets, puis Save/Open et les exports réussissent.
- Édition pendant la lecture, édition après preview et Open plus récent : aucune
  remise tardive ne remplace le document.
- Import échoué et refus en Animate : aucun undo step ni perte de document.
- Drops mixtes et seconde preview refusés sans import partiel ni perte des options.
- Cache préchauffé d'un ancien rig, remplacement puis trois cycles Undo/Redo :
  clés et pixels identiques à une session neuve sur le document courant ; aucune
  sélection d'un bone différent sous un ID réutilisé.

Les trois tests M003.1 adaptés gardent leurs vérifications de protection. Ils
passent par la confirmation finale après preview ; aucun test n'est supprimé.
Les comparaisons après Save/Open utilisent les noms sur disque et tous les
bytes d'assets ; les IDs mémoire sont comparés séparément pendant Undo/Redo,
conformément à ADR 0004, puisqu'ils sont reconstruits à l'ouverture.

| Contrôle final | Résultat |
|---|---|
| `cargo test --workspace --all-targets` | **1061 réussis**, 0 échec ; M004 1047 + B04 5 + B05 9 |
| Formatage | Conforme |
| Clippy workspace/all-targets | Réussi ; **25 diagnostics**, mêmes messages que la baseline |
| Build workspace | Réussi, dépréciations préexistantes seulement |
| WASM core/render | Réussi |
| TypeScript runtime, tests compilés existants exécutés en lecture seule | **26 réussis** |
| Typecheck core et Phaser, `--noEmit` | Réussi |
| Exporter/évaluateur Rust contre fixture TypeScript | 128 cas identiques |
| JSON Rust contre fixture de round-trip TypeScript | 4 cas identiques |
| Vérification des fixtures Phaser | Réussie |
| Exports après import Replace et Merge | Runtime/atlas, PNG et spritesheet réussis |

Les tests complets reprennent M001 (écriture sûre, staging/rollback, locks Windows,
backups, dirty state, autosave et recovery), M003.1 et M004.1/M004.2. Les snapshots
et formats runtime v1/v2 sont inchangés. Journaux externes : `work/m0051`.

## Windows réel

Copies isolées de walker, de ses assets et de tagged_rig.psd. Originaux inchangés.

1. File/Import/Photoshop PSD : preview visible avant toute mutation, avec couches,
   tags/inférence, Scale, Skip hidden layers et Replace/Merge.
2. Merge dans walker : 12 → 17 bones et 21 images au total. Compte rendu :
   quatre bones, neuf slots, 11 images importées, une séquence de trois frames,
   deux couches ignorées nommées. Undo revient à walker ; Redo restaure l'import.
3. Drop réel exécuté par l'utilisateur depuis l'Explorateur : preview observée
   ouverte, confirmation humaine « oui c'est bon pour moi », capture enregistrée.
   Aucun dépôt OS n'est prétendu produit par une simple injection RawInput.
4. Import Replace sur le document modifié : Save/Discard/Cancel affiché. Cancel
   conserve les 17 bones et les options. Nouvelle demande, Save écrit le rig
   fusionné avant le remplacement à quatre bones. Le fichier sauvegardé a été
   rouvert ensuite avec ses 17 bones et ses bonnes images.
5. Assets : même preview et remplacement ; après correction du cache, 11 images
   PSD et leurs couleurs correctes dans le viewport et les thumbnails. Undo et
   Redo rétablissent correctement les deux jeux de pixels dans le même processus.
6. Save As vers `tagged-final.ankh`, puis Open : relecture avec les 11 images PSD,
   aucune ressource absente. Close/Cancel conserve le document ; Close/Discard
   ferme la copie de test. Les erreurs d'écriture sont injectées par les tests
   natifs ; aucune coupure électrique n'est revendiquée en GUI.
7. Dernier binaire après correction du statut : Open du projet PSD puis Ctrl+S,
   message Saved visible, quatre bones et pixels corrects, fermeture propre.

Computer Use utilise le skill Windows, plus le clavier physique et le dépôt
réel fournis par l'utilisateur. Les boîtes rfd exposent parfois un owner 16×16
avec une fenêtre liée plus grande : après observation, `set_value` et Return
ont permis de choisir les fichiers. Les refus de coordonnées ne sont pas des
défauts produit. Préférences utilisateur restaurées byte pour byte à la clôture.

## Fichiers et périmètre

| Fichiers | Rôle |
|---|---|
| `editor/src/app.rs` | Preview différée, actions de confirmation, guards et drops |
| `editor/src/ui/psd_import.rs` | Préparation partagée, options, décision et rapport |
| `editor/src/ui/assets.rs` | Requête vers le parcours commun |
| `editor/src/ui/canvas/mod.rs` | Routing PSD pour viewport détaché, décodeur image préservé |
| `editor/src/session.rs` | Requête transitoire de preview |
| `editor/src/app_state.rs` | Réutilisation du reset et invalidation des caches d'historique |
| `editor/src/app_psd_workflow_tests.rs` | Neuf régressions de parcours |
| `editor/src/app_lifecycle_tests.rs` | Adaptation des trois tests PSD M003.1 |
| Ce rapport | Résultats, preuves et limites |

Aucune dépendance ajoutée. Aucun changement du runtime, de main, des formats,
du renderer, du MCP ou de l'IA. Develop reste au fast-forward M004 accepté.

## Limites et suite reportée

- Un PSD par drop ; mélanges et dépôts multiples refusés explicitement.
- Drops natifs avec chemin uniquement ; pas de fichier virtuel provenant d'une
  autre application. Le suffixe PSB est routé vers le lecteur existant, sans
  certification de support PSB supplémentaire.
- Le parse de preview est asynchrone, mais la conversion finale, le mesh tracing
  et le dispatch restent ceux de la preview existante, sur le thread UI. De très
  gros PSD peuvent donc encore bloquer pendant Import ; aucun gain de latence ou
  limite mémoire n'est annoncé. Les clones de PendingPsd restent une dette existante.
- L'annulation écarte le résultat d'un lecteur en cours ; elle n'interrompt pas
  le décodeur dans sa lecture. Aucun résultat écarté ne touche le document.
- Le fallback de drop d'un viewport détaché est testé avec egui ; la preuve OS
  réelle porte sur la fenêtre principale. Pas de certification exhaustive des
  dispositions multi-écrans et de tous les focus matériels.
- Une modification de document après preview exige une nouvelle preview ; les
  options obsolètes ne sont pas appliquées automatiquement.
- Les garanties de crash-safety de Save et des exports restent celles de M001 :
  aucune nouvelle revendication d'atomicité multi-fichier ou de durabilité.

M005.2/M005.3/M005.4, M006 et IA non commencés. Deux commits locaux distincts
B04 et B05, sans amend, push, merge distant ou release. Attendre la revue.
