//! Session-only export review. Existing planners and publishers remain authoritative.
use crate::app_state::AppState;
use ankhimate_export::{
    preflight::{self, Report, Severity, Target},
    preset::Preset,
    run::{self, Plan, Survey},
};
use ankhimate_formats::schema::Project;
use ankhimate_render::{encoding::FrameEncoding, framing::FrameOptions, sequence::SequenceRequest};
use eframe::egui;
use std::path::PathBuf;

#[derive(Debug)]
enum Job {
    Data,
    Batch(Box<crate::batch_export::Batch>),
    Rendered {
        request: SequenceRequest,
        sheet: Option<(u32, u32)>,
        encoding: FrameEncoding,
        options: FrameOptions,
    },
}

#[derive(Debug)]
pub struct Review {
    revision: u64,
    project: Project,
    destination: PathBuf,
    plan: Plan,
    job: Job,
    report: Report,
    survey: Survey,
    accept_warnings: bool,
    accept_replacements: bool,
    error: Option<String>,
}

impl Review {
    pub fn data(state: &AppState, preset: &Preset, destination: PathBuf, project: Project) -> Self {
        let target = Target::for_preset(preset);
        let plan = run::plan(&project, &state.doc.assets, preset).map_err(|e| e.to_string());
        let mut review = Self::from_plan(state, destination, project, target, plan);
        if !preset.atlas.enabled && !state.doc.assets.images.is_empty() {
            review.report.push(Severity::Warning, "no-atlas", &preset.name,
                "Atlas disabled: texture loading and UV mapping must be qualified for this preset and its consumer");
        }
        let copied_images = if preset.copy_images {
            state.doc.assets.images.len()
        } else {
            0
        };
        let atlas_pages = review.plan.binaries.len().saturating_sub(copied_images);
        if target == Target::PhaserAtlas && (!preset.atlas.enabled || atlas_pages != 1) {
            review.report.push(Severity::Error, "phaser-atlas-pages", &preset.name,
                "This Phaser atlas template requires exactly one atlas page; its JSON cannot map multiple pages. Use Spritesheet or a one-page atlas");
        }
        review
    }

    pub fn from_plan(
        state: &AppState,
        destination: PathBuf,
        project: Project,
        target: Target,
        plan: Result<Plan, String>,
    ) -> Self {
        let report = preflight::inspect(&project, &state.doc.assets, target, None);
        Self::assemble(state, destination, project, Job::Data, report, plan)
    }

    pub fn rendered(
        state: &AppState,
        request: SequenceRequest,
        destination: PathBuf,
        sheet: Option<(u32, u32)>,
    ) -> Self {
        Self::rendered_encoded(state, request, destination, sheet, FrameEncoding::Png)
    }

    pub fn rendered_encoded(
        state: &AppState,
        request: SequenceRequest,
        destination: PathBuf,
        sheet: Option<(u32, u32)>,
        encoding: FrameEncoding,
    ) -> Self {
        Self::rendered_options(
            state,
            request,
            destination,
            sheet,
            encoding,
            FrameOptions {
                padding: sheet.map_or(0, |(_, p)| p),
                ..Default::default()
            },
        )
    }
    pub fn rendered_options(
        state: &AppState,
        request: SequenceRequest,
        destination: PathBuf,
        sheet: Option<(u32, u32)>,
        encoding: FrameEncoding,
        mut options: FrameOptions,
    ) -> Self {
        if let Some((_, padding)) = sheet {
            options.padding = padding;
        }
        let encoding = if sheet.is_some() {
            FrameEncoding::Png
        } else {
            encoding
        };
        let project = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        let mut report = preflight::inspect(
            &project,
            &state.doc.assets,
            Target::Rendered,
            Some(&request.animation),
        );
        if let FrameEncoding::Jpeg { background } = encoding {
            report.push(Severity::Info,"jpeg-background","Sequence",format!("JPEG quality 95, opaque RGB background {:?}; transparency is composited onto this colour. JPEG is lossy.",background));
        }
        if options.crops() || (sheet.is_none() && options.padding > 0) {
            report.push(Severity::Warning,"render-reconstruction","Output",
                "Opt-in pixel cropping/padding requires reconstruction metadata v1. Camera/source canvas stay unchanged; read source_rect and content_rect to place each frame. Existing untrimmed-only readers, including the Phaser example, do not support this mode. Spritesheet cells keep source dimensions; page count is not reduced.");
        }
        let plan = options
            .validate(request.width, request.height)
            .map_err(|e| e.to_string())
            .and_then(|_| match sheet {
                Some((max_page, padding)) => {
                    crate::spritesheet_export::output_plan(&state.doc, &request, max_page, padding)
                        .map(|(_, p)| p)
                }
                None => {
                    if encoding == FrameEncoding::Png {
                        crate::png_export::output_plan(&state.doc, &request)
                    } else {
                        crate::png_export::output_plan_encoded(&state.doc, &request, encoding)
                    }
                }
            });
        Self::assemble(
            state,
            destination,
            project,
            Job::Rendered {
                request,
                sheet,
                encoding,
                options,
            },
            report,
            plan,
        )
    }

