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
    crate::tangent_linking::mode_ui(ui, state);
    if crate::tangent_linking::mode(state) != crate::tangent_linking::Mode::Broken {
        linked_ui(ui, state);
        return;
    }
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

fn linked_ui(ui: &mut egui::Ui, state: &mut AppState) {
    use crate::tangent_linking::{Numeric, Side};
    let id = ui.id().with("linked_bezier_draft");
    let stored = ui.ctx().data(|d| d.get_temp::<Numeric>(id));
    let mut draft = match stored.filter(|d| d.valid(state)) {
        Some(d) => d,
        None => match Numeric::new(state, Side::Incoming) {
            Ok(d) => d,
            Err(e) => {
                ui.label(e);
                return;
            }
        },
    };
    let editable = crate::tangent_linking::editable(state, &draft.junction.addr).is_ok();
    ui.group(|ui|{
        ui.label(egui::RichText::new("Bezier tangents — linked junction").strong());
        ui.label(format!("Central key {} at {}s. Incoming belongs to this key; outgoing belongs to the next segment.",draft.junction.index,draft.junction.keys[1].time));
        ui.label("Metric: seconds / native value units. Session mode; not saved per key.");
        let old=draft.side;
        ui.horizontal(|ui|{
            ui.label("Driver:");
            ui.selectable_value(&mut draft.side,Side::Incoming,"Incoming");
            ui.selectable_value(&mut draft.side,Side::Outgoing,"Outgoing");
        });
        if old!=draft.side && let Ok(reset)=Numeric::new(state,draft.side){draft=reset;}
        let pair=draft.pair();
        if let Ok([ankhimate_core::animation::Interp::Bezier{in_handle,..},ankhimate_core::animation::Interp::Bezier{out_handle,..}])=pair{
            let offset=if draft.side==Side::Incoming{2}else{0};
            let v=if offset==2{out_handle}else{in_handle};
            draft.fields[offset]=v.x.to_string();draft.fields[offset+1]=v.y.to_string();
        }
        egui::Grid::new("linked_bezier_numbers").show(ui,|ui|{
            for (i,label) in ["Incoming X","Incoming Y","Outgoing X","Outgoing Y"].into_iter().enumerate(){
                ui.label(label);
                let driver=(i<2)==(draft.side==Side::Incoming);
                ui.add_enabled(driver && editable,egui::TextEdit::singleline(&mut draft.fields[i]).desired_width(100.0));
                ui.end_row();
            }
        });
        if let Err(e)=draft.pair(){ui.colored_label(ui.visuals().error_fg_color,e);}
        ui.horizontal(|ui|{
            if ui.add_enabled(editable && draft.pair().is_ok(),egui::Button::new("Apply tangents")).clicked() && let Err(e)=draft.apply(state){state.session.set_status(e);}
            if ui.button("Cancel tangent edits").clicked() && let Ok(reset)=Numeric::new(state,draft.side){draft=reset;}
        });
    });
    ui.ctx().data_mut(|d| d.insert_temp(id, draft));
}

#[cfg(test)]
mod t820_tests {
    use crate::{
        app_state::AppState,
        tangent_linking::{self, Mode, tests::fixture},
    };
    use eframe::egui;
    fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000., 900.),
                )),
                events,
                ..Default::default()
            },
            |ui| super::ui(ui, s),
        )
    }
    fn find(out: &egui::FullOutput, text: &str) -> egui::Pos2 {
        out.shapes
            .iter()
            .find_map(|s| {
                if let egui::Shape::Text(t) = &s.shape {
                    (t.galley.text() == text).then_some(t.pos + t.galley.size() * 0.5)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Missing {text}"))
    }
    fn click(ctx: &egui::Context, s: &mut AppState, pos: egui::Pos2) {
        frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
        for pressed in [true, false] {
            frame(
                ctx,
                s,
                vec![egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                }],
            );
        }
    }
    #[test]
    fn t820_linked_numeric_real_typing_apply_and_cancel() {
        for apply in [false, true] {
            let (mut s, _, _) = fixture();
            tangent_linking::choose(&mut s, Mode::Mirrored).unwrap();
            let before = ankhimate_document::snapshot::snapshot(&s.doc);
            let ctx = egui::Context::default();
            frame(&ctx, &mut s, vec![]);
            let out = frame(&ctx, &mut s, vec![]);
            let p = find(&out, "0.75");
            click(&ctx, &mut s, p);
            frame(
                &ctx,
                &mut s,
                vec![
                    egui::Event::Key {
                        key: egui::Key::A,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers {
                            ctrl: true,
                            command: true,
                            ..Default::default()
                        },
                    },
                    egui::Event::Text("0.8".into()),
                ],
            );
            assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
            assert!(!s.history.can_undo());
            let out = frame(&ctx, &mut s, vec![]);
            assert!(
                out.shapes
                    .iter()
                    .any(|sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text()=="0.8"))
            );
            click(
                &ctx,
                &mut s,
                find(
                    &out,
                    if apply {
                        "Apply tangents"
                    } else {
                        "Cancel tangent edits"
                    },
                ),
            );
            assert_eq!(s.history.undo_depth(), usize::from(apply));
            if apply {
                assert!(ankhimate_document::snapshot::snapshot(&s.doc) != before);
                s.undo();
                assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
            } else {
                assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
            }
        }
    }
    #[test]
    fn t820_link_modes_are_visible_before_authoring() {
        let mut state = crate::app_state::AppState::default();
        let ctx = eframe::egui::Context::default();
        let out = ctx.run_ui(Default::default(), |ui| super::ui(ui, &mut state));
        for label in ["Broken", "Aligned", "Mirrored"] {
            assert!(out.shapes.iter().any(|s| matches!(&s.shape, eframe::egui::epaint::Shape::Text(t) if t.galley.text().contains(label))), "Missing tangent mode: {label}");
        }
        assert!(!state.history.can_undo());
    }
}
