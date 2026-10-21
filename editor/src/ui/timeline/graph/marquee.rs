//! A transient graph gesture, committing through the existing KeySelection only.
use super::*;
use ankhimate_document::clipboard::{KeyClip, keys::NamedTrack};
use std::sync::Arc;
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Point {
    pub key: KeyRef,
    pub center: egui::Pos2,
}
#[derive(Clone, PartialEq)]
pub(super) struct Mapping {
    pub rect: egui::Rect,
    pub horizontal: [f32; 3],
    pub bounds: [f64; 2],
}
struct Lease {
    identity: u64,
    revision: u64,
    anim: AnimationId,
    animation: Animation,
    selection: Vec<KeyRef>,
    bones: Vec<ankhimate_core::ids::BoneId>,
    slots: Vec<ankhimate_core::ids::SlotId>,
    focus: Option<crate::session::Selection>,
    points: Vec<Point>,
    mapping: Mapping,
}
#[derive(Clone)]
struct Gesture {
    start: egui::Pos2,
    additive: bool,
    dragging: bool,
    lease: Arc<Lease>,
    last_frame: u64,
}
pub(super) fn available(state: &AppState, animation: &Animation, addr: &TimelineAddr) -> bool {
    animation
        .timelines
        .iter()
        .filter(|t| addr.matches_timeline(t))
        .count()
        == 1
        && NamedTrack::capture(&state.doc, addr).is_ok()
}
fn valid_point(p: egui::Pos2) -> bool {
    p.x.is_finite() && p.y.is_finite()
}
pub(super) fn band(start: egui::Pos2, end: egui::Pos2, viewport: egui::Rect) -> Option<egui::Rect> {
    (valid_point(start) && valid_point(end) && viewport.is_finite() && viewport.is_positive())
        .then(|| egui::Rect::from_two_pos(start, end).intersect(viewport))
        .filter(|r| r.is_positive())
}
/// Full typed equality decides duplicates; the hash only chooses a bucket.
fn union(initial: &[KeyRef], hits: impl IntoIterator<Item = KeyRef>) -> Vec<KeyRef> {
    let mut result = Vec::new();
    let mut buckets = std::collections::HashMap::<(u64, usize), Vec<KeyRef>>::new();
    for key in initial.iter().cloned().chain(hits) {
        let bucket = buckets
            .entry((key.addr.stable_id(), key.index))
            .or_default();
        if !bucket.contains(&key) {
            bucket.push(key.clone());
            result.push(key);
        }
    }
    result
}
pub(super) fn pick(points: &[Point], band: egui::Rect, initial: &[KeyRef]) -> Vec<KeyRef> {
    union(
        initial,
        points
            .iter()
            .filter(|p| valid_point(p.center) && band.contains(p.center))
            .map(|p| p.key.clone()),
    )
}
impl Lease {
    fn matches(
        &self,
        s: &AppState,
        anim: AnimationId,
        animation: &Animation,
        selection: &[KeyRef],
        points: &[Point],
        mapping: &Mapping,
    ) -> bool {
        self.identity == s.diagnostics.document_identity()
            && self.revision == s.revision
            && self.anim == anim
            && s.session.active_animation == Some(anim)
            && s.session.is_animating()
            && self.animation == *animation
            && self.selection == selection
            && self.bones == s.session.selected_bones
            && self.slots == s.session.selected_slots
            && self.focus == s.session.selection
            && self.points == points
            && self.mapping == *mapping
    }
}
#[allow(
    clippy::too_many_arguments,
    reason = "Same graph inputs, no alternative selection or coordinate model"
)]
pub(super) fn ui(
    ui: &mut egui::Ui,
    s: &mut AppState,
    anim: AnimationId,
    animation: &Animation,
    selection: &[KeyRef],
    points: Vec<Point>,
    mapping: Mapping,
    bg: &egui::Response,
    painter: &egui::Painter,
) {
    let id = ui.id().with("graph_marquee");
    let frame = ui.ctx().cumulative_frame_nr();
    let mut gesture = ui.ctx().data(|d| d.get_temp::<Gesture>(id));
    ui.ctx().data_mut(|d| d.remove::<Gesture>(id));
    let (pointer, origin, down, released, focused, cancel, mods, other, scroll) = ui.input(|i| {
        (
            i.pointer.interact_pos(),
            i.pointer.press_origin(),
            i.pointer.primary_down(),
            i.pointer.primary_released(),
            i.focused,
            i.key_pressed(egui::Key::Escape)
                || i.raw
                    .events
                    .iter()
                    .any(|e| matches!(e, egui::Event::PointerGone)),
            i.modifiers,
            i.pointer.button_down(egui::PointerButton::Middle)
                || i.pointer.button_down(egui::PointerButton::Secondary),
            i.smooth_scroll_delta != egui::Vec2::ZERO || (i.zoom_delta() - 1.).abs() > 1e-4,
        )
    });
    let admissible = focused
        && !cancel
        && !other
        && !scroll
        && !mods.alt
        && s.session.is_animating()
        && s.session.active_animation == Some(anim)
        && pointer.is_some_and(|p| valid_point(p) && mapping.rect.contains(p));
    if !admissible {
        return;
    }
    if gesture.as_ref().is_some_and(|g| {
        frame > g.last_frame + 1
            || !g
                .lease
                .matches(s, anim, animation, selection, &points, &mapping)
    }) {
        return; // Never recapture a different target during the same pointer gesture.
    }
    if gesture.is_none()
        && bg.is_pointer_button_down_on()
        && ui.input(|i| i.pointer.primary_pressed())
        && let Some(start) = origin.filter(|p| valid_point(*p) && mapping.rect.contains(*p))
    {
        gesture = Some(Gesture {
            start,
            additive: mods.ctrl,
            dragging: false,
            last_frame: frame,
            lease: Arc::new(Lease {
                identity: s.diagnostics.document_identity(),
                revision: s.revision,
                anim,
                animation: animation.clone(),
                selection: selection.to_vec(),
                bones: s.session.selected_bones.clone(),
                slots: s.session.selected_slots.clone(),
                focus: s.session.selection.clone(),
                points,
                mapping: mapping.clone(),
            }),
        });
    }
    let Some(mut g) = gesture else { return };
    g.dragging |= bg.dragged_by(egui::PointerButton::Primary);
    if !g.dragging {
        if down {
            g.last_frame = frame;
            ui.ctx().data_mut(|d| d.insert_temp(id, g));
        }
        return;
    }
    let Some(band) = band(g.start, pointer.unwrap(), mapping.rect) else {
        return;
    };
    let color = ui.visuals().selection.bg_fill;
    painter.rect_filled(band, 0., color.gamma_multiply(0.15));
    painter.rect_stroke(
        band,
        0.,
        egui::Stroke::new(1., color),
        egui::StrokeKind::Inside,
    );
    if released {
        let initial = if g.additive {
            g.lease.selection.as_slice()
        } else {
            &[]
        };
        let refs = pick(&g.lease.points, band, initial);
        // Validate the entire native result before changing selection. A mixed
        // FFD/scalar union or any corrupt/ambiguous reference preserves selection.
        if !refs.is_empty() && KeyClip::capture(&s.doc, anim, &refs).is_err() {
            return;
        }
        crate::key_selection::set(s, anim, refs);
    } else if down {
        g.last_frame = frame;
        ui.ctx().data_mut(|d| d.insert_temp(id, g));
    }
}
