//! Dopesheet clipboard operators. Arguments are transient row/key indices;
//! the application clipboard immediately snapshots values and named targets.
use crate::{app_state::AppState, registry::OpResult};
use ankhimate_core::ids::AnimationId;
use ankhimate_document::{
    Args,
    clipboard::{Clipboard, KeyClip},
    commands::key_cmds::{KeyRef, PasteKeys},
};

pub fn selection_args(
    state: &AppState,
    anim: AnimationId,
    refs: &[KeyRef],
) -> Result<Args, String> {
    let animation = state
        .doc
        .animations
        .get(anim)
        .ok_or("Animation no longer exists")?;
    let mut tracks = Vec::new();
    let mut indices = Vec::new();
    for r in refs {
        let matches: Vec<_> = animation
            .timelines
            .iter()
            .enumerate()
            .filter(|(_, t)| r.addr.matches_timeline(t))
            .map(|(i, _)| i)
            .collect();
        if matches.len() != 1 {
            return Err("Selected track is missing or ambiguous".into());
        }
        tracks.push(matches[0]);
        indices.push(r.index);
    }
    Ok(Args::from_json(
        serde_json::json!({"animation":animation.name,"tracks":tracks,"indices":indices}),
    ))
}

fn selection(state: &AppState, args: &Args) -> Result<(AnimationId, Vec<KeyRef>), String> {
    if !state.session.is_animating() {
        return Err("Key clipboard actions require Animate mode".into());
    }
    let name = args.str("animation").map_err(|e| e.to_string())?;
    let animations: Vec<_> = state
        .doc
        .animations
        .iter()
        .filter(|(_, a)| a.name == name)
        .map(|(id, _)| id)
        .collect();
    if animations.len() != 1 {
        return Err("Animation name is missing or ambiguous".into());
    }
    let anim = animations[0];
    if Some(anim) != state.session.active_animation {
        return Err("Clipboard request is for a different animation".into());
    }
    let tracks = args.u32_list("tracks").map_err(|e| e.to_string())?;
    let indices = args.u32_list("indices").map_err(|e| e.to_string())?;
    if tracks.len() != indices.len() {
        return Err("Track/key selections have different lengths".into());
    }
    let animation = &state.doc.animations[anim];
    let mut refs = Vec::new();
    for (track, index) in tracks.into_iter().zip(indices) {
        let timeline = animation
            .timelines
            .get(track as usize)
            .ok_or("Selected timeline index is invalid")?;
        let addr = ankhimate_document::clipboard::keys::address(timeline)?;
        refs.push(KeyRef {
            addr,
            index: index as usize,
        });
    }
    Ok((anim, refs))
}

pub fn copy(state: &mut AppState, args: &Args) -> OpResult {
    let result = selection(state, args)
        .and_then(|(anim, refs)| {
            if refs.iter().any(|r| {
                matches!(
                    r.addr,
                    ankhimate_document::commands::key_cmds::TimelineAddr::PathPosition { .. }
                )
            }) {
                return Err(
                    "Path Position clipboard is deferred; use its key context controls".into(),
                );
            }
            Ok((anim, refs))
        })
        .and_then(|(anim, refs)| KeyClip::capture(&state.doc, anim, &refs));
    match result {
        Ok(clip) => {
            state
                .session
                .set_status(format!("Copied {} key(s)", clip.keys.len()));
            state.session.clipboard = Clipboard::Keys(clip);
        }
        Err(reason) => state.session.set_status(format!("Copy refused: {reason}")),
    }
    OpResult::done()
}

