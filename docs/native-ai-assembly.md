# Native character assembly (T-807 development build)

This local milestone extends the existing AI proposal boundary. It does not
assemble a flattened image, replace artwork, or qualify visual/model behavior.
No GUI, inference or renderer tests were run for T-807; maintainer acceptance
of T-806 vision and T-807 assembly remains pending.

## Read, propose, approve

Enable project metadata and tools; enable editing explicitly. Switch to Setup.
Ask the agent to inventory real slots/attachments/bones. `ankhimate_read` now
accepts `slots`; `attachments` includes type, skin, bone, lock/editability,
Setup draw index, region local position/rotation/scale/dimensions/pivot,
graphic logical reference and resolved dimensions, FK Setup corners and current
world origin where defined. Inventory is capped at 256 slots/attachments;
metadata retains the existing 64 KiB send cap. Counts signal truncation.
Private image source paths and bytes are not sent by metadata. Texture references
that resemble absolute paths or contain controls are redacted. Project names
remain untrusted data; images require the unchanged T-806 preview/consent flow.

`ankhimate_assemble` accepts existing bone operations plus the three native
operations below. The typed Rust definitions and validation are authoritative.

| Operation | Meaning | Safety boundary |
|---|---|---|
| `attachment.set_region` | Absolute bone-local x/y, rotation in degrees, x/y scale | Exact owned region; preserve texture, UV, pivot, dimensions, sequence |
| `slot.set_bone` | Explicit `keep_world:true` FK Setup association | Single region entry across **all** skins; valid unlocked bone; no constraints/skin-dependent chains; finite invertible frames; four corners within 0.01 units |
| `slot.set_draw_order` | Complete Setup slot-name permutation | Only moved single-region slots; no clipping ranges; no locked moved slot |

The transform limits are absolute position <= 1,000,000, rotation <= 360,000
degrees and scale magnitude 0.0001–100; all numbers must be finite. Scale may
be negative explicitly. Shared slots can receive an exact region transform in
one skin; global reassociation/reordering is refused to protect their siblings.
Fallback entries must be addressed in their actual owning skin.

Use an earlier-created bone by its unique name, never a guessed ID. For example,
create a helper under the torso, associate one region, then place it in that
helper's local frame. Changing Setup can change animated poses. World preservation
means **FK Setup geometry only**, not constraints, every skin or every animation.
The native region has no shear: conversions losing shear are rejected rather
than approximated. Meshes, weights, inverse binds, vertices, triangles, FFD and
other attachments remain read-only. No image or slot creation was added.

A proposal shows exact target, old/new bone, local transforms and order changes.
Approval rechecks revision, locks, mode and references, then replays the native
commands in the existing transaction. One approval, one Undo. Rejection and
validation failures do not mutate the project or discard Redo. The existing
bounded model correction mechanism receives structured errors. No automatic
Save. Undo/Redo remaps recreated bone IDs through native history; SetSlotBone's
stored inverse now observes those remaps as well.

## Manual acceptance pending

Use the isolated Jade Sentinel copy delivered next to `Ankhimate-T807-dev.exe`.
Never use original projects for acceptance.

1. A: ask for a small change to one inventoried **region**. Inspect the proposal,
   approve, verify placement and ordinary manual editing. Reject mesh edits.
2. B: ask for one new helper bone plus association of a single-entry region,
   with explicit FK Setup world preservation. Confirm one coherent proposal.
3. C: Undo/Redo three times, Save As, close/reopen the copy. Inspect unchanged
   image links and clips. Verify one Undo reverses the whole approved lot.
4. D: reject a proposal; verify no changes. Try a locked bone or shared slot;
   expect an explicit refusal with no partial bone creation.

Optional T-806 vision qualification can share this session with a compatible
local model, after preview and explicit image consent. Distinguish native facts,
image observations and artistic guesses. This guide does not claim that Qwen
has been tested.

See [embedded manual](ai/ASSEMBLY.md), [T-806 vision](native-ai-vision.md) and
[AI boundary](native-ai.md).
