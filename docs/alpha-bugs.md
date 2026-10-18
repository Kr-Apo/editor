# Signaler un problème de l’alpha

Créer une issue dans le dépôt éditeur, ou transmettre ces éléments au mainteneur :

- Identifiant du candidat et commits de `VERSIONS.json`, SHA-256 du ZIP.
- Windows, GPU et pilote ; préciser si Rust/Node sont absents de la machine.
- Étapes minimales, résultat attendu, résultat observé et fréquence.
- Projet minimal `.ankh` **avec son dossier `.assets`** ; PSD minimal si import.
- Message exact et, si disponible, journal existant expurgé des données privées.

Travailler sur une copie. Ne pas joindre de chemins personnels, clés, projets
clients ou images sans autorisation. Ne pas supprimer le projet, l’autosave ou
un backup pour tenter de reproduire une perte. Aucune télémétrie n’est requise.

## Réserves finales

| Priorité | Réserve / décision avant publication |
|---|---|
| P0 — validation de sortie | **Dernier essai manuel du nouveau ZIP et accord de sortie manquants.** Les anciennes preuves ne certifient pas ce binaire. Toute corruption/perte/crash reproductible sur le parcours essentiel détectée alors bloque la sortie. Aucun tel nouveau défaut n’est démontré par M007.2. |
| P1 | `open failed: json error: trailing characters at line 1 column 2` transitoire, observé M007.1b3 ; cause non établie. À investiguer ou reporter explicitement avec revue ; vérifier Save As/réouverture et préserver les fichiers. Aucun correctif prétendu. |
| P1 | PSD lourds : clone de preview et import peuvent bloquer l’UI. Mesure synthétique M006.1 limitée ; pas de garantie générale de performance. Alpha limitée aux projets vérifiés, test d’un PSD utilisateur représentatif demandé. |
| P2 | Clic droit sur le nom d’un bone inaccessible : utiliser le clic droit sur son **icône**, contournement qualifié M007.1a. |
| P2 | Physique native/TS incompatible, paths avancés, linked meshes/FFD hérité CPU, blending non-Normal, composition multi-skins et clipping complexe : exclus du périmètre fidèle, avertissements à lire. |
| P2 | Dopesheet FFD limité, réimport PSD automatique incomplet ; ne pas remplacer un projet animé sans copie/preview. |
| P2 | Binaire non signé : vérifier le hash et la provenance. Aucune consigne de contourner Windows/antivirus ; remonter un blocage de lancement au mainteneur. |
| P2 | Localisation, vidéo, navigateur éditeur, panic dump, accessibilité complète et performances générales : reportés après l’alpha. |

Un P2 exclu et documenté ne devient pas automatiquement un P0. La physique n’est
pas annoncée compatible : elle n’est pas une condition du parcours alpha retenu.
