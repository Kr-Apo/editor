//! A narrow adapter over native region/slot commands, not a second attachment model.
use super::tools::{Action, Snapshot, ToolError};
use ankhimate_core::{
    attachment::{Attachment, RegionAttachment},
    ids::{SkinId, SlotId},
    skeleton::Skeleton,
    transforms::Affine2,
};
use ankhimate_document::{
    commands::{
        EditCommand,
        attachment_cmds::{RegionProps, SetRegionProps},
        slot_cmds::{SetDrawOrder, SetSlotBone},
    },
    doc::Document,
};
use serde_json::{Value, json};
pub const WORLD_TOLERANCE: f32 = 0.01;
fn problem(
    source: &Snapshot,
    a: &Action,
    index: usize,
    code: &str,
    field: &str,
    cause: &str,
) -> Box<ToolError> {
    Box::new(ToolError{code:code.into(),operation:a.operation().into(),action_index:Some(index),field:Some(field.into()),cause:cause.into(),correction:"Read exact skin/slot/attachment identities. Edit explicitly owned regions in Setup; association needs a single-entry slot, an unlocked existing bone and a representable FK Setup conversion. No weights, meshes or implicit fallback edits.".into(),document_changed:false,revision:source.revision})
}
fn unique_slot(s: &Skeleton, name: &str) -> Option<SlotId> {
    let mut ids = s
        .slots
        .iter()
        .filter(|(_, slot)| slot.name == name)
        .map(|(id, _)| id);
    let first = ids.next()?;
    ids.next().is_none().then_some(first)
}
fn unique_skin(s: &Skeleton, name: &str) -> Option<SkinId> {
    let mut ids = s
        .skins
        .iter()
        .filter(|(_, skin)| skin.name == name)
        .map(|(id, _)| id);
    let first = ids.next()?;
    ids.next().is_none().then_some(first)
}
fn finite(m: Affine2) -> bool {
    [m.a, m.b, m.c, m.d, m.tx, m.ty]
        .iter()
        .all(|n| n.is_finite())
}
fn valid_region(r: &RegionAttachment) -> bool {
    r.local_offset.is_finite()
        && r.local_offset.abs().max_element() <= 1e6
        && r.local_rotation.is_finite()
        && r.local_rotation.to_degrees().abs() <= 360000.0
        && r.local_scale.is_finite()
        && r.local_scale.abs().min_element() >= 0.0001
        && r.local_scale.abs().max_element() <= 100.0
        && r.width.is_finite()
        && r.height.is_finite()
        && (0.0..=1e6).contains(&r.width)
        && (0.0..=1e6).contains(&r.height)
        && r.pivot.is_finite()
}
fn entry_count(s: &Skeleton, slot: SlotId) -> usize {
    s.skins
        .values()
        .map(|skin| skin.names_for_slot(slot).count())
        .sum()
}
fn has_clipping(s: &Skeleton) -> bool {
    s.skins.values().any(|skin| {
        skin.entries
            .values()
            .any(|a| matches!(a, Attachment::Clipping(_)))
    })
}
fn region_local(r: &RegionAttachment) -> Value {
    json!({"position":r.local_offset.to_array(),"rotation_degrees":r.local_rotation.to_degrees(),"scale":r.local_scale.to_array(),"dimensions":[r.width,r.height],"pivot":r.pivot.to_array()})
}
pub fn corners(s: &Skeleton, slot: SlotId, r: &RegionAttachment) -> [glam::Vec2; 4] {
    let m = s.setup_world(s.slots[slot].bone);
    r.local_corners().map(|p| m.transform_point(p))
}

