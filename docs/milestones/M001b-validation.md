---
title: M001b — Fiabilité de l’export et sécurité des fichiers
description: Échappement JSON, validation des chemins, staging et rollback sous Windows.
---

# M001b — Rapport de validation

**14 octobre 2026 — milestone terminé, en attente de revue.** Aucun commit,
staging Git ou push effectué. M001c et M002 ne sont pas commencés.

## 1. État initial et checkpoint proposé

| Dépôt | Branche conservée | HEAD conservé |
|---|---|---|
| `D:\Ankhimate\editor` | `dev/premium-pipeline` | `9ab4fa439c3690ca99325e24f57b24dd7399b90b` |
| `D:\Ankhimate\runtime` | `dev/three-webgpu` | `513a1045d7cdb6f66323f81ac7b43aaee7074450` |

Les changements initiaux correspondent aux fichiers de M001a et aux deux edits
utilisateur de dépendance (`runtime/package-lock.json`, `runtime/phaser/package.json`,
alignement vers `@ankhimate/runtime@0.1.1`). Aucun autre changement initial identifié.
Le sous-module communautaire reste à `b0e7739f0004af935f9b6b5a6f9c8023fefe37bc`.
Les index Git restent vides.

Les rapports M000/M001a ont été relus. Avant toute correction, les **79 tests de
la crate export** et les **25 tests TypeScript core** passent ; les builds/typechecks
Phaser passent aussi. Le gate inter-dépôts reproduit les **128 références M001a**.
La validation complète M001a enregistrait 936 tests Rust et 25 tests TypeScript.

Une copie pré-M001b des fichiers modifiés/non suivis, des diffs binaires Git et
des HEAD a été conservée hors des dépôts. Elle permet de distinguer les versions
M001a des versions actuelles des fichiers partagés. **Cette copie n’est pas un
checkpoint Git et aucun commit n’a été créé.**

Stratégie après autorisation explicite :

1. Créer un commit M000/M001a ciblé dans l’éditeur : les deux rapports, le contrat
   runtime, le preset v2, ses régressions, la fixture et le générateur de parité.
   Proposition : `feat(export): preserve scalar axes in runtime format v2`.
2. Créer un commit M001a ciblé dans le runtime : types, lecteur, évaluateur,
   régressions, références, gate de parité et README à leurs versions pré-M001b.
   Proposition : `feat(core): read runtime v2 with independent axes`.
3. Garder l’alignement Phaser utilisateur séparé de ces deux commits ; demander
   son autorisation propre avant de le committer. Ne pas utiliser `git add .`.
4. Pour les fichiers partagés, indexer les versions du snapshot pré-M001b après
   revue et vérifier le checkpoint dans un checkout isolé ; conserver le contenu
   M001b dans l’arbre de travail. Puis soumettre M001b à des commits distincts.

Cette stratégie reste une proposition ; elle n’a modifié ni branches ni index.

## 2. Bugs confirmés et causes

| Défaut | Cause | Correction |
|---|---|---|
| Un guillemet dans un nom produit un JSON invalide ; backslashes/contrôles peuvent altérer les chaînes | Interpolation brute `"{{name}}"`, avec HTML escaping désactivé volontairement | Sérialiser les valeurs et clés avec `{{json name}}`, basé sur `serde_json` |
| JSON mal formé publié depuis un preset sauvegardé/personnalisé ou une sortie binaire | Aucun contrôle de syntaxe avant écriture | Parser tous les outputs `.json` avant toute mutation disque ; erreur avec ligne/colonne |
| Un échec ultérieur laisse le squelette déjà écrasé | Ancien `write_one` utilise `fs::write` successivement, sans staging ni sauvegarde | Préparer tous les fichiers, puis remplacer avec backups et rollback inverse |
| Les images peuvent échapper à la racine ou écraser un texte | Confinement/collisions appliqués uniquement aux fichiers de templates | Validation commune textes, pages atlas et images copiées ; répétée pour les `Plan` construits directement |
| Collisions de casse, séparateurs et préfixes fichier/dossier ; noms Windows/ADS problématiques | Comparaison exacte de chaînes et validation lexicale incomplète | Normalisation, comparaison de casse, détection des préfixes, noms portables et canonicalisation des préfixes existants |
| Une jonction peut rediriger des écritures ou le relevé des orphelins | Parents non inspectés ; parcours utilisant `is_dir` et suivant les liens | Métadonnées sans suivi ; refus des symlinks/reparse points ; relevé sans descente dans les jonctions |

Le défaut de perte des canaux et courbes corrigé en M001a reste couvert par ses
tests inchangés ; aucune modification du contrat d’animation n’était nécessaire.

## 3. Fichiers et fonctions concernés

Sérialisation native : `export/src/presets/ankhimate_runtime.json`, corps du
template `skeleton`. Le rendu passe par `template::Engine::render` et le helper
`json_helper` (`export/src/template.rs`, lignes 71/155), déjà basé sur `serde_json`.
`Context::build` et `Context::to_value` restent inchangés. Le format reste un
template, conformément au contrat du dépôt.

