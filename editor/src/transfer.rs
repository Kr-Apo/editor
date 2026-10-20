//! Explicit local project import. Source readers never replace the destination.
use crate::app_state::AppState;
use ankhimate_core::ids::BoneId;
use ankhimate_document::{
    Document,
    snapshot::{Snapshot, snapshot},
    transfer::Plan,
};
use eframe::egui;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};

struct Guard {
    identity: u64,
    revision: u64,
    name: String,
    fps: u32,
    snapshot: Snapshot,
}
impl Guard {
    fn capture(s: &AppState) -> Self {
        Self {
            identity: s.diagnostics.document_identity(),
            revision: s.revision,
            name: s.doc.meta.name.clone(),
            fps: s.doc.meta.fps,
            snapshot: snapshot(&s.doc),
        }
    }
    fn quick(&self, s: &AppState) -> bool {
        self.identity == s.diagnostics.document_identity()
            && self.revision == s.revision
            && self.name == s.doc.meta.name
            && self.fps == s.doc.meta.fps
    }
    fn matches(&self, s: &AppState) -> bool {
        self.quick(s) && self.snapshot == snapshot(&s.doc)
    }
}
struct Source {
    path: PathBuf,
    bytes: Vec<u8>,
    snapshot: Snapshot,
    doc: Document,
}
impl Source {
    fn load(path: &Path) -> Result<Self, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("Cannot read source: {e}"))?;
        let loaded = ankhimate_formats::load(path)
            .map_err(|e| format!("Cannot load source: {e}"))?
            .0;
        if !loaded.report.is_clean() {
            return Err("Source load report has unresolved or lossy data; transfer refused".into());
        }
        if std::fs::read(path).map_err(|e| e.to_string())? != bytes {
            return Err("Source changed while loading; select it again".into());
        }
        let doc = Document::from_loaded(loaded);
        Ok(Self {
            path: path.into(),
            bytes,
            snapshot: snapshot(&doc),
            doc,
        })
    }
    fn verify(&self) -> Result<(), String> {
        let fresh = Self::load(&self.path)?;
        if fresh.bytes != self.bytes || fresh.snapshot != self.snapshot {
            return Err("Source project or image bytes changed; reload and prepare again".into());
        }
        Ok(())
    }
}
struct Loading {
    guard: Guard,
    receiver: Receiver<Result<Source, String>>,
}
struct Review {
    guard: Guard,
    plan: Plan,
}
#[derive(Default)]
pub struct Panel {
    pub open: bool,
    source: Option<Source>,
    loading: Option<Loading>,
    root: Option<BoneId>,
    parent: Option<BoneId>,
    review: Option<Review>,
    error: Option<String>,
}
pub fn start(s: &mut AppState, path: PathBuf) {
    if !s.session.can_edit_structure() {
        s.session
            .set_status("Switch to Setup mode to transfer a rig subtree");
        return;
    }
    let guard = Guard::capture(s);
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(Source::load(&path));
    });
    s.transfer = Panel {
        open: true,
        loading: Some(Loading { guard, receiver }),
        ..Default::default()
    };
}
pub fn poll(s: &mut AppState, ctx: &egui::Context) {
    let Some(loading) = &s.transfer.loading else {
        return;
    };
    match loading.receiver.try_recv() {
        Ok(result) => {
            let valid = loading.guard.matches(s);
            s.transfer.loading = None;
            if !valid {
                s.transfer.error=Some("Destination changed during source loading; choose the source again. Nothing imported.".into());
                return;
            }
            match result {
                Ok(source) => s.transfer.source = Some(source),
                Err(e) => s.transfer.error = Some(e),
            }
        }
        Err(mpsc::TryRecvError::Empty) => {
            ctx.request_repaint_after(std::time::Duration::from_millis(50))
        }
        Err(_) => {
            s.transfer.loading = None;
            s.transfer.error = Some("Source reader stopped; destination preserved".into());
        }
    }
}
fn editable(s: &AppState) -> bool {
    s.session.can_edit_structure()
        && !s.session.has_pending_pose()
        && !s.session.has_previews()
        && s.transfer
            .parent
            .is_none_or(|b| s.doc.skeleton.bones.contains_key(b) && !s.session.is_bone_locked(b))
}
pub fn prepare(s: &mut AppState) -> Result<(), String> {
    if !editable(s) {
        return Err("Setup mode and an unlocked existing parent are required; commit/cancel pose previews first".into());
    }
    s.transfer.review = None;
    let source = s
        .transfer
        .source
        .as_ref()
        .ok_or("Choose a source project first")?;
    let root = s.transfer.root.ok_or("Choose one source subtree root")?;
    let plan = Plan::prepare(&source.doc, &s.doc, root, s.transfer.parent)?;
    s.transfer.review = Some(Review {
        guard: Guard::capture(s),
        plan,
    });
    Ok(())
}
pub fn approve(s: &mut AppState) -> Result<(), String> {
    if !editable(s) {
        return Err("Transfer is not editable in this mode or parent/preview context".into());
    }
    let review = s
        .transfer
        .review
        .take()
        .ok_or("Prepare and review a transfer first")?;
    if !review.guard.matches(s) {
        return Err("Destination changed; prepare again. Nothing imported.".into());
    }
    s.transfer
        .source
        .as_ref()
        .ok_or("Source is no longer available")?
        .verify()?;
    let root = review.plan.root;
    let command = review.plan.into_command(&s.doc)?;
    if !s.dispatch(Box::new(command)) {
        return Err("Native command refused; destination preserved".into());
    }
    s.session.select_bone(Some(root));
    s.session.set_status(
        "Rig subtree transferred as one Undo step. Save to make the copied images portable.",
    );
    s.transfer = Default::default();
    Ok(())
}
pub fn cancel(s: &mut AppState) {
    s.transfer = Default::default();
}
/// Returns the confirm/cancel rects for deterministic in-memory interaction tests.
pub fn contents(ui: &mut egui::Ui, s: &mut AppState) -> (Option<egui::Rect>, egui::Rect) {
    ui.label("Source is read-only; the destination stays open. Only reviewed compatible content is imported.");
    if s.transfer.loading.is_some() {
        ui.label("Reading source project and image bytes…");
    }
    if let Some(e) = &s.transfer.error {
        ui.colored_label(ui.visuals().error_fg_color, e);
    }
    let mut root = s.transfer.root;
    let mut parent = s.transfer.parent;
    if let Some(source) = &s.transfer.source {
        ui.label(format!(
            "Source: {} ({})",
            source.doc.meta.name,
            source
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project")
        ));
        egui::ComboBox::from_id_salt("transfer_source_root")
            .selected_text(
                root.and_then(|id| source.doc.skeleton.bones.get(id))
                    .map(|b| b.name.as_str())
                    .unwrap_or("Choose subtree root…"),
            )
            .show_ui(ui, |ui| {
                for id in &source.doc.skeleton.update_order {
                    if let Some(b) = source.doc.skeleton.bones.get(*id) {
                        let parent_name = b
                            .parent
                            .and_then(|id| source.doc.skeleton.bones.get(id))
                            .map(|b| b.name.as_str())
                            .unwrap_or("root");
                        ui.selectable_value(
                            &mut root,
                            Some(*id),
                            format!("{} — parent: {}", b.name, parent_name),
                        );
                    }
                }
            });
        egui::ComboBox::from_id_salt("transfer_parent")
            .selected_text(
                parent
                    .and_then(|id| s.doc.skeleton.bones.get(id))
                    .map(|b| b.name.as_str())
                    .unwrap_or("New root — preserve LOCAL values"),
            )
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut parent, None, "New root — preserve LOCAL values");
                for id in &s.doc.skeleton.update_order {
                    if let Some(b) = s.doc.skeleton.bones.get(*id) {
                        ui.add_enabled_ui(!s.session.is_bone_locked(*id), |ui| {
                            ui.selectable_value(&mut parent, Some(*id), &b.name);
                        });
                    }
                }
            });
    }
    if root != s.transfer.root || parent != s.transfer.parent {
        s.transfer.review = None;
        s.transfer.root = root;
        s.transfer.parent = parent;
    }
    if ui
        .add_enabled(
            editable(s) && root.is_some() && s.transfer.source.is_some(),
            egui::Button::new("Prepare transfer (no changes)"),
        )
        .clicked()
    {
        if let Err(e) = prepare(s) {
            s.transfer.error = Some(e);
        } else {
            s.transfer.error = None;
        }
    }
    let mut confirm = None;
    if let Some(review) = &s.transfer.review {
        let a = &review.plan.summary;
        ui.separator();
        ui.label(format!(
            "{} bones · {} slots · {} regions · {} images · {} constraints · {} clips · {} keys",
            a.bones.len(),
            a.slots.len(),
            a.attachments,
            a.images.len(),
            a.constraints.len(),
            a.clips.len(),
            a.keys
        ));
        for note in &a.notes {
            ui.label(note);
        }
        let renamed = a
            .bones
            .iter()
            .chain(&a.slots)
            .chain(&a.constraints)
            .chain(&a.clips)
            .filter(|(a, b)| a != b)
            .count()
            + a.images
                .iter()
                .filter(|i| i.source != i.destination)
                .count();
        if renamed > 0 {
            ui.colored_label(ui.visuals().warn_fg_color,format!("{renamed} name mappings differ from the source. Review collision/reuse details below; existing objects are never overwritten."));
        }
        ui.collapsing("Image bytes / collision resolution", |ui| {
            for image in &a.images {
                ui.label(format!(
                    "{} → {}: {}, {} bytes",
                    image.source,
                    image.destination,
                    if image.reused {
                        "REUSE exact bytes/dimensions"
                    } else {
                        "IMPORT; never overwrite"
                    },
                    image.bytes
                ));
            }
        });
        ui.collapsing("All bone, slot, constraint and clip name mappings", |ui| {
            for (kind, items) in [
                ("Bone", &a.bones),
                ("Slot", &a.slots),
                ("Constraint", &a.constraints),
                ("New clip", &a.clips),
            ] {
                for (old, new) in items {
                    ui.label(format!("{kind}: {old} → {new}"));
                }
            }
        });
        let current = review.guard.quick(s);
        if !current {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "Review is stale. Prepare again before confirmation.",
            );
        }
        let response = ui.add_enabled(
            current && editable(s),
            egui::Button::new("Confirm reviewed transfer"),
        );
        confirm = Some(response.rect);
        if response.clicked()
            && let Err(e) = approve(s)
        {
            s.transfer.error = Some(e);
        }
    }
    let c = ui.button("Cancel transfer");
    if c.clicked() {
        cancel(s);
    }
    (confirm, c.rect)
}
pub fn window(ctx: &egui::Context, s: &mut AppState) {
    if !s.transfer.open {
        return;
    }
    let mut open = true;
    egui::Window::new("Import from Ankhimate Project")
        .open(&mut open)
        .default_width(680.0)
        .default_height(520.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                contents(ui, s);
            });
        });
    if !open {
        cancel(s);
    }
}
#[cfg(test)]
mod tests;
