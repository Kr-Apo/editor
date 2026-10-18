# M004.2 — Spritesheets animées

Validation Windows du 15 octobre 2026. Base propre :
`01980e1c5a60a804cdf3f53b37d6be924986a111`, branche de travail créée depuis cette
base : `feat/m0042-spritesheets`. `develop` reste à
`25f3a301920a4139db16a7820c42bf5d7c15c822`. Le runtime reste propre à
`6d8f3454aacd6b69fbc3c242b54ba98e19c23f79`, sur `fix/m0032-phaser-alpha`.
Ni runtime, ni upstream, ni formats runtime v1/v2 modifiés. Aucun push/merge/release.

## Audit, choix et implémentation

M004.1 fournissait déjà `Sequence::prepare`, la caméra commune, le rendu CPU,
l'échantillonnage exclusif, le worker et le publisher généré avec staging et
rollback. Il manquait l'assemblage par pages et son contrat de métadonnées.

- `Sequence::render_rgba` expose la même frame avant son encodage. `render` encode
  ce buffer avec les mêmes paramètres qu'avant. Aucun second renderer.
- `render::spritesheet::Layout` valide la grille et produit pages/rectangles en
  ordre déterministe. Ses données restent immuables après validation. Le panneau
  utilise une prévalidation du nombre de pages sans allocation des rectangles.
- Une page RGBA à la fois, une seule frame temporaire, puis encodage PNG. Les
  pixels sont copiés sans composition ; le padding prolonge exactement leurs
  bords RGBA. Cellules inutilisées transparentes, sans trim ni rotation.
- `editor::spritesheet_export` produit `spritesheet.json` version 1 et appelle
  le publisher M004.1 inchangé. Toute collision est refusée, sans remplacement.
- Le panneau partage le worker M004.1, ses snapshots et son comportement
  New/Open/Close. Les paramètres choisis au lancement sont capturés ; changer
  d'affichage ne change pas le job en cours.

Le choix de grille, le pivot, les exclusions et la limite mémoire ont été
annoncés avant l'implémentation. Les tests des nouvelles API ont été écrits avant
elles et ont initialement échoué à la compilation, faute de ces API. Aucun
défaut M001/M003 n'a été rouvert ni corrigé dans cette mission.

## Fichiers et dépendances

| Fichiers | Modification |
|---|---|
| `render/src/spritesheet.rs` | Grille multi-page, prévalidation, padding, assemblage et cinq tests |
| `render/src/sequence.rs` | Accès RGBA et dimensions, réutilisation inchangée du paint CPU |
| `render/src/lib.rs` | Déclaration du module |
| `editor/src/spritesheet_export.rs` | Orchestration, JSON et deux tests sur les trois clips réels |
| `editor/src/ui/png_sequence.rs` | Paramètres supplémentaires, worker partagé, progression/annulation et test de capture des paramètres |
| `editor/src/ui/export.rs`, `editor/src/main.rs` | Branchement de l'option Spritesheet |
| `examples/spritesheet-phaser/` | Banc indépendant, loader public Phaser, exports réels, références PNG, hashes, tests et lockfile |
| `docs/spritesheets.md`, ce rapport | Contrat, limites et validations |

Aucune dépendance Rust ajoutée. L'exemple isolé fixe Phaser 3.90.0 et Playwright
1.58.2, uniquement pour sa lecture et ses tests ; aucune dépendance Ankhimate
runtime. Son installation a réussi depuis le cache npm, hors du workspace Rust.
Les exports et les références sont protégés par hashes et `.gitattributes`
contre les conversions de fins de ligne des checkouts Windows.

## Métadonnées et mémoire

Le contrat complet est dans [spritesheets.md](../spritesheets.md) : clip, FPS,
durée en secondes, looping, nombre de frames, pages/dimensions, index/page/rect
par frame, dimensions d'origine, durée en millisecondes, padding/extrusion,
absence de trim/rotation, caméra et pivot communs. Le pivot normalisé [0,5;0,5]
correspond au centre de la caméra en coordonnées monde ; la projection de
l'origine monde dans chaque frame est également fournie. JSON sérialisé par
serde_json. La lecture suit les indices explicites, même si les tableaux sont
réordonnés. Ce contrat n'est pas le JSON atlas natif de Phaser.