pub fn apply(
    doc: &mut Document,
    source: &Snapshot,
    a: &Action,
    index: usize,
) -> Result<Box<dyn EditCommand>, Box<ToolError>> {
    let err = |code, field, cause| problem(source, a, index, code, field, cause);
    if let Action::DrawOrder { slots } = a {
        if slots.len() > 256 {
            return Err(err(
                "invalid_draw_order",
                "slots",
                "At most 256 slots per assembly order",
            ));
        }
        if has_clipping(&doc.skeleton) {
            return Err(err(
                "clipping_order_protected",
                "slots",
                "Clipping ranges make this reorder unsafe; use the manual editor",
            ));
        }
        let mut order = vec![];
        for name in slots {
            let id = unique_slot(&doc.skeleton, name)
                .ok_or_else(|| err("slot_not_unique", "slots", "Missing or ambiguous slot"))?;
            if order.contains(&id) {
                return Err(err(
                    "duplicate_slot",
                    "slots",
                    "Each slot must appear exactly once",
                ));
            }
            order.push(id);
        }
        if order.len() != doc.skeleton.slots.len() || doc.skeleton.draw_order.len() != order.len() {
            return Err(err(
                "invalid_draw_order",
                "slots",
                "A complete valid order is required",
            ));
        }
        let mut original = doc.skeleton.draw_order.clone();
        original.sort();
        original.dedup();
        if original.len() != order.len()
            || original
                .iter()
                .any(|id| !doc.skeleton.slots.contains_key(*id))
        {
            return Err(err(
                "invalid_draw_order",
                "slots",
                "Existing order is invalid",
            ));
        }
        for (at, id) in order.iter().enumerate() {
            if doc.skeleton.draw_order[at] != *id {
                if source.locked.contains(&doc.skeleton.slots[*id].bone) {
                    return Err(err(
                        "bone_locked",
                        "slots",
                        "A moved slot belongs to a locked bone",
                    ));
                }
                if entry_count(&doc.skeleton, *id) != 1
                    || doc
                        .skeleton
                        .skins
                        .values()
                        .filter_map(|skin| {
                            skin.names_for_slot(*id)
                                .next()
                                .and_then(|name| skin.get(*id, name))
                        })
                        .any(|a| !matches!(a, Attachment::Region(_)))
                {
                    return Err(err(
                        "shared_or_nonregion_slot",
                        "slots",
                        "Reorder supports single-region slots only",
                    ));
                }
            }
        }
        let mut command = SetDrawOrder::new(order);
        command.apply(doc);
        return Ok(Box::new(command));
    }
    let (skin, slot, name) = match a {
        Action::RegionTransform {
            skin,
            slot,
            attachment,
            ..
        }
        | Action::SlotBone {
            skin,
            slot,
            attachment,
            ..
        } => (skin, slot, attachment),
        _ => {
            return Err(err(
                "invalid_assembly_action",
                "op",
                "Unsupported assembly action",
            ));
        }
    };
    let slot = unique_slot(&doc.skeleton, slot)
        .ok_or_else(|| err("slot_not_unique", "slot", "Missing or ambiguous slot"))?;
    let skin = unique_skin(&doc.skeleton, skin)
        .ok_or_else(|| err("skin_not_unique", "skin", "Missing or ambiguous skin"))?;
    let bone = doc.skeleton.slots[slot].bone;
    if !doc.skeleton.bones.contains_key(bone) {
        return Err(err("invalid_slot_bone", "slot", "Slot bone is missing"));
    }
    if source.locked.contains(&bone) {
        return Err(err("bone_locked", "slot", "The slot's bone is locked"));
    }
    let region = match doc.skeleton.skins[skin].get(slot, name) {
        Some(Attachment::Region(r)) => r.clone(),
        Some(_) => {
            return Err(err(
                "attachment_read_only",
                "attachment",
                "Only existing regions can be edited; mesh, weights, binds and other geometry are read-only",
            ));
        }
        None => {
            return Err(err(
                "attachment_not_owned",
                "skin/slot/attachment",
                "No explicitly owned entry in this skin; fallback is never edited implicitly",
            ));
        }
    };
    if !valid_region(&region) {
        return Err(err(
            "invalid_region",
            "attachment",
            "Existing region has invalid geometry",
        ));
    }
    match a {
        Action::RegionTransform {
            x,
            y,
            rotation,
            scale_x,
            scale_y,
            ..
        } => {
            let mut props = RegionProps::from_region(&region);
            props.offset = glam::vec2(*x, *y);
            props.rotation = rotation.to_radians();
            props.scale = glam::vec2(*scale_x, *scale_y);
            let mut check = region.clone();
            check.local_offset = props.offset;
            check.local_rotation = props.rotation;
            check.local_scale = props.scale;
            if !rotation.is_finite() || rotation.abs() > 360000.0 || !valid_region(&check) {
                return Err(err(
                    "invalid_region_transform",
                    "x/y/rotation/scale_x/scale_y",
                    "Finite local coordinates <=1000000, rotation magnitude <=360000 degrees and scale magnitude 0.0001–100 required",
                ));
            }
            let mut command = SetRegionProps::new(skin, slot, name, props);
            command.apply(doc);
            Ok(Box::new(command))
        }
        Action::SlotBone {
            bone: target,
            keep_world,
            ..
        } => {
            if !keep_world {
                return Err(err(
                    "keep_world_required",
                    "keep_world",
                    "Only explicit FK Setup world-preserving association is supported",
                ));
            }
            if entry_count(&doc.skeleton, slot) != 1 {
                return Err(err(
                    "shared_slot",
                    "slot",
                    "Slot has other attachments/skin entries; association would implicitly affect them",
                ));
            }
            if !doc.skeleton.constraints.is_empty() {
                return Err(err(
                    "constrained_association",
                    "bone",
                    "Association is limited to rigs without constraints; no constrained-pose preservation is promised",
                ));
            }
            let ids: Vec<_> = doc
                .skeleton
                .bones
                .iter()
                .filter(|(_, b)| b.name == *target)
                .map(|(id, _)| id)
                .collect();
            let [target] = ids.as_slice() else {
                return Err(err(
                    "bone_not_unique",
                    "bone",
                    "Missing or ambiguous destination bone",
                ));
            };
            let target = *target;
            if source.locked.contains(&target) {
                return Err(err("bone_locked", "bone", "Destination bone is locked"));
            }
            for start in [bone, target] {
                let mut at = Some(start);
                while let Some(id) = at {
                    if doc
                        .skeleton
                        .skins
                        .values()
                        .any(|skin| skin.bones.contains(&id))
                    {
                        return Err(err(
                            "skin_bone_association",
                            "bone",
                            "Skin-dependent bone chains cannot receive a global slot binding",
                        ));
                    }
                    at = doc.skeleton.bones[id].parent;
                }
            }
            let from = doc.skeleton.setup_world(bone);
            let to = doc.skeleton.setup_world(target);
            if !finite(from) || !finite(to) || from.invert().is_none() || to.invert().is_none() {
                return Err(err(
                    "singular_conversion",
                    "bone",
                    "Nonfinite or singular Setup frame; native fallback inversion is not allowed here",
                ));
            }
            let before = corners(&doc.skeleton, slot, &region);
            if before.iter().any(|v| !v.is_finite()) {
                return Err(err(
                    "invalid_world_geometry",
                    "attachment",
                    "World coordinates are not finite",
                ));
            }
            let mut command = SetSlotBone::keeping_world(&doc.skeleton, slot, target);
            command.apply(doc);
            let Some(Attachment::Region(after)) = doc.skeleton.skins[skin].get(slot, name) else {
                unreachable!()
            };
            let after_corners = corners(&doc.skeleton, slot, after);
            if !valid_region(after)
                || after_corners.iter().zip(before).any(|(after, before)| {
                    !after.is_finite() || after.distance(before) > WORLD_TOLERANCE
                })
            {
                return Err(err(
                    "nonrepresentable_conversion",
                    "bone",
                    "Native conversion would lose shear or exceed 0.01 world units at a corner; proposal rejected",
                ));
            }
            Ok(Box::new(command))
        }
        _ => unreachable!(),
    }
}

