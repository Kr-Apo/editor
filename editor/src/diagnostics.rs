//! Derived diagnostics: revision-scoped, coalesced, never saved or undoable.
use crate::{app_state::AppState, ui::Tab};
use ankhimate_document::{
    Document,
    diagnostics::{self, Issue, Location},
};
use eframe::egui;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

static NEXT_DOCUMENT: AtomicU64 = AtomicU64::new(1);
const DEBOUNCE: Duration = Duration::from_millis(200);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp {
    revision: u64,
    name: String,
    fps: u32,
}
impl Stamp {
    fn new(doc: &Document, revision: u64) -> Self {
        Self {
            revision,
            name: doc.meta.name.clone(),
            fps: doc.meta.fps,
        }
    }
}

use ankhimate_document::snapshot::{Snapshot, snapshot};
#[derive(Clone)]
pub struct Ticket {
    document: u64,
    stamp: Stamp,
    snapshot: Snapshot,
    pub issue: Issue,
}
pub struct Cache {
    identity: u64,
    observed: Option<Stamp>,
    evaluated: Option<Stamp>,
    due: Option<Duration>,
    snapshot: Option<Snapshot>,
    pub issues: Vec<Issue>,
    pub scans: u64,
    pub last_elapsed: Duration,
    pub open: bool,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            identity: NEXT_DOCUMENT.fetch_add(1, Ordering::Relaxed),
            observed: None,
            evaluated: None,
            due: None,
            snapshot: None,
            issues: Vec::new(),
            scans: 0,
            last_elapsed: Duration::ZERO,
            open: false,
        }
    }
}
impl Cache {
    pub(crate) fn document_identity(&self) -> u64 {
        self.identity
    }
    pub fn stale(&self) -> bool {
        self.evaluated.is_none() || self.observed != self.evaluated
    }
    /// Controlled monotonic elapsed time makes debounce tests deterministic.
    pub fn poll(&mut self, doc: &Document, revision: u64, now: Duration) -> Option<Duration> {
        let stamp = Stamp::new(doc, revision);
        if self.observed.as_ref() != Some(&stamp) {
            self.observed = Some(stamp.clone());
            self.due = Some(if self.evaluated.is_none() {
                now
            } else {
                now.saturating_add(DEBOUNCE)
            });
        }
        if self.due.is_some_and(|due| now >= due) {
            let start = std::time::Instant::now();
            self.issues = diagnostics::inspect(doc);
            self.snapshot = Some(snapshot(doc));
            self.evaluated = Some(stamp);
            self.due = None;
            self.scans += 1;
            self.last_elapsed = start.elapsed();
        }
        self.due.map(|due| due.saturating_sub(now))
    }
    pub fn ticket(&self, index: usize) -> Option<Ticket> {
        if self.stale() {
            return None;
        }
        Some(Ticket {
            document: self.identity,
            stamp: self.evaluated.clone()?,
            snapshot: self.snapshot.clone()?,
            issue: self.issues.get(index)?.clone(),
        })
    }
}
pub fn refresh(state: &mut AppState, ctx: &egui::Context) {
    let now = Duration::from_secs_f64(ctx.input(|i| i.time).max(0.0));
    if let Some(wait) = state.diagnostics.poll(&state.doc, state.revision, now) {
        ctx.request_repaint_after(wait);
    }
}
pub fn navigate(state: &mut AppState, ticket: &Ticket) -> bool {
    if ticket.document != state.diagnostics.identity
        || ticket.stamp != Stamp::new(&state.doc, state.revision)
        || ticket.snapshot != snapshot(&state.doc)
    {
        state.diagnostics.evaluated = None;
        state.diagnostics.observed = None;
        state
            .session
            .set_status("Issue is stale. Diagnostics will refresh; nothing was selected.");
        return false;
    }
    if state.session.has_pending_pose() || state.session.has_previews() {
        state
            .session
            .set_status("Commit or cancel the current pose preview before navigating.");
        return false;
    }
    let s = &state.doc.skeleton;
    let valid = match &ticket.issue.location {
        Location::Bone(id) => s.bones.contains_key(*id),
        Location::Slot(id) => s.slots.contains_key(*id),
        Location::Skin(id) => s.skins.contains_key(*id),
        Location::Attachment { skin, slot, name } => {
            s.skins
                .get(*skin)
                .is_some_and(|s| s.get(*slot, name).is_some())
                && s.slots
                    .get(*slot)
                    .is_some_and(|sl| s.bones.contains_key(sl.bone))
        }
        Location::Constraint(id) => s.constraints.contains_key(*id),
        Location::Animation(id) => state.doc.animations.contains_key(*id),
        Location::Key { animation, key } => {
            use ankhimate_document::commands::key_cmds::TimelineAddr as A;
            let target_exists = match key.addr {
                A::PathPosition { constraint } => {
                    ankhimate_core::path::position_target(s, constraint).is_ok()
                }
                // FFD selection has its own topology lease; diagnostic key tickets
                // remain restricted to the existing scalar/slot clipboard surface.
                ankhimate_document::commands::key_cmds::TimelineAddr::Deform { .. } => false,
                A::Bone { bone, .. } => s.bones.contains_key(bone),
                A::SlotColor { slot } | A::SlotVisible { slot } | A::SlotAttachment { slot } => {
                    s.slots.contains_key(slot)
                }
            };
            target_exists
                && state.doc.animations.get(*animation).is_some_and(|a| {
                    a.timelines
                        .iter()
                        .filter(|t| key.addr.matches_timeline(t))
                        .count()
                        == 1
                })
                && ankhimate_document::clipboard::KeyClip::capture(
                    &state.doc,
                    *animation,
                    std::slice::from_ref(key),
                )
                .is_ok()
        }
        Location::Document | Location::Asset(_) => false,
    };
    if !valid {
        state.session.set_status(
            "This issue has no safe selectable target. Use its location in the report.",
        );
        return false;
    }
    match &ticket.issue.location {
        Location::Bone(id) => {
            state.session.select_bone(Some(*id));
            state.session.focus_tab = Some(Tab::Inspector);
        }
        Location::Slot(id) => {
            state.session.select_slot(Some(*id));
            state.session.focus_tab = Some(Tab::Inspector);
        }
        Location::Attachment { skin, slot, name } => {
            let bone = state.doc.skeleton.slots[*slot].bone;
            state.session.set_active_skin(*skin);
            state.session.select_attachment(*slot, name.clone(), bone);
            state.session.focus_tab = Some(Tab::SlotEditor);
        }
        Location::Skin(id) => {
            state.session.set_active_skin(*id);
            state.session.clear_geometry_selection();
            state.session.focus_tab = Some(Tab::Skins);
        }
        Location::Constraint(id) => {
            state.session.select_constraint(*id);
            state.session.focus_tab = Some(Tab::Inspector);
        }
        Location::Animation(id) | Location::Key { animation: id, .. } => {
            // Set an EXISTING clip before entering Animate, so this path cannot
            // invoke CreateAnimation. Keep the key lease entry point unchanged.
            state.session.clear_geometry_selection();
            state.session.active_animation = Some(*id);
            state.session.playing = false;
            state.set_work_mode(crate::session::WorkMode::Animate);
            if let Location::Key { key, .. } = &ticket.issue.location {
                let capture = ankhimate_document::clipboard::KeyClip::capture(
                    &state.doc,
                    *id,
                    std::slice::from_ref(key),
                )
                .expect("validated above");
                state.session.playhead = capture.keys[0].key.time;
                crate::key_selection::set(state, *id, vec![key.clone()]);
                state.session.focus_tab = Some(Tab::Timeline);
            } else {
                crate::key_selection::set(state, *id, Vec::new());
                state.session.focus_tab = Some(Tab::Animations);
            }
        }
        _ => return false,
    }
    crate::geometry_edit::sync(state);
    state
        .session
        .set_status("Selected issue location. Camera unchanged; export preflight still applies.");
    true
}

