# M005.4b1 — Native FFD outgoing curves

Validation du 15 octobre 2026. Branche `fix/m0054b1-ffd-curves`, base propre
`develop` / `367238cfa432fb597bbe3c40a64f1c75217a4312`. Aucun merge, amend,
push, changement de version ou publication. A02, A03 et les sujets P2 restent
hors périmètre.

## Cause et correction minimale

Rust échantillonne un segment avec l'interpolation de sa clé d'arrivée
(`core/src/animation.rs`, `ease(raw, b.interp)`). Le contrat du contexte et le
lecteur runtime attendent la courbe sur la clé de départ. `scalar_keys` faisait
déjà la conversion ; la branche `Timeline::Deform` copiait encore `k.interp`.

Le contexte FFD lit maintenant l'interpolation de `keys[i+1]`. La dernière clé
porte `linear`, neutre puisqu'aucun segment ne la suit. Rien d'autre n'est
modifié : temps, nombre/ordre des clés, offsets, sommets, indices, influences,
skinning, évaluateur Rust, template natif, préflight et publication.

Le format runtime reste **v2**, le contexte **1**, le preset **1**. La correction
respecte le contrat public existant. Les templates personnalisés qui auraient
compensé l'ancien décalage doivent retirer cette compensation ; aucun template
utilisateur n'est réécrit automatiquement. Les presets et tests communautaires
existants passent sans modification.

## Régressions avant correction

Six tests `export/tests/ffd_curves.rs` ajoutés avant le correctif : cinq rouges
(stepped, Bézier rigide, Bézier pondéré, sparse, clé unique) et le contrôle
linéaire vert. Les fixtures natives et oracles M005.4a sont régénérés hors des
dépôts sans retouche du JSON :

| Cas | Offset max avant | Sommet max avant | Offset max après |
| --- | --- | --- | --- |
| Stepped rigide | 15 | 11,51149 | 0 |
| Bézier rigide | 3,72432899 | 2,85817570 | 0,000005617 |
| Bézier pondéré | 2,97946358 | 2,35394994 | 0,000004875 |
| Linéaire rigide | 0 | 0,000009118 | 0 |
| Linéaire pondéré | 0 | 0,000007633 | 0 |

Aux temps 0 / 0,25 / 0,5 / 0,75, les trois défauts produisaient neuf poses GPU
rouges sur vingt. Les vingt deviennent vertes. Le sommet max final est inférieur
à **0,000011**, largement sous la tolérance de **0,01**. Les poids 0,25/0,75,
huit influences, quatre sommets et six indices restent identiques.

## Borne décimale : extension autorisée séparément

La vérification des bornes a reproduit un second problème : une clé à `0.8f32`
est écrite `0.800000011920929`. Rust convertit le playhead `0.8` en f32 et atteint
la clé, tandis que le sampler FFD TS utilisait directement le double et restait
sur le segment stepped précédent. Écart de **20**. Ce résultat a été signalé,
sans modifier le runtime dans le périmètre Rust initial.

L'utilisateur a ensuite autorisé explicitement un correctif TypeScript dans
un checkpoint distinct. Branche runtime `fix/m0054b1-ffd-time-boundaries`, base
`94ec1dfde999d554c9604586c6bdbb419e5c0b23`. Seul le temps d'échantillonnage FFD
**v2** est normalisé avec `Math.fround`, comme les pistes osseuses v2 existantes.
La sémantique v1 reste inchangée. Aucun changement de renderer/skin/clipping.

Trois tests runtime ajoutés : régression sur export natif réel, holds aux bornes
sparse, v1 implicite/explicite inchangé. Avant correction : 33/34 tests passent,
la nouvelle borne native échoue. Après : **34/34**. Treize temps limites du banc
Rust/TS, de -0,25 à 1,25, passent après correction, dont 0,8 demandé en double et
la même valeur f32 sérialisée. Une preuve GPU séparée couvre la borne exacte
sans remplacer le temps demandé par celui du JSON.

## Fidélité et stabilité des sorties

Phaser 3.90.0, Windows NT 10.0.26300.0, Brave/Chromium 155.0.8059.80,
ANGLE/D3D11 RTX 3090. Tests WebGL réels, `gl.readPixels`, pas de mock.
Comparaison CPU ramenée en PMA : aplats RGB ≤2/255, alpha ≤1/255, aucun pixel
d'aplat au-delà de 4/255, désaccord global aux arêtes ≤2 %. Observé sur les vingt
poses nominales : RGB PMA max **0,753/255**, alpha **0**, zéro aplat divergent.
Ces petits meshes/textures uniformes ne qualifient pas toutes les combinaisons
avancées, le filtrage ou les arêtes de grands projets.

Pour les cinq fixtures M005.4a, seules les propriétés `curve` FFD changent dans
le JSON natif. Les linéaires restent identiques comme données JSON. Les sorties
PNG et spritesheet de deux exécutions (avant/après) restent **identiques octet
par octet** aux sorties acceptées de M005.4a : **300 frames**, plus métadonnées
et pages. Tous les crops de spritesheets reproduisent les PNG.

Réexport réel de walker `walk` / `leg_ik` et du projet tagged_rig : **180 PNG**
identiques aux pixels M004.1, reconstruction des sheets identique, douze poses
CPU avec caméra commune identiques. Les fixtures originales et les références
acceptées sont intactes. Les 19 anciens fichiers et 59 fichiers dark tint
conservent leurs empreintes ; les **53 régressions WebGL** existantes passent.

## Compatibilité

- Les anciens exports v1 et v2 restent lisibles. Un véritable walker v1 a été
  produit avec le preset historique ; ses poses restent identiques dans les
  lecteurs de référence et actuel. V1 conserve son temps double, explicitement
  testé, y compris les anciens holds stepped aux bornes décimales.
- Les anciens v2 non linéaires mal sérialisés doivent être **réexportés** pour
  retrouver leurs courbes. Le nouveau lecteur ne déplace ni ne répare les
  courbes d'un ancien JSON. Les quatre poses des anciens exports concernés
  restent identiques à celles du lecteur antérieur.
- La normalisation temporelle FFD v2 est un changement intentionnel et limité
  aux seuils de précision f32 ; les données sérialisées ne sont pas arrondies
  pour masquer un écart du consommateur.
- Les 128 références Rust/TS osseuses et les quatre round trips JSON/échappement
  sont recalculés et inchangés.

## Gates et limites

- `cargo test --locked --workspace --all-targets` : **1 100 tests réussis**
  (1 094 précédents +6 nouveaux).
- `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets`,
  `cargo build --locked --workspace` : réussis.
- WASM : core, formats, document, export et render vérifiés sur
  `wasm32-unknown-unknown` : réussi.
- Tests de sécurité, préflight, publication, chemins Windows et rollback inclus
  dans le workspace : réussis. Aucune modification de leurs garanties.
- Runtime : build/typecheck et 34 tests réussis ; aucun warning TS.
- Clippy : les **25 occurrences de diagnostics** préexistantes sont conservées,
  sans nouvelle occurrence ; warnings de compilation existants inchangés.

Le dossier externe de livraison conserve les logs rouges/verts, exports réels,
captures, résultats numériques, diff complet et empreintes. La validation ne
prétend pas résoudre A02, A03, les linked deforms hérités CPU, la physique, les
paths, les modes de blending ou les skins alternatifs des exports rendus.
Pas de nouvelle qualification native du viewport egui/wgpu ; la preuve graphique
de ce correctif d'export est CPU/Phaser WebGL.