    pub fn batch(
        state: &AppState,
        selected: &[ankhimate_core::ids::AnimationId],
        settings: crate::batch_export::Settings,
        destination: PathBuf,
    ) -> Self {
        let project = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        match crate::batch_export::Batch::new(&state.doc, selected, settings) {
            Ok(batch) => {
                let mut report = batch.diagnostics(&state.doc);
                if let Err(e) = ankhimate_export::batch::inspect(&batch.plan, &destination) {
                    report.push(Severity::Error, "batch-destination", "Batch", e.to_string());
                }
                report.push(Severity::Info,"batch-publication","Batch","All clips are prepared in an owned sibling temporary directory, then the complete directory is published to a new destination. Existing folders are always refused. Abrupt crashes and unrelated filesystem writers remain outside ordinary failure guarantees.");
                let plan = Plan {
                    files: batch
                        .plan
                        .files
                        .iter()
                        .map(|f| ankhimate_export::template::RenderedFile {
                            path: f.path.clone(),
                            contents: f.contents.clone(),
                        })
                        .collect(),
                    binaries: batch.plan.binaries.clone(),
                };
                Self::assemble(
                    state,
                    destination,
                    project,
                    Job::Batch(Box::new(batch)),
                    report,
                    Ok(plan),
                )
            }
            Err(e) => Self::assemble(
                state,
                destination,
                project,
                Job::Data,
                Report::default(),
                Err(e),
            ),
        }
    }

    fn assemble(
        state: &AppState,
        destination: PathBuf,
        project: Project,
        job: Job,
        mut report: Report,
        plan: Result<Plan, String>,
    ) -> Self {
        let plan = match plan {
            Ok(plan) => plan,
            Err(e) => {
                report.push(Severity::Error, "export-plan", "Output", e);
                Plan::default()
            }
        };
        if plan.paths().is_empty() {
            report.push(
                Severity::Error,
                "empty-output",
                "Output",
                "No files are planned",
            );
        }
        if let Err(e) = run::inspect(&plan, &destination) {
            report.push(Severity::Error, "unsafe-output", "Output", e.to_string());
        }
        let survey = plan.survey(&destination);
        if matches!(job, Job::Rendered { .. }) && !survey.replaced.is_empty() {
            report.push(
                Severity::Error,
                "rendered-collision",
                "Output",
                "Image sequence and Spritesheet require unused output names; choose a new folder",
            );
        }
        report.push(Severity::Info,"publication","Files","Read-only inventory: orphans are never deleted. Permissions, locks and paths are checked again by the staged publisher; this review does not reserve the destination. Orphan discovery is best effort.");
        Self {
            revision: state.revision,
            project,
            destination,
            plan,
            job,
            report,
            survey,
            accept_warnings: false,
            accept_replacements: false,
            error: None,
        }
    }

    fn ready(&self) -> bool {
        self.report.allowed(self.accept_warnings)
            && (self.survey.replaced.is_empty() || self.accept_replacements)
    }

