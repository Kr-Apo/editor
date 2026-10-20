---
title: Safe mesh retracing
description: Retrace compatible owned meshes without silently clearing weights or deformations.
---

# Safe mesh retracing (T-823 local candidate)

T-402 remains PARTIAL. All cumulative Windows/manual review is deferred.
The existing alpha tracer, refinement and triangulation are reused unchanged;
only the visibility of its existing affine UV-frame helper is widened internally.

## Workflow

On a project copy, convert a simple Region to Mesh if needed, select the attachment
in Setup and choose Trace from image…. Review the outline/settings and current/new
vertex and triangle counts, weight entries, binds, FFD and linked status. A valid
operation requires explicit OK. Cancel changes no document/history. A refusal is
visible and has no Apply path. The existing modal scrolls when its content is tall.

## Weight transfer contract

New points and old setup_vertices share the slot-bone-local Setup coordinate frame.
For each new point choose minimum squared Euclidean distance, calculated in f64
from native f32 coordinates; exact ties choose lowest old vertex index. Copy the
complete source row verbatim, retaining BoneIds, weights and order. No arbitrary
influences, pruning or normalization. Preview discloses source vertices not sampled;
nearest-neighbour does NOT promise identical artistic deformation. Every source
BoneId must occur in at least one new row, including existing zero-weight entries,
or the operation refuses. Another mesh sampling the same image is not edited.

The bounded first scope uses at most four influences per source row (existing
painting default, not a native format ceiling) and 16,000,000 old/new point-pair
comparisons. Exceeding a limit refuses without reducing data. Empty rows in a
partially weighted mesh keep native rigid-fallback semantics. All-empty weighted
tables, malformed/duplicate influences, missing bones and unnormalized values refuse.

## Binds and geometry

Vertices are not moved into arbitrary bone coordinates. Native binds equal
inverse(Setup world of influence bone) * Setup world of slot bone. They must be
finite, invertible, complete and agree exactly with the current native convention
(no tolerance window for noncanonical caches). Existing matrices are retained verbatim; no rebind during
Apply. Save/Load reconstructs the derived nonserialized cache by the unchanged
native rebind pass. Noncanonical caches refuse rather than drift at reload.

The existing trace projector recovers one affine UV-to-local frame. Weighted
sources must agree with it at every point within 0.0001 local unit; non-affine
sources refuse instead of inventing a different projector. Rigid sources retain
the historical projection behavior. Old/new finite vertex/UV arrays and valid,
nondegenerate triangle indices are required. Pinned edges are refused, not left
pointing at new indices or silently deleted. No evaluator/skinning/render change.

## Deformations, links and skins

Any inline ffd_keyframes or matching Timeline::Deform blocks retracing, including
empty reset keys/tracks. Weighted FFD is influence-sized; rigid FFD vertex-sized.
No offsets are remapped, deleted or zeroed. Linked targets and shared linked
sources refuse; unpinned links are conservatively checked across owning/fallback
skins. Skin overrides sharing the same slot/attachment address also refuse.
Image sequences and ambiguous/missing targets refuse. Unrelated FFD and meshes
remain exact. Source images and animation/bone/slot identities are not changed.

## History and stale state

Preparation is isolated and the full result is validated before dispatch. One
native TraceMesh command stores exact before/after attachments and does not merge.
One OK means one Undo. Three-cycle tests compare full document snapshots, including
binds. Redo uses stored data, without retracing or image I/O. An exact structural
no-op does not enter history; refusals and Cancel retain Redo. No drag is merged.

The modal captures document identity/revision/exact snapshot, selected slots,
focused selection and active skin. Setup, no pending pose previews, slot/influence
locks and native ownership/references are rechecked. A stale preview refuses;
reopen it to continue. Success invalidates old geometry selections/drafts using
T-815 leases and normal native revision updates invalidate T-818 diagnostics.
Existing paint commands and Save/Save As/recovery writers are unchanged.

## Qualification and open scope

Headless tests cover multi-bone rows, ties, malformed/over-budget data, bind/FFD/
link refusals, native geometry, retained-vertex animation samples, exact history,
Save/Load, export planning, painting and real egui in-memory OK/Cancel clicks.
The release benchmark measures outline+geometry/triangulation separately from
safe preparation/remapping on a synthetic 512x512 image and 1024-point source;
there is no fragile CI timing assertion. Optional refinement, every-project and
artistically identical deformation performance are not certified. Human Windows,
DPI/theme and visual/painting checks remain in the delivery checklist. No public
operation or new format/runtime capability. T-402 remains PARTIAL.
