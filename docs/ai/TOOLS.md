# Native tools v2

ankhimate_read: section overview, bones, attachments, slots, animations,
selection or bone. Bone needs exact `bone` name; returns locks, length, local
Setup and current world affine. Array sections bones/attachments/slots/animations
accept offset 0–4096 and limit 1–64 (default 32 with offset); return items,
available, next_offset, revision. Selection is an object. Cached reads reference
earlier source_call_id in this Send. Bounded data, no paths/pixels; truncation
means incomplete knowledge.

ankhimate_animate: summary (nonempty, <=1000 UTF-8 bytes), actions (1–128), ordered:
- animation.create: op, name, duration, looping. New clip only.
- animation.set: op, animation, duration, looping. Existing clip; NEVER creates.
- animation.select: op, animation. Existing; session selection only.
- key.add: op, animation, bone, channel, time, value, interpolation. Existing or
  earlier-created clip; updates same channel/time.
Names: unique, nonempty, <=128 UTF-8 bytes, no controls. Finite duration
0.01–3600 seconds, containing keys; time [0,duration]. Channels: translate_x/y,
rotate, scale_x/y, shear_x/y (exact snake_case). Translation/rotation/shear are
Setup OFFSETS, angles DEGREES; scale MULTIPLIES Setup (1=rest). Value <=1000000
magnitude; scale magnitude 0.0001–100. Curves linear, stepped, smooth
(normalized Bezier 0.42/0, 0.58/1). No extra fields.

ankhimate_rig: [Setup](RIGGING.md), no animation mixed in batch.
ankhimate_assemble: [regions/slots](ASSEMBLY.md) + bones.
ankhimate_read_mesh / ankhimate_weights: [paged weights](WEIGHTS.md).
ankhimate_viewport: {} requests [consent](VISION.md), NOT pixels.

Edits need explicit approval, valid locks/references/revision, one Undo lot;
Save manual. Errors: code, operation, action_index, field, cause, correction,
document_changed:false, revision. At most two corrections within original
budget. Invalid means no edits; only native applied receipts prove success.
[Animation examples](examples.json), [Rig examples](rig-examples.json). No scripts.
