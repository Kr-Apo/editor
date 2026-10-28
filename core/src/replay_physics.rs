//! Explicit B3 Physics-only instance. No transport or default-profile activation.
use crate::{
    constraints::{Constraint, PhysicsConstraint},
    ids::{BoneId, ConstraintId},
    math::Transform,
    physics::BoneSim,
    pose::{self, Pose},
    replay_clock::{ClockError, Progress, ReplayClock, ScheduledTick, TickResult},
    skeleton::Skeleton,
    transforms::Affine2,
};
use glam::Vec2;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct ReplayContext {
    identity: Arc<()>,
    epoch: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    Invalid(&'static str),
    Context,
    Faulted,
    Clock(ClockError),
    Input(String),
    Numerical,
}
impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ReplayError {}
impl From<ClockError> for ReplayError {
    fn from(e: ClockError) -> Self {
        Self::Clock(e)
    }
}
pub type ReplayInputs = Vec<(BoneId, Transform)>;
#[derive(Debug, Clone)]
pub struct ReplayRecord {
    pub constraint: ConstraintId,
    pub bone: BoneId,
    pub rotation: f32,
    pub position: Vec2,
    pub rotation_velocity: f32,
    pub position_velocity: Vec2,
    pub anchor: Vec2,
}
#[derive(Debug, Clone)]
pub struct ReplayFrame {
    pub tick: u64,
    pub pose: Pose,
    pub records: Vec<ReplayRecord>,
}
/// Detached numerical audit view; no restore or mutable-state access.
#[derive(Debug, Clone)]
pub struct ReplayNumericalState {
    pub active: Vec<ConstraintId>,
    pub records: Vec<ReplayRecord>,
}
#[derive(Clone)]
struct Record {
    sim: BoneSim,
    anchor: Vec2,
}
pub struct ReplayPhysicsInstance {
    rig: Skeleton,
    context: ReplayContext,
    clock: ReplayClock,
    records: HashMap<ConstraintId, Record>,
    frame: ReplayFrame,
    faulted: bool,
    active: Vec<ConstraintId>,
    calendar: Option<crate::replay_skin::SkinCalendar>,
}
impl ReplayPhysicsInstance {
    pub fn new(
        rig: &Skeleton,
        epoch: u64,
        initial: &[(BoneId, Transform)],
    ) -> Result<Self, ReplayError> {
        Self::build(rig, epoch, initial, None)
    }
    pub fn with_skins(
        rig: &Skeleton,
        epoch: u64,
        initial: &[(BoneId, Transform)],
        events: &[crate::replay_skin::SkinEvent],
    ) -> Result<Self, ReplayError> {
        Self::build(
            rig,
            epoch,
            initial,
            Some(crate::replay_skin::SkinCalendar::new(rig, events)?),
        )
    }
    fn build(
        rig: &Skeleton,
        epoch: u64,
        initial: &[(BoneId, Transform)],
        calendar: Option<crate::replay_skin::SkinCalendar>,
    ) -> Result<Self, ReplayError> {
        let active = match &calendar {
            Some(c) => c.at(0).active.clone(),
            None => validate(rig)?,
        };
        let mut clock = ReplayClock::new();
        clock.reset(epoch)?;
        let pose = prepare(rig, initial)?;
        let mut records = HashMap::new();
        for id in &active {
            let Constraint::Physics(p) = &rig.constraints[*id] else {
                unreachable!()
            };
            let (anchor, _) = parent(rig, &pose, p.bone)?;
            records.insert(
                *id,
                Record {
                    sim: BoneSim::default(),
                    anchor,
                },
            );
        }
        let mut s = Self {
            rig: rig.clone(),
            context: ReplayContext {
                identity: Arc::new(()),
                epoch,
            },
            clock,
            records,
            frame: ReplayFrame {
                tick: 0,
                pose,
                records: Vec::new(),
            },
            faulted: false,
            active,
            calendar,
        };
        s.frame.records = s.record_snapshots();
        Ok(s)
    }
    pub fn skin_at(
        &self,
        tick: u64,
    ) -> Result<Option<crate::replay_skin::SkinSelection>, ReplayError> {
        if tick > crate::replay_animation::MAX_SAMPLE_TICK {
            return Err(ReplayError::Invalid("skin query outside horizon"));
        }
        Ok(self.calendar.as_ref().map(|c| c.at(tick).clone()))
    }
    pub fn context(&self) -> ReplayContext {
        self.context.clone()
    }
    pub fn clock(&self) -> crate::replay_clock::ClockSnapshot {
        self.clock.snapshot()
    }
    pub fn frame(&self) -> &ReplayFrame {
        &self.frame
    }
    pub fn numerical_state(&self) -> ReplayNumericalState {
        let ids: Vec<_> = self
            .rig
            .constraint_order
            .iter()
            .filter(|id| self.records.contains_key(id))
            .copied()
            .collect();
        ReplayNumericalState {
            active: self.active.clone(),
            records: snapshots(&self.rig, &self.records, &ids),
        }
    }
    pub fn faulted(&self) -> bool {
        self.faulted
    }
    /// Replace a controller context, not the application's document lifecycle.
    pub fn reset(
        &mut self,
        rig: &Skeleton,
        epoch: u64,
        initial: &[(BoneId, Transform)],
    ) -> Result<(), ReplayError> {
        if epoch <= self.context.epoch {
            return Err(ReplayError::Clock(ClockError::InvalidEpoch));
        }
        let next = Self::new(rig, epoch, initial)?;
        *self = next;
        Ok(())
    }
    fn check(&self, c: &ReplayContext) -> Result<(), ReplayError> {
        if !Arc::ptr_eq(&c.identity, &self.context.identity) || c.epoch != self.context.epoch {
            return Err(ReplayError::Context);
        }
        if self.faulted {
            return Err(ReplayError::Faulted);
        }
        Ok(())
    }
    pub fn request_tick(
        &mut self,
        c: &ReplayContext,
        tick: u64,
        input: impl FnMut(ScheduledTick) -> Result<ReplayInputs, String>,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.request_time(c, tick, 120, input, observe)
    }
    /// Schedule without invoking inputs, Physics or observers. The B2 offered
    /// tick is cancelled before any numerical mutation; pump resumes that tick.
    pub fn queue_time(
        &mut self,
        c: &ReplayContext,
        n: u64,
        d: u64,
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.check(c)?;
        Ok(self
            .clock
            .request_time(c.epoch, n, d, |_| TickResult::Cancelled)?)
    }
    pub fn request_time(
        &mut self,
        c: &ReplayContext,
        n: u64,
        d: u64,
        mut input: impl FnMut(ScheduledTick) -> Result<ReplayInputs, String>,
        mut observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.check(c)?;
        let Self {
            clock,
            rig,
            records,
            frame,
            faulted,
            active,
            calendar,
            ..
        } = self;
        Ok(clock.request_time(c.epoch, n, d, |tick| {
            run(
                rig,
                records,
                frame,
                faulted,
                active,
                calendar.as_ref(),
                tick,
                &mut input,
                &mut observe,
            )
        })?)
    }
    pub fn pump(
        &mut self,
        c: &ReplayContext,
        budget: u32,
        mut input: impl FnMut(ScheduledTick) -> Result<ReplayInputs, String>,
        mut observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.check(c)?;
        let Self {
            clock,
            rig,
            records,
            frame,
            faulted,
            active,
            calendar,
            ..
        } = self;
        Ok(clock.pump(c.epoch, budget, |tick| {
            run(
                rig,
                records,
                frame,
                faulted,
                active,
                calendar.as_ref(),
                tick,
                &mut input,
                &mut observe,
            )
        })?)
    }
    fn record_snapshots(&self) -> Vec<ReplayRecord> {
        snapshots(&self.rig, &self.records, &self.active)
    }
}
fn snapshots(
    rig: &Skeleton,
    records: &HashMap<ConstraintId, Record>,
    active: &[ConstraintId],
) -> Vec<ReplayRecord> {
    active
        .iter()
        .map(|id| {
            let r = &records[id];
            let Constraint::Physics(p) = &rig.constraints[*id] else {
                unreachable!()
            };
            ReplayRecord {
                constraint: *id,
                bone: p.bone,
                rotation: r.sim.rotation,
                position: r.sim.position,
                rotation_velocity: r.sim.rotation_velocity,
                position_velocity: r.sim.position_velocity,
                anchor: r.anchor,
            }
        })
        .collect()
}
#[allow(clippy::too_many_arguments)]
fn run(
    rig: &Skeleton,
    records: &mut HashMap<ConstraintId, Record>,
    frame: &mut ReplayFrame,
    faulted: &mut bool,
    active: &mut Vec<ConstraintId>,
    calendar: Option<&crate::replay_skin::SkinCalendar>,
    tick: ScheduledTick,
    input: &mut impl FnMut(ScheduledTick) -> Result<ReplayInputs, String>,
    observe: &mut impl FnMut(&ReplayFrame),
) -> TickResult<ReplayError> {
    run_with(
        rig,
        records,
        frame,
        faulted,
        active,
        calendar,
        tick,
        input,
        observe,
        &mut integrate,
    )
}
fn candidate(
    records: &HashMap<ConstraintId, Record>,
    active: &[ConstraintId],
) -> (HashMap<ConstraintId, Record>, Vec<ConstraintId>) {
    (records.clone(), active.to_vec())
}
fn accept(
    records: &mut HashMap<ConstraintId, Record>,
    active: &mut Vec<ConstraintId>,
    frame: &mut ReplayFrame,
    next: (HashMap<ConstraintId, Record>, Vec<ConstraintId>),
    next_frame: ReplayFrame,
) {
    *records = next.0;
    *active = next.1;
    *frame = next_frame;
}
#[allow(clippy::too_many_arguments)]
fn run_with(
    rig: &Skeleton,
    records: &mut HashMap<ConstraintId, Record>,
    frame: &mut ReplayFrame,
    faulted: &mut bool,
    active: &mut Vec<ConstraintId>,
    calendar: Option<&crate::replay_skin::SkinCalendar>,
    tick: ScheduledTick,
    input: &mut impl FnMut(ScheduledTick) -> Result<ReplayInputs, String>,
    observe: &mut impl FnMut(&ReplayFrame),
    step: &mut impl FnMut(
        &Skeleton,
        &mut Pose,
        &PhysicsConstraint,
        &mut Record,
        Vec2,
        Affine2,
    ) -> Result<(), ReplayError>,
) -> TickResult<ReplayError> {
    // Pessimistic fault guard survives input/observer/solver unwind. Committed
    // numerical state is never borrowed mutably by the candidate computation.
    *faulted = true;
    let (mut next_records, mut next_active) = candidate(records, active);
    let result = (|| {
        let values = input(tick).map_err(ReplayError::Input)?;
        let mut pose = prepare(rig, &values)?;
        if let Some(calendar) = calendar {
            let next = &calendar.at(tick.index).active;
            if next != &next_active {
                next_records.retain(|id, _| next.contains(id));
                next_active.clone_from(next);
            }
        }
        for id in &next_active {
            let Constraint::Physics(p) = &rig.constraints[*id] else {
                unreachable!()
            };
            let (anchor, inverse) = parent(rig, &pose, p.bone)?;
            let r = next_records.entry(*id).or_insert_with(|| Record {
                sim: BoneSim::default(),
                anchor,
            });
            step(rig, &mut pose, p, r, anchor, inverse)?;
        }
        let next = ReplayFrame {
            tick: tick.index,
            pose,
            records: snapshots(rig, &next_records, &next_active),
        };
        observe(&next);
        Ok(next)
    })();
    match result {
        Ok(next) => {
            // No callback or fallible computation between this move and B2's
            // Complete->completed increment. The synchronous boundary is hidden.
            accept(records, active, frame, (next_records, next_active), next);
            *faulted = false;
            TickResult::Complete
        }
        Err(e) => TickResult::Failed(e), // Drop candidates; preserve accepted prefix.
    }
}
fn integrate(
    rig: &Skeleton,
    pose: &mut Pose,
    p: &PhysicsConstraint,
    r: &mut Record,
    anchor: Vec2,
    inverse: Affine2,
) -> Result<(), ReplayError> {
    let delta = inverse.transform_vector(anchor - r.anchor);
    let push = inverse.transform_vector(p.wind + p.gravity);
    r.anchor = anchor;
    r.sim
        .replay_step(delta, push, p.inertia, p.strength, p.damping, p.mass);
    if !r.sim.finite() || !delta.is_finite() || !push.is_finite() {
        return Err(ReplayError::Numerical);
    }
    pose::apply_physics_offsets(rig, pose, p, r.sim.rotation, r.sim.position);
    finite_pose(pose)
}
pub(crate) fn finite_pose(p: &Pose) -> Result<(), ReplayError> {
    for (_, t) in p.locals.iter() {
        if !t.position.is_finite()
            || !t.rotation.is_finite()
            || !t.scale.is_finite()
            || !t.shear.is_finite()
        {
            return Err(ReplayError::Numerical);
        }
    }
    for (_, w) in p.worlds.iter() {
        if ![w.a, w.b, w.c, w.d, w.tx, w.ty]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(ReplayError::Numerical);
        }
    }
    Ok(())
}
fn prepare(rig: &Skeleton, inputs: &[(BoneId, Transform)]) -> Result<Pose, ReplayError> {
    let mut pose = Pose::new();
    pose::evaluate(rig, &[], &mut pose);
    let mut seen = HashSet::new();
    for (id, t) in inputs {
        if !rig.bones.contains_key(*id) || !seen.insert(*id) {
            return Err(ReplayError::Invalid("unknown/duplicate input bone"));
        }
        pose.locals.insert(*id, *t);
    }
    pose::update_worlds(rig, &mut pose);
    finite_pose(&pose)?;
    Ok(pose)
}
fn parent(rig: &Skeleton, pose: &Pose, bone: BoneId) -> Result<(Vec2, Affine2), ReplayError> {
    let b = rig
        .bones
        .get(bone)
        .ok_or(ReplayError::Invalid("missing bone"))?;
    let world = b.parent.map(|p| pose.world(p)).unwrap_or(Affine2::IDENTITY);
    let inverse = world
        .invert()
        .ok_or(ReplayError::Invalid("singular parent"))?;
    if ![
        inverse.a, inverse.b, inverse.c, inverse.d, inverse.tx, inverse.ty,
    ]
    .iter()
    .all(|v| v.is_finite())
    {
        return Err(ReplayError::Numerical);
    }
    Ok((world.transform_point(Vec2::ZERO), inverse))
}
pub(crate) fn validate(rig: &Skeleton) -> Result<Vec<ConstraintId>, ReplayError> {
    if !rig.skins.contains_key(rig.default_skin) {
        return Err(ReplayError::Invalid("missing default skin"));
    }
    let mut seen = HashSet::new();
    for b in &rig.update_order {
        let bone = rig
            .bones
            .get(*b)
            .ok_or(ReplayError::Invalid("invalid bone order"))?;
        if !seen.insert(*b) || bone.parent.is_some_and(|p| !seen.contains(&p) || p == *b) {
            return Err(ReplayError::Invalid("invalid bone parent/order"));
        }
    }
    if seen.len() != rig.bones.len() {
        return Err(ReplayError::Invalid("incomplete bone order"));
    }
    let mut seen = HashSet::new();
    let mut names = HashSet::new();
    for id in &rig.constraint_order {
        if !seen.insert(*id) {
            return Err(ReplayError::Invalid("duplicate constraint order"));
        }
        let Some(Constraint::Physics(p)) = rig.constraints.get(*id) else {
            return Err(ReplayError::Invalid("B3 requires Physics-only constraints"));
        };
        if !names.insert(&p.name) {
            return Err(ReplayError::Invalid("duplicate constraint identity"));
        }
        if !rig.bones.contains_key(p.bone)
            || ![
                p.inertia,
                p.strength,
                p.damping,
                p.mass,
                p.mix,
                p.wind.x,
                p.wind.y,
                p.gravity.x,
                p.gravity.y,
            ]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(ReplayError::Invalid("invalid Physics reference/parameters"));
        }
    }
    if seen.len() != rig.constraints.len() {
        return Err(ReplayError::Invalid("incomplete constraint order"));
    }
    for skin in rig.skins.values() {
        if skin
            .constraints
            .iter()
            .any(|c| !rig.constraints.contains_key(*c))
            || skin.bones.iter().any(|b| !rig.bones.contains_key(*b))
        {
            return Err(ReplayError::Invalid("invalid skin member"));
        }
    }
    let skipped = rig.inactive_skin_members(&[rig.default_skin]).1;
    Ok(rig
        .constraint_order
        .iter()
        .filter(|id| !skipped.contains(id))
        .copied()
        .collect())
}

#[cfg(test)]
#[path = "replay_transaction_tests.rs"]
mod transaction_tests;
