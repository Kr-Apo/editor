//! Opt-in, single-clip author sampling for ReplayV1. No application transport.
use crate::{
    animation::{Animation, Interp, Timeline},
    pose::{self, Pose},
    replay_clock::Progress,
    replay_physics::{
        ReplayContext, ReplayError, ReplayFrame, ReplayInputs, ReplayPhysicsInstance,
    },
    skeleton::Skeleton,
};
use std::collections::HashSet;

/// Conservative horizon: adjacent tick times remain distinguishable as f32.
/// Reset with a new epoch before exceeding this horizon; no time accumulation.
pub const MAX_SAMPLE_TICK: u64 = (1 << 23) - 1;

/// Same hold/clamp/wrap policy as native players, computed in f64 from the
/// integer tick, then rounded once for the existing f32 evaluator.
pub fn canonical_time(tick: u64, duration: f32, looping: bool) -> Result<f32, ReplayError> {
    if tick > MAX_SAMPLE_TICK || !duration.is_finite() || duration < 0.0 {
        return Err(ReplayError::Invalid("unrepresentable clip time"));
    }
    let time = tick as f64 / 120.0;
    Ok(if duration == 0.0 {
        0.0
    } else if looping {
        (time % f64::from(duration)) as f32
    } else {
        time.min(f64::from(duration)) as f32
    })
}

