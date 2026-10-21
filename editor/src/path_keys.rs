//! Bounded Path authoring in the existing inspector and key context menu.
use crate::{
    app_state::AppState,
    session::{Selection, WorkMode},
};
use ankhimate_core::{
    animation::Interp,
    ids::{AnimationId, ConstraintId},
};
use ankhimate_document::{
    commands::key_cmds::{KeyRef, TimelineAddr},
    path_keys::{Action, Capture},
};
use eframe::egui;
#[derive(Clone, Debug, PartialEq)]
struct Stamp {
    identity: u64,
    revision: u64,
    clip: Option<AnimationId>,
    mode: WorkMode,
    selection: Option<Selection>,
    playhead: f32,
    fps: u32,
}
impl Stamp {
    fn new(s: &AppState) -> Self {
        Self {
            identity: s.diagnostics.document_identity(),
            revision: s.revision,
            clip: s.session.active_animation,
            mode: s.session.work_mode,
            selection: s.session.selection.clone(),
            playhead: s.session.playhead,
            fps: s.doc.meta.fps,
        }
    }
}
pub fn editable(s: &AppState, id: ConstraintId) -> Result<(), String> {
    if !s.session.is_animating() {
        return Err("Switch to Animate to key Path Position".into());
    }
    let p = ankhimate_core::path::position_target(&s.doc.skeleton, id)?;
    if p.bones
        .iter()
        .chain(std::iter::once(&s.doc.skeleton.slots[p.slot].bone))
        .any(|b| s.session.is_bone_locked(*b))
    {
        return Err("Path/slot bone is locked".into());
    }
    if p.mix_rotate == 0. && p.mix_translate == 0. {
        return Err("Path constraint is disabled (zero mixes)".into());
    }
    Ok(())
}
fn apply(s: &mut AppState, stamp: &Stamp, id: ConstraintId, action: Action) -> Result<(), String> {
    if *stamp != Stamp::new(s) {
        return Err("Stale Path key draft; reload before applying".into());
    }
    editable(s, id)?;
    let animation = s.session.active_animation.ok_or("Select an animation")?;
    let capture = Capture::read(&s.doc, animation, id)?;
    let command = capture.command(&s.doc, action)?;
    s.session.path_preview = None;
    if let Some(cmd) = command {
        if !s.dispatch(Box::new(cmd)) {
            return Err("Path key command refused".into());
        }
    } else {
        s.refresh_pose();
    }
    Ok(())
}
#[derive(Clone, Debug)]
pub struct Preview {
    stamp: Stamp,
    id: ConstraintId,
    value: f32,
}
pub fn preview_animation(s: &mut AppState) -> Option<ankhimate_core::animation::Animation> {
    let p = s.session.path_preview.as_ref()?;
    if p.stamp != Stamp::new(s) || editable(s, p.id).is_err() {
        s.session.path_preview = None;
        return None;
    }
    let mut a = ankhimate_core::animation::Animation::new("Path Position preview", 0.);
    a.timelines
        .push(ankhimate_core::animation::Timeline::PathPosition {
            constraint: p.id,
            keys: vec![ankhimate_core::animation::Key::linear(0., p.value)],
        });
    Some(a)
}
pub fn take_pending(s: &mut AppState) -> Option<(ConstraintId, f32)> {
    preview_animation(s)?;
    s.session.path_preview.take().map(|p| (p.id, p.value))
}
pub fn commit_pending(s: &mut AppState, id: ConstraintId, value: f32) {
    let stamp = Stamp::new(s);
    let time = s.session.playhead;
    if let Err(e) = apply(
        s,
        &stamp,
        id,
        Action::Put {
            time,
            value,
            interp: Interp::Linear,
        },
    ) {
        s.session.set_status(e);
    }
}
fn preview(s: &mut AppState, stamp: &Stamp, id: ConstraintId, value: f32) -> Result<(), String> {
    if *stamp != Stamp::new(s) {
        return Err("Stale Path preview".into());
    }
    editable(s, id)?;
    // Reuse native validation before presenting a transient effective pose.
    let c = Capture::read(
        &s.doc,
        s.session.active_animation.ok_or("Select a clip")?,
        id,
    )?;
    c.command(
        &s.doc,
        Action::Put {
            time: s.session.playhead,
            value,
            interp: Interp::Linear,
        },
    )?;
    s.session.path_preview = Some(Preview {
        stamp: stamp.clone(),
        id,
        value,
    });
    s.refresh_pose();
    Ok(())
}
#[derive(Clone)]
struct Draft {
    stamp: Stamp,
    value: String,
    time: String,
    dirty: bool,
    blocked: bool,
    interp: Interp,
    error: Option<String>,
}
fn parsed(text: &str) -> Result<f32, String> {
    let v = text
        .trim()
        .replace('−', "-")
        .replace(',', ".")
        .parse::<f32>()
        .map_err(|_| "Enter a finite number")?;
    if v.is_finite() {
        Ok(v)
    } else {
        Err("Enter a finite number".into())
    }
}
fn fields(
    ui: &mut egui::Ui,
    s: &mut AppState,
    id: ConstraintId,
    index: Option<usize>,
    initial: f32,
    initial_time: f32,
    interp: Interp,
) {
    let key = ui.make_persistent_id(("path_position_draft", id, index));
    let fresh = || Draft {
        stamp: Stamp::new(s),
        value: initial.to_string(),
        time: initial_time.to_string(),
        dirty: false,
        blocked: false,
        interp,
        error: None,
    };
    let mut d = ui
        .data_mut(|data| data.get_temp::<Draft>(key))
        .unwrap_or_else(fresh);
    if d.stamp != Stamp::new(s) {
        if d.dirty {
            d.blocked = true;
        } else {
            d = fresh();
        }
    }
    let mut changed = false;
    let mut finished = false;
    ui.horizontal(|ui| {
        ui.label("Position (fraction)");
        let r = ui.text_edit_singleline(&mut d.value);
        probe(ui.ctx(), "value", &r);
        changed |= r.changed();
        finished |= r.lost_focus();
    });
    if index.is_some() {
        ui.horizontal(|ui| {
            ui.label("Time (seconds)");
            let r = ui.text_edit_singleline(&mut d.time);
            probe(ui.ctx(), "time", &r);
            changed |= r.changed();
        });
    }
    d.dirty |= changed;
    let reason = editable(s, id).err();
    if let Some(e) = &reason {
        ui.weak(e);
    }
    if d.blocked {
        ui.colored_label(
            egui::Color32::LIGHT_RED,
            "Context changed. Reload draft before applying.",
        );
        if ui.button("Reload Path draft").clicked() {
            d = fresh();
        }
    }
    let mut submit = false;
    let mut delete = false;
    ui.horizontal(|ui| {
        let r = ui.add_enabled(
            reason.is_none() && !d.blocked,
            egui::Button::new(if index.is_some() {
                "Apply Path key"
            } else {
                "Key Position"
            }),
        );
        probe(ui.ctx(), "key", &r);
        submit = r.clicked();
        if index.is_some() {
            let r = ui.add_enabled(
                reason.is_none() && !d.blocked,
                egui::Button::new("Delete Path key"),
            );
            probe(ui.ctx(), "delete", &r);
            delete = r.clicked();
        }
    });
    if index.is_none()
        && s.session.path_preview.is_some()
        && ui.button("Discard Path preview").clicked()
    {
        s.session.path_preview = None;
        s.refresh_pose();
        ui.data_mut(|data| data.remove::<Draft>(key));
        return;
    }
    if index.is_some() {
        ui.menu_button("Path interpolation", |ui| {
            for (label, value) in ankhimate_document::commands::key_cmds::presets::all() {
                if ui.button(label).clicked() {
                    d.interp = value;
                    d.dirty = true;
                    ui.close();
                }
            }
        });
    }
    if index.is_none() {
        ui.weak("Auto-key commits a changed value on field completion. With Auto-key off, completion previews the value without changing Setup. Press K or Key Position to commit; changing context discards it.");
    }
    if index.is_none() && !s.session.auto_key && finished && d.dirty && !d.blocked && !submit {
        match parsed(&d.value).and_then(|v| preview(s, &d.stamp, id, v)) {
            Ok(()) => {}
            Err(e) => d.error = Some(e),
        }
    }
    if submit
        || delete
        || (index.is_none() && s.session.auto_key && finished && d.dirty && !d.blocked)
    {
        let action = if delete {
            Ok(Action::Delete {
                index: index.expect("key action"),
            })
        } else {
            parsed(&d.time).and_then(|time| {
                parsed(&d.value).map(|value| match index {
                    Some(index) => Action::Replace {
                        index,
                        time,
                        value,
                        interp: d.interp,
                    },
                    None => Action::Put {
                        time,
                        value,
                        interp: d.interp,
                    },
                })
            })
        };
        match action.and_then(|a| apply(s, &d.stamp, id, a)) {
            Ok(()) => {
                ui.data_mut(|data| data.remove::<Draft>(key));
                return;
            }
            Err(e) => d.error = Some(e),
        }
    }
    if let Some(e) = &d.error {
        ui.colored_label(egui::Color32::LIGHT_RED, e);
    }
    ui.data_mut(|data| data.insert_temp(key, d));
}
pub fn position_ui(ui: &mut egui::Ui, s: &mut AppState, id: ConstraintId) {
    if !s.session.is_animating() {
        return;
    }
    let Some(ankhimate_core::constraints::Constraint::Path(p)) = s.doc.skeleton.constraints.get(id)
    else {
        return;
    };
    let setup = p.position;
    let effective = s.pose.path_position.get(id).copied().unwrap_or(setup);
    ui.label(format!(
        "Path Position: Setup {setup} · evaluated {effective}"
    ));
    let interp = s
        .session
        .active_animation
        .and_then(|a| s.doc.animations.get(a))
        .and_then(|a| {
            a.timelines.iter().find_map(|t| {
                if let ankhimate_core::animation::Timeline::PathPosition { constraint, keys } = t {
                    (*constraint == id).then(|| {
                        keys.iter()
                            .find(|k| k.time == s.session.playhead)
                            .map(|k| k.interp)
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .unwrap_or_default();
    fields(ui, s, id, None, effective, s.session.playhead, interp);
}
pub fn key_menu(ui: &mut egui::Ui, s: &mut AppState, animation: AnimationId, key: &KeyRef) {
    let TimelineAddr::PathPosition { constraint } = key.addr else {
        return;
    };
    if s.session.active_animation != Some(animation) {
        ui.label("Path key belongs to another clip");
        return;
    }
    match Capture::read(&s.doc, animation, constraint).and_then(|c| {
        c.keys
            .get(key.index)
            .cloned()
            .ok_or("Path key missing".into())
    }) {
        Ok(k) => fields(
            ui,
            s,
            constraint,
            Some(key.index),
            k.value,
            k.time,
            k.interp,
        ),
        Err(e) => {
            ui.label(e);
        }
    }
    ui.weak("Path key: time, value, delete and interpolation only. Graph, drag/group edits and clipboard are deferred.");
}
fn probe(_ctx: &egui::Context, _name: &str, _r: &egui::Response) {
    #[cfg(test)]
    _ctx.data_mut(|d| d.insert_temp(egui::Id::new(("t831", _name)), _r.rect));
}
#[cfg(test)]
pub(crate) mod tests;