    fn receipt(&self, result: &str) -> String {
        serde_json::to_string_pretty(&serde_json::json!({
            "destination":self.destination,"diagnostics":self.report,
            "created":self.survey.created,"replaced":self.survey.replaced,"orphans_preserved":self.survey.orphans,
            "fidelity_warnings_accepted":self.accept_warnings,
            "replacements_authorized":self.accept_replacements,"result":result
        })).expect("preflight receipt serializes")
    }

    fn publish(&mut self, state: &mut AppState, ctx: &egui::Context) -> Result<(), String> {
        if !self.ready() {
            return Err(
                "Resolve errors and explicitly accept the listed warnings / replacements first"
                    .into(),
            );
        }
        if state.revision != self.revision
            || ankhimate_formats::convert::to_schema(&state.doc.as_project_ref()) != self.project
        {
            return Err("Project changed since review. Cancel and prepare a new preflight; nothing was written".into());
        }
        if matches!(self.job, Job::Batch(_)) {
            ankhimate_export::batch::inspect(&self.plan, &self.destination)
                .map_err(|e| e.to_string())?;
        }
        run::inspect(&self.plan, &self.destination).map_err(|e| e.to_string())?;
        let current = self.plan.survey(&self.destination);
        if current != self.survey {
            self.survey = current;
            self.accept_warnings = false;
            self.accept_replacements = false;
            if matches!(self.job, Job::Rendered { .. }) && !self.survey.replaced.is_empty() {
                self.report.push(
                    Severity::Error,
                    "rendered-collision",
                    "Output",
                    "Output appeared since review; choose a new destination",
                );
            }
            return Err("Directory inventory changed. Review it again; nothing was written".into());
        }
        match &self.job {
            Job::Batch(batch) => {
                let png = &mut state.session.export.png;
                if png.is_running() {
                    return Err("A rendered export is already running".into());
                }
                // Rebuild on the same checked snapshot; no UI changes can alter reviewed settings.
                let ids: Vec<_> = batch.clips.iter().map(|c| c.id).collect();
                let job = crate::batch_export::Batch::new(&state.doc, &ids, batch.settings)?;
                png.start_batch(
                    state.doc.clone(),
                    job,
                    self.destination.clone(),
                    ctx.clone(),
                );
                state.session.export.last_preflight =
                    Some(self.receipt(
                        "Batch worker launch requested; consult its completion/error status",
                    ));
            }
            Job::Data => {
                let result =
                    run::write_reviewed(&self.plan, &self.destination, &self.survey.replaced)
                        .map_err(|e| e.to_string());
                state.session.export.last_preflight = Some(self.receipt(match &result {
                    Ok(()) => "Published successfully",
                    Err(e) => e,
                }));
                result?;
                state.session.export.last_export = Some(format!(
                    "Wrote {} files to {} after preflight",
                    self.plan.paths().len(),
                    self.destination.display()
                ));
            }
            Job::Rendered {
                request,
                sheet,
                encoding,
                options,
            } => {
                let png = &mut state.session.export.png;
                if png.is_running() {
                    return Err("A rendered export is already running".into());
                }
                png.spritesheet = sheet.is_some();
                let sequence_padding = png.framing.padding;
                png.framing = *options;
                if sheet.is_some() {
                    png.framing.padding = sequence_padding;
                }
                if sheet.is_none() {
                    png.encoding = *encoding;
                }
                if let Some((max_page, padding)) = sheet {
                    png.max_page = *max_page;
                    png.padding = *padding;
                }
                png.start(
                    state.doc.clone(),
                    request.clone(),
                    self.destination.clone(),
                    ctx.clone(),
                );
                state.session.export.last_preflight = Some(self.receipt(
                    "Rendered worker launch requested; consult its completion/error status",
                ));
            }
        }
        Ok(())
    }
}

fn paths(ui: &mut egui::Ui, label: &str, entries: &[String]) {
    ui.collapsing(format!("{label} ({})", entries.len()), |ui| {
        egui::ScrollArea::vertical()
            .id_salt(label)
            .max_height(110.0)
            .show_rows(ui, 18.0, entries.len(), |ui, range| {
                for i in range {
                    ui.label(&entries[i]);
                }
            });
    });
}

