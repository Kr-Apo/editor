# M005.1 — B04, raccourcis de sauvegarde

Validation Windows du 15 octobre 2026. Branche `fix/m0051-editor-workflow`.

## Intégration préalable autorisée

Avant toute modification : éditeur propre, `develop` à
`25f3a301920a4139db16a7820c42bf5d7c15c822`,
`feat/m0042-spritesheets` à `b0995586d28b4278fdd7936cfb1fe186f9fa58b0`.
Ascendance vérifiée : exactement deux commits supplémentaires, M004.1
`01980e1c5a60a804cdf3f53b37d6be924986a111`, puis M004.2. Fast-forward local
effectué ; `develop` reste ensuite à M004.2. Branches conservées. `main` reste à
`9ab4fa439c3690ca99325e24f57b24dd7399b90b` ; runtime propre et inchangé à
`6d8f3454aacd6b69fbc3c242b54ba98e19c23f79`. Aucun push, merge distant ou release.

## Défauts démontrés et correction

Le bloc des raccourcis fichier a été isolé sans changement de comportement dans
`AnkhimateApp::file_shortcut`, puis testé avec de vrais événements `egui::RawInput`.
Trois tests échouaient avant correction :

- Ctrl+S pendant la capture Settings déclenchait Save avant que la capture soit
  consultée ; Ctrl+Shift+S présentait le même risque avec Save As.
- Le test utilisait les modificateurs globaux de la frame : Ctrl relâché avant
  le repaint faisait perdre Save, et le S restait disponible pour Scale.
- Inversement, Ctrl pressé après un simple S pouvait transformer ce S en Save.

La décision utilise maintenant les modificateurs portés par l'événement Key,
refuse Alt/MacCmd, respecte la capture Settings et consomme les touches de
fichier avant le keymap et les widgets. Les répétitions sont consommées sans
rouvrir des dialogues. Aucune modification des bindings ni du mécanisme d'écriture.
Save/Save As passent toujours par les mêmes actions différées et fileops M001.
Les confirmations et la récupération gardent la priorité.

## Régressions et validations

Cinq nouveaux tests dans `editor/src/app_shortcut_tests.rs` : capture Settings,
relâchement Ctrl avant la frame et absence de Scale, ordre inverse/modificateur
Alt, plusieurs états de focus sans altération des valeurs, sauvegarde réelle par
le dispatch et préservation d'un New suspendu par Cancel.

| Contrôle | Résultat |
|---|---|
| Baseline M004 intégrée | 1047 tests Rust réussis |
| Régressions avant correctif | 3 échecs de comportement, 2 réussites |
| Tests ciblés après correction | 5/5 réussis |
| `cargo test --workspace --all-targets` | 1052 réussis, zéro échec |
| Formatage | Conforme |
| Clippy workspace/all-targets | Réussi, 25 diagnostics comme la baseline |
| Build workspace | Réussi, dépréciations préexistantes seulement |
| WASM core/render | Réussi |

Journaux externes : `work/m0051/b04-*.log`. Les contrôles de compatibilité globaux
seront aussi repris à la clôture B05, sans écriture dans le dépôt runtime.

## Clavier physique et Computer Use

Le binaire a ouvert une copie isolée de walker avec ses assets. L'utilisateur
a reçu le protocole viewport : Ctrl+S, Ctrl+Shift+S puis Cancel ; ensuite Filter :
Ctrl+Shift+S puis Cancel, en vérifiant Translate. Après indication de l'emplacement
de Filter, il a répondu « ok c'est bon de mon coté ». Il s'agit d'un retour humain,
pas d'une mesure instrumentée des événements matériels. L'état observé ensuite
montre Saved, le focus dans Filter, les 12 bones et Translate inchangé.

Computer Use a également ouvert Save As depuis Filter et annulé avec Échap.
Dans Settings/Keys, le bouton « press keys… » reçoit Ctrl+Shift+S, affiche cette
combinaison et n'ouvre aucune boîte Save As. Captures externes remises avec le
rapport. Les préférences ont été restaurées à l'identique après fermeture.

Un clic UI Automation sur index a donné des coordonnées hors fenêtre ; une
nouvelle observation et les coordonnées de la capture ont permis de poursuivre.
Ce refus d'injection n'est pas une preuve de défaut des raccourcis. Les défauts
retenus sont ceux reproduits par egui, indépendamment de Computer Use.

## Fichiers et limites

- `editor/src/app.rs` : extraction et correction du dispatch fichier.
- `editor/src/app_shortcut_tests.rs` : cinq tests.
- Ce rapport : audit et preuve.

Les événements sans Ctrl ne sont pas artificiellement transformés en raccourcis.
Les champs reçoivent toujours la frappe normale. Ce milestone ne refond pas le
keymap général, les raccourcis de lecture ni les conventions de clavier macOS.
