---
title: M001c — Sécurité des projets et récupération
---

# M001c — Rapport de validation

Livraison du 14 octobre 2026, prête pour revue, **sans commit ni push**. Aucun
travail M002. Le contrat complet et ses limites figurent dans
[project-save-safety.md](../project-save-safety.md).

## Périmètre et état initial

- Éditeur : `dev/premium-pipeline`, HEAD
  `32655bc68cb93aa6ca82830903b51beb5f29d2f3`, arbre initial propre.
- Runtime : `dev/three-webgpu`, HEAD
  `f478463cc431550cc8faa9788e52ca5fef667c95`, arbre initial et final propres.
- M001a/M001b et l’alignement Phaser sont conservés. Aucun changement du rendu,
  de l’intégration Three.js, de l’architecture MCP, des fonctionnalités IA, du
  schéma projet ou des formats/presets d’export. Aucun historique réécrit.

Avant modification, audit de `AnkhimateApp`, `FileAction`, `FileLoad`,
`resolve_frame`, `run_file_action`, `poll_file_load`, `AppState::replace_document`,
du compteur de révision et de l’historique, de `fileops`, `Autosave`, de la
découverte des copies et du retour anticipé de la page de démarrage, puis de
`formats::save/load`, `container::atomic_write`, du confinement et de la
dénomination des images externes. Les rapports M000/M001a/M001b et le checkpoint
précédent servent de référence.

## Causes racines et corrections

| Problème confirmé | Cause | Correction |
|---|---|---|
| New/Close/Open sans protection | Pas de point de sauvegarde ni de contrôleur commun ; New remplaçait immédiatement le document | Point de sauvegarde basé sur le contenu persistant ; confirmation Save/Discard/Cancel ; interception de la fermeture native |
| Résultat de chargement tardif | Le lecteur pouvait adopter son résultat après remplacement ou édition du document | Récepteur courant + garde de révision et de contenu ; invalidation des anciens lecteurs ; handoff suspendu pendant les prompts |
| Fichiers temporaires/backups écrasables | Noms `.tmp` et `.bak-PID` réutilisés ; rename Windows peut remplacer un fichier existant | Stage créé exclusivement, lock par destination et manifest ; aucun backup existant réutilisé |
| Répertoire remplacé par un projet | Test `exists()` insuffisant, sans validation du type de destination | Validation des fichiers et de tous les parents existants avant écriture |
| Restauration silencieusement défaillante | Erreur de rename inverse ignorée ; aucune donnée de récupération exploitable | Erreur explicite, backup conservé, inventory des stages ; aucune restauration sur destination occupée |
| Copie de récupération supprimée par une autre session/Save As | `discard()` supprimait un chemin calculé sans preuve de propriété ; untitled global réutilisé | Conservation des copies ; namespace untitled par processus/document ; maintien de la lecture du chemin legacy |
| Récupération manquante/masquée | Chemin demandé perdu si ouverture impossible ; mtime d’un primaire corrompu utilisé ; retour startup avant rendu du prompt | Recherche au chemin demandé, distinction primaire valide/invalide, prompt également sur startup |
| Perte de provenance PSD après Open | Reconstruction partielle de `Document` malgré l’existence de `Document::from_loaded` | Réutilisation de cette abstraction pour Open/adoption/récupération |
| Sauvegarde avec image externe corrompue | Toute image existante était considérée valide sans comparer ses octets | Réutilisation seulement si octets identiques ; sinon échec préservant les fichiers existants |
| Profil JSON en majuscules écrit comme binaire | Détection de suffixe sensible à la casse | Reconnaissance sans casse ; namespace des assets inchangé |

Save As gardait déjà son chemin en cas d’erreur normale dans `FileOutcome` ; la
correction centralise ce comportement et teste les erreurs réelles/verrous. Le
compteur de révision demeure un compteur d’invalidation : il ne devient pas un
faux indicateur de contenu sauvegardé. La récupération conserve aussi les
warnings de références/images manquantes et marque le document récupéré dirty.

## Tests de régression et méthode

**45 tests ajoutés**, passant de 957 à 1 002 tests Rust :

