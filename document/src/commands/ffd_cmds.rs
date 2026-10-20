//! One validated existing Deform key operation, never geometry or a new key.
use super::{EditCommand, key_cmds::TimelineAddr};
use crate::{Document, WorkMode};
use ankhimate_core::{animation::Timeline, ids::AnimationId};
pub struct EditFfdKey {
    pub(crate) animation: AnimationId,
    pub(crate) addr: TimelineAddr,
    pub(crate) track: usize,
    pub(crate) before: Timeline,
    pub(crate) after: Option<Timeline>,
    pub(crate) duration_before: f32,
    pub(crate) duration_after: f32,
    pub(crate) gesture: Option<u64>,
}
impl EditFfdKey {
    fn put(&self, d: &mut Document, value: Option<Timeline>, duration: f32) {
        let Some(a) = d.animations.get_mut(self.animation) else {
            return;
        };
        if let Some(i) = a
            .timelines
            .iter()
            .position(|t| self.addr.matches_timeline(t))
        {
            a.timelines.remove(i);
        }
        if let Some(t) = value {
            a.timelines.insert(self.track.min(a.timelines.len()), t);
        }
        a.duration = duration;
    }
}
impl EditCommand for EditFfdKey {
    fn apply(&mut self, d: &mut Document) {
        self.put(d, self.after.clone(), self.duration_after);
    }
    fn revert(&mut self, d: &mut Document) {
        self.put(d, Some(self.before.clone()), self.duration_before);
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Animate)
    }
    fn label(&self) -> &str {
        "Edit FFD Key"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn merge(&mut self, next: &dyn EditCommand) -> bool {
        let Some(n) = next.as_any().downcast_ref::<Self>() else {
            return false;
        };
        if self.gesture.is_none()
            || self.gesture != n.gesture
            || self.animation != n.animation
            || self.addr != n.addr
            || self.after.as_ref() != Some(&n.before)
            || self.duration_after != n.duration_before
        {
            return false;
        }
        self.after = n.after.clone();
        self.duration_after = n.duration_after;
        true
    }
}
