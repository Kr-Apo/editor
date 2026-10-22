# État local T837 — 22 octobre 2026

Ce checkout contient le correctif ciblé de persistance Recovery T837 sur T836 `1d3cc13bd5fda93c541dc8698918dedcdd146290`, avec documentation utilisateur et deux livraisons locales séparées. Aucun format, runtime, numéro de version ou artefact public n’est modifié. Les tests réellement exécutés et le commit final figurent dans la livraison T837. L’Alpha publique v0.2.0-alpha.1 et RC2 conservent leurs propres preuves.

Les **444 contrôles manuels historiques restent PENDING MANUAL REVIEW** ; T837 ajoute 15 contrôles distincts, eux aussi en attente. Consultez [le catalogue utilisateur](FEATURES.md), [le démarrage](GETTING-STARTED.md), [la compatibilité](COMPATIBILITY.md) et [la récupération](project-recovery.md). Les 101 IDs conservent 62 COMPLETE, 33 PARTIAL, 1 ABSENT, 3 SUPERSEDED, 2 DEFERRED. T208/T209/T211/T404/T502 restent PARTIAL, sans reclassement par effet d’une mission récente.

<details>
<summary>Référence antérieure conservée — état daté T836 ou avant</summary>

# Sharing projects and preparing a future alpha

## Which artifact is this?

Public v0.2.0-alpha.1 is the accepted T811/T812 release; RC2 is an older independent frozen archive. T836 is a local candidate from accepted T835. Internal application0.2.0-alpha.1, author schema1, envelope1 and runtime v2 are independent numbers. None certifies equivalent features.

| Data / operation | Safe expectation |
|---|---|
| Older valid author files without new features | Current editor retains supported data/assets; actual migrations, reader rejection, Save/Reload and recovery tests remain authoritative |
| New project with PathPosition | Use a reader containing T831 or later (this candidate includes it). Old closed-enum readers reject `path_position`; public-alpha inability is supported by its source contract, not a newly executed old-GUI test |
| Sharing PathPosition files | Keep a full backup and use a recent editor. Never strip the tag, silently bake a static path or round-trip through an old editor |
| Native runtime v2 / script export with PathPosition | Refused before publication. PixiJS/Phaser do not acquire support from an editor feature |
| CPU sequence/spritesheet | Raster poses can include native PathPosition through the existing evaluator; default skin, Normal blending, physics excluded |
| T835 clip order | Current author list is saved in the existing array; old editors may sort/resave it in another order. Old-reader resave preservation is not guaranteed |
| Other native-v2 features | Use matching external core/adapter and its documented subset. Shared core v1 reading is not a promise every adapter accepts v1 |
| Save/export interrupted by failure | Staging/backups/rollback cover tested ordinary failures. No guarantee of crash-atomic multi-file visibility or power-loss durability |

No format migration/version bump/runtime change here. Tentative next editor candidate label **0.2.0-alpha.2** is a planning proposal only. Before a future alpha, approve exact source/binary, minimum feature reader, release notes,444-case qualification disposition and new clean-PC evidence. Do not reuse old-archive acceptance as certification of the new binary. Preserve all licenses, attribution and original artifacts.

[Detailed historical format evidence](format-release-compatibility.md), [format specification](format-spec.md), [export boundaries](formats/export-runtime.md), [local workflow](prealpha-workflow.md).

</details>
