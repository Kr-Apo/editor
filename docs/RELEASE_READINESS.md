# État local T837 — 22 octobre 2026

Ce checkout contient le correctif ciblé de persistance Recovery T837 sur T836 `1d3cc13bd5fda93c541dc8698918dedcdd146290`, avec documentation utilisateur et deux livraisons locales séparées. Aucun format, runtime, numéro de version ou artefact public n’est modifié. Les tests réellement exécutés et le commit final figurent dans la livraison T837. L’Alpha publique v0.2.0-alpha.1 et RC2 conservent leurs propres preuves.

Les **444 contrôles manuels historiques restent PENDING MANUAL REVIEW** ; T837 ajoute 15 contrôles distincts, eux aussi en attente. Consultez [le catalogue utilisateur](FEATURES.md), [le démarrage](GETTING-STARTED.md), [la compatibilité](COMPATIBILITY.md) et [la récupération](project-recovery.md). Les 101 IDs conservent 62 COMPLETE, 33 PARTIAL, 1 ABSENT, 3 SUPERSEDED, 2 DEFERRED. T208/T209/T211/T404/T502 restent PARTIAL, sans reclassement par effet d’une mission récente.

<details>
<summary>Référence antérieure conservée — état daté T836 ou avant</summary>

# Release readiness — local T836 preparation

**No new publication authorized or performed.** Accepted public Alpha v0.2.0-alpha.1 remains editor9233b8e /runtimec4c38df, recorded by T812. RC2 remains byte-identical. This checkout and its Windows-Test-Candidate are local development/test material with unchanged internal versions.

| Gate | Current disposition |
|---|---|
| Source | Accepted T835514c6aa8 plus T836 cleanup/docs/sample copies; no product code/dependency/format/runtime changes |
| Automated SOL checks | Exact results/logs in T836 VALIDATION.json; no invented PASS if a check fails |
| Independent Qwen | Handoff prepared; independent execution pending, no model inference required or performed |
| Manual Windows |444 cumulative cases PENDING MANUAL REVIEW; no new physical Windows/artistic acceptance |
| Native author exchange | PathPosition recent-reader requirement and old-editor clip-order risk documented; future alpha policy still a review gate |
| Game runtime | Protected external adapters, supported native-v2 subset only; no new GPU campaign or PathPosition support |
| Distribution | Unsigned local x64 EXE/Jade/notices/hash manifest, no public ZIP/tag/version change |

Tentative next editor label0.2.0-alpha.2 is a proposal only. Any future release needs its own approved artifacts/notes/minimum reader and appropriate retesting. Old clean-PC/GPU/AI reports apply only to their recorded scopes. [Compatibility](release-compatibility.md), [manual plan](MANUAL_QA_PENDING.md), [build](CANDIDATE_BUILD.md), [Jade](jade-sentinel.md).

</details>
