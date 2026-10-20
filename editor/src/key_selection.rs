//! One session selection shared by sheet, graph and inspector. Indices are leases,
//! valid only for the captured revision/clip/data; they are never persistent IDs.
use crate::app_state::AppState;
use ankhimate_core::ids::AnimationId;
use ankhimate_document::{
    clipboard::KeyClip,
    commands::key_cmds::{KeyRef, MoveKeys},
};
#[derive(Debug, Clone, Default)]
pub struct KeySelection {
    animation: Option<AnimationId>,
    revision: u64,
    refs: Vec<KeyRef>,
    captured: Option<KeyClip>,
    ffd: Option<crate::ffd_keys::Lease>,
}
pub fn read(state: &mut AppState, anim: AnimationId) -> Vec<KeyRef> {
    let s = &state.session.key_selection;
    if let Some(lease) = &s.ffd {
        if s.animation == Some(anim) && lease.matches(state) {
            return s.refs.clone();
        }
        state.session.key_selection = Default::default();
        return Vec::new();
    }
    let valid = s.animation == Some(anim)
        && state.session.active_animation == Some(anim)
        && s.revision == state.revision
        && KeyClip::capture(&state.doc, anim, &s.refs).ok() == s.captured;
    if valid {
        s.refs.clone()
    } else {
        state.session.key_selection = Default::default();
        Vec::new()
    }
}
pub fn set(state: &mut AppState, anim: AnimationId, refs: Vec<KeyRef>) {
    if refs.iter().any(|r| {
        matches!(
            r.addr,
            ankhimate_document::commands::key_cmds::TimelineAddr::Deform { .. }
        )
    }) {
        if refs.len() != 1 {
            state
                .session
                .set_status("FFD selection is limited to one key; mixed/grouped selection refused");
            return;
        }
        state.session.key_selection =
            match crate::ffd_keys::Lease::new(state, anim, refs[0].clone()) {
                Ok(l) if state.session.active_animation == Some(anim) => KeySelection {
                    animation: Some(anim),
                    revision: state.revision,
                    refs,
                    captured: None,
                    ffd: Some(l),
                },
                Err(e) => {
                    state.session.set_status(e);
                    KeySelection::default()
                }
                _ => KeySelection::default(),
            };
        return;
    }
    let captured = KeyClip::capture(&state.doc, anim, &refs).ok();
    state.session.key_selection =
        if captured.is_some() && state.session.active_animation == Some(anim) {
            KeySelection {
                animation: Some(anim),
                revision: state.revision,
                refs,
                captured,
                ffd: None,
            }
        } else {
            KeySelection::default()
        };
}
/// Re-resolve only our own move by unique destination times after sorting. Refuse
/// collisions (including clamp-to-zero) before dispatch so Redo is preserved.
pub fn move_keys(
    state: &mut AppState,
    anim: AnimationId,
    refs: &[KeyRef],
    delta: f32,
) -> Option<Vec<KeyRef>> {
    if state.session.active_animation != Some(anim) || refs.is_empty() || !delta.is_finite() {
        return None;
    }
    if refs.iter().any(|r| {
        matches!(
            r.addr,
            ankhimate_document::commands::key_cmds::TimelineAddr::Deform { .. }
        )
    }) {
        if refs != read(state, anim) {
            return None;
        }
        return crate::ffd_keys::move_delta(state, anim, delta, None).ok();
    }
    KeyClip::capture(&state.doc, anim, refs).ok()?;
    let a = state.doc.animations.get(anim)?;
    let mut targets = Vec::new();
    for r in refs {
        if let ankhimate_document::commands::key_cmds::TimelineAddr::Bone { bone, .. } = r.addr
            && state.session.is_bone_locked(bone)
        {
            return None;
        }
        let timeline = a.timelines.iter().find(|t| r.addr.matches_timeline(t))?;
        let all = (0..key_count(timeline))
            .map(|index| KeyRef {
                addr: r.addr.clone(),
                index,
            })
            .collect::<Vec<_>>();
        let keys = KeyClip::capture(&state.doc, anim, &all).ok()?;
        let dest = (keys.keys.get(r.index)?.key.time + delta).max(0.0);
        if !dest.is_finite()
            || targets
                .iter()
                .any(|(addr, t)| addr == &r.addr && *t == dest)
        {
            return None;
        }
        if keys.keys.iter().enumerate().any(|(i, k)| {
            k.key.time == dest
                && !refs.contains(&KeyRef {
                    addr: r.addr.clone(),
                    index: i,
                })
        }) {
            return None;
        }
        targets.push((r.addr.clone(), dest));
    }
    if !state.dispatch(Box::new(MoveKeys::new(anim, refs.to_vec(), delta))) {
        return None;
    }
    let a = state.doc.animations.get(anim)?;
    let mut moved = Vec::new();
    for (addr, time) in targets {
        let t = a.timelines.iter().find(|t| addr.matches_timeline(t))?;
        let all = (0..key_count(t))
            .map(|index| KeyRef {
                addr: addr.clone(),
                index,
            })
            .collect::<Vec<_>>();
        let keys = KeyClip::capture(&state.doc, anim, &all).ok()?;
        let indices = keys
            .keys
            .iter()
            .enumerate()
            .filter(|(_, k)| k.key.time == time)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if indices.len() != 1 {
            return None;
        }
        moved.push(KeyRef {
            addr,
            index: indices[0],
        });
    }
    set(state, anim, moved.clone());
    Some(moved)
}

fn key_count(t: &ankhimate_core::animation::Timeline) -> usize {
    use ankhimate_core::animation::Timeline;
    match t {
        Timeline::BoneRotate { keys, .. }
        | Timeline::BoneTranslate { keys, .. }
        | Timeline::BoneScale { keys, .. }
        | Timeline::BoneShear { keys, .. } => keys.len(),
        Timeline::SlotColor { keys, .. } => keys.len(),
        Timeline::SlotVisible { keys, .. } => keys.len(),
        Timeline::SlotAttachment { keys, .. } => keys.len(),
        _ => 0,
    }
}

impl KeySelection {
    pub fn clear_ffd(&mut self) {
        if self.ffd.is_some() {
            *self = Self::default();
        }
    }
}