Écriture multi-fichiers : `run::plan_with` prépare le contexte, les templates,
l’atlas et les images ; `run::export` appelle `run::write`. L’ancien `write_one`
est remplacé par le module privé `staged`, notamment `validate_path`,
`validate_paths`, `validate_destination`, `destination_key`, `write_with` et
`cleanup_stage`. `run::walk` évite de suivre les jonctions. Les champs publics de
`Plan` et l’API des consommateurs restent inchangés.

Fichiers modifiés/ajoutés **par M001b**, distincts des autres fichiers M001a déjà
présents dans le diff Git :

| Éditeur — relatif à `D:\Ankhimate\editor` | Rôle |
|---|---|
| `export/src/presets/ankhimate_runtime.json` | Chaînes/clés JSON sérialisées, v2 conservé |
| `export/src/presets/generic_json.json` | Même correction, schéma générique conservé |
| `export/src/presets/phaser_atlas.json` | Clés de frames et nom de page sérialisés |
| `export/src/lib.rs` | Module privé du writer |
| `export/src/run.rs` | Validation commune, délégation au staging, relevé sans suivi des liens |
| `export/src/staged.rs` — nouveau | Préflight JSON/chemins, staging, lock, manifest, publication et rollback |
| `export/tests/export.rs` | Deux tests de chaînes, dont round-trip jusqu’au disque |
| `export/tests/safety.rs` — nouveau | Douze régressions de sécurité exécutées sur Windows |
| `export/examples/runtime_strings_roundtrip.rs` — nouveau | Quatre vrais exports natifs pour le lecteur TS |
| `docs/export-safety.md` — nouveau | Garanties, compatibilité et récupération manuelle |
| `docs/export-plan.md` | Remplace la promesse absolue d’atomicité par les garanties réelles |
| `docs/formats/export-runtime.md` | JSON et writer commun documentés |
| `docs/milestones/M001b-validation.md` — nouveau | Présent rapport |

| Runtime — relatif à `D:\Ankhimate\runtime` | Rôle |
|---|---|
| `core/test/export-strings.test.ts` — nouveau | Round-trip des vraies sorties Rust et évaluation X=5 à t=0,5 |
| `core/test/fixtures/runtime_strings_roundtrip.json` — nouveau | Références générées par Rust, pas des attentes reconstruites en TS |
| `scripts/check-rust-strings.mjs` — nouveau | Contrôle anti-obsolescence en lecture seule |
| `core/README.md` | Utilisation du nouveau gate |

Les fichiers `core/src/types.ts`, `rig.ts`, `evaluate.ts`, les fixtures/tests/gates
de parité M001a et `editor/export/src/context.rs` sont inchangés octet pour octet.
Les deux fichiers d’alignement Phaser sont également identiques aux empreintes
relevées au début. Aucun renderer, MCP, interface d’animation, dépendance ou format
communautaire n’est modifié.

## 4. Régressions et résultats

Avant correction : deux tests JSON échouent au parsing, les **46 anciens tests
export** restent verts. Les premiers tests de sécurité donnent **5 échecs et
1 succès**, reproduisant l’écrasement avant erreur, l’évasion d’un Plan direct,
les collisions et les noms Windows. Deux tests supplémentaires reproduisent
ensuite la publication de JSON mal formé, avant d’ajouter le contrôle de syntaxe.
Ces journaux rouges sont conservés avec les résultats définitifs.

**21 nouveaux tests Rust sur Windows :** deux dans `export.rs`, douze dans
`safety.rs`, sept tests unitaires de transaction. Ils couvrent :

- noms de projet/os/slots/skins/attachments/animations, références et payloads
  d’événements ; guillemets, backslashes, tous les U+0000–U+001F, accents, CJK,
  emoji, U+2028/U+2029 et caractères spéciaux ; rendu des trois presets JSON ;
- JSON mal formé dans une sortie ultérieure ou binaire `.JSON`, racine vide,
  évasion, collisions texte/image, casse, slash/backslash et préfixes ;
- noms de périphériques Windows, ADS, suffixes interdits, atlas/images copiées,
  destinations en lecture seule, jonctions réelles et orphelins ;
- préparation, flush, backup et publication : **11 points d’échec injectés** dans
  un test paramétré ; anciens octets restaurés, nouveaux fichiers/parents retirés ;
- restauration/removal impossible : backups et manifest conservés, erreur
  explicite ; lock occupé conservé ; cleanup après publication signalé correctement ;
- fichier réellement ouvert sous Windows sans partage DELETE : le rename échoue
  après les premières publications et le rollback restaure tous les anciens octets.

Le test Unix de symlink est conditionnel et n’a pas été exécuté sur ce poste.
Il n’est pas compté parmi les 21 nouveaux tests Windows.

**Un nouveau test TypeScript**, sur quatre sorties Rust : mêmes noms/payloads,
keys `__proto__`, `constructor`, `toString` et X seule évaluée à 5 à t=0,5.
Aucune adaptation du lecteur/évaluateur n’a été requise.

