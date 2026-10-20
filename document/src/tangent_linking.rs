//! Editing geometry only; normalized storage and core sampling stay unchanged.
pub use crate::commands::tangent_cmds::SetLinkedTangents;
use crate::{Document, commands::key_cmds::TimelineAddr};
use ankhimate_core::{
    animation::{Interp, Key, Timeline},
    ids::AnimationId,
};
use glam::{DVec2, Vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Broken,
    Aligned,
    Mirrored,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    #[default]
    Incoming,
    Outgoing,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Junction {
    pub anim: AnimationId,
    pub addr: TimelineAddr,
    pub index: usize,
    timeline: Timeline,
    pub keys: [Key<f32>; 3],
}
fn scalar_keys(t: &Timeline) -> Result<Vec<Key<f32>>, String> {
    Ok(match t {
        Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneRotate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => keys.clone(),
        Timeline::SlotColor { keys, .. } => keys
            .iter()
            .map(|k| Key {
                time: k.time,
                value: k.value[3],
                interp: k.interp,
            })
            .collect(),
        _ => return Err("Linking requires an editable scalar graph channel".into()),
    })
}
fn handles(i: Interp) -> Result<(Vec2, Vec2), String> {
    match i {
        Interp::Bezier {
            out_handle,
            in_handle,
        } if [out_handle, in_handle]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(&v.x)) =>
        {
            Ok((out_handle, in_handle))
        }
        _ => Err(
            "Both adjacent segments must have valid Bezier handles; no automatic conversion".into(),
        ),
    }
}
impl Junction {
    pub fn read(
        doc: &Document,
        anim: AnimationId,
        addr: TimelineAddr,
        index: usize,
    ) -> Result<Self, String> {
        match &addr {
            TimelineAddr::Bone { bone, .. } if !doc.skeleton.bones.contains_key(*bone) => {
                return Err("Bone target is absent".into());
            }
            TimelineAddr::SlotColor { slot } if !doc.skeleton.slots.contains_key(*slot) => {
                return Err("Slot target is absent".into());
            }
            _ => {}
        }
        let a = doc.animations.get(anim).ok_or("Animation is absent")?;
        let mut tracks = a.timelines.iter().filter(|t| addr.matches_timeline(t));
        let t = tracks.next().ok_or("Track is absent")?;
        if tracks.next().is_some() {
            return Err("Track is ambiguous".into());
        }
        let keys = scalar_keys(t)?;
        if index == 0 || index + 1 >= keys.len() {
            return Err("Select one central key with two neighboring segments".into());
        }
        if keys
            .iter()
            .any(|k| !k.time.is_finite() || !k.value.is_finite())
            || keys.windows(2).any(|w| w[0].time >= w[1].time)
        {
            return Err("Keys must have finite values and strictly increasing times".into());
        }
        let keys = [
            keys[index - 1].clone(),
            keys[index].clone(),
            keys[index + 1].clone(),
        ];
        handles(keys[1].interp)?;
        handles(keys[2].interp)?;
        if matches!(t, Timeline::BoneRotate { .. })
            && keys
                .windows(2)
                .any(|w| (f64::from(w[1].value) - f64::from(w[0].value)).abs() >= 180.0)
        {
            return Err(
                "Rotation shortest-arc/wrap junctions are not qualified for linked geometry".into(),
            );
        }
        Ok(Self {
            anim,
            addr,
            index,
            timeline: t.clone(),
            keys,
        })
    }
    pub fn matches(&self, doc: &Document) -> bool {
        Self::read(doc, self.anim, self.addr.clone(), self.index).as_ref() == Ok(self)
    }
    fn spans(&self, side: Side) -> (f64, f64) {
        let n = if side == Side::Incoming { 0 } else { 1 };
        (
            f64::from(self.keys[n + 1].time) - f64::from(self.keys[n].time),
            f64::from(self.keys[n + 1].value) - f64::from(self.keys[n].value),
        )
    }
    pub fn handle(&self, side: Side) -> Vec2 {
        match side {
            Side::Incoming => handles(self.keys[1].interp).unwrap().1,
            Side::Outgoing => handles(self.keys[2].interp).unwrap().0,
        }
    }
    pub fn vector(&self, side: Side, h: Vec2) -> DVec2 {
        let (dt, dv) = self.spans(side);
        let offset = if side == Side::Incoming { 1.0 } else { 0.0 };
        DVec2::new(
            (f64::from(h.x) - offset) * dt,
            (f64::from(h.y) - offset) * dv,
        )
    }
    pub fn solve(&self, mode: Mode, side: Side, moved: Vec2) -> Result<[Interp; 2], String> {
        if !moved.is_finite() || !(0.0..=1.0).contains(&moved.x) {
            return Err("Handle X must be finite in [0,1]; Y finite, overshoot allowed".into());
        }
        let (left_out, _) = handles(self.keys[1].interp)?;
        let (_, right_in) = handles(self.keys[2].interp)?;
        let opposite = if side == Side::Incoming {
            Side::Outgoing
        } else {
            Side::Incoming
        };
        let driver = self.vector(side, moved);
        let mut other = self.handle(opposite);
        if mode != Mode::Broken {
            let length = self.vector(opposite, other).length();
            let target = match mode {
                Mode::Mirrored => -driver,
                Mode::Aligned if driver.length() > 0.0 => -driver / driver.length() * length,
                Mode::Aligned if length == 0.0 => DVec2::ZERO,
                _ => return Err("Zero driver has no alignment direction".into()),
            };
            let (dt, dv) = self.spans(opposite);
            let offset = if opposite == Side::Incoming { 1.0 } else { 0.0 };
            if dv == 0.0 && target.y != 0.0 {
                return Err("Flat opposite segment cannot represent a vertical handle".into());
            }
            let x = offset + target.x / dt;
            let y = if dv == 0.0 {
                f64::from(other.y)
            } else {
                offset + target.y / dv
            };
            other = Vec2::new(x as f32, y as f32);
            if !(0.0..=1.0).contains(&x) || !other.is_finite() {
                return Err(
                    "Linked handle cannot be represented without clamping or overflow".into(),
                );
            }
            let actual = self.vector(opposite, other);
            if (actual - target).length() > 2e-5 * target.length().max(1.0) {
                return Err("Linked handle loses too much precision in native f32 storage".into());
            }
        }
        let (left_in, right_out) = match side {
            Side::Incoming => (moved, other),
            Side::Outgoing => (other, moved),
        };
        // The untouched far handles remain byte-for-byte identical.
        let result = [
            Interp::Bezier {
                out_handle: left_out,
                in_handle: left_in,
            },
            Interp::Bezier {
                out_handle: right_out,
                in_handle: right_in,
            },
        ];
        for i in result {
            handles(i)?;
        }
        Ok(result)
    }
    pub fn command(
        &self,
        doc: &Document,
        pair: [Interp; 2],
        gesture: Option<u64>,
    ) -> Result<Option<SetLinkedTangents>, String> {
        if !self.matches(doc) {
            return Err("Tangent capture is stale; select the key again".into());
        }
        for p in pair {
            handles(p)?;
        }
        if pair == [self.keys[1].interp, self.keys[2].interp] {
            return Ok(None);
        }
        let mut after = self.timeline.clone();
        macro_rules! write {
            ($keys:expr) => {{
                $keys[self.index].interp = pair[0];
                $keys[self.index + 1].interp = pair[1];
            }};
        }
        match &mut after {
            Timeline::BoneTranslate { keys, .. }
            | Timeline::BoneRotate { keys, .. }
            | Timeline::BoneScale { keys, .. }
            | Timeline::BoneShear { keys, .. } => write!(keys),
            Timeline::SlotColor { keys, .. } => write!(keys),
            _ => return Err("Track is not editable".into()),
        }
        Ok(Some(SetLinkedTangents {
            anim: self.anim,
            addr: self.addr.clone(),
            index: self.index,
            before: self.timeline.clone(),
            after,
            gesture,
        }))
    }
}
#[cfg(test)]
mod tests;
