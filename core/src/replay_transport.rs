//! Experimental headless transport. Existing players remain LegacyT839.
use crate::{
    animation::Animation,
    pose::Pose,
    replay_animation::{MAX_SAMPLE_TICK, ReplayAnimationInstance, canonical_time},
    replay_clock::{ClockSnapshot, Progress, RationalTime, Stop},
    replay_physics::{ReplayError, ReplayFrame},
    skeleton::Skeleton,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportState {
    Stopped,
    Playing,
    Paused,
    SeekPreview,
    Replaying,
    Faulted,
    Disposed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoseKind {
    Pure,
    Simulated,
}
#[derive(Debug, Clone)]
pub struct TransportContext {
    identity: Arc<()>,
    epoch: u64,
}
impl TransportContext {
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TransportSnapshot {
    pub state: TransportState,
    pub clip: String,
    pub clock: Option<ClockSnapshot>,
    pub position: Option<u64>,
    pub clip_time: Option<f32>,
    pub pose_kind: Option<PoseKind>,
    pub preview_time: Option<RationalTime>,
    pub resume_state: Option<TransportState>,
    pub replay_target: Option<u64>,
    pub replay_complete: bool,
}
struct Source {
    rig: Skeleton,
    clips: Vec<Animation>,
    skins: Option<Vec<crate::replay_skin::SkinEvent>>,
}
pub struct ReplayTransport {
    source: Option<Arc<Source>>,
    instance: Option<ReplayAnimationInstance>,
    clip: String,
    context: TransportContext,
    state: TransportState,
    resume_state: Option<TransportState>,
    preview: Option<ReplayFrame>,
    preview_time: Option<RationalTime>,
    replay_target: Option<u64>,
}
impl ReplayTransport {
    pub fn new(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
    ) -> Result<Self, ReplayError> {
        Self::candidate(
            Arc::new(Source {
                rig: rig.clone(),
                clips: clips.to_vec(),
                skins: None,
            }),
            name,
            epoch,
        )
    }
    pub fn with_skins(
        rig: &Skeleton,
        clips: &[Animation],
        name: &str,
        epoch: u64,
        events: &[crate::replay_skin::SkinEvent],
    ) -> Result<Self, ReplayError> {
        Self::candidate(
            Arc::new(Source {
                rig: rig.clone(),
                clips: clips.to_vec(),
                skins: Some(events.to_vec()),
            }),
            name,
            epoch,
        )
    }
    pub fn skin_at(
        &self,
        tick: u64,
    ) -> Result<Option<crate::replay_skin::SkinSelection>, ReplayError> {
        self.instance
            .as_ref()
            .ok_or(ReplayError::Invalid("disposed transport"))?
            .skin_at(tick)
    }
    fn candidate(source: Arc<Source>, name: &str, epoch: u64) -> Result<Self, ReplayError> {
        let instance = match &source.skins {
            Some(events) => ReplayAnimationInstance::with_skins(
                &source.rig,
                &source.clips,
                name,
                epoch,
                events,
            )?,
            None => ReplayAnimationInstance::new(&source.rig, &source.clips, name, epoch)?,
        };
        let preview = Some(ReplayFrame {
            tick: 0,
            pose: instance.frame().pose.clone(),
            records: vec![],
        });
        Ok(Self {
            source: Some(source),
            instance: Some(instance),
            clip: name.into(),
            context: TransportContext {
                identity: Arc::new(()),
                epoch,
            },
            state: TransportState::Stopped,
            resume_state: None,
            preview,
            preview_time: None,
            replay_target: None,
        })
    }
    pub fn context(&self) -> TransportContext {
        self.context.clone()
    }
    fn check(&self, c: &TransportContext) -> Result<(), ReplayError> {
        if self.state == TransportState::Disposed {
            return Err(ReplayError::Invalid("disposed transport"));
        }
        if !Arc::ptr_eq(&c.identity, &self.context.identity) || c.epoch != self.context.epoch {
            return Err(ReplayError::Context);
        }
        Ok(())
    }
    fn ready(&self, c: &TransportContext) -> Result<(), ReplayError> {
        self.check(c)?;
        if self.state == TransportState::Faulted {
            Err(ReplayError::Faulted)
        } else {
            Ok(())
        }
    }
    fn fresh(&self, name: &str, epoch: u64) -> Result<Self, ReplayError> {
        if epoch <= self.context.epoch {
            return Err(ReplayError::Invalid("epoch must increase"));
        }
        Self::candidate(
            self.source.as_ref().expect("live source").clone(),
            name,
            epoch,
        )
    }
    fn next_epoch(&self) -> Result<u64, ReplayError> {
        self.context
            .epoch
            .checked_add(1)
            .ok_or(ReplayError::Invalid("epoch overflow"))
    }
    pub fn frame(&self) -> Result<&ReplayFrame, ReplayError> {
        if self.state == TransportState::Disposed {
            return Err(ReplayError::Invalid("disposed transport"));
        }
        Ok(self
            .preview
            .as_ref()
            .unwrap_or_else(|| self.instance.as_ref().expect("live instance").frame()))
    }
    pub fn sample_tick(&self, tick: u64) -> Result<Pose, ReplayError> {
        self.instance
            .as_ref()
            .ok_or(ReplayError::Invalid("disposed transport"))?
            .sample_tick(tick)
    }
    /// Numerical frame, distinct from the optional pure seek preview.
    pub fn committed_frame(&self) -> Result<&ReplayFrame, ReplayError> {
        Ok(self
            .instance
            .as_ref()
            .ok_or(ReplayError::Invalid("disposed transport"))?
            .frame())
    }
    pub fn clip_time_at(&self, tick: u64) -> Result<f32, ReplayError> {
        let source = self
            .source
            .as_ref()
            .ok_or(ReplayError::Invalid("disposed transport"))?;
        let clip = source
            .clips
            .iter()
            .find(|a| a.name == self.clip)
            .expect("validated clip");
        canonical_time(tick, clip.duration, clip.looping)
    }
    pub fn snapshot(&self) -> TransportSnapshot {
        let clock = self.instance.as_ref().map(|i| i.clock());
        let position = self
            .preview
            .as_ref()
            .map(|p| p.tick)
            .or_else(|| clock.map(|c| c.completed));
        let clip_time = position.and_then(|n| {
            let source = self.source.as_ref()?;
            let a = source.clips.iter().find(|a| a.name == self.clip)?;
            canonical_time(n, a.duration, a.looping).ok()
        });
        TransportSnapshot {
            state: self.state,
            clip: self.clip.clone(),
            clock,
            position,
            clip_time,
            pose_kind: position.map(|_| {
                if self.preview.is_some() {
                    PoseKind::Pure
                } else {
                    PoseKind::Simulated
                }
            }),
            preview_time: self.preview_time,
            resume_state: self.resume_state,
            replay_target: self.replay_target,
            replay_complete: self.replay_target.is_some_and(|target| {
                clock.is_some_and(|c| c.completed == target && c.backlog == 0)
            }),
        }
    }
    pub fn play(
        &mut self,
        c: &TransportContext,
        name: &str,
        restart: bool,
    ) -> Result<(), ReplayError> {
        self.ready(c)?;
        if restart || name != self.clip {
            let mut next = self.fresh(name, self.next_epoch()?)?;
            next.state = TransportState::Playing;
            next.preview = None;
            *self = next;
            return Ok(());
        }
        match self.state {
            TransportState::SeekPreview => Err(ReplayError::Invalid(
                "seek requires replay_to or explicit restart at zero",
            )),
            TransportState::Replaying => Err(ReplayError::Invalid(
                "replay already pending; pump or restart explicitly",
            )),
            TransportState::Paused => self.resume(c),
            TransportState::Stopped | TransportState::Playing => {
                self.preview = None;
                self.state = TransportState::Playing;
                Ok(())
            }
            _ => Err(ReplayError::Invalid("cannot play")),
        }
    }
    pub fn pause(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.ready(c)?;
        match self.state {
            TransportState::Playing | TransportState::Replaying => {
                self.resume_state = Some(self.state);
                self.state = TransportState::Paused;
                Ok(())
            }
            TransportState::Paused => Ok(()),
            _ => Err(ReplayError::Invalid("cannot pause")),
        }
    }
    pub fn resume(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.ready(c)?;
        if self.state != TransportState::Paused {
            return Err(ReplayError::Invalid(
                "resume requires ordinary/replay pause; seek needs explicit replay",
            ));
        }
        self.state = self.resume_state.take().expect("paused continuation");
        if self.state == TransportState::Playing {
            self.replay_target = None;
        }
        Ok(())
    }
    pub fn stop(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.ready(c)?;
        *self = self.fresh(&self.clip, self.next_epoch()?)?;
        Ok(())
    }
    pub fn reset(&mut self, c: &TransportContext, epoch: u64) -> Result<(), ReplayError> {
        self.check(c)?;
        *self = self.fresh(&self.clip, epoch)?;
        Ok(())
    }
    pub fn seek_tick(&mut self, c: &TransportContext, tick: u64) -> Result<(), ReplayError> {
        self.seek_time(c, tick, 120)
    }
    pub fn seek_time(&mut self, c: &TransportContext, n: u64, d: u64) -> Result<(), ReplayError> {
        self.ready(c)?;
        let (time, tick) = target(n, d)?;
        let mut next = self.fresh(&self.clip, self.next_epoch()?)?;
        let pose = next.sample_tick(tick)?;
        next.preview = Some(ReplayFrame {
            tick,
            pose,
            records: vec![],
        });
        next.preview_time = Some(time);
        next.state = TransportState::SeekPreview;
        *self = next;
        Ok(())
    }
    /// Fresh origin only. Queue without integrating: the caller can pause before
    /// tick one, or pump one tick at a time for arbitrary cancellation granularity.
    pub fn replay_to(&mut self, c: &TransportContext, tick: u64) -> Result<(), ReplayError> {
        self.replay_to_time(c, tick, 120)
    }
    pub fn replay_to_time(
        &mut self,
        c: &TransportContext,
        n: u64,
        d: u64,
    ) -> Result<(), ReplayError> {
        self.ready(c)?;
        let (_, tick) = target(n, d)?;
        let mut next = self.fresh(&self.clip, self.next_epoch()?)?;
        let i = next.instance.as_mut().expect("candidate");
        i.queue_time(&i.context(), n, d)?;
        next.preview = None;
        next.replay_target = Some(tick);
        if tick == 0 {
            next.state = TransportState::Paused;
            next.resume_state = Some(TransportState::Playing);
        } else {
            next.state = TransportState::Replaying;
        }
        *self = next;
        Ok(())
    }
    fn finish(
        &mut self,
        old: TransportState,
        result: Result<Progress<ReplayError>, ReplayError>,
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.state = old;
        if self.instance.as_ref().expect("live").faulted() {
            self.state = TransportState::Faulted;
        }
        if let Ok(p) = &result {
            if matches!(p.stop, Stop::Failed(_)) {
                self.state = TransportState::Faulted;
            } else if old == TransportState::Replaying && p.state.backlog == 0 {
                self.state = TransportState::Paused;
                self.resume_state = Some(TransportState::Playing);
            }
        }
        result
    }
    pub fn request_tick(
        &mut self,
        c: &TransportContext,
        tick: u64,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.request_time(c, tick, 120, observe)
    }
    pub fn request_time(
        &mut self,
        c: &TransportContext,
        n: u64,
        d: u64,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.ready(c)?;
        if self.state != TransportState::Playing {
            return Err(ReplayError::Invalid("request requires playing"));
        }
        let old = self.state;
        self.state = TransportState::Faulted; // survives observer panic
        let i = self.instance.as_mut().expect("live");
        let result = i.request_time(&i.context(), n, d, observe);
        self.finish(old, result)
    }
    pub fn pump(
        &mut self,
        c: &TransportContext,
        budget: u32,
        observe: impl FnMut(&ReplayFrame),
    ) -> Result<Progress<ReplayError>, ReplayError> {
        self.ready(c)?;
        if !matches!(
            self.state,
            TransportState::Playing | TransportState::Replaying
        ) {
            return Err(ReplayError::Invalid("pump requires playing or replaying"));
        }
        let old = self.state;
        self.state = TransportState::Faulted;
        let i = self.instance.as_mut().expect("live");
        let result = i.pump(&i.context(), budget, observe);
        self.finish(old, result)
    }
    pub fn cross_fade(
        &self,
        c: &TransportContext,
        _name: &str,
        _duration: f32,
    ) -> Result<(), ReplayError> {
        self.ready(c)?;
        Err(ReplayError::Invalid("crossfade unsupported in B5A"))
    }
    pub fn set_skin(&self, c: &TransportContext, _name: &str) -> Result<(), ReplayError> {
        self.ready(c)?;
        Err(ReplayError::Invalid("skin changes unsupported in B5A"))
    }
    pub fn start_from_preview(&self, c: &TransportContext) -> Result<(), ReplayError> {
        self.ready(c)?;
        Err(ReplayError::Invalid(
            "nonzero rest-start unsupported; use replay_to or restart at zero",
        ))
    }
    pub fn dispose(&mut self, c: &TransportContext) -> Result<(), ReplayError> {
        self.check(c)?;
        self.instance = None;
        self.source = None;
        self.preview = None;
        self.preview_time = None;
        self.replay_target = None;
        self.resume_state = None;
        self.state = TransportState::Disposed;
        Ok(())
    }
}
fn target(n: u64, d: u64) -> Result<(RationalTime, u64), ReplayError> {
    let (time, tick) = RationalTime::checked(n, d)?;
    if tick > MAX_SAMPLE_TICK {
        return Err(ReplayError::Invalid("unrepresentable clip time"));
    }
    Ok((time, tick))
}
