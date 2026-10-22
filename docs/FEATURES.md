# Ankhimate : les fonctionnalités disponibles

État local **T837, 22 octobre 2026 — candidat de développement**, non publié. Ankhimate — Kr-Apo Community Fork (Unofficial) est un éditeur Windows natif d’animation squelettique 2D, issu d’Ankhimate/editor. Les licences et crédits amont restent applicables. L’interface conserve ses noms anglais.

Un personnage associe images, bones, slots et attachments ; les clips animent ces éléments dans un projet `.ankh`. Jade Sentinel est le seul exemple utilisateur fourni. Ce candidat diffère de l’Alpha publique v0.2.0-alpha.1 et de l’archive RC2 : un numéro interne identique ne signifie pas que les exécutables ont les mêmes capacités.

## Importer et gérer les images

- Importez des images PNG, JPEG ou WebP dans la bibliothèque **Assets**, puis associez-les aux attachments. Les pixels source restent des assets séparés du projet.
- **File > Import** ouvre les importeurs enregistrés. Un PSD dispose d’une preview commune aux entrées File, Assets et glisser-déposer d’un PSD. Choisissez les calques, **Scale**, **Skip hidden layers**, remplacement ou fusion avant validation. Annuler ne remplace pas le document.
- Les groupes PSD deviennent des bones ; les calques images deviennent slots/régions. Les groupes imbriqués sont parcourus sans dupliquer leurs descendants. Les tags `$pivot`, `$ik ` et `@skin:` ont un périmètre précis ; ce n’est pas une promesse de reproduction de tous les effets Photoshop.
- La mise à jour **Update existing PSD images (keep rig)** conserve le rig et les animations dans son périmètre compatible. Une modification de chemin de calque demande une correspondance explicite. Les cas ambigus, remplacements de topologie et migrations FFD non représentables sont refusés, pas devinés.
- Utilisez les fonctions de relink/reload pour des images manquantes et examinez les diagnostics. Une sauvegarde peut refuser de publier un projet dont les pixels nécessaires sont absents.
- Les atlases regroupent les textures pour l’export. Les formats étrangers dépendent des plugins effectivement installés ; tous ne sont pas intégrés à ce dossier Windows.

## Construire le rig en Setup

Créez et nommez les **bones**, modifiez leur hiérarchie et leurs transforms, puis associez les **slots** et leurs **attachments**. Les régions ont position, taille, pivot et image. Les meshes, paths, bounding boxes et attachments de clipping ont leurs propres outils : un mesh n’est pas une région.

L’inspecteur permet une édition numérique des transforms et de la géométrie. Pour des sommets sélectionnés, **Set/Add**, **Apply coordinates** et **Apply UV coordinates** valident le brouillon avant une commande Undo. Les UV sont dans la texture source, U à droite, V vers le bas ; les positions Setup sont locales au bone, Y vers le haut. Les valeurs non finies et les UV hors de la plage acceptée sont refusés. Les expressions telles que `120/2` ne sont pas un calculateur pris en charge.

Les **skins** nommés permettent des variantes d’attachments et d’images, avec fallback vers le skin par défaut. L’affectation d’une image est indépendante par skin dans les cas représentables. La composition de skins dans l’éditeur n’implique pas une composition arbitraire identique dans un moteur de jeu.

Contraintes disponibles côté Rust : IK (une cible, chaînes de 1, 2 ou plusieurs bones), Transform et Path. Leurs paramètres et l’ordre des contraintes comptent. Les paths sont des polylignes ; la physique possède un état de simulation. Ces outils ne sont pas tous utilisables par les runtimes distribués : vérifiez la cible avant de construire un rig qui en dépend.

Le clipping utilise un attachment masque, un end slot et l’ordre de dessin. Ce n’est pas une simple superposition d’images. Sa présence dans l’éditeur n’est pas une certification de tous les masques, overlaps ou modes de rendu possibles.

## Éditer les meshes et les poids

