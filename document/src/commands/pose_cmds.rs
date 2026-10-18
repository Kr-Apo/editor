//! A prevalidated pose gesture, using the existing transform/key commands.
use super::{EditCommand, Group, IdRemap};
use crate::{WorkMode, doc::Document};
use ankhimate_core::ids::AnimationId;

/// Caller resolves and validates every destination and routes every child before
/// dispatch. Unlike a drag, a paste never merges with its predecessor/successor.
/// No child is pushed onto History separately and redo owns all captured values.
pub struct PastePose {
    group: Group,
    mode: WorkMode,
    animation: Option<AnimationId>,
    before_duration: Option<f32>,
}
impl PastePose {
    pub fn new(
        commands: Vec<Box<dyn EditCommand>>,
        mode: WorkMode,
        animation: Option<AnimationId>,
        mirrored: bool,
    ) -> Self {
        Self {
            group: Group::new(
                commands,
                if mirrored {
                    "Paste Mirrored Pose"
                } else {
                    "Paste Pose"
                },
            ),
            mode,
            animation,
            before_duration: None,
        }
    }
}
impl EditCommand for PastePose {
    fn apply(&mut self, doc: &mut Document) {
        if self.before_duration.is_none() {
            self.before_duration = self
                .animation
                .and_then(|id| doc.animations.get(id))
                .map(|a| a.duration);
        }
        self.group.apply(doc);
    }
    fn revert(&mut self, doc: &mut Document) {
        self.group.revert(doc);
        // AddKey snapshots timelines, but also grows duration. A complete paste
        // transaction must restore that metadata when undone beyond clip end.
        if let Some(a) = self.animation.and_then(|id| doc.animations.get_mut(id))
            && let Some(duration) = self.before_duration
        {
            a.duration = duration;
        }
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(self.mode)
    }
    fn label(&self) -> &str {
        self.group.label()
    }
    fn take_remap(&mut self) -> IdRemap {
        self.group.take_remap()
    }
    fn apply_remap(&mut self, remap: &IdRemap) {
        self.group.apply_remap(remap);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
