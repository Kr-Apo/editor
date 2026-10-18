# Périmètre alpha candidat M007.2

Candidat local, non publié. Cette matrice est la référence unique du périmètre
alpha ; une fonction présente dans le modèle n'est pas une promesse de fidélité.
Références produit : éditeur d644653 + préparation documentaire M007.2 ; runtime
56dc0f1 (code snapshot m0061-8888e0e inchangé).
Les identifiants complets et SHA-256 livrés figurent dans VERSIONS.json et SHA256SUMS.txt.

| Fonction | Éditeur / export natif | TypeScript / Phaser | PNG / spritesheet | Niveau de preuve / limite |
|---|---|---|---|---|
| PSD, rig, animation, save/open, historique | Socle alpha | Consomme le véritable export | Consomme le même document | Régressions M001/M003/M005 ; walker et tagged_rig |
| Canaux scalaires X/Y, courbes indépendantes | Pris en charge, format v2 | Pris en charge ; lecteur v1 conservé | Évaluateur Rust commun | Parité numérique M001a, tolérance 0,01 unité |
| Régions, attachments, atlas, ordre, alpha Normal | Pris en charge | Pris en charge | Pris en charge | Exports réels et mesures WebGL M003/M004 |
| PNG temporel / spritesheet | Clip, FPS, caméra commune, pages déterministes | Sans runtime pour lire les pixels | Skin par défaut, blending Normal uniquement | 60 frames pour 1 s à 60 FPS ; 180 frames de référence |
| Dark tint statique RGBA | Transmis par le preset natif actuel | Qualifié avec snapshot exact ci-dessous et Normal | Renderer CPU existant | M005.3 : valeurs statiques et alpha ; aucun canal dark inventé |
| Meshes pondérés / FFD | Présents ; mapping FFD corrigé | Scénarios multi-bones et FFD qualifiés | Évaluation CPU commune | M005.4b1, plusieurs temps et bornes stepped ; pas toutes les compositions |
| Affectation d’image par skin | Inspecteur Setup, région uniquement ; conserve géométrie/UV et asset partagé | Deux images réellement distinctes qualifiées | Skin par défaut uniquement | M007.1b3 : pas meshes ni séquences ; fallback et Undo/Redo testés |
| Changement de skin / UV GPU | Export natif | Cas région/mesh, pages multiples et animation qualifiés | Skins alternatifs exclus | M005.4b2 ; compositions multi-skins non qualifiées |
| Clipping | Auteur et export présents | Cas M005.4b3 qualifiés, dont un concave simple | Renderer CPU existant | Ne garantit pas tous les polygones concaves ni toutes les interactions |
| Physique | Simulateur Rust avec état ; données exportées | **Incompatible avec l'export natif actuel** : bone simulé dans target, bones vide, consommateur utilise bones[0] ; ne s'active pas | Refus explicite | Exclue alpha. Ce défaut réel n'est pas une simple absence de tests |
| Paths avancés | Présents | Divergences de jonction et fallback skin démontrées | Présents mais non qualifiés généralement | P2, exclus du périmètre fidèle alpha |
| Add / Multiply / Screen | Présents | Divergences CPU/Phaser de composition démontrées | Refus des exports rendus | P2, Normal uniquement pour le parcours alpha |
| Linked mesh avec FFD hérité | Présent | Cas limités | Défaut CPU d'héritage identifié | P2, exclu de la fidélité alpha ; avertissement conservé |
| Chaînes de linked meshes | Modèle présent | Résolution sur un niveau : géométrie/déformation peut manquer | Même limite sur un niveau | P2, incompatibilité signalée, exclue alpha |
| Compositions de skins / séquences complexes | Présentes en partie | Insuffisamment qualifiées | Skin par défaut uniquement | Exclues des promesses alpha |

## Combinaison runtime qualifiée

