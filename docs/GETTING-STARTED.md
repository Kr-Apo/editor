# Premiers pas — candidat local T837

1. Extrayez complètement le ZIP dans un dossier ordinaire accessible en écriture. Double-cliquez sur **Ankhimate.exe**. Aucun script n’est requis. L’exécutable n’est pas signé ; ce candidat n’est pas une nouvelle publication publique.
2. À l’accueil ou par **File > Open…**, ouvrez `Examples/Jade-Sentinel/Sentinelle.ankh` dans le dossier extrait. Il n’existe pas de bouton spécial « Open Jade ». Gardez `Sentinelle.assets` au même endroit : cinq PNG y sont requis.
3. Examinez les bones et slots de la hiérarchie, puis choisissez **Animate** et un clip dans le manager/Timeline. `idle` dure 2 s ; `attack` dure 1,5 s avec impact à 0,75 s. Jouez et déplacez le playhead.
4. Faites **File > Save As…** (Ctrl+Shift+S) dans un nouveau dossier/nom avant toute modification. Conservez l’exemple de référence. Vérifiez le mode Setup/Animate avant d’éditer.
5. Effectuez une petite modification, Undo/Redo, puis **Save** (Ctrl+S). Rouvrez votre copie pour contrôler projet et assets. Un Save échoué ne doit pas vous faire poursuivre une fermeture ou un remplacement.
6. Dans le panneau **Export**, choisissez **Data / atlas** et le preset natif pour un jeu compatible, ou **Image sequence / Spritesheet** pour les pixels. Choisissez un dossier de sortie neuf et lisez le preflight. Les exports CPU utilisent le skin par défaut, blending Normal, sans physique. MP4/GIF nécessitent FFmpeg externe.

Un dialogue **Recover unsaved work?** peut proposer une ancienne copie conservée. Recover la charge sans écraser son primaire ; utilisez Save As. Ignore mémorise ce seul état sans supprimer les fichiers ; retrouvez-les via **File > Review Recovery Copies…**. Une copie nouvelle/modifiée est à nouveau éligible. Si le choix ne peut être mémorisé, l’erreur est affichée ; **Later (this session)** n’est pas durable. Ne supprimez pas vos sidecars pour essayer ce candidat.

Consultez [les fonctions](FEATURES.md), [la compatibilité](COMPATIBILITY.md) et [les limites](KNOWN-LIMITATIONS.md). Les essais physiques Windows du nouveau candidat restent à réaliser par le mainteneur.
