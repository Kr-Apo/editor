---
title: M001c.1 — Autosave au repos
---

# M001c.1 — Rapport de validation

Validation du 14 octobre 2026. Correctif ciblé sur `dev/premium-pipeline`, depuis
`ede5ed640289b3e652c1ee772ac4dfcf560d8444` (M001c), arbre initial propre. Nouveau
commit local autorisé après validation ; aucun amend, push ou travail M002.
Runtime `dev/three-webgpu` conservé à
`f478463cc431550cc8faa9788e52ca5fef667c95`, arbre propre.

## Cause confirmée

L'ancien `Autosave::advance` accumulait les `stable_dt` des frames. L'autosave
ne demandait aucun réveil à son échéance. Dans egui 0.34.3,
`input_state::InputState::begin_pass` utilise le delta prédit quand la frame
précédente n'a pas demandé de repaint immédiat : une attente réactive de trente
secondes ne devient donc pas trente secondes dans cet accumulateur. Le playback
appelle `request_repaint()` et masquait le défaut en produisant des frames.

Audit de `update`, des retours startup/close, des file actions, de `should_autosave`,
de `Autosave::{advance,write,reset}`, du dirty snapshot et des sources locales
egui/eframe. Les garanties du writer et les handlers de récupération ne sont
pas modifiés. Le polling intervient désormais après les modifications/actions
de la frame, y compris sur startup : la dernière interaction arme son réveil.

## Correction

- Temps monotone réel `std::time::Instant`, transmis explicitement au scheduler
  pour permettre des tests sans attente. `Idle / Wait(Duration) / Due` permet à
  l'application d'utiliser `Context::request_repaint_after` avec le temps restant.
- Aucun réveil autosave pour un document propre, désactivation, récupération
  en attente ou contenu identique à la dernière copie réussie. Aucun marquage
  du document comme sauvegardé par l'autosave. Une nouvelle modification après
  copie réussie arme un intervalle complet ; les edits pendant un intervalle
  déjà armé ne repoussent pas son échéance.
- Après erreur : même document/path/sélection/historique/dirty state, erreur
  visible, nouvelle tentative après un intervalle complet ; pas de boucle
  immédiate ni de rattrapage en rafale. Save manuel conserve son reset et les
  anciennes copies. Untitled garde son namespace de récupération isolé.
- Modification du délai appliquée au temps écoulé ; zéro désactive, réactivation
  recommence un intervalle. Un délai manuel énorme utilise des réveils bornés
  à un jour pour éviter un overflow du backend, sans changer l'échéance logique.
- Tooltip corrigé uniquement comme documentation : les copies sont conservées
  après Save et les documents sans chemin ont leur emplacement temporaire.

## Régressions avant implémentation

Le test CPU egui `reactive_idle_frame_must_write_after_thirty_seconds` a échoué
sur le timer précédent avec l'assertion « idle elapsed time must trigger a
recovery copy without playback ». Six frames initiales règlent les repaints de
démarrage, puis aucune frame entre t=0,10 et t=30,10. Cet échec comportemental
est distinct du log de compilation obtenu en définissant la nouvelle API par
tests avant son implémentation.

**11 tests ajoutés**, de 1 002 à **1 013** :

| Fichier | Couverture ajoutée |
|---|---|
| `editor/src/autosave_clock_tests.rs` | 7 : échéance sans frames, clean/disabled/recovery, absence de réécritures puis nouvelle modification, frames playback/sparse, échec et retry sans spin, changements de délai, deadlines dépassées et délais extrêmes |
| `editor/src/app_lifecycle_tests.rs` | 4 : régression réactive, délai egui et arrêt après succès, Save manuel puis nouveau délai, erreur conservant document/sélection/historique/primaire |

Les tests existants du timer sont adaptés à l'horloge contrôlée ; les régressions
M001c de récupération, document sans chemin, verrous Windows, rollback et
préservation des backups continuent à s'exécuter.

## Contrôles automatisés

Rust 1.95.0, dépendances verrouillées, exécution offline sous Windows.

| Contrôle | Résultat |
|---|---|
| `cargo test --workspace --all-targets --locked --offline` | **1 013 pass**, 0 échec, 0 ignoré |
| `cargo fmt --all --check` | Code 0 |
| `cargo clippy --workspace --all-targets --locked --offline` | Code 0 ; **25 diagnostics préexistants**, comparaison des messages au log M001c : aucune différence |
| `cargo build --workspace --locked --offline` | Code 0 |
| `cargo check -p ankhimate-core --target wasm32-unknown-unknown --locked --offline` | Code 0 ; core WASM, pas desktop WASM |
| Runtime `npm test` | **26 pass**, 0 échec/skipped |
| Runtime `npm run build` et `npm run typecheck` | Codes 0, deux workspaces |
| `node scripts/check-rust-parity.mjs C:/path/to/editor C:/path/to/cargo.exe` | 128 références identiques au Rust courant |
| `node scripts/check-rust-strings.mjs C:/path/to/editor C:/path/to/cargo.exe` | 4 round trips identiques |
| Parité Rust/TypeScript | 3 328 composantes, erreur maximale 0,00000643730700744527, tolérance 0,01 |

