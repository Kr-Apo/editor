//! Checked preparation for the numeric UI; legacy fixed constructors stay unchanged.
use super::{Animation, RetimeAnimation, Timeline, mapped_time, retime_clip};
use ankhimate_core::ids::AnimationId;

#[derive(Debug, Clone, PartialEq)]
pub struct RetimeSummary {
    pub factor: f32,
    pub offset: f32,
    pub duration_before: f32,
    pub duration_after: f32,
    pub tracks: usize,
    pub keys: usize,
    pub changed_keys: usize,
    pub before_range: Option<(f32, f32)>,
    pub after_range: Option<(f32, f32)>,
    pub clamped: usize,
    pub outside_duration: usize,
    pub events: usize,
    pub markers: usize,
    pub bone_offsets: usize,
    pub no_op: bool,
}
pub struct PreparedRetime {
    pub summary: RetimeSummary,
    pub command: RetimeAnimation,
}
fn times(t: &Timeline) -> Vec<f32> {
    macro_rules! times {
        ($keys:expr) => {
            $keys.iter().map(|k| k.time).collect()
        };
    }
    match t {
        Timeline::BoneTranslate { keys, .. } => times!(keys),
        Timeline::BoneRotate { keys, .. } => times!(keys),
        Timeline::BoneScale { keys, .. } => times!(keys),
        Timeline::BoneShear { keys, .. } => times!(keys),
        Timeline::SlotVisible { keys, .. } => times!(keys),
        Timeline::SlotColor { keys, .. } => times!(keys),
        Timeline::SlotAttachment { keys, .. } => times!(keys),
        Timeline::DrawOrder { keys } => times!(keys),
        Timeline::IkMix { keys, .. } => times!(keys),
        Timeline::IkBendDirection { keys, .. } => times!(keys),
        Timeline::PathPosition { keys, .. } | Timeline::IkSoftness { keys, .. } => times!(keys),
        Timeline::TransformConstraintMix { keys, .. } => times!(keys),
        Timeline::Deform { keys, .. } => times!(keys),
    }
}
fn range(r: &mut Option<(f32, f32)>, t: f32) {
    *r = Some(r.map_or((t, t), |(a, b)| (a.min(t), b.max(t))));
}
fn checked(value: f64, name: &str) -> Result<f32, String> {
    let result = value as f32;
    if !value.is_finite() || !result.is_finite() || (value != 0.0 && result == 0.0) {
        return Err(format!(
            "{name} is not representable as a finite nonzero f32"
        ));
    }
    Ok(result)
}
impl RetimeAnimation {
    /// All native timelines, with f32 arithmetic and the existing duration rule.
    /// This is pure: the caller must still lease its editor context before dispatch.
    pub fn prepare_numeric(
        id: AnimationId,
        source: &Animation,
        factor: f64,
        offset: f64,
    ) -> Result<PreparedRetime, String> {
        let factor = checked(factor, "Time Scale")?;
        let offset = checked(offset, "Offset")?;
        if factor <= 0.0 {
            return Err("Time Scale must be positive".into());
        }
        if !source.duration.is_finite() || source.duration < 0.0 {
            return Err("Invalid source duration".into());
        }
        let mut after = source.clone();
        retime_clip(&mut after, factor, offset);
        // Check the raw duration too: max(0) must not hide overflow.
        if !mapped_time(source.duration, factor, offset).is_finite()
            || !after.duration.is_finite()
            || (source.duration > 0.0 && offset == 0.0 && after.duration == 0.0)
        {
            return Err("Duration overflow or underflow".into());
        }
        let mut summary = RetimeSummary {
            factor,
            offset,
            duration_before: source.duration,
            duration_after: after.duration,
            tracks: source.timelines.len(),
            keys: 0,
            changed_keys: 0,
            before_range: None,
            after_range: None,
            clamped: 0,
            outside_duration: 0,
            events: source.events.len(),
            markers: source.markers.len(),
            bone_offsets: source.bone_offsets.len(),
            no_op: source == &after,
        };
        for (index, (a, b)) in source.timelines.iter().zip(&after.timelines).enumerate() {
            let a = times(a);
            let b = times(b);
            if a.iter().any(|t| !t.is_finite() || *t < 0.0) || a.windows(2).any(|w| w[0] >= w[1]) {
                return Err(format!(
                    "Track {index}: source times must be finite, nonnegative and strictly increasing"
                ));
            }
            for (&old, &new) in a.iter().zip(&b) {
                let raw = mapped_time(old, factor, offset);
                if !raw.is_finite() || (old > 0.0 && offset == 0.0 && new == 0.0) {
                    return Err(format!("Track {index}: time overflow or underflow"));
                }
                summary.keys += 1;
                summary.changed_keys += usize::from(old != new);
                summary.clamped += usize::from(raw < 0.0);
                summary.outside_duration += usize::from(new > after.duration);
                range(&mut summary.before_range, old);
                range(&mut summary.after_range, new);
            }
            if b.windows(2).any(|w| w[0] >= w[1]) {
                return Err(format!(
                    "Track {index}: time collision after clamp or f32 rounding; no keys were merged"
                ));
            }
        }
        Ok(PreparedRetime {
            summary,
            command: Self {
                anim: id,
                factor,
                offset,
                before: None,
                label: "Retime Animation",
            },
        })
    }
}
#[cfg(test)]
#[path = "retime_numeric_tests.rs"]
mod tests;
