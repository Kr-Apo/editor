# Safe FFD Key Clipboard (T-834)

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** in the bounded scope below. T-209/T-404 remain PARTIAL. No physical Windows GUI, artist, inference or new runtime qualification.

## Copy and explicit destination

In Animate, choose a clip and active skin displaying an owned mesh. Click one existing **deform** key or Ctrl-click/marquee several keys **from that same row only**. Right-click → **Copy FFD Keys**. Copy changes Session clipboard only, never document/dirty state/revision/Undo/Redo. On refusal the previous clipboard remains intact.

Move the playhead and explicitly select one or more keys in the destination **deform** row, then right-click → **Paste FFD Keys…**. Review the identified skin/slot/attachment, key count, result range and fixed duration. Apply once, or Cancel/Escape. An acknowledged warning is required for keys beyond duration. No selected row means refusal; a selected bone/name/count is never a destination. An existing valid nonempty Deform row is required; creating empty rows is deferred.

Same-clip and inter-clip paste are supported **within this document and the exact same owned mesh**. Scalar Copy/Paste, mirrored bone offsets, pose clipboard, key shortcuts and scalar row-order mapping keep their existing behavior. New FFD actions are dedicated context entries, not a new keyboard override. Single-key T-821 drag/time/delete/interpolation remains single-key even when FFD groups are available for clipboard.

## Owned native payload and compatibility

One immutable Clipboard::Deform payload owns Key<Vec<Vec2>> values and original Interp. No live key indices, source clip ID, image bytes or borrowed mesh pointers survive Copy. Rigid vectors are per setup vertex; weighted vectors are per influence in existing flattened vertex/influence order. Native empty-vector zero values stay empty. No normalization, rounding, text serialization round trip or fabricated scalar average.

Identity = document identity + exact TimelineAddr::Deform(slot,attachment,owner SkinId). The owning entry must be globally unique; active skin may resolve to this exact default owner. Different owner/slot/name is refused even if vertex count or image looks identical. A mesh has no persistent heap-generation UUID: replacing the same owner entry with structurally identical native data is semantically indistinguishable. This is an entry-identity contract, not a claim of heap-allocation identity.

Full structural equality, not a hash/count heuristic: ordered setup vertices/UVs, triangle and pinned-edge indices/order, influence rows/bone IDs/order/weights, inverse bind matrices, inline FFD, texture reference, slot metadata, skin ownership lists/default skin and relevant FK Setup ancestry/transforms/inheritance. Any changed value with unproven neutral impact refuses. Native validation checks key sizes, finite values/handles/times, ordered tracks, topology, normalized valid influences and usable bind coverage. Existing T-823 triangle validation is reused. Nothing is repaired.

Linked destinations, shared sources referenced by pinned or unpinned links, multiple skin overrides and sequence meshes refuse. Native Copy/Paste validates even if the loader accepted stale topology. Loader migration/neutralization is unchanged and remains T-404 debt. Cross-project payloads refuse even if the new document is a clone with identical names/data. After deleting only the source clip, the owned payload can still paste if its exact mesh exists unchanged and the destination validates.

## Time, curves and collisions

First copied source time anchors the block. For 0.25,0.75,1.25 s at playhead2, result is2,2.5,3 s. Relative calculation uses f64 from native f32 source times; checked conversion to f32 rejects nonfinite/overflow/underflow/order collapse. Values and interpolation/normalized Bezier handles (including overshoot Y) remain native and unchanged. Equivalent-time sample tests reuse the existing vector sampler; it is not modified.

Sorted insertion is atomic. Any equality/collision among inserted times or with existing destination keys refuses the entire block; no replacement, deletion, fusion or clamping. This deliberately differs from scalar PasteKeys exact-time replacement. Duration stays unchanged; keys beyond it are retained, counted and need explicit UI consent. A successful nonempty collision-free paste always adds keys; empty/malformed payloads refuse, so there is no no-op history entry.

## Preparation, history and persistence

One explicit Paste action prepares the preview. No document dispatch/Undo for preview. Immutable clipboard Arc identity, document identity/revision, clip, exact selected target refs/owner, active skin, playhead, mode and slot/influence locks form a transient lease. Changes disable Apply; cancel/reopen to prepare again. Apply rechecks complete native owner/target data and exact payload before one native PasteDeformKeys command. It wraps existing targeted EditFfdKey snapshot application, with fixed duration, no merge and Animate-mode enforcement. Locks remain Session-side because Document has no lock state.

One paste = one Undo. Three full snapshot cycles compare clips, vectors/curves, meshes, weights/binds, skins and asset bytes. Undo/Redo does not read the clipboard; clearing it before Redo is safe. Save As/reopen retains pasted native data and existing sidecar/save guarantees. No new crash-durability claim.

Preparation/validation are synchronous explicit operations. Payload construction copies selected vectors once; borrowed validation avoids copying all source vectors for row eligibility. Sheet grouping reuses the shared selection storage and borrowed equality; no permanent mesh/index cache. The modal never captures geometry/payload per repaint. The preexisting single-key edit menu retains its own edit capture while open. CPU observations have no fragile thresholds and do not certify Windows latency.

## Unchanged contracts and remaining work

No .ankh/runtime version, evaluator, sampler, renderer or runtime change. T-830 global and T-833 seven-channel bone offset remain unchanged; FFD never joins selected-bone retiming. PathPosition remains unchanged and explicitly refused by incompatible runtime exports. Minimum-reader/exchange release policy still needs review before a future alpha.

Out of scope: cross-project/cross-mesh FFD retargeting, linked/shared ownership, topology repair/remapping, generic constraint/draw-order/event clipboard, empty-row creation, grouped FFD editing and a vector Graph editor. The T-832 101 task statuses remain62 COMPLETE/33 PARTIAL/1 ABSENT/3 SUPERSEDED/2 DEFERRED. Its352 controls plus28 T-833 and37 T-834 controls remain pending.