Aucun nouveau warning de compilation/Clippy. Le format runtime v2, ses axes et
curves, la lecture v1, les presets, escaping/déterminisme, les tests atlas et les
scénarios de publication/rollback export M001b restent validés. Les 61 fichiers
export/runtime du manifeste de protection M001c restent identiques par SHA-256.
La normalisation LF/CRLF habituelle de Git est indépendante de ces diagnostics.

## Test graphique Windows réel

Binaire desktop debug compilé avec ce correctif, backend réactif habituel,
projet isolé `Idle.ankh.json` issu de `tagged_rig.psd` (4 bones, 11 images), délai
réglé dans Settings à **0,5 min / 30 s**. Aucun input UI ni capture pendant chaque
attente ; contrôle du fichier par lecture externe, pas par repaint injecté.

| Scénario | Résultat mesuré |
|---|---|
| Setup, root X de 0 à 1, No clip, aucun playback | Copie écrite à **30,0828771 s** après le début de l'input ; mtime `2026-10-14T15:54:27.2528771Z` ; primaire inchangé |
| Toujours immobile après cette copie | Mtime inchangée jusqu'à t=65,066 s ; aucune réécriture observée |
| Nouvelle modification, root X de 1 à 2, sans playback | Une seule écriture à **30,0467674 s** ; première détection à environ 30,154 s, polling 100 ms ; primaire inchangé |
| File > Save puis document propre au repos 35,107 s | Aucune écriture ; copie conservée ; primaire sauvegardé avec root X=2 |
| Clip créé via Animate puis playback | Copie mise à jour pendant la lecture ; playback encore actif après 35 s, primaire inchangé. L'échéance était déjà armée par la création du clip : les 13,799 s mesurées après Play ne sont pas un nouveau délai de 30 s |

Le premier observateur a démarré tardivement (première lecture à environ
35,310 s) : son délai de 30,083 s provient de la mtime du fichier. Le deuxième
cycle observe continuellement l'échéance et confirme ce résultat. Ces nombres
utilisent l'horloge système des métadonnées pour l'observation, tandis que le
timer de production utilise uniquement l'horloge monotone.

Save final et fermeture normale effectués. Les préférences initiales sont
restaurées octet pour octet après fermeture. Fixture PSD inchangée, SHA-256 :
`8c0cb6c1e1491c40a417e467f171584f9cd2fceb947a07532355f73117a85314`.
Les erreurs d'écriture sont couvertes automatiquement ; aucun nouvel essai
graphique de panne électrique ou de défaut ACL n'est revendiqué.

Journaux locaux : `work/m001c1/{red-reactive,red-clock-api,test,fmt,clippy,build,wasm,runtime}.log`
et `work/m001c1/gui/{idle1-result,idle2-result,clean-result,playback-result}.json`
dans le dossier de travail Codex du 14 octobre. Le rapport graphique M001c
antérieur est conservé dans `outputs/M001c_resultats_graphiques_PSD_2026-10-14.md`.

## Fichiers du checkpoint et limites

Sept fichiers seulement : `editor/src/app.rs`, `editor/src/autosave.rs`,
`editor/src/autosave_clock_tests.rs` (nouveau), `editor/src/app_lifecycle_tests.rs`,
`editor/src/ui/settings.rs`, `docs/project-save-safety.md` et le présent rapport.
Aucune dépendance, aucun writer, renderer, MCP, IA, export ou runtime modifié.

L'OS peut retarder le réveil, l'event loop peut être bloquée, et sérialisation/
écriture restent synchrones. Suspension et temps de traitement peuvent retarder
l'achèvement ; aucune garantie temps réel, atomicité supplémentaire ou durabilité
power-loss. Un réveil déjà programmé peut subsister une fois après Save mais ne
réécrit pas un document propre. Les snapshots de contenu ont un coût sur gros
projets. Les limites de sauvegarde/recovery M001c restent documentées dans
[project-save-safety.md](../project-save-safety.md).

Les observations startup et raccourcis clavier du rapport graphique restent
au backlog, sans correctif inclus. M001c.1 s'arrête à ce checkpoint local.
