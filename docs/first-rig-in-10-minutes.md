---
title: First rig in ten minutes
description: Explore the existing eight-bone IK example and weight brush on disposable copies, without code or AI.
---

# First rig in ten minutes

Ankhimate — Kr-Apo Community Fork (Unofficial). This exercise uses existing tools;
it does not certify artistic results. T-813–T-816 desktop qualification is
**PENDING MANUAL REVIEW**. No AI, internet connection, compiler or FFmpeg is needed.
Ten minutes is a suggested exercise, not a measured completion time.

## 1. Open a disposable example (minutes 0–2)

Start the local T-816 development executable. Use **File → Open** to open
[`samples/tentacle.ankh`](../samples/tentacle.ankh) from the source tree, or
`Samples/Tentacle/tentacle.ankh` beside this delivery. It has no external images:
this is a bones-only rig, so enable bones in the viewport if hidden. Do not expect
character artwork. Use **File → Save As** to save a practice copy in a new folder.
Leave the supplied example unchanged.

The top-level **Setup / Animate** choice determines what edits mean. **Setup**
changes the rig's base structure; **Animate** changes keys in the selected clip.
`Tab` switches modes when a text field is not taking keyboard input. Use the
visible mode control if a shortcut is captured by a field.

## 2. Examine the chain (minutes 2–5)

In **Setup**, expand the Hierarchy and select `tentacle1`. The chain runs from
`tentacle1` through `tentacle8`; `root` supports it. The separate handle is
`tentacle-target`. Select either a chain bone or that target and look in
**Properties**; reopen hidden panels through **View**. The `tentacle-ik` section
shows its bone count, **Chain root**, **Chain order**, **Solver**, **Evaluated
span** and **Target**. You may also select the constraint in the tree.

**Evaluated span** is the sum of the current world-space joint gaps and the last
tip segment. It includes the evaluated pose/scale/stretch. It is not a prediction
of maximum reach or the separate natural-reach input used by softness/stretch.
A diagnostic replaces it for missing/invalid references or degenerate segments.

Select `tentacle-target`, use the Translate tool (`T`), and make a small drag.
The existing FABRIK solver updates all eight bones. Undo with **Edit → Undo**;
Redo restores the authoring edit. Bone editing locks prevent direct editing,
not evaluation by an IK constraint. Avoid locking the target during this exercise.

In Setup, try **Mix**: 0 gives the authored FK pose, 1 uses IK. **Flip bend**
prefers the opposite bend side. With three or more bones there can be many valid
poses; bend direction is a preference, not a guarantee of a unique shape.
**Stiffness** controls how much the solver retains the starting pose. Undo your
experiments. Keep defaults for the next step.

To create your own IK later: select a continuous parent–child chain in Hierarchy
using Shift-click, then **Properties → Add driver… → IK target (N-bone chain)**.
This creates a target and constraint as one native Undo step in Setup. The UI
sorts the chain by ancestry; unrelated selections cannot create a chain.
One bone aims, two use the analytic solver, and three or more use FABRIK.

## 3. Change a key (minutes 5–8)

Switch to **Animate**, show **Animations** and **Dopesheet** through **View**, and
select the existing `curl` clip (2 seconds). Play, stop, and scrub. Select
`tentacle-target`; place the playhead halfway through the clip. Enable the
existing **Auto-key** control, then slightly change its Translate X field or
translate it in the viewport. The edit changes the target's animation offset,
not the Setup structure. Inspect the corresponding Translate X key in the
Dopesheet; Undo and Redo should remove/restore your edit. See the
[animation/curve guide](graph-editor.md) for detailed key editing.

IK property fields in Properties edit Setup and are disabled in Animate. For
constraint animation, use the existing IK timeline tracks; this exercise only
keys the target. No new constraint-key UI is introduced in T-816.

## 4. Save your practice file (minutes 8–10)

Use **File → Save As** with another name if you have not already done so. Keep
any generated `.assets` folder with its project. Reopen your copy and confirm
the clip and pose. If prompted about unsaved changes, **Cancel** keeps the
current document, **Save** must succeed before replacement, and **Discard**
abandons your edits. Never use Discard on work you need.

## Try weight painting separately

Use a **copy** of the delivered Jade Sentinel `Sentinelle.ankh` and its complete
`Sentinelle.assets` folder (the unchanged approved sample, not unsaved Qwen edits).
Do not paint over the original. Tentacle contains no mesh to paint.

1. Enter **Setup**, select a slot containing a weighted mesh, then **View →
   Weights**. The panel identifies the active influence by name and lock state.
2. Click **Activate Weight Paint**, or use `W`. Click a bone row in **Weights**
   to choose its influence; the mesh remains selected. A bone click on the
   canvas while using Weight Paint has the same targeting behavior.
3. Start with **Add** and a modest **Weight**. **Radius** is world-space radius
   (4–400), independent of viewport zoom. **Feather** (0–1) controls the gradient:
   0 is a hard edge, 1 fades outward from the center.
4. Hover a bone row to reveal its lock. Locking reserves that influence's share;
   a locked active influence is not painted. Locks are session settings and are
   not saved. The status includes text, so color is not the only cue.
5. Drag lightly over the mesh, inspect the result, then **Undo / Redo**. Add,
   Subtract, Replace and Smooth are the existing brush modes. **Weight** is the
   target in Add/Replace, removal rate in Subtract and blend strength in Smooth.
   Direct uses selected vertices and its slider instead of a brush.
6. Save As a separate copy and reopen. Inspect the mesh again. Normalization is
   handled by native weight operations; there is no separate Normalize brush mode.

[Full deformation guide](animator/deformation.md) documents Auto, Bind, Prune,
Update bindings and other whole-mesh operations. Do not use them simply to find
the brush: they change data and have different purposes.

## Where these features play

IK is an editor/Rust-core capability. Native v2 can carry constraint data, but
the current alpha game adapters exclude IK/physics and do not bake solved poses.
Do not expect this Tentacle rig to play faithfully in PixiJS or Phaser by simply
exporting it. See [rigging capabilities](rigging-capabilities.md),
[format contracts](format-spec.md) and [known alpha limitations](KNOWN_LIMITATIONS.md).
