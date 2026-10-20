//! Read-only author-document diagnostics. Publication remains preflight's job.
use crate::{
    Document,
    commands::key_cmds::{BoneProperty, KeyRef, TimelineAddr},
};
use ankhimate_core::{
    attachment::Attachment,
    constraints::{Constraint, IkConstraint},
    ids::{AnimationId, BoneId, ConstraintId, SkinId, SlotId},
    skeleton::Skeleton,
};
pub use ankhimate_export::preflight::Severity;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Document,
    Bone(BoneId),
    Slot(SlotId),
    Attachment {
        skin: SkinId,
        slot: SlotId,
        name: String,
    },
    Skin(SkinId),
    Constraint(ConstraintId),
    Animation(AnimationId),
    Key {
        animation: AnimationId,
        key: KeyRef,
    },
    Asset(String),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Issue {
    pub code: String,
    pub severity: Severity,
    pub subject: String,
    pub message: String,
    pub location: Location,
}
impl Issue {
    pub fn navigable(&self) -> bool {
        !matches!(self.location, Location::Document | Location::Asset(_))
    }
}

/// The structural checks used by the T816 IK inspector, independent of pose.
/// Walks are bounded even for malformed in-memory parent cycles.
pub fn ik_structure(s: &Skeleton, ik: &IkConstraint) -> Result<(), String> {
    if ik.bones.is_empty() {
        return Err("Invalid IK: empty chain".into());
    }
    if !s.bones.contains_key(ik.target) {
        return Err("Invalid IK: missing target".into());
    }
    if ik.bones.contains(&ik.target) {
        return Err("Invalid IK: target belongs to the chain".into());
    }
    for (i, bone) in ik.bones.iter().enumerate() {
        if !s.bones.contains_key(*bone) {
            return Err("Invalid IK: missing chain bone".into());
        }
        if ik.bones[..i].contains(bone) {
            return Err("Invalid IK: repeated chain bone".into());
        }
        if i > 0 {
            let mut seen = BTreeSet::new();
            let mut parent = s.bones[*bone].parent;
            let mut found = false;
            while let Some(p) = parent {
                if !seen.insert(p) {
                    break;
                }
                if p == ik.bones[i - 1] {
                    found = true;
                    break;
                }
                parent = s.bones.get(p).and_then(|b| b.parent);
            }
            if !found {
                return Err(
                    "Invalid IK presentation: chain must be ordered ancestor to descendant".into(),
                );
            }
        }
    }
    Ok(())
}

/// Map a schema subject only when unambiguous; never guess across namespaces.
fn locate(doc: &Document, subject: &str) -> Location {
    let s = &doc.skeleton;
    let mut locations = Vec::new();
    for (id, b) in &s.bones {
        if b.name == subject {
            locations.push(Location::Bone(id));
        }
    }
    for (id, b) in &s.slots {
        if b.name == subject {
            locations.push(Location::Slot(id));
        }
    }
    for (id, b) in &s.skins {
        if b.name == subject {
            locations.push(Location::Skin(id));
        }
        for (slot, name) in b.entries.keys() {
            let slot_name = s.slots.get(*slot).map(|s| s.name.as_str()).unwrap_or("");
            if format!("{}/{}/{}", b.name, slot_name, name) == subject {
                locations.push(Location::Attachment {
                    skin: id,
                    slot: *slot,
                    name: name.clone(),
                });
            }
        }
    }
    for (id, b) in &s.constraints {
        if b.name() == subject {
            locations.push(Location::Constraint(id));
        }
    }
    for (id, b) in &doc.animations {
        if b.name == subject {
            locations.push(Location::Animation(id));
        }
    }
    for a in doc.assets.images.values() {
        if a.name == subject {
            locations.push(Location::Asset(a.name.clone()));
        }
    }
    if locations.len() == 1 {
        locations.remove(0)
    } else {
        Location::Document
    }
}