Créez un mesh à partir d’une région compatible, éditez ses sommets, sa triangulation et ses UV, puis utilisez les outils de poids pour le déformer par plusieurs bones. Le panneau **Weights** expose Add/Subtract/Replace/Smooth, la saisie directe, le rayon, le feather et les verrous de session. Les influences et inverse binds doivent rester cohérents avec la bind pose.

Le retracing d’un mesh pondéré reste une opération protégée : les meshes owned compatibles, sans FFD ni partage ambigu, peuvent être préparés et validés avant Apply. Les linked meshes, propriétaires partagés, contraintes de topologie ou références FFD ambiguës peuvent provoquer un refus. Il n’existe pas de migration universelle de poids ou de FFD vers n’importe quelle nouvelle topologie.

En **Animate**, déplacer les sommets crée/édite une déformation FFD selon le routage natif ; un drag complet forme une commande Undo et utilise une référence stable. Les offsets existants, transforms et influences sont pris en compte. Conservez des clés de repos avant/après un impact : une unique clé tenue n’est pas une animation de retour automatique.

## Animer

- Créez, dupliquez, renommez et supprimez les clips. Le manager propose **Move Up / Move Down** : l’ordre auteur est sauvegardé, sans renommer les clips. L’ancienne durée affichée en frames et l’entrée automatique en Animate depuis la sélection du manager restent incomplètes.
- **Setup** modifie la structure/pose de référence ; **Animate** travaille sur le clip actif. L’auto-key peut enregistrer un retour exact au setup pour un canal déjà animé, sans créer des clés sur tous les autres canaux. Auto-key désactivé conserve les prévisualisations et commandes manuelles existantes.
- La Timeline/Dopesheet affiche les pistes de bones et les pistes éditables de slots, contraintes, événements, draw order et FFD selon leur type. Toutes n’ont pas les mêmes menus ni capacités de sélection multiple.
- Le **Graph Editor** édite les courbes scalaires Linear, Stepped et Bézier, y compris les overshoots. Les tangentes **Broken / Aligned / Mirrored**, la sélection rectangulaire et **Normalize View Only** facilitent le travail. Normalize change l’affichage, pas les valeurs enregistrées. La sélection/édition normalisée universelle et un Graph scalaire pour les vecteurs FFD ne sont pas fournis.
- Le retiming numérique, le changement de FPS avec Undo/Redo et le décalage des clés des bones sélectionnés sont disponibles dans leur périmètre validé. Un décalage sélectionné n’est pas un retargeting de toutes les contraintes ni une mise à l’échelle universelle du temps.
- Le presse-papiers de clés scalaires conserve temps relatifs et courbes, avec destinations explicites. **Paste Keys Mirrored (Offsets)** reflète les offsets des pistes de bones : X, rotation et shear changent de signe, Y et scale sont conservés. Il ne garantit pas une pose absolue symétrique pour des setups asymétriques et ne devine pas les suffixes gauche/droite.
- Les blocs de poses multibones et les blocs de clés compatibles passent par une transaction native unique. Les destinations ambiguës, verrouillées ou incompatibles sont refusées avant écriture.
- Les clés **deform** ont un menu de temps, suppression et interpolation. Leur presse-papiers dédié accepte un bloc d’une même piste/mesh owned dans le même document, avec propriétaire et topologie vérifiés. Il ne copie pas librement FFD entre projets, skins ambigus ou linked meshes.
- **Onion Skinning** montre des poses voisines avec réglages de teinte/opacité et sélection ; ce n’est pas une simulation physique complète des fantômes.
- **Path Position** anime la fraction de position d’une contrainte Path existante : **Key Position**, preview puis clé manuelle si auto-key est off, et ligne dédiée dans le Dopesheet. Path Spacing animé et export runtime de PathPosition ne sont pas disponibles.
- Les événements et changements d’attachment/draw order ont leur sens propre. Ne supposez pas que tout le presse-papiers, Graph ou miroir s’applique à eux. Les historiques Undo/Redo concernent le document ; la caméra/sélection restent des états de session.

## Sauvegarder et récupérer

**Save** écrit le chemin actif ; **Save As…** ne le change qu’après succès. **New**, **Open…**, la fermeture et l’accès aux récupérations depuis un document modifié respectent **Save / Discard / Cancel**. Un Save échoué n’autorise pas à perdre le document courant. Travaillez sur une copie et gardez le dossier `.assets` avec le projet.