pub fn badge(ui: &mut egui::Ui, state: &mut AppState) -> egui::Response {
    let counts = state.diagnostics.issues.iter().fold([0; 3], |mut n, i| {
        n[match i.severity {
            diagnostics::Severity::Error => 0,
            diagnostics::Severity::Warning => 1,
            diagnostics::Severity::Info => 2,
        }] += 1;
        n
    });
    let label = if state.diagnostics.stale() {
        "Diagnostics — refreshing (previous results)".into()
    } else {
        format!(
            "Diagnostics: {} errors · {} warnings · {} info",
            counts[0], counts[1], counts[2]
        )
    };
    let r = ui.button(label);
    if r.clicked() {
        state.diagnostics.open = !state.diagnostics.open;
    }
    r
}
pub fn list(ui: &mut egui::Ui, state: &mut AppState) -> Option<egui::Rect> {
    ui.label("Read-only document inspection. Export preflight checks target fidelity and file safety separately.");
    if state.diagnostics.stale() {
        ui.label("Refreshing — previous results are stale; navigation disabled.");
    }
    if state.diagnostics.issues.is_empty() && !state.diagnostics.stale() {
        ui.label("No issues detected by the current rules.");
    }
    let mut clicked = None;
    let mut first_button = None;
    egui::ScrollArea::vertical().show_rows(
        ui,
        46.0,
        state.diagnostics.issues.len(),
        |ui, range| {
            for index in range {
                let issue = &state.diagnostics.issues[index];
                let (level, color) = match issue.severity {
                    diagnostics::Severity::Error => ("Error", ui.visuals().error_fg_color),
                    diagnostics::Severity::Warning => ("Warning", ui.visuals().warn_fg_color),
                    diagnostics::Severity::Info => ("Info", ui.visuals().text_color()),
                };
                ui.push_id(index, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(color, level);
                        let response = ui.add_enabled(
                            !state.diagnostics.stale() && issue.navigable(),
                            egui::Button::new(format!("{} — {}", issue.code, issue.subject)),
                        );
                        if first_button.is_none() && response.enabled() {
                            first_button = Some(response.rect);
                        }
                        if response.clicked() {
                            clicked = Some(index);
                        }
                    });
                    ui.label(&issue.message);
                });
            }
        },
    );
    if let Some(ticket) = clicked.and_then(|index| state.diagnostics.ticket(index)) {
        navigate(state, &ticket);
    }
    first_button
}
pub fn window(ctx: &egui::Context, state: &mut AppState) {
    if !state.diagnostics.open {
        return;
    }
    let mut open = true;
    egui::Window::new("Document diagnostics")
        .open(&mut open)
        .default_width(650.0)
        .default_height(340.0)
        .show(ctx, |ui| {
            list(ui, state);
        });
    state.diagnostics.open = open;
}
#[cfg(test)]
mod tests;
