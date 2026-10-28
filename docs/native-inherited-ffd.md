# Native inherited FFD ownership — A10

`Skeleton::resolve_mesh_pose` is the shared CPU/viewport resolution boundary. It returns the displayed owner, geometry owner, deform owner, displayed texture, host bone and sampled offsets. `MeshPose::vertices` uses existing native skinning; no sampler or solver is added.

Qualified linked domain: one fixed default skin, direct source mesh, fixed topology/influences, one clip and existing Physics. No dynamic skin/mixing/clipping qualification. A null link skin resolves the sole default skin. Unknown skins, multiple-skin ambiguity, missing owners, nonmesh sources, chains/cycles, malformed topology/UV/influences, missing/singular binds, invalid effective offsets and nonfinite vertices are explicit errors. Nonlinked behavior remains unchanged.

- `inherit_deform=true`: only the exact source `(skin,slot,attachment)` owns the FFD. A displayed own track is ignored; absence of the source track means rest, never an own-track or same-size fallback.
- `inherit_deform=false`: only a displayed own track applies. Native author schema/API and export can represent it using the displayed slot/name. No own track means rest, without borrowing source animation. Existing UI/shared-mesh FFD editing protections are retained; this does not add a linked-mesh creation/key editor.
- Source geometry, UVs, triangles, influence coefficients and derived inverse binds are borrowed unchanged. Weighted offsets are packed per influence, in source author space. The bind acts on `base+offset`, equivalently `I*base + linear(I)*offset`; translating the vector itself is incorrect. Empty influence rows keep displayed-host rigid fallback and consume no weighted offsets. Explicit empty offset arrays retain the existing rest-reset meaning.

Pose::deforms remains the canonical sampled `(slot,name)` map; aliases are not inserted. The one-skin restriction gives each tuple a single skin owner. The viewport sprite and linked selection outline use the same resolver as CPU. The existing mutable texture-preparation pass reports current linked errors; paint remains read-only. CPU returns Error::Invalid on rejected linked geometry.

Compatibility: no project/native v2/schema/preset change. Existing source-based inherited data now renders as its flag specifies. Data which used displayed own FFD despite `true` intentionally changes behavior; use `false` for an own track. Invalid links no longer silently fall back to their own geometry. Static valid links and nonlinked deformation remain covered. Multi-skin linked resolution is explicitly outside this initial qualification.

Headless evidence uses API authoring, Save/Reopen, native export, accepted native Replay Physics matrices, independent f64 source/bind/vector algebra and Bezier inversion, CPU pixels and actual viewport SpriteDraw buffers. No interactive GUI/GPU authoring qualification is claimed. The runtime A9 and its inherited Gate B remain unchanged/HOLD pending a separately reviewed consumer implementation.

Rendered-sequence Physics and inherited-FFD preflight exclusions remain conservative. The CPU error text was updated to remove the now-obsolete displayed-only lookup claim; this mission does not open new sequence/spritesheet/video export features.
