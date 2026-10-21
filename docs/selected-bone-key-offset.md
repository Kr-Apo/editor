# Selected-bone key offset (T-833)

**Implemented locally / PENDING MANUAL REVIEW.** No GUI, inference or publication.

Choose an active clip in Animate, explicitly select bones, then **Pose → Offset Selected Bones…**. Enter signed or fractional frames and click **Prepare preview**. Review the exact bones, track/key counts, committed FPS, frame/second offset, fixed duration, first/last times, safe-clamp count and keys beyond duration. Accept warnings explicitly if present; **Apply** commits once. Cancel/Escape discard the draft without changing revision, dirty state, history or Redo. Zero/no-key changes cannot Apply.

## Exact ownership

`Session.selected_bones` supplies exact BoneIds. No name heuristics, whole-rig fallback, automatic children or KeyRef selection. Native BoneTranslate X/Y, BoneRotate, BoneScale X/Y and BoneShear X/Y only. Missing channels are not created. Duplicate/removed IDs or duplicate addresses refuse. A targeted locked bone refuses the entire operation.

Nonselected bones, slots, attachments, visibility/color, draw order, Deform/FFD, IK/Transform/PathPosition, events, markers, evaluation offsets, other clips, Setup, images, binds, weights and UV stay unchanged. T-905 non-destructive sampling offsets are separate. T-211 remains PARTIAL because the historical Shift+K binding is still unidentified. The T-832 matrix's 101 categories and 352 manual scenarios are retained unchanged.

## Time and collisions

`seconds = frames / committed_project_fps` in f64, checked conversion to f32. FPS 1–240, following SetProjectFps. Native mapping is `max(0, old_time + seconds)` in f32, identical to global offset's mapping. No interpolation or sampler change. Duration never changes, even when the shifted keys exceed it. Those keys remain and require acknowledgement. One clamped key per track is allowed only if strict ordering remains; ambiguous clamp or rounding collision refuses the whole batch, without deleting, sorting or merging keys. Nonfinite, unrepresentable, negative/unsorted source times and overflow refuse. Sub-f32 changes can be no-ops.

## Preparation, history and cost

Explicit Prepare builds a disposable typed snapshot of matching tracks only. It never dispatches/undoes, increments revision or consumes Redo. Ordinary typing/repaint performs no key capture or preview rebuild. Lease captures document identity/revision, clip, exact selected BoneIds, selected locks, mode, committed FPS and input text; matching track indices/addresses/payloads are held in the prepared object and revalidated once immediately before dispatch. A context change invalidates Apply and requires explicit Prepare again. No selection recapture on Apply.

One native command stores targeted before/after timelines. It has no merge behavior. No-op/refusal/Cancel never create an entry; separate Applies stay separate. Undo/Redo restore exact snapshots, including a safe clamp. Save/reopen retains native times with existing safe-write guarantees, not a new crash-durability promise. Preparation and final validation scan timeline headers plus selected keys; memory and Apply/Undo/Redo copies scale with selected payload. CPU observations are not Windows performance certification.

Global T-830 and fixed actions are unchanged. PathPosition remains excluded here and still explicitly refused by unsupported runtime exporters. Author-format/minimum-reader release policy remains pending before another alpha. No format, version, renderer or runtime change.