/// Canonical values for the only mutable stores; skins' tuple keys become rows.
pub fn fingerprint(s: &Skeleton) -> Value {
    let skins:Vec<_>=s.skins.iter().map(|(id,skin)|{let mut entries:Vec<_>=skin.entries.iter().collect();entries.sort_by_key(|(key, _)| *key);json!({"id":id,"name":skin.name,"bones":skin.bones,"constraints":skin.constraints,"entries":entries.into_iter().map(|((slot,name),a)|json!([slot,name,a,if let Attachment::Mesh(m)=a { let mut binds:Vec<_>=m.inverse_bind_matrices.iter().collect();binds.sort_by_key(|(bone,_)|*bone); json!(binds) }else{Value::Null}])).collect::<Vec<_>>()})}).collect();
    json!({"bones":s.bones,"slots":s.slots,"order":s.draw_order,"skins":skins})
}
pub fn review(doc: &Document, a: &Action) -> Value {
    match a {
        Action::DrawOrder { .. } => {
            json!({"draw_order":doc.skeleton.draw_order.iter().filter_map(|id|doc.skeleton.slots.get(*id).map(|s|&s.name)).collect::<Vec<_>>()})
        }
        Action::RegionTransform {
            skin,
            slot,
            attachment,
            ..
        }
        | Action::SlotBone {
            skin,
            slot,
            attachment,
            ..
        } => {
            let found = unique_slot(&doc.skeleton, slot).zip(unique_skin(&doc.skeleton, skin));
            if let Some((slot, skin)) = found
                && let Some(Attachment::Region(region)) =
                    doc.skeleton.skins[skin].get(slot, attachment)
            {
                return json!({"skin":doc.skeleton.skins[skin].name,"slot":doc.skeleton.slots[slot].name,"attachment":attachment,"type":"region","bone":doc.skeleton.bones.get(doc.skeleton.slots[slot].bone).map(|b|&b.name),"local":region_local(region),"draw_index":doc.skeleton.draw_order.iter().position(|id|*id==slot)});
            }
            Value::Null
        }
        _ => Value::Null,
    }
}

