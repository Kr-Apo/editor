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
handles alone cannot deform its constant value. T-814 did not add linked modes or per-channel normalization. T-820 now adds the
bounded session linking described below; normalization remains deferred.

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
T-704's normalized overlay remains deferred; T-825 adds the local marquee below. T-820's scoped
linked editing is described below and is not a broader GUI qualification.

## 6. Session linking at a central key (T-820)

Select one central scalar key B with Bezier segments A→B and B→C. Choose text
buttons **Broken / Aligned / Mirrored** in the existing tangents inspector.
Choosing a mode does not modify handles. The next valid edit applies it.
Aligned retains the opposite length captured at gesture/draft start; Mirrored
uses equal opposite vectors in seconds/native-value Euclidean geometry, not
pixels or two unrelated normalized segments. See [full contract](bezier-tangent-linking.md).

B.in is on the preceding segment; B.out is actually stored in C.interp.out.
In linked mode, numeric fields explicitly show Incoming and Outgoing around B.
Choose a Driver; only its two fields are editable. The opposite fields preview
the calculated result read-only. Changing driver abandons the draft. Apply
authors both segments in one isolated native operation; Cancel authors nothing.
Broken keeps the historical four Out/In fields on one segment and multi-key Apply.

Only B's incoming and outgoing graph handles are draggable while linked. A
continuous gesture keeps its initial opposite length, publishes both Interps
together, and merges under one unique token. A stale capture stops; restart the
gesture after Undo, external edits, clip/document changes or sorting. Separate
drags and numeric applies are separate Undo entries. No-op edits do not dispatch.

Mode is session-only and resets to Broken when its selection/data/revision/
document/clip lease is invalidated, on Undo/Redo and reopen. Own linked edits
rebind the same selection and preserve the mode. It is not stored per key and
does not turn existing animation into a continuously constrained runtime curve.

End keys, multi-selection, Linear/Stepped neighbors, impossible normalized X,
nonfinite/overflow values and flat segments needing a vertical handle refuse
linked editing; there is no implicit conversion/clamping. Both-flat horizontal
joins are supported. Overshoot Y is preserved. Rotation joins spanning a raw
adjacent delta >=180 degrees are refused because raw graph geometry and shortest
arc sampling can disagree. Core sampling remains unchanged, including its extreme
overflow/gap limitation. Setup and locked/missing targets cannot author changes.

All T-820 desktop Windows checks remain **PENDING MANUAL REVIEW**. Existing
automated egui pointer/text tests are headless state/command evidence only.

## 7. Rectangular key selection (T-825)

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for existing scalar key points.
T-704/T-912 remain PARTIAL; normalized overlay and broader GUI acceptance remain.

Press on empty curve-plot space and drag past egui's normal drag threshold. The
rectangle works in all four directions and uses the existing theme selection
fill/border. Only visible finite key centres inside the plot/rectangle are
candidates, including its boundary. A segment crossing the rectangle and Bezier
handles are not selected. Collapsed or non-soloed rows, offscreen keys, events,
FFD, constraints and other nonnumeric tracks are excluded. Bone scalar channels
and slot alpha retain their existing raw display/storage conventions, including
rotation's possible difference from shortest-arc sampling. No normalized display
or new interpretation is introduced.

- Plain drag replaces selection on release. A positive-area empty rectangle clears.
- **Ctrl+drag** adds (union), matching the Dopesheet's existing marquee convention.
  Empty additive rectangles preserve selection. Ctrl is captured at press.
- **Ctrl-click** keeps its existing per-key toggle, including removal. There is
  no subtractive marquee, new Shift selection rule or implicit Cmd remapping.
- Plain empty click, subthreshold jitter and zero-area gestures keep selection.
- Key and Bezier-handle widgets win over the background. Alt remains key-drag
  free snapping; empty Alt drags do not start a marquee. Nonprimary buttons and
  wheel/pinch stay reserved for existing navigation/menus. The existing wheel/
  pinch/shared scrollbar navigation is unchanged; graph-specific middle-button
  pan is not implemented by this task (the Dopesheet has its own middle pan).

Selection does not change while drawing. **Escape**, loss of window focus,
PointerGone, leaving the plot (including release outside), wheel/pinch, another
button, hidden-graph frame gaps or a changed context cancels. Re-entering while
still held never restarts a canceled gesture. Release and start again. The
rectangle is clipped to the plot; this conservative V1 does not auto-scroll.

A transient egui-memory gesture captures document identity/revision, active clip,
exact animation, typed key addresses, visible positions/mapping and both key and
object selection at press. The rectangle appears only after the native threshold.
Every subsequent update revalidates that baseline. Changes to rows/folding/solo,
zoom/pan/framing, keys, mode, clip, object selection, Undo/Redo or document cancel
instead of mapping old indices to new keys. Full typed equality, not positions,
times or address hashes alone, resolves duplicate screen positions. No new
persistent key IDs, author fields or second KeySelection.

