# Cross-project rig transfer — T-819 local candidate

**LOCAL_CANDIDATE / PENDING_MANUAL**. Internal version stays 0.2.0-alpha.1.
Same-document Copy/Paste and ordinary File/Open are unchanged.

## Procedure

1. Open a disposable destination copy in **Setup** mode.
2. Choose **File → Import from Ankhimate Project…** and a source `.ankh`
   (supported readable/compact author JSON profiles are also accepted).
3. The source loads separately, read-only. The destination remains open and dirty
   changes are retained. Loading or selecting a file does not import anything.
4. Select exactly one source subtree root. The list shows each bone's parent.
5. Select a destination parent or **New root — preserve LOCAL values**.
6. Click **Prepare transfer (no changes)**. Read counts, limitations and mappings.
   Changed names are flagged; expand image/bone/slot/constraint/clip details.
7. Click **Confirm reviewed transfer**, or **Cancel transfer** / close the window.
8. Inspect the new root, Undo/Redo as desired, then Save As to a fresh destination.

No native source file is saved, no OS clipboard is populated, and no network is
used. Do not confuse this with importing/replacing the complete open document.

## Supported bounded contract

- One subtree (up to 512 bones and 1024 slots).
- Source **default-skin** region attachments, including image sequences. They map
  explicitly into the destination default skin; partial empty alternate skins
  elsewhere do not invent overrides. Pivot, UV, region and slot properties copy
  unchanged apart from logical texture names and native IDs.
- Required image bytes and dimensions, up to 64 MiB total selected encoded bytes.
  In-memory image decoding/integrity uses the existing validators. No empty
  substitute is created. Optional source filesystem provenance is not imported.
- Wholly internal IK and Transform constraints, preserving their relative order.
- Bone translate X/Y, rotate, scale X/Y and shear X/Y timelines. Times in seconds,
  values, independent axes and Linear/Stepped/Bezier handles (including Y overshoot)
  remain unchanged. Imported clips receive new distinct names; looping, duration,
  editor markers and selected-bone time offsets are retained/remapped.
- New slots append after the destination's existing slots, in their source
  relative setup draw order. Existing destination slots/order are untouched.

**Coordinates:** local setup transforms and animation offsets are copied exactly.
Source-world placement, parent motion and absolute animated world poses under a
different ancestor are not preserved. No hidden matrix inversion/compensation or
universal retargeting is performed. Choosing a source limb below another bone
excludes that ancestor under this explicit local-coordinate contract.

## Images and collisions

Required regions/sequence frames are looked up by unique logical asset names.
Missing/ambiguous/corrupt bytes or incompatible dimensions refuse preparation.
Byte-for-byte equal images with equal dimensions may reuse an existing destination
asset even if its name differs; the choice is deterministic (lexicographic name).
Different bytes never overwrite an existing asset: native unique naming (`_2`,
`_3`, …) is previewed and every transferred texture/sequence reference is remapped.
Existing attachments are never rewritten. Bones, slots, constraints and clips
likewise show their actual native new names; homonymous clips are never merged.

After Save/Save As, the normal content-addressed destination `.assets` sidecar
contains the required bytes. The source project/sidecar is no longer a dependency.
Keep the destination `.ankh` and its own `.assets` together when moving it.

## Refusals and deliberate exclusions

Selected mesh/weighted/linked, FFD, clipping, path, physics or alternate-skin
overrides/gating are refused. External constraints touching the selection and
relevant slot/constraint/deform/draw-order tracks refuse. Events in a clip whose
bone keys would transfer refuse rather than silently disappear. Invalid source
load reports, source/destination integrity errors, ambiguous logical image names,
missing roots/parents and unsupported modes refuse before document publication.

Unrelated source objects/clips, editor folders, export presets and PSD reimport
provenance are outside this explicitly reviewed selection. This is not complete
project merging, animated world-pose retargeting or an arbitrary rig converter.
Existing same-document PasteBones retains its older broader policies.

## Transaction, staleness and history

Preparation runs native AddAssets/PasteBones on a cloned destination, followed
by validation. Confirmation uses one existing ReplaceDocument native command:
the live document never exposes a half-imported asset or rig. Its exact inactive
document is retained for Undo/Redo; Redo does not reread the source. One transfer
is one Undo step and does not merge into a drag or previous paste.

The plan checks the T-818 document identity, existing revision, metadata and shared
native snapshot. Source project bytes and freshly loaded asset data are rechecked
at confirmation. Changed destination/source, deleted/locked parent, pending pose
preview or Animate mode cannot silently apply. Old source readers are discarded
on project reset; loading completion cannot replace a newer document. This is
not a cross-process source filesystem lock: the approved immutable bytes are
the data applied after the final check.

Cancel/error leaves document, assets, history/Redo, saved snapshot and selection
unchanged. Successful transfer moves the native revision and invalidates T-818
diagnostics normally. New root selection follows existing session semantics;
Undo may clear it rather than selecting a reused object on Redo. T-813 Save,
autosave/recovery, unknown-field/numeric guards and publication guarantees remain
unchanged; a successful in-memory transfer is not a completed disk save.

Source loading uses a background reader. Preparation/final source verification
remain synchronous; full source/destination snapshots and a staged document copy
consume memory proportional to the projects. The selected-content limits are
not a hostile-input decompression/decoder or universal UI-latency guarantee.
No renderer/runtime format or adapter qualification changes.

## Deferred manual qualification

Use `outputs/T819-Cross-Project-Transfer/MANUAL-CHECKLIST.md` and retained cumulative
T-813–T-818 checklists. Technical fixtures are generated from native APIs, not
artistically qualified projects. No desktop GUI, model or browser run was made
for this milestone. T-909 remains PARTIAL for advanced types and workflows.
