//! Targeted native command for adjacent Bezier segments.
use super::{EditCommand, IdRemap, key_cmds::TimelineAddr};
use crate::{Document, WorkMode};
use ankhimate_core::{animation::Timeline, ids::AnimationId};
/// One native timeline replacement: two segment Interps, one publication/Undo.
pub struct SetLinkedTangents {
    pub(crate) anim: AnimationId,
    pub(crate) addr: TimelineAddr,
    pub(crate) index: usize,
    pub(crate) before: Timeline,
    pub(crate) after: Timeline,
    pub(crate) gesture: Option<u64>,
}
impl SetLinkedTangents {
    fn put(&self, doc: &mut Document, value: &Timeline) {
        if let Some(a) = doc.animations.get_mut(self.anim)
            && let Some(t) = a
                .timelines
                .iter_mut()
                .find(|t| self.addr.matches_timeline(t))
        {
            *t = value.clone();
        }
    }
}
impl EditCommand for SetLinkedTangents {
    fn apply(&mut self, doc: &mut Document) {
        self.put(doc, &self.after);
    }
    fn revert(&mut self, doc: &mut Document) {
        self.put(doc, &self.before);
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Animate)
    }
    fn label(&self) -> &str {
        "Link Bezier Tangents"
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
            || self.anim != n.anim
            || self.addr != n.addr
            || self.index != n.index
            || self.after != n.before
        {
            return false;
        }
        self.after = n.after.clone();
        true
    }
    fn apply_remap(&mut self, r: &IdRemap) {
        if let TimelineAddr::Bone { bone, .. } = &mut self.addr {
            *bone = r.bone(*bone);
        }
        for t in [&mut self.before, &mut self.after] {
            if let Some(b) = t.bone() {
                t.set_bone(r.bone(b));
            }
        }
    }
}
