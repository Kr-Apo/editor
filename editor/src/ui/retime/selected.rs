//! The selected-bone action has no fallback to the whole clip or descendants.
use super::{AppState, Theme, egui, parse};
use ankhimate_core::ids::{AnimationId, BoneId};
use ankhimate_document::commands::key_cmds::PreparedBoneOffset;

#[derive(PartialEq)]
struct Stamp {
    document: u64,
    revision: u64,
    clip: Option<AnimationId>,
    fps: u32,
    animate: bool,
    bones: Vec<BoneId>,
    locks: Vec<bool>,
    frames: String,
}
impl Stamp {
    fn new(s: &AppState, frames: &str) -> Self {
        Self {
            document: s.diagnostics.document_identity(),
            revision: s.revision,
            clip: s.session.active_animation,
            fps: s.doc.meta.fps,
            animate: s.session.is_animating(),
            bones: s.session.selected_bones.clone(),
            locks: s
                .session
                .selected_bones
                .iter()
                .map(|b| s.session.is_bone_locked(*b))
                .collect(),
            frames: frames.into(),
        }
    }
}
#[derive(Default)]
pub(super) struct Panel {
    draft: Option<Draft>,
}
struct Draft {
    frames: String,
    prepared: Option<(Stamp, PreparedBoneOffset)>,
    error: Option<String>,
    ack: bool,
}
impl Default for Draft {
    fn default() -> Self {
        Self {
            frames: "0".into(),
            prepared: None,
            error: None,
            ack: false,
        }
    }
}
fn available(s: &AppState) -> Result<(), String> {
    if !s.session.is_animating() {
        return Err("Switch to Animate".into());
    }
    if s.session
        .active_animation
        .and_then(|id| s.doc.animations.get(id))
        .is_none()
    {
        return Err("Select a clip".into());
    }
    if s.session.selected_bones.is_empty() {
        return Err("Select at least one bone (no whole-clip fallback)".into());
    }
    if s.session
        .selected_bones
        .iter()
        .any(|b| !s.doc.skeleton.bones.contains_key(*b))
    {
        return Err("Selected bone no longer exists".into());
    }
    Ok(())
}
impl Draft {
    fn prepare(&mut self, s: &AppState) {
        self.prepared = None;
        self.ack = false;
        let result = (|| {
            available(s)?;
            let p = PreparedBoneOffset::prepare(
                &s.doc,
                s.session.active_animation.ok_or("Select a clip")?,
                &s.session.selected_bones,
                parse(&self.frames)?,
            )?;
            if p.affected_bones().any(|b| s.session.is_bone_locked(b)) {
                return Err("A targeted bone is locked; nothing will be offset".into());
            }
            Ok((Stamp::new(s, &self.frames), p))
        })();
        match result {
            Ok(p) => {
                self.prepared = Some(p);
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn invalidate(&mut self, s: &AppState) {
        if self
            .prepared
            .as_ref()
            .is_some_and(|(stamp, _)| *stamp != Stamp::new(s, &self.frames))
        {
            self.prepared = None;
            self.ack = false;
            self.error = Some("Context changed. Prepare again before Apply.".into());
        }
    }
    fn enabled(&self) -> bool {
        self.prepared.as_ref().is_some_and(|(_, p)| {
            p.summary.changed_keys > 0 && (!p.summary.needs_ack() || self.ack)
        })
    }
    fn apply(&mut self, s: &mut AppState) -> bool {
        self.invalidate(s);
        if !self.enabled() {
            return false;
        }
        let (_, p) = self.prepared.take().expect("enabled");
        match p.into_command(&s.doc) {
            Ok(Some(c)) => s.dispatch(c),
            Ok(None) => false,
            Err(e) => {
                self.error = Some(e);
                false
            }
        }
    }
}
impl Panel {
    pub(super) fn menu(&mut self, ui: &mut egui::Ui, s: &AppState) {
        let reason = available(s).err();
        let r = ui.add_enabled(
            reason.is_none(),
            egui::Button::new("Offset Selected Bones…"),
        );
        probe(ui.ctx(), "menu", &r);
        let r=r.on_hover_text(reason.as_deref().unwrap_or("Offset directly owned transform keys only; duration stays unchanged. No automatic descendants."));
        if r.clicked() {
            self.draft = Some(Draft::default());
            ui.close();
        }
    }
    pub(super) fn show(&mut self, ctx: &egui::Context, s: &mut AppState, theme: &Theme) {
        let Some(mut d) = self.draft.take() else {
            return;
        };
        d.invalidate(s);
        let response=crate::ui::dialog::Dialog::new("selected_bone_offset","Offset Selected Bones").width(530.).show(ctx,theme,|ui|{
            let name=s.session.active_animation.and_then(|id|s.doc.animations.get(id)).map_or("No clip",|a|a.name.as_str());
            ui.label(format!("Clip: {name} · committed project FPS: {}",s.doc.meta.fps));
            ui.label(format!("{} explicitly selected bones (no descendants or whole-clip fallback)",s.session.selected_bones.len()));
            ui.label("Only Translate X/Y, Rotate, Scale X/Y and Shear X/Y. Duration, other tracks, events/markers/evaluation offsets stay unchanged.");
            ui.horizontal(|ui|{ui.label("Offset in Frames");let r=ui.text_edit_singleline(&mut d.frames);probe(ctx,"frames",&r);if r.changed(){d.prepared=None;d.ack=false;}});
            let r=ui.button("Prepare preview");probe(ctx,"prepare",&r);if r.clicked(){d.prepare(s);}
            if let Some((_,p))=&d.prepared{
                let v=&p.summary;
                ui.label(format!("Bones: {}",v.bones.iter().map(|(_,n)|n.as_str()).collect::<Vec<_>>().join(", ")));
                ui.label(format!("{} frames / {} FPS = {} s · duration unchanged: {} s",v.frames,v.fps,v.offset,v.duration));
                ui.label(format!("{} tracks · {} keys · {} changed",v.tracks,v.keys,v.changed_keys));
                ui.label(format!("First/last: {:?} → {:?} s",v.before_range,v.after_range));
                ui.label(format!("Collisions: 0 · clamped: {} · beyond duration: {}",v.clamped,v.outside_duration));
                if v.changed_keys==0{ui.label("No change. Apply is disabled.");}
                if v.needs_ack(){let r=ui.checkbox(&mut d.ack,"I accept the reported clamp/outside-duration keys; duration stays unchanged.");probe(ctx,"ack",&r);}
            }
            if let Some(e)=&d.error{ui.colored_label(egui::Color32::LIGHT_RED,e);}
            ui.horizontal(|ui|{let a=ui.add_enabled(d.enabled(),egui::Button::new("Apply"));probe(ctx,"apply",&a);let c=ui.button("Cancel");probe(ctx,"cancel",&c);(a.clicked(),c.clicked())}).inner
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
    _ctx.data_mut(|d| d.insert_temp(egui::Id::new(("t833", _name)), _r.rect));
}
#[cfg(test)]
mod tests;
