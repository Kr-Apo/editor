//! Draft text is session UI state, never authored until explicit Apply.
use crate::{app_state::AppState, key_selection};
use ankhimate_core::{animation::Interp, ids::AnimationId};
use ankhimate_document::{
    clipboard::KeyClip,
    commands::key_cmds::{KeyRef, SetInterp, TimelineAddr},
};
use eframe::egui;
#[derive(Clone, PartialEq)]
pub(crate) struct Draft {
    anim: AnimationId,
    revision: u64,
    refs: Vec<KeyRef>,
    captured: KeyClip,
    pub fields: [String; 4],
}
impl Draft {
    pub fn new(state: &mut AppState, anim: AnimationId) -> Result<Self, String> {
        let refs = key_selection::read(state, anim);
        if refs.is_empty() {
            return Err("Select a Bezier segment end key".into());
        }
        let captured = KeyClip::capture(&state.doc, anim, &refs)?;
        for r in &refs {
            if r.index == 0
                || matches!(
                    r.addr,
                    TimelineAddr::SlotAttachment { .. } | TimelineAddr::SlotVisible { .. }
                )
            {
                return Err("Select Bezier keys with a preceding key".into());
            }
        }
        if captured
            .keys
            .iter()
            .any(|k| !matches!(k.key.interp, Interp::Bezier { .. }))
        {
            return Err("Numeric handles require an entirely Bezier selection; use an explicit interpolation preset first".into());
        }
        let Interp::Bezier {
            out_handle,
            in_handle,
        } = captured.keys[0].key.interp
        else {
            unreachable!()
        };
        Ok(Self {
            anim,
            revision: state.revision,
            refs,
            captured,
            fields: [out_handle.x, out_handle.y, in_handle.x, in_handle.y].map(|n| n.to_string()),
        })
    }
    fn parsed(&self) -> Result<Interp, String> {
        let mut v = [0.0; 4];
        for (n, text) in v.iter_mut().zip(&self.fields) {
            *n = text
                .trim()
                .parse::<f32>()
                .map_err(|_| "Enter four finite numbers")?;
            if !n.is_finite() {
                return Err("Handles must be finite".into());
            }
        }
        if !(0.0..=1.0).contains(&v[0]) || !(0.0..=1.0).contains(&v[2]) {
            return Err("Time X must be in [0, 1]; value Y may overshoot".into());
        }
        Ok(Interp::Bezier {
            out_handle: glam::vec2(v[0], v[1]),
            in_handle: glam::vec2(v[2], v[3]),
        })
    }
    pub fn apply(&self, state: &mut AppState) -> Result<(), String> {
        if !state.session.is_animating() {
            return Err("Switch to Animate mode to edit tangents".into());
        }
        if self.revision != state.revision
            || key_selection::read(state, self.anim) != self.refs
            || KeyClip::capture(&state.doc, self.anim, &self.refs)
                .ok()
                .as_ref()
                != Some(&self.captured)
        {
            return Err("Selection changed; select the keys again".into());
        }
        for r in &self.refs {
            if let TimelineAddr::Bone { bone, .. } = r.addr
                && state.session.is_bone_locked(bone)
            {
                return Err("Selected bone is locked".into());
            }
        }
        let interp = self.parsed()?;
        if self.captured.keys.iter().all(|k| k.key.interp == interp) {
            return Ok(());
        }
        if !state.dispatch(Box::new(
            SetInterp::new(self.anim, self.refs.clone(), interp).isolated(),
        )) {
            return Err("Tangent edit refused".into());
        }
        key_selection::set(state, self.anim, self.refs.clone());
        Ok(())
    }
}
pub fn ui(ui: &mut egui::Ui, state: &mut AppState) {
    let id = ui.id().with("numeric_bezier_draft");
    let current = state
        .session
        .active_animation
        .and_then(|a| Draft::new(state, a).ok());
    let Some(current) = current else {
        ui.ctx().data_mut(|d| d.remove::<Draft>(id));
        return;
    };
    let mut draft = ui
        .ctx()
        .data(|d| d.get_temp::<Draft>(id))
        .filter(|d| {
            d.revision == current.revision
                && d.refs == current.refs
                && d.captured == current.captured
                && d.anim == current.anim
        })
        .unwrap_or(current);
    ui.group(|ui| {
        ui.label(egui::RichText::new("Bezier tangents").strong());
        ui.label(format!("{} / {} key(s)", state.doc.animations[draft.anim].name, draft.refs.len()));
        for (r, k) in draft.refs.iter().zip(&draft.captured.keys) {
            ui.label(format!("{} {:?} — t={}s, key {}", k.track.name, k.track.kind, k.key.time, r.index));
        }
        ui.label("Incoming segment: X = normalized time [0,1]; Y = normalized value (overshoot allowed).");
        if draft.refs.len() > 1 {
            ui.label("Apply replaces all four handles on ALL selected Bezier keys with these values.");
        }
        egui::Grid::new("bezier_numbers").show(ui, |ui| {
            for (label, value) in ["Out X", "Out Y", "In X", "In Y"].into_iter().zip(&mut draft.fields) {
                ui.label(label);
                ui.add(egui::TextEdit::singleline(value).desired_width(100.0));
                ui.end_row();
            }
        });
        if let Err(error) = draft.parsed() {
            ui.colored_label(ui.visuals().error_fg_color, error);
        }
        ui.horizontal(|ui| {
            if ui.add_enabled(state.session.is_animating() && draft.parsed().is_ok(), egui::Button::new("Apply tangents")).clicked()
                && let Err(error) = draft.apply(state)
            {
                state.session.set_status(error);
            }
            if ui.button("Cancel tangent edits").clicked()
                && let Ok(reset) = Draft::new(state, draft.anim)
            {
                draft = reset;
            }
        });
        if !state.session.is_animating() {
            ui.label("Switch to Animate mode to edit tangents");
        }
    });
    ui.ctx().data_mut(|d| d.insert_temp(id, draft));
}
