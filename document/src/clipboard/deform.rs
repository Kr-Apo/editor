//! Same-document, exact-owner FFD clipboard. Values remain native owned Vec2s.
use crate::{
    Document, EditCommand, WorkMode,
    commands::{
        ffd_cmds::EditFfdKey,
        key_cmds::{KeyRef, TimelineAddr},
    },
    ffd_keys::Capture,
};
use ankhimate_core::{
    animation::{Key, Timeline},
    attachment::{Attachment, MeshAttachment},
    ids::{AnimationId, BoneId, ConstraintId, SkinId},
    math::Transform,
    slot::Slot,
    transforms::Inherit,
};
use glam::Vec2;
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
struct SetupBone {
    id: BoneId,
    parent: Option<BoneId>,
    local: Transform,
    inherit: Inherit,
}
#[derive(Debug, Clone)]
struct Owner {
    addr: TimelineAddr,
    mesh: MeshAttachment,
    slot: Slot,
    skin_name: String,
    skin_bones: Vec<BoneId>,
    skin_constraints: Vec<ConstraintId>,
    default_skin: SkinId,
    setup: Vec<SetupBone>,
}
impl Owner {
    fn read(d: &Document, addr: &TimelineAddr) -> Result<Self, String> {
        let TimelineAddr::Deform {
            slot,
            attachment,
            owner: Some(skin),
        } = addr
        else {
            return Err("Resolve one exact FFD owner".into());
        };
        if crate::ffd_keys::owner(d, *slot, attachment)? != *skin {
            return Err("FFD owner changed".into());
        }
        let sl = d.skeleton.slots.get(*slot).ok_or("FFD slot missing")?;
        let sk = d.skeleton.skins.get(*skin).ok_or("FFD skin missing")?;
        let Some(Attachment::Mesh(m)) = sk.get(*slot, attachment) else {
            return Err("FFD target is not an owned mesh".into());
        };
        if m.linked.is_some() || m.sequence.is_some() {
            return Err("Linked/sequence FFD clipboard is unsupported".into());
        }
        if !d.skeleton.skins.contains_key(d.skeleton.default_skin)
            || d.skeleton
                .slots
                .values()
                .filter(|s| s.name == sl.name)
                .count()
                != 1
            || d.skeleton
                .skins
                .values()
                .filter(|s| s.name == sk.name)
                .count()
                != 1
        {
            return Err("FFD owner names/default skin are missing or ambiguous".into());
        }
        for s in d.skeleton.skins.values() {
            for a in s.entries.values() {
                if let Attachment::Mesh(other) = a
                    && let Some(l) = &other.linked
                    && l.slot == sl.name
                    && l.attachment == *attachment
                    && l.skin.as_deref().is_none_or(|name| name == sk.name)
                {
                    return Err("FFD source is shared by a linked mesh; paste is refused".into());
                }
            }
        }
        crate::retrace::topology(&m.setup_vertices, &m.uvs, &m.triangles)?;
        let mut influences = HashSet::new();
        for row in &m.weights {
            let unique: HashSet<_> = row.iter().map(|w| w.bone).collect();
            let sum: f32 = row.iter().map(|w| w.weight).sum();
            if row.len() > crate::retrace::MAX_INFLUENCES
                || unique.len() != row.len()
                || (!row.is_empty() && (!sum.is_finite() || (sum - 1.).abs() > 0.0001))
            {
                return Err("Invalid FFD influence structure/normalization".into());
            }
            influences.extend(unique);
        }
        if m.inverse_bind_matrices.len() != influences.len() {
            return Err("FFD bind coverage differs from its exact influences".into());
        }
        let mut setup = Vec::new();
        let mut seen = HashSet::new();
        for root in std::iter::once(sl.bone).chain(influences) {
            let mut current = Some(root);
            let mut path = HashSet::new();
            while let Some(id) = current {
                if !path.insert(id) {
                    return Err("Cyclic FFD Setup ancestry".into());
                }
                let bone = d.skeleton.bones.get(id).ok_or("FFD Setup bone missing")?;
                let t = bone.local_transform;
                if !t.position.is_finite()
                    || !t.rotation.is_finite()
                    || !t.scale.is_finite()
                    || !t.shear.is_finite()
                {
                    return Err("Invalid FFD Setup transform".into());
                }
                if seen.insert(id) {
                    setup.push(SetupBone {
                        id,
                        parent: bone.parent,
                        local: t,
                        inherit: bone.inherit,
                    });
                }
                current = bone.parent;
            }
        }
        setup.sort_by_key(|b| b.id);
        Ok(Self {
            addr: addr.clone(),
            mesh: m.clone(),
            slot: sl.clone(),
            skin_name: sk.name.clone(),
            skin_bones: sk.bones.clone(),
            skin_constraints: sk.constraints.clone(),
            default_skin: d.skeleton.default_skin,
            setup,
        })
    }
    fn matches(&self, d: &Document) -> bool {
        // Borrowed equality: no vector payload copying on a selection repaint.
        let TimelineAddr::Deform {
            slot,
            attachment,
            owner: Some(skin),
        } = &self.addr
        else {
            return false;
        };
        let Some(sk) = d.skeleton.skins.get(*skin) else {
            return false;
        };
        let Some(sl) = d.skeleton.slots.get(*slot) else {
            return false;
        };
        let Some(Attachment::Mesh(m)) = sk.get(*slot, attachment) else {
            return false;
        };
        let old = &self.mesh;
        d.skeleton.default_skin == self.default_skin
            && sk.name == self.skin_name
            && sk.bones == self.skin_bones
            && sk.constraints == self.skin_constraints
            && sl.name == self.slot.name
            && sl.bone == self.slot.bone
            && sl.attachment == self.slot.attachment
            && sl.color == self.slot.color
            && sl.dark_color == self.slot.dark_color
            && sl.blend_mode == self.slot.blend_mode
            && m.linked.is_none()
            && m.sequence.is_none()
            && m.texture == old.texture
            && m.setup_vertices == old.setup_vertices
            && m.uvs == old.uvs
            && m.triangles == old.triangles
            && m.edges == old.edges
            && m.inverse_bind_matrices == old.inverse_bind_matrices
            && m.weights.len() == old.weights.len()
            && m.weights.iter().zip(&old.weights).all(|(a, b)| {
                a.len() == b.len()
                    && a.iter()
                        .zip(b)
                        .all(|(a, b)| a.bone == b.bone && a.weight == b.weight)
            })
            && m.ffd_keyframes.len() == old.ffd_keyframes.len()
            && m.ffd_keyframes
                .iter()
                .zip(&old.ffd_keyframes)
                .all(|(a, b)| a.time == b.time && a.vertex_offsets == b.vertex_offsets)
            && self.setup.iter().all(|b| {
                d.skeleton.bones.get(b.id).is_some_and(|now| {
                    now.parent == b.parent
                        && now.local_transform == b.local
                        && now.inherit == b.inherit
                })
            })
    }
}
/// Immutable source data; no source clip ID, key index or borrowed value survives Copy.
#[derive(Debug, Clone)]
pub struct DeformClip {
    document: u64,
    owner: Owner,
    keys: Vec<Key<Vec<Vec2>>>,
}
impl DeformClip {
    pub fn capture(
        d: &Document,
        document: u64,
        anim: AnimationId,
        refs: &[KeyRef],
    ) -> Result<Self, String> {
        let first = refs.first().ok_or("Select FFD keys to copy")?;
        if refs.iter().any(|r| r.addr != first.addr)
            || !matches!(first.addr, TimelineAddr::Deform { owner: Some(_), .. })
        {
            return Err("Copy exactly one owned FFD track; mixed selections are refused".into());
        }
        let mut indices: Vec<_> = refs.iter().map(|r| r.index).collect();
        indices.sort_unstable();
        if indices.windows(2).any(|w| w[0] == w[1]) {
            return Err("Duplicate FFD key references".into());
        }
        let c = crate::ffd_keys::read_borrowed(d, anim, first)?;
        let owner = Owner::read(d, &first.addr)?;
        let keys = indices
            .iter()
            .map(|i| {
                c.keys
                    .get(*i)
                    .cloned()
                    .ok_or_else(|| "FFD key reference is stale".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            document,
            owner,
            keys,
        })
    }
    pub fn address(&self) -> &TimelineAddr {
        &self.owner.addr
    }
    pub fn keys(&self) -> &[Key<Vec<Vec2>>] {
        &self.keys
    }
    pub fn affected_bones(&self) -> Vec<BoneId> {
        std::iter::once(self.owner.slot.bone)
            .chain(self.owner.mesh.weights.iter().flatten().map(|w| w.bone))
            .collect()
    }
    /// For grouped shared selection leases, using indexed borrows instead of per-key captures.
    pub fn source_matches(
        &self,
        d: &Document,
        document: u64,
        anim: AnimationId,
        refs: &[KeyRef],
    ) -> bool {
        if document != self.document || !self.owner.matches(d) || refs.len() != self.keys.len() {
            return false;
        }
        let Some(a) = d.animations.get(anim) else {
            return false;
        };
        let mut tracks = a
            .timelines
            .iter()
            .filter(|t| self.address().matches_timeline(t));
        let Some(Timeline::Deform { keys, .. }) = tracks.next() else {
            return false;
        };
        tracks.next().is_none()
            && refs
                .iter()
                .zip(&self.keys)
                .all(|(r, k)| r.addr == *self.address() && keys.get(r.index) == Some(k))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PasteSummary {
    pub keys: usize,
    pub first: f32,
    pub last: f32,
    pub outside_duration: usize,
    pub duration: f32,
}
pub struct PreparedDeformPaste {
    pub summary: PasteSummary,
    source: Arc<DeformClip>,
    capture: Capture,
    after: Timeline,
}
impl PreparedDeformPaste {
    pub fn prepare(
        d: &Document,
        document: u64,
        anim: AnimationId,
        clip: Arc<DeformClip>,
        target: &TimelineAddr,
        playhead: f32,
    ) -> Result<Self, String> {
        if document != clip.document {
            return Err("FFD clipboard belongs to another document".into());
        }
        if target != clip.address() {
            return Err("FFD destination must be the exact same skin/slot/attachment owner".into());
        }
        // Full owner validation once per explicit preparation, including new sharing references.
        Owner::read(d, target)?;
        if !clip.owner.matches(d) {
            return Err("FFD topology/owner/Setup changed since Copy; copy again".into());
        }
        if !playhead.is_finite() || playhead < 0. {
            return Err("FFD playhead must be finite and nonnegative".into());
        }
        let c = Capture::read(
            d,
            anim,
            KeyRef {
                addr: target.clone(),
                index: 0,
            },
        )?;
        let first = clip.keys.first().ok_or("FFD clipboard is empty")?.time;
        let mut incoming = Vec::with_capacity(clip.keys.len());
        for k in &clip.keys {
            let raw = f64::from(playhead) + (f64::from(k.time) - f64::from(first));
            let time = raw as f32;
            if !raw.is_finite() || !time.is_finite() || (raw > 0. && time == 0.) || time < 0. {
                return Err("FFD paste time is not representable".into());
            }
            incoming.push(Key {
                time,
                value: k.value.clone(),
                interp: k.interp,
            });
        }
        if incoming.windows(2).any(|w| w[0].time >= w[1].time) {
            return Err("FFD pasted times collide after f32 rounding".into());
        }
        let summary = PasteSummary {
            keys: incoming.len(),
            first: incoming[0].time,
            last: incoming.last().expect("nonempty").time,
            outside_duration: incoming.iter().filter(|k| k.time > c.duration).count(),
            duration: c.duration,
        };
        // Linear sorted merge; equality refuses, never overwrites existing FFD values.
        let mut old = c.keys.iter().peekable();
        let mut new = incoming.into_iter().peekable();
        let mut keys = Vec::with_capacity(c.keys.len() + summary.keys);
        while let (Some(a), Some(b)) = (old.peek(), new.peek()) {
            if a.time == b.time {
                return Err("FFD destination time is occupied; replacement is refused".into());
            }
            if a.time < b.time {
                keys.push((*old.next().expect("peeked")).clone());
            } else {
                keys.push(new.next().expect("peeked"));
            }
        }
        keys.extend(old.cloned());
        keys.extend(new);
        let TimelineAddr::Deform {
            slot, attachment, ..
        } = target
        else {
            return Err("Not a Deform target".into());
        };
        let after = Timeline::Deform {
            slot: *slot,
            attachment: attachment.clone(),
            keys,
        };
        Ok(Self {
            summary,
            source: clip,
            capture: c,
            after,
        })
    }
    pub fn affected_bones(&self) -> Vec<BoneId> {
        self.source.affected_bones()
    }
    pub fn into_command(
        self,
        d: &Document,
        document: u64,
        clipboard: &Arc<DeformClip>,
    ) -> Result<Box<dyn EditCommand>, String> {
        if document != self.source.document
            || !Arc::ptr_eq(&self.source, clipboard)
            || !self.source.owner.matches(d)
            || !self.capture.matches(d)
        {
            return Err("FFD paste context/payload changed; prepare again".into());
        }
        Owner::read(d, self.source.address())?;
        let before = d.animations[self.capture.animation].timelines[self.capture.track].clone();
        Ok(Box::new(PasteDeformKeys(EditFfdKey {
            animation: self.capture.animation,
            addr: self.source.address().clone(),
            track: self.capture.track,
            before,
            after: Some(self.after),
            duration_before: self.capture.duration,
            duration_after: self.capture.duration,
            gesture: None,
        })))
    }
}
struct PasteDeformKeys(EditFfdKey);
impl EditCommand for PasteDeformKeys {
    fn apply(&mut self, d: &mut Document) {
        self.0.apply(d);
    }
    fn revert(&mut self, d: &mut Document) {
        self.0.revert(d);
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Animate)
    }
    fn label(&self) -> &str {
        "Paste FFD Keys"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