pub fn show(ctx: &egui::Context, state: &mut AppState, theme: &crate::theme::Theme) {
    let Some(mut review) = state.session.export.review.take() else {
        return;
    };
    let mut publish = false;
    let mut cancel = false;
    let dialog = crate::ui::dialog::Dialog::new("export-preflight", "Export preflight")
        .width(670.0)
        .max_height(560.0)
        .show(ctx, theme, |ui| {
            ui.label(review.destination.display().to_string());
            ui.label(format!(
                "{} planned files | {} replacements | {} orphans preserved",
                review.plan.paths().len(),
                review.survey.replaced.len(),
                review.survey.orphans.len()
            ));
            egui::ScrollArea::vertical()
                .id_salt("export-diagnostics")
                .max_height(210.0)
                .show(ui, |ui| {
                    for item in &review.report.items {
                        let (label, color) = match item.severity {
                            Severity::Error => ("ERROR", egui::Color32::LIGHT_RED),
                            Severity::Warning => ("WARNING", egui::Color32::YELLOW),
                            Severity::Info => ("INFO", egui::Color32::LIGHT_BLUE),
                        };
                        ui.colored_label(
                            color,
                            format!("{label} [{}] {}: {}", item.code, item.subject, item.message),
                        );
                    }
                });
            paths(ui, "Create", &review.survey.created);
            paths(ui, "Replace", &review.survey.replaced);
            paths(ui, "Orphans — preserved", &review.survey.orphans);
            if review.report.has_warnings() {
                ui.checkbox(
                    &mut review.accept_warnings,
                    "I accept the listed fidelity / qualification warnings",
                );
            }
            if matches!(review.job, Job::Data) && !review.survey.replaced.is_empty() {
                ui.checkbox(
                    &mut review.accept_replacements,
                    "I authorize replacing the listed existing files",
                );
            }
            if let Some(error) = &review.error {
                ui.colored_label(egui::Color32::LIGHT_RED, error);
            }
            ui.horizontal(|ui| {
                publish = ui
                    .add_enabled(review.ready(), egui::Button::new("Publish export"))
                    .clicked();
                cancel = ui.button("Cancel").clicked();
                if ui.button("Copy diagnostics").clicked() {
                    ctx.copy_text(
                        review.receipt(
                            review
                                .error
                                .as_deref()
                                .unwrap_or("Awaiting publication; no writes by preflight"),
                        ),
                    );
                }
            });
        });
    if cancel || dialog.closed {
        return;
    }
    if publish {
        match review.publish(state, ctx) {
            Ok(()) => return,
            Err(e) => review.error = Some(e),
        }
    }
    state.session.export.review = Some(review);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn review(state: &AppState, path: PathBuf) -> Review {
        Review::data(
            state,
            &ankhimate_export::presets::default_preset(),
            path,
            ankhimate_formats::convert::to_schema(&state.doc.as_project_ref()),
        )
    }

    #[test]
    fn t818_global_inspection_does_not_change_review_consent_inventory_or_writer() {
        let mut state = AppState::default();
        let temp = tempfile::tempdir().unwrap();
        let mut first = review(&state, temp.path().to_path_buf());
        let receipt = first.receipt("before");
        state
            .diagnostics
            .poll(&state.doc, state.revision, std::time::Duration::ZERO);
        let second = review(&state, temp.path().to_path_buf());
        assert_eq!(receipt, second.receipt("before"));
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 0);
        first.accept_warnings = true;
        first
            .publish(&mut state, &egui::Context::default())
            .unwrap();
        let mut repeated = review(&state, temp.path().to_path_buf());
        assert!(!repeated.ready()); // replacement consent still mandatory
        state.diagnostics.poll(
            &state.doc,
            state.revision,
            std::time::Duration::from_secs(1),
        );
        assert!(!repeated.ready());
        repeated.accept_replacements = true;
        assert!(repeated.ready());
        state.doc.meta.fps = 0;
        let invalid = Review::from_plan(
            &state,
            temp.path().join("invalid"),
            ankhimate_formats::convert::to_schema(&state.doc.as_project_ref()),
            Target::Runtime,
            Err("blocked plan".into()),
        );
        assert!(!invalid.ready());
        assert!(!temp.path().join("invalid").exists());
    }

    #[test]
    fn phaser_atlas_page_checks_follow_the_real_plan_even_with_nested_stems() {
        let mut state = AppState::default();
        let mut bytes = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(8, 8, image::Rgba([200, 50, 100, 255]))
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        state
            .doc
            .assets
            .add(ankhimate_core::assets::ImageAsset::new(
                "art",
                bytes.into_inner(),
                8,
                8,
            ));
        let mut preset = Preset::from_json(ankhimate_export::presets::PHASER_ATLAS).unwrap();
        preset.atlas.stem = "images/small".into();
        preset.atlas.settings.max_page = 16;
        let temp = tempfile::tempdir().unwrap();
        let make = |state: &AppState, preset: &Preset| {
            Review::data(
                state,
                preset,
                temp.path().to_path_buf(),
                ankhimate_formats::convert::to_schema(&state.doc.as_project_ref()),
            )
        };
        assert!(!make(&state, &preset).report.has_errors());
        let first = state.doc.assets.images.values().next().unwrap().clone();
        state
            .doc
            .assets
            .add(ankhimate_core::assets::ImageAsset::new(
                "art2",
                first.bytes,
                8,
                8,
            ));
        preset.copy_images = true;
        assert!(
            make(&state, &preset)
                .report
                .items
                .iter()
                .any(|d| d.code == "phaser-atlas-pages" && d.severity == Severity::Error)
        );
    }

    #[test]
    fn late_lock_failure_preserves_document_assets_history_and_selection() {
        let mut state = AppState::default();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh");
        state.replace_document(ankhimate_document::Document::from_loaded(
            ankhimate_formats::load(&path).unwrap().0,
        ));
        state.create_animation();
        state.session.selection = Some(crate::session::Selection::Bone(
            state.doc.skeleton.update_order[0],
        ));
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let revision = state.revision;
        let selection = state.session.selection.clone();
        let history = state.history.undo_label().map(str::to_owned);
        let assets: Vec<_> = state
            .doc
            .assets
            .images
            .values()
            .map(|a| (a.name.clone(), a.bytes.clone()))
            .collect();
        let temp = tempfile::tempdir().unwrap();
        let mut review = review(&state, temp.path().to_path_buf());
        assert!(review.ready());
        std::fs::write(temp.path().join(".ankhimate-export.lock"), b"other export").unwrap();
        assert!(
            review
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert_eq!(revision, state.revision);
        assert_eq!(selection, state.session.selection);
        assert_eq!(history.as_deref(), state.history.undo_label());
        assert_eq!(
            before,
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
        );
        assert_eq!(
            assets,
            state
                .doc
                .assets
                .images
                .values()
                .map(|a| (a.name.clone(), a.bytes.clone()))
                .collect::<Vec<_>>()
        );
        state.undo();
        assert_ne!(
            before,
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
        );
        state.redo();
        assert_eq!(
            before,
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
        );
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
    }
    #[test]
    fn cancel_is_read_only_and_keeps_document_history_selection() {
        let state = AppState::default();
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("cancelled");
        let history = state.history.undo_label();
        let selection = state.session.selection.clone();
        let review = review(&state, output.clone());
        assert!(review.ready());
        drop(review);
        assert!(!output.exists());
        assert_eq!(
            before,
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap()
        );
        assert_eq!(history, state.history.undo_label());
        assert_eq!(selection, state.session.selection);
    }
    #[test]
    fn replacement_needs_consent_and_orphans_survive() {
        let mut state = AppState::default();
        let temp = tempfile::tempdir().unwrap();
        let mut first = review(&state, temp.path().to_path_buf());
        first
            .publish(&mut state, &egui::Context::default())
            .unwrap();
        std::fs::write(temp.path().join("orphan.txt"), b"keep").unwrap();
        let mut next = review(&state, temp.path().to_path_buf());
        assert!(!next.ready());
        assert!(next.publish(&mut state, &egui::Context::default()).is_err());
        next.accept_replacements = true;
        next.publish(&mut state, &egui::Context::default()).unwrap();
        assert_eq!(
            std::fs::read(temp.path().join("orphan.txt")).unwrap(),
            b"keep"
        );
        assert!(
            state
                .session
                .export
                .last_preflight
                .as_ref()
                .unwrap()
                .contains("\"replacements_authorized\": true")
        );
    }
    #[test]
    fn inventory_changes_and_stale_documents_do_not_publish() {
        let mut state = AppState::default();
        let temp = tempfile::tempdir().unwrap();
        let mut review = review(&state, temp.path().to_path_buf());
        let path = review.plan.paths()[0].to_string();
        std::fs::write(temp.path().join(&path), b"previous").unwrap();
        assert!(
            review
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert!(!review.ready());
        assert_eq!(std::fs::read(temp.path().join(&path)).unwrap(), b"previous");
        review.accept_replacements = true;
        state.revision += 1;
        assert!(
            review
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert_eq!(std::fs::read(temp.path().join(&path)).unwrap(), b"previous");
    }
    #[test]
    fn warnings_require_consent_and_errors_cannot_be_overridden() {
        let mut state = AppState::default();
        let temp = tempfile::tempdir().unwrap();
        let project = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
        let mut review = Review::from_plan(
            &state,
            temp.path().join("custom"),
            project,
            Target::Custom,
            Ok(Plan {
                files: vec![ankhimate_export::template::RenderedFile {
                    path: "custom.json".into(),
                    contents: "{}".into(),
                }],
                binaries: vec![],
            }),
        );
        assert!(!review.ready());
        review.accept_warnings = true;
        assert!(review.ready());
        review
            .publish(&mut state, &egui::Context::default())
            .unwrap();
        assert!(
            state
                .session
                .export
                .last_preflight
                .as_ref()
                .unwrap()
                .contains("\"fidelity_warnings_accepted\": true")
        );
        let mut blocked = super::tests::review(&state, temp.path().join("blocked"));
        blocked.report.push(
            Severity::Error,
            "integrity",
            "project",
            "Invalid attachment",
        );
        blocked.accept_warnings = true;
        blocked.accept_replacements = true;
        assert!(
            blocked
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert!(!temp.path().join("blocked").exists());
    }
    #[test]
    fn rendered_collisions_stay_blocking_and_workers_keep_sixty_frames() {
        for sheet in [None, Some((32, 1))] {
            let mut state = AppState::default();
            state
                .doc
                .animations
                .insert(ankhimate_core::animation::Animation::new("static", 1.0));
            let temp = tempfile::tempdir().unwrap();
            let request = SequenceRequest {
                animation: "static".into(),
                fps: 60,
                width: 8,
                height: 8,
            };
            let mut review =
                Review::rendered(&state, request.clone(), temp.path().to_path_buf(), sheet);
            assert!(review.ready(), "{:?}", review.report);
            review
                .publish(&mut state, &egui::Context::default())
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while state.session.export.png.is_running() {
                state.session.export.png.poll();
                assert!(std::time::Instant::now() < deadline);
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            assert!(
                state
                    .session
                    .export
                    .png
                    .message
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .contains("60")
            );
            let mut blocked = Review::rendered(&state, request, temp.path().to_path_buf(), sheet);
            blocked.accept_warnings = true;
            blocked.accept_replacements = true;
            assert!(!blocked.ready());
            assert!(
                blocked
                    .publish(&mut state, &egui::Context::default())
                    .is_err()
            );
        }
    }
    #[test]
    fn encoded_preflight_is_read_only_and_worker_uses_reviewed_encoding_and_background() {
        for encoding in [
            FrameEncoding::Png,
            FrameEncoding::Jpeg {
                background: [24, 80, 160],
            },
            FrameEncoding::WebpLossless,
        ] {
            let mut state = AppState::default();
            state
                .doc
                .animations
                .insert(ankhimate_core::animation::Animation::new("static", 1.0));
            let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
            let temp = tempfile::tempdir().unwrap();
            let dest = temp.path().join("output");
            let request = SequenceRequest {
                animation: "static".into(),
                fps: 4,
                width: 8,
                height: 8,
            };
            let cancelled =
                Review::rendered_encoded(&state, request.clone(), dest.clone(), None, encoding);
            assert!(cancelled.ready(), "{:?}", cancelled.report);
            assert!(
                cancelled
                    .plan
                    .paths()
                    .contains(&encoding.frame_name(0).as_str())
            );
            drop(cancelled);
            assert!(!dest.exists());
            let mut review =
                Review::rendered_encoded(&state, request.clone(), dest.clone(), None, encoding);
            // Changes both before publication and after launch must not alter reviewed output.
            state.session.export.png.encoding = FrameEncoding::Jpeg {
                background: [255, 0, 255],
            };
            review
                .publish(&mut state, &egui::Context::default())
                .unwrap();
            state.session.export.png.encoding = FrameEncoding::Png;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while state.session.export.png.is_running() {
                state.session.export.png.poll();
                assert!(std::time::Instant::now() < deadline);
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            assert!(state.session.export.png.message.as_ref().unwrap().is_ok());
            let metadata: serde_json::Value =
                serde_json::from_slice(&std::fs::read(dest.join("sequence.json")).unwrap())
                    .unwrap();
            assert_eq!(metadata["filename_pattern"], encoding.filename_pattern());
            assert_eq!(
                metadata["background"],
                serde_json::json!(encoding.background())
            );
            assert_eq!(std::fs::read_dir(&dest).unwrap().count(), 5);
            assert!(dest.join(encoding.frame_name(0)).is_file());
            assert_eq!(
                ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
                before
            );
            let mut collision =
                Review::rendered_encoded(&state, request, dest.clone(), None, encoding);
            collision.accept_replacements = true;
            collision.accept_warnings = true;
            assert!(!collision.ready());
            assert!(
                collision
                    .publish(&mut state, &egui::Context::default())
                    .is_err()
            );
            assert_eq!(std::fs::read_dir(dest).unwrap().count(), 5);
        }
    }
    #[test]
    fn reconstruction_requires_consent_and_captures_all_reviewed_options() {
        use ankhimate_render::framing::BoundsMode;
        for (encoding, sheet) in [
            (FrameEncoding::Png, None),
            (FrameEncoding::WebpLossless, None),
            (
                FrameEncoding::Jpeg {
                    background: [24, 80, 160],
                },
                None,
            ),
            (FrameEncoding::Png, Some((24, 1))),
        ] {
            let mut state = AppState::default();
            state
                .doc
                .animations
                .insert(ankhimate_core::animation::Animation::new("static", 1.0));
            let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
            let history = state.history.undo_label().map(str::to_owned);
            let selection = state.session.selection.clone();
            let temp = tempfile::tempdir().unwrap();
            let dest = temp.path().join("new");
            let options = FrameOptions {
                bounds: BoundsMode::GlobalPixels,
                trim: true,
                padding: 1,
            };
            let req = SequenceRequest {
                animation: "static".into(),
                fps: 4,
                width: 8,
                height: 8,
            };
            let mut review = Review::rendered_options(
                &state,
                req.clone(),
                dest.clone(),
                sheet,
                encoding,
                options,
            );
            assert!(!review.ready());
            assert!(
                review
                    .publish(&mut state, &egui::Context::default())
                    .is_err()
            );
            assert!(!dest.exists());
            review.accept_warnings = true;
            assert!(review.ready());
            state.session.export.png.framing = FrameOptions::default();
            review
                .publish(&mut state, &egui::Context::default())
                .unwrap();
            if sheet.is_some() {
                assert_eq!(
                    state.session.export.png.framing.padding, 0,
                    "sheet padding must not change sequence padding"
                );
            }
            state.session.export.png.framing = FrameOptions::default();
            state.session.export.png.padding = 0;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while state.session.export.png.is_running() {
                state.session.export.png.poll();
                assert!(std::time::Instant::now() < deadline);
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            assert!(state.session.export.png.message.as_ref().unwrap().is_ok());
            let name = if sheet.is_some() {
                "spritesheet.json"
            } else {
                "sequence.json"
            };
            let metadata: serde_json::Value =
                serde_json::from_slice(&std::fs::read(dest.join(name)).unwrap()).unwrap();
            assert_eq!(
                metadata["reconstruction"]["options"],
                serde_json::to_value(options).unwrap()
            );
            assert_eq!(
                metadata["reconstruction"]["source_size"],
                serde_json::json!({"w":8,"h":8})
            );
            assert_eq!(
                ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
                before
            );
            assert_eq!(history, state.history.undo_label().map(str::to_owned));
            assert_eq!(selection, state.session.selection);
            let mut collision =
                Review::rendered_options(&state, req, dest, sheet, encoding, options);
            collision.accept_replacements = true;
            collision.accept_warnings = true;
            assert!(!collision.ready());
        }
    }
    #[test]
    fn batch_preflight_consent_snapshot_and_history_are_preserved() {
        use crate::batch_export::Settings;
        use ankhimate_render::framing::BoundsMode;
        let mut state = AppState::default();
        let first = state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("first", 1.0));
        let second = state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("second", 2.0));
        state.dispatch(Box::new(
            ankhimate_document::commands::key_cmds::RenameAnimation::new(first, "renamed"),
        ));
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let history = state.history.undo_label().map(str::to_owned);
        assert!(history.is_some());
        let selection = state.session.selection.clone();
        let revision = state.revision;
        let settings = Settings {
            fps: 4,
            width: 8,
            height: 8,
            encoding: FrameEncoding::WebpLossless,
            framing: FrameOptions {
                bounds: BoundsMode::GlobalPixels,
                trim: true,
                padding: 1,
            },
            max_page: None,
        };
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("batch");
        let mut review = Review::batch(&state, &[second, first], settings, output.clone());
        assert!(!review.ready());
        assert!(
            review
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert!(!output.exists());
        review.accept_warnings = true;
        assert!(review.ready());
        state.session.export.png.encoding = FrameEncoding::Jpeg {
            background: [255; 3],
        };
        state.session.export.png.fps = 30;
        review
            .publish(&mut state, &egui::Context::default())
            .unwrap();
        state.session.export.png.batch_selected.clear();
        state.session.export.png.framing = FrameOptions::default();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while state.session.export.png.is_running() {
            state.session.export.png.poll();
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        assert!(state.session.export.png.message.as_ref().unwrap().is_ok());
        let manifest: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output.join("batch.json")).unwrap()).unwrap();
        assert_eq!(manifest["total_frames"], 12);
        for clip in manifest["clips"].as_array().unwrap() {
            let path = output.join(clip["metadata"].as_str().unwrap());
            let meta: serde_json::Value =
                serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            assert_eq!(meta["fps"], 4);
            assert_eq!(meta["filename_pattern"], "frame_%06d.webp");
            assert_eq!(
                meta["reconstruction"]["options"],
                serde_json::to_value(settings.framing).unwrap()
            );
        }
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
        assert_eq!(state.history.undo_label().map(str::to_owned), history);
        assert_eq!(state.session.selection, selection);
        assert_eq!(state.revision, revision);
        state.undo();
        assert_eq!(state.doc.animations[first].name, "first");
        state.redo();
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
    }
    #[test]
    fn batch_preflight_cancel_collision_and_stale_document_never_write() {
        use crate::batch_export::Settings;
        let mut state = AppState::default();
        let id = state
            .doc
            .animations
            .insert(ankhimate_core::animation::Animation::new("clip", 1.0));
        let settings = Settings {
            fps: 4,
            width: 8,
            height: 8,
            encoding: FrameEncoding::Png,
            framing: FrameOptions::default(),
            max_page: None,
        };
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("batch");
        let cancelled = Review::batch(&state, &[id], settings, output.clone());
        assert!(cancelled.ready());
        drop(cancelled);
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 0);
        let mut stale = Review::batch(&state, &[id], settings, output.clone());
        state.revision += 1;
        assert!(
            stale
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert!(!output.exists());
        let mut collision = Review::batch(&state, &[id], settings, output.clone());
        std::fs::create_dir(&output).unwrap();
        collision.accept_replacements = true;
        assert!(
            collision
                .publish(&mut state, &egui::Context::default())
                .is_err()
        );
        assert_eq!(std::fs::read_dir(&output).unwrap().count(), 0);
        let mut existing = Review::batch(&state, &[id], settings, output);
        existing.accept_replacements = true;
        existing.accept_warnings = true;
        assert!(!existing.ready());
    }
}
