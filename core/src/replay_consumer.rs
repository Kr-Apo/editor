//! Opt-in bone-only consumer boundary. No renderer, host clock or float-delta policy.
use crate::{
    animation::Animation,
    ids::BoneId,
    math::Transform,
    replay_clock::{Progress, RationalTime},
    replay_physics::{ReplayError, ReplayFrame, ReplayRecord},
    replay_skin::SkinEvent,
    replay_transport::{ReplayTransport, TransportContext, TransportSnapshot, TransportState},
    skeleton::Skeleton,
    transforms::Affine2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumerHealth {
    Healthy,
    Faulted,
}
#[derive(Debug, Clone)]
pub struct ConsumerBone {
    pub name: String,
    pub local: Transform,
    pub world: Affine2,
}
/// Owned, retainable bone/Physics snapshot. Request metadata is intent, not pose time.
#[derive(Debug, Clone)]
pub struct ConsumerFrame {
    pub context: TransportContext,
    pub epoch: u64,
    pub clip: String,
    pub requested: u64,
    pub completed: u64,
    pub backlog: u64,
    pub pose_tick: u64,
    pub pose_time: RationalTime,
    pub clip_time: f32,
    pub skin: String,
    pub state: TransportState,
    pub health: ConsumerHealth,
    pub target_reached: bool,
    pub bones: Vec<ConsumerBone>,
    pub records: Vec<ReplayRecord>,
}
#[derive(Debug, Clone)]
pub struct ConsumerPreview {
    pub context: TransportContext,
    pub epoch: u64,
    pub tick: u64,
    pub pose_time: RationalTime,
    pub clip_time: f32,
    pub skin: String,
    pub bones: Vec<ConsumerBone>,
}
pub struct ReplayConsumerBridge {
    transport: ReplayTransport,
    bones: Vec<(BoneId, String)>,
    default_skin: String,
}
impl ReplayConsumerBridge {
    pub fn new(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
        events: Option<&[SkinEvent]>,
    ) -> Result<Self, ReplayError> {
        let transport = match events {
            Some(e) => ReplayTransport::with_skins(rig, clips, name, epoch, e)?,
            None => ReplayTransport::new(rig, clips, name, epoch)?,
        };
        Ok(Self {
            transport,
            bones: rig
                .update_order
                .iter()
                .map(|b| (*b, rig.bones[*b].name.clone()))
                .collect(),
            default_skin: rig.skins[rig.default_skin].name.clone(),
        })
    }
    pub fn context(&self) -> TransportContext {
        self.transport.context()
    }
    pub fn transport_status(&self) -> TransportSnapshot {
        self.transport.snapshot()
    }
    fn skin(&self, tick: u64) -> Result<String, ReplayError> {
        Ok(self
            .transport
            .skin_at(tick)?
            .map(|s| s.skin)
            .unwrap_or_else(|| self.default_skin.clone()))
    }
    fn bones(&self, f: &ReplayFrame) -> Vec<ConsumerBone> {
        self.bones
            .iter()
            .map(|(id, name)| ConsumerBone {
                name: name.clone(),
                local: f.pose.locals[*id],
                world: f.pose.world(*id),
            })
            .collect()
    }
    pub fn current_frame(&self) -> Result<ConsumerFrame, ReplayError> {
        let s = self.transport.snapshot();
        let f = self.transport.committed_frame()?;
        let c = s
            .clock
            .ok_or(ReplayError::Invalid("no initialized frame"))?;
        if f.tick != c.completed {
            return Err(ReplayError::Invalid("incoherent committed frame"));
        }
        let health = if s.state == TransportState::Faulted {
            ConsumerHealth::Faulted
        } else {
            ConsumerHealth::Healthy
        };
        Ok(ConsumerFrame {
            context: self.context(),
            epoch: c.epoch,
            clip: s.clip,
            requested: c.requested,
            completed: c.completed,
            backlog: c.backlog,
            pose_tick: f.tick,
            pose_time: RationalTime::checked(f.tick, 120)?.0,
            clip_time: self.transport.clip_time_at(f.tick)?,
            skin: self.skin(f.tick)?,
            state: s.state,
            health,
            target_reached: c.backlog == 0 && health == ConsumerHealth::Healthy,
            bones: self.bones(f),
            records: f.records.clone(),
        })
    }
    /// Pure preview is deliberately not a committed consumer frame.
    pub fn preview_frame(&self) -> Result<Option<ConsumerPreview>, ReplayError> {
        if self.transport.snapshot().state != TransportState::SeekPreview {
            self.transport.committed_frame()?;
            return Ok(None);
        }
        let f = self.transport.frame()?;
        Ok(Some(ConsumerPreview {
            context: self.context(),
            epoch: self.context().epoch(),
            tick: f.tick,
            pose_time: RationalTime::checked(f.tick, 120)?.0,
            clip_time: self.transport.clip_time_at(f.tick)?,
            skin: self.skin(f.tick)?,
            bones: self.bones(f),
        }))
    }
    /// Presentation errors occur outside simulation and never fault or advance it.
    pub fn present(
        &self,
        consumer: impl FnOnce(&ConsumerFrame) -> Result<(), String>,
    ) -> Result<(), ReplayError> {
        consumer(&self.current_frame()?)
            .map_err(|e| ReplayError::Input(format!("presentation: {e}")))
    }
    pub fn play(
        &mut self,
        c: &TransportContext,
        name: &str,
        restart: bool,
    ) -> Result<(), ReplayError> {
        self.transport.play(c, name, restart)
    }
    pub fn pause(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.transport.pause(c)
    }
    pub fn resume(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.transport.resume(c)
    }
    pub fn stop(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.transport.stop(c)
    }
    pub fn reset(&mut self, c: &TransportContext, epoch: u64) -> Result<(), ReplayError> {
        self.transport.reset(c, epoch)
    }
    pub fn seek_tick(&mut self, c: &TransportContext, tick: u64) -> Result<(), ReplayError> {
        self.transport.seek_tick(c, tick)
    }
    pub fn seek_time(&mut self, c: &TransportContext, n: u64, d: u64) -> Result<(), ReplayError> {
        self.transport.seek_time(c, n, d)
    }
    pub fn replay_from_origin(
        &mut self,
        c: &TransportContext,
        tick: u64,
    ) -> Result<(), ReplayError> {
        self.transport.replay_to(c, tick)
    }
    /// Optional precommit audit keeps the existing B3 failure contract; it is NOT rendering.
    pub fn request_tick(
        &mut self,
        c: &TransportContext,
        tick: u64,
        audit: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.transport.request_tick(c, tick, audit)
    }
    pub fn request_time(
        &mut self,
        c: &TransportContext,
        n: u64,
        d: u64,
        audit: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.transport.request_time(c, n, d, audit)
    }
    pub fn pump(
        &mut self,
        c: &TransportContext,
        budget: u32,
        audit: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.transport.pump(c, budget, audit)
    }
    pub fn dispose(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.transport.dispose(c)?;
        self.bones.clear();
        self.default_skin.clear();
        Ok(())
    }
}
