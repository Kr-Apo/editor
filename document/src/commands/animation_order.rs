//! Adjacent author-order edits. Preparation refuses before touching History.
use super::EditCommand;
use crate::Document;
use ankhimate_core::ids::AnimationId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

pub struct MoveAnimation {
    before: Vec<AnimationId>,
    after: Vec<AnimationId>,
}
impl MoveAnimation {
    /// None is a boundary no-op; Err is an invalid native collection/target.
    /// Dispatch Some immediately after preparation under the caller's lease.
    pub fn prepare(
        doc: &Document,
        target: AnimationId,
        direction: Direction,
    ) -> Result<Option<Self>, String> {
        if !doc.animations.contains_key(target) {
            return Err("Animation no longer exists".into());
        }
        let mut seen = std::collections::HashSet::new();
        if doc
            .animation_order
            .iter()
            .any(|id| !doc.animations.contains_key(*id) || !seen.insert(*id))
        {
            return Err("Invalid author animation order; reopen a valid project".into());
        }
        let mut names = std::collections::HashSet::new();
        if doc.animations.values().any(|a| !names.insert(&a.name)) {
            return Err("Ambiguous animation names on disk".into());
        }
        let mut after = doc.animation_ids();
        let index = after
            .iter()
            .position(|id| *id == target)
            .ok_or("Missing animation")?;
        let other = match direction {
            Direction::Up => index.checked_sub(1),
            Direction::Down => index.checked_add(1).filter(|j| *j < after.len()),
        };
        let Some(other) = other else {
            return Ok(None);
        };
        after.swap(index, other);
        Ok(Some(Self {
            before: doc.animation_order.clone(),
            after,
        }))
    }
}
impl EditCommand for MoveAnimation {
    fn apply(&mut self, doc: &mut Document) {
        doc.animation_order.clone_from(&self.after);
    }
    fn revert(&mut self, doc: &mut Document) {
        doc.animation_order.clone_from(&self.before);
    }
    fn label(&self) -> &str {
        "Move Animation"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