Page maximale : 4096×4096. La borne des buffers RGBA explicitement détenus est
`4 × (plus grande page + une frame)` : **128 Mio au maximum**, et **10633792
octets, soit 10,14 Mio**, sur les fixtures 512×512 / page max 2048 / padding 2.
Ce n'est pas une borne de mémoire totale : textures décodées, snapshot,
métadonnées/transaction, PNG encodé et scratch du codec sont supplémentaires.
Les rectangles et métadonnées sont O(nombre de frames), pas les pixels RGBA.
Le test de borne maximale ne prétend pas mesurer le RSS total du processus.

## Gates automatisés

Rust/Cargo 1.99.0, Node 24.18.0. Exécutions natives hors sandbox pour les gates
filesystem Windows. Journaux détaillés dans le dossier externe `work/m0042`.

| Contrôle | Résultat |
|---|---|
| `cargo test --workspace --all-targets` | **1047 réussis**, 0 échec/ignoré ; baseline M004.1 1039 + 8 |
| `cargo fmt --all --check` | Réussi |
| `cargo clippy --workspace --all-targets` | Réussi ; 25 diagnostics préexistants, aucun ajout |
| `cargo build --workspace` | Réussi ; trois dépréciations préexistantes de l'éditeur |
| `cargo check -p ankhimate-core -p ankhimate-render --target wasm32-unknown-unknown` | Réussi |
| Tests TypeScript runtime en lecture seule | **26 réussis**, aucun fichier runtime écrit |
| Typecheck core et Phaser, `--noEmit` | Réussi |
| Référence des axes Rust/TS M001a | 128 cas identiques |
| Référence JSON Rust/TS M001b | 4 cas identiques |
| Exemple : `npm run check` | Trois tests JS réussis, syntaxe et hashes validés |
| Exemple : `npm test` | Lecture WebGL réelle et 12 comparaisons réussies |
| Installation/exécution indépendante | `npm ci --offline`, puis tests complets réussis dans une copie hors des dépôts, chemin avec espaces |

Les huit tests ajoutés couvrent grille et pages, paramètres invalides/overflow,
borne RGBA, comptage long sans disposition complète, extraction des frames,
annulation entre frames, padding semi-transparent et cellules inutilisées,
trois clips réels, déterminisme, métadonnées/Unicode, pivot/caméra, échec
d'écriture, collisions, préservation des anciennes sorties, refus avant création
du dossier et capture des paramètres du worker.

Les garanties M001 et les régressions M003/M004.1 restent dans la suite complète :
atlas, presets, JSON, axes/courbes, staged publication, rollback et conservation
des backups, erreurs et verrous Windows réels, cycle de sauvegarde, dirty state,
historique, chargements concurrents, autosave, récupération et fermeture.
Le publisher n'a pas été modifié ; ses injections de panne pendant publication
et rollback s'appliquent à la même API générée utilisée par les spritesheets.

## Exports réels et pixels M004.1

Un banc externe appelle les mêmes API que le panneau, sans patch de données.
Paramètres : 60 FPS, 512×512 par frame, page max 2048, padding/extrusion 2.

| Source / clip | Frames/pages | Deux exports, temps debug total |
|---|---|---:|
| `samples/walker.ankh` / `walk` | 60 / 7 | 14,776 s |
| `samples/walker.ankh` / `leg_ik` | 60 / 7 | 15,929 s |
| Projet M002 issu de `tagged_rig.psd` / `animation_1` | 60 / 7 | 15,224 s |

Pour chaque clip, deux exports donnent les mêmes bytes PNG et JSON. Les 180
frames extraites par leurs rectangles sont **strictement identiques en RGBA** aux
PNG M004.1 acceptés. Chaque pixel du padding est comparé au pixel de bord attendu,
y compris alpha ; présence de pixels visibles et complètement transparents
confirmée. Le compteur de frames et la caméra sont ceux de la séquence commune.

## Phaser réel

