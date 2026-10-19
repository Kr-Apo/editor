//! Video-only settings/preflight; delegates diagnostics and job ownership to existing paths.
use crate::{
    app_state::AppState,
    video_export::{self, Encoder, Format, QualityMode, Settings},
};
use ankhimate_export::preflight::{self, Report, Severity, Target};
use eframe::egui;
use std::path::PathBuf;
#[derive(Debug, Default)]
pub struct State {
    pub settings: Settings,
    /// Simple entry; the existing Video tab retains all T-602 controls.
    pub basic_mp4: bool,
    review: Option<Review>,
}

/// Session-only configuration. Never alters playback, document or preferences.
pub fn open_mp4(state: &mut AppState) {
    let clip = state
        .session
        .active_animation
        .filter(|id| state.doc.animations.contains_key(*id))
        .or_else(|| state.doc.animations.keys().next());
    let animation = clip
        .and_then(|id| state.doc.animations.get(id))
        .map(|a| a.name.clone())
        .unwrap_or_default();
    state.session.export.video_settings = State {
        basic_mp4: true,
        settings: Settings {
            clip,
            request: ankhimate_render::sequence::SequenceRequest {
                animation,
                width: 1280,
                height: 720,
                fps: 30,
            },
            background: state.session.export.video_settings.settings.background,
            executable: state.session.export.ffmpeg_path.clone(),
            ..Default::default()
        },
        review: None,
    };
    state.session.export.video = true;
    state.session.focus_tab = Some(crate::ui::Tab::Export);
}
#[derive(Debug)]
struct Review {
    settings: Settings,
    destination: PathBuf,
    revision: u64,
    project: ankhimate_formats::schema::Project,
    report: Report,
    consent: bool,
    error: Option<String>,
}
impl Review {
    fn new(state: &AppState, settings: Settings, destination: PathBuf) -> Self {
        let project = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        let focused = settings.snapshot(&state.doc);
        let export_doc = focused.as_ref().unwrap_or(&state.doc);
        let export_project = ankhimate_formats::convert::to_schema(&export_doc.as_project_ref());
        let request = settings
            .request_for(&state.doc)
            .unwrap_or_else(|_| settings.request.clone());
        let mut report = preflight::inspect(
            &export_project,
            &export_doc.assets,
            Target::Rendered,
            Some(&request.animation),
        );
        for result in [
            settings.count(&state.doc).map(|_| ()),
            ankhimate_export::new_file::inspect(&destination).map(|_| ()),
            video_export::locate(&settings.executable).map(|_| ()),
        ] {
            if let Err(e) = result {
                report.push(
                    Severity::Error,
                    "video-preflight",
                    settings.format.label(),
                    e,
                );
            }
        }
        if destination
            .extension()
            .and_then(|e| e.to_str())
            .is_none_or(|e| !e.eq_ignore_ascii_case(settings.format.extension()))
        {
            report.push(
                Severity::Error,
                "extension",
                settings.format.label(),
                format!("Choose a .{} file", settings.format.extension()),
            );
        }
        let codec=match settings.format {
            Format::Mp4=>settings.mp4_description(),
            Format::Gif=>"256-colour palettegen + paletteuse / opaque / no container repeat / centisecond timing".into()
        };
        report.push(Severity::Info,"video-encoding",settings.format.label(),format!("Clip {:?}, {} FPS, {} × {}, {} content cycle(s), {} samples, opaque RGB {:?}. {codec}. Executable: {}. No audio, resize or crop.",request.animation,request.fps,request.width,request.height,settings.cycles,settings.count(&state.doc).unwrap_or(0),settings.background,if settings.executable.is_empty(){"PATH"}else{&settings.executable}));
        if settings.format == Format::Mp4 && settings.encoder == Encoder::Libx264Rgb {
            report.push(Severity::Warning,"rgb-h264-compatibility","MP4","libx264rgb is software H.264 RGB / High 4:4:4 Predictive. Many browsers, TVs and hardware decoders cannot play it. Qualified with FFmpeg software decoding only; use libx264 for broader compatibility.");
        }
        report.push(Severity::Info,"open-after-export","Published file",if settings.open_after_export{"One native Windows association-open request after complete success only. Opening failure is reported separately; the published file stays valid."}else{"Open after export is off; no reader launch."});
        if settings.format == Format::Gif {
            report.push(Severity::Info,"gif-quantization","GIF","Two complete rendering passes, one private palette PNG. GIF delays are rounded to centiseconds; encoded content plays once, no automatic container repeat. Palette quantization/dithering is lossy; no transparency.");
            if request.fps > 50 {
                report.push(Severity::Warning,"gif-fast-timing","GIF","Some players clamp delays below 2 centiseconds. Playback may be slower at this FPS; use 50 FPS or lower for broader timing compatibility.");
            }
        }
        report.push(Severity::Info,"video-publication",settings.format.label(),format!("Create only: {}. No replacements or orphan deletion. Required FFmpeg capabilities are checked by the worker. Prepared output is linked from owned sibling staging; hard-link support is required. Crash durability and concurrent filesystem changes are not guaranteed.",destination.display()));
        Self {
            settings,
            destination,
            revision: state.revision,
            project,
            report,
            consent: false,
            error: None,
        }
    }
    fn launch(&self, state: &mut AppState, ctx: &egui::Context) -> Result<(), String> {
        if !self.report.allowed(self.consent) {
            return Err("Resolve errors and accept fidelity warnings first".into());
        }
        if self.revision != state.revision
            || self.project != ankhimate_formats::convert::to_schema(&state.doc.as_project_ref())
        {
            return Err("Project changed since review; prepare a new preflight".into());
        }
        if state.session.export.png.is_running() {
            return Err("Another rendered export is running".into());
        }
        ankhimate_export::new_file::inspect(&self.destination)?;
        state.session.export.last_preflight=Some(serde_json::to_string_pretty(&serde_json::json!({"destination":self.destination,"diagnostics":self.report,"fidelity_warnings_accepted":self.consent,"result":"Video worker requested; consult completion status"})).unwrap());
        state.session.export.png.start_video(
            state.doc.clone(),
            self.settings.clone(),
            self.destination.clone(),
            ctx.clone(),
        );
        Ok(())
    }
}
pub fn ui(ui: &mut egui::Ui, state: &mut AppState, theme: &crate::theme::Theme) {
    state.session.export.png.video_status(ui);
    let running = state.session.export.png.is_running();
    let v = &mut state.session.export.video_settings;
    v.settings
        .executable
        .clone_from(&state.session.export.ffmpeg_path);
    if v.settings.clip.is_none() {
        v.settings.clip = state
            .doc
            .animations
            .iter()
            .find(|(_, a)| a.name == v.settings.request.animation)
            .map(|(id, _)| id)
            .or_else(|| state.doc.animations.keys().next());
    }
    if let Some(clip) = v.settings.clip.and_then(|id| state.doc.animations.get(id)) {
        v.settings.request.animation.clone_from(&clip.name);
    }
    ui.heading(if v.basic_mp4 { "Video MP4" } else { "Video" });
    ui.label("External FFmpeg required. Configure Settings → Video, or leave its path empty to use PATH.");
    match video_export::locate(&v.settings.executable) {
        Ok(path) => {
            ui.label(format!(
                "FFmpeg: found {} — encoder checked on export",
                path.display()
            ));
        }
        Err(_) => {
            ui.colored_label(
                egui::Color32::YELLOW,
                "FFmpeg: not found — choose ffmpeg.exe in Settings → Video",
            );
        }
    }
    ui.hyperlink_to("Official FFmpeg downloads", video_export::DOWNLOAD);
    ui.add_enabled_ui(!running, |ui| {
        if !v.basic_mp4 {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut v.settings.format, Format::Mp4, "MP4");
                ui.selectable_value(&mut v.settings.format, Format::Gif, "Animated GIF");
            });
        }
        egui::ComboBox::from_id_salt("video_clip")
            .selected_text(
                v.settings
                    .clip
                    .and_then(|id| {
                        state
                            .doc
                            .animations
                            .iter()
                            .enumerate()
                            .find(|(_, (key, _))| *key == id)
                            .map(|(i, (_, a))| format!("{} · #{}", a.name, i + 1))
                    })
                    .unwrap_or_else(|| "Select an animation".into()),
            )
            .show_ui(ui, |ui| {
                for (i, (id, a)) in state.doc.animations.iter().enumerate() {
                    ui.selectable_value(
                        &mut v.settings.clip,
                        Some(id),
                        format!("{} · #{}", a.name, i + 1),
                    );
                }
            });
        if let Some(clip) = v.settings.clip.and_then(|id| state.doc.animations.get(id)) {
            v.settings.request.animation.clone_from(&clip.name);
        }
        egui::Grid::new("video_settings").show(ui, |ui| {
            if v.basic_mp4 {
                ui.label("Resolution");
                let mut size = (v.settings.request.width, v.settings.request.height);
                egui::ComboBox::from_id_salt("mp4_resolution")
                    .selected_text(if size == (1920, 1080) {
                        "1920 × 1080 (1080p)"
                    } else {
                        "1280 × 720 (720p)"
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut size, (1280, 720), "1280 × 720 (720p)");
                        ui.selectable_value(&mut size, (1920, 1080), "1920 × 1080 (1080p)");
                    });
                (v.settings.request.width, v.settings.request.height) = size;
                ui.end_row();
                ui.label("FPS");
                egui::ComboBox::from_id_salt("mp4_fps")
                    .selected_text(format!("{} FPS", v.settings.request.fps))
                    .show_ui(ui, |ui| {
                        for fps in [24, 30, 60] {
                            ui.selectable_value(
                                &mut v.settings.request.fps,
                                fps,
                                format!("{fps} FPS"),
                            );
                        }
                    });
                ui.end_row();
            } else {
                ui.label("FPS");
                ui.add(egui::DragValue::new(&mut v.settings.request.fps).range(
                    1..=if v.settings.format == Format::Gif {
                        100
                    } else {
                        240
                    },
                ));
                ui.end_row();
                ui.label(if v.settings.format == Format::Mp4 {
                    "Width (even)"
                } else {
                    "Width"
                });
                ui.add(egui::DragValue::new(&mut v.settings.request.width).range(2..=8192));
                ui.end_row();
                ui.label(if v.settings.format == Format::Mp4 {
                    "Height (even)"
                } else {
                    "Height"
                });
                ui.add(egui::DragValue::new(&mut v.settings.request.height).range(2..=8192));
                ui.end_row();
            }
            ui.label("Opaque background");
            ui.color_edit_button_srgb(&mut v.settings.background);
            ui.end_row();
            if v.settings.format == Format::Mp4 && !v.basic_mp4 {
                ui.label("Encoder");
                egui::ComboBox::from_id_salt("mp4_encoder")
                    .selected_text(v.settings.encoder.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut v.settings.encoder,
                            Encoder::Libx264,
                            "libx264 — compatible H.264",
                        );
                        ui.selectable_value(
                            &mut v.settings.encoder,
                            Encoder::Libx264Rgb,
                            "libx264rgb — software RGB, limited players",
                        );
                    });
                ui.end_row();
                ui.label("Quality mode");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut v.settings.quality, QualityMode::Crf, "CRF");
                    ui.selectable_value(
                        &mut v.settings.quality,
                        QualityMode::TargetBitrate,
                        "Target bitrate",
                    );
                });
                ui.end_row();
                match v.settings.quality {
                    QualityMode::Crf => {
                        ui.label("CRF (0 best, 51 lowest quality)");
                        ui.add(egui::DragValue::new(&mut v.settings.crf).range(0..=51));
                    }
                    QualityMode::TargetBitrate => {
                        ui.label("Target bitrate (kbit/s)");
                        ui.add(egui::DragValue::new(&mut v.settings.bitrate_kbps).range(
                            video_export::MIN_BITRATE_KBPS..=video_export::MAX_BITRATE_KBPS,
                        ));
                    }
                }
                ui.end_row();
            }
            ui.label("Content cycles");
            if v.basic_mp4 {
                egui::ComboBox::from_id_salt("mp4_cycles")
                    .selected_text(format!(
                        "{} time{}",
                        v.settings.cycles,
                        if v.settings.cycles == 1 { "" } else { "s" }
                    ))
                    .show_ui(ui, |ui| {
                        for cycles in 1..=3 {
                            ui.selectable_value(
                                &mut v.settings.cycles,
                                cycles,
                                format!("{cycles} time{}", if cycles == 1 { "" } else { "s" }),
                            );
                        }
                    });
            } else {
                ui.add(egui::DragValue::new(&mut v.settings.cycles).range(1..=1_000_000));
            }
            ui.end_row();
        });
        if crate::system_open::available() {
            ui.checkbox(&mut v.settings.open_after_export, "Open after export");
        } else {
            ui.label("Opening exported files is unavailable on this platform");
        }
    });
    if v.settings.format == Format::Gif {
        ui.label("GIF: 256 colours • opaque • palette quantization and dithering change pixels");
        ui.label("GIF repetition: play encoded content once (no container loop)");
        ui.label("Delays rounded to centiseconds; some players slow down GIFs above 50 FPS.");
    } else {
        ui.label(v.settings.mp4_description());
        if v.settings.encoder == Encoder::Libx264Rgb {
            ui.label("RGB H.264: software decoding qualified only; many browsers and hardware players are incompatible.");
        }
    }
    ui.label("Default skin • Normal blending • physics refused • no audio • common camera");
    ui.label("New files only: existing videos are never replaced. Choose an unused filename.");
    let validation = v.settings.count(&state.doc);
    match &validation {
        Ok(n) => {
            ui.label(format!(
                "{n} frames • {:.3} seconds",
                *n as f64 / f64::from(v.settings.request.fps)
            ));
        }
        Err(e) => {
            ui.colored_label(egui::Color32::LIGHT_RED, e);
        }
    }
    if ui
        .add_enabled(
            !running && validation.is_ok(),
            egui::Button::new(format!("Export {}…", v.settings.format.label())),
        )
        .clicked()
        && let Some(destination) = rfd::FileDialog::new()
            .set_title(format!("{} — choose a new file", v.settings.format.label()))
            .add_filter(v.settings.format.label(), &[v.settings.format.extension()])
            .set_file_name(format!("animation.{}", v.settings.format.extension()))
            .save_file()
    {
        let settings = v.settings.clone();
        state.session.export.video_settings.review =
            Some(Review::new(state, settings, destination));
    }
    let Some(mut review) = state.session.export.video_settings.review.take() else {
        return;
    };
    let mut launch = false;
    let mut cancel = false;
    let dialog = crate::ui::dialog::Dialog::new("video-preflight", "Video export preflight")
        .width(650.0)
        .max_height(550.0)
        .show(ui.ctx(), theme, |ui| {
            ui.label(review.destination.display().to_string());
            egui::ScrollArea::vertical()
                .max_height(330.0)
                .show(ui, |ui| {
                    for item in &review.report.items {
                        ui.label(format!(
                            "{:?} [{}]: {}",
                            item.severity, item.code, item.message
                        ));
                    }
                });
            if review.report.has_warnings() {
                ui.checkbox(
                    &mut review.consent,
                    "I accept the listed fidelity / qualification warnings",
                );
            }
            if let Some(e) = &review.error {
                ui.colored_label(egui::Color32::LIGHT_RED, e);
            }
            ui.horizontal(|ui| {
                launch = ui
                    .add_enabled(
                        review.report.allowed(review.consent),
                        egui::Button::new("Publish export"),
                    )
                    .clicked();
                cancel = ui.button("Cancel").clicked();
            });
        });
    if cancel || dialog.closed {
        return;
    }
    if launch {
        match review.launch(state, ui.ctx()) {
            Ok(()) => return,
            Err(e) => review.error = Some(e),
        }
    }
    state.session.export.video_settings.review = Some(review);
}
#[cfg(test)]
mod tests {
    use super::*;
    fn state() -> AppState {
        AppState {
            doc: ankhimate_document::Document::from_loaded(
                ankhimate_formats::load(
                    &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("../samples/walker.ankh"),
                )
                .unwrap()
                .0,
            ),
            ..Default::default()
        }
    }