pub fn timeline_addr(t: &ankhimate_core::animation::Timeline) -> Option<TimelineAddr> {
    use ankhimate_core::animation::Timeline as T;
    Some(match t {
        T::BoneTranslate { bone, axis, .. } => TimelineAddr::Bone {
            bone: *bone,
            property: BoneProperty::Translate,
            axis: Some(*axis),
        },
        T::BoneRotate { bone, .. } => TimelineAddr::Bone {
            bone: *bone,
            property: BoneProperty::Rotate,
            axis: None,
        },
        T::BoneScale { bone, axis, .. } => TimelineAddr::Bone {
            bone: *bone,
            property: BoneProperty::Scale,
            axis: Some(*axis),
        },
        T::BoneShear { bone, axis, .. } => TimelineAddr::Bone {
            bone: *bone,
            property: BoneProperty::Shear,
            axis: Some(*axis),
        },
        T::SlotColor { slot, .. } => TimelineAddr::SlotColor { slot: *slot },
        T::SlotVisible { slot, .. } => TimelineAddr::SlotVisible { slot: *slot },
        T::SlotAttachment { slot, .. } => TimelineAddr::SlotAttachment { slot: *slot },
        _ => return None,
    })
}

pub fn inspect(doc: &Document) -> Vec<Issue> {
    use Severity::*;
    let s = &doc.skeleton;
    let project = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let base = ankhimate_export::preflight::inspect(
        &project,
        &doc.assets,
        ankhimate_export::preflight::Target::Document,
        None,
    );
    let mut out: Vec<_> = base
        .items
        .into_iter()
        .filter(|i| i.code != "key-outside-clip")
        .map(|i| Issue {
            location: locate(doc, &i.subject),
            code: i.code,
            severity: i.severity,
            subject: i.subject,
            message: i.message,
        })
        .collect();
    let mut push = |severity, code: &str, subject: String, message: String, location| {
        out.push(Issue {
            severity,
            code: code.into(),
            subject,
            message,
            location,
        })
    };
    if doc.meta.fps == 0 {
        push(
            Error,
            "invalid-fps",
            "Project".into(),
            "Timeline FPS must be positive".into(),
            Location::Document,
        );
    }
    // Names/schema cannot express a missing native ID if another object has an
    // empty name. Keep these native checks explicitly typed.
    for (id, b) in &s.bones {
        if !s.update_order.contains(&id) {
            push(
                Error,
                "bone-update-order",
                b.name.clone(),
                "Bone is absent from the evaluation order".into(),
                Location::Bone(id),
            );
        }
    }
    let mut update_seen = BTreeSet::new();
    if s.update_order
        .iter()
        .any(|id| !s.bones.contains_key(*id) || !update_seen.insert(*id))
    {
        push(
            Error,
            "invalid-update-order",
            "Bones".into(),
            "Evaluation order contains a missing or repeated bone".into(),
            Location::Document,
        );
    }
    let mut ordered = BTreeSet::new();
    for slot in &s.draw_order {
        if !ordered.insert(*slot) {
            push(
                Error,
                "duplicate-draw-order",
                "Draw order".into(),
                "A setup slot occurs more than once".into(),
                Location::Slot(*slot),
            );
        }
    }
    let mut used = BTreeSet::new();
    for (skin_id, skin) in &s.skins {
        if skin.entries.is_empty() && skin.bones.is_empty() && skin.constraints.is_empty() {
            push(
                Info,
                "empty-skin",
                skin.name.clone(),
                "Skin has no overrides or gated objects; default fallback remains valid".into(),
                Location::Skin(skin_id),
            );
        }
        for (slot_id, slot) in &s.slots {
            if let Some(name) = &slot.attachment
                && s.resolve(skin_id, slot_id, name).is_none()
                && s.skins
                    .values()
                    .any(|skin| skin.get(slot_id, name).is_some())
            {
                push(
                    Warning,
                    "attachment-fallback-missing",
                    slot.name.clone(),
                    format!(
                        "Setup attachment `{name}` cannot resolve in skin `{}` or the default fallback",
                        skin.name
                    ),
                    Location::Slot(slot_id),
                );
            }
        }
        for ((slot, name), a) in &skin.entries {
            let label = format!(
                "{}/{}/{}",
                skin.name,
                s.slots
                    .get(*slot)
                    .map(|s| s.name.as_str())
                    .unwrap_or("<missing slot>"),
                name
            );
            let loc = Location::Attachment {
                skin: skin_id,
                slot: *slot,
                name: name.clone(),
            };
            let (texture, seq) = match a {
                Attachment::Region(r) => (Some(&r.texture), r.sequence.as_ref()),
                Attachment::Mesh(m) => {
                    if m.linked.is_none() {
                        for (v, ws) in m.weights.iter().enumerate() {
                            if !ws.is_empty()
                                && ws.iter().all(|w| w.weight.is_finite() && w.weight >= 0.0)
                            {
                                let sum: f32 = ws.iter().map(|w| w.weight).sum();
                                if sum > 0.0 && (sum - 1.0).abs() > 0.001 {
                                    push(
                                        Warning,
                                        "weight-sum",
                                        label.clone(),
                                        format!(
                                            "Vertex {v}: weight sum {sum}; expected approximately 1 (no automatic normalization)"
                                        ),
                                        loc.clone(),
                                    );
                                }
                            }
                        }
                        let mut bad_bind = false;
                        for w in m.weights.iter().flatten().filter(|w| w.weight > 0.0) {
                            bad_bind |= m.inverse_bind_matrices.get(&w.bone).is_none_or(|b| {
                                ![b.a, b.b, b.c, b.d, b.tx, b.ty]
                                    .iter()
                                    .all(|v| v.is_finite())
                                    || b.invert().is_none()
                            });
                        }
                        if bad_bind {
                            push(Warning,"bind-fallback",label.clone(),"A positive influence has no usable derived bind; the native evaluator falls back to setup".into(),loc.clone());
                        }
                        if m.edges
                            .iter()
                            .flatten()
                            .any(|i| *i as usize >= m.setup_vertices.len())
                        {
                            push(
                                Error,
                                "invalid-mesh-edge",
                                label.clone(),
                                "Mesh edge references a missing vertex".into(),
                                loc.clone(),
                            );
                        }
                        let deform_len = if m.weights.is_empty() {
                            m.setup_vertices.len()
                        } else {
                            m.influence_count()
                        };
                        if m.ffd_keyframes.iter().any(|k| {
                            !k.time.is_finite()
                                || (!k.vertex_offsets.is_empty()
                                    && k.vertex_offsets.len() != deform_len)
                                || k.vertex_offsets.iter().any(|v| !v.is_finite())
                        }) {
                            push(
                                Error,
                                "invalid-inline-deform",
                                label.clone(),
                                "Inline FFD time, count or coordinates are invalid".into(),
                                loc.clone(),
                            );
                        }
                    }
                    (Some(&m.texture), m.sequence.as_ref())
                }
                Attachment::Clipping(c) => {
                    if c.vertices.iter().any(|v| !v.is_finite()) {
                        push(
                            Error,
                            "invalid-polygon",
                            label.clone(),
                            "Clipping coordinates must be finite".into(),
                            loc.clone(),
                        );
                    }
                    (None, None)
                }
                Attachment::Path(p) => {
                    if p.vertices.iter().any(|v| !v.is_finite()) {
                        push(
                            Error,
                            "invalid-path",
                            label.clone(),
                            "Path coordinates must be finite".into(),
                            loc.clone(),
                        );
                    }
                    (None, None)
                }
                _ => (None, None),
            };
            if let Some(t) = texture {
                used.insert(t.clone());
            }
            if let Some(seq) = seq {
                used.extend(seq.frames.iter().cloned());
            }
        }
    }
    for asset in doc.assets.images.values() {
        if !used.contains(&asset.name) {
            push(
                Info,
                "unused-image",
                asset.name.clone(),
                "No attachment or sequence references this library image; it is retained".into(),
                Location::Asset(asset.name.clone()),
            );
        }
    }
    for (id, c) in &s.constraints {
        if let Constraint::Ik(ik) = c
            && let Err(message) = ik_structure(s, ik)
        {
            push(
                Error,
                "invalid-ik-chain",
                c.name().into(),
                message,
                Location::Constraint(id),
            );
        }
    }
    for (id, clip) in &doc.animations {
        let mut addrs = Vec::new();
        for t in &clip.timelines {
            let addr = timeline_addr(t);
            if let Some(a) = &addr {
                if addrs.contains(a) {
                    push(
                        Error,
                        "duplicate-timeline",
                        clip.name.clone(),
                        "Several tracks address the same property; key navigation is unavailable"
                            .into(),
                        Location::Animation(id),
                    );
                }
                addrs.push(a.clone());
            }
            let value = serde_json::to_value(t).expect("native timeline serializes");
            let keys = value
                .as_object()
                .and_then(|v| v.values().next())
                .and_then(|v| v.get("keys"))
                .and_then(|v| v.as_array());
            if let Some(keys) = keys {
                for (index, k) in keys.iter().enumerate() {
                    if let Some(time) = k["time"].as_f64()
                        && (time < 0.0 || time > f64::from(clip.duration) + 0.000001)
                    {
                        let loc = addr
                            .clone()
                            .map(|addr| Location::Key {
                                animation: id,
                                key: KeyRef { addr, index },
                            })
                            .unwrap_or(Location::Animation(id));
                        let channel = match &addr {
                            Some(TimelineAddr::Bone {
                                bone,
                                property,
                                axis,
                            }) => format!(
                                "{} / {} {}",
                                s.bones
                                    .get(*bone)
                                    .map(|b| b.name.as_str())
                                    .unwrap_or("<missing bone>"),
                                property.label(),
                                axis.map(|a| a.label()).unwrap_or("")
                            ),
                            Some(TimelineAddr::SlotColor { slot })
                            | Some(TimelineAddr::SlotVisible { slot })
                            | Some(TimelineAddr::SlotAttachment { slot }) => format!(
                                "Slot {}",
                                s.slots
                                    .get(*slot)
                                    .map(|s| s.name.as_str())
                                    .unwrap_or("<missing slot>")
                            ),
                            None => "Track".into(),
                        };
                        push(
                            Warning,
                            "key-outside-clip",
                            clip.name.clone(),
                            format!(
                                "{channel}: key {index} at {time:.6} s is outside duration {}; retained",
                                clip.duration
                            ),
                            loc,
                        );
                    }
                }
            }
            use ankhimate_core::animation::Timeline as T;
            if let T::DrawOrder { keys } = t {
                for k in keys {
                    let mut seen = BTreeSet::new();
                    if k.value.iter().any(|(slot, _)| !seen.insert(*slot)) {
                        push(
                            Warning,
                            "duplicate-draw-offset",
                            clip.name.clone(),
                            "Repeated slot offset: native evaluation uses the first entry".into(),
                            Location::Animation(id),
                        );
                    }
                }
            }
            let wrong_kind = match t {
                T::IkMix { constraint, .. }
                | T::IkSoftness { constraint, .. }
                | T::IkBendDirection { constraint, .. } => {
                    !matches!(s.constraints.get(*constraint), Some(Constraint::Ik(_)))
                }
                T::TransformConstraintMix { constraint, .. } => !matches!(
                    s.constraints.get(*constraint),
                    Some(Constraint::Transform(_))
                ),
                _ => false,
            };
            if wrong_kind {
                push(Error,"invalid-track-address",clip.name.clone(),"Constraint timeline targets the wrong native constraint type or a missing constraint".into(),Location::Animation(id));
            }
        }
    }
    fn rank(s: Severity) -> u8 {
        match s {
            Error => 0,
            Warning => 1,
            Info => 2,
        }
    }
    out.sort_by(|a, b| {
        (
            rank(a.severity),
            &a.code,
            &a.subject,
            &a.message,
            format!("{:?}", a.location),
        )
            .cmp(&(
                rank(b.severity),
                &b.code,
                &b.subject,
                &b.message,
                format!("{:?}", b.location),
            ))
    });
    out.dedup();
    out
}
#[cfg(test)]
mod tests;
