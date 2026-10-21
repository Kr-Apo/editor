# Numeric animation retiming (T-830)

Local candidate: **COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE**. Desktop acceptance is deferred.

In Animate, select a clip, open **Pose → Retime…**, enter **Time Scale** and **Offset in Frames**, then **Prepare preview**. Inputs are drafts: no dirty state, revision or history changes. Apply commits one native `RetimeAnimation`; Cancel, Escape, backdrop and X discard the draft. No-op and refused operations never consume Redo. Separate Applies never merge.

## Native contract

Every existing timeline in the active clip is retimed, independent of bone selection: translate, rotate, scale, shear, slot visibility/color/attachment, draw order, IK mix/bend/softness, transform constraint mix and Deform (including weighted FFD offsets). No FFD payload or geometry is edited. Other clips are untouched. Numeric preparation refuses tracks driving a locked bone or a slot bound to one; constraint driven/target bones are also checked. Existing fixed shortcuts remain unchanged.

`offset_seconds = frame_offset / project_fps`, converted to finite f32 after f64 division. Signed fractional frames and decimal/scientific notation are accepted; comma decimal and Unicode minus are understood. FPS must be within the existing transport editing bounds 1–240; legacy invalid/out-of-range values are not silently clamped. Correct the project FPS explicitly first.

Native f32 arithmetic is `t_after = max(0, t_before * scale + offset_seconds)`. Scale is any positive finite representable f32; zero, negatives, overflow and underflow to zero are refused. The historical fixed scaled constructor's 0.01 clamp remains unchanged; numeric input is never silently clamped to that factor.

**Duration follows the existing native rule:** if offset_seconds is exactly zero, multiply duration by scale. If offset_seconds is nonzero, keep the old duration, even in a combined operation. For duration 2 s, scale 2 and offset 3 frames at 30 FPS, keys map to `2*t+0.1`, while duration remains 2 s. The dialog displays this rule and actual before/after values. A positive factor is a duration scale, not playback speed: scale 2 doubles times; it is slower when duration scales with it.

Events, markers, bone evaluation offsets and looping are unchanged. Their nonempty presence requires explicit acknowledgement because timing alignment may change. These exclusions are visible before Apply. This destructive key-time edit is separate from T-905's existing non-destructive per-bone evaluation offsets; T-905 remains PARTIAL.

## Validation and preview

Preparation clones only the native animation and executes the same `retime_clip` helper used by native Apply. It validates finite/nonnegative strictly increasing input times, finite/nonnegative duration, f32 parameter conversion, raw mapped overflow/underflow, and strict output ordering per timeline. Equal times on different tracks are allowed. Equal or reordered times on the same track are refused: no sorting, fusion or deletion.

A negative result may clamp a single key to zero if ordering remains strict. The clamp count requires explicit acceptance. Multiple clamped keys, f32 rounding collisions or other non-strict output times refuse the entire operation. Undo restores original snapshots exactly even after a valid clamp. Keys beyond duration remain (not silently trimmed or duration-extended), are counted and require acknowledgement. Zero duration is representable; any resulting keys after zero are reported as outside duration.

Preview shows clip/scope/FPS, scale, frame offset and seconds, duration before/after, timeline/key count, changed key count, first/last times, zero collisions on valid output, clamp/outside counts and excluded metadata. Refusals identify the failing track where applicable. Values, interpolation, normalized Bezier handles/modes, FFD values, setup, assets, weights, UV and binds are unchanged. Affine positive time mapping preserves corresponding sampling within f32 accuracy outside truncated/clamped cases; no universal clamp fidelity claim.

## Context and performance

One explicit Prepare request computes the preview; typing and ordinary repaint do not clone or capture key data. The prepared lease contains document identity, revision, active animation, relevant bone/slot selection, lock set, mode, FPS and exact input strings. Changes invalidate Apply until another explicit Prepare. Key selection does not determine whole-clip scope and is not consulted or recaptured. Native revision covers author edits and Undo/Redo, including FPS. Transient data is never serialized. Apply consumes the prepared command once.

Preparation is O(keys plus payload size), synchronous on explicit request; 100/1000/10000-key headless CPU observations are in the delivery. It is not a desktop performance guarantee or a general large-edit optimization. Graph Raw/Normalize, T-827 capture and T-829 membership remain unchanged. Existing fixed retime shortcuts retain their historical weaker validation; this milestone does not retrofit their UX.

## Deferred manual checklist

Use a disposable animated copy. Exercise Pose → Retime, 2 / 0.5 / 1.25 scales, positive/negative frame offsets, warnings/refusals, Cancel/Escape, three Undo/Redo cycles, FPS/context invalidation, Save As and reopen. Inspect curves, FFD and excluded metadata. All desktop controls remain **PENDING MANUAL REVIEW**; no GUI or inference was run during T-830.
