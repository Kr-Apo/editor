//! Guarded existing FFD key editing through the shared selection.
use crate::{app_state::AppState, key_selection};
use ankhimate_core::{
    attachment::Attachment,
    ids::{AnimationId, SkinId},
};
use ankhimate_document::{
    commands::{
        attachment_cmds::owning_skin,
        key_cmds::{KeyRef, TimelineAddr},
    },
    ffd_keys::{Action, Capture},
};
use eframe::egui;
#[derive(Clone, Debug)]
pub struct Lease {
    pub capture: Capture,
    identity: u64,
    revision: u64,
    skin: SkinId,
    mode: crate::session::WorkMode,
}
impl Lease {
    pub fn new(s: &AppState, animation: AnimationId, key: KeyRef) -> Result<Self, String> {
        let capture = Capture::read(&s.doc, animation, key)?;
        context(s, &capture)?;
        Ok(Self {
            capture,
            identity: s.diagnostics.document_identity(),
            revision: s.revision,
            skin: s.session.active_skin,
            mode: s.session.work_mode,
        })
    }
    pub fn matches(&self, s: &AppState) -> bool {
        self.identity == s.diagnostics.document_identity()
            && self.revision == s.revision
            && self.skin == s.session.active_skin
            && self.mode == s.session.work_mode
            && s.session.active_animation == Some(self.capture.animation)
            && context(s, &self.capture).is_ok()
            && self.capture.matches(&s.doc)
    }
}
fn context(s: &AppState, c: &Capture) -> Result<(), String> {
    context_addr(s, &c.key.addr)
}
fn context_addr(s: &AppState, addr: &TimelineAddr) -> Result<(), String> {
    let TimelineAddr::Deform {
        slot,
        attachment,
        owner,
    } = addr
    else {
        return Err("Not a Deform address".into());
    };
    if !s.doc.skeleton.skins.contains_key(s.session.active_skin)
        || owning_skin(&s.doc, s.session.active_skin, *slot, attachment) != *owner
    {
        return Err("Active skin does not resolve to the reviewed FFD owner".into());
    }
    if s.pose.attachment_name(&s.doc.skeleton, *slot) != Some(attachment.as_str()) {
        return Err("Select the displayed mesh attachment for this FFD row".into());
    }
    Ok(())
}
pub fn editable(s: &AppState, c: &Capture) -> Result<(), String> {
    editable_addr(s, &c.key.addr)
}
fn editable_addr(s: &AppState, addr: &TimelineAddr) -> Result<(), String> {
    if !s.session.is_animating() {
        return Err("Switch to Animate to edit FFD keys".into());
    }
    context_addr(s, addr)?;
    let TimelineAddr::Deform {
        slot,
        attachment,
        owner: Some(skin),
    } = addr
    else {
        return Err("Unresolved FFD owner".into());
    };
    let bone = s.doc.skeleton.slots[*slot].bone;
    let Some(Attachment::Mesh(m)) = s.doc.skeleton.skins[*skin].get(*slot, attachment) else {
        return Err("Mesh disappeared".into());
    };
    if s.session.is_bone_locked(bone)
        || m.weights
            .iter()
            .flatten()
            .any(|w| s.session.is_bone_locked(w.bone))
    {
        return Err("FFD mesh/slot bone or influencing bone is locked".into());
    }
    Ok(())
}
pub fn reason(s: &AppState, animation: AnimationId, key: KeyRef) -> Option<String> {
    ankhimate_document::ffd_keys::validate(&s.doc, animation, &key)
        .and_then(|_| editable_addr(s, &key.addr))
        .err()
}
pub fn selected(s: &mut AppState, anim: AnimationId) -> Result<Lease, String> {
    let refs = key_selection::read(s, anim);
    if refs.len() != 1 || !matches!(refs[0].addr, TimelineAddr::Deform { .. }) {
        return Err(
            "Select exactly one existing FFD key; grouped FFD edits are not supported".into(),
        );
    }
    Lease::new(s, anim, refs[0].clone())
}
pub fn apply(
    s: &mut AppState,
    lease: &Lease,
    action: Action,
    gesture: Option<u64>,
) -> Result<(), String> {
    let refs = key_selection::read(s, lease.capture.animation);
    if !lease.matches(s) || refs != vec![lease.capture.key.clone()] {
        return Err("FFD key selection changed; select again".into());
    }
    editable(s, &lease.capture)?;
    let Some(cmd) = lease.capture.command(&s.doc, action, gesture)? else {
        return Ok(());
    };
    if !s.dispatch(Box::new(cmd)) {
        return Err("FFD native command refused".into());
    }
    match action {
        Action::Delete => s.session.key_selection = Default::default(),
        Action::Move(time) => {
            let mut r = lease.capture.key.clone();
            let a = &s.doc.animations[lease.capture.animation];
            let t = a
                .timelines
                .iter()
                .find(|t| r.addr.matches_timeline(t))
                .ok_or("Moved track disappeared")?;
            if let ankhimate_core::animation::Timeline::Deform { keys, .. } = t {
                r.index = keys
                    .iter()
                    .position(|k| k.time == time)
                    .ok_or("Moved key disappeared")?;
            }
            key_selection::set(s, lease.capture.animation, vec![r]);
        }
        _ => key_selection::set(s, lease.capture.animation, vec![lease.capture.key.clone()]),
    }
    Ok(())
}
pub fn move_delta(
    s: &mut AppState,
    anim: AnimationId,
    delta: f32,
    token: Option<u64>,
) -> Result<Vec<KeyRef>, String> {
    let l = selected(s, anim)?;
    let time = l.capture.keys[l.capture.key.index].time + delta;
    apply(s, &l, Action::Move(time), token)?;
    Ok(key_selection::read(s, anim))
}
pub fn menu(ui: &mut egui::Ui, s: &mut AppState, anim: AnimationId) {
    let l = selected(s, anim);
    let available = l
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|l| editable(s, &l.capture));
    ui.label("FFD key — time/interpolation only; offsets are unchanged");
    if let Ok(l) = &l
        && let TimelineAddr::Deform {
            slot,
            attachment,
            owner: Some(owner),
        } = &l.capture.key.addr
    {
        ui.label(format!(
            "{} / {} / {}",
            s.doc.skeleton.skins[*owner].name, s.doc.skeleton.slots[*slot].name, attachment
        ));
    }
    if let Err(e) = &available {
        ui.colored_label(ui.visuals().error_fg_color, e);
    }
    let Ok(l) = l else {
        return;
    };
    ui.label(format!(
        "t={} s; {} offset entries",
        l.capture.keys[l.capture.key.index].time,
        l.capture.keys[l.capture.key.index].value.len()
    ));
    let id = ui.id().with("ffd_time_draft");
    let stored = ui.ctx().data(|d| d.get_temp::<(u64, KeyRef, String)>(id));
    let mut text = stored
        .filter(|(rev, r, _)| *rev == s.revision && r == &l.capture.key)
        .map(|(_, _, t)| t)
        .unwrap_or_else(|| l.capture.keys[l.capture.key.index].time.to_string());
    ui.horizontal(|ui| {
        ui.label("Time (s)");
        ui.add_enabled(
            available.is_ok(),
            egui::TextEdit::singleline(&mut text).desired_width(90.),
        );
        if ui
            .add_enabled(available.is_ok(), egui::Button::new("Move FFD key"))
            .clicked()
        {
            let result = text
                .trim()
                .parse::<f32>()
                .map_err(|_| "Enter a finite nonnegative time".to_string())
                .and_then(|t| apply(s, &l, Action::Move(t), None));
            if let Err(e) = result {
                s.session.set_status(e);
            } else {
                ui.close();
            }
        }
    });
    ui.ctx()
        .data_mut(|d| d.insert_temp(id, (l.revision, l.capture.key.clone(), text)));
    if ui
        .add_enabled(available.is_ok(), egui::Button::new("Delete FFD key"))
        .clicked()
    {
        if let Err(e) = apply(s, &l, Action::Delete, None) {
            s.session.set_status(e);
        }
        ui.close();
    }
    ui.menu_button("FFD interpolation", |ui| {
        for (label, i) in ankhimate_document::commands::key_cmds::presets::all() {
            if ui
                .add_enabled(available.is_ok(), egui::Button::new(label))
                .clicked()
            {
                if let Err(e) = apply(s, &l, Action::Interpolation(i), None) {
                    s.session.set_status(e);
                }
                ui.close();
            }
        }
    });
    ui.weak(
        "Copy FFD Keys/Paste FFD Keys reuse this exact mesh; the edits above remain single-key.",
    );
}
#[cfg(test)]
pub(crate) mod tests;
