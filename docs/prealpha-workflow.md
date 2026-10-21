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