| Emplacement | Ajouts et couverture |
|---|---|
| `editor/src/app_lifecycle_tests.rs` | 25 : Save/Discard/Cancel, préservation sélection/historique/undo/redo, annulation du picker, erreurs Save avant New/Open/Close, Save As verrouillé, demandes rapides, chargement asynchrone, point de sauvegarde/metadata, récupération manquante/corrompue/périmée, stage interrompu avec assets, Échap/Close natif via CPU egui, warnings de récupération |
| Tests `app.rs` | 2 : New ne détruit pas un document modifié ; un ancien Open ne remplace pas un document plus récent |
| Tests `autosave.rs` | 3 : copie d’une autre session conservée, namespace untitled indépendant, modifications persistantes sans bump de révision |
| Tests `fileops.rs` | 1 : provenance PSD conservée après sauvegarde/ouverture |
| Tests `container.rs` | 2 : anciens temp/backups inchangés, répertoire de destination jamais déplacé/remplacé |
| Tests `formats/src/lib.rs` | 4 : image corrompue, réutilisation d’image identique en lecture seule, parent inexistant rejeté avant assets, suffixes JSON en majuscules |
| Tests `safe_write.rs` | 8 : préparation/flush/backup/publication, échec restauration, échec cleanup après publication, vrai verrou Windows, paths invalides/lock occupé, device namespaces, lecture seule/PermissionDenied injecté, rollback sur destination réoccupée |

Les tests ont été écrits avant les corrections correspondantes. **8 échecs
d’assertion reproduisent les défauts du checkpoint d’entrée** : 2 lifecycle,
2 writer, 1 autosave, 1 provenance, 1 asset corrompu et 1 profil JSON. Les nouvelles
interfaces de confirmation/récupération ont aussi été définies par tests avant
implémentation, avec des échecs de compilation d’API absente, distincts de ces
reproductions comportementales. La revue du nouveau code a ensuite produit
4 échecs d’assertion supplémentaires, corrigés avant validation : readonly asset,
parent inexistant, device namespace et warning de récupération perdu.

Les étapes du writer possèdent un seam privé d’injection de fautes, jamais une
option activable par l’utilisateur. Les verrous Windows utilisent de vrais
handles refusant le partage DELETE. Les interruptions sont simulées par des
fichiers laissés dans un stage avec manifest, primaire déplacé et payload tronqué ;
ce ne sont pas des tests de coupure électrique. Les tests de lifecycle utilisent
les handlers de production sans ouvrir de sélecteur natif. `Config::save` est
neutralisé uniquement en compilation de tests pour ne pas modifier les vrais
paramètres utilisateur ; le comportement de production reste identique.

## Validation finale

Environnement Windows local, Rust 1.95.0, dépendances verrouillées et offline.

