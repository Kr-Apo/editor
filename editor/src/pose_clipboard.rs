//! Resolve and route a whole pose before applying a single undoable gesture.
use crate::{
    app_state::AppState,
    edit_router::{self, EditIntent, Routed},
};
use ankhimate_document::{clipboard::PoseClip, commands::pose_cmds::PastePose};

impl AppState {
    pub(crate) fn paste_pose(&mut self, clip: PoseClip, mirrored: bool) {
        if let Err(reason) = self.try_paste_pose(clip, mirrored) {
            self.session.set_status(format!("Paste refused: {reason}"));
        }
    }
    fn try_paste_pose(&mut self, clip: PoseClip, mirrored: bool) -> Result<(), String> {
        let clip = if mirrored { clip.mirrored() } else { clip };
        if clip.entries.is_empty() {
            return Err("Pose is empty".into());
        }
        let anim = if self.session.is_animating() {
            let id = self
                .session
                .active_animation
                .filter(|id| self.doc.animations.contains_key(*id))
                .ok_or("No valid animation selected")?;
            if !self.session.playhead.is_finite() {
                return Err("Invalid playhead".into());
            }
            Some(id)
        } else {
            None
        };
        let mut seen = std::collections::HashSet::new();
        let mut commands = vec![];
        let mut pending = vec![];
        // No document or preview writes until ALL names, locks, values and
        // routing decisions are validated. Ambiguity is never resolved by order.
        for (name, value) in &clip.entries {
            if !seen.insert(name) {
                return Err(format!("Duplicate pose target: {name}"));
            }
            let mut matches = self
                .doc
                .skeleton
                .bones
                .iter()
                .filter(|(_, b)| b.name == *name);
            let (bone, _) = matches
                .next()
                .ok_or_else(|| format!("Missing bone: {name}"))?;
            if matches.next().is_some() {
                return Err(format!("Ambiguous bone: {name}"));
            }
            if self.session.is_bone_locked(bone) {
                return Err(format!("Locked bone: {name}"));
            }
            if !value.position.is_finite()
                || !value.rotation.is_finite()
                || !value.scale.is_finite()
                || !value.shear.is_finite()
            {
                return Err(format!("Invalid pose value: {name}"));
            }
            match edit_router::route(
                EditIntent::BoneLocal {
                    bone,
                    value: *value,
                },
                &self.doc,
                &self.session,
            ) {
                Routed::Commands(children) => {
                    if children.iter().any(|cmd| {
                        cmd.requires_mode()
                            .is_some_and(|mode| mode != self.session.work_mode)
                    }) {
                        return Err("Pose not editable in this mode".into());
                    }
                    commands.extend(children);
                }
                Routed::Pending => pending.push((bone, *value)),
                Routed::Refused(reason) => return Err(reason.into()),
            }
        }
        if !commands.is_empty() {
            self.dispatch(Box::new(PastePose::new(
                commands,
                self.session.work_mode,
                anim,
                mirrored,
            )));
        }
        // Auto-key OFF retains the existing session-only preview policy: no
        // authored edit or artificial undo entry. Names were validated above.
        for (bone, value) in pending {
            self.commit_bone_pose(bone, value);
        }
        let what = if mirrored { "mirrored pose" } else { "pose" };
        self.session
            .set_status(format!("Pasted {what} onto {} bone(s)", clip.entries.len()));
        Ok(())
    }
}
