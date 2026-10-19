use super::*;

fn state() -> AppState {
    let mut state = AppState {
        doc: crate::video_export::tests::walker(),
        ..Default::default()
    };
    state.session.active_animation = state.doc.animations.keys().last();
    state
}

#[test]
fn t8105_mp4_entry_uses_selected_clip_and_safe_presets_without_editing_project() {
    let mut state = state();
    let before = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
    let active = state.session.active_animation;
    let registry = crate::registry::Registry::with_builtins();
    let op = registry
        .get("export.video_mp4")
        .expect("Missing File / Export / Video MP4 action");
    assert_eq!(op.label(), "Video MP4…");
    assert!(registry.invoke("export.video_mp4", &mut state).is_some());
    assert_eq!(state.session.focus_tab, Some(crate::ui::Tab::Export));
    assert!(state.session.export.video);
    let s = &state.session.export.video_settings.settings;
    assert_eq!(s.clip, active);
    assert_eq!(
        (s.request.width, s.request.height, s.request.fps, s.cycles),
        (1280, 720, 30, 1)
    );
    assert_eq!(s.encoder, Encoder::Libx264);
    assert_eq!(s.format, Format::Mp4);
    assert_eq!(
        before,
        ankhimate_formats::convert::to_schema(&state.doc.as_project_ref())
    );
    assert!(!state.history.can_undo());
}

#[test]
fn t8105_basic_panel_exposes_resolution_fps_cycles_and_dependency_status() {
    let mut state = state();
    assert!(
        crate::registry::Registry::with_builtins()
            .invoke("export.video_mp4", &mut state)
            .is_some(),
        "MP4 entry missing"
    );
    let ctx = egui::Context::default();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 1200.0),
            )),
            ..Default::default()
        },
        |root| {
            egui::CentralPanel::default().show_inside(root, |ui| {
                super::ui(ui, &mut state, &crate::theme::Theme::default())
            });
        },
    );
    let labels: Vec<_> = output
        .shapes
        .iter()
        .filter_map(|s| match &s.shape {
            egui::epaint::Shape::Text(t) => Some(t.galley.text().to_owned()),
            _ => None,
        })
        .collect();
    for label in [
        "Resolution",
        "1280 × 720 (720p)",
        "30 FPS",
        "1 time",
        "Opaque background",
        "Export MP4…",
    ] {
        assert!(
            labels.iter().any(|s| s == label),
            "Missing {label}: {labels:?}"
        );
    }
    assert!(labels.iter().any(|s| s.starts_with("FFmpeg:")));
    assert!(!labels.iter().any(|s| s == "Encoder" || s == "Animated GIF"));
}
