# Ankhimate — Le Crapaud Pharaon

Une seule animation `frappe_sol`, selon la demande simplifiée du 16 octobre 2026.
Le sceptre est tenu dès la pose initiale, puis levé et frappé contre le sol.

1. Installer/utiliser Node.js 22 ou plus récent (testé : 24.18.0).
2. Double-cliquer `Lancer-demo.cmd`.
3. Ouvrir `http://127.0.0.1:4177` dans Chrome/Brave/Edge avec WebGL actif.
4. Utiliser **Rejouer la frappe**, **Pause**, **Boucle** et le curseur de temps.
5. Fermer la fenêtre du serveur pour arrêter la démonstration locale.

Alternative depuis ce dossier : `node server.mjs` puis `http://127.0.0.1:4173`.
Ne pas ouvrir directement `index.html` en `file://` : le chargement des modules
et du JSON demande HTTP. Aucune installation npm ni connexion Internet n'est
nécessaire au lancement.

À gauche : le vrai squelette runtime, rendu par Phaser WebGL. À droite : les
PNG du renderer CPU Ankhimate à la frame correspondante. Les fichiers dans
`assets/runtime` sont des exports natifs sans retouche. `runtime/` est la copie
exacte du snapshot M006.1 `m0061-8888e0e` ; hashes et versions dans `SNAPSHOT.json`.
Les licences des dépendances sont dans `licenses/`.

La scène principale ne démontre pas FFD, skins alternatifs, clipping, physique,
changement de clip ou événements. Un seul skin par défaut et le blending Normal
sont employés. La comparaison des images ne promet pas l'identité CPU/GPU : le
filtrage des textures et la rastérisation diffèrent.
