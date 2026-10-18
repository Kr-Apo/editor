# Dernier essai manuel du nouveau ZIP Windows

M007.2 n’a effectué **aucun nouveau test GUI**. Cet essai est la dernière étape
avant revue de sortie ; ne pas publier automatiquement après ces gestes.

1. Vérifier le SHA-256 du ZIP fourni, extraire entièrement dans un dossier neuf
   accessible en écriture, avec espaces/accents. Ne pas extraire sur M006.1.
2. Sans Rust/Node, lancer `Ankhimate.exe`, ouvrir `samples/walker.ankh`, passer
   en Animate et lire `walk`, puis `leg_ik`. Noter Windows/GPU/pilote.
3. Save As dans un dossier de travail. Modifier, tester New/Open/Close avec
   **Cancel**, puis Save ; rouvrir et vérifier le projet et ses assets.
4. Importer `samples/tagged_rig.psd` via File, lire la preview et le résumé,
   Undo/Redo, Save As, fermer/rouvrir. Essayer aussi une **copie** d’un PSD réel.
5. Régler autosave à 30 s, modifier puis attendre sans interaction ; relancer
   après interruption d’une copie de test et vérifier Recover/Ignore. Ne pas
   supprimer les sauvegardes. Signaler tout message d’ouverture inattendu.
6. Exporter dans trois dossiers neufs : données/atlas, PNG sequence et
   spritesheet. Pour `walk` 1 s à 60 FPS : 60 frames. Vérifier le préflight,
   l’annulation et le refus/consentement des collisions ; lire les limites.
7. Ouvrir `showcase/projet/Crapaud-Pharaon.ankh`, lire `frappe_sol` 3 s, sans
   écraser l’exemple. Les assets générés sont fournis, pas la référence privée.
8. **Option Phaser séparée** : installer/utiliser Node **22+**, puis dans `game`,
   `node server.mjs` et ouvrir `http://127.0.0.1:4173`. Déplacement, changement
   de clip et compteur d’événements ; aucun npm install/CDN requis. Node n’est
   pas nécessaire à l’éditeur. Garder le snapshot livré.

Retour attendu : hash du ZIP, version Windows, GPU/pilote, résultat des étapes,
messages exacts et réserves. Utiliser [alpha-bugs.md](alpha-bugs.md). Si une perte
de données, un crash ou une réouverture corrompue apparaît, arrêter et conserver
le projet, ses assets et les backups ; la publication est bloquée.
