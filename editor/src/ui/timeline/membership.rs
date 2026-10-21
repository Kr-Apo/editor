//! Ephemeral render lookup only: KeySelection remains the validation authority.
use ankhimate_document::commands::key_cmds::{KeyRef, TimelineAddr};
use std::collections::{HashSet, hash_map::RandomState};
use std::hash::BuildHasher;

// An algorithm choice, never a limit on the number of selected keys.
const LINEAR_MAX: usize = 64;
pub(super) struct Membership<S = RandomState> {
    index: Option<HashSet<(TimelineAddr, usize), S>>,
}
impl Membership {
    pub(super) fn new(selection: &[KeyRef]) -> Self {
        Self::with_hasher(selection, RandomState::new())
    }
}
impl<S: BuildHasher> Membership<S> {
    #[cfg(test)]
    pub(super) fn capacity(&self) -> usize {
        self.index.as_ref().map_or(0, |i| i.capacity())
    }
    fn with_hasher(selection: &[KeyRef], hasher: S) -> Self {
        let index = (selection.len() > LINEAR_MAX).then(|| {
            let mut set = HashSet::with_capacity_and_hasher(selection.len(), hasher);
            set.extend(selection.iter().map(|key| (key.addr.clone(), key.index)));
            set
        });
        Self { index }
    }
    pub(super) fn contains(&self, selection: &[KeyRef], key: &KeyRef) -> bool {
        match &self.index {
            Some(index) => index.contains(&(key.addr.clone(), key.index)),
            None => selection.contains(key),
        }
    }
}

#[cfg(test)]
mod tests;
