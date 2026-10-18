---
title: PSD import
description: Import layered artwork and use tags to infer useful rig structure.
---

# PSD import

A layered PSD is already a rig nobody has told the computer about. The group
structure is the hierarchy, the layer bounds are the placement, and the stacking
order is the draw order. Ankhimate reads that structure out rather than asking
you to rebuild it by hand.

Everything below is driven by **layer names**. A name is the one thing every art
tool round-trips, and the one thing an artist can fix without leaving Photoshop.

## The mapping

| In the PSD | Becomes |
|---|---|
| layer group | a bone; nested groups nest |
| image layer | a slot plus a region attachment, sized and placed from the layer bounds |
| layer named `$pivot` | its group's bone origin, instead of the group's bounding-box centre |
| group named `$ik <name>` | a bone, plus an IK constraint named `<name>` over the chain inside it |
| top-level group `@skin:<name>` | its layers land in skin `<name>` rather than the default skin |

Draw order follows the PSD's own layer order, so what you see in Photoshop is
what you see in the viewport.

### `$pivot`

Without it, a group's bone sits at the centre of the group's bounds — fine for a
head, wrong for a shoulder. Add a small layer named `$pivot` inside the group and
put it where the joint is; its centre becomes the bone origin. The layer itself
is not imported.

### `$ik `

A group named `$ik leg` becomes a bone called `leg` and an IK constraint called
`leg` over the bones beneath it, with a target bone created at the chain's tip —
the handle you actually drag.

The chain is the **single-file** run of descendants. IK solves a line of bones,
so if a group branches, the first branch is taken and the rest is left to you.
Guessing which fork you meant would be worse than stopping.

A group with fewer than two bones under it is reported as skipped rather than
producing a constraint that cannot solve.

### `@skin:`

A top-level group named `@skin:winter` puts everything inside it into a skin
called `winter`. Its layers hang from the *parent* group's bone, so alternate art
sits where the base art sits instead of under a bone that only exists in one
outfit.

## Options

| Option | What it does |
|---|---|
| Scale | World units per PSD pixel. A 2048px character at `0.5` becomes 1024 units tall. |
| Skip hidden layers | On by default. A hidden layer is usually a reference sketch or an alternate the artist did not delete. |
| Flatten group | Composites a group into one attachment instead of a bone with children. A face that never articulates does not need eleven bones. |
| Replace the open project | Off merges the imported rig under a bone of its own, keeping existing bones and animations. |

Per-layer tick boxes control what is imported. Ticking a group ticks everything
inside it.

## Coordinates

PSD measures from the top-left with Y down. Ankhimate measures from the canvas
centre with Y up (PLAN §2.2). The conversion happens in one place, `layer_center`
in `formats/src/psd.rs`, and is covered by unit tests — a second copy of it would
be a second chance to disagree about which way is up.

Layer bounds in the file are **inclusive** on the right and bottom edges, so the
width is `right - left + 1`. Off by one there makes every attachment a pixel
short and every pivot half a pixel out.

## Re-import existing images (T-302)

Use the existing **File > Import PSD** preview (also available from Assets and a
single PSD drop), then choose **Update existing PSD images (keep rig)**. Read the
change report and select **Update images**, or Cancel. This is distinct from
Replace (new rig, no old animation) and Merge (additional rig).

The preview does not mutate the document. A confirmed update is one Setup-mode
Undo/Redo command. The document revision/snapshot must still match the preview;
a newer document cancels it. Redo retains pixels and geometry in memory and does
not read the PSD again. Existing texture caches are invalidated on update/Undo/Redo.

| Source change | Behavior |
|---|---|
| Unique exact raw layer path, same dimensions, different RGBA | Replace that image's bytes in its existing AssetDb entry; retain IDs and rig |
| Same decoded RGBA/dimensions, even different PNG encoding | Unchanged; no redundant update/Undo step |
| Dimensions change, one simple region owns the image | Scale region width/height by new/old image dimensions; preserve authored pixel scale, pivot, offset, rotation, local scale and UVs |
| Same leaf name in different nested groups | Distinct full paths; never match by leaf name or pixel resemblance |
| Rename, reparent or tag change affecting a raw path | Added + missing; no guessed reassignment |
| Added layer | Report for manual import; do not create bones/slots automatically |
| Missing layer | Keep existing image, attachment, bone, slot, keys and constraints |
| Duplicate source/document paths or literal path separators in names | Refuse the entire update |
| Changed mesh/FFD, linked/shared image, sequence, flattened group image, unreferenced asset | Refuse the entire update; no approximation or mesh-to-region conversion |
| Changed hidden layer/group, cleared/unreadable image, inconsistent image dimensions | Refuse the entire update and preserve current art |

Paths are the existing `psd_layer_paths` mapping from **asset name** to raw layer
path, including tags. They are scoped to the selected PSD and the stored mapping;
there is no persistent Photoshop file/layer UUID. Multiple provenance entries for
the same path are a conflict, including after importing two similar PSDs.
The report distinguishes unchanged/changed image candidates, added, actually missing,
and refused operations; markers/empty/non-image candidates have separate notes.
Any conflict blocks all changes, including otherwise safe images in the batch.

Reimport preserves bones, parents/setup, slots/draw order, skins and unrelated
attachments, animation curves, constraints and FFD keys. It does **not** reposition
a manually rigged attachment from the PSD's current canvas coordinates or apply
source visibility. Original source bounding boxes are not stored, so unchanged means
identical decoded pixels and dimensions, not proof that a layer never moved in Photoshop.
Existing authored pivots remain fixed when size changes.

For an open native author file, review/apply first checks that its supplied data can
survive the current loaded-model conversion, with absolute numeric tolerance 0.0001.
Unknown/dropped fields, unresolved references, missing source/assets or incompatible
ordering cause a conservative refusal. This can reject noncanonical but otherwise
readable files; it never rewrites them to make the check pass. Opaque export preset
JSON remains supported. See [format limitations](format-spec.md).
This gate cannot recover unknown fields already lost by an earlier save through
another workflow. Unsaved documents contain only the current model; callers of the
headless preparation API are responsible for checking any original file first.

Save As/reopen uses the established [safe-save protocol](project-save-safety.md).
Saving to a fresh folder is supported; no source PSD is ever modified. Reimport
itself performs no file writes. There is no new atomicity/durability guarantee.

The targeted automated evidence uses modified copies of `nested_groups.psd`, its
ambiguous-sibling companion, animated rigs, three Undo/Redo cycles, Save As/reopen,
texture upload bytes and exports. No interactive GUI or Photoshop-effects validation
is claimed by this checkpoint. General complex-attachment refresh and automatic
add/delete reconciliation remain outside the completed subset, so T-302 stays partial.

## Known limits

- **Negative layer bounds.** Art dragged off the canvas edge makes `psd 0.3.5`
  index out of bounds and panic. The call is caught and the layer is reported as
  skipped, so the import survives, but that layer does not come in. Crop or move
  the art inside the canvas to import it.
- **Layer effects, adjustment layers, masks and blend modes** are not read. What
  you get is the layer's own pixels.
- **Rotation.** A PSD has none to read, so every bone comes in unrotated.
- Historical checked-in PSD fixtures now cover flat, simple and nested imports.
  T-302's new tests extend these with modified RAW-channel copies; Photoshop effects,
  adjustments and masks remain outside this qualification.