/// Owns both immutable author snapshots and the B3 controller. A sampler from
/// another rig cannot be substituted through this surface, even with equal IDs.
pub struct ReplayAnimationInstance {
    rig: Skeleton,
    clip: Animation,
    physics: ReplayPhysicsInstance,
}
impl ReplayAnimationInstance {
    pub fn new(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
    ) -> Result<Self, ReplayError> {
        Self::build(rig, clips, name, epoch, None)
    }
    pub fn with_skins(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
        events: &[crate::replay_skin::SkinEvent],
    ) -> Result<Self, ReplayError> {
        Self::build(rig, clips, name, epoch, Some(events))
    }
    fn build(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
        events: Option<&[crate::replay_skin::SkinEvent]>,
    ) -> Result<Self, ReplayError> {
        super::replay_physics::validate(rig)?;
        let mut matches = clips.iter().filter(|c| c.name == name);
        let clip = matches.next().ok_or(ReplayError::Invalid("unknown clip"))?;
        if matches.next().is_some() {
            return Err(ReplayError::Invalid("ambiguous clip"));
        }
        validate(rig, clip)?;
        let initial = inputs(rig, clip, 0)?;
        Ok(Self {
            rig: rig.clone(),
            clip: clip.clone(),
            physics: match events {
                Some(e) => ReplayPhysicsInstance::with_skins(rig, epoch, &initial, e)?,
                None => ReplayPhysicsInstance::new(rig, epoch, &initial)?,
            },
        })
    }
    pub fn skin_at(
        &self,
        tick: u64,
    ) -> Result<Option<crate::replay_skin::SkinSelection>, ReplayError> {
        self.physics.skin_at(tick)
    }
    pub fn context(&self) -> ReplayContext {
        self.physics.context()
    }
    pub fn clock(&self) -> crate::replay_clock::ClockSnapshot {
        self.physics.clock()
    }
    pub fn frame(&self) -> &ReplayFrame {
        self.physics.frame()
    }
    pub fn faulted(&self) -> bool {
        self.physics.faulted()
    }
    pub fn sample_tick(&self, tick: u64) -> Result<Pose, ReplayError> {
        let mut pose = sample(&self.rig, &self.clip, tick)?;
        if let Some(s) = self.skin_at(tick)? {
            let skin = self
                .rig
                .skins
                .iter()
                .find(|(_, k)| k.name == s.skin)
                .expect("validated skin")
                .0;
            pose::evaluate_skinned(
                &self.rig,
                &[(
                    &self.clip,
                    canonical_time(tick, self.clip.duration, self.clip.looping)?,
                    1.0,
                )],
                &[skin],
                &mut pose,
            );
            super::replay_physics::finite_pose(&pose)?;
        }
        Ok(pose)
    }
    pub fn reset(
        &mut self,
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
    ) -> Result<(), ReplayError> {
        if epoch <= self.clock().epoch {
            return Err(ReplayError::Invalid("epoch must increase"));
        }
        let next = Self::new(rig, clips, name, epoch)?;
        *self = next;
        Ok(())
    }
    pub fn request_tick(
        &mut self,
        c: &ReplayContext,
        tick: u64,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.request_time(c, tick, 120, observe)
    }
    /// Queue an exact target without sampling or advancing Physics.
    pub fn queue_time(
        &mut self,
        c: &ReplayContext,
        n: u64,
        d: u64,
    ) -> Result<Progress<ReplayError>, ReplayError> {
        if d != 0 && u128::from(n) * 120 / u128::from(d) > u128::from(MAX_SAMPLE_TICK) {
            return Err(ReplayError::Invalid("unrepresentable clip time"));
        }
        self.physics.queue_time(c, n, d)
    }
    pub fn request_time(
        &mut self,
        c: &ReplayContext,
        n: u64,
        d: u64,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        if d != 0 && u128::from(n) * 120 / u128::from(d) > u128::from(MAX_SAMPLE_TICK) {
            return Err(ReplayError::Invalid("unrepresentable clip time"));
        }
        let (rig, clip) = (&self.rig, &self.clip);
        self.physics.request_time(
            c,
            n,
            d,
            |t| inputs(rig, clip, t.index).map_err(|e| e.to_string()),
            observe,
        )
    }
    pub fn pump(
        &mut self,
        c: &ReplayContext,
        budget: u32,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        let (rig, clip) = (&self.rig, &self.clip);
        self.physics.pump(
            c,
            budget,
            |t| inputs(rig, clip, t.index).map_err(|e| e.to_string()),
            observe,
        )
    }
}
fn sample(rig: &Skeleton, clip: &Animation, tick: u64) -> Result<Pose, ReplayError> {
    let time = canonical_time(tick, clip.duration, clip.looping)?;
    let mut pose = Pose::new();
    // Physics-only, no numerical state: native evaluator leaves springs at rest.
    pose::evaluate(rig, &[(clip, time, 1.0)], &mut pose);
    super::replay_physics::finite_pose(&pose)?;
    Ok(pose)
}
fn inputs(rig: &Skeleton, clip: &Animation, tick: u64) -> Result<ReplayInputs, ReplayError> {
    let time = canonical_time(tick, clip.duration, clip.looping)?;
    let p = pose::animation_locals(rig, clip, time);
    super::replay_physics::finite_pose(&p)?;
    Ok(rig
        .update_order
        .iter()
        .map(|id| (*id, p.locals[*id]))
        .collect())
}
fn validate(rig: &Skeleton, clip: &Animation) -> Result<(), ReplayError> {
    canonical_time(0, clip.duration, clip.looping)?;
    if !clip.events.is_empty() {
        return Err(ReplayError::Invalid("B4 does not replay events"));
    }
    for skin in rig.skins.values() {
        for attachment in skin.entries.values() {
            use crate::attachment::Attachment;
            if matches!(attachment, Attachment::Region(r) if r.sequence.is_some())
                || matches!(attachment, Attachment::Mesh(m) if m.sequence.is_some())
            {
                return Err(ReplayError::Invalid(
                    "B4 does not sample attachment sequences",
                ));
            }
        }
    }
    // Baking an offset moves f32 key boundaries; parity of nonzero author
    // offsets versus baked native times is not qualified by this surface.
    if !clip.bone_offsets.is_empty() {
        return Err(ReplayError::Invalid("B4 requires unshifted bone timelines"));
    }
    let mut seen = HashSet::new();
    for t in &clip.timelines {
        let (bone, channel, keys) = match t {
            Timeline::BoneTranslate { bone, axis, keys } => (*bone, axis.index(), keys),
            Timeline::BoneRotate { bone, keys } => (*bone, 2, keys),
            Timeline::BoneScale { bone, axis, keys } => (*bone, 3 + axis.index(), keys),
            Timeline::BoneShear { bone, axis, keys } => (*bone, 5 + axis.index(), keys),
            _ => {
                return Err(ReplayError::Invalid(
                    "B4 supports bone transform channels only",
                ));
            }
        };
        if !rig.bones.contains_key(bone) || !seen.insert((bone, channel)) {
            return Err(ReplayError::Invalid("unknown bone or duplicate channel"));
        }
        let mut previous = f32::NEG_INFINITY;
        for k in keys {
            if !k.time.is_finite() || !k.value.is_finite() || k.time <= previous {
                return Err(ReplayError::Invalid("invalid/same-time keys"));
            }
            previous = k.time;
            if let Interp::Bezier {
                out_handle,
                in_handle,
            } = k.interp
                && (!out_handle.is_finite()
                    || !in_handle.is_finite()
                    || !(0.0..=1.0).contains(&out_handle.x)
                    || !(0.0..=1.0).contains(&in_handle.x))
            {
                return Err(ReplayError::Invalid("invalid Bezier handles"));
            }
        }
    }
    Ok(())
}
