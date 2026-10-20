# T820 — contract fixed before implementation

Single selected scalar junction B, compatible Bezier segments A→B and B→C.
Supported graph channels: bone scalar transforms and slot alpha. No new keys.
For rotation, reject either raw adjacent delta of magnitude >=180 degrees;
shortest-arc discontinuities need separate design, not sampler changes.

Let dL=tB-tA, vL=B-A, dR=tC-tB, vR=C-B. In B coordinates:
incoming=((B.in.x-1)*dL,(B.in.y-1)*vL);
outgoing=(C.out.x*dR,C.out.y*vR).
All geometry uses f64 seconds and native scalar value units, Euclidean length
sqrt(time²+value²), independent of viewport/zoom. Changing units changes metric;
no claim of screen-length equality under anisotropic zoom.

Broken preserves historical segment-local four-field editing. Choosing a mode
does not modify coordinates or history. Aligned/Mirrored affect the NEXT edit.
Aligned: driver defines direction; opposite vector is negative driver normalized
times the opposite length captured at gesture/draft start. Mirrored: negative
driver exactly in this metric. Other two handles and all keys stay unchanged.
Zero driver: Mirrored yields zero opposite; Aligned refuses unless opposite was
also zero. Flat opposite segment accepts only a horizontal target; retain its
irrelevant normalized Y. Both-flat supports horizontal alignment/mirroring.

Convert vectors back using each segment's OWN duration and delta. Reject rather
than clamp an opposite X outside [0,1], nonfinite/f32-overflow coordinates,
zero/unsorted/invalid times, incompatible interpolation, or nonrepresentable
flat vertical vector. Preserve overshoot Y. Validate f32-rounded geometric result
with relative tolerance 2e-5 (unit floor for near-zero comparisons).

Session mode defaults Broken, bound to the single selected junction and exact
revision/data/document identity; reset on selection/clip/document/external edit,
Undo/Redo or reopen. Own linked edits refresh that lease. Multi-selection uses
existing Broken fields only. No persisted mode and no retroactive alignment.

Linked numeric fields are explicitly relabeled Incoming X/Y and Outgoing X/Y
around B. A driver selector enables only its pair; the opposite pair previews
the computed result read-only. Changing driver discards the draft to avoid
contradictory instructions. Apply validates once; Cancel discards draft only.
Broken retains original Out/In fields of the selected ending segment.

Drag: central B is selected; only B.in and C.out can participate while linked.
Capture stable initial geometry/selection/revision at start; use latest guarded
data for command before, fixed baseline for Aligned opposite length. Native
single-timeline swap replaces both Interps without intermediate publication.
Unique per-pointer token merges one gesture; numeric applies never merge.
Invalidation stops the gesture without reattaching it to reused key indices.
Setup/locked/stale/missing selection refuses before dispatch, retaining Redo.
No-op creates no command. All file persistence uses existing native formats.


## Native implementation and qualification

[Geometry](../document/src/tangent_linking.rs), [command](../document/src/commands/tangent_cmds.rs),
[session leases](../editor/src/tangent_linking.rs), [inspector](../editor/src/ui/inspector/tangents.rs)
and [graph gestures](../editor/src/ui/timeline/graph.rs). No-op returns no command.
The command stores one target timeline before/after, declares Animate, and
coalesces only contiguous edits to the same target with an identical gesture token.
There is no change to the generic History or existing SetInterp callers.

Selection/data captures reuse T814 key leases/KeyClip and T818 document identity.
Source author fields and sampler are unchanged; edited coordinates persist through
the existing Save. Manual qualification is deferred, not certified by headless tests.
