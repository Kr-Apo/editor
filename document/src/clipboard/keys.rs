//! Owned dopesheet keys. No source animation id or key indices survive Copy.
use crate::{
    commands::key_cmds::{BoneProperty, KeyRef, TimelineAddr},
    doc::Document,
};
use ankhimate_core::{
    animation::{Axis, Interp, Key, Timeline},
    ids::AnimationId,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TrackKind {
    Bone(BoneProperty, Option<Axis>),
    Color,
    Attachment,
    Visible,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTrack {
    pub name: String,
    pub kind: TrackKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyData {
    Scalar(f32),
    Color([f32; 4]),
    Attachment(Option<String>),
    Visible(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopiedKey {
    pub track: NamedTrack,
    pub key: Key<KeyData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyClip {
    /// In source row order, then key order. Times are captured source times.
    pub keys: Vec<CopiedKey>,
}

/// Read-only rows have no editable TimelineAddr. Never use their UI placeholder.
pub fn address(t: &Timeline) -> Result<TimelineAddr, String> {
    Ok(match t {
        Timeline::BoneTranslate {bone,axis,..} => TimelineAddr::Bone {bone:*bone,property:BoneProperty::Translate,axis:Some(*axis)},
        Timeline::BoneRotate {bone,..} => TimelineAddr::Bone {bone:*bone,property:BoneProperty::Rotate,axis:None},
        Timeline::BoneScale {bone,axis,..} => TimelineAddr::Bone {bone:*bone,property:BoneProperty::Scale,axis:Some(*axis)},
        Timeline::BoneShear {bone,axis,..} => TimelineAddr::Bone {bone:*bone,property:BoneProperty::Shear,axis:Some(*axis)},
        Timeline::SlotColor {slot,..} => TimelineAddr::SlotColor {slot:*slot},
        Timeline::SlotAttachment {slot,..} => TimelineAddr::SlotAttachment {slot:*slot},
        Timeline::SlotVisible {slot,..} => TimelineAddr::SlotVisible {slot:*slot},
        _ => return Err("This timeline is read-only in the dopesheet: draw order, constraints and FFD cannot be copied/pasted here".into()),
    })
}

impl NamedTrack {
    pub fn capture(doc: &Document, addr: &TimelineAddr) -> Result<Self, String> {
        Self::capture_using(doc, addr, |named| named.resolve(doc).map(|_| ()))
    }

    fn capture_using(
        doc: &Document,
        addr: &TimelineAddr,
        resolve: impl FnOnce(&Self) -> Result<(), String>,
    ) -> Result<Self, String> {
        let (name, kind) = match addr {
            TimelineAddr::Deform { .. } => {
                return Err("FFD vectors are excluded from the general key clipboard".into());
            }
            TimelineAddr::Bone {
                bone,
                property,
                axis,
            } => {
                if (*property == BoneProperty::Rotate) != axis.is_none() {
                    return Err("Ambiguous bone property/axis".into());
                }
                (
                    doc.skeleton
                        .bones
                        .get(*bone)
                        .ok_or("Missing bone")?
                        .name
                        .clone(),
                    TrackKind::Bone(*property, *axis),
                )
            }
            TimelineAddr::SlotColor { slot } => (
                doc.skeleton
                    .slots
                    .get(*slot)
                    .ok_or("Missing slot")?
                    .name
                    .clone(),
                TrackKind::Color,
            ),
            TimelineAddr::SlotAttachment { slot } => (
                doc.skeleton
                    .slots
                    .get(*slot)
                    .ok_or("Missing slot")?
                    .name
                    .clone(),
                TrackKind::Attachment,
            ),
            TimelineAddr::SlotVisible { slot } => (
                doc.skeleton
                    .slots
                    .get(*slot)
                    .ok_or("Missing slot")?
                    .name
                    .clone(),
                TrackKind::Visible,
            ),
        };
        let named = Self { name, kind };
        resolve(&named)?; // Reject duplicate names, even on the source rig.
        Ok(named)
    }

    pub fn resolve(&self, doc: &Document) -> Result<TimelineAddr, String> {
        if let TrackKind::Bone(property, axis) = self.kind {
            if (property == BoneProperty::Rotate) != axis.is_none() {
                return Err("Ambiguous bone property/axis".into());
            }
            let ids: Vec<_> = doc
                .skeleton
                .bones
                .iter()
                .filter(|(_, b)| b.name == self.name)
                .map(|(id, _)| id)
                .collect();
            if ids.len() != 1 {
                return Err(format!("Bone '{}' is missing or ambiguous", self.name));
            }
            Ok(TimelineAddr::Bone {
                bone: ids[0],
                property,
                axis,
            })
        } else {
            let ids: Vec<_> = doc
                .skeleton
                .slots
                .iter()
                .filter(|(_, s)| s.name == self.name)
                .map(|(id, _)| id)
                .collect();
            if ids.len() != 1 {
                return Err(format!("Slot '{}' is missing or ambiguous", self.name));
            }
            Ok(match self.kind {
                TrackKind::Color => TimelineAddr::SlotColor { slot: ids[0] },
                TrackKind::Attachment => TimelineAddr::SlotAttachment { slot: ids[0] },
                TrackKind::Visible => TimelineAddr::SlotVisible { slot: ids[0] },
                _ => unreachable!(),
            })
        }
    }
}

fn captured_key(t: &Timeline, index: usize) -> Result<Key<KeyData>, String> {
    fn own<T: Clone>(
        keys: &[Key<T>],
        index: usize,
        convert: fn(T) -> KeyData,
    ) -> Option<Key<KeyData>> {
        keys.get(index).map(|k| Key {
            time: k.time,
            value: convert(k.value.clone()),
            interp: k.interp,
        })
    }
    match t {
        Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneRotate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => own(keys, index, KeyData::Scalar),
        Timeline::SlotColor { keys, .. } => own(keys, index, KeyData::Color),
        Timeline::SlotAttachment { keys, .. } => own(keys, index, KeyData::Attachment),
        Timeline::SlotVisible { keys, .. } => own(keys, index, KeyData::Visible),
        _ => None,
    }
    .ok_or_else(|| "Missing key index or unsupported timeline".into())
}

impl KeyClip {
    /// Mirror bone animation OFFSETS only, never absolute setup/local poses.
    /// Keys retain their order, source times and normalized easing handles.
    pub fn mirrored_offsets(&self) -> Result<Self, String> {
        let mut mirrored = self.clone();
        for copied in &mut mirrored.keys {
            validate(&copied.track.kind, &copied.key)?;
            let negate = match copied.track.kind {
                TrackKind::Bone(BoneProperty::Translate, Some(Axis::X))
                | TrackKind::Bone(BoneProperty::Rotate, None)
                | TrackKind::Bone(BoneProperty::Shear, Some(_)) => true,
                TrackKind::Bone(BoneProperty::Translate, Some(Axis::Y))
                | TrackKind::Bone(BoneProperty::Scale, Some(_)) => false,
                _ => return Err("Mirrored offsets require bone transform tracks only (valid property/axis); slots, FFD, constraints, draw order and events are excluded".into()),
            };
            let KeyData::Scalar(value) = &mut copied.key.value else {
                return Err("Mirrored bone offsets require scalar keys".into());
            };
            if negate {
                *value = -*value;
            }
        }
        Ok(mirrored)
    }
    pub fn capture(doc: &Document, anim: AnimationId, refs: &[KeyRef]) -> Result<Self, String> {
        // Small requests retain the old allocation-free-of-indexes path.
        if refs.len() <= 16 {
            Self::capture_linear(doc, anim, refs)
        } else {
            capture_batch::capture(doc, anim, refs)
        }
    }

    fn capture_linear(doc: &Document, anim: AnimationId, refs: &[KeyRef]) -> Result<Self, String> {
        if refs.is_empty() {
            return Err("No selected keys".into());
        }
        let animation = doc
            .animations
            .get(anim)
            .ok_or("Source animation no longer exists")?;
        let mut keys = Vec::new();
        let mut seen = Vec::new();
        for r in refs {
            if seen.contains(r) {
                continue;
            }
            seen.push(r.clone());
            let timelines: Vec<_> = animation
                .timelines
                .iter()
                .filter(|t| r.addr.matches_timeline(t))
                .collect();
            if timelines.len() != 1 {
                return Err("Source track is missing or ambiguous".into());
            }
            let track = NamedTrack::capture(doc, &r.addr)?;
            let key = captured_key(timelines[0], r.index)?;
            validate(&track.kind, &key)?;
            keys.push(CopiedKey { track, key });
        }
        Ok(Self { keys })
    }

    /// Resolve everything before dispatch: one bad target refuses the whole block.
    /// Explicit destination rows correspond by row order and must retain kind/axis.
    pub fn resolve(
        &self,
        doc: &Document,
        anim: AnimationId,
        time: f32,
        destinations: &[TimelineAddr],
    ) -> Result<Vec<(TimelineAddr, Key<KeyData>)>, String> {
        if !time.is_finite() || time < 0.0 {
            return Err(
                "Paste time must be finite and non-negative; no clamping is performed".into(),
            );
        }
        let animation = doc
            .animations
            .get(anim)
            .ok_or("Destination animation no longer exists")?;
        let mut sources = Vec::new();
        for k in &self.keys {
            validate(&k.track.kind, &k.key)?;
            if !sources.contains(&k.track) {
                sources.push(k.track.clone());
            }
        }
        let first = self
            .keys
            .iter()
            .map(|k| k.key.time)
            .min_by(f32::total_cmp)
            .ok_or("Clipboard contains no keys")?;
        let mut targets = Vec::new();
        for addr in destinations {
            if !targets.contains(addr) {
                targets.push(addr.clone());
            }
        }
        if !targets.is_empty() && targets.len() != sources.len() {
            return Err("Selected destination row count does not match copied tracks".into());
        }
        let mut resolved = Vec::new();
        for (index, source) in sources.iter().enumerate() {
            let addr = if targets.is_empty() {
                source.resolve(doc)?
            } else {
                let target = NamedTrack::capture(doc, &targets[index])?;
                if target.kind != source.kind {
                    return Err(
                        "Destination row property/axis differs from source; paste refused".into(),
                    );
                }
                targets[index].clone()
            };
            let matches: Vec<_> = animation
                .timelines
                .iter()
                .filter(|t| addr.matches_timeline(t))
                .collect();
            if matches.len() > 1 {
                return Err("Destination track is ambiguous".into());
            }
            if let Some(t) = matches.first() {
                validate_existing_times(t)?;
            }
            resolved.push(addr);
        }
        let mut result: Vec<(TimelineAddr, Key<KeyData>)> = Vec::new();
        for copied in &self.keys {
            let index = sources.iter().position(|s| *s == copied.track).unwrap();
            let mut key = copied.key.clone();
            key.time = time + (key.time - first);
            if !key.time.is_finite() {
                return Err("Paste time overflow".into());
            }
            if result
                .iter()
                .any(|(addr, k)| *addr == resolved[index] && k.time == key.time)
            {
                return Err("Copied keys collapse onto the same time; paste refused".into());
            }
            if let KeyData::Attachment(Some(name)) = &key.value {
                let TimelineAddr::SlotAttachment { slot } = resolved[index] else {
                    unreachable!()
                };
                if !doc
                    .skeleton
                    .skins
                    .values()
                    .any(|skin| skin.get(slot, name).is_some())
                {
                    return Err(format!(
                        "Attachment '{name}' is missing on destination slot"
                    ));
                }
            }
            result.push((resolved[index].clone(), key));
        }
        Ok(result)
    }
}

fn validate(kind: &TrackKind, key: &Key<KeyData>) -> Result<(), String> {
    if !key.time.is_finite() || key.time < 0.0 {
        return Err("Invalid source key time".into());
    }
    let valid = match (kind, &key.value) {
        (TrackKind::Bone(_, _), KeyData::Scalar(v)) => v.is_finite(),
        (TrackKind::Color, KeyData::Color(v)) => v.iter().all(|v| v.is_finite()),
        (TrackKind::Attachment, KeyData::Attachment(_))
        | (TrackKind::Visible, KeyData::Visible(_)) => true,
        _ => false,
    };
    if !valid {
        return Err("Key value is invalid or not representable on its track".into());
    }
    if let Interp::Bezier {
        out_handle,
        in_handle,
    } = key.interp
        && (!out_handle.is_finite()
            || !in_handle.is_finite()
            || !(0.0..=1.0).contains(&out_handle.x)
            || !(0.0..=1.0).contains(&in_handle.x))
    {
        return Err("Invalid Bezier handles".into());
    }
    Ok(())
}

fn validate_existing_times(t: &Timeline) -> Result<(), String> {
    let times: Vec<_> = match t {
        Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneRotate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => keys.iter().map(|k| k.time).collect(),
        Timeline::SlotColor { keys, .. } => keys.iter().map(|k| k.time).collect(),
        Timeline::SlotAttachment { keys, .. } => keys.iter().map(|k| k.time).collect(),
        Timeline::SlotVisible { keys, .. } => keys.iter().map(|k| k.time).collect(),
        _ => return Err("Unsupported destination timeline".into()),
    };
    if times.iter().any(|t| !t.is_finite() || *t < 0.0) || times.windows(2).any(|t| t[0] >= t[1]) {
        return Err("Destination key times are invalid, unsorted or duplicated".into());
    }
    Ok(())
}

mod capture_batch;

#[cfg(test)]
mod t827_tests;
