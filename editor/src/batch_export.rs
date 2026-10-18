//! Batch orchestration only: each clip uses the unchanged individual export API.
use ankhimate_core::{animation::Animation, ids::AnimationId};
use ankhimate_document::Document;
use ankhimate_export::{
    preflight::{self, Report, Severity, Target},
    run::{self, Plan},
    template::RenderedFile,
};
use ankhimate_render::{
    encoding::FrameEncoding, framing::FrameOptions, sequence::SequenceRequest, spritesheet::Layout,
};
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub encoding: FrameEncoding,
    pub framing: FrameOptions,
    pub max_page: Option<u32>,
}
impl Settings {
    pub fn request(self, name: &str) -> SequenceRequest {
        SequenceRequest {
            animation: name.into(),
            fps: self.fps,
            width: self.width,
            height: self.height,
        }
    }
    /// Reuse SequenceRequest's duration rounding without cloning asset bytes per UI repaint.
    pub fn count(self, animation: &Animation) -> Result<u32, String> {
        self.framing
            .validate(self.width, self.height)
            .map_err(|e| e.to_string())?;
        let mut timing = Document::new();
        timing
            .animations
            .insert(Animation::new(&animation.name, animation.duration));
        let count = self
            .request(&animation.name)
            .frame_count(&timing)
            .map_err(|e| e.to_string())?;
        if let Some(page) = self.max_page {
            Layout::page_count(count, self.width, self.height, page, self.framing.padding)
                .map_err(|e| e.to_string())?;
        }
        Ok(count)
    }
}
#[derive(Debug)]
pub struct Clip {
    pub id: AnimationId,
    pub source_index: usize,
    pub name: String,
    pub folder: String,
    pub frame_count: u32,
    pub duration: f32,
}
#[derive(Debug)]
pub struct Batch {
    pub settings: Settings,
    pub clips: Vec<Clip>,
    pub plan: Plan,
    pub total_frames: u64,
}
fn folder(index: usize, name: &str) -> String {
    let prefix = format!("{:04}-", index + 1);
    let budget = 80 - prefix.encode_utf16().count();
    let mut tail = String::new();
    let mut units = 0;
    for c in name.chars() {
        let c = if c.is_control() || "/\\<>:\"|?*".contains(c) {
            '_'
        } else {
            c
        };
        if units + c.len_utf16() > budget {
            break;
        }
        units += c.len_utf16();
        tail.push(c);
    }
    let tail = tail.trim_matches([' ', '.']);
    format!(
        "{prefix}{}",
        if tail.is_empty() { "animation" } else { tail }
    )
}
/// Select one clip by ID on a private snapshot; author data is never altered.
fn focused(doc: &Document, id: AnimationId) -> Result<Document, String> {
    if !doc.animations.contains_key(id) {
        return Err("Selected animation no longer exists".into());
    }
    let mut snapshot = doc.clone();
    snapshot.animations.retain(|key, _| key == id);
    Ok(snapshot)
}

