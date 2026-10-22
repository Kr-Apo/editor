# Complément manuel T837

**15 contrôles PENDING_MANUAL**, en plus des444 instances historiques conservées exactement dans MANUAL-QA-444.csv. Aucun geste physique ni capture exécuté par SOL. Ne jamais provoquer un crash sur les données réelles : préparez des copies et une racine de récupération isolée. Notez OS/build, GPU/pilote, DPI, focus, SHA EXE/projet, résultat attendu/observé, date et réviseur.

1. **T837-M01 — PENDING_MANUAL** : Premier lancement sans récupération dans une racine de test isolée : accueil sans popup ni copie vierge.

2. **T837-M02 — PENDING_MANUAL** : Copie de récupération réelle isolée : vérifier le dialogue et son identité.

3. **T837-M03 — PENDING_MANUAL** : Ignore : comparer projet, copie et assets avant/après ; aucune donnée effacée.

4. **T837-M04 — PENDING_MANUAL** : Fermer et redémarrer : même copie inchangée non reproposée.

5. **T837-M05 — PENDING_MANUAL** : Nouvelle copie et copie modifiée : nouvelle proposition, sans masquer tous les Untitled.

6. **T837-M06 — PENDING_MANUAL** : File > Review Recovery Copies… : retrouver une copie ignorée, y compris depuis un projet modifié (Cancel).

7. **T837-M07 — PENDING_MANUAL** : Recover puis Save As dans un nouveau dossier : dirty, assets, réouverture et original préservés.

8. **T837-M08 — PENDING_MANUAL** : Vérifier SHA du ZIP et son unique racine Ankhimate ; aucun chemin absolu ou traversant.

9. **T837-M09 — PENDING_MANUAL** : Double-clic Ankhimate.exe sans script ni toolchain ; contrôler accueil et fermeture.

10. **T837-M10 — PENDING_MANUAL** : File > Open… Examples/Jade-Sentinel/Sentinelle.ankh : cinq assets, idle/attack, Save As sur copie.

11. **T837-M11 — PENDING_MANUAL** : Vérifier navigation des licences/fonts/attributions et offre option-ext, sans présumer une équivalence légale.

12. **T837-M12 — PENDING_MANUAL** : Extraire dans un second dossier neuf et refaire ouverture/enregistrement sans dépendance à la source.

13. **T837-M13 — PENDING_MANUAL** : Contrôler les raccourcis Save/Save As/Undo/Redo avec focus viewport et champ de saisie.

14. **T837-M14 — PENDING_MANUAL** : Choisir les clips dans le manager et Timeline ; contrôler lecture et ordre sans promettre entrée Animate automatique.

15. **T837-M15 — PENDING_MANUAL** : New/Open/Close et accueil : Save/Discard/Cancel, erreur de persistance Ignore et Later (this session), aucune perte.
