# Compatibilité — candidat local T837

Ce candidat n’est ni RC2 ni la publication v0.2.0-alpha.1. Son numéro interne n’a pas changé ; identifiez le fichier par son checksum et le checkpoint de livraison.

| Élément | Contrat |
|---|---|
| Projets existants | Le lecteur actuel accepte les profils Ankh v1 authentiques testés (binaire, JSON compact/lisible), avec assets séparés. Ce n’est pas une promesse de migration de tout prototype historique. |
| Fichiers plus récents/inconnus | Versions, flags et membres non représentables sont refusés ; ne modifiez pas une étiquette de version à la main. Gardez l’original. |
| PathPosition | Extension additive de variante fermée, malgré auteur/enveloppe toujours 1. Un ancien lecteur rejette ces nouveaux projets ; exports natifs/script non représentables refusés. Aucun support runtime PixiJS/Phaser revendiqué. |
| Ordre des clips | Sauvegardé dans l’ordre du tableau d’animations. Un ancien éditeur peut le perdre après réenregistrement. Les identités/keyframes ne sont pas renommées. |
| Runtime v1/v2 | Aucune modification T837. Les lecteurs externes et leurs sous-ensembles ont leur contrat propre ; l’exporteur natif produit v2, pas une animation baked de toute contrainte. |
| PixiJS / Phaser | Runtimes externes, profils qualifiés bornés. FFD/skins/dark tint/clipping/blending dépendent du cas/adaptateur ; contraintes avancées, physique et linked meshes ne sont pas garantis. Pas de nouvelle qualification WebGL dans T837. |
| Exports rendus | CPU Rust existant, skin par défaut, blending Normal, physique refusée. Le JSON spritesheet est Ankhimate, pas Phaser natif. MP4/GIF : FFmpeg compatible installé séparément. |
| Recovery | Décisions Ignore dans un registre séparé, jamais dans `.ankh`. Une ancienne version ignore ce registre et peut reproposer une copie ; aucun format de projet n’est modifié. |

Windows x64 natif est la cible construite. L’EXE est non signé. Pas de certification générale mobile, WebGPU, WebGL1, macOS/Linux, pilotes ou toutes cartes graphiques. Conservez des copies avant de tester une version locale. Le code correspondant au candidat est conservé dans le dépôt et identifié dans le README de livraison ; T837 n’est pas publié.