impl Batch {
    pub fn new(
        doc: &Document,
        selected: &[AnimationId],
        settings: Settings,
    ) -> Result<Self, String> {
        if settings.max_page.is_some() && settings.encoding != FrameEncoding::Png {
            return Err("Spritesheet batches require PNG encoding".into());
        }
        if selected.is_empty() {
            return Err("Select at least one animation".into());
        }
        for (i, id) in selected.iter().enumerate() {
            if !doc.animations.contains_key(*id) || selected[..i].contains(id) {
                return Err("Missing or duplicate animation selection".into());
            }
        }
        let mut clips = vec![];
        let mut plan = Plan::default();
        let mut total_frames = 0u64;
        for (index, (id, animation)) in doc
            .animations
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| selected.contains(id))
        {
            let count = settings.count(animation)?;
            let snapshot = focused(doc, id)?;
            let req = settings.request(&animation.name);
            let individual = if let Some(page) = settings.max_page {
                crate::spritesheet_export::output_plan(
                    &snapshot,
                    &req,
                    page,
                    settings.framing.padding,
                )?
                .1
            } else {
                crate::png_export::output_plan_encoded(&snapshot, &req, settings.encoding)?
            };
            let name = folder(index, &animation.name);
            for f in individual.files {
                plan.files.push(RenderedFile {
                    path: format!("{name}/{}", f.path),
                    contents: f.contents,
                });
            }
            for (path, bytes) in individual.binaries {
                plan.binaries.push((format!("{name}/{path}"), bytes));
            }
            clips.push(Clip {
                id,
                source_index: index,
                name: animation.name.clone(),
                folder: name,
                frame_count: count,
                duration: animation.duration,
            });
            total_frames += u64::from(count);
        }
        let manifest = serde_json::json!({"batch_version":1,"kind":if settings.max_page.is_some(){"spritesheet_png"}else{"image_sequence"},"total_frames":total_frames,
            "clips":clips.iter().map(|c|serde_json::json!({"source_index":c.source_index,"animation":c.name,"folder":c.folder,"fps":settings.fps,"duration":c.duration,"frame_count":c.frame_count,
                "metadata":format!("{}/{}",c.folder,if settings.max_page.is_some(){"spritesheet.json"}else{"sequence.json"})})).collect::<Vec<_>>()});
        plan.files.push(RenderedFile {
            path: "batch.json".into(),
            contents: serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?,
        });
        Ok(Self {
            settings,
            clips,
            plan,
            total_frames,
        })
    }
    pub fn diagnostics(&self, doc: &Document) -> Report {
        let mut report = Report::default();
        for clip in &self.clips {
            match focused(doc, clip.id) {
                Ok(snapshot) => {
                    let schema = ankhimate_formats::convert::to_schema(&snapshot.as_project_ref());
                    let mut r = preflight::inspect(
                        &schema,
                        &snapshot.assets,
                        Target::Rendered,
                        Some(&clip.name),
                    );
                    for d in &mut r.items {
                        d.subject = format!("{} / {}", clip.folder, d.subject);
                    }
                    report.items.extend(r.items);
                }
                Err(e) => report.push(Severity::Error, "batch-selection", &clip.name, e),
            }
        }
        if self.settings.framing.crops()
            || (self.settings.max_page.is_none() && self.settings.framing.padding > 0)
        {
            report.push(Severity::Warning,"render-reconstruction","Batch","Each clip requires reconstruction metadata v1. The current Phaser sheet example does not support cropped sheets; fixed source-sized cells do not reduce page count.");
        }
        if let FrameEncoding::Jpeg { background } = self.settings.encoding
            && self.settings.max_page.is_none()
        {
            report.push(Severity::Info,"jpeg-background","Batch",format!("All clips: JPEG quality 95, opaque RGB background {background:?}; transparency is composited before lossy encoding."));
        }
        report
    }
    pub fn export(
        &self,
        doc: &Document,
        destination: &Path,
        cancel: &AtomicBool,
        mut progress: impl FnMut(&str, u64, u64),
    ) -> Result<u64, String> {
        let report = self.diagnostics(doc);
        if report.has_errors() {
            return Err(report.errors());
        }
        ankhimate_export::batch::write_new_directory(
            &self.plan,
            destination,
            |stage| {
                let mut done = 0u64;
                for clip in &self.clips {
                    if cancel.load(Ordering::Relaxed) {
                        return Err("batch cancelled".into());
                    }
                    let snapshot = focused(doc, clip.id)?;
                    let request = self.settings.request(&clip.name);
                    let output = stage.join(&clip.folder);
                    let report = |phase: &'static str, n: u32, _: u32| {
                        let amount = if phase == "Rendering" {
                            done + u64::from(n).min(u64::from(clip.frame_count))
                        } else {
                            done
                        };
                        progress(
                            &format!("{} — {phase}", clip.name),
                            amount,
                            self.total_frames,
                        );
                    };
                    let count = if let Some(page) = self.settings.max_page {
                        crate::spritesheet_export::export_options(
                            &snapshot,
                            &request,
                            page,
                            self.settings.framing,
                            &output,
                            cancel,
                            report,
                        )
                    } else {
                        crate::png_export::export_options(
                            &snapshot,
                            &request,
                            self.settings.encoding,
                            self.settings.framing,
                            &output,
                            cancel,
                            report,
                        )
                    }?;
                    if count != clip.frame_count {
                        return Err("clip frame count changed since preflight".into());
                    }
                    done += u64::from(count);
                    progress("Clip complete", done, self.total_frames);
                }
                if cancel.load(Ordering::Relaxed) {
                    return Err("batch cancelled".into());
                }
                let manifest = Plan {
                    files: vec![RenderedFile {
                        path: "batch.json".into(),
                        contents: self
                            .plan
                            .files
                            .iter()
                            .find(|f| f.path == "batch.json")
                            .unwrap()
                            .contents
                            .clone(),
                    }],
                    binaries: vec![],
                };
                run::write(&manifest, stage).map_err(|e| e.to_string())
            },
            || cancel.load(Ordering::Relaxed),
        )
        .map_err(|e| e.to_string())?;
        progress("Complete", self.total_frames, self.total_frames);
        Ok(self.total_frames)
    }
}
