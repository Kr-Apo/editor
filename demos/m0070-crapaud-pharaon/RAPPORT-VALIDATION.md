# Le Crapaud Pharaon — validation locale du 16 octobre 2026

## Résultat et périmètre

Le projet éditable et un clip simple `frappe_sol` sont livrés pour revue. La
demande récente de l'utilisateur remplace les trois clips initiaux : pas de
création d'idle, salut ou marche dans cette livraison. Le sceptre est tenu dans
la pose initiale, levé, suspendu, puis ramené au sol avec un petit rebond. Une
réaction légère de la tête accompagne l'impact. Il n'y a pas de changement
dynamique de parentage ou de fermeture animée des doigts.

Le recalage utilisateur a été préservé. Comparaison du projet sauvegardé avant
et après animation : bones, transforms de setup et assets identiques. Seuls
changent les animations, les presets d'export, l'offset de l'attachment sceptre
(25, 15 au lieu de 71, 64), et la référence du slot du pagne retiré, passée à
`none` pour éviter un attachment manquant. La copie utilisateur originale est
conservée séparément, ainsi que les sauvegardes et autosaves existants.

## Authoring réellement effectué

Import du PSD à groupes simples dans l'exécutable M006.1 candidate2, parentages
par l'interface, recalage manuel de l'utilisateur, Save As vers un nouveau
projet, création et renommage du clip, clés dans la timeline via l'inspecteur,
sauvegarde et réouverture d'une copie avec ses assets. Aucun JSON auteur ou
runtime n'a été corrigé à la main. Les outils hors dépôt lisent le projet pour
mesurer les résultats ; ils n'écrivent pas de données d'animation.

| Élément | État et preuve |
|---|---|
| PSD multicouche | Présent, import réel ; variante simple : 16 bones, 18 images/slots à l'import |
| Rig final | 16 bones, 18 slots, 17 attachments visibles ; skin default, régions, blending Normal |
| Animation | Créée dans l'interface ; 3 s, 30 FPS, 3 pistes de rotation, 17 clés |
| Projet et assets | Chargement Rust sans diagnostic ; réouverture Windows de la copie et sélection du bras réussies |
| Lecture éditeur | Lecture en boucle observée, timeline et pose haute capturées après réouverture |
| Export natif | Vrai `skeleton.json` v2 + `atlas.png` depuis le panneau Export ; préflight sans avertissement de fidélité |
| Séquence PNG | 90 RGBA transparents 512 × 512, fin exclusive, cadrage commun |
| Spritesheet | 10 pages de 1548 × 1548, padding/extrude 2, 90 rectangles explicites |
| Phaser | Vrai export natif, snapshot M006.1 vérifié par hashes ; WebGL matériel, 17 meshes visibles |
| Vidéo | Capture de 6,5 s du canvas Phaser vivant ; aucun rendu préfabriqué substitué au squelette |
| Vidéo éditeur | Non produite : l'API Windows employée permet les captures, pas l'enregistrement vidéo |
| FFD / autre skin / clipping sur ce personnage | Non démontrés ; aucune nouvelle qualification annoncée |
| Physique / paths / blending avancé | Non utilisés et non qualifiés par cette scène |

Clés : bras/poignet aux frames 0, 30, 39, 46, 50 et 58, avec rotations opposées
pour conserver le sceptre vertical. Tête aux frames 0, 39, 46, 50 et 58.
L'impact intervient à 46/30 s, puis le repos est tenu jusqu'à 3 s.

## Mesures et régressions

- **1 103 tests Rust** : 0 échec (`cargo test --workspace --all-targets`).
- Formatage : positif. Clippy workspace/all-targets : positif ; multiensemble
  des warnings identique à M006.1, aucun warning produit ajouté.
- Build workspace : positif, trois warnings de dépréciation existants.
- WASM core/formats/document/render/export : positif.
- Build et typecheck TypeScript core/Phaser : positifs ; **34 tests** positifs.
- Référence Rust/export actuelle : les **128 cas de parité** enregistrés sont
  identiques ; les tests TypeScript les évaluent également.
- Régressions WebGL : **21** cas de base, **32** dark tint, **153** skins et
  **74** clipping positifs, sans warning ni erreur d'infrastructure sur les
  exécutions complètes retenues. Le clipping concave optionnel n'a pas été
  rejoué ; aucune nouvelle couverture n'est revendiquée pour lui.
- Premier essai skins : timeout navigateur de 30 s, après **129** cas positifs,
  au cas `mesh-multi-dark/default` suivant. Rejeu complet **153/153** positif.
  Premier résultat conservé ; cause exacte du timeout non déterminée, pas de
  divergence numérique constatée dans les cas terminés.
- Nouveau personnage : **12 poses WebGL** comparées aux worlds Rust, dont
  0/0,25/0,5/0,75/1/1,3 s et les frames d'impact/rebond/retour. Écart maximal
  sur les composantes world : **0,0000411**, sous la tolérance 0,01.
