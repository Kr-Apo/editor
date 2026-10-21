# Ankhimate — Next roadmap, reconciled after T-831

Audit date: **2026-10-21**. Product frozen at T-831
`4e4d50b0aa5b7ac160b204540afa878f66fa4d61`, parent T-830
`12252e476b95f9d2ac250d5d0746c589c10269b3`. **T-832 changes documentation only.**

Public Alpha `v0.2.0-alpha.1` remains the exact T-811/T-812 artifact: editor
`9233b8e888c811e7c71300101f865a9e74b3f489`, runtime
`c4c38df269cfcb30f941221454b7202f40d3fa5b`. T-813–T-831 refinements are local
candidates, even though their internal version string remains 0.2.0-alpha.1.
No GUI/inference/public operation in T-832. No local Windows control is marked PASS.
Accepted **T-831** evidence is 2,058 Rust passes / 0 failures / 4 existing ignores;
**T-832 does not rerun that suite** and adds no tests or executable. Relative to
T-816/T-817's 1,596 passes, the historical registration increase is 462, not 462
new tests executed by this documentation mission.


All missions below are **proposals, not authorizations or work started**. No new
P0 corruption/crash is demonstrated. P0 below identifies release gates with data
or compatibility impact, not a claim that every advanced feature is critical.

## Five ordered next missions

| Order / proposal | Historical task / existing capability | Exact remainder and user value | Data / format-runtime risk | Dependencies / size | Why now / measurable acceptance and stop |
|---|---|---|---|---|---|
| 1 — Feature-level format/release policy (P0 gate) | T-605/T-108/T-502; valid old files load, new Path files persist and old readers reject | State minimum reader and exchange/candidate policy so colleagues do not mistake the unchanged version1 or app string for interoperability | High impact if communicated incorrectly; no current silent downgrade demonstrated | Frozen T-831 3-profile/old-reader evidence; Small documentary decision, code/migration only by later authorization | Matrix old/new with/without Path and all targets; no implicit field removal; stop before any unapproved schema/runtime/version change |
| 2 — Cumulative Windows qualification (P0 release gate, no assumed code) | T-813–T-831 implementation and 352 pending instances already exist | Execute the retained map on exact T-831, record real focus/artwork/Undo/Save/recovery/export/performance behavior | Real data safety interactions; use disposable copies only; no new alpha implied | Order1 compatibility policy, complete fixtures and chosen Windows hardware; Medium manual campaign | Every source ID PASS/FAIL/NOT RUN with exact artifacts and reproducible failures; stop on data-loss/crash and preserve evidence; no automatic fix/publication |
| 3 — Selection-scoped numeric retiming (P1 artist workflow) | T-211; whole-clip checked Retime and fixed shortcuts already work | Add an explicit selected-track/key scope without turning T-905 static offsets into key rewriting; avoids moving unrelated animation | Medium: collisions, duration/excluded metadata and stale addresses; no format/runtime extension expected | Existing commands/leases, clarify exact selection semantics first; Small/Medium | Red selection-scope test, untouched other tracks, refused batch retains Redo, one Undo and 3 cycles, Save/reopen; stop on ambiguous complex ownership |
| 4 — Bounded FFD key-block clipboard (P1) | T-209/T-404; scalar/slot key blocks and one-key FFD editing already work | Own vector payload for one exact compatible mesh/topology; reuse KeyClip/PasteKeys or a justified typed extension, no scalar flattening | High: influence-count layout, owner/skin/linked references and assets; no topology or retargeting | T-821 capture invariants and explicit mapping; Medium | Exact vector/time/curve snapshots, all-or-nothing mismatch/lock/stale refusal, clipboard-free Redo,3 cycles and Save/reopen; stop before topology/FFD format expansion |
| 5 — Native PathSpacing animation (P1 optional enrichment) | T-502; static spacing and animated position already work | Add spacing keys only after auditing units/identity and version policy; retain same solver rather than replacing Path | Medium/high author-format exchange risk; JS support not assumed | Order1, manually reviewed PathPosition, separate authorization; Medium | Three-key5-bone numerical references across speed/open/closed cases, strict identity/history/3 profiles and explicit unsupported-export policy; stop if migration exceeds authorization |

## Further P1 and P2 backlog

T-302 remains a bounded single explicit PSD mapping; consider multi-missing-path
review only with stable provenance and full preservation tests. T-909 region copy
already works: explicit Move or mesh transfer needs its own ownership and source
mutation contract. T-402 preserves compatible weights, but FFD-dependent retrace
and general performance/artistic guarantees remain separate. T-905 beyond per-bone
offsets and T-902 expressions/neighbour snapping are real remaining extensions.

P2: reproduce B03 with a physical target/focus record before changing menus;
add clip reorder only (T-208), not a new animation manager; audit remaining
metadata commands without reopening closed FPS B02; measure remaining large
command/model/payload cost rather than repeating T-827/T-829 optimizations.
For T-912 decide the historical opt-in snapping intent versus existing default
frame snap/Alt-free convention before proposing a focused policy change.

## Deferred / separate contracts

Advanced runtime constraints/linked meshes/blending and stateful physics parity,
universal multi-skin composition, web editor, localization, additional platforms,
signed installers and major format migrations are separate. Preserve current
exports and exclusions; absence of advanced parity is not automatically P0.
No GPU/inference/GUI work or product development occurs in this reconciliation.

The old T-818–T-831 proposals are implemented checkpoints, never next work to recreate. See [current inventory](FEATURE_INVENTORY.md), [manual map](cumulative-manual-qa-map.md), [format policy boundary](format-release-compatibility.md) and [historical notes](checkpoint-journal-before-T832.md).
