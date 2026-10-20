---
title: Graph editor interaction rules
description: The graph editor's selection, handle, snapping, and navigation behavior.
---

# Graph editor interaction rules (T-912)

Four rules and one deferral, each written down because it answers a complaint
filed repeatedly against the established editors' curve editors — unexpected
snapping, curves that cannot be seen, tangents that overshoot without saying so,
and handles that cannot be grabbed back. They are easy to undo by accident in a
later change, so the reasoning lives here rather than only in the diff.

The source of the complaints is a five-year cluster of independent forum threads
against Spine's graph editor. That evidence is **title-level**: the forum is
JS-rendered and resisted extraction, so treat the specific claims as unconfirmed.
What is confirmed is the shape — independent users, several years, one feature,
no resolution. The rules below are written against that shape.

---

## 1. A selected curve is never invisible

**Rule.** The value range is fitted to the *sampled curve*, not to the key
values, at the same 2px step the curve is drawn at.

**Why.** A bezier with steep enough handles travels past the values it
interpolates between. Framing on keys alone drew that overshoot outside the
panel: the curve left the top of the view and nothing said it had. Measuring
what is about to be stroked is the only framing that cannot lie.

**What would break it.** Reverting to `channels.values` for the range because it
is cheaper. It is cheaper, and it is wrong for exactly the curves a graph editor
exists to edit.

## 2. Snapping is opt-in and its state is always visible

**Rule.** Dragging a key snaps to whole frames; **Alt** drags freely. A badge in
the top-left corner reads `snap: frames` or `free`, every frame, whether or not
anything is being dragged.

**Why.** Silent snapping is the specific complaint. A key nudged a third of a
frame springs back to where it was and the editor never says why — which reads
as the drag not working rather than as a mode doing its job.

**What would break it.** Hiding the badge when nothing is selected, or when
snapping is on "because that is the default". A mode is only reasonable-about if
it is legible *before* it surprises you.

## 3. An overshooting tangent is visually distinct from a well-behaved one

**Rule.** When a curve travels more than 10% of its keyed range past its keys,
the panel says `overshoot N%` in the warning colour.

**Why.** Once rule 1 re-frames the view to contain an overshoot, the overshoot
stops being visible *as* an overshoot — it just looks like a curve. But the
difference between a bounce someone authored and a handle dragged too far is the
whole question, and the shape alone no longer answers it.

**Not clamped, deliberately.** An overshoot is a legitimate thing to author; it
is how a bounce is made. Silently flattening one would be worse than the
invisibility this replaced. The editor reports; the animator decides.

**The threshold.** 10% of the keyed range. Below that, an ease that peeks past
its key is ordinary and flagging it would be noise — a warning that fires on
everything is a warning nobody reads.

## 4. A handle is always grabbable

**Rule.** A handle whose true position falls outside the panel is drawn pinned to
the edge, as a hollow ring rather than a filled dot, and stays draggable. Pinning
moves the marker; the stored value is never touched.

**Why.** Rule 1 frames the *sampled curve*, and a cubic does not pass through its
control points — so a handle shaping a legitimate overshoot routinely sits
outside the view. The grab box is gated on the panel rect, so a marker drawn out
there is unreachable: the handle could not be dragged back and the only way out
would be undo. Grabbing a pinned handle and dragging snaps it to the pointer,
which is the recovery gesture.

**What would break it.** Widening the auto-fit to include handle positions
instead. It sounds like the same fix and is worse twice over: the overshoot
percentage in rule 3 is computed from that same range, so the badge would report
a number that is not overshoot, and one handle at four times the span zooms the
view to mostly dead space, flattening every curve in the panel.

## 5. Tangent values can be typed

**T-814 local implementation; PENDING MANUAL REVIEW.**

Select an editable key in the dopesheet or graph. Both panels and the inspector
share one session selection; selecting does not author data. A selected graph key
has an accent ring. Ctrl-click toggles graph keys; the existing dopesheet marquee
remains available. No persistent key IDs or project fields were added.

The inspector's **Bezier tangents** block appears for an entirely Bezier selection
whose keys have predecessors. Easing belongs to the **ending** key of the segment.
Out X/Y and In X/Y are segment-normalized control points, not seconds, degrees,
slopes or world coordinates. X must be finite and in [0,1]; Y must be finite but
may be negative or exceed 1. A flat segment has no value span; changing its Y
handles alone cannot deform its constant value. Broken/aligned/mirrored handle
modes and per-channel normalization are not added.

Values use round-trippable f32 text. Typing changes only a draft. **Apply tangents**
validates all four values, current selection/revision and locks before dispatching
one native `SetInterp` command. **Cancel tangent edits** restores the current
stored values. Apply does not convert Linear/Stepped keys; use an explicit preset
first. The first key has no incoming segment and is not a numeric tangent target.
For multiple selected Bezier keys, Apply explicitly copies all four displayed
handles (initially from the first selected key) to **all** of them in one Undo.
Each Apply is separate from preceding edits. Handle drags merge within their
pointer gesture, but separate gestures have distinct merge groups. Animation
interpolation commands require Animate at the native command boundary.

Selection indices are revision-scoped leases. Changing document/clip, deleting,
Undo/Redo, or an unrecognized external edit clears them rather than guessing a
replacement key. T-814's own tangent applies rebind the same keys; timeline moves
resolve unique destination times after sorting. Colliding/clamped destinations
are refused before dispatch. If a graph point crosses another point, its capture
stops at that reorder; release and start a new drag to continue. This conservative
boundary avoids transferring the gesture to the former index's next occupant.
Combined graph value/time drags retain the existing separate native commands;
T-814 does not redesign their general Undo granularity.

Auto-fit still measures sampled curves, with padding, overshoot badges and pinned
handle markers. Bounds/mapping arithmetic uses f64 to avoid f32 span overflow,
without changing the authored f32 values or the core sampler. Bounds stay fixed
while the primary pointer is held and refit on release; shared horizontal scroll
and zoom are retained. Extremely large finite values/handles can still overflow
the existing core sampler: the graph reports non-finite samples and draws gaps
instead of non-finite geometry or silently rewriting the curve. This is not a
claim of meaningful animation evaluation over the entire f32 range. No new
snapping policy or shortcut: the existing `snap: frames` badge and Alt `free` remain.

Automated command, selection, round-trip and in-memory egui regressions exercise
these paths. No desktop session, screenshot, playback observation or GUI Windows
qualification was performed. T-813 and T-814 remain **PENDING MANUAL REVIEW**.
T-704's broader normalize/tie-mode/graph marquee requirements remain deferred.
