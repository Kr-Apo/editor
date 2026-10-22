//! Small diagnostic surface; no document/session mutation or external requests.
use eframe::egui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SOURCE_COMMIT: &str = match option_env!("ANKHIMATE_SOURCE_COMMIT") {
    Some(commit) => commit,
    None => "not recorded (development build; consult the build manifest)",
};

pub fn menu(ui: &mut egui::Ui, open: &mut bool) {
    ui.menu_button("Help", |ui| {
        if ui.button("About Ankhimate").clicked() {
            *open = true;
            ui.close();
        }
    });
}

pub fn show(ctx: &egui::Context, open: &mut bool) {
    if !*open {
        return;
    }
    egui::Window::new("About Ankhimate")
        .open(open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Ankhimate");
            ui.label(format!("Version: {VERSION}"));
            ui.label("Build: local candidate / development (not published)");
            ui.label("Source commit:");
            ui.add(egui::Label::new(SOURCE_COMMIT).selectable(true).wrap());
            ui.separator();
            ui.label("Kr-Apo Community Fork (Unofficial)");
            ui.label("Derived from Ankhimate/editor; upstream credits retained.");
            ui.label("MIT OR Apache-2.0. See LICENSE-MIT, LICENSE-APACHE and Licenses/ for third-party credits and source notices.");
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(output: &egui::FullOutput, label: &str) -> egui::Pos2 {
        output
            .shapes
            .iter()
            .find_map(|s| match &s.shape {
                egui::Shape::Text(t) if t.galley.job.text == label => {
                    Some(t.pos + t.galley.size() * 0.5)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("Missing text: {label}"))
    }

    #[test]
    fn t838_help_menu_really_opens_about_with_pointer_events() {
        let ctx = egui::Context::default();
        let mut open = false;
        let mut frame = |events| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900.0, 700.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    egui::MenuBar::new().ui(ui, |ui| menu(ui, &mut open));
                },
            )
        };
        let click = |position, pressed| {
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                },
            ]
        };
        frame(vec![]);
        let help = text(&frame(vec![]), "Help");
        frame(click(help, true));
        frame(click(help, false));
        let about = text(&frame(vec![]), "About Ankhimate");
        frame(click(about, true));
        frame(click(about, false));
        assert!(open);
    }

    #[test]
    fn t838_about_keeps_version_provenance_and_licenses_visible() {
        assert_eq!(VERSION, "0.1.5");
        let ctx = egui::Context::default();
        let mut open = true;
        let _ = ctx.run_ui(Default::default(), |ui| show(ui.ctx(), &mut open));
        let output = ctx.run_ui(Default::default(), |ui| show(ui.ctx(), &mut open));
        text(&output, "Version: 0.1.5");
        text(&output, SOURCE_COMMIT);
        text(
            &output,
            "Build: local candidate / development (not published)",
        );
        assert!(output.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.job.text.contains("MIT OR Apache-2.0"))));
    }
}