| Contrôle | Résultat |
|---|---|
| `cargo test --workspace --all-targets --locked --offline` | **1 002 réussis**, 0 échec, 0 ignoré |
| `cargo test --workspace --doc --locked --offline` | Code 0 ; aucun doctest existant |
| `cargo fmt --all --check` | Code 0 |
| `cargo clippy --workspace --all-targets --locked --offline` | Code 0 ; **25 diagnostics préexistants**, messages comparés au log postcommit M001b : aucune différence |
| `cargo build --workspace --locked --offline` | Code 0 |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown --locked --offline` | Code 0 ; périmètre WASM du core, pas de build WASM du desktop |
| Runtime `npm test` | **26 réussis**, 0 échec, 0 skipped ; Phaser build/typecheck réussis |
| Runtime `npm run build` / `npm run typecheck` | Codes 0, deux workspaces |
| `node scripts/check-rust-parity.mjs C:/path/to/editor C:/path/to/cargo.exe` | **128 références identiques** au Rust courant |
| `node scripts/check-rust-strings.mjs C:/path/to/editor C:/path/to/cargo.exe` | **4 références identiques** au Rust courant |
| Comparaison TypeScript/Rust | **3 328 composantes**, erreur maximale **0,00000643730700744527**, tolérance 0,01 |
| Protection source | **61 fichiers** export/runtime vérifiés par SHA-256 : aucune modification |
| `git diff --check`, deux dépôts | Codes 0 ; runtime propre ; HEAD inchangés |

Les 25 diagnostics Clippy restent ceux du checkpoint précédent : suggestions
préexistantes document/plugins/PSD/UI, 3 dépréciations de l’éditeur, helper de test
core inutilisé et affectation de test document inutilisée. Aucun nouveau warning.
Git signale sa normalisation LF/CRLF habituelle, sans whitespace error.

La suite inclut toujours la parité des axes/curves M001a, déterminisme d’export,
escaping/round trips M001b, publication/rollback/backups export et tests des
atlases/presets. Le répertoire `export` et le runtime sont inchangés octet pour
octet ; leurs contrôles sont réexécutés, pas seulement déduits de ce constat.

## Fichiers modifiés

| Fichier | Rôle |
|---|---|
| `editor/src/app.rs` | Confirmation, point sauvegardé, fermeture, barrières de chargement, récupération et rendu startup |
| `editor/src/app_lifecycle_tests.rs` — nouveau | Régressions du contrôleur et CPU egui |
| `editor/src/autosave.rs` | Détection par contenu, isolation untitled, erreurs visibles, recherche de récupération |
| `editor/src/fileops.rs` | Adoption complète du Document, suffixes Save As, test provenance |
| `editor/src/config.rs` | Protection des paramètres utilisateur pendant les tests seulement |
| `formats/src/container.rs` | Délégation du writer et inventory des copies récupérables, régressions |
| `formats/src/lib.rs` | Assets préparés/validés avant publication, profils inchangés, chargement avec namespace d’origine |
| `formats/src/safe_write.rs` — nouveau | Publication, validation Windows, lock, backup, rollback, tests de faute |
| `docs/project-save-safety.md` — nouveau | Contrat, procédure, garanties et limitations |
| `docs/milestones/M001c-validation.md` — nouveau | Présent rapport |

Aucune dépendance ajoutée, aucune modification de Cargo.lock ou package-lock,
aucun fichier du runtime changé. Les changements sont non indexés, en attente
de revue ; il n’existe aucun commit M001c.

## Garanties, compatibilité et dette restante

Le nouveau fichier est préparé et flushé avant de déplacer l’ancien. Une erreur
ordinaire préserve/restaure celui-ci. Si le rollback échoue, le backup et son
manifest restent disponibles et l’échec est explicite. Save échoué, même après
publication avec cleanup échoué, n’autorise aucune destruction du document.
Save As ne change son chemin qu’après succès complet. La récupération ne
sauvegarde jamais silencieusement sur le primaire et Ignore garde la copie.

**Aucune atomicité multi-fichier ni durabilité power-loss revendiquée.** Une
fenêtre sans primaire existe entre renames ; directories/rename metadata ne sont
pas synchronisés. Un crash peut laisser stage et lock ; le lock n’est pas effacé
automatiquement. Les assets non référencés et copies conservées peuvent
s’accumuler. Les writers externes non coopératifs peuvent courir entre les checks
et renames. Le writer conserve les octets, pas nécessairement les ACL/metadata
du fichier remplacé ; permissions héritées du dossier.

Ankh v1, runtime v2, lecteur runtime v1 et presets sont compatibles sans migration.
Les chemins invalides, liens/junctions/reparse points et destinations read-only
sont volontairement refusés pour écrire ; choisir une destination ordinaire.
Les assets identiques read-only restent utilisables. Les profils en majuscules
sont maintenant reconnus comme leurs équivalents minuscules, sans changer leur
namespace d’assets existant.

Dette hors M001c : sauvegardes/snapshots synchrones sur gros projets ; arrêt
physique des readers ; navigateur de plusieurs copies et purge contrôlée ;
arbitrage de documents partagés entre sessions ; recovery legacy `.bak` manuel ;
tests de coupure électrique/SMB/ACL réelles et parcours GUI natif avec GPU. La
récupération propose la copie admissible la plus récente ; mtime et Ignore par
session restent des conventions existantes, pas une garantie de versioning.

M001c s’arrête à cette livraison. Attendre la revue et l’autorisation avant tout
commit, puis une autorisation distincte avant le milestone suivant.
