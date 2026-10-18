//! Session-only controls and worker for rendered export. No document edits.
use crate::app_state::AppState;
use ankhimate_render::{
    encoding::FrameEncoding,
    framing::{BoundsMode, FrameOptions},
    sequence::SequenceRequest,
};
use eframe::egui;
use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
};

#[derive(Debug, Default)]
pub struct PngUiState {
    pub selected: String,
    pub batch: bool,
    pub batch_selected: Vec<ankhimate_core::ids::AnimationId>,
    pub batch_folder: String,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub spritesheet: bool,
    pub encoding: FrameEncoding,
    pub framing: FrameOptions,
    pub max_page: u32,
    pub padding: u32,
    job: Option<Job>,
    pub message: Option<Result<String, String>>,
    pub open_result: Option<Result<(), String>>,
}
#[derive(Debug)]
struct Job {
    cancel: Arc<AtomicBool>,
    progress: Arc<Mutex<(&'static str, u64, u64)>>,
    receiver: mpsc::Receiver<Result<String, String>>,
    opening: Option<mpsc::Receiver<Result<(), String>>>,
}
impl Drop for Job {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

impl PngUiState {
    pub fn is_running(&self) -> bool {
        self.job.is_some()
    }
    pub fn cancel(&self) {
        if let Some(job) = &self.job {
            job.cancel.store(true, Ordering::Relaxed);
        }
    }
    pub fn poll(&mut self) {
        let Some(job) = &self.job else {
            return;
        };
        match job.receiver.try_recv() {
            Ok(result) => {
                self.open_result = job
                    .opening
                    .as_ref()
                    .and_then(|receiver| receiver.try_recv().ok());
                self.message = Some(result);
                self.job = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                self.message=Some(Err("Rendered image export worker stopped unexpectedly. Inspect any retained export transaction before retrying".into()));
                self.job = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
    }
    /// Uses the existing job owner so New/Open/Close also cancel and await video.
    pub(crate) fn start_video(
        &mut self,
        doc: ankhimate_document::Document,
        settings: crate::video_export::Settings,
        destination: PathBuf,
        ctx: egui::Context,
    ) {
        self.start_video_with_opener(
            doc,
            settings,
            destination,
            ctx,
            crate::system_open::open_file,
        );
    }
    pub(crate) fn start_video_with_opener(
        &mut self,
        doc: ankhimate_document::Document,
        settings: crate::video_export::Settings,
        destination: PathBuf,
        ctx: egui::Context,
        opener: impl FnOnce(&std::path::Path) -> Result<(), String> + Send + 'static,
    ) {
        if self.is_running() {
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(Mutex::new(("Checking FFmpeg", 0, 1)));
        let (sender, receiver) = mpsc::channel();
        let (opening_sender, opening_receiver) = mpsc::channel();
        let job = Job {
            cancel: cancel.clone(),
            progress: progress.clone(),
            receiver,
            opening: Some(opening_receiver),
        };
        self.message = None;
        self.open_result = None;
        match std::thread::Builder::new()
            .name("video-export".into())
            .spawn(move || {
                let result = crate::video_export::export(
                    &doc,
                    &settings,
                    &destination,
                    &cancel,
                    |phase, done, total| {
                        if let Ok(mut p) = progress.lock() {
                            *p = (phase, u64::from(done), u64::from(total));
                        }
                        ctx.request_repaint();
                    },
                )
                .map(|count| {
                    format!(
                        "Exported {count} {} frames to {}",
                        settings.format.label(),
                        destination.display()
                    )
                });
                if result.is_ok() && settings.open_after_export && !cancel.load(Ordering::Relaxed) {
                    if let Ok(mut p) = progress.lock() {
                        *p = ("Opening published file", 1, 1);
                    }
                    ctx.request_repaint();
                    let _ = opening_sender.send(opener(&destination));
                }
                let _ = sender.send(result);
                ctx.request_repaint();
            }) {
            Ok(_) => self.job = Some(job),
            Err(e) => self.message = Some(Err(format!("Cannot start video export: {e}"))),
        }
    }
    pub(crate) fn video_status(&mut self, ui: &mut egui::Ui) {
        self.poll();
        if let Some(job) = &self.job {
            let progress = job.progress.lock().ok().map(|p| *p);
            if progress.is_some_and(|p| p.0 == "Opening published file") {
                ui.label("File published; opening request in progress");
            } else if ui.button("Cancel export").clicked() {
                self.cancel();
            }
            if let Some((phase, done, total)) = progress {
                ui.add(
                    egui::ProgressBar::new(done as f32 / total.max(1) as f32)
                        .text(format!("{phase}: {done}/{total}")),
                );
            }
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }
        if let Some(message) = &self.message {
            match message {
                Ok(s) => {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, s);
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::LIGHT_RED, e);
                }
            }
        }
        if let Some(result) = &self.open_result {
            match result {
                Ok(()) => {
                    ui.label("Published file: opening requested from Windows");
                }
                Err(e) => {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        format!("Export published successfully; opening failed: {e}"),
                    );
                }
            }
        }
    }
    pub(crate) fn start(
        &mut self,
        doc: ankhimate_document::Document,
        request: SequenceRequest,
        destination: PathBuf,
        ctx: egui::Context,
    ) {
        if self.is_running() {
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(Mutex::new(("Framing", 0, 1)));
        let (sender, receiver) = mpsc::channel();
        let job = Job {
            cancel: cancel.clone(),
            progress: progress.clone(),
            receiver,
            opening: None,
        };
        self.message = None;
        self.open_result = None;
        let encoding = self.encoding;
        let sheet_settings = self.spritesheet.then_some((self.max_page, self.padding));
        let options = FrameOptions {
            padding: if self.spritesheet {
                self.padding
            } else {
                self.framing.padding
            },
            ..self.framing
        };
        let result = std::thread::Builder::new()
            .name("rendered-image-export".into())
            .spawn(move || {
                let report = |phase, done, total| {
                    if let Ok(mut current) = progress.lock() {
                        *current = (phase, u64::from(done), u64::from(total));
                    }
                    ctx.request_repaint();
                };
                let result = match sheet_settings {
                    Some((max_page, _)) if options.crops() => {
                        crate::spritesheet_export::export_options(
                            &doc,
                            &request,
                            max_page,
                            options,
                            &destination,
                            &cancel,
                            report,
                        )
                    }
                    Some((max_page, padding)) => crate::spritesheet_export::export(
                        &doc,
                        &request,
                        max_page,
                        padding,
                        &destination,
                        &cancel,
                        report,
                    ),
                    None => {
                        if options != FrameOptions::default() {
                            crate::png_export::export_options(
                                &doc,
                                &request,
                                encoding,
                                options,
                                &destination,
                                &cancel,
                                report,
                            )
                        } else if encoding == FrameEncoding::Png {
                            crate::png_export::export(&doc, &request, &destination, &cancel, report)
                        } else {
                            crate::png_export::export_encoded(
                                &doc,
                                &request,
                                encoding,
                                &destination,
                                &cancel,
                                report,
                            )
                        }
                    }
                }
                .map(|count| {
                    format!(
                        "Exported {count} {} frames to {}",
                        if sheet_settings.is_some() {
                            "spritesheet"
                        } else {
                            encoding.label()
                        },
                        destination.display()
                    )
                });
                let _ = sender.send(result);
                ctx.request_repaint();
            });
        match result {
            Ok(_) => self.job = Some(job),
            Err(e) => self.message = Some(Err(format!("Cannot start image export: {e}"))),
        }
    }
    pub(crate) fn start_batch(
        &mut self,
        doc: ankhimate_document::Document,
        batch: crate::batch_export::Batch,
        destination: PathBuf,
        ctx: egui::Context,
    ) {
        if self.is_running() {
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(Mutex::new(("Batch rendering", 0, batch.total_frames)));
        let (sender, receiver) = mpsc::channel();
        let job = Job {
            cancel: cancel.clone(),
            progress: progress.clone(),
            receiver,
            opening: None,
        };
        self.message = None;
        self.open_result = None;
        let result = std::thread::Builder::new()
            .name("rendered-batch-export".into())
            .spawn(move || {
                let clips = batch.clips.len();
                let result = batch
                    .export(&doc, &destination, &cancel, |_, done, total| {
                        if let Ok(mut p) = progress.lock() {
                            *p = ("Batch rendering", done, total);
                        }
                        ctx.request_repaint();
                    })
                    .map(|count| {
                        format!(
                            "Exported {count} frames across {clips} clips to {}",
                            destination.display()
                        )
                    });
                let _ = sender.send(result);
                ctx.request_repaint();
            });
        match result {
            Ok(_) => self.job = Some(job),
            Err(e) => self.message = Some(Err(format!("Cannot start batch export: {e}"))),
        }
    }
}

pub fn ui(ui: &mut egui::Ui, state: &mut AppState) {
    egui::ScrollArea::vertical()
        .id_salt("png_sequence_scroll")
        .show(ui, |ui| controls(ui, state));
}

fn controls(ui: &mut egui::Ui, state: &mut AppState) {
    let png = &mut state.session.export.png;
    png.poll();
    if png.fps == 0 {
        png.fps = state.doc.meta.fps.clamp(1, 240);
        png.width = 512;
        png.height = 512;
        png.max_page = 2048;
        png.padding = 2;
        png.batch_folder = "animation-batch".into();
    }
    png.batch_selected
        .retain(|id| state.doc.animations.contains_key(*id));
    let clips: Vec<_> = state
        .doc
        .animations
        .values()
        .map(|clip| clip.name.clone())
        .collect();
    if !clips.contains(&png.selected) {
        png.selected = clips.first().cloned().unwrap_or_default();
    }
    ui.label(
        egui::RichText::new(if png.spritesheet {
            "Spritesheet"
        } else {
            "Image sequence"
        })
        .strong(),
    );
    ui.add_enabled_ui(!png.is_running(), |ui| {
        if !png.spritesheet {
            egui::ComboBox::from_id_salt("sequence_encoding")
                .selected_text(png.encoding.label())
                .show_ui(ui, |ui| {
                    for encoding in [
                        FrameEncoding::Png,
                        FrameEncoding::Jpeg { background: [255; 3] },
                        FrameEncoding::WebpLossless,
                    ] {
                        if ui.selectable_label(
                            png.encoding.extension() == encoding.extension(),
                            encoding.label(),
                        ).clicked() {
                            png.encoding = encoding;
                        }
                    }
                });
            if let FrameEncoding::Jpeg { background } = &mut png.encoding {
                ui.horizontal(|ui| {
                    ui.label("JPEG opaque background:");
                    ui.color_edit_button_srgb(background);
                });
                ui.label("JPEG removes alpha by compositing onto this colour (default white). Lossy, quality 95.");
            }
        }
        egui::ComboBox::from_id_salt("render_bounds")
            .selected_text(match png.framing.bounds {
                BoundsMode::Historical => "Historical framing",
                BoundsMode::GlobalPixels => "Global pixel bounds (crop)",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut png.framing.bounds, BoundsMode::Historical, "Historical framing");
                ui.selectable_value(&mut png.framing.bounds, BoundsMode::GlobalPixels, "Global pixel bounds (lossless crop, no rescale)");
            });
        ui.checkbox(&mut png.framing.trim, "Trim transparent pixels per frame");
        if png.framing.crops() || (!png.spritesheet && png.framing.padding > 0) {
            ui.label("Cropped/padded output requires reconstruction metadata v1; source canvas and camera stay unchanged.");
            if png.spritesheet {
                ui.label("Fixed source-sized cells: trim does not reduce page count. The current Phaser example accepts untrimmed sheets only.");
            }
        }
        ui.checkbox(&mut png.batch,"Batch export");
        if png.batch {
            ui.label("All checked clips use these settings. Each clip keeps its own timing, camera and reconstruction.");
            let settings=batch_settings(png);
            for (ordinal,(id,clip)) in state.doc.animations.iter().enumerate() {
                let mut checked=png.batch_selected.contains(&id);
                let count=settings.count(clip).map(|n|format!("{n} frames")).unwrap_or_else(|e|format!("Invalid: {e}"));
                if ui.checkbox(&mut checked,format!("{} · {} · {count}",ordinal+1,clip.name)).changed() {
                    if checked {png.batch_selected.push(id);}else{png.batch_selected.retain(|key|*key!=id);}
                }
            }
            ui.horizontal(|ui|{ui.label("New batch folder:");ui.text_edit_singleline(&mut png.batch_folder);});
        } else {
        egui::ComboBox::from_id_salt("png_clip")
            .selected_text(if png.selected.is_empty() {
                "No animation"
            } else {
                &png.selected
            })
            .show_ui(ui, |ui| {
                for clip in &clips {
                    ui.selectable_value(&mut png.selected, clip.clone(), clip);
                }
            });
        }
        egui::Grid::new("png_options")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("FPS");
                ui.add(egui::DragValue::new(&mut png.fps).range(1..=240));
                ui.end_row();
                ui.label("Canvas width");
                ui.add(egui::DragValue::new(&mut png.width).range(1..=8192));
                ui.end_row();
                ui.label("Canvas height");
                ui.add(egui::DragValue::new(&mut png.height).range(1..=8192));
                ui.end_row();
                if png.spritesheet {
                    ui.label("Max page");
                    ui.add(egui::DragValue::new(&mut png.max_page).range(1..=4096));
                    ui.end_row();
                    ui.label("Padding");
                    ui.add(egui::DragValue::new(&mut png.padding).range(0..=32));
                    ui.end_row();
                } else {
                    ui.label("Image padding");
                    ui.add(egui::DragValue::new(&mut png.framing.padding).range(0..=32));
                    ui.end_row();
                }
            });
    });
    ui.label(
        egui::RichText::new(
            if !png.spritesheet && matches!(png.encoding, FrameEncoding::Jpeg { .. }) {
                "Opaque RGB • constant framing • default skin"
            } else {
                "Transparent RGBA • common camera • default skin"
            },
        )
        .small()
        .weak(),
    );
    ui.label(
        egui::RichText::new("Use a new folder: existing output names are never replaced.")
            .small()
            .weak(),
    );
    let request = SequenceRequest {
        animation: png.selected.clone(),
        fps: png.fps,
        width: png.width,
        height: png.height,
    };
    let options = FrameOptions {
        padding: if png.spritesheet {
            png.padding
        } else {
            png.framing.padding
        },
        ..png.framing
    };
    let validation = if png.batch {
        batch_summary(&state.doc, png)
    } else {
        options
            .validate(png.width, png.height)
            .and_then(|_| request.frame_count(&state.doc))
            .and_then(|count| {
                if png.spritesheet {
                    ankhimate_render::spritesheet::Layout::page_count(
                        count,
                        png.width,
                        png.height,
                        png.max_page,
                        png.padding,
                    )
                    .map(|pages| (count, pages))
                } else {
                    Ok((count, 0))
                }
            })
            .map(|(n, p)| (u64::from(n), u64::from(p)))
            .map_err(|e| e.to_string())
    };
    match &validation {
        Ok((count, pages)) => {
            ui.label(if png.batch {
                format!(
                    "{} clips · {count} total frames · {pages} pages",
                    png.batch_selected.len()
                )
            } else if png.spritesheet {
                format!("{count} frames · {pages} page(s)")
            } else {
                format!("{count} frames · {} …", png.encoding.frame_name(0))
            });
        }
        Err(e) => {
            ui.label(
                egui::RichText::new(e.to_string())
                    .small()
                    .color(egui::Color32::LIGHT_RED),
            );
        }
    }
    if let Some(job) = &png.job {
        ui.horizontal(|ui| {
            if job.cancel.load(Ordering::Relaxed) {
                ui.label("Cancelling safely…");
            } else if ui
                .button(if png.spritesheet {
                    "Cancel spritesheet export"
                } else {
                    "Cancel image export"
                })
                .clicked()
            {
                png.cancel();
            }
            if let Ok((phase, done, total)) = job.progress.lock().map(|p| *p) {
                ui.add(
                    egui::ProgressBar::new(done as f32 / total.max(1) as f32)
                        .text(format!("{phase}: {done}/{total}")),
                );
            }
        });
    } else if ui
        .add_enabled(
            validation.is_ok(),
            egui::Button::new(if png.batch {
                "Export batch…".into()
            } else if png.spritesheet {
                "Export spritesheet…".into()
            } else {
                format!("Export {} sequence…", png.encoding.label())
            }),
        )
        .clicked()
        && let Some(destination) = rfd::FileDialog::new()
            .set_title(if png.batch {
                "Batch — choose an existing parent for the new folder"
            } else if png.spritesheet {
                "Spritesheet — choose a new output folder"
            } else {
                "Image sequence — choose a new output folder"
            })
            .pick_folder()
    {
        let sheet = png.spritesheet.then_some((png.max_page, png.padding));
        let encoding = png.encoding;
        if png.batch {
            let settings = batch_settings(png);
            let ids = png.batch_selected.clone();
            let destination = destination.join(&png.batch_folder);
            state.session.export.review = Some(Box::new(crate::export_preflight::Review::batch(
                state,
                &ids,
                settings,
                destination,
            )));
            ui.ctx().request_repaint();
            return;
        }
        state.session.export.review = Some(Box::new(
            if options.crops() || (sheet.is_none() && options.padding > 0) {
                crate::export_preflight::Review::rendered_options(
                    state,
                    request,
                    destination,
                    sheet,
                    encoding,
                    options,
                )
            } else if sheet.is_some() || encoding == FrameEncoding::Png {
                crate::export_preflight::Review::rendered(state, request, destination, sheet)
            } else {
                crate::export_preflight::Review::rendered_encoded(
                    state,
                    request,
                    destination,
                    sheet,
                    encoding,
                )
            },
        ));
        ui.ctx().request_repaint();
        return;
    }
    if let Some(message) = &png.message {
        match message {
            Ok(text) => {
                ui.colored_label(egui::Color32::LIGHT_GREEN, text);
            }
            Err(text) => {
                ui.colored_label(egui::Color32::LIGHT_RED, text);
            }
        }
    }
}

fn batch_settings(png: &PngUiState) -> crate::batch_export::Settings {
    crate::batch_export::Settings {
        fps: png.fps,
        width: png.width,
        height: png.height,
        encoding: if png.spritesheet {
            FrameEncoding::Png
        } else {
            png.encoding
        },
        framing: FrameOptions {
            padding: if png.spritesheet {
                png.padding
            } else {
                png.framing.padding
            },
            ..png.framing
        },
        max_page: png.spritesheet.then_some(png.max_page),
    }
}
fn batch_summary(
    doc: &ankhimate_document::Document,
    png: &PngUiState,
) -> Result<(u64, u64), String> {
    ankhimate_export::batch::folder_name(&png.batch_folder)?;
    if png.batch_selected.is_empty() {
        return Err("Select at least one animation".into());
    }
    let settings = batch_settings(png);
    let mut total = 0;
    let mut pages = 0;
    for id in &png.batch_selected {
        let clip = doc
            .animations
            .get(*id)
            .ok_or("Selected animation no longer exists")?;
        let count = settings.count(clip)?;
        total += u64::from(count);
        if let Some(page) = settings.max_page {
            pages += u64::from(
                ankhimate_render::spritesheet::Layout::page_count(
                    count,
                    settings.width,
                    settings.height,
                    page,
                    settings.framing.padding,
                )
                .map_err(|e| e.to_string())?,
            );
        }
    }
    Ok((total, pages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spritesheet_worker_keeps_the_selected_output_settings_from_launch() {
        let mut doc = ankhimate_document::Document::new();
        doc.animations
            .insert(ankhimate_core::animation::Animation::new("static", 1.0));
        let dir = tempfile::tempdir().unwrap();
        let mut png = PngUiState {
            spritesheet: true,
            max_page: 128,
            padding: 2,
            ..Default::default()
        };
        png.start(
            doc,
            SequenceRequest {
                animation: "static".into(),
                fps: 60,
                width: 16,
                height: 16,
            },
            dir.path().to_path_buf(),
            egui::Context::default(),
        );
        png.spritesheet = false;
        png.max_page = 1;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while png.is_running() {
            png.poll();
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        assert!(
            png.message
                .unwrap()
                .unwrap()
                .contains("60 spritesheet frames")
        );
        let metadata: serde_json::Value =
            serde_json::from_slice(&std::fs::read(dir.path().join("spritesheet.json")).unwrap())
                .unwrap();
        assert_eq!(metadata["max_page_size"], 128);
        assert_eq!(metadata["pages"].as_array().unwrap().len(), 2);
        assert!(!dir.path().join("sequence.json").exists());
    }

    #[test]
    fn replacing_a_document_cancels_and_retains_its_worker_without_changing_new_document() {
        let mut state = AppState::default();
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        state.session.export.png.job = Some(Job {
            cancel: cancel.clone(),
            progress: Arc::new(Mutex::new(("Rendering", 1, 60))),
            receiver,
            opening: None,
        });
        let mut new_doc = ankhimate_document::Document::new();
        new_doc.meta.name = "newer document".into();
        state.replace_document(new_doc);
        assert!(cancel.load(Ordering::Relaxed));
        assert!(state.session.export.png.is_running());
        sender.send(Err("cancelled safely".into())).unwrap();
        state.session.export.png.poll();
        assert!(!state.session.export.png.is_running());
        assert_eq!(state.doc.meta.name, "newer document");
        assert_eq!(state.history.undo_label(), None);
    }

    #[test]
    fn a_second_start_cannot_replace_an_in_flight_export() {
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let mut png = PngUiState {
            job: Some(Job {
                cancel: cancel.clone(),
                progress: Arc::new(Mutex::new(("Framing", 0, 60))),
                receiver,
                opening: None,
            }),
            ..Default::default()
        };
        png.start(
            ankhimate_document::Document::new(),
            SequenceRequest {
                animation: "missing".into(),
                fps: 60,
                width: 32,
                height: 32,
            },
            PathBuf::new(),
            egui::Context::default(),
        );
        assert!(!cancel.load(Ordering::Relaxed));
        sender.send(Ok("original export".into())).unwrap();
        png.poll();
        assert_eq!(png.message, Some(Ok("original export".into())));
    }
    #[test]
    fn batch_ui_exposes_multi_animation_selection() {
        let mut state = AppState::default();
        state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("first", 1.0));
        state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("second", 2.0));
        let ctx = egui::Context::default();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 1200.0),
                )),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show_inside(root, |ui| controls(ui, &mut state));
            },
        );
        assert!(output.shapes.iter().any(|s|matches!(&s.shape,egui::epaint::Shape::Text(t) if t.galley.text()=="Batch export")),"Export panel has no multi-animation selection mode");
    }
    #[test]
    fn batch_egui_clicks_select_multiple_clips_and_update_counts_without_document_edits() {
        fn frame(
            ctx: &egui::Context,
            state: &mut AppState,
            events: Vec<egui::Event>,
        ) -> egui::FullOutput {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 1600.0),
                    )),
                    events,
                    ..Default::default()
                },
                |root| {
                    egui::CentralPanel::default().show_inside(root, |ui| controls(ui, state));
                },
            )
        }
        fn label(output: &egui::FullOutput, text: &str) -> egui::Pos2 {
            output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::epaint::Shape::Text(t) if t.galley.text() == text => {
                        Some(t.pos + t.galley.rect.center().to_vec2())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing UI label: {text}"))
        }
        fn click(ctx: &egui::Context, state: &mut AppState, pos: egui::Pos2) {
            frame(
                ctx,
                state,
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            frame(
                ctx,
                state,
                vec![egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }],
            );
        }
        let mut state = AppState::default();
        state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("first", 1.0));
        state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("second", 2.0));
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let ctx = egui::Context::default();
        let out = frame(&ctx, &mut state, vec![]);
        let p = label(&out, "Batch export");
        click(&ctx, &mut state, p);
        assert!(state.session.export.png.batch);
        state.session.export.png.fps = 4;
        let out = frame(&ctx, &mut state, vec![]);
        let p = label(&out, "1 · first · 4 frames");
        click(&ctx, &mut state, p);
        let out = frame(&ctx, &mut state, vec![]);
        let p = label(&out, "2 · second · 8 frames");
        click(&ctx, &mut state, p);
        assert_eq!(state.session.export.png.batch_selected.len(), 2);
        assert_eq!(
            batch_summary(&state.doc, &state.session.export.png).unwrap(),
            (12, 0)
        );
        state.session.export.png.fps = 8;
        assert_eq!(
            batch_summary(&state.doc, &state.session.export.png).unwrap(),
            (24, 0)
        );
        state.session.export.png.batch_selected.clear();
        assert!(batch_summary(&state.doc, &state.session.export.png).is_err());
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
        assert_eq!(state.history.undo_label(), None);
    }
}
#[cfg(test)]
mod opening_tests {
    use super::*;
    use crate::video_export::tests::{helper, walker};
    use crate::video_export::{Format, Settings};
    fn settings(format: Format, mode: &str) -> Settings {
        Settings {
            format,
            open_after_export: true,
            request: SequenceRequest {
                animation: "walk".into(),
                fps: 2,
                width: 8,
                height: 8,
            },
            executable: helper(mode).to_string_lossy().into_owned(),
            ..Default::default()
        }
    }
    fn wait(png: &mut PngUiState) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(8);
        while png.is_running() {
            png.poll();
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    #[test]
    fn successful_mp4_and_gif_open_once_after_publication_with_literal_unicode_paths() {
        for format in [Format::Mp4, Format::Gif] {
            let d = tempfile::tempdir().unwrap();
            let path = d
                .path()
                .join(format!("été & [copie] %video%.{}", format.extension()));
            let seen = Arc::new(Mutex::new(Vec::new()));
            let recorded = seen.clone();
            let mut png = PngUiState::default();
            png.start_video_with_opener(
                walker(),
                settings(format, "ok"),
                path.clone(),
                egui::Context::default(),
                move |p| {
                    assert!(p.is_file());
                    recorded.lock().unwrap().push(p.to_owned());
                    Ok(())
                },
            );
            wait(&mut png);
            for _ in 0..5 {
                png.poll();
            }
            assert!(png.message.as_ref().unwrap().is_ok());
            assert!(png.open_result.as_ref().unwrap().is_ok());
            assert_eq!(*seen.lock().unwrap(), vec![path]);
        }
    }
    #[test]
    fn opener_failure_preserves_published_success_and_bytes() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("movie.mp4");
        let mut png = PngUiState::default();
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let captured = bytes.clone();
        png.start_video_with_opener(
            walker(),
            settings(Format::Mp4, "ok"),
            path.clone(),
            egui::Context::default(),
            move |p| {
                *captured.lock().unwrap() = std::fs::read(p).unwrap();
                Err("No file association".into())
            },
        );
        wait(&mut png);
        assert!(png.message.as_ref().unwrap().is_ok());
        assert_eq!(
            png.open_result.as_ref().unwrap().as_ref().unwrap_err(),
            "No file association"
        );
        assert_eq!(std::fs::read(path).unwrap(), *bytes.lock().unwrap());
    }
    #[test]
    fn cancellation_collision_error_and_opt_out_never_call_opener() {
        for case in ["cancel", "collision", "failure", "disabled"] {
            let d = tempfile::tempdir().unwrap();
            let path = d.path().join("movie.mp4");
            let mut s = settings(
                Format::Mp4,
                if case == "failure" {
                    "fail"
                } else if case == "cancel" {
                    "block"
                } else {
                    "ok"
                },
            );
            if case == "collision" {
                std::fs::write(&path, b"old").unwrap();
            }
            if case == "disabled" {
                s.open_after_export = false;
            }
            let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let counter = calls.clone();
            let mut png = PngUiState::default();
            png.start_video_with_opener(
                walker(),
                s,
                path.clone(),
                egui::Context::default(),
                move |_| {
                    counter.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                },
            );
            if case == "cancel" {
                png.cancel();
            }
            wait(&mut png);
            assert_eq!(calls.load(Ordering::Relaxed), 0, "{case}");
            assert!(png.open_result.is_none());
            assert_eq!(png.message.as_ref().unwrap().is_ok(), case == "disabled");
            if case == "collision" {
                assert_eq!(std::fs::read(path).unwrap(), b"old");
            }
        }
    }
}
