# T-840C7B-E1 — Native qualification

## Evidence inventory

before/ and after/ contain eight pairs: C7B near-singular/simple/mixed, C7A unweighted simple/mixed, walker, minimal and Jade Sentinel. All use genuine saved author projects, actual assets and the built-in native template. Source .ankh data/runtime JSON is never manually patched. Each helper saves/reloads into a fresh directory and asserts the complete author schema and all image bytes. The three green-native/ projects additionally contain BIND-REFERENCE.json, 241 committed ticks per normal clip, 17 preview ticks per normal clip and the expected fault-at-20 trace. Each project has 537 measured states, not 537 graphical captures.

The source author fixture added to export/tests/fixtures/native_bind_near_singular is byte-for-byte the saved blocker, not a recreated simplified substitute. Primitive test art is independent of protected user characters.

| Project | Maximum native-skinning discrepancy over 537 states |
|---|---:|
| near-singular | 0.0014520627973979572 |
| simple | 0.000011779067321526782 |
| mixed | 0.00002681665378645448 |

The committed example native_bind_reference builds native bind state, captures source matrices, exports and independently forward-skins serialized influences using scalar f64, then compares against native skin_vertex_with_deform and retains its measured result before asserting <=0.01. The separate Python decimal oracle verifies bind geometry without invoking the exporter inverse. BIND-REFERENCE.json records setup worlds/inverses and source mesh/source-f32 WORLD coordinates. The two oracles expose rather than erase the distinction between f64 ideal affine, f32 source and f32 native inverse/matrix multiplication.

## Reproduction

From Editor, with toolchain 1.95 and existing locked dependencies:

    cargo +1.95 test --offline --locked -p ankhimate-export --lib --test native_weighted_bind -- --nocapture
    cargo +1.95 run --offline --locked -p ankhimate-export --example native_bind_reference -- export/tests/fixtures/native_bind_near_singular/Character.ankh NEW_OUTPUT_DIRECTORY

Existing directories are refused. For general reference projects outside the bounded Replay fixture domain, append export-only. No graphics is initialized. The delivery scripts/oracle.py reconstructs before/green-native against their saved BIND-REFERENCE files; scripts/inventory.py compares the eight before/after pairs. Run these from the original evidence-workspace root, or adjust only their evidence directory location. REPRODUCE.md provides the layout and qualification boundaries. No prototype C7B source is needed.

The unchanged C7A public parseRig accepts all 19 real native outputs and preserves their numeric data exactly. The existing compiled core tests (105) and targeted C7A tests (102) pass. This verifies parsing/legacy behavior and preserved C7A functionality; it does not turn on weighted mesh rendering in its experimental graphics API. No new TypeScript implementation, image patch, UV fix, WebGL capture or performance measurement is included.

All delivered project/assets, native JSON/atlases, raw oracles/logs, patch and reports are sealed by SHA256SUMS.txt after the local commit. Commit SHA is in LOCAL-COMMITS.json and editor-commit.patch, avoiding a self-referential committed document hash.
