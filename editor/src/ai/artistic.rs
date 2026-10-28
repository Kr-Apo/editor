//! Small approved animation operations over the existing native commands.
use super::tools::{Action, Channel, Snapshot, ToolError};
use ankhimate_core::{
    animation::{Axis, Timeline},
    attachment::Attachment,
    ids::AnimationId,
};
use ankhimate_document::{
    Document,
    commands::{
        EditCommand,
        event_cmds::AddEvent,
        key_cmds::{AddDeformKey, BoneProperty, DeleteKeys, KeyRef, TimelineAddr},
    },
};
use serde_json::{Value, json};

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadDeform {
    pub animation: String,
    pub skin: String,
    pub slot: String,
    pub attachment: String,
    pub key_index: usize,
    pub start: usize,
    pub count: usize,
}
pub fn read(source: &Snapshot, args: &ReadDeform) -> Result<Value, String> {
    if args.count == 0 || args.count > 32 {
        return Err("Read 1–32 native offset pairs per page".into());
    }
    let doc = &source.doc;
    let clips = doc
        .animations
        .values()
        .filter(|a| a.name == args.animation)
        .collect::<Vec<_>>();
    let [clip] = clips.as_slice() else {
        return Err("Animation absent or ambiguous".into());
    };
    let slots = doc
        .skeleton
        .slots
        .iter()
        .filter(|(_, s)| s.name == args.slot)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    let [slot] = slots.as_slice() else {
        return Err("Slot absent or ambiguous".into());
    };
    let owner = ankhimate_document::ffd_keys::owner(doc, *slot, &args.attachment)?;
    if doc.skeleton.skins[owner].name != args.skin {
        return Err("Wrong owning skin".into());
    }
    let Some(Attachment::Mesh(mesh)) = doc.skeleton.skins[owner].get(*slot, &args.attachment)
    else {
        return Err("Not a mesh".into());
    };
    let tracks = clip
        .timelines
        .iter()
        .filter_map(|t| match t {
            Timeline::Deform {
                slot: s,
                attachment,
                keys,
            } if *s == *slot && *attachment == args.attachment => Some(keys),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [keys] = tracks.as_slice() else {
        return Err("FFD track absent or ambiguous; no key was synthesized".into());
    };
    let key = keys.get(args.key_index).ok_or("Key index is absent")?;
    if args.start >= key.value.len() {
        return Err("Offset page is outside the stored key".into());
    }
    let end = args.start.saturating_add(args.count).min(key.value.len());
    Ok(
        json!({"revision":source.revision,"animation":args.animation,"skin":args.skin,"slot":args.slot,"attachment":args.attachment,"time":key.time,"key_index":args.key_index,"key_count":keys.len(),"interpolation":key.interp,"offset_pairs":key.value.len(),"expected_pairs":if mesh.weights.is_empty(){mesh.setup_vertices.len()}else{mesh.influence_count()},"start":args.start,"offsets":key.value[args.start..end],"next_start":(end<key.value.len()).then_some(end),"order":"rigid: vertex; weighted: vertex then existing ordered influence","document_changed":false}),
    )
}
pub fn read_definition() -> Value {
    json!({"name":"ankhimate_read_deform","description":"Read 1–32 stored native FFD offset pairs from an exact owned mesh/key. No interpolation, images, paths or synthesized keys. Read successive pages only when needed; use native weighted influence order.","parameters":{"type":"object","properties":{"animation":{"type":"string"},"skin":{"type":"string"},"slot":{"type":"string"},"attachment":{"type":"string"},"key_index":{"type":"integer","minimum":0},"start":{"type":"integer","minimum":0},"count":{"type":"integer","minimum":1,"maximum":32}},"required":["animation","skin","slot","attachment","key_index","start","count"],"additionalProperties":false}})
}

pub fn apply(
    doc: &mut Document,
    source: &Snapshot,
    action: &Action,
    index: usize,
) -> Result<(AnimationId, Value), Box<ToolError>> {
    let err = |cause: &str| {
        Box::new(ToolError {code:"unsafe_artistic_edit".into(),operation:action.operation().into(),action_index:Some(index),field:Some("target/time/values".into()),cause:cause.into(),correction:"Read exact identities and existing keys. Use an unlocked unique owned mesh, full native influence-order offsets (at most 512 pairs), and an existing clip/time. No links, topology or binds are edited.".into(),document_changed:false,revision:source.revision})
    };
    let (name, time) = match action {
        Action::DeformKey {
            animation, time, ..
        }
        | Action::DeformDelete {
            animation, time, ..
        }
        | Action::KeyDelete {
            animation, time, ..
        }
        | Action::EventAdd {
            animation, time, ..
        } => (animation, *time),
        _ => unreachable!(),
    };
    let clips = doc
        .animations
        .iter()
        .filter(|(_, a)| a.name == *name)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    let [id] = clips.as_slice() else {
        return Err(err("Animation is absent or ambiguous"));
    };
    let id = *id;
    if !time.is_finite() || time < 0. || time > doc.animations[id].duration {
        return Err(err("Time must be finite and within clip duration"));
    }
    let before =
        serde_json::to_value(&doc.animations[id]).map_err(|_| err("Cannot snapshot animation"))?;
    let mut mesh_guard = Value::Null;
    match action {
        Action::EventAdd { name, .. } => {
            if name.trim().is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
                return Err(err(
                    "Event name must contain 1–128 bytes without control characters",
                ));
            }
            if doc.animations[id].events.len() >= 256
                || doc.animations[id]
                    .events
                    .iter()
                    .any(|e| e.name == *name && (e.time - time).abs() < 1e-4)
            {
                return Err(err("Duplicate event or event budget exceeded"));
            }
            AddEvent::new(id, name, time).apply(doc);
        }
        Action::KeyDelete { bone, channel, .. } => {
            let bones = doc
                .skeleton
                .bones
                .iter()
                .filter(|(_, b)| b.name == *bone)
                .map(|(id, _)| id)
                .collect::<Vec<_>>();
            let [bone] = bones.as_slice() else {
                return Err(err("Bone is absent or ambiguous"));
            };
            if source.locked.contains(bone) {
                return Err(err("Bone is locked"));
            }
            let (property, axis) = match channel {
                Channel::TranslateX => (BoneProperty::Translate, Some(Axis::X)),
                Channel::TranslateY => (BoneProperty::Translate, Some(Axis::Y)),
                Channel::Rotate => (BoneProperty::Rotate, None),
                Channel::ScaleX => (BoneProperty::Scale, Some(Axis::X)),
                Channel::ScaleY => (BoneProperty::Scale, Some(Axis::Y)),
                Channel::ShearX => (BoneProperty::Shear, Some(Axis::X)),
                Channel::ShearY => (BoneProperty::Shear, Some(Axis::Y)),
            };
            let addr = TimelineAddr::Bone {
                bone: *bone,
                property,
                axis,
            };
            delete_at(doc, id, addr, time).map_err(&err)?;
        }
        Action::DeformKey {
            skin,
            slot,
            attachment,
            ..
        }
        | Action::DeformDelete {
            skin,
            slot,
            attachment,
            ..
        } => {
            if source.setup {
                return Err(err("Switch to Animate before proposing FFD keys"));
            }
            let slots = doc
                .skeleton
                .slots
                .iter()
                .filter(|(_, s)| s.name == *slot)
                .map(|(id, _)| id)
                .collect::<Vec<_>>();
            let [sl] = slots.as_slice() else {
                return Err(err("Slot is absent or ambiguous"));
            };
            let sl = *sl;
            let sk =
                ankhimate_document::ffd_keys::owner(doc, sl, attachment).map_err(|e| err(&e))?;
            if doc.skeleton.skins[sk].name != *skin {
                return Err(err("Skin does not own this attachment"));
            }
            let Some(Attachment::Mesh(m)) = doc.skeleton.skins[sk].get(sl, attachment) else {
                return Err(err("Attachment is not a mesh"));
            };
            let mut binds = m
                .inverse_bind_matrices
                .iter()
                .map(|(bone, b)| (bone, [b.a, b.b, b.c, b.d, b.tx, b.ty]))
                .collect::<Vec<_>>();
            binds.sort_by_key(|(bone, _)| **bone);
            mesh_guard = json!({"mesh":m,"binds":binds,"slot_bone":doc.skeleton.slots[sl].bone});
            if m.linked.is_some() || !m.ffd_keyframes.is_empty() || doc.skeleton.skins.values().flat_map(|s|s.entries.values()).any(|a|matches!(a,Attachment::Mesh(other) if other.linked.as_ref().is_some_and(|l|l.slot==*slot&&l.attachment==*attachment&&(l.skin.is_none()||l.skin.as_deref()==Some(skin))))) {return Err(err("Linked/shared ownership or inline FFD is not editable by this tool"))}
            if source.locked.contains(&doc.skeleton.slots[sl].bone)
                || m.weights.iter().flatten().any(|w| {
                    source.locked.contains(&w.bone) || !doc.skeleton.bones.contains_key(w.bone)
                })
            {
                return Err(err("Mesh references an absent or locked bone"));
            }
            let count = if m.weights.is_empty() {
                m.setup_vertices.len()
            } else {
                m.influence_count()
            };
            if count == 0
                || count > 512
                || m.uvs.len() != m.setup_vertices.len()
                || m.triangles.is_empty()
                || m.triangles
                    .iter()
                    .flatten()
                    .any(|i| *i as usize >= m.setup_vertices.len())
                || m.setup_vertices
                    .iter()
                    .chain(&m.uvs)
                    .any(|p| !p.is_finite())
            {
                return Err(err(
                    "Invalid topology or full offset array exceeds 512 pairs",
                ));
            }
            if !m.weights.is_empty()
                && (m.weights.len() != m.setup_vertices.len()
                    || m.weights.iter().any(|r| {
                        r.is_empty()
                            || r.iter().any(|w| !w.weight.is_finite() || w.weight <= 0.)
                            || (r.iter().map(|w| w.weight).sum::<f32>() - 1.).abs() > 1e-4
                    })
                    || m.weights.iter().flatten().any(|w| {
                        m.inverse_bind_matrices.get(&w.bone).is_none_or(|b| {
                            b.invert().is_none()
                                || ![b.a, b.b, b.c, b.d, b.tx, b.ty]
                                    .iter()
                                    .all(|v| v.is_finite())
                        })
                    }))
            {
                return Err(err(
                    "Weights and inverse binds must be valid before FFD authoring",
                ));
            }
            let tracks=doc.animations[id].timelines.iter().filter(|t|matches!(t,Timeline::Deform{slot,attachment:name,..} if *slot==sl && name==attachment)).collect::<Vec<_>>();
            if tracks.len()>1 || tracks.iter().any(|t|matches!(t,Timeline::Deform{keys,..} if keys.iter().any(|k| !k.time.is_finite() || k.time<0. || k.time>doc.animations[id].duration || k.value.len()!=count || k.value.iter().any(|v|!v.is_finite())))) {return Err(err("Existing FFD track is ambiguous, sparse or malformed; no implicit conversion"))}
            match action {
                Action::DeformKey { offsets, .. } => {
                    if offsets.len() != count
                        || offsets
                            .iter()
                            .flatten()
                            .any(|v| !v.is_finite() || v.abs() > 1e6)
                    {
                        return Err(err(
                            "Provide exactly one finite offset pair per native component, within +/-1e6",
                        ));
                    }
                    if tracks.iter().any(|t|matches!(t,Timeline::Deform{keys,..} if keys.len()>=64 && !keys.iter().any(|k|(k.time-time).abs()<1e-4))) {return Err(err("FFD key budget exceeded"))}
                    AddDeformKey::new(
                        id,
                        sl,
                        attachment,
                        time,
                        offsets.iter().map(|v| glam::vec2(v[0], v[1])).collect(),
                    )
                    .apply(doc);
                }
                Action::DeformDelete { .. } => delete_at(
                    doc,
                    id,
                    TimelineAddr::Deform {
                        slot: sl,
                        attachment: attachment.clone(),
                        owner: Some(sk),
                    },
                    time,
                )
                .map_err(err)?,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok((
        id,
        json!({"op":action.operation(),"target":action,"mesh_guard":mesh_guard,"before":before,"after":doc.animations[id],"warning":"Native animation data only. Full FFD arrays use native vertex/influence order. Runtime admission remains separate; no artistic quality is guaranteed."}),
    ))
}

fn delete_at(
    doc: &mut Document,
    id: AnimationId,
    addr: TimelineAddr,
    time: f32,
) -> Result<(), &'static str> {
    let tracks = doc.animations[id]
        .timelines
        .iter()
        .filter(|t| addr.matches_timeline(t))
        .collect::<Vec<_>>();
    let [track] = tracks.as_slice() else {
        return Err("Key track is absent or ambiguous");
    };
    let times = match track {
        Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneRotate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => keys.iter().map(|k| k.time).collect::<Vec<_>>(),
        Timeline::Deform { keys, .. } => keys.iter().map(|k| k.time).collect(),
        _ => return Err("Unsupported key type"),
    };
    let indices = times
        .iter()
        .enumerate()
        .filter(|(_, t)| (**t - time).abs() < 1e-4)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let [index] = indices.as_slice() else {
        return Err("Exactly one key at the requested native time is required");
    };
    DeleteKeys::new(
        id,
        vec![KeyRef {
            addr,
            index: *index,
        }],
    )
    .apply(doc);
    Ok(())
}

pub fn definitions() -> Vec<Value> {
    let mut ops = Vec::new();
    for op in ["deform.key", "deform.key_delete", "key.delete", "event.add"] {
        let mut p = json!({"op":{"const":op},"animation":{"type":"string"},"time":{"type":"number","minimum":0}});
        let mut required = vec!["op", "animation", "time"];
        if op.starts_with("deform.") {
            for name in ["skin", "slot", "attachment"] {
                p[name] = json!({"type":"string"});
                required.push(name);
            }
        }
        if op == "deform.key" {
            p["offsets"] = json!({"type":"array","minItems":1,"maxItems":512,"items":{"type":"array","minItems":2,"maxItems":2,"items":{"type":"number","minimum":-1e6,"maximum":1e6}},"description":"Full native offset array: rigid vertex order; weighted vertex then ordered influence. No sparse arrays or implicit topology changes."});
            required.push("offsets");
        }
        if op == "key.delete" {
            p["bone"] = json!({"type":"string"});
            p["channel"] = json!({"enum":["translate_x","translate_y","rotate","scale_x","scale_y","shear_x","shear_y"]});
            required.extend(["bone", "channel"]);
        }
        if op == "event.add" {
            p["name"] = json!({"type":"string","minLength":1,"maxLength":128});
            required.push("name");
        }
        ops.push(json!({"type":"object","properties":p,"required":required,"additionalProperties":false}));
    }
    ops
}