- `gl.readPixels` : pixels visibles présents sur les 12 poses, aucun code
  d'erreur GL ; 17 meshes. Images du vrai canvas conservées pour comparaison.
- CPU/GPU : écart RGBA prémultiplié moyen inférieur à **1/255** sur les frames
  effectivement comparables ; environ 1,1 % des pixels dépassent 32/255 sur au
  moins un canal. Différences de rasterisation/filtrage présentes, pas
  d'identité pixel à pixel revendiquée. Seuil exploratoire du banc : moyenne
  <8/255 sur toute l'image, complétée par l'inspection visuelle et la parité des bones.
- Reconstitution spritesheet → PNG : **90/90 identiques**, avec caméra commune,
  transparence et dimensions vérifiées. 59 images distinctes ; les autres sont
  les poses de repos tenues.
- Deuxième évaluation Rust : **90/90 PNG identiques** ; deuxième export natif :
  JSON et atlas identiques octet par octet à ceux publiés par l'interface.
  La spritesheet n'a pas fait l'objet d'une deuxième publication GUI.
- Phaser : lecture continue **180,026 s**, 60 boucles supplémentaires, aucune
  erreur JavaScript ni ressource manquante ; pause, curseur, replay et arrêt
  sans boucle testés. Le nombre observé de meshes est stable, sans mesure
  exhaustive d'allocation de buffers GPU.

Navigateur mesuré : Chrome 155.0.8059.84, ANGLE Direct3D11 / RTX 3090 ;
Node.js 24.18.0. Runtime `m0061-8888e0e`, core 0.1.1, adaptateur 0.1.0,
Phaser 3.90.0 et fflate 0.8.3. Aucun téléchargement CDN au lancement.

## Obstacles d'authoring et limites résiduelles

1. **Groupes PSD imbriqués** : 44 images/slots au lieu de 18, par répétition des
   descendants dans chaque ancêtre (`formats/src/psd.rs`, parcours des groupes
   avec `get_group_sub_layers`). La source imbriquée est conservée comme preuve
   privée ; le PSD de démonstration retenu utilise des groupes simples.
2. **Clic sur le nom d'un bone** : le clic droit physique confirmé par
   l'utilisateur n'ouvrait pas le menu. Dans `editor/src/ui/tree.rs`,
   l'interaction du nom recouvre celle de la ligne. Le menu et le parentage
   fonctionnent en cliquant le petit glyphe du bone. Aucun correctif produit.
3. **Retour exactement au setup par auto-key** : une rotation tapée à zéro ne
   crée pas de clé de retour, car `editor/src/edit_router.rs::bone_keys`
   compare au setup. Observation reproduite dans la timeline ; la frappe
   utilise 0,1° et la contre-rotation −0,1°. La tête termine aussi à 0,1° :
   petite différence de boucle, pas de prétention à un retour mathématique
   exactement nul. Aucun contournement par écriture du projet.
4. L'os `coiffe` reste vide : son slot est porté par `tete`. La main gauche
   reste sous root. Ce clip ne les anime pas indépendamment. Le rig ne doit
   pas être présenté comme une démonstration exhaustive de rigging avancé.
5. Pas de nouveau scénario FFD, skin ou clipping : la demande simplifiée a
   recentré cette livraison sur une frappe. Les tests historiques assurent la
   non-régression des fixtures existantes, pas leur authoring sur ce personnage.

La préparation des fichiers et la publication emploient les pipelines M001/
M004 existants. Aucun test destructif de rollback ou de coupure électrique
n'a été ajouté : ne pas présenter ce travail d'assets comme une nouvelle
preuve d'atomicité multi-fichier ou de durabilité.

## Livraison et provenance

Le projet est modifiable ; ses références d'assets sont relatives et les
`source_path` sont nulles. Pas de chemin personnel dans le projet ou les
exports natifs. La préparation graphique vient de pièces originales générées,
pas d'une animation externe ou d'une découpe d'un personnage aplati.

La référence utilisateur, la source PSD imbriquée de diagnostic, les backups,
autosaves et captures Windows affichant des chemins personnels restent privés
dans le dossier de travail ; ils sont exclus du checkpoint partageable. Les
mesures, la capture Phaser sans chemins personnels, les sources compatibles et
les licences des dépendances peuvent accompagner la revue locale. Les droits
de publication définitifs des assets restent à approuver par l'utilisateur.

Les branches de référence M006.1 sont conservées. Les éventuels changements
du checkpoint de démonstration ne portent que sur les assets, la page locale
et sa documentation ; aucune modification du moteur, runtime ou format.
Aucun push, merge distant, changement de version ni publication.

Les scripts de mesure et les logs bruts restent dans `work/m0070` hors dépôts.
Le retour utilisateur Windows 11 25H2 sur l'autre PC demeure une déclaration
d'essais import/sauvegarde/exports, et non une qualification exhaustive M006.2.
