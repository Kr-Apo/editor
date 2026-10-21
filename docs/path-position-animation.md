# Native Path Position Animation (T-831)

Status: **COMPLETE / PENDING_MANUAL / LOCAL_CANDIDATE** for this bounded property.
T-502 remains **PARTIAL**. No desktop session or JavaScript qualification was performed.

## Meaning and evaluation

Setup `PathConstraint.position` is unchanged. `Timeline::PathPosition` stores an
absolute position in the same fractional units, targeting a native `ConstraintId`.
It is not a bone offset and is not measured in pixels. With `constant_speed`, the
fraction multiplies world-space arc length; otherwise it multiplies segment/index
extent. Spacing retains the existing formula, including endpoint crowding. Open
paths clamp and closed paths wrap. Duplicate zero-length segments use the existing
sampler; zero-length/missing paths cannot receive new keys. Paths remain polylines.

Each evaluation resets transient overrides, samples the existing scalar
Linear/Stepped/normalized Bézier interpolator, then supplies the effective value
to the same Path solver at its existing place in `constraint_order`. Setup is never
rewritten. Existing clip weights and constraint rotation/translation mixes remain
the authority; this feature adds no new clip mixer. No track means the same static
solver call and outputs as before. Duplicate drivers in one clip are rejected at
authoring/format boundaries; core ignores that ambiguous override defensively.
Finite overflow in animated placement cannot propagate NaNs into driven bones.

## Authoring

In Animate, select an existing Path constraint and an active clip. The inspector
distinguishes Setup from evaluated Position. Enter a finite fraction and click
**Key Position** at the playhead. Auto-key commits a changed value on field
completion. With Auto-key off, completion creates a session-only preview; **K** or
**Key Position** commits it, and **Discard Path preview** cancels it. Context changes
discard the preview. No document mutation occurs merely by displaying a panel.
A first key later than zero also seeds the Setup value at zero in the same command.
In Setup, the existing Setup Position control remains a structural edit only.

The Dopesheet groups **Path Position** under the exact constraint. Select one key
and open its context menu: edit **Time (seconds)** and **Position (fraction)**,
choose **Path interpolation**, then **Apply Path key**; **Delete Path key** removes
it. Interpolation presets reuse native curves, including overshoot. An existing
key at the same time is updated by Key Position; moving onto another key refuses.
Negative times, non-finite inputs and malformed curves refuse before mutation.
Graph curves, key dragging/group operations and clipboard operations for this new
track are deliberately deferred; existing bone/slot/FFD tools are unchanged.

## Identity and history

Slotmap IDs are exact in memory. On disk the existing names contract applies:
one uniquely named Path constraint, one default skin, one Setup slot/Path entry,
and uniquely resolvable bone references are required for this new track. No
first-homonym fallback is allowed. The solver still uses the default-skin Setup
Path attachment, not animated attachment selection or alternate skins.
Missing/replaced IDs, wrong types, missing bones/slots/geometry, locked driven or
slot bones, zero mixes, stale context and concurrent property tracks refuse.
The edit lease includes document identity/revision, mode, selected entity, active
clip, playhead and FPS. Locks and target references are revalidated on submission.

Each accepted edit is one native history command, with no merge into a preceding
gesture. No-op/refusal preserves Redo. Keyed Path constraint deletion removes
dependent PathPosition tracks; Undo restores the exact constraint IDs, solve order
and timelines, so older key commands remain meaningful. Numeric whole-clip T-830
retiming includes PathPosition and checks its target and locks before preparation.

## Files and compatibility

Author schema/envelope version **1** remains unchanged. The additive tagged variant
is `{"kind":"path_position","constraint":"Path name","keys":[...]}`. Keys have
the existing scalar time/value/curve shape. Compact tags for existing members stay
unchanged; the new name is carried literally. No reinterpretation or migration of
old values is performed. The schema inventory and conversion both directions are
updated. New files round-trip all three author profiles with assets and evaluated
poses; old projects acquire no artificial tracks. Save As changes the active path
only after success and existing publication/recovery protections remain in force.

**Older readers cannot edit these new files.** Frozen T-830 rejects the unknown
enum variant in readable JSON, compact JSON and binary, before returning an
editable document. This is an intentional fail-closed forward-compatibility limit,
not a claim that all v1 features are available to every v1 reader. The pre-T-813
public reader uses the same closed timeline enum and cannot interpret this tag.

## Export policy

| Target | Policy for animated PathPosition |
|---|---|
| Author `.ankh`, `.ankh.json`, `.ankh.min.json` | Supported, strict reference validation |
| CPU PNG/JPEG/WebP sequences | Existing Sequence/evaluator, default skin/Normal/no physics restrictions retained |
| CPU spritesheets | Same Sequence frames; reconstructed RGBA compared exactly |
| CPU video/GIF | Same Sequence input; historical encoder/process checks remain, no new visual/video qualification |
| Native runtime v2, legacy, generic/custom templates | Blocking preflight and plan refusal before atlas or writes |
| Script/plugin exporter | Refused before executing script; original document returned |
| PixiJS / Phaser | Unchanged; no new field and no claimed support |

Static Path diagnostics (`path-unqualified`) remain explicit. Rendered output is
not a qualification of all constraint interactions. No JavaScript data exporter
may silently substitute a static path for this animation. Runtime v1/v2, rendered
metadata, staging/collisions/rollback and historical output references are unchanged.

## Remaining limits

PathSpacing keys, geometric Bézier handles, Graph editing, group/drag/clipboard
Path keys and runtime path parity are separate work. The finite f32 domain and
existing solver hierarchy/mix conventions apply. Manual Windows authoring, artistic
judgment and cumulative desktop checks remain **PENDING MANUAL REVIEW**.
