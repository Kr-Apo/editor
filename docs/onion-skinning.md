# Onion skinning — local T-824

**COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for the bounded refinement.
Historical T-703 visual acceptance is still PARTIAL. No new published release.

## Controls and colors

Animate with an active clip: click the existing onion transport icon. Right-click
for steps per side (1–5), Step between keys, Selected bones only, Opacity and
Distance falloff. Ghosts are artwork under the current artwork and gizmos, not
additional bone overlays. Past uses the existing theme `channel_shear` token
(reddish in bundled themes), future `channel_rotate` (greenish). These shared
roles are editable in existing Theme settings; no new theme format. Custom
palettes must preserve contrast. The menu labels Past/Current/Future explain
the roles; the current pose's colors and alpha are unchanged.

Ghost alpha multiplier is `opacity / step^falloff`, multiplied by the theme
color's alpha and the existing slot/art alpha. Opacity is 0..1 (default 0.32),
falloff exponent 0..2 (default 0.5). Zero falloff gives equal opacity; default
matches the previous 0.32/sqrt(step) attenuation. Nonfinite input falls back to
the defaults; finite out-of-range values are clamped at the render boundary.
Distance means step rank (nearest key or frame = 1), not elapsed seconds.

## Times

Frame mode preserves playhead-relative `t ± step/fps`, without quantizing the
playhead. FPS is native u32, so nonfinite FPS is unrepresentable; zero disables
frame sampling. Playhead/duration must be finite, duration positive and playhead
within the clip. Both modes omit samples outside inclusive [0,duration], omit
current-time samples, and avoid duplicate times. No loop wrapping or repeated
endpoint clamping. Only the onion planner changes; playback and core sampling
are unchanged. Floating-point resolution can reduce the available sample count.

Key mode retains all animation Timeline families through the existing key_times
helper: bone translate/rotate/scale/shear, slot visibility/color/attachment, draw
order, IK mix/bend/softness, transform-constraint mix and Deform. Events, markers,
inline attachment FFD and sampling offsets do not add reference times. The mode
is global across the clip, independent of selection. Finite in-range times are
sorted, coalesced with the existing 1e-5-second tolerance, and walked nearest-first
on each side. Keys within that tolerance of the playhead are excluded. No implicit
new keying or family-specific evaluator.

## Selection, safety and persistence

Selected-only filters drawn slots by their owning bone. It does not filter by
weight influence membership or prune evaluation dependencies. Parents, IK,
weighted bones and clipping still use the full reference pose. No live selected
bone means no ghost evaluations. Existing skin stack, posed attachment, draw
order, visibility, texture, mesh and clipping paths are reused.

Only Session fields change: no key, history entry, dirty content, geometry,
material, export metadata or .ankh field. New AppState resets defaults; project
loading in the running session follows the existing Session lifecycle. Saving
and reopening does not persist onion preferences. Theme colors follow existing
Theme configuration persistence independently of the document.

Off, Setup, missing clip, hidden artwork and zero opacity perform zero ghost
pose evaluations. There is no existing safe ghost-pose cache: each requested
visible render evaluates up to ten stateless poses. Appearance-only changes
still evaluate valid times, without adding repaint scheduling. Fresh immutable
reads avoid stale document/clip/selection/Undo/Redo cache entries. Existing texture
upload caching remains intact. Physics ghosts do not simulate accumulated state.

## Deferred validation

No desktop GUI, GPU, inference or artistic validation was run. Headless tests
cover logical submissions and egui events, not actual displayed pixels. Review
light/dark contrast, scrubbing, IK/weighted art and perceived performance on
copies during the cumulative T-813–T-824 manual campaign. CPU timings are local
observations rather than a universal budget. Historical T-823 video/process
concurrency sensitivity is retained; final regression tests use four threads.
