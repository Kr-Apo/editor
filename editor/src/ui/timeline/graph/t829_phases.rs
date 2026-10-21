// Observer of actual private handle preparation; only built in Rust tests.
use super::*;
pub(crate) fn handles(state: &AppState, selection: &[KeyRef]) -> (f64, usize) {
    let anim = state.session.active_animation.unwrap();
    let model = TimelineModel::build(state, anim);
    let ctx = egui::Context::default();
    let mut elapsed = 0.;
    let out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            ..Default::default()
        },
        |ui| {
            let painter = ui.painter().clone();
            let rect = egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(600., 500.));
            let layout = Layout {
                sheet_x0: 30.,
                px_per_sec: 260.,
                scroll_sec: 0.,
                fps: 30,
            };
            let mut selection = selection.to_vec();
            let start = std::time::Instant::now();
            for group in &model.groups {
                for row in &group.rows {
                    for ch in &row.channels {
                        let _ = bezier_handles(
                            ui,
                            &painter,
                            &row.addr,
                            ch.axis,
                            &row.keys,
                            &ch.values,
                            &layout,
                            rect,
                            &|v| (300. - v) as f32,
                            0.,
                            10000.,
                            egui::Color32::GREEN,
                            &mut selection,
                            crate::tangent_linking::Mode::Broken,
                        );
                    }
                }
            }
            elapsed = start.elapsed().as_secs_f64() * 1000.;
        },
    );
    (elapsed, out.shapes.len())
}
