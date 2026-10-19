# Native region assembly v1

ankhimate_assemble: summary + actions (1–128), Setup only, one approval/Undo.
Read slots, attachments, bones first. Bone = transform; slot = bone + depth;
attachment = named geometry owned by a skin. Fallback is NOT ownership.
- attachment.set_region: op, skin, slot, attachment, x, y, rotation, scale_x,
  scale_y. Existing REGION; absolute BONE-LOCAL values, rotation DEGREES.
  Preserve texture/UV/pivot/dimensions/sequence. Finite |position|<=1000000,
  |rotation|<=360000, 0.0001<=|scale|<=100. Negative scale is reflection.
- slot.set_bone: op, skin, slot, attachment, bone, keep_world:true. ONE entry
  across ALL skins; no constraints/skin-dependent bones. Four FK SETUP corners
  must stay within 0.01 units; singular or shear-losing conversion REJECTS.
  Animated poses may change: warn. Shared slots cannot be reassigned.
- slot.set_draw_order: op, slots = FULL slot-name permutation. Only moved
  single-region slots, no clipping. Setup order, not animation.
Combine bone.create(name,parent,position,rotation_degrees,length),
bone.setup(bone,position,rotation_degrees,scale), bone.rename(bone,name).
Bone values are PARENT-local Setup; create before referencing by exact name.
Place joints at plausible pivots. Associate first, then place in NEW bone frame.
Image pixels are not local units; separate facts, observations and estimates.
Images need T-806 consent. No mesh/bind/weight/vertex/FFD edits, image replacement,
implicit fallback, slot creation or animation actions. Locks enforced. Shared
regions allow exact-owner transforms only. No files/auto Save. Invalid => no
partial edit. [Examples](assembly-examples.json); Rust schemas are authoritative.
