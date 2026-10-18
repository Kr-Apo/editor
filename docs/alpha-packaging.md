# Préparer le candidat local M007.2

Cette procédure nécessite les outils de développement uniquement sur la machine
qui fabrique le candidat. L'utilisateur de l'éditeur n'en a pas besoin.
Aucun téléchargement automatique, push, npm publish ou release n'est déclenché.

## Construire

Utiliser le checkpoint M007.2 descendant de d644653 consigné dans VERSIONS.json,
ses lockfiles et Rust
1.95.0 x86_64-pc-windows-msvc. Faire les tests workspace, fmt, Clippy et WASM avant
la construction. Le rendu n'est pas remplacé pour le packaging.

```powershell
$env:CARGO_TARGET_DIR = '<dossier-neutre-sans-chemin-personnel>'
$env:RUSTFLAGS = '-C target-feature=+crt-static --remap-path-prefix=<editor>=editor --remap-path-prefix=<profil>=build-user'
$env:CARGO_PROFILE_RELEASE_STRIP = 'symbols'
cargo build --release -p ankhimate-editor --locked --offline
```

Un dossier de compilation neutre est nécessaire : les assertions C de QuickJS
peuvent incorporer leur chemin UTF-16, que le remapping Rust seul ne retire pas.
Ne pas patcher les octets de l'exécutable. Vérifier PE subsystem 2, imports DLL
système, absence de CRT dynamique ou de dépendance Rust/Node et de chemins de
profil ASCII/UTF-16 avant empaquetage. Les ressources GUI (fontes, icônes, logo,
shaders) sont incorporées ; les notices de fontes doivent rester livrées.

Dans le runtime : npm ci, npm run build, npm test, npm run typecheck. Dans
examples/phaser-game : npm ci, npm run check. Les archives m0061-8888e0e et leur
provenance sont versionnées ensemble. Exécuter ensuite :

```powershell
node scripts/prepare-alpha-game.mjs <nouveau-dossier-jeu>
```

Le script vérifie les empreintes et l'identité de chaque fichier dist installé
avant de copier le JavaScript public, Phaser 3.90.0 et fflate 0.8.3.
Ne pas remplacer ces archives par les anciens fichiers portant les mêmes versions.

## Assembler

Préparer les trois exports rendus de référence avec le renderer CPU existant
(walker walk/leg_ik et animation_1 du tagged_rig auteur), à 60 FPS, 512 × 512,
pages 2048, padding 2. Comparer les 180 frames aux références M004 et aux crops.
Collecter les notices de toutes les dépendances Cargo du binaire (hors dev),
fontes Lucide/egui, Rust et dépendances Phaser ; conserver leur provenance.
Les crates dont le paquet omet les licences sont complétées depuis leur commit
amont exact. Le manifeste des dépendances n'embarque pas les chemins de la machine.

```powershell
./tools/package-alpha.ps1 -Binary <exe> -PreparedGame <jeu> -Notices <licences> -RenderedExamples <exports> -RuntimeRepository <runtime> -Output <nouveau-candidat> -Candidate 'M007.2 local alpha candidate' -Showcase <projet-et-assets-curates> -BuildManifest <manifest-json>
```

Le script refuse tout dossier/ZIP existant, n'efface rien, copie explicitement
walker.assets, crée les versions et SHA-256 puis le ZIP. Un échec conserve le
nouveau dossier pour diagnostic, sans remplacer un candidat antérieur. Le ZIP
n'est pas revendiqué bit-à-bit reproductible (toolchain/horodatages), les données
et pixels sont déterministes. Le script assemble des entrées déjà validées ; il
ne remplace pas leurs tests ni une vérification de sécurité du contenu final.

Relire l'archive : pas de secrets, chemins personnels, node_modules, caches,
PDB, logs ou staging. Vérifier chaque SHA après extraction dans un nouveau dossier.
En M007.2, réutiliser les preuves WebGL/300 secondes déjà acceptées ; ne pas
annoncer de nouveau smoke GUI. Le dernier essai manuel suit alpha-final-check.md.
Enregistrer
séparément mesures de démarrage et PSD lourds : les petits exemples ne prouvent
pas les performances en production. Les conditions Windows propre sont dans
alpha-scope.md ; elles appartiennent à M006.2 après revue.
