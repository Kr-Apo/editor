//! One transient gesture; the native command is the only author write.
use super::*;
use ankhimate_document::commands::document_cmds::SetProjectFps;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Stamp {
    document: u64,
    revision: u64,
    fps: u32,
}
impl Stamp {
    fn new(state: &AppState) -> Self {
        Self {
            document: state.diagnostics.document_identity(),
            revision: state.revision,
            fps: state.doc.meta.fps,
        }
    }
}
#[derive(Clone)]
struct Draft {
    stamp: Stamp,
    value: u32,
    active: bool,
    pointer_gesture: bool,
    blocked: bool,
    epoch: u64,
    widget: Option<egui::Id>,
    frame: u64,
}
impl Draft {
    fn fresh(stamp: Stamp, frame: u64, epoch: u64, blocked: bool) -> Self {
        Self {
            stamp,
            value: stamp.fps,
            active: false,
            pointer_gesture: false,
            blocked,
            epoch,
            widget: None,
            frame,
        }
    }
    fn release_capture(&self, ui: &egui::Ui) {
        if let Some(id) = self.widget {
            ui.memory_mut(|m| m.surrender_focus(id));
            if ui.ctx().is_being_dragged(id) {
                ui.ctx().stop_dragging();
            }
        }
    }
}
fn parse(text: &str) -> Option<f64> {
    // Retain egui's whitespace and Unicode-minus conventions, then validate.
    let text: String = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| if c == '−' { '-' } else { c })
        .collect();
    let value = text.parse::<f64>().ok()?;
    (value.is_finite()
        && value.fract() == 0.0
        && (f64::from(SetProjectFps::MIN)..=f64::from(SetProjectFps::MAX)).contains(&value))
    .then_some(value)
}
pub(super) fn ui(ui: &mut egui::Ui, state: &mut AppState) {
    let id = ui.id().with("project_fps_draft");
    let frame = ui.ctx().cumulative_frame_nr();
    let stamp = Stamp::new(state);
    let mut draft = ui
        .ctx()
        .data(|d| d.get_temp::<Draft>(id))
        .unwrap_or_else(|| Draft::fresh(stamp, frame, 0, false));
    let (down, escape, focused, gone) = ui.input(|i| {
        (
            i.pointer.primary_down(),
            i.key_pressed(egui::Key::Escape),
            i.focused,
            i.raw
                .events
                .iter()
                .any(|e| matches!(e, egui::Event::PointerGone)),
        )
    });
    let stale = draft.stamp != stamp || (draft.active && frame > draft.frame.saturating_add(1));
    let cancel = draft.active && (escape || !focused || (gone && draft.pointer_gesture));
    if stale || cancel {
        draft.release_capture(ui);
        draft = Draft::fresh(
            stamp,
            frame,
            draft.epoch.wrapping_add(1),
            draft.blocked || (draft.active && down),
        );
        ui.ctx().request_repaint();
    }
    if !down {
        draft.blocked = false;
    }
    let invalid = std::cell::Cell::new(false);
    let response = ui.push_id((stamp.document, stamp.revision, stamp.fps, draft.epoch), |ui| {
        ui.add_enabled(!draft.blocked, egui::DragValue::new(&mut draft.value)
            .range(SetProjectFps::MIN..=SetProjectFps::MAX).speed(1)
            .clamp_existing_to_range(false).update_while_editing(false)
            .custom_parser(|text| { let parsed = parse(text); if parsed.is_none() { invalid.set(true); } parsed }))
            .on_hover_text("Project FPS, 1–240 whole frames per second. Drag: release to apply. Type: Enter or leave the field to apply. Escape cancels. Key times are unchanged.")
    }).inner;
    draft.widget = Some(response.id);
    let one_shot = response.changed() && !response.has_focus() && !response.dragged() && !down;
    let finished = response.drag_stopped() || response.lost_focus() || one_shot;
    draft.active |=
        response.has_focus() || response.is_pointer_button_down_on() || response.dragged();
    draft.pointer_gesture = response.is_pointer_button_down_on() || response.dragged();
    if finished && !draft.blocked && !cancel {
        if invalid.get() {
            state
                .session
                .set_status("Project FPS must be a whole number from 1 to 240; change cancelled");
        } else {
            match SetProjectFps::new(&state.doc, draft.value) {
                Ok(Some(command)) => {
                    state.dispatch(Box::new(command));
                }
                Ok(None) => {}
                Err(error) => state.session.set_status(error),
            }
        }
        draft.release_capture(ui);
        draft = Draft::fresh(Stamp::new(state), frame, draft.epoch.wrapping_add(1), false);
        ui.ctx().request_repaint();
    }
    draft.frame = frame;
    ui.ctx().data_mut(|d| d.insert_temp(id, draft));
}
