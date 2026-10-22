# T838 — v0.1.5 local release preparation

Current local preparation,2026-10-22, based on accepted T837 `726217cb1795873ede43d7f41e935452c0e28002`. Product0.1.5 is a deliberate numbering reset, not a schema/runtime change. Welcome no longer shows a version; Help > About Ankhimate keeps build diagnostics. The four maintainer-supplied English guides are integrated with justified context corrections. New Editor/Runtime community forks are not created and their URLs are unset. The former public Alpha/RC2 are historical artifacts, not current download endpoints.

Historical101IDs remain62 COMPLETE/33 PARTIAL/1 ABSENT/3 SUPERSEDED/2 DEFERRED. T208/T209/T211/T404/T502 remain PARTIAL. All459 existing manual controls remain PENDING MANUAL REVIEW; the new18-step smoke test is separate. See [release preparation](t838-release-preparation.md), [features](FEATURES.md), [compatibility](COMPATIBILITY.md), [runtime contract](editor-runtime-compatibility.md).

<details>
<summary>Historical local state through T837 — retained evidence, not current release identity</summary>

# État local T837 — 22 octobre 2026

Ce checkout contient le correctif ciblé de persistance Recovery T837 sur T836 `1d3cc13bd5fda93c541dc8698918dedcdd146290`, avec documentation utilisateur et deux livraisons locales séparées. Aucun format, runtime, numéro de version ou artefact public n’est modifié. Les tests réellement exécutés et le commit final figurent dans la livraison T837. L’Alpha publique v0.2.0-alpha.1 et RC2 conservent leurs propres preuves.

Les **444 contrôles manuels historiques restent PENDING MANUAL REVIEW** ; T837 ajoute 15 contrôles distincts, eux aussi en attente. Consultez [le catalogue utilisateur](FEATURES.md), [le démarrage](GETTING-STARTED.md), [la compatibilité](COMPATIBILITY.md) et [la récupération](project-recovery.md). Les 101 IDs conservent 62 COMPLETE, 33 PARTIAL, 1 ABSENT, 3 SUPERSEDED, 2 DEFERRED. T208/T209/T211/T404/T502 restent PARTIAL, sans reclassement par effet d’une mission récente.

<details>
<summary>Référence antérieure conservée — état daté T836 ou avant</summary>

# Current local pre-alpha workflow — T836

Start with [README](../README.md), [Jade](jade-sentinel.md), [compatibility](release-compatibility.md), [inventory](FEATURE_INVENTORY.md), [tasks](TASKS.md), [manual plan](MANUAL_QA_PENDING.md). Local source is based on accepted T835; public v0.2.0-alpha.1/RC2 stay frozen. T836 adds no product capability and changes no version, dependency, format or runtime.

| Local checkpoint | Implemented refinement, still subject to its limits |
|---|---|
| T813 | Strict project read/Save/Load/recovery protections |
| T814–T816 | Animation/geometry input and rigging discovery |
| T817/T832 |101-task factual inventory/reconciliation |
| T818/T819 | Diagnostics/navigation and bounded region rig transfer |
| T820/T821 | Linked Bézier tangents and editable FFD keys |
| T822/T823 | Explicit PSD path resolution and safe compatible weighted retracing |
| T824–T827 | Onion refinement, Graph marquee/Normalize and selection validation cost |
| T828/T829 | Native FPS Undo and highlighting cost |
| T830/T831 | Numeric whole-clip retime and native PathPosition |
| T833 | Selected-bone transformation-key time offsets |
| T834 | Exact owned-mesh FFD clipboard, existing nonempty row only |
| T835 | Persistent clip Move Up/Down, stable active ID, independent Undo |

T208 remains partial for duration-in-frames display and manager selection entering Animate; do not recreate ordering. T209/T211/T404/T502 keep unsupported tracks/ownership, selection/time-scaling, loader-topology and Path/runtime limitations. Counts62COMPLETE/33PARTIAL/1ABSENT/3SUPERSEDED/2DEFERRED remain101. Manual444 pending is a separate population, not a completion percentage.

Source housekeeping retires the M006/M007 packager and old primary walkthroughs into [historical records](history/README.md). Crapaud paths stay for required regression/evidence inputs and are marked historical-only. Unknown autosaves, old build caches, reports, executables, PSD/PNG oracles and licenses are preserved; no broad purge. A clean active workspace does not mean destroying provenance.

The delivery contains a local Windows-Test-Candidate, exact hashes, disposable Jade qualification evidence and a Qwen handoff. Qwen must run independent checks, not accept the SOL report as its own test results. No inference/backend/cloud call is required or executed. No publication step is part of this procedure.

</details>

</details>
