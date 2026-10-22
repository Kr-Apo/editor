# Limites à connaître — T837

- Candidat Windows x64 non signé, local uniquement ; les contrôles physiques et artistiques restent en attente. L’Alpha publique est un autre artefact.
- Sauvegardez une copie avec ses assets. Aucun writer ne garantit une transaction multi-fichier durable après panne électrique. Les fichiers temporaires peuvent être nettoyés par Windows ; l’autosave n’est pas un backup permanent.
- Ignore conserve les données. Si son registre est inaccessible/corrompu ou perdu, une copie peut revenir ; une erreur de persistance est visible. Les copies valides plus anciennes qu’un primaire validé sont filtrées par horodatage. Pour revoir un autre projet nommé, ouvrez son chemin puis utilisez File > Review Recovery Copies…. Ce n’est pas un navigateur global de tous les disques.
- La récupération ne restaure pas l’ancien historique Undo, les sélections ou toutes les dernières modifications après crash. Un fichier endommagé ne devient pas automatiquement réparable.
- Les effets PSD complexes, les changements de topologie/ownership, les meshes liés et la migration des FFD ont des exclusions. Les refus protègent les références existantes.
- L’export CPU reste default skin / Normal / sans physique. Le JSON spritesheet n’est pas un atlas Phaser natif. MP4/GIF nécessitent FFmpeg externe ; aucun FFmpeg n’est inclus.
- PathPosition fonctionne côté auteur/CPU, mais les anciens lecteurs le refusent et les runtimes de jeu ne le prennent pas en charge. Les contraintes avancées, linked meshes et tous les modes de blending n’ont pas une parité universelle.
- T208 : affichage des durées en frames et entrée Animate depuis le manager encore incomplets. Certains types de clés ne participent pas aux commandes de groupe, Graph ou clipboard ; FFD reste un cas borné sans Graph vectoriel.
- Crossfade interrompu peut présenter une discontinuité selon le runtime. Aucun nouveau benchmark GPU ni qualification matérielle n’accompagne ce dossier.
- L’assistant IA est facultatif, dépend du fournisseur et de son modèle installé/configuré. Mocks et commandes natives testées ne démontrent pas une qualité artistique ou un support réel de tous les modèles. Ne partagez pas de projets privés ni de secrets dans un rapport de bug.

Consultez [FEATURES](FEATURES.md), [COMPATIBILITY](COMPATIBILITY.md) et [GETTING-STARTED](GETTING-STARTED.md). Les priorités futures concernent la qualification manuelle, les limites des workflows et la fiabilité ; aucune date de publication n’est promise.
