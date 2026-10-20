---
title: Explicit PSD path-change resolution
description: Update one renamed or moved simple-region image without rebuilding its rig.
---

# Explicit PSD path-change resolution (T-822)

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**, not a public release.
T-302 remains PARTIAL. See [PSD import](psd-import.md) for the unchanged exact-path
refresh, Replace/Merge options and [safe save](project-save-safety.md).

## Workflow and consent

Load the modified PSD through the existing File Import, Assets preview or single
PSD drop. Choose Update existing PSD images (keep rig), then Resolve path change….
Explicitly select one old missing image and one new added full raw path. Prepare
path mapping reads and validates; it does not edit the document. Review old/new
paths, logical asset, owner skin/slot/region, image and region dimensions, same or
different decoded RGBA, provenance and the full exact-path image batch. No pairing
is inferred from a short name, visual resemblance, pixels or a single candidate.

Apply path mapping and image batch confirms the whole disclosed plan. All existing
exact-path changes in the report are included; they cannot be silently omitted.
Any conflict blocks everything. Cancel path mapping clears the draft only; whole
preview Cancel/Escape preserves the document, selection and Redo. Without a mapping,
the original Update images action retains its exact-path behavior.

## Identity, pixels and dimensions

The existing AssetId and asset name remain; no bones, slots, attachments, clips,
tracks, constraints or skins are recreated/renamed. Same decoded RGBA and dimensions
retain the exact old encoded bytes. A path-only change still makes one Undo entry.
Different pixels replace the existing asset bytes and invalidate editor caches.
Dimension changes run the same T-302 region width/height × new/old pixel dimensions
code. Authored pivot, offset, rotation, scale and UV stay fixed. PSD canvas movement
does not move the rig. All unrelated meshes, weights, inverse binds, FFD, keys and
Bezier handles remain exact. Changed target FFD/topology is not retargeted.

## Fail-closed boundary

Only one missing region image is resolved per operation. Other missing image
provenance refuses the operation: the legacy global asset-name -> raw-path map
contains no PSD file/layer UUID, so affiliation with several incomplete sources
cannot be established safely. Duplicate paths, separator ambiguity, claimed new
paths, missing assets, protected/invalid/stale references, skin overrides, shared
art, meshes/linked sources, target FFD, sequences, flattening, hidden or undecodable
art and changed processing/inheritance/legacy tags refuse. Equivalent tags are
compared conservatively as literal bracket sequences; not all semantically equal
rewrites are accepted. Existing global exact-path refusals are retained.

A human selects the source and the association. Two different PSDs with identical
inventories have no persistent native identity; this workflow does not certify
Photoshop identity. It never chooses the first candidate or tries to reconstruct
the PSD history. Bulk group renames producing multiple missing images are excluded.

## Transaction and persistence

The plan stores an exact document snapshot and source bytes; its session lease
also stores document identity, revision and selected source path. Apply verifies
file bytes again, current mode, owner locks and unchanged selections. App lifecycle
checks and the conservative original-author-file guard are unchanged. A stale plan
or invalid batch does not dispatch and does not consume Redo. One confirmed native
ReimportPsd command owns before/after documents and makes one Undo step. Three-cycle
headless tests compare complete states. Redo never reads the source PSD.

Save/Load uses existing writers, no format migration. The new path persists and
the next exact-path refresh recognizes it. Reimport itself writes nothing; source
PSDs are never edited. There is no new atomicity/durability promise, nor a filesystem
reservation after the final source read. Diagnostics invalidate by native revision.

## Qualification

Synthetic real RAW-channel PSDs exercise rename, nested move, 32×32 -> 64×48 resize,
duplicates, tags, flattening and sequence refusals. Native and in-memory egui tests
cover selection/Prepare/Apply/Cancel, cache bytes, stale plans, Save/Load and native
v2 export. Interactive Windows, Photoshop-effects and artistic checks are deferred
to the cumulative 26-step delivery checklist. No inference or GPU test is claimed.
