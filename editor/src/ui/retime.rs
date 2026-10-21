//! Numeric retiming: explicit preparation, one native command, transient context lease.
use crate::{app_state::AppState, theme::Theme};
use ankhimate_core::{
    animation::Timeline,
    ids::{AnimationId, BoneId, SlotId},
};
use ankhimate_document::commands::{
    document_cmds::SetProjectFps,
    key_cmds::{PreparedRetime, RetimeAnimation},
};
use eframe::egui;
// Pose: Offset Selected Bones… uses an explicit, separate scope.
// Pose → Offset Selected Bones… is an explicit, separate scope.
mod selected;
#[derive(PartialEq)]
struct Stamp {
    document: u64,
    revision: u64,
    animation: Option<AnimationId>,
    fps: u32,
    factor: String,
    frames: String,
    animate: bool,
    bones: Vec<BoneId>,
    slots: Vec<SlotId>,
    locked: Vec<BoneId>,
}
impl Stamp {
    fn new(s: &AppState, factor: &str, frames: &str) -> Self {
        Self {
            document: s.diagnostics.document_identity(),
            revision: s.revision,
            factor: factor.into(),
            frames: frames.into(),
            animation: s.session.active_animation,
            fps: s.doc.meta.fps,
            animate: s.session.is_animating(),
            bones: s.session.selected_bones.clone(),
            slots: s.session.selected_slots.clone(),
            locked: s
                .session
                .locked_bones
                .iter()
                .filter_map(|(b, v)| v.then_some(b))
                .collect(),
        }
    }
}
#[derive(Default)]
pub struct Panel {
    draft: Option<Draft>,
    selected: selected::Panel,
}
struct Draft {
    factor: String,
    frames: String,
    prepared: Option<(Stamp, PreparedRetime)>,
    error: Option<String>,
    acknowledged: bool,
}
impl Default for Draft {
    fn default() -> Self {
        Self {
            factor: "1".into(),
            frames: "0".into(),
            prepared: None,
            error: None,
            acknowledged: false,
        }
    }
}
fn parse(text: &str) -> Result<f64, String> {
    text.trim()
        .replace('−', "-")
        .replace(',', ".")
        .parse::<f64>()
        .map_err(|_| "Enter finite numeric values".into())
}
fn prepare(s: &AppState, factor: &str, frames: &str) -> Result<(Stamp, PreparedRetime), String> {
    if !s.session.is_animating() {
        return Err("Switch to Animate and select a clip".into());
    }
    let id = s.session.active_animation.ok_or("Select a clip")?;
    let clip = s.doc.animations.get(id).ok_or("Clip no longer exists")?;
    if !(SetProjectFps::MIN..=SetProjectFps::MAX).contains(&s.doc.meta.fps) {
        return Err("Set a valid project FPS (1–240) before preparing".into());
    }
    for t in &clip.timelines {
        if let Timeline::PathPosition { constraint, .. } = t {
            crate::path_keys::editable(s, *constraint)?;
            ankhimate_document::path_keys::Capture::read(&s.doc, id, *constraint)?;
        }
        let mut bones = Vec::new();
        if let Some(b) = t.bone() {
            bones.push(b);
        }
        match t {
            Timeline::SlotVisible { slot, .. }
            | Timeline::SlotColor { slot, .. }
            | Timeline::SlotAttachment { slot, .. }
            | Timeline::Deform { slot, .. } => {
                if let Some(slot) = s.doc.skeleton.slots.get(*slot) {
                    bones.push(slot.bone);
                }
            }
            Timeline::DrawOrder { .. } => {
                bones.extend(s.doc.skeleton.slots.values().map(|s| s.bone))
            }
            Timeline::PathPosition { constraint, .. }
            | Timeline::IkMix { constraint, .. }
            | Timeline::IkBendDirection { constraint, .. }
            | Timeline::IkSoftness { constraint, .. }
            | Timeline::TransformConstraintMix { constraint, .. } => {
                if let Some(c) = s.doc.skeleton.constraints.get(*constraint) {
                    bones.extend(c.affected_bones());
                    bones.extend(c.target());
                }
            }
            _ => {}
        }
        if bones.iter().any(|b| s.session.is_bone_locked(*b)) {
            return Err(
                "A targeted track uses a locked bone; unlock it or cancel (whole clip operation)"
                    .into(),
            );
        }
    }
    let frame_text = frames;
    let frames = parse(frames)?;
    if !frames.is_finite() {
        return Err("Offset frames must be finite".into());
    }
    let p = RetimeAnimation::prepare_numeric(
        id,
        clip,
        parse(factor)?,
        frames / f64::from(s.doc.meta.fps),
    )?;
    Ok((Stamp::new(s, factor, frame_text), p))
}
fn needs_ack(p: &PreparedRetime) -> bool {
    let v = &p.summary;
    v.clamped > 0 || v.outside_duration > 0 || v.events > 0 || v.markers > 0 || v.bone_offsets > 0
}
impl Draft {
    fn prepare(&mut self, s: &AppState) {
        self.acknowledged = false;
        match prepare(s, &self.factor, &self.frames) {
            Ok(p) => {
                self.prepared = Some(p);
                self.error = None;
            }
            Err(e) => {
                self.prepared = None;
                self.error = Some(e);
            }
        }
    }
    fn invalidate(&mut self, s: &AppState) {
        if self
            .prepared
            .as_ref()
            .is_some_and(|(stamp, _)| *stamp != Stamp::new(s, &self.factor, &self.frames))
        {
            self.prepared = None;
            self.acknowledged = false;
            self.error = Some("Context changed. Prepare again before Apply.".into());
        }
    }
    fn apply(&mut self, s: &mut AppState) -> bool {
        self.invalidate(s);
        let Some((_, p)) = &self.prepared else {
            return false;
        };
        if p.summary.no_op || (needs_ack(p) && !self.acknowledged) {
            return false;
        }
        let (_, p) = self.prepared.take().expect("checked");
        s.dispatch(Box::new(p.command))
    }
}
impl Panel {
    pub fn menu(&mut self, ui: &mut egui::Ui, s: &AppState) {
        self.selected.menu(ui, s);
        let r = ui.add_enabled(
            s.session.is_animating() && s.session.active_animation.is_some(),
            egui::Button::new("Retime…"),
        );
        probe(ui.ctx(), "menu", &r);
        if r.clicked() {
            self.draft = Some(Draft::default());
            ui.close();
        }
    }
    pub fn show(&mut self, ctx: &egui::Context, s: &mut AppState, theme: &Theme) {
        self.selected.show(ctx, s, theme);
        let Some(mut d) = self.draft.take() else {
            return;
        };
        d.invalidate(s);
        let response=super::dialog::Dialog::new("numeric_retime","Retime Animation").width(510.0).show(ctx,theme,|ui|{
            let name=s.session.active_animation.and_then(|id|s.doc.animations.get(id)).map_or("No clip",|a|a.name.as_str());
            ui.label(format!("Clip: {name} · project FPS: {}",s.doc.meta.fps));
            ui.label("All clip timelines, regardless of bone selection (slots, constraints and FFD included).");
            ui.label("Events, markers and bone evaluation offsets stay unchanged.");
            ui.horizontal(|ui|{ui.label("Time Scale");let r=ui.text_edit_singleline(&mut d.factor);probe(ctx,"factor",&r);if r.changed(){d.prepared=None;d.acknowledged=false;}});
            ui.horizontal(|ui|{ui.label("Offset in Frames");let r=ui.text_edit_singleline(&mut d.frames);probe(ctx,"frames",&r);if r.changed(){d.prepared=None;d.acknowledged=false;}});
            ui.label("Times = time × scale + offset, clamped to zero. Nonzero offset preserves clip duration, even with scaling.");
            let r=ui.button("Prepare preview");probe(ctx,"prepare",&r);if r.clicked(){d.prepare(s);}
            if let Some((_,p))=&d.prepared{
                let v=&p.summary;
                ui.label(format!("Scale: {} · {} frames = {} s",v.factor,d.frames,v.offset));
                ui.label(format!("Duration: {} → {} s · {} tracks · {} keys ({} changed)",v.duration_before,v.duration_after,v.tracks,v.keys,v.changed_keys));
                ui.label(format!("First/last: {:?} → {:?} s",v.before_range,v.after_range));
                ui.label(format!("Collisions: 0 · clamped: {} · beyond duration: {}",v.clamped,v.outside_duration));
                ui.label(format!("Unchanged: {} events, {} markers, {} bone offsets",v.events,v.markers,v.bone_offsets));
                if v.no_op{ui.label("No change; Apply is disabled.");}
                if needs_ack(p){let r=ui.checkbox(&mut d.acknowledged,"I accept the reported clamp/outside-duration and unchanged timing metadata.");probe(ctx,"ack",&r);}
            }
            if let Some(e)=&d.error{ui.colored_label(egui::Color32::LIGHT_RED,e);}
            ui.horizontal(|ui|{
                let enabled=d.prepared.as_ref().is_some_and(|(_,p)|!p.summary.no_op&&(!needs_ack(p)||d.acknowledged));
                let a=ui.add_enabled(enabled,egui::Button::new("Apply"));probe(ctx,"apply",&a);
                let c=ui.button("Cancel");probe(ctx,"cancel",&c);(a.clicked(),c.clicked())
            }).inner
        });
        if response.closed || response.inner.1 {
            return;
        }
        if response.inner.0 && d.apply(s) {
            return;
        }
        self.draft = Some(d);
    }
}
fn probe(_ctx: &egui::Context, _name: &str, _r: &egui::Response) {
    #[cfg(test)]
    _ctx.data_mut(|d| d.insert_temp(egui::Id::new(("t830", _name)), _r.rect));
}
#[cfg(test)]
mod tests;
