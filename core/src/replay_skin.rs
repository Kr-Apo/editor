//! Immutable, opt-in canonical skin calendar. No document or format changes.
use crate::{ids::ConstraintId, replay_physics::ReplayError, skeleton::Skeleton};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinEvent {
    pub tick: u64,
    pub skin: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinSelection {
    pub skin: String,
    /// Native constraint identities in author order; never inferred from bones.
    pub active: Vec<ConstraintId>,
}
pub(crate) struct SkinCalendar {
    events: Vec<(u64, SkinSelection)>,
}
impl SkinCalendar {
    pub(crate) fn new(rig: &Skeleton, events: &[SkinEvent]) -> Result<Self, ReplayError> {
        super::replay_physics::validate(rig)?;
        if events.first().map(|e| e.tick) != Some(0) {
            return Err(ReplayError::Invalid(
                "skin calendar must start at tick zero",
            ));
        }
        let mut names = std::collections::HashSet::new();
        for skin in rig.skins.values() {
            if !names.insert(&skin.name) {
                return Err(ReplayError::Invalid("ambiguous skin identity"));
            }
        }
        let mut resolved = Vec::new();
        let mut sets = std::collections::HashMap::new();
        for (id, skin) in rig.skins.iter() {
            let skipped = rig.inactive_skin_members(&[id]).1;
            sets.insert(
                skin.name.as_str(),
                rig.constraint_order
                    .iter()
                    .filter(|c| !skipped.contains(c))
                    .copied()
                    .collect::<Vec<_>>(),
            );
        }
        for e in events {
            if e.tick > super::replay_animation::MAX_SAMPLE_TICK
                || resolved.last().is_some_and(|(t, _)| *t >= e.tick)
            {
                return Err(ReplayError::Invalid(
                    "skin events must be strictly ordered within sample horizon",
                ));
            }
            let active = sets
                .get(e.skin.as_str())
                .ok_or(ReplayError::Invalid("unknown skin"))?;
            resolved.push((
                e.tick,
                SkinSelection {
                    skin: e.skin.clone(),
                    active: active.clone(),
                },
            ));
        }
        Ok(Self { events: resolved })
    }
    pub(crate) fn at(&self, tick: u64) -> &SkinSelection {
        &self.events[self.events.partition_point(|(t, _)| *t <= tick) - 1].1
    }
}