    fn settings() -> Settings {
        Settings {
            request: ankhimate_render::sequence::SequenceRequest {
                animation: "walk".into(),
                fps: 30,
                width: 64,
                height: 48,
            },
            ..Default::default()
        }
    }
    #[test]
    fn cancelled_preflight_has_no_writes_and_preserves_document() {
        let state = state();
        let before = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        let d = tempfile::tempdir().unwrap();
        let review = Review::new(&state, settings(), d.path().join("film.mp4"));
        drop(review);
        assert_eq!(fs_count(d.path()), 0);
        assert_eq!(
            before,
            ankhimate_formats::convert::to_schema(&state.doc.as_project_ref())
        );
        assert!(!state.history.can_undo());
    }
    fn fs_count(p: &std::path::Path) -> usize {
        std::fs::read_dir(p).unwrap().count()
    }
    #[test]
    fn changed_document_and_racing_destinations_block_before_worker() {
        let mut state = state();
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("film.mp4");
        let mut review = Review::new(&state, settings(), out.clone());
        // No installed dependency is necessary for this integrity/dispatch seam.
        review.report.items.retain(|d| d.code != "video-preflight");
        review.consent = true;
        state.revision += 1;
        assert!(
            review
                .launch(&mut state, &egui::Context::default())
                .unwrap_err()
                .contains("changed")
        );
        state.revision -= 1;
        std::fs::write(&out, b"old").unwrap();
        assert!(
            review
                .launch(&mut state, &egui::Context::default())
                .unwrap_err()
                .contains("exists")
        );
        assert!(!state.session.export.png.is_running());
        assert_eq!(std::fs::read(out).unwrap(), b"old");
    }
    #[test]
    fn settings_serialize_and_legacy_preferences_default_to_path() {
        let config: crate::config::Config = serde_json::from_str("{}").unwrap();
        assert!(config.ffmpeg_path.is_empty());
        let config = crate::config::Config {
            ffmpeg_path: "C:/été & tools/ffmpeg.exe".into(),
            ..Default::default()
        };
        let saved = serde_json::to_string(&config).unwrap();
        assert_eq!(
            serde_json::from_str::<crate::config::Config>(&saved)
                .unwrap()
                .ffmpeg_path,
            config.ffmpeg_path
        );
    }
    #[test]
    fn mp4_controls_have_background_crf_cycles_and_dependency_notice() {
        let mut state = state();
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
        for text in [
            "Opaque background",
            "CRF (0 best, 51 lowest quality)",
            "Content cycles",
            "Export MP4…",
            "Official FFmpeg downloads",
        ] {
            assert!(
                output.shapes.iter().any(
                    |s| matches!(&s.shape,egui::epaint::Shape::Text(t) if t.galley.text()==text)
                ),
                "Missing {text}"
            );
        }
    }
}
#[cfg(test)]
mod gif_entry_regression {
    #[test]
    fn video_panel_offers_animated_gif() {
        let ctx = eframe::egui::Context::default();
        let mut state = crate::app_state::AppState::default();
        let output = ctx.run_ui(
            eframe::egui::RawInput {
                screen_rect: Some(eframe::egui::Rect::from_min_size(
                    eframe::egui::Pos2::ZERO,
                    eframe::egui::vec2(1000.0, 1200.0),
                )),
                ..Default::default()
            },
            |root| {
                eframe::egui::CentralPanel::default().show_inside(root, |ui| {
                    super::ui(ui, &mut state, &crate::theme::Theme::default())
                });
            },
        );
        assert!(output.shapes.iter().any(|s|matches!(&s.shape,eframe::egui::epaint::Shape::Text(t) if t.galley.text()=="Animated GIF")),"Animated GIF is missing from the Video panel");
    }
}
#[cfg(test)]
mod gif_controls_tests {
    use super::*;
    #[test]
    fn gif_mode_shows_relevant_controls_and_hides_mp4_options() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        state.session.export.video_settings.settings.format = Format::Gif;
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
        for name in [
            "Animated GIF",
            "Opaque background",
            "Content cycles",
            "GIF repetition: play encoded content once (no container loop)",
            "Export Animated GIF…",
        ] {
            assert!(labels.iter().any(|l| l == name), "Missing {name}");
        }
        assert!(!labels.iter().any(|l| l.contains("CRF")
            || l.contains("libx264")
            || l.contains("yuv420p")
            || l.contains("bitrate")));
        assert!(labels.iter().any(|l| l.contains("256 colours")));
    }
}
#[cfg(test)]
mod final_controls_red {
    use super::*;
    fn labels() -> Vec<String> {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 1400.0),
                )),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show_inside(root, |ui| {
                    super::ui(ui, &mut state, &crate::theme::Theme::default())
                });
            },
        );
        output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::epaint::Shape::Text(t) => Some(t.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }
    #[test]
    fn video_offers_target_bitrate() {
        assert!(
            labels().iter().any(|s| s == "Target bitrate"),
            "Target bitrate control is missing"
        );
    }
    #[test]
    fn video_offers_opt_in_open_after_export() {
        assert_eq!(
            labels().iter().any(|s| s == "Open after export"),
            crate::system_open::available(),
            "Open-after-export availability must match native support"
        );
    }
}
#[cfg(test)]
mod final_controls_interactions {
    use super::*;
    fn frame(
        ctx: &egui::Context,
        state: &mut AppState,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 1400.0),
                )),
                events,
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show_inside(root, |ui| {
                    super::ui(ui, state, &crate::theme::Theme::default())
                });
            },
        )
    }
    fn click(ctx: &egui::Context, state: &mut AppState, text: &str) {
        let output = frame(ctx, state, vec![]);
        let pos = output
            .shapes
            .iter()
            .find_map(|s| match &s.shape {
                egui::epaint::Shape::Text(t) if t.galley.text() == text => {
                    Some(t.pos + t.galley.rect.center().to_vec2())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("missing control {text}"));
        for pressed in [true, false] {
            frame(
                ctx,
                state,
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
        }
    }
    #[test]
    fn quality_and_open_controls_toggle_without_author_edits_and_lock_during_export() {
        let mut state = AppState {
            doc: crate::video_export::tests::walker(),
            ..Default::default()
        };
        let original = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        let assets = serde_json::to_vec(&state.doc.assets).unwrap();
        let ctx = egui::Context::default();
        assert!(
            !state
                .session
                .export
                .video_settings
                .settings
                .open_after_export
        );
        click(&ctx, &mut state, "Target bitrate");
        assert_eq!(
            state.session.export.video_settings.settings.quality,
            QualityMode::TargetBitrate
        );
        if crate::system_open::available() {
            click(&ctx, &mut state, "Open after export");
            assert!(
                state
                    .session
                    .export
                    .video_settings
                    .settings
                    .open_after_export
            );
        }
        let s = Settings {
            request: ankhimate_render::sequence::SequenceRequest {
                animation: "walk".into(),
                fps: 2,
                width: 8,
                height: 8,
            },
            quality: QualityMode::TargetBitrate,
            executable: crate::video_export::tests::helper("block")
                .to_string_lossy()
                .into_owned(),
            ..Default::default()
        };
        let d = tempfile::tempdir().unwrap();
        state.session.export.png.start_video_with_opener(
            state.doc.clone(),
            s,
            d.path().join("movie.mp4"),
            ctx.clone(),
            |_| panic!("cancelled export must not open"),
        );
        click(&ctx, &mut state, "CRF");
        assert_eq!(
            state.session.export.video_settings.settings.quality,
            QualityMode::TargetBitrate
        );
        if crate::system_open::available() {
            click(&ctx, &mut state, "Open after export");
            assert!(
                state
                    .session
                    .export
                    .video_settings
                    .settings
                    .open_after_export
            );
        }
        click(&ctx, &mut state, "Animated GIF");
        assert_eq!(
            state.session.export.video_settings.settings.format,
            Format::Mp4
        );
        state.session.export.png.cancel();
        let end = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while state.session.export.png.is_running() {
            state.session.export.png.poll();
            assert!(std::time::Instant::now() < end);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        assert_eq!(
            original,
            ankhimate_formats::convert::to_schema(&state.doc.as_project_ref())
        );
        assert_eq!(assets, serde_json::to_vec(&state.doc.assets).unwrap());
        assert_eq!(state.history.undo_depth(), 0);
    }
    #[test]
    fn preflight_reports_selected_rate_and_requires_rgb_compatibility_consent() {
        let state = AppState {
            doc: crate::video_export::tests::walker(),
            ..Default::default()
        };
        let d = tempfile::tempdir().unwrap();
        let s = Settings {
            quality: QualityMode::TargetBitrate,
            bitrate_kbps: 321,
            encoder: Encoder::Libx264Rgb,
            request: ankhimate_render::sequence::SequenceRequest {
                animation: "walk".into(),
                fps: 30,
                width: 64,
                height: 48,
            },
            executable: std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            ..Default::default()
        };
        let review = Review::new(&state, s, d.path().join("rgb.mp4"));
        assert!(
            review
                .report
                .items
                .iter()
                .any(|i| i.message.contains("target 321 kbit/s"))
        );
        assert!(
            review
                .report
                .items
                .iter()
                .any(|i| i.code == "rgb-h264-compatibility" && i.severity == Severity::Warning)
        );
        assert!(!review.report.allowed(false));
        assert!(review.report.allowed(true));
    }
}

#[cfg(test)]
#[path = "video_t8105_tests.rs"]
mod t8105_tests;