L’autosave utilise l’intervalle de Settings (zéro désactive), une horloge monotone et un réveil différé au repos. Un document vierge inchangé ne produit pas de copie inutile. Les copies d’un projet nommé sont des sidecars séparés ; les documents sans chemin utilisent un espace temporaire. Une autosave n’est pas un Save manuel ni une sauvegarde indépendante durable.

Au démarrage, **Recover unsaved work?** propose les copies pertinentes. **Ignore** enregistre une décision pour cette identité et cet état précis, sans supprimer projet, copie ou assets. Une copie nouvelle ou modifiée peut revenir. **File > Review Recovery Copies…** permet de revoir aussi les copies ignorées du projet actif et les Untitled conservés. Pour un autre projet nommé, ouvrez son chemin d’abord. Plusieurs copies se succèdent après Ignore ; Recover charge la copie affichée.

Un échec d’enregistrement du choix est affiché : le dialogue reste disponible ; **Later (this session)** permet alors de différer sans prétendre mémoriser le choix. Les fichiers corrompus ne sont pas réparés automatiquement. Recover ne remplace le document qu’après chargement et revalidation ; il le marque modifié. Utilisez Save As et rouvrez le résultat. L’historique d’une ancienne session n’est pas sauvegardé dans `.ankh`.

Les copies plus anciennes qu’un primaire validé ne sont normalement pas proposées. Les horodatages restent une heuristique. Windows peut nettoyer ses fichiers temporaires ; les marqueurs Ignore peuvent être perdus/corrompus et provoquer une nouvelle proposition. Pas de promesse d’atomicité multi-fichier, de durabilité après panne électrique ni de récupération de chaque dernière frappe.

## Exporter

Le panneau **Export** distingue **Data / atlas**, **Image sequence** et **Spritesheet**. La prévalidation classe erreurs, avertissements de fidélité et informations, présente fichiers/collisions/remplacements/orphelins, puis demande les consentements nécessaires. Elle ne réserve pas le dossier : chemins, verrous, permissions, staging et rollback sont vérifiés à la publication. Annuler ne publie rien ; une erreur ordinaire préserve les sorties précédentes dans les garanties documentées.

**Data / atlas** utilise le preset natif v2 ou des templates personnalisés sur le contexte documenté. Les courbes scalaires X/Y restent indépendantes. Le fait qu’un champ soit exporté (contrainte, physique, linked mesh…) ne signifie pas qu’un runtime sait l’appliquer. Un projet contenant PathPosition est refusé pour les exports runtime non représentables.

**Image sequence** : PNG RGBA, JPEG avec fond RGB opaque explicite, WebP RGBA sans perte. Le renderer CPU et l’évaluation Rust existants sont réutilisés. Temps `i / fps`, fin exclusive, cadrage commun ; une seconde à 60 FPS donne 60 images. Trim, padding et bornes optionnels fournissent les informations de reconstruction. Les métadonnées Ankhimate conservent leur clé historique `png_sequence_version` même pour JPEG/WebP : utilisez le motif de nom de fichier, pas une extension supposée.

**Spritesheet** produit des pages PNG transparentes et un JSON Ankhimate, avec indices/rectangles/padding/pivot. Ce JSON n’est pas un atlas Phaser natif prêt à charger sans adaptateur. Les pixels sont reconstruisibles selon ses métadonnées. L’export batch prépare plusieurs clips dans des sous-dossiers indépendants ; son ordre historique par indice source n’est pas le nouvel ordre du manager.

**MP4/GIF** passent par un exécutable FFmpeg externe compatible, non fourni. Choisissez son chemin dans Settings ou utilisez celui du PATH ; les options d’encodage validées, l’annulation et les erreurs restent visibles. Pas de codec garanti sans cet outil. Le système n’installe pas FFmpeg automatiquement.

Les exports rendus restent limités au **skin par défaut**, au **blending Normal** et refusent la **physique**. Les contraintes évaluables par le CPU ne deviennent pas pour autant compatibles avec le runtime. Un refus protecteur n’est pas à contourner en modifiant le JSON.

