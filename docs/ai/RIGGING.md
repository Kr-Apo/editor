# Setup bone rigging v1

Use `ankhimate_rig` with exactly `summary` and `actions` (1–128, summary <=1000
UTF-8 bytes). Setup mode REQUIRED; ask the user to switch before proposing.
Read `ankhimate_read` section `bone` with exact `bone` name for parent details.

- `bone.create`: `op`, `name`, `parent` (existing name or null for root),
  `position` [x,y], `rotation_degrees`, `length`. Creates a unique bone; no
  guessed IDs or forward parent references. Earlier-created parents are allowed.
- `bone.setup`: `op`, `bone`, `position` [x,y], `rotation_degrees`, `scale` [x,y].
  Sets absolute PARENT-LOCAL setup values, not animation offsets. Shear and
  length stay unchanged. Coordinates are world-unit magnitudes <=1000000;
  rotation magnitude <=360000 degrees; scale magnitude 0.0001–100 per axis.
- `bone.rename`: `op`, `bone`, `name`. Preserves native IDs and references.

Names: nonempty, unique, <=128 UTF-8 bytes, no controls. Length: 0–1000000.
All numbers finite. Locked bones/parents and cyclic or missing hierarchies are
refused. No existing-bone reparent, delete, attachment/weight/mesh/FFD changes.
Creation uses identity scale/shear and ordinary native inheritance.

Place joints at plausible pivots; distinguish estimates from exact native facts.
Image pixels are NOT parent-local values. Prefer a small accessory under a real
unlocked torso. Setup changes preserve keys/references, but can visibly move art
and change animated poses: warn and request approval. One native Undo/Redo lot.
See [rig examples](rig-examples.json); [vision coordinates](VISION.md).
