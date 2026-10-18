# Keyframe clipboard: mirrored bone offsets (T-209.3)

In Animate mode, select editable bone transform keys in the dopesheet and use
**Copy selected keys**. Select destination **keys on the intended rows**, set
the playhead, then use **Paste Keys Mirrored (Offsets)** in the context menu.
Existing destination-key selection supplies row identity; this adds no separate
empty-row selection tool. Right-clicking an unselected key selects its row/key
according to the existing dopesheet behavior. For several destination rows,
select their keys first and right-click within that selection.

The action is disabled until source and destinations validate. Source track order
is the copied visible-row order; selected destination rows map in current visible
row order, independent of Ctrl-click order. Multiple selected keys on one row
count as one destination row. The number and property/axis of the rows must match.
No `.L`, `.R`, left/right naming convention is guessed; use arbitrary bone names
and choose destinations explicitly. Unlike **Paste keys at playhead**, mirrored
paste refuses a missing destination selection instead of falling back to the
original named tracks. Normal paste retains its existing behavior.

## Values and scope

| Bone animation channel | New key value |
|---|---|
| Translate X | `-value` |
| Translate Y | unchanged |
| Rotate | `-value` (degrees remain degrees) |
| Scale X / Y | unchanged (multipliers, not additive deltas) |
| Shear X / Y | `-value` (existing units retained) |

This mirrors **animation offsets**, not absolute local poses. No setup transform,
name, hierarchy, image, asset or skin is edited. For an asymmetric destination,
setup X=100 plus a mirrored translation offset -12 gives X=88; it does not promise
reflection of the source absolute pose. Existing pose mirroring is a different
operation with its existing absolute-transform semantics.

KeyClip is copied by value. Original source times/order remain in the clipboard;
pasted times are `destination_playhead + (source_time - earliest_source_time)`,
using the same f32 validation and exact-time replacement as normal PasteKeys.
Linear, Stepped and Bezier interpolation remain unchanged. Normalized Bezier
handles are not negated: negating the values of both endpoints already negates
that eased scalar curve. Overshooting y handles remain unbounded as before.

## Rotation boundaries

No key normalization, wrapping, evaluator change or unit conversion is introduced.
Real sampler tests use Linear, Stepped and overshooting Bezier at times
0, 0.25, 0.5, 0.75 and 1 s for endpoints 0→±180, ±179→∓179, ±180→∓180,
179.999→-179.999 and 540→-540 degrees.

| Source keys | Mirrored stored keys | Linear midpoint source / mirrored |
|---|---|---|
| 0 → 180 | -0 → -180 | 90 / -90 |
| 0 → -180 | -0 → 180 | -90 / 90 |
| 179 → -179 | -179 → 179 | 180 / -180 |
| -179 → 179 | 179 → -179 | -180 / 180 |
| 180 → -180 | -180 → 180 | 180 / -180 |
| 540 → -540 | -540 → 540 | 540 / -540 |

The existing shortest-arc sampler may return an equivalent angle outside the
endpoint representation: 179→-179 passes through 180 and 180.5 before holding
-179 at the last key. A difference of 360 degrees in numeric representation is
not a changed orientation. Multi-turn intent is not newly implemented. For these
fixtures, mirrored samples equal the negative original within 0.0001 degree.
This does not promise absolute/world-space reflection under asymmetric setups,
constraints or hierarchy transforms.

## Refusals and history

Requires finite values/times/handles and valid editable bone property/axis rows.
Missing, stale, ambiguous, locked, colliding, wrong-count or incompatible targets
refuse before writing. Mixed selections containing slots or read-only tracks
refuse the whole operation. Slot colors/visibility/attachments, FFD, IK and other
constraints, draw order and events have no mirror in this action. No retargeting
or image reflection is inferred.

The existing PasteKeys transaction owns all transformed values. One authored block
is one Undo/Redo step, independent of drags and adjacent pastes. Redo does not read
the clipboard. Undo restores timelines and duration; a rejected operation leaves
Redo intact. Existing T-210 pending property previews use the same rebasing hooks
on paste, Undo and Redo. This explicit key action writes keys even when auto-key
is off, exactly like normal key-block paste; auto-key-off **pose** paste remains
session preview only (T-209.2). Save/reopen preserves the authored keys/assets.