Snapshot local **m0061-8888e0e** : @ankhimate/runtime 0.1.1,
@ankhimate/phaser-runtime 0.1.0, Phaser 3.90.0, fflate 0.8.3.
Les numéros npm seuls ne prouvent pas la présence des corrections dark tint,
FFD, UV skin et clipping. Utiliser les archives identifiées par leurs SHA-256
et le JavaScript du jeu préparé à partir de ces archives. Aucun package publié.
Le format runtime reste **2**, le lecteur accepte les anciens v1/v2.
Un ancien v2 à FFD non linéaire mal sérialisée doit être **réexporté** depuis
le projet auteur ; changer son numéro ou son lecteur ne restaure pas les courbes.

Le préflight ignore l'identité réelle de votre consommateur. Son avertissement
dark tint et le consentement restent nécessaires pour un consommateur inconnu.
Un ancien preset qui omet le sombre conserve sa perte connue : choisir le preset
intégré actuel, puis le consommateur qualifié. Aucun avertissement ne certifie une
fonction non testée. Ne pas utiliser le JSON runtime comme format auteur.

## Garanties de fichiers

Save/Save As préparent une écriture et ne changent le chemin actif qu'après
succès. Autosave reste séparée du projet primaire. Les exports préparent leurs
fichiers puis publient avec sauvegardes et rollback pour les échecs ordinaires.
La prévalidation ne réserve pas le dossier ; chemins, collisions, permissions
et verrous sont revérifiés lors de la publication. Les orphelins sont conservés.
PNG/spritesheet refusent une destination en collision. Ne pas confondre ces
protections avec une transaction atomique multi-fichier ou une garantie de
durabilité après panne de courant : une interruption brutale peut laisser un
staging ou une sauvegarde à récupérer. Ne jamais supprimer une sauvegarde sans
avoir vérifié le projet et ses assets.

## Qualification Windows restante

Le nouveau candidat M007.2 est préparé sans essai GUI. Le retour utilisateur M006.2
confirme lancement, import PSD, sauvegarde et exports sur Windows 11 25H2, mais
ne certifie pas ce nouveau ZIP ni tous les scénarios de panne/GPU/Phaser.
La préparation s'effectue sur une machine Windows de développement.
Restent à qualifier après revue : Windows propre sans toolchain, extraction et
chemins variés (espaces/Unicode), GPU/pilotes différents, signature et avertissement
Windows éventuel, droits limités, antivirus/verrous réels, longue session et
projets PSD de production lourds. Aucune garantie générale de performances ni
validation de distribution publique ne découle des petits exemples fournis.
Vidéo, IA, nouvelles fonctions avancées et publication sont hors périmètre.

## Mesure exploratoire PSD lourd M006.1

Sur la machine de préparation, un PSD synthétique non compressé de 100 calques
512 × 512 (105 652 682 octets) prend environ 31 ms à lire, 31 ms à analyser et
310 ms à importer via les API Rust Release. La copie des seuls octets utilisée
par la preview coûte en moyenne 18,0 ms (50 répétitions). Le code clone la preview
à chaque passage UI et confirme l'import sur le thread UI ; des pauses sont donc
possibles sur de gros PSD. Ce banc synthétique ne mesure pas la latence complète
ressentie, les textures GPU ou tous les PSD Photoshop de production. Pas de
promesse de 60 FPS ou d'import sans blocage ; qualification réelle à poursuivre
après revue, sans correctif de performance dans M006.1.

## Anomalies résiduelles (sans correctif dans M007.2)

- Menu contextuel sur le **nom** d’un bone inaccessible : clic droit sur son
  **icône** est le contournement qualifié M007.1a.
- Message transitoire `open failed: json error: trailing characters at line 1
  column 2` observé pendant M007.1b3 : cause non établie, non déclaré résolu.
  Conserver le projet et ses assets, vérifier la réouverture avant toute action
  destructive et joindre les étapes au rapport de bug.
- FFD : drag corrigé, mais pas de nouvel éditeur de courbes/dopesheet FFD.
- Réimport PSD automatique préservant toute animation non finalisé ; utiliser
  preview Replace/Merge et travailler sur une copie.

Les corrections M007.1b1–b4 ne qualifient pas davantage la physique, les paths,
les compositions de skins, les linked meshes ou le clipping complexe.
