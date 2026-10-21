# Ankhimate — candidat Windows local M007.2

Candidat de revue, non publié et non signé. Extraire tout le ZIP dans un dossier
accessible en écriture. Conserver la structure, notamment samples/walker.assets.

## Démarrer

1. Lancer Ankhimate.exe. L'éditeur ne nécessite ni Rust, ni Cargo, ni Node.
   Un GPU/pilote compatible wgpu est nécessaire ; ce nouveau ZIP reste à essayer manuellement sur le PC cible.
   Ne pas contourner un avertissement de sécurité Windows : soumettre le candidat
   à la revue prévue avant toute distribution.
2. Ouvrir samples/walker.ankh (ou le bouton Sample walker). Ses dix images sont
   dans walker.assets. Passer en Animate, choisir walk ou leg_ik, puis Play.
3. Pour travailler, faire Save As dans votre propre dossier. Garder le fichier
   .ankh et son dossier .assets ensemble. Ctrl+S sauvegarde ; Ctrl+Shift+S ouvre
   Save As. Save/Discard/Cancel protège New/Open/Close ; Cancel conserve le projet.
4. Pour tester un PSD : Setup > File > Import > Photoshop PSD, choisir
   samples/tagged_rig.psd. La preview propose Scale, Skip hidden layers,
   Replace/Merge. Import affiche un compte rendu. Undo/Redo restaure l'import
   sans relire le PSD. Ce petit PSD géométrique sert de test, pas de rig complexe.

## Exporter

Afficher le panneau Export depuis View. Utiliser une destination neuve dédiée,
à l'extérieur du dossier extrait. Le préflight présente erreurs, fidélité et
fichiers avant toute publication. Lire les avertissements ; autoriser explicitement
une perte connue ou un remplacement seulement si c'est votre intention.
Cancel n'écrit rien. Un préflight n'est pas une réservation du dossier.

- Données : preset intégré Ankhimate runtime actuel, puis skeleton.json et toutes
  les pages PNG de l'atlas. Le format reste v2. Ne pas retoucher le JSON à la main.
- PNG sequence : walk, 60 FPS, 512 × 512, dossier neuf. Une seconde produit 60
  images RGBA et sequence.json ; cadrage commun, fin exclusive.
- Spritesheet : mêmes paramètres, page maximale 2048, padding 2. Les pages et
  spritesheet.json conservent chaque index/rectangle/pivot. Plusieurs pages sont
  normales. Les pixels reconstruits correspondent à la séquence fournie.

Le dossier exports contient les vrais exemples walker-walk, walker-leg_ik et
le clip animation_1 du projet dérivé de tagged_rig. Chaque cas contient sequence
et sheet. Le PSD seul ne contient pas encore le clip animé du jeu.

## Jeu Phaser

Node 22+ est requis uniquement pour le petit serveur local (24.18.0 testé ; Node
n'est pas inclus). Dans un terminal ouvert dans game : `node server.mjs`.
Ouvrir http://127.0.0.1:4173 dans un navigateur WebGL ; arrêter avec Ctrl+C.
Aucun npm global, npm install, build TypeScript ou CDN n'est requis.
Ne pas ouvrir index.html avec file://. Aucun double-clic n'est promis.
Flèches/A/D déplacent le personnage ; Espace ou les boutons changent walk/leg_ik.
Le compteur montre les vrais événements footstep. Le second personnage vient du PSD.
Le guide complet est docs/psd-to-phaser.md ; game/SNAPSHOT.json identifie les
bibliothèques et assets réellement chargés.

## Limites et sécurité

Lire docs/alpha-scope.md avant d'utiliser une fonction avancée. Normal et skin
par défaut sont le périmètre des exports rendus ; physique et autres blend modes
sont refusés. La physique native est incompatible avec le consommateur TS actuel.
Les anciens v2 à FFD non linéaire erronée doivent être réexportés. Les autres
exclusions y sont explicites. Autosave reste distinct du fichier principal.
Les écritures préparées et le rollback protègent les échecs ordinaires, sans
promesse de transaction multi-fichier atomique ou de durabilité après panne.

Versions exactes : VERSIONS.json ; empreintes : SHA256SUMS.txt (hors lui-même).
Le SHA du ZIP est fourni à côté de l'archive. Licences et notices : licenses,
et game/licenses pour Phaser et ses dépendances. Ce candidat contient les exemples du projet, la fixture PSD et le projet Crapaud
Pharaon avec ses assets générés, autorisés pour redistribution par le mainteneur.
La référence privée, les sauvegardes et les preuves de travail sont exclues.
Le retour M006.2 ne remplace pas le dernier essai manuel de ce nouveau ZIP.

## Créer une animation et récupérer son travail

En Setup, créer les bones et rattacher les images ; utiliser les groupes du PSD
pour conserver une hiérarchie. En Animate, créer/sélectionner un clip, choisir sa
durée, placer le playhead, activer Auto-key et modifier la pose. Un retour exact
au setup d’un canal déjà animé crée sa clé ; vérifier le début et la fin de boucle.
Pour une déformation, convertir la région en mesh en Setup puis déplacer les
sommets en Animate. Le drag est corrigé ; le dopesheet FFD reste limité.

Dans Skins, dupliquer default puis sélectionner le skin. L’inspecteur **Skin
image** affecte une image existante à sa région sans remplacer les pixels
partagés ; **Import image to library…** ajoute un asset sans créer un slot.
Les meshes et séquences sont refusés par ce sélecteur.

Settings règle l’intervalle autosave (0 désactive). Une copie séparée est écrite
après modification, même au repos. Sans chemin, elle est conservée dans le
dossier temporaire. Au redémarrage, Recover ouvre la copie sans écraser le
projet ; Ignore permet de la décliner sans supprimer la sauvegarde. Vérifier
le résultat puis Save As. Une copie autosave peut rester après Save : ne pas
la supprimer sans vérification. Une coupure brutale ne garantit pas la durabilité.

Le projet Crapaud est dans `showcase/projet` : garder `.ankh` et `.assets`
ensemble, passer en Animate et lire `frappe_sol` (3 s). Il conserve le recalage
et le pagne retiré ; aucun enrichissement FFD/skin/clipping n’y a été intégré.

Rapporter un problème : [alpha-bugs.md](alpha-bugs.md). Dernier essai
manuel : [alpha-final-check.md](alpha-final-check.md).
