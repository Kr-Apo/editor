# T-840C7B-E1 — Native weighted bind export

Verdict: PASS for this bounded native-export correction. C7B weighted graphics remains HOLD; WebGL and performance are NOT_RUN.

## Cause and change

B6 `275082e8f26d42389ed05fb92c45e2822006525d` used machine epsilon (approximately 1.1920929e-7) as its export determinant cutoff while core accepts abs(det) >= 1e-12. The rejected export path returned WORLD coordinates in influence-local x/y fields. The real saved C7B fixture reproduces 1.00390625 unit error after f32 conversion. A threshold substitution alone also retains f32 cancellation at large origins and four-decimal bone serialization can collapse smaller valid scales.

The exporter now checks core inverse eligibility and finite native matrices/inverses, solves relative-translation coordinates in f64, and checks forward reconstruction at the existing 0.01 tolerance. Invalid bindings produce explicit inspection errors and export planning fails before publication. There is no identity or WORLD replacement. The native template retains authored bone precision on weighted rigs; unweighted formatting stays unchanged. Actual serialized native numbers are checked again, including recognized older saved templates, which are never rewritten.

Runtime v2 field names, shapes, units and versions are unchanged. The existing contract specifies JSON numbers, not float32 storage of derived coordinates; .ankh vertices, weights, bind/evaluation math remain f32. Derived influence coordinates retain f64 precision. No shared math/evaluator/Physics/clock/consumer or TypeScript code changed. This does not promise identical precision in a consumer that first downcasts all JSON coordinates to f32.

## Qualification

- Permanent RED from the actual saved author project; Save/Reload and native export, not a patched runtime JSON.
- GREEN: 0.000330185 maximum unrounded bind residual versus source-f32 WORLD; zero after f32 conversion on near-singular. Independent 70-digit decimal forward oracle detects the original substitution.
- Three saved native projects, 537 numerical states each; maximum native skinning discrepancies: near-singular 0.001452063, simple 0.000011780, mixed 0.000026817. These include committed ticks, preview seeks and the expected numerical-fault scenario. They are numerical checks, not new weighted graphics acceptance.
- Nine added Rust tests (two private math tests, seven saved/native integration tests), including nine transformed setups, exact native threshold neighbours, invalid binds, repeated exports, old template precision refusal, unknown influence and malformed rows.
- Full workspace: 2,231 passed / four ignored. Fmt, workspace Clippy/build, core WASM and docs-check pass. The 25 Clippy diagnostic messages match B6 exactly; no new product warnings.
- Existing C7A 102 targeted tests and 105 compiled core tests pass; the unchanged public parseRig accepts 19 genuine before/after/corrected exports without numeric mutation. No fresh graphics/performance claim.

## Scope and review points

Production files: export/src/context.rs, run.rs, preflight.rs and presets/ankhimate_runtime.json. Support: permanent fixture/test, native numerical example, export/format documentation, two reviewed source digests in docs/format-schema.json, and these six reports. Source digests are the deliberate contract-review guard, not updated historical goldens.

Singular, native-rejected and nonfinite binds now explicitly fail export where B6 emitted misleading coordinates. An old native template which rounds away more than 0.01 unit also fails explicitly. These intentional error semantics need review; no saved template or project is migrated. The valid historical inventory has no refusals. Floating-point conditioning is checked per point by reconstruction, not certified by determinant magnitude alone. The tested finite domain is bounded; there is no all-matrices theorem or new general graphics guarantee.

27,950 protected prior files, original repositories/branches/remotes, 97 archived prototype files and RC2 are preserved. Four unweighted reference JSONs and all eight author saves/atlases/assets are byte-identical before/after; weighted differences are exhaustively inventoried. No push, merge, tag, version or publication. Local commit/Git receipt, complete patch, hashes and raw logs are in the delivery. STOP for review.
