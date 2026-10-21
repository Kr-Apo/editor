//! Explicit bone ownership, pure preparation and targeted native snapshots.
use super::{BoneProperty, TimelineAddr, mapped_time, numeric};
use crate::{
    Document, WorkMode,
    commands::{EditCommand, IdRemap},
};
use ankhimate_core::{
    animation::{Key, Timeline},
    ids::{AnimationId, BoneId},
};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedOffsetSummary {
    pub bones: Vec<(BoneId, String)>,
    pub tracks: usize,
    pub keys: usize,
    pub changed_keys: usize,
    pub frames: f64,
    pub fps: u32,
    pub offset: f32,
    pub duration: f32,
    pub before_range: Option<(f32, f32)>,
    pub after_range: Option<(f32, f32)>,
    pub clamped: usize,
    pub outside_duration: usize,
}
impl SelectedOffsetSummary {
    pub fn needs_ack(&self) -> bool {
        self.clamped > 0 || self.outside_duration > 0
    }
}
fn address(t: &Timeline) -> Option<TimelineAddr> {
    let (bone, property, axis) = match t {
        Timeline::BoneTranslate { bone, axis, .. } => (*bone, BoneProperty::Translate, Some(*axis)),
        Timeline::BoneRotate { bone, .. } => (*bone, BoneProperty::Rotate, None),
        Timeline::BoneScale { bone, axis, .. } => (*bone, BoneProperty::Scale, Some(*axis)),
        Timeline::BoneShear { bone, axis, .. } => (*bone, BoneProperty::Shear, Some(*axis)),
        _ => return None,
    };
    Some(TimelineAddr::Bone {
        bone,
        property,
        axis,
    })
}
fn keys_mut(t: &mut Timeline) -> &mut Vec<Key<f32>> {
    match t {
        Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneRotate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => keys,
        _ => unreachable!("only direct bone transformations are captured"),
    }
}
fn targets<'a>(
    doc: &'a Document,
    anim: AnimationId,
    bones: &[BoneId],
) -> Result<Vec<(usize, &'a Timeline)>, String> {
    if bones.is_empty() {
        return Err("Select at least one bone; empty selection never means the whole clip".into());
    }
    let set: HashSet<_> = bones.iter().copied().collect();
    if set.len() != bones.len() {
        return Err("Duplicate bone selection".into());
    }
    if bones.iter().any(|b| !doc.skeleton.bones.contains_key(*b)) {
        return Err("Selected bone no longer exists".into());
    }
    let clip = doc.animations.get(anim).ok_or("Clip no longer exists")?;
    let mut addresses = HashSet::new();
    let mut result = Vec::new();
    for (i, t) in clip.timelines.iter().enumerate() {
        if t.bone().is_some_and(|b| set.contains(&b)) {
            let addr = address(t).ok_or("Unsupported bone track")?;
            if !addresses.insert(addr) {
                return Err("Ambiguous duplicate bone track".into());
            }
            result.push((i, t));
        }
    }
    Ok(result)
}
/// A disposable preview. Not a command until revalidated against the live document.
pub struct PreparedBoneOffset {
    pub summary: SelectedOffsetSummary,
    anim: AnimationId,
    bones: Vec<BoneId>,
    tracks: Vec<(usize, Timeline, Timeline)>,
}
impl PreparedBoneOffset {
    /// No history, evaluation, selection expansion or document mutation.
    pub fn prepare(
        doc: &Document,
        anim: AnimationId,
        bones: &[BoneId],
        frames: f64,
    ) -> Result<Self, String> {
        use crate::commands::document_cmds::SetProjectFps;
        if !(SetProjectFps::MIN..=SetProjectFps::MAX).contains(&doc.meta.fps) {
            return Err("Set a valid project FPS (1–240)".into());
        }
        if !frames.is_finite() {
            return Err("Offset frames must be finite".into());
        }
        let offset = numeric::checked(frames / f64::from(doc.meta.fps), "Offset")?;
        let source = targets(doc, anim, bones)?;
        let duration = doc.animations[anim].duration;
        if !duration.is_finite() || duration < 0. {
            return Err("Invalid clip duration".into());
        }
        let mut summary = SelectedOffsetSummary {
            bones: bones
                .iter()
                .map(|b| (*b, doc.skeleton.bones[*b].name.clone()))
                .collect(),
            tracks: source.len(),
            keys: 0,
            changed_keys: 0,
            frames,
            fps: doc.meta.fps,
            offset,
            duration,
            before_range: None,
            after_range: None,
            clamped: 0,
            outside_duration: 0,
        };
        let mut tracks = Vec::with_capacity(source.len());
        for (i, t) in source {
            let before = numeric::times(t);
            if before.iter().any(|v| !v.is_finite() || *v < 0.)
                || before.windows(2).any(|w| w[0] >= w[1])
            {
                return Err(format!(
                    "Track {i}: source times must be finite, nonnegative and strictly increasing"
                ));
            }
            let mut after = t.clone();
            for k in keys_mut(&mut after) {
                let old = k.time;
                let raw = mapped_time(old, 1., offset);
                if !raw.is_finite() {
                    return Err(format!("Track {i}: time overflow"));
                }
                k.time = raw.max(0.);
                summary.keys += 1;
                summary.changed_keys += usize::from(k.time != old);
                summary.clamped += usize::from(raw < 0.);
                summary.outside_duration += usize::from(k.time > duration);
                numeric::range(&mut summary.before_range, old);
                numeric::range(&mut summary.after_range, k.time);
            }
            if numeric::times(&after).windows(2).any(|w| w[0] >= w[1]) {
                return Err(format!(
                    "Track {i}: collision after clamp or f32 rounding; no keys merged"
                ));
            }
            tracks.push((i, t.clone(), after));
        }
        Ok(Self {
            summary,
            anim,
            bones: bones.to_vec(),
            tracks,
        })
    }
    /// Cheap lock targets are exact bones with directly owned tracks (including empty tracks).
    pub fn affected_bones(&self) -> impl Iterator<Item = BoneId> + '_ {
        self.tracks.iter().filter_map(|(_, t, _)| t.bone())
    }
    /// Final, pure validation. Call immediately before native dispatch; UI additionally leases revision/identity/locks.
    pub fn into_command(self, doc: &Document) -> Result<Option<Box<dyn EditCommand>>, String> {
        let current = targets(doc, self.anim, &self.bones)?;
        if doc.meta.fps != self.summary.fps
            || doc.animations[self.anim].duration != self.summary.duration
            || current.len() != self.tracks.len()
            || current
                .iter()
                .zip(&self.tracks)
                .any(|((i, t), (j, b, _))| i != j || *t != b)
        {
            return Err("Target tracks/FPS/duration changed; prepare again".into());
        }
        if self.summary.changed_keys == 0 {
            return Ok(None);
        }
        Ok(Some(Box::new(OffsetBoneKeys {
            anim: self.anim,
            tracks: self.tracks,
        })))
    }
}
struct OffsetBoneKeys {
    anim: AnimationId,
    tracks: Vec<(usize, Timeline, Timeline)>,
}
impl EditCommand for OffsetBoneKeys {
    fn apply(&mut self, doc: &mut Document) {
        if let Some(a) = doc.animations.get_mut(self.anim) {
            for (i, _, after) in &self.tracks {
                a.timelines[*i] = after.clone();
            }
        }
    }
    fn revert(&mut self, doc: &mut Document) {
        if let Some(a) = doc.animations.get_mut(self.anim) {
            for (i, before, _) in &self.tracks {
                a.timelines[*i] = before.clone();
            }
        }
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Animate)
    }
    fn label(&self) -> &str {
        "Offset Selected Bone Keys"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn apply_remap(&mut self, remap: &IdRemap) {
        for (_, before, after) in &mut self.tracks {
            for t in [before, after] {
                if let Some(b) = t.bone() {
                    t.set_bone(remap.bone(b));
                }
            }
        }
    }
}
#[cfg(test)]
#[path = "selected_offset_tests.rs"]
mod tests;
