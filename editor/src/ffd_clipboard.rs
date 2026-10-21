//! Same-track FFD clipboard UI; generic scalar/pose operators remain unchanged.
use crate::{app_state::AppState, key_selection, theme::Theme};
use ankhimate_core::{
    attachment::Attachment,
    ids::{AnimationId, BoneId, SkinId},
};
use ankhimate_document::{
    clipboard::{
        Clipboard,
        deform::{DeformClip, PreparedDeformPaste},
    },
    commands::{
        attachment_cmds::owning_skin,
        key_cmds::{KeyRef, TimelineAddr},
    },
};
use eframe::egui;
use std::sync::Arc;

#[cfg(test)]
#[path = "ffd_clipboard/t835_tests.rs"]
mod order_tests;

fn context(s: &AppState, anim: AnimationId, addr: &TimelineAddr) -> Result<(), String> {
    if !s.session.is_animating() || s.session.active_animation != Some(anim) {
        return Err("Select the active clip in Animate".into());
    }
    let TimelineAddr::Deform {
        slot,
        attachment,
        owner: Some(skin),
    } = addr
    else {
        return Err("Select one exact FFD row, not a bone or a mixed selection".into());
    };
    if !s.doc.skeleton.skins.contains_key(s.session.active_skin)
        || owning_skin(&s.doc, s.session.active_skin, *slot, attachment) != Some(*skin)
        || s.pose.attachment_name(&s.doc.skeleton, *slot) != Some(attachment.as_str())
    {
        return Err("Active skin/attachment does not resolve to this FFD owner".into());
    }
    let sl = s
        .doc
        .skeleton
        .slots
        .get(*slot)
        .ok_or("FFD slot disappeared")?;
    let Some(Attachment::Mesh(m)) = s
        .doc
        .skeleton
        .skins
        .get(*skin)
        .and_then(|sk| sk.get(*slot, attachment))
    else {
        return Err("Owned FFD mesh is missing".into());
    };
    if s.session.is_bone_locked(sl.bone)
        || m.weights
            .iter()
            .flatten()
            .any(|w| s.session.is_bone_locked(w.bone))
    {
        return Err("FFD slot/influencing bone is locked".into());
    }
    Ok(())
}
fn target(refs: &[KeyRef]) -> Result<&TimelineAddr, String> {
    let first = refs
        .first()
        .ok_or("Explicitly select a destination FFD key row")?;
    if !matches!(first.addr, TimelineAddr::Deform { owner: Some(_), .. })
        || refs.iter().any(|r| r.addr != first.addr)
    {
        return Err("Select keys from exactly one owned FFD track; mixed selections refuse".into());
    }
    Ok(&first.addr)
}
/// Only the sheet's clipboard grouping uses this lease. T821 single-key mutation stays single-key.
#[derive(Clone, Debug)]
pub(crate) struct GroupLease {
    document: u64,
    revision: u64,
    animation: AnimationId,
    skin: SkinId,
    clip: Arc<DeformClip>,
    refs: Vec<KeyRef>,
}
impl GroupLease {
    pub(crate) fn new(s: &AppState, anim: AnimationId, refs: &[KeyRef]) -> Result<Self, String> {
        context(s, anim, target(refs)?)?;
        let mut refs = refs.to_vec();
        refs.sort_by_key(|r| r.index);
        let clip = Arc::new(DeformClip::capture(
            &s.doc,
            s.diagnostics.document_identity(),
            anim,
            &refs,
        )?);
        Ok(Self {
            document: s.diagnostics.document_identity(),
            revision: s.revision,
            animation: anim,
            skin: s.session.active_skin,
            clip,
            refs,
        })
    }
    pub(crate) fn matches(&self, s: &AppState) -> bool {
        self.document == s.diagnostics.document_identity()
            && self.revision == s.revision
            && s.session.active_animation == Some(self.animation)
            && s.session.active_skin == self.skin
            && s.session.is_animating()
            && self
                .clip
                .source_matches(&s.doc, self.document, self.animation, &self.refs)
    }
    pub(crate) fn refs(&self) -> Vec<KeyRef> {
        self.refs.clone()
    }
}
fn current(
    s: &mut AppState,
    anim: AnimationId,
    expected: &[KeyRef],
) -> Result<Vec<KeyRef>, String> {
    let refs = key_selection::read(s, anim);
    if refs != expected {
        return Err("FFD selection is stale; select the row again".into());
    }
    context(s, anim, target(&refs)?)?;
    Ok(refs)
}
pub(crate) fn copy(s: &mut AppState, anim: AnimationId, expected: &[KeyRef]) -> Result<(), String> {
    let refs = current(s, anim, expected)?;
    let clip = DeformClip::capture(&s.doc, s.diagnostics.document_identity(), anim, &refs)?;
    let count = clip.keys().len();
    s.session.clipboard = Clipboard::Deform(Arc::new(clip));
    s.session
        .set_status(format!("Copied {count} FFD key(s); exact mesh only"));
    Ok(())
}
struct Stamp {
    document: u64,
    revision: u64,
    animation: AnimationId,
    skin: SkinId,
    playhead: f32,
    refs: Vec<KeyRef>,
    locks: Vec<(BoneId, bool)>,
}
impl Stamp {
    fn matches(&self, s: &AppState) -> bool {
        self.document == s.diagnostics.document_identity()
            && self.revision == s.revision
            && s.session.active_animation == Some(self.animation)
            && self.skin == s.session.active_skin
            && s.session.is_animating()
            && self.playhead == s.session.playhead
            && key_selection::matches_refs(s, self.animation, &self.refs)
            && self
                .locks
                .iter()
                .all(|(b, v)| s.session.is_bone_locked(*b) == *v)
    }
}
pub(crate) struct Draft {
    stamp: Stamp,
    payload: Arc<DeformClip>,
    plan: Option<PreparedDeformPaste>,
    ack: bool,
    error: Option<String>,
    label: String,
}
impl Draft {
    fn new(s: &mut AppState, anim: AnimationId, expected: &[KeyRef]) -> Result<Self, String> {
        let refs = current(s, anim, expected)?;
        let Clipboard::Deform(payload) = &s.session.clipboard else {
            return Err("Clipboard contains no FFD keys".into());
        };
        let payload = payload.clone();
        let plan = PreparedDeformPaste::prepare(
            &s.doc,
            s.diagnostics.document_identity(),
            anim,
            payload.clone(),
            target(&refs)?,
            s.session.playhead,
        )?;
        let mut bones = plan.affected_bones();
        bones.sort();
        bones.dedup();
        let locks = bones
            .into_iter()
            .map(|b| (b, s.session.is_bone_locked(b)))
            .collect();
        let TimelineAddr::Deform {
            slot,
            attachment,
            owner: Some(skin),
        } = payload.address()
        else {
            return Err("Unresolved FFD owner".into());
        };
        let label = format!(
            "{} / {} / {}",
            s.doc.skeleton.skins[*skin].name, s.doc.skeleton.slots[*slot].name, attachment
        );
        Ok(Self {
            stamp: Stamp {
                document: s.diagnostics.document_identity(),
                revision: s.revision,
                animation: anim,
                skin: s.session.active_skin,
                playhead: s.session.playhead,
                refs,
                locks,
            },
            payload,
            plan: Some(plan),
            ack: false,
            error: None,
            label,
        })
    }
    fn invalidate(&mut self, s: &AppState) {
        if !self.stamp.matches(s)
            || !matches!(&s.session.clipboard,Clipboard::Deform(p)if Arc::ptr_eq(p,&self.payload))
        {
            self.plan = None;
            self.ack = false;
            self.error = Some("FFD paste context changed; cancel and prepare again.".into());
        }
    }
    fn enabled(&self) -> bool {
        self.plan
            .as_ref()
            .is_some_and(|p| p.summary.outside_duration == 0 || self.ack)
    }
    fn apply(&mut self, s: &mut AppState) -> bool {
        self.invalidate(s);
        if !self.enabled() {
            return false;
        }
        if let Err(e) = context(s, self.stamp.animation, self.payload.address()) {
            self.plan = None;
            self.error = Some(e);
            return false;
        }
        let p = self.plan.take().expect("enabled");
        let result = p.into_command(&s.doc, s.diagnostics.document_identity(), &self.payload);
        match result {
            Ok(c) => {
                let done = s.dispatch(c);
                if done {
                    s.session
                        .set_status("Pasted FFD keys (one Undo); duration unchanged");
                }
                done
            }
            Err(e) => {
                self.error = Some(e);
                false
            }
        }
    }
}
pub(crate) fn menu(ui: &mut egui::Ui, s: &mut AppState, anim: AnimationId, refs: &[KeyRef]) {
    let reason = target(refs).and_then(|addr| context(s, anim, addr)).err();
    ui.label(format!(
        "FFD clipboard: {} selected key(s); exact mesh only",
        refs.len()
    ));
    let r = ui.add_enabled(reason.is_none(), egui::Button::new("Copy FFD Keys"));
    probe(ui.ctx(), "copy", &r);
    if r.clicked() {
        if let Err(e) = copy(s, anim, refs) {
            s.session.set_status(format!("Copy FFD refused: {e}"));
        }
        ui.close();
    }
    let r = ui.add_enabled(
        reason.is_none() && matches!(s.session.clipboard, Clipboard::Deform(_)),
        egui::Button::new("Paste FFD Keys…"),
    );
    probe(ui.ctx(), "paste", &r);
    if r.clicked() {
        match Draft::new(s, anim, refs) {
            Ok(d) => s.session.ffd_paste = Some(d),
            Err(e) => s.session.set_status(format!("Paste FFD refused: {e}")),
        }
        ui.close();
    }
    if let Some(e) = reason {
        ui.weak(e);
    }
    ui.weak("Same document/owned mesh only. Choose an existing Deform row; collisions refuse, no silent replacement.");
}
pub(crate) fn show(ctx: &egui::Context, s: &mut AppState, theme: &Theme) {
    let Some(mut d) = s.session.ffd_paste.take() else {
        return;
    };
    d.invalidate(s);
    let response = crate::ui::dialog::Dialog::new("paste_ffd_keys", "Paste FFD Keys")
        .width(520.)
        .show(ctx, theme, |ui| {
            ui.label(format!("Target: {}", d.label));
            if let Some(p) = &d.plan {
                let v = &p.summary;
                ui.label(format!("{} FFD keys · {} → {} s", v.keys, v.first, v.last));
                ui.label(format!(
                    "Clip duration unchanged: {} s · beyond duration: {}",
                    v.duration, v.outside_duration
                ));
                ui.label("Vectors/interpolations preserved; existing keys are never replaced.");
                if v.outside_duration > 0 {
                    let r = ui.checkbox(
                        &mut d.ack,
                        "I accept keys outside the unchanged clip duration.",
                    );
                    probe(ctx, "ack", &r);
                }
            }
            if let Some(e) = &d.error {
                ui.colored_label(egui::Color32::LIGHT_RED, e);
            }
            ui.horizontal(|ui| {
                let a = ui.add_enabled(d.enabled(), egui::Button::new("Apply FFD Paste"));
                probe(ctx, "apply", &a);
                let c = ui.button("Cancel");
                probe(ctx, "cancel", &c);
                (a.clicked(), c.clicked())
            })
            .inner
        });
    if response.closed || response.inner.1 {
        return;
    }
    if response.inner.0 && d.apply(s) {
        return;
    }
    s.session.ffd_paste = Some(d);
}
fn probe(_ctx: &egui::Context, _name: &str, _r: &egui::Response) {
    #[cfg(test)]
    _ctx.data_mut(|d| d.insert_temp(egui::Id::new(("t834", _name)), _r.rect));
}
#[cfg(test)]
mod tests;
