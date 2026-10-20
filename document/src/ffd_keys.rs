//! Bounded existing Deform track reads and edits. No new FFD evaluator or format.
use crate::{
    Document,
    commands::{
        ffd_cmds::EditFfdKey,
        key_cmds::{KeyRef, TimelineAddr},
    },
};
use ankhimate_core::{
    animation::{Interp, Key, Timeline},
    attachment::Attachment,
    ids::{AnimationId, SkinId, SlotId},
};
use glam::Vec2;
#[derive(Clone, Debug, PartialEq)]
pub struct Capture {
    pub animation: AnimationId,
    pub key: KeyRef,
    pub keys: Vec<Key<Vec<Vec2>>>,
    pub mesh: serde_json::Value,
    pub track: usize,
    pub duration: f32,
}
#[derive(Clone, Copy)]
pub enum Action {
    Move(f32),
    Delete,
    Interpolation(Interp),
}
pub fn owner(d: &Document, slot: SlotId, name: &str) -> Result<SkinId, String> {
    let entries = d
        .skeleton
        .skins
        .iter()
        .filter(|(_, s)| s.get(slot, name).is_some())
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    if entries.len() != 1 {
        return Err(
            "FFD slot/attachment must have exactly one owning skin; overrides are ambiguous".into(),
        );
    }
    Ok(entries[0])
}
pub fn interp_valid(i: Interp) -> bool {
    match i {
        Interp::Bezier {
            out_handle,
            in_handle,
        } => [out_handle, in_handle]
            .iter()
            .all(|h| h.is_finite() && (0.0..=1.0).contains(&h.x)),
        _ => true,
    }
}
impl Capture {
    pub fn read(d: &Document, animation: AnimationId, key: KeyRef) -> Result<Self, String> {
        let TimelineAddr::Deform {
            slot,
            attachment,
            owner: Some(skin),
        } = &key.addr
        else {
            return Err("FFD owner is unresolved; choose one exact owned mesh".into());
        };
        if owner(d, *slot, attachment)? != *skin {
            return Err("FFD skin ownership changed".into());
        }
        let sl = d.skeleton.slots.get(*slot).ok_or("FFD slot is absent")?;
        if !d.skeleton.bones.contains_key(sl.bone) {
            return Err("FFD slot bone is absent".into());
        }
        let sk = d.skeleton.skins.get(*skin).ok_or("FFD skin is absent")?;
        let Some(Attachment::Mesh(m)) = sk.get(*slot, attachment) else {
            return Err("FFD target is not an owned mesh".into());
        };
        if m.linked.is_some() {
            return Err("Linked meshes cannot be edited through FFD keys".into());
        }
        for s in d.skeleton.skins.values() {
            for a in s.entries.values() {
                if let Attachment::Mesh(other) = a
                    && let Some(l) = &other.linked
                {
                    let source_skin = l
                        .skin
                        .as_deref()
                        .or_else(|| {
                            d.skeleton
                                .skins
                                .get(d.skeleton.default_skin)
                                .map(|s| s.name.as_str())
                        })
                        .ok_or("Linked source default skin is absent")?;
                    if l.slot == sl.name && l.attachment == *attachment && source_skin == sk.name {
                        return Err(
                            "Source of a linked mesh is shared and read-only for FFD key editing"
                                .into(),
                        );
                    }
                }
            }
        }
        let n = m.setup_vertices.len();
        if n < 3
            || m.uvs.len() != n
            || m.setup_vertices
                .iter()
                .chain(&m.uvs)
                .any(|v| !v.is_finite())
            || m.triangles.is_empty()
            || m.triangles.iter().flatten().any(|i| *i as usize >= n)
            || m.edges.iter().flatten().any(|i| *i as usize >= n)
        {
            return Err("FFD mesh topology/UVs are invalid".into());
        }
        if !m.weights.is_empty() && m.weights.len() != n {
            return Err("FFD weight rows do not match setup vertices".into());
        }
        for weights in &m.weights {
            let sum: f32 = weights.iter().map(|w| w.weight).sum();
            if !weights.is_empty() && (!sum.is_finite() || (sum - 1.).abs() > 0.001) {
                return Err("FFD influence weights must be normalized".into());
            }
            for w in weights {
                if !d.skeleton.bones.contains_key(w.bone)
                    || !w.weight.is_finite()
                    || w.weight < 0.
                    || w.weight > 1.
                {
                    return Err("FFD influence is invalid".into());
                }
                let b = m
                    .inverse_bind_matrices
                    .get(&w.bone)
                    .ok_or("FFD inverse bind is absent; no implicit rebind")?;
                if [b.a, b.b, b.c, b.d, b.tx, b.ty]
                    .iter()
                    .any(|f| !f.is_finite())
                    || b.invert().is_none()
                {
                    return Err("FFD inverse bind is invalid".into());
                }
            }
        }
        if m.inverse_bind_matrices.iter().any(|(id, b)| {
            !d.skeleton.bones.contains_key(*id)
                || [b.a, b.b, b.c, b.d, b.tx, b.ty]
                    .iter()
                    .any(|f| !f.is_finite())
                || b.invert().is_none()
        }) {
            return Err("FFD bind cache contains invalid matrices or bone references".into());
        }
        let count = if m.weights.is_empty() {
            n
        } else {
            m.influence_count()
        };
        if count == 0 {
            return Err("FFD mesh has no representable offset entries".into());
        }
        let a = d
            .animations
            .get(animation)
            .ok_or("FFD animation is absent")?;
        let tracks = a
            .timelines
            .iter()
            .enumerate()
            .filter(|(_, t)| key.addr.matches_timeline(t))
            .collect::<Vec<_>>();
        if tracks.len() != 1 {
            return Err("FFD track is absent or ambiguous".into());
        }
        let Timeline::Deform { keys, .. } = tracks[0].1 else {
            unreachable!()
        };
        if keys.get(key.index).is_none() {
            return Err("FFD key index is absent".into());
        }
        if !a.duration.is_finite()
            || a.duration < 0.
            || keys.iter().any(|k| {
                !k.time.is_finite()
                    || k.time < 0.
                    || !interp_valid(k.interp)
                    || (!k.value.is_empty() && k.value.len() != count)
                    || k.value.iter().any(|v| !v.is_finite())
            })
            || keys.windows(2).any(|k| k[0].time >= k[1].time)
        {
            return Err(
                "FFD keys have invalid times, interpolation or topology-sized offsets".into(),
            );
        }
        if m.ffd_keyframes.iter().any(|k| {
            !k.time.is_finite()
                || (!k.vertex_offsets.is_empty() && k.vertex_offsets.len() != count)
                || k.vertex_offsets.iter().any(|v| !v.is_finite())
        }) {
            return Err("FFD inline data is invalid; key editing does not repair it".into());
        }
        let mut binds = m
            .inverse_bind_matrices
            .iter()
            .map(|(id, b)| (*id, [b.a, b.b, b.c, b.d, b.tx, b.ty]))
            .collect::<Vec<_>>();
        binds.sort_by_key(|(id, _)| *id);
        Ok(Self {
            animation,
            key,
            keys: keys.clone(),
            mesh: serde_json::json!([m, binds, sl, sk.name, sk.bones, sk.constraints]),
            track: tracks[0].0,
            duration: a.duration,
        })
    }
    pub fn matches(&self, d: &Document) -> bool {
        Self::read(d, self.animation, self.key.clone()).as_ref() == Ok(self)
    }
    pub fn command(
        &self,
        d: &Document,
        action: Action,
        gesture: Option<u64>,
    ) -> Result<Option<EditFfdKey>, String> {
        if !self.matches(d) {
            return Err("FFD key capture is stale; select it again".into());
        }
        let mut keys = self.keys.clone();
        let mut duration = self.duration;
        match action {
            Action::Move(time) => {
                if !time.is_finite() || time < 0. {
                    return Err("FFD time must be finite and nonnegative".into());
                }
                if keys[self.key.index].time == time {
                    return Ok(None);
                }
                if keys
                    .iter()
                    .enumerate()
                    .any(|(i, k)| i != self.key.index && k.time == time)
                {
                    return Err("FFD destination time is occupied; replacement is refused".into());
                }
                keys[self.key.index].time = time;
                keys.sort_by(|a, b| a.time.total_cmp(&b.time));
                duration = duration.max(time);
            }
            Action::Delete => {
                keys.remove(self.key.index);
            }
            Action::Interpolation(i) => {
                if !interp_valid(i) {
                    return Err("Invalid FFD Bezier handles: X [0,1], finite Y".into());
                }
                if keys[self.key.index].interp == i {
                    return Ok(None);
                }
                keys[self.key.index].interp = i;
            }
        }
        if matches!(action, Action::Move(_)) {
            let mut staged = d.animations[self.animation].clone();
            if let Timeline::Deform { keys: target, .. } = &mut staged.timelines[self.track] {
                *target = keys.clone();
            }
            duration = duration.max(staged.content_duration());
        }
        let before = d.animations[self.animation].timelines[self.track].clone();
        let TimelineAddr::Deform {
            slot, attachment, ..
        } = &self.key.addr
        else {
            unreachable!()
        };
        let after = (!keys.is_empty()).then(|| Timeline::Deform {
            slot: *slot,
            attachment: attachment.clone(),
            keys,
        });
        Ok(Some(EditFfdKey {
            animation: self.animation,
            addr: self.key.addr.clone(),
            track: self.track,
            before,
            after,
            duration_before: self.duration,
            duration_after: duration,
            gesture,
        }))
    }
}