pub fn paste(state: &mut AppState, args: &Args) -> OpResult {
    paste_impl(state, args, false)
}
pub fn paste_mirrored_offsets(state: &mut AppState, args: &Args) -> OpResult {
    paste_impl(state, args, true)
}
/// The menu uses the same read-only preparation as invocation, including locks.
pub fn can_paste_mirrored_offsets(state: &AppState, args: &Args) -> bool {
    prepare_paste(state, args, true).is_ok()
}
fn prepare_paste(state: &AppState, args: &Args, mirrored: bool) -> Result<PasteKeys, String> {
    let (anim, refs) = selection(state, args)?;
    let Clipboard::Keys(clip) = &state.session.clipboard else {
        return Err("Clipboard does not contain selected keys".into());
    };
    // Destination key selection is also validated before using its row identities.
    if !refs.is_empty() {
        KeyClip::capture(&state.doc, anim, &refs)?;
    }
    let targets: Vec<_> = refs.iter().map(|r| r.addr.clone()).collect();
    if mirrored {
        for target in &targets {
            let ankhimate_document::commands::key_cmds::TimelineAddr::Bone { bone, .. } = target
            else {
                return Err("Mirrored offsets require bone destination rows only".into());
            };
            if state.session.is_bone_locked(*bone) {
                return Err("Destination bone is locked".into());
            }
        }
        PasteKeys::new_mirrored_offsets(&state.doc, anim, clip, state.session.playhead, &targets)
    } else {
        PasteKeys::new(&state.doc, anim, clip, state.session.playhead, &targets)
    }
}
fn paste_impl(state: &mut AppState, args: &Args, mirrored: bool) -> OpResult {
    match prepare_paste(state, args, mirrored) {
        Ok(cmd) => {
            state.dispatch(Box::new(cmd));
            state.session.set_status(if mirrored {
                "Pasted bone key offsets mirrored (one Undo); asymmetric setups need not produce an absolute mirrored pose"
            } else {"Pasted selected keys (one Undo)"});
        }
        Err(reason) => state.session.set_status(format!("Paste refused: {reason}")),
    }
    OpResult::done()
}

fn sample(state: &AppState) -> ankhimate_core::pose::Pose {
    let mut pose = ankhimate_core::pose::Pose::default();
    let clips: Vec<_> = state
        .session
        .active_animation
        .and_then(|id| state.doc.animations.get(id))
        .map(|clip| vec![(clip, state.session.playhead, 1.0)])
        .unwrap_or_default();
    ankhimate_core::pose::evaluate(&state.doc.skeleton, &clips, &mut pose);
    pose
}

/// Only key-block edits use this hook; other document/history behavior is unchanged.
pub(crate) fn preview_before(
    state: &AppState,
    command: &dyn ankhimate_document::EditCommand,
) -> Option<ankhimate_core::pose::Pose> {
    (command.as_any().is::<PasteKeys>()
        && state.session.is_animating()
        && state.session.has_pending_pose())
    .then(|| sample(state))
}

/// Pending transforms are stored whole. Rebase their untouched components onto
/// the new sample, retaining the user's actual pending properties. The same
/// operation is needed on paste, Undo AND Redo, without adding history entries.
pub(crate) fn rebase_preview(state: &mut AppState, before: Option<ankhimate_core::pose::Pose>) {
    let Some(before) = before else {
        return;
    };
    let after = sample(state);
    for bone in state.session.pending_pose.clone() {
        if let (Some(value), Some(old), Some(new)) = (
            state.session.preview_locals.get_mut(bone),
            before.locals.get(bone),
            after.locals.get(bone),
        ) {
            if value.rotation == old.rotation {
                value.rotation = new.rotation;
            }
            for axis in 0..2 {
                if value.position[axis] == old.position[axis] {
                    value.position[axis] = new.position[axis];
                }
                if value.scale[axis] == old.scale[axis] {
                    value.scale[axis] = new.scale[axis];
                }
                if value.shear[axis] == old.shear[axis] {
                    value.shear[axis] = new.shear[axis];
                }
            }
            if *value == *new {
                state.session.preview_locals.remove(bone);
                state.session.pending_pose.retain(|&b| b != bone);
            }
        }
    }
    let unchanged: Vec<_> = state
        .session
        .pending_slot_colors
        .iter()
        .filter(|(slot, color)| before.slot_colors.get(*slot) == Some(*color))
        .map(|(slot, _)| slot)
        .collect();
    for slot in unchanged {
        state.session.pending_slot_colors.remove(slot);
    }
}
