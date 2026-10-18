---
title: M000 — Baseline du pipeline premium
description: Validation reproductible du 14 octobre 2026, avant les corrections M001a.
---

# M000 — Baseline validée le 14 octobre 2026

## État initial et périmètre

| Dépôt | Branche | Commit | État initial |
|---|---|---|---|
| `D:\Ankhimate\editor` | `dev/premium-pipeline` | `9ab4fa439c3690ca99325e24f57b24dd7399b90b` | Propre |
| `D:\Ankhimate\runtime` | `dev/three-webgpu` | `513a1045d7cdb6f66323f81ac7b43aaee7074450` | `package-lock.json` et `phaser/package.json` modifiés par l'utilisateur |

Les deux modifications existantes alignent la dépendance Phaser sur
`@ankhimate/runtime@0.1.1`. Elles ont été conservées, sans résolution de dépendances
ni réécriture du lockfile. Aucun commit, push ou changement de branche effectué.

Le sous-module `community-plugins` est désormais initialisé au commit épinglé
`b0e7739f0004af935f9b6b5a6f9c8023fefe37bc`. Les dépendances Cargo et npm nécessaires
aux commandes ci-dessous étaient disponibles. Aucune installation n'a été
effectuée pendant cette mission. Les blocages de cache relevés par l'audit initial
ne s'appliquent donc plus à cette baseline.

Environnement exécuté : Windows, Rust **1.95.0** (toolchain du dépôt), Node
**24.18.0**. Cette baseline ne certifie pas macOS/Linux, un usage graphique prolongé
ou la parité visuelle du futur renderer Three.js.

## Commandes et résultats avant modification du code produit

| Commande | Résultat |
|---|---|
| `cargo test --workspace --all-targets --locked --offline` | **933 tests réussis**, aucun échec, aucun test ignoré |
| `cargo clippy --workspace --all-targets --locked --offline` | Code 0, warnings préexistants conservés |
| `cargo fmt --all --check` | Code 0 |
| `cargo build --workspace --locked --offline` | Code 0 ; build dev, pas nouveau lancement graphique ni build release |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown --locked --offline` | Code 0 |
| Runtime : `npm test` | **17 tests core réussis** ; Phaser build/typecheck réussis |
| Runtime : `npm run build` | Les deux workspaces réussissent |
| Runtime : `npm run typecheck` | Les deux workspaces réussissent |

Répartition Rust : core 190, document 185 (168 + 7 + 10), editor 235, export 76
(13 atlas + 43 export + 20 template), formats 88, MCP 24, plugins 130, render 5.
Les exemples et xtask exécutent aussi leurs cibles de tests, sans test déclaré.
La commande `--all-targets` ci-dessus n'exécute pas les doctests : aucun résultat de
doctest supplémentaire n'est revendiqué.

Cargo et rustfmt sont installés sous `<user-home>/.cargo/bin` sur ce poste ;
utiliser le chemin absolu si le shell n'a pas ce répertoire dans son PATH.
`git submodule status` nécessite ici une exécution hors des restrictions de
processus du sandbox ; son échec restreint n'est pas un défaut du dépôt.

## Warnings préexistants

Build/test de l'éditeur :

- `editor/src/ui/plugin_panel.rs:593` : `egui::SelectableLabel` et son constructeur
  dépréciés, soit deux diagnostics ;
- `editor/src/ui/canvas/camera.rs:39` : `glam::Mat4::orthographic_rh` dépréciée.

Tests : helper `shear` inutilisé dans `core/src/pose.rs:1185` ; affectation à `doc`
inutilisée dans `document/src/commands/mod.rs:555`.

Clippy ajoute les avertissements déjà présents : boucle d'indices dans
`document/src/shape_ops.rs`, grande variante d'erreur et types complexes dans
`plugins`, éléments après modules de tests, `vec!` inutiles dans `formats`,
initialisation suivie d'affectation dans des tests `document`, conditions
imbriquées et clones inutiles dans l'éditeur. La commande retourne 0 ; elle n'a
pas été exécutée avec `-D warnings`. Aucun `clippy --fix` utilisé.

Ces warnings restent enregistrés comme dette existante. Les corriger n'est pas
une condition implicite pour étendre le périmètre de M001a.

## Limitation documentaire préexistante

Le contrôle supplémentaire `cargo run --locked --offline -p xtask -- docs-check`
a été essayé pendant la finalisation documentaire. Il compare les références
générées octet par octet : les fichiers checkoutés en CRLF étaient signalés comme
stale alors que leur contenu normalisé était identique. Un essai de `docs-sync`
a rétabli LF dans ces deux références, **sans changement de contenu ni diff Git
normalisé**. Après ce diagnostic, les deux fichiers ont été restaurés octet pour
octet afin de ne pas livrer de changements de fins de ligne hors périmètre.

Pendant l'essai avec les références en LF, le contrôle échoue ensuite sur le lien déjà présent dans
`docs/formats/index.md` : `../format-spec/`. Le checker attend un chemin existant
sur disque, tandis que ce lien emploie une route documentaire. Ni cette page ni
le checker ne sont modifiés par M001a. Sur l'état livré, `docs-check` s'arrête de
nouveau au diagnostic CRLF/stale ; le problème de lien sous-jacent reste aussi
identifié. Il reste donc un gate documentaire rouge
préexistant, distinct des commandes Rust/TypeScript/WASM vertes listées ci-dessus.

## Conclusion M000

La baseline de compilation et de tests est reproductible sur ce poste avec les
versions verrouillées. Elle ne démontre pas la fidélité du contrat d'export : les
régressions mono-axe et courbes X/Y identifiées dans l'audit restent les entrées de
M001a. Le résultat de ce milestone est décrit dans
[M001a — validation](M001a-validation.md).