Commit goes through native KeySelection. The same keys appear in Dopesheet and
Properties. T-820 Aligned/Mirrored require one compatible junction and reset to
Broken after a changed multi-selection; no first-key-only linked edit. Old numeric
drafts are invalidated. Existing multi-key Broken Apply can copy all four handles
to an entirely compatible selected set; selecting first/non-Bezier keys still
refuses numeric tangent editing. Mixed existing FFD + scalar Ctrl-unions refuse
without changing the FFD selection. FFD remains Dopesheet-only.

Selection creates no Undo entry and changes neither dirty content, revision,
curves, tangents, assets nor export bytes. Existing multi-selection operations
retain their own native guards; this feature does not add collective key/handle
movement. Preferences/selection remain transient and do not enter .ankh.

CPU measurements distinguish display, rectangle search and native selection
validation. Large selections retain pre-existing quadratic KeyClip/selection
paths: a 10,000-key debug native set took roughly 0.57 s locally. This is not a
new optimization or real-time guarantee; desktop responsiveness is unqualified.
All Windows/light-dark/artistic checks remain **PENDING MANUAL REVIEW** together
with T-813–T-824. No desktop GUI, GPU benchmark or inference was run.

## 8. Normalized comparison view (T-826)

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for Normalize View Only (Option B).
T-704/T-912 remain PARTIAL. Earlier numbered sections describe their original Raw
checkpoint behavior; this section adds the optional projection without replacing
Raw. The Graph header has a **Normalize** toggle; off is Raw and the default.

For each displayed editable scalar channel, scan finite key values paired with
finite key times over the complete active track, independent of horizontal zoom
and pan. Finite negative/out-of-duration times participate. Compute min/max and
`(value - min) / (max - min)` in f64 after converting stored f32 values, before any
subtraction. Exact constant/single-key tracks appear flat at 0.5; no epsilon range
or artificial amplitude. Empty/all-invalid tracks do not acquire a fabricated
range. Existing sampling gaps remain gaps. Nonfinite samples are not repaired.

The shared comparison frame is relative -0.1 to 1.1 with ticks 0, 0.5 and 1.
It is fixed rather than Raw's sampled auto-fit: panning does not change a channel's
reference. Bezier overshoots can leave 0..1 and the frame; the existing geometric
safety bound and plot clipping apply, with an outside-frame notice. No clamp to
0..1 and no sampler changes. Rotation retains native stored angle conventions,
including Raw keys differing from shortest-arc sampled paths. Translation,
rotation/shear degrees, scale factors and slot alpha keep their native meaning.

Property/axis colors are unchanged. Hover **Normalize: relative · View Only · Raw
to edit** for target, property, units, native min/max, key count and constant
annotation. Relative 0.5 is not an authored 0.5 degree/unit. Key centres, sampled
segments, click hitboxes and T-825 marquee candidates use the same transformation.
Coincident points retain typed identities; rectangle selection can select them
all. Click/Ctrl-click and replace/Ctrl-union marquee retain shared Dopesheet
selection semantics and visible-point exclusions. Switching mode cancels stale
marquee/author captures even when the two numeric bounds happen to match; holding
the pointer and switching back cannot resume the old edit.

Normalize hides handles and blocks graph key value/time drags and graph-local K
insertion. **Switch to Raw for graph editing.** This is a view-only Graph mode,
not a global project lock: native Properties/Dopesheet operations keep their own
policies. Raw Broken/Aligned/Mirrored semantics and inverse transforms remain.
No normalized FFD vectors, events, constraints or nonnumeric tracks are invented.

The toggle uses existing temporary `tl_view` memory: it follows clips and projects
in the same egui context, but resets with a fresh context/restarted app. No Config,
.ankh, author key, dirty revision, Undo entry or Redo loss is produced by toggling.
Save/Reload and native/rendered exports keep authored values. Source-level native
sampler/handle/edit functions are unchanged; logical Raw shapes are compared with
an independent exact T-825 checkout, including flat and overshooting fixtures.
These checks are headless logical evidence, not GPU pixels or manual validation.

Bounds are collected once per eligible channel per frame; subsequent transforms
reuse them. This adds no KeyClip capture or selection mutation per frame. Existing
native large-selection validation is quadratic and remains a future scoped task;
10k debug selections took about 0.6 s locally. Phase-separated CPU observations
in the delivery are not a Windows/GPU performance guarantee. All 24 new manual
scenarios and the cumulative T-813–T-825 desktop campaign remain pending.

## 9. Large native selection validation (T-827)

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for the native capture bottleneck,
not a general graph performance certificate. Earlier sections retain their
checkpoint scope. Native large captures now index immutable tracks/names locally
and deduplicate exact typed references while retaining request order and all key
checks. Small requests retain the old path. There is no selection cap or persistent
key cache. Scalar invalid/empty sets still clear; grouped/mixed FFD still refuses
under its existing policy. Dirty/Undo/Redo semantics are unchanged.

T-825 marquee and T-826 coordinates, colors, controls and View Only restrictions
are unchanged. Faster native validation does not remove graph/sheet linear
selection membership checks or repeated work in existing key-editing commands.
Paired debug observations in the local delivery separate native set/read from
whole selected-graph/sheet display; even 10k selected-key display can remain slow.
Do not equate a faster capture with a certified frame rate or desktop input
latency. All T-827 and cumulative T-813–T-826 manual checks remain pending;
T-704/T-912 remain PARTIAL.
