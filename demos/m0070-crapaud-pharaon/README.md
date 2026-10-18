# Ankhimate — Le Crapaud Pharaon

**Animation simple livrée pour revue locale, non publiée.**

Selon la demande du 16 octobre 2026, le modèle recalé par l'utilisateur est
conservé et le pagne retiré. Un seul clip **`frappe_sol`**, de 3 secondes à
30 FPS, remplace la demande initiale de trois animations. Le sceptre est tenu
dès le départ : levée, suspension, frappe rapide, petit rebond et repos.

## Ouvrir le projet

Dans Ankhimate M006.1 candidate2, ouvrir `projet/Crapaud-Pharaon.ankh`.
Garder le dossier voisin `Crapaud-Pharaon.assets` avec le fichier. Passer en
**Animate**, choisir `frappe_sol` et cliquer Lecture dans la timeline.
Le nom interne du projet reste `untitled`, comme dans le recalage utilisateur.

Les clés du bras, du poignet et de la tête ont été créées dans l'interface.
Aucune animation ou structure de projet n'a été fabriquée par modification de
JSON. La copie du recalage remise par l'utilisateur est conservée séparément.

## Regarder la démonstration Phaser

Suivre `demo/README.md` ou lancer `demo/Lancer-demo.cmd` avec Node.js 22+.
La page locale propose lecture, pause, boucle et déplacement dans le temps.
À gauche, le vrai squelette WebGL ; à droite, les PNG CPU correspondants.
`preuves/frappe-sol-phaser.webm` est une capture réelle du canvas Phaser.

## Contenu

- `sources/` : planche de pièces originales, PNG séparés, PSD multicouche et provenance.
- `projet/` : projet modifiable et assets.
- `exports/runtime/` : `skeleton.json` v2 et atlas natifs, sans retouche.
- `exports/png/` : 90 PNG RGBA 512 × 512 et `sequence.json`.
- `exports/spritesheet/` : dix pages PNG et JSON décrivant les 90 frames.
- `demo/` : démonstration autonome avec le snapshot runtime M006.1, sans CDN.
- `preuves/` : captures et mesures ; `RAPPORT-VALIDATION.md` décrit la portée exacte.

Les scénarios FFD, skins alternatifs, clipping et les trois clips initiaux ne
sont pas présentés sur ce personnage. Les régressions historiques du produit
ne constituent pas une nouvelle qualification de ces scénarios sur ce rig.

Les références utilisateur et captures comportant des chemins Windows sont
des pièces de travail privées, exclues du checkpoint partageable. Aucun droit
de publication sur l'image de référence fournie n'est présumé. Les obstacles
d'authoring sont consignés dans le rapport ; aucun correctif produit n'a été
engagé. Cette livraison ne clôt pas à elle seule la qualification M006.2.
