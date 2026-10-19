# Recipes — select actual names, never assume this example's rig

## Breathing
Read the rig and clips. Choose the actual chest/torso bone. Create a unique
2-second looping clip, then scale_y keys at 0:1, 1:1.03, 2:1 with smooth
interpolation. Example names are placeholders; replace them only with names
actually returned by read. Offer the native proposal and await approval.

## Attack
Read hierarchy and existing clips. Choose a few relevant unlocked bones. Create
a new clip with rest at 0, anticipation around 0.25, impact around 0.5, recovery
around 0.8 and rest at the end. Use modest local rotation offsets in degrees.
Coordinate chest, arm and forearm only if those names exist. Do not promise
contacts, IK, physics or event creation; these are not tools in this version.

## Walk or idle
Read the actual rig and current motion. An idle may use small smooth scale and
rotation offsets. A simple walk may alternate leg rotations with body motion;
unsupported contact/constraint behavior must be disclosed. Match start/end rest
for loops, keep the action count within budget and avoid damaging existing clips.

## Diagnose
Read overview/bones/attachments/animations. Explain observed hierarchy, clips,
locks and truncation. Distinguish evidence from suggested artistic improvements.
Ask for manual structural editing when current tools cannot solve the problem.

## Observe then rig
Request viewport observation. Wait for the user's reviewed, authorized image
Send; read the actual torso bone. Propose a small accessory with explicit parent
and local Setup estimates through ankhimate_rig. Explain uncertainty and wait for
native approval. Never turn an image guess into a guaranteed observed coordinate.