## Utiliser un runtime de jeu

PixiJS et Phaser appartiennent au dépôt runtime séparé. Leurs profils qualifiés portent sur un sous-ensemble natif v2 : transforms, animations scalaires, meshes pondérés/FFD compatibles, skins, teinte sombre statique et clipping/blending selon les limites de chaque adaptateur. Les résultats historiques sont liés aux versions et scénarios exacts, pas à tous les rigs.

Ce dossier contient l’éditeur et Jade, **pas un runtime de jeu ou une démo navigateur**. Les contraintes avancées, physique, linked meshes et modes non qualifiés ne bénéficient d’aucune promesse universelle. **PathPosition n’est pas compatible avec ces runtimes à ce checkpoint.** Consultez COMPATIBILITY avant d’exporter ; un rendu CPU réussi ne prouve pas une parité GPU.

## Assistance IA facultative

**View > AI Assistant**, puis Settings, configure un des six profils réellement implémentés : **OpenAI, Anthropic, DeepSeek, Ollama, LM Studio, NInfer**. Les trois derniers visent des serveurs locaux loopback déjà installés. Aucun modèle n’est inclus, téléchargé ou lancé automatiquement. Les modèles doivent réellement accepter le protocole, les outils et, si utilisée, la vision du fournisseur choisi.

La conversation seule n’exige pas le partage du projet. Les lectures bornées de métadonnées, les captures visuelles et les éditions demandent leurs autorisations distinctes. Les images ne sont pas envoyées automatiquement. Les chemins source et fichiers complets ne sont pas le contexte par défaut. Les clés cloud restent en session ou dans Windows Credential Manager si l’utilisateur demande leur stockage.

L’agent peut lire des parties du rig, proposer des clés d’animation, de petites opérations sur bones, de l’assemblage de régions/slots et des corrections de poids de meshes owned existants. La lecture des meshes est paginée ; normalisation/lissage utilisent des opérations déterministes bornées. Pas de création libre de géométrie, remplacement de binds ou modification implicite d’UV/topologie/FFD. Les verrous, révisions et références sont revalidés.

Les propositions montrent les opérations avant approbation. **Approve native edits / Refuse changes** et les contrôles natifs restent l’autorité ; du texte du modèle ne vaut pas permission. Un lot accepté forme une transaction Undo/Redo, sans relancer automatiquement l’inférence. Les lots périmés ou invalides sont refusés sans application partielle. Une correction technique des poids n’est pas un résultat artistiquement parfait.

Les tests automatisés avec mocks valident transports et contrats ; ils ne certifient ni la qualité de Qwen, ni tous les fournisseurs réels, ni l’assemblage artistique. La qualification manuelle correspondante reste distincte et en attente. Aucun appel réel d’IA n’a été effectué pour T837.

## Au quotidien

Ouvrez Jade via **File > Open…**, puis sauvegardez une copie avec **Ctrl+Shift+S**. **Ctrl+S** sauvegarde ; **Ctrl+N / Ctrl+O** démarrent les parcours New/Open protégés ; **Ctrl+Z / Ctrl+Y** utilisent l’historique natif par défaut. Vérifiez la keymap dans Settings : les raccourcis sont configurables et certains champs capturent les touches.

Choisissez explicitement Setup ou Animate, le clip, le skin et l’attachment concernés. Utilisez la hiérarchie pour sélectionner, le viewport pour naviguer/manipuler et l’inspecteur pour les valeurs précises. Les outils de sélection, zoom/pan, cadrage et gizmos sont contextuels ; les commandes désactivées signalent un périmètre non éditable. Les diagnostics aident à naviguer vers un élément, sans corriger automatiquement tous les problèmes ni garantir un cadrage de caméra.

Avant de signaler un défaut, notez version du candidat, étapes, mode, clip et erreur. Une petite reproduction sans projet privé ni identifiants suffit. Les contrôles physiques Windows et artistiques du candidat restent **PENDING_MANUAL** ; cette documentation décrit le code disponible, pas une certification de chaque geste.
