# Alpha M007.2 — état et preuves

Candidat local Windows x64, pas une version 1.0 ni une publication. Le manifeste
livré `VERSIONS.json` identifie le checkpoint de documentation/construction ;
`SHA256SUMS.txt` identifie ses fichiers. Le SHA-256 du ZIP est fourni à côté.

## Preuves réutilisées

| Mission | Garantie établie / référence |
|---|---|
| M001a/b | Axes scalaires v2 et lecteur v1 ; parité Rust/TS à 0,01 ; JSON échappé, préparation/publication avec rollback et conservation des backups. |
| M001c/c.1 | Save/Discard/Cancel, Save As après succès, isolation de l’autosave, récupération, horloge monotone et réveil au repos. Checkpoint lifecycle `ede5ed6` et suites actuelles. |
| M003.1 | Import PSD Replace/Merge, document complet après trois cycles Undo/Redo, startup après succès ; `25f3a30`. |
| M003.2/M004 | Phaser jouable, orientation ; PNG/spritesheet déterministes, caméra commune, 60 frames pour 1 s à 60 FPS ; `01980e1`, `b099558`. |
| M005.1/2 | Save/Save As selon le focus, preview PSD commune ; préflight et consentements ; `8022196`, `096658f`, `13f62df`. |
| M005.3/4 | Sombre statique, FFD, UV après changement de skin, clipping ciblé : consommateurs qualifiés par snapshot. Les exclusions d’alpha-scope restent applicables. |
| M006.1 | ZIP Release portable à CRT statique, licences, snapshot Phaser, contrôles de chemins/hashes ; éditeur `8219ca3`, runtime `56dc0f1`. |
| M006.2 | Retour humain du 16 octobre : lancement, PSD, sauvegarde et exports sur un autre PC Windows 11 25H2. Sans détail GPU ni essai Phaser confirmé ; pas une certification de ce nouveau ZIP. |
| M007.0 | Démonstrateur original `be4b2f6` : recalage manuel, pagne retiré, clip frappe_sol 3 s ; conservé, sans enrichissement. |
| M007.1b1 | Drag FFD fondé sur une référence stable, meshes rigides/pondérés ; `77cca66`. |
| M007.1b2 | Retour exact au setup d’un canal déjà animé, sans clés parasites ; `542f78e`. |
| M007.1b3 | Image indépendante par région/skin, import bibliothèque, Undo/Redo/fallback ; `d359952`. 1 139 tests Rust, 34 TS, 153 scénarios WebGL historiques et 39 poses dédiées, 360 bascules rendues. |
| M007.1b4 | Chaque feuille PSD imbriquée importée une fois, parent et ordre source ; `d644653`. 1 153 tests Rust, 14 nouvelles régressions ; 44 → 18 slots/images sur le vrai PSD, 16 bones conservés. Save As/réouverture Windows et export réel relu dans Phaser, 18 meshes distincts. |

Sources et tests : `formats/src/psd.rs`, `formats/tests/psd_nested.rs`,
`document/tests/psd_history.rs`, `editor/src/ui/inspector/image_tests.rs`,
`editor/src/edit_router.rs`, `editor/src/autosave_clock_tests.rs`, `export/tests`,
`render/src/sequence.rs`, `render/src/spritesheet.rs`. Le dépôt runtime séparé
contient les tests core et Phaser. Les rapports locaux détaillés ne sont pas
embarqués : ils contiennent des pièces de travail et des chemins personnels.

## Validation de ce candidat

M007.2 ne modifie aucun code d’édition/rendu/runtime et n’effectue aucun essai GUI.
Une passe Rust workspace locked/all-targets, fmt, Clippy, build Release statique,
WASM core, tests/build/typecheck TypeScript et contrôle de documentation est
effectuée ; les résultats exacts sont joints au rapport local de livraison.
Les avertissements Rust historiques restent visibles : aucun nouveau warning
n’est accepté ; cela n’équivaut pas à un Clippy global `-D warnings` réussi.

Le packaging vérifie PE x64/GUI, DLL système/absence de CRT dynamique, licences,
absence de chemins personnels, intégrité ZIP et hashes après extraction dans
un chemin avec espaces et Unicode. Les quatre commits correctifs sont ancêtres
du checkpoint construit ; le manifeste de construction relie les sources au
hash du nouvel exécutable. La seule version affichée dans l’interface ne suffit pas.

Les batteries WebGL déjà acceptées ne sont pas rejouées en M007.2. Leur historique
n’est pas présenté comme un nouveau test du ZIP. Le dernier smoke manuel du
nouveau candidat reste à effectuer selon [alpha-final-check.md](alpha-final-check.md).
