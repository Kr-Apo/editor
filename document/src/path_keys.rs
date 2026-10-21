//! Checked PathPosition authoring, using existing generic key mutations on a disposable clip.
use crate::{
    Document, WorkMode,
    commands::{
        EditCommand,
        key_cmds::{AddKey, DeleteKeys, KeyRef, KeyValue, TimelineAddr},
    },
};
use ankhimate_core::{
    animation::{Animation, Interp, Key, Timeline},
    ids::{AnimationId, ConstraintId},
};
#[derive(Clone, Debug)]
pub struct Capture {
    pub animation: AnimationId,
    pub constraint: ConstraintId,
    pub keys: Vec<Key<f32>>,
    before: Animation,
    target: serde_json::Value,
}
#[derive(Clone, Debug)]
pub enum Action {
    Put {
        time: f32,
        value: f32,
        interp: Interp,
    },
    Replace {
        index: usize,
        time: f32,
        value: f32,
        interp: Interp,
    },
    Delete {
        index: usize,
    },
}
fn target(doc: &Document, id: ConstraintId) -> Result<serde_json::Value, String> {
    let p = ankhimate_core::path::position_target(&doc.skeleton, id)?;
    if doc
        .skeleton
        .constraints
        .values()
        .filter(|c| c.name() == p.name)
        .count()
        != 1
    {
        return Err("Path constraint name is ambiguous on disk".into());
    }
    let slot = &doc.skeleton.slots[p.slot];
    for start in p.bones.iter().chain(std::iter::once(&slot.bone)) {
        let mut cursor = Some(*start);
        while let Some(id) = cursor {
            let bone = &doc.skeleton.bones[id];
            if doc
                .skeleton
                .bones
                .values()
                .filter(|b| b.name == bone.name)
                .count()
                != 1
            {
                return Err("Path ancestor name is ambiguous on disk".into());
            }
            cursor = bone.parent;
        }
    }
    if doc
        .skeleton
        .slots
        .values()
        .filter(|s| s.name == slot.name)
        .count()
        != 1
        || doc
            .skeleton
            .skins
            .values()
            .filter(|s| s.name == doc.skeleton.skins[doc.skeleton.default_skin].name)
            .count()
            != 1
        || p.bones.iter().chain(std::iter::once(&slot.bone)).any(|id| {
            let name = &doc.skeleton.bones[*id].name;
            doc.skeleton
                .bones
                .values()
                .filter(|b| &b.name == name)
                .count()
                != 1
        })
    {
        return Err("Path skin, slot or bone name is ambiguous on disk".into());
    }
    let path = doc.skeleton.skins[doc.skeleton.default_skin]
        .get(
            p.slot,
            slot.attachment
                .as_deref()
                .ok_or("Missing Path attachment")?,
        )
        .ok_or("Missing Path")?;
    serde_json::to_value((
        p,
        slot,
        path,
        doc.skeleton.default_skin,
        &doc.skeleton.constraint_order,
    ))
    .map_err(|e| e.to_string())
}
fn key_valid(k: &Key<f32>) -> bool {
    k.time.is_finite()
        && k.time >= 0.
        && k.value.is_finite()
        && match k.interp {
            Interp::Bezier {
                out_handle,
                in_handle,
            } => {
                out_handle.is_finite()
                    && in_handle.is_finite()
                    && (0.0..=1.).contains(&out_handle.x)
                    && (0.0..=1.).contains(&in_handle.x)
            }
            _ => true,
        }
}
impl Capture {
    pub fn read(
        doc: &Document,
        animation: AnimationId,
        constraint: ConstraintId,
    ) -> Result<Self, String> {
        let target = target(doc, constraint)?;
        let before = doc
            .animations
            .get(animation)
            .ok_or("Animation missing")?
            .clone();
        if !before.duration.is_finite() || before.duration < 0. {
            return Err("Invalid animation duration".into());
        }
        let tracks: Vec<_> = before
            .timelines
            .iter()
            .filter_map(|t| {
                if let Timeline::PathPosition {
                    constraint: c,
                    keys,
                } = t
                {
                    (*c == constraint).then_some(keys)
                } else {
                    None
                }
            })
            .collect();
        if tracks.len() > 1 {
            return Err("Concurrent PathPosition tracks are ambiguous".into());
        }
        let keys = tracks.first().map_or_else(Vec::new, |k| (*k).clone());
        if keys.iter().any(|k| !key_valid(k)) || keys.windows(2).any(|k| k[0].time >= k[1].time) {
            return Err("Invalid PathPosition keys/times/curves".into());
        }
        Ok(Self {
            animation,
            constraint,
            keys,
            before,
            target,
        })
    }
    pub fn matches(&self, doc: &Document) -> bool {
        doc.animations.get(self.animation) == Some(&self.before)
            && target(doc, self.constraint).is_ok_and(|t| t == self.target)
    }
    pub fn command(&self, doc: &Document, action: Action) -> Result<Option<PathEdit>, String> {
        if !self.matches(doc) {
            return Err("Path key context changed; reselect the target".into());
        }
        let p = ankhimate_core::path::position_target(&doc.skeleton, self.constraint)?;
        if p.mix_translate == 0. && p.mix_rotate == 0. {
            return Err("Path constraint is disabled (both mixes are zero)".into());
        }
        let addr = TimelineAddr::PathPosition {
            constraint: self.constraint,
        };
        let mut staged = Document::new();
        let id = staged.animations.insert(self.before.clone());
        let put = match action {
            Action::Put {
                time,
                value,
                interp,
            } => Some((time, value, interp)),
            Action::Replace {
                index,
                time,
                value,
                interp,
            } => {
                self.keys.get(index).ok_or("Path key index missing")?;
                if self
                    .keys
                    .iter()
                    .enumerate()
                    .any(|(i, k)| i != index && k.time == time)
                {
                    return Err("Path key time collision".into());
                }
                DeleteKeys::new(
                    id,
                    vec![KeyRef {
                        addr: addr.clone(),
                        index,
                    }],
                )
                .apply(&mut staged);
                Some((time, value, interp))
            }
            Action::Delete { index } => {
                self.keys.get(index).ok_or("Path key index missing")?;
                DeleteKeys::new(
                    id,
                    vec![KeyRef {
                        addr: addr.clone(),
                        index,
                    }],
                )
                .apply(&mut staged);
                None
            }
        };
        if let Some((time, value, interp)) = put {
            if !key_valid(&Key {
                time,
                value,
                interp,
            }) {
                return Err("Path key must have finite time/value, nonnegative time and valid normalized Bezier handles".into());
            }
            if self.keys.is_empty() && time > 0. {
                AddKey::new(
                    id,
                    addr.clone(),
                    0.,
                    KeyValue::Scalar(p.position),
                    Interp::Linear,
                )
                .apply(&mut staged);
            }
            AddKey::new(id, addr, time, KeyValue::Scalar(value), interp).apply(&mut staged);
        }
        let after = staged.animations[id].clone();
        Ok((after != self.before).then_some(PathEdit {
            capture: self.clone(),
            after,
        }))
    }
}
pub struct PathEdit {
    capture: Capture,
    after: Animation,
}
impl EditCommand for PathEdit {
    fn apply(&mut self, doc: &mut Document) {
        if self.capture.matches(doc) {
            doc.animations[self.capture.animation] = self.after.clone();
        }
    }
    fn revert(&mut self, doc: &mut Document) {
        if doc.animations.get(self.capture.animation) == Some(&self.after) {
            doc.animations[self.capture.animation] = self.capture.before.clone();
        }
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Animate)
    }
    fn label(&self) -> &str {
        "Edit Path Position key"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