| Commande finale | Résultat |
|---|---|
| `cargo test --workspace --all-targets --locked --offline` | **957 réussis**, 0 échec, 0 ignoré ; 936 M001a + 21 |
| `cargo fmt --all --check` | Code 0 |
| `cargo clippy --workspace --all-targets --locked --offline` | Code 0 ; **25 diagnostics exactement identiques à M001a**, aucun nouveau warning |
| `cargo build --workspace --locked --offline` | Code 0, build dev complet |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown --locked --offline` | Code 0 |
| Runtime `npm test` | **26 tests core réussis**, 0 échec/skipped ; Phaser build/typecheck verts |
| Runtime `npm run build` | Code 0, deux workspaces |
| Runtime `npm run typecheck` | Code 0, deux workspaces |
| `node scripts/check-rust-parity.mjs C:/path/to/editor C:/path/to/cargo.exe` | **128 références identiques** au Rust actuel |
| `node scripts/check-rust-strings.mjs C:/path/to/editor C:/path/to/cargo.exe` | **4 références identiques** au Rust actuel |
| `git diff --check`, deux dépôts | Codes 0 |
| Comparaison SHA-256 et snapshot M001a | Toutes les protections vérifiées ; anciens tests/reports/fixtures inchangés |

La parité M001a compare toujours **3 328 composantes**, avec une tolérance de
**0,01** et le même écart maximal **0,00000643730700744527**. Les snapshots numériques
n’ont pas été régénérés. Les tests existants de déterminisme, absence de timestamp,
atlas et presets passent dans la suite complète. Les noms temporaires n’entrent
pas dans le contenu exporté.

Clippy n’utilise pas `-D warnings` : les avertissements historiques restent une
dette. Le runtime n’a pas de formatter/linter configuré ; style revu et whitespace
Git vérifié. Le contrôle documentaire supplémentaire était déjà rouge à M000/M001a
(références CRLF et lien documentaire non résolu) ; il n’a pas été relancé ni
modifié ici. Les liens relatifs des nouveaux documents sont vérifiés sur disque.

## 5. Garanties de sécurité et limites

Le [contrat détaillé](../export-safety.md) décrit le mécanisme : lock exclusif,
staging sous la racine, manifest sérialisé, `write_all`/`sync_all`, handles fermés,
backup par rename, publication, puis nettoyage limité aux fichiers temporaires
appartenant à la transaction.

Un échec de validation/préparation conserve les destinations précédentes. Un
échec de publication avec rollback réussi restaure les fichiers précédents et
retire les nouveaux outputs. Si le rollback échoue, aucun succès n’est annoncé :
les backups restants, leur manifest et leur emplacement sont conservés/signalés.
Un échec de cleanup après succès indique explicitement que les outputs sont
déjà publiés. Les orphelins ne sont jamais supprimés.

**Pas d’atomicité multi-fichiers parfaite.** Les lecteurs peuvent observer un
ensemble mixte ou une absence temporaire entre deux renames. Un arrêt brutal,
crash OS ou coupure électrique peut laisser une publication partielle, des
backups et un lock. Le manifest est un inventaire, pas un journal de commit.
La durabilité des renames de répertoires n’est pas garantie par `sync_all` des
fichiers. Pas de récupération automatique après crash dans M001b.

Le lock coordonne seulement les exporters coopérants sur la même racine. Les
changements concurrents d’autres logiciels, les courses sur les permissions et
la topologie, les règles exotiques de casse/normalisation et les filesystems
réseau/virtuels restent des limites. La publication ne promet pas de conserver
les timestamps, l’identité de fichier ou les ACL de la destination précédente.
La conservation/restauration concerne les fichiers d’origine par rename et leurs
octets ; la nouvelle sortie a les métadonnées du staging.

## 6. Compatibilité et dette restante

- Runtime **v2**, axes scalaires, courbes indépendantes et offsets baked conservés ;
  lecteur v1 implicite/explicite inchangé. Context/preset restent v1. Aucun package
  n’est publié et aucun numéro npm ne change.
- Presets générique/Phaser : schémas et atlas inchangés ; seules les interpolations
  de chaînes sont corrigées. Formats communautaires et autres extensions inchangés.
- Compatibilité volontairement plus stricte sur les chemins : collisions, noms
  non portables, racines/parents liés, read-only et `.json` mal formés sont refusés.
  Un nom JSON valide peut être invalide comme filename ; adapter le template de
  chemin ou le nom pour les exports par animation/images libres.
- Presets sauvegardés/personnalisés : corriger les interpolations avec `json` ou
  sélectionner le preset intégré actuel. La vérification `.json` protège les
  fichiers précédents mais ne réécrit pas le template. Pas de migration silencieuse.
- Dette : journal/recovery durable, isolation des lecteurs, protection contre
  writers non coopérants, validation sémantique de formats tiers, métadonnées/ACL,
  tests exécutés sur Unix/filesystems réseau, coûts mémoire/disque des gros exports,
  warnings et gate documentaire historiques. Le nettoyage d’orphelins reste
  délibérément une décision utilisateur.

M001b s’arrête à cette livraison. Aucun travail M001c/M002 n’est engagé ; la suite
et les commits nécessitent une nouvelle autorisation.
