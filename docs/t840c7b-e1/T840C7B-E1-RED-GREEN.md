# T-840C7B-E1 — RED / GREEN

## Unchanged baseline and genuine project

B6 workspace built before correction (B6-baseline-build.log). The saved C7B near-singular Character.ankh and all its image assets were loaded, saved into a new directory, reopened with identical author schema/image bytes and exported by the real native template. The permanent test failed for geometry, not compilation: RED-permanent.log. RED-native/ and before/ contain the original bad native exports; original C7B evidence is untouched.

The torso matrix is [a,b,c,d,tx,ty] = [0.00009999999747378752,0,-8.742277735063197e-12,0.00009999999747378752,10000,25]. The f32 determinant is approximately 9.99999949e-9. Core cutoff: 1e-12. Old export cutoff: f32::EPSILON. Native original vertex 0 WORLD: (9999.99609375,25.011882781982422).

| Vertex 0 | Old export | Corrected export |
|---|---|---|
| Influence LOCAL x | 9999.99609375 (actually WORLD) | -40.319550044994656 |
| Influence LOCAL y | 25.011882781982422 (actually WORLD) | 118.8201043337502 |
| Reconstructed WORLD x | approximately 10000.99999958 | 9999.995968044059 |
| Reconstructed WORLD y | approximately 25.00250118 | 25.01188201013321 |
| Maximum fixture error after f32 | 1.00390625 | 0 |
| Maximum unrounded source-f32 error | 1.003905833893441 | 0.00033018468275258783 |

INDEPENDENT-ORACLE.json contains every source vertex, bone matrix, native inverse, determinant, coefficient, exported LOCAL and reconstructed WORLD. Python decimal forward transforms at 70 digits do not call the corrected inverse conversion. Maximum corrected residual against exact affine H*v is approximately 1.31e-18 on the blocker; source-f32 residual above is the relevant practical comparison. Tolerance stays 0.01.

## Permanent tests and intermediate failures

Two private math tests plus seven native integration tests cover identity/translation/rotation/uniform/nonuniform/reflection, determinants around 1e-7/1e-8/1e-12 and the exact native boundary, singular/nonfinite/extreme inputs, one/multiple influences, zero/nonunit coefficients, actual saved near-singular, repeated deterministic output, finite reconstruction, UV/index preservation, malformed rows/unknown bone and legacy rounding refusal. No renderer or runtime test thresholds were changed.

All intermediate logs are retained. Early implementation/test harness corrections included a String unwrap compile error, exact widened-f32 assertions versus parser f64 last-bit rounding, and a tiny host fixture whose collapse was legitimately below tolerance (the corrected regression uses a separate root host). A saved-preset classification regression exposed by existing dark-tint tests was fixed by exact recognition of shipped template versions. The first full suite hit the source-digest contract guard; only two reviewed source digests were updated after verifying unchanged native field tables and historical data.

Windows restricted execution denied hard links in pre-existing staging tests; the unchanged tests pass with normal Windows filesystem access. The B6 archive needed the existing community-plugins fixture checkout for full Clippy. A shared target cache retained a B6 metadata artifact during comparison; refreshing/rebuilding current export sources resolved it. Core tests initially used the wrong cwd, then passed 105/105; a launcher Unicode-print error occurred after those tests exited 0. These are retained setup/harness failures, not falsely counted green runs.