fn graphic(state: &crate::app_state::AppState, texture: &str) -> Value {
    let private = texture.contains(":")
        || texture.starts_with(['/', '\\'])
        || texture.chars().any(char::is_control);
    let assets: Vec<_> = state
        .doc
        .assets
        .images
        .values()
        .filter(|a| a.name == texture)
        .collect();
    json!({"reference":if private{"<private reference redacted>"}else{texture},"reference_redacted":private,"resolved_uniquely":assets.len()==1,"dimensions":if assets.len()==1{Some([assets[0].width,assets[0].height])}else{None}})
}
pub fn metadata(state: &crate::app_state::AppState) -> (Vec<Value>, Vec<Value>) {
    let s = &state.doc.skeleton;
    let hierarchy = super::rig::check_hierarchy(s).is_ok();
    let stack = state.session.skin_stack();
    let slots=s.slots.iter().take(256).map(|(id,slot)|{
        let showing=state.pose.attachment_name(s,id);let owner=showing.and_then(|name|stack.iter().chain(std::iter::once(&s.default_skin)).copied().find(|skin|s.skins.get(*skin).is_some_and(|skin|skin.get(id,name).is_some())));
        json!({"slot":slot.name,"bone":s.bones.get(slot.bone).map(|b|&b.name),"setup_attachment":slot.attachment,"current_attachment":showing,"resolved_skin":owner.and_then(|id|s.skins.get(id).map(|s|&s.name)),"uses_default_fallback":owner==Some(s.default_skin)&&state.session.active_skin!=s.default_skin,"skin_stack":stack.iter().filter_map(|id|s.skins.get(*id).map(|s|&s.name)).take(64).collect::<Vec<_>>(),"draw_index":s.draw_order.iter().position(|i|*i==id),"current_draw_index":state.pose.draw_order.iter().position(|i|*i==id),"locked":state.session.is_bone_locked(slot.bone),"entry_count_all_skins":entry_count(s,id)})
    }).collect();
    let mut attachments = vec![];
    for (_, skin) in s.skins.iter() {
        let mut entries: Vec<_> = skin.entries.iter().collect();
        entries.sort_by_key(|(key, _)| *key);
        for ((slot, name), attachment) in entries {
            if attachments.len() >= 256 {
                break;
            }
            let sl = s.slots.get(*slot);
            let locked = sl.is_none_or(|slot| state.session.is_bone_locked(slot.bone));
            let kind = match attachment {
                Attachment::Region(_) => "region",
                Attachment::Mesh(_) => "mesh",
                Attachment::Clipping(_) => "clipping",
                Attachment::Path(_) => "path",
                Attachment::BoundingBox(_) => "bounding_box",
                Attachment::Point(_) => "point",
            };
            let mut row = json!({"skin":skin.name,"slot":sl.map(|s|&s.name),"attachment":name,"kind":kind,"bone":sl.and_then(|sl|s.bones.get(sl.bone)).map(|b|&b.name),"locked":locked,"editable":matches!(attachment,Attachment::Region(_))&&!locked&&state.session.work_mode==crate::session::WorkMode::Setup,"edit_requires_mode":"setup","draw_index":s.draw_order.iter().position(|id|id==slot),"association_single_entry":entry_count(s,*slot)==1,"world_space_note":"FK Setup geometry versus evaluated current pose; image pixels are not local offsets"});
            match attachment {
                Attachment::Region(r) => {
                    row["local"] = region_local(r);
                    row["graphic"] = graphic(state, &r.texture);
                    row["setup_world_corners"] = if hierarchy && sl.is_some() {
                        json!(corners(s, *slot, r).map(|v| v.to_array()))
                    } else {
                        Value::Null
                    };
                    row["current_world_origin"] = sl
                        .map(|sl| {
                            json!(
                                state
                                    .pose
                                    .world(sl.bone)
                                    .transform_point(r.local_offset)
                                    .to_array()
                            )
                        })
                        .unwrap_or(Value::Null);
                }
                Attachment::Mesh(m) => {
                    row["mesh"] = json!({"vertices":m.setup_vertices.len(),"triangles":m.triangles.len(),"weighted":!m.weights.is_empty(),"linked":m.linked.is_some(),"ffd_keys":m.ffd_keyframes.len()});
                    row["graphic"] = graphic(state, &m.texture);
                    row["read_only_reason"] = json!(
                        "Mesh geometry, binds and FFD are read-only. No region transform; use ankhimate_read_mesh to inspect eligibility for bounded weight edits."
                    );
                }
                _ => {
                    row["read_only_reason"] = json!("Only region transforms are writable in T-807")
                }
            }
            attachments.push(row);
        }
    }
    (slots, attachments)
}
pub fn definition() -> Value {
    let mut variants =
        super::rig::definition()["parameters"]["properties"]["actions"]["items"]["oneOf"]
            .as_array()
            .unwrap()
            .clone();
    variants.extend([
        json!({"type":"object","properties":{"op":{"const":"attachment.set_region"},"skin":{"type":"string"},"slot":{"type":"string"},"attachment":{"type":"string"},"x":{"type":"number"},"y":{"type":"number"},"rotation":{"type":"number","description":"Absolute local degrees"},"scale_x":{"type":"number"},"scale_y":{"type":"number"}},"required":["op","skin","slot","attachment","x","y","rotation","scale_x","scale_y"],"additionalProperties":false}),
        json!({"type":"object","properties":{"op":{"const":"slot.set_bone"},"skin":{"type":"string"},"slot":{"type":"string"},"attachment":{"type":"string"},"bone":{"type":"string"},"keep_world":{"const":true}},"required":["op","skin","slot","attachment","bone","keep_world"],"additionalProperties":false}),
        json!({"type":"object","properties":{"op":{"const":"slot.set_draw_order"},"slots":{"type":"array","items":{"type":"string"},"maxItems":256}},"required":["op","slots"],"additionalProperties":false})
    ]);
    json!({"name":"ankhimate_assemble","description":"Propose Setup assembly with native bone/region/slot commands in one approved Undo lot. Existing regions only; explicit skin ownership. Slot association needs a single entry across ALL skins and preserves FK Setup world corners only. No weighted/linked mesh edits, shared-slot reassignment, implicit fallback or animation actions. Read slots and attachments first; positions are absolute bone-local values.","parameters":{"type":"object","properties":{"summary":{"type":"string","minLength":1},"actions":{"type":"array","minItems":1,"maxItems":128,"items":{"oneOf":variants}}},"required":["summary","actions"],"additionalProperties":false}})
}