Exemple indépendant sans runtime squelettique. Les pages sont chargées comme
textures normales ; les rectangles sont enregistrés par `texture.add`, puis
`anims.create` reçoit les références explicites aux pages/frames et le FPS.
La position et le pivot restent constants lors des changements de page.
Les trois clips bouclent conformément à leur métadonnée. Pas de blend entre clips.

WebGL sur Chromium **154.0.8037.0**, ANGLE / Direct3D11, NVIDIA RTX 3090. Phaser
3.90.0 et Playwright 1.58.2. Le navigateur existant a été explicitement sélectionné
par `ANKHIMATE_CHROMIUM` ; le téléchargement Chromium par défaut de Playwright
n'est pas le navigateur de cette preuve. Versions et backend enregistrés dans
les résultats. Aucun navigateur supplémentaire installé.

Pour chacun des trois clips : trois boucles observées, 60 indices et 180
changements de frame, animation de 1000 ms à 60 FPS, position/pivot vérifiés.
Les intervalles réels entre répétitions sont proches de 1000 ms ; le test autorise
±200 ms pour le scheduling. Les valeurs exactes sont dans `webgl-results.json`.
Les 12 poses (indices 0/15/30/45) rendues depuis les pages sont comparées aux PNG
individuels chargés dans le même WebGL : **écart RGBA maximal 0**, zéro pixel hors
tolérance de 2/255. Zéro erreur JS, warning ou ressource manquante. Captures remises.

## Windows : panneau Export

Computer Use, binaire debug natif, copies externes avec leurs assets ; originaux
inchangés et préférences restaurées byte pour byte.

- Sélection Spritesheet, `walk`, 60 FPS, 512×512, page 2048, padding 2 :
  **60 frames, sept pages, un JSON**. Les huit fichiers sont identiques au banc.
- Page 512 pour une frame 512 + padding 2 : erreur explicite et export désactivé,
  avant écriture.
- Réexport vers la destination existante : refus sans écrasement, huit hashes
  inchangés, aucun staging résiduel.
- Annulation réelle d'un export à 240 FPS (240 frames/27 pages) pendant la
  préparation de `sheet_000007.png` : message « export cancelled; rollback
  completed, previous output files preserved ». Seul le fichier antérieur reste.
  Le premier essai à 60 FPS avait terminé avant le clic et a été conservé comme
  export complet ; il n'est pas présenté comme une annulation réussie.
- Dernier binaire avec prévalidation constante : panneau rouvert et comptage
  30 FPS / 30 frames / quatre pages confirmé, puis fermeture propre.

Les erreurs OS/publication sont couvertes par tests automatiques et injections
existantes ; aucune panne matérielle ni coupure électrique graphique n'est simulée.

## Sûreté, compatibilité et limites

Staging/flush de tout l'ensemble avant publication ; refus de toute destination
existante ; rollback des erreurs ordinaires ; préservation des fichiers tiers
et des backups si restauration impossible ; échec explicite si rollback/cleanup
incomplet. L'export n'est pas déclaré réussi dans ces cas. Les arrêts brutaux
peuvent laisser verrou, staging ou sorties partielles, et les lecteurs concurrents
peuvent voir une publication intermédiaire. **Pas d'atomicité multi-fichier ni de
garantie de durabilité des renames**. Pas de journal/récupération automatique
d'export. Voir [export-safety.md](../export-safety.md).

Exclusions conservées : physique, skins autres que le défaut, blending autre
que Normal. Aucun nouveau contrat runtime v1/v2. Pas de trim, rotation, packing
irrégulier, mipmaps certifiés, événements transformés en logique de jeu, vidéo ou
autre renderer. Le padding zéro désactive la garde ; des mipmaps/downscales
arbitraires peuvent dépasser la garde choisie. Pas de certification sur projets
100000 frames, teintes sombres ou déformations/clipping complexes.
Annulation coopérative ; une frame, un encodage de page ou une opération OS
courante doit finir avant sa prise en compte. Pas de délai maximal garanti.

Les pages/JSON réels, le banc portable et ses captures/résultats sont livrés avec
le rapport externe et le diff complet. Commit local distinct après validation,
sans amend. M005/M006/IA non commencés ; attendre la revue finale.
