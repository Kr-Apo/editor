//! Running an export (T-603b).
//!
//! Rendering is pure; writing is not. This module is where a bug destroys work
//! that was never ours to lose, so it holds three rules:
//!
//! 1. **Confine every path** to the output directory. See [`crate::template::confine`].
//! 2. **Prepare, then publish with rollback.** Render every file to memory and
//!    stage all bytes before replacing any destination. Ordinary failures restore
//!    previous files; failed restoration retains backups and reports their path.
//!    This is not filesystem-wide atomicity or recovery after a process crash.
//! 3. **Never delete orphans.** A stale-file sweep is the obvious convenience and it is
//!    refused: `output_dir` is user-chosen and can be pointed at a source tree
//!    by accident. Orphans are reported, and the user decides.

use crate::atlas::{self, Atlas};
use crate::context::{Context, ExportInfo};
use crate::preset::{Cadence, Preset};
use crate::template::{Engine, RenderedFile, TemplateError};
use ankhimate_core::assets::AssetDb;
use ankhimate_formats::schema::Project;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ExportError {
    Atlas(crate::atlas::AtlasError),
    Template(TemplateError),
    /// Two templates rendered to the same path — the later would silently
    /// overwrite the earlier, so the export stops instead.
    DuplicatePath {
        path: String,
        first: String,
        second: String,
    },
    Io {
        path: String,
        reason: String,
    },
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Atlas(e) => write!(f, "{e}"),
            ExportError::Template(e) => write!(f, "{e}"),
            ExportError::DuplicatePath {
                path,
                first,
                second,
            } => write!(
                f,
                "outputs '{first}' and '{second}' conflict at '{path}'; one would overwrite the \
                 other, so nothing was written"
            ),
            ExportError::Io { path, reason } => write!(f, "could not write '{path}': {reason}"),
        }
    }
}

impl std::error::Error for ExportError {}

impl From<TemplateError> for ExportError {
    fn from(e: TemplateError) -> Self {
        ExportError::Template(e)
    }
}

/// Everything an export produced, before any of it touches the disk.
///
/// Holding the whole set in memory is what makes rule 2 possible, and it is
/// affordable: these are text files and a handful of atlas pages.
#[derive(Debug, Default)]
pub struct Plan {
    pub files: Vec<RenderedFile>,
    /// `(relative path, PNG bytes)`.
    pub binaries: Vec<(String, Vec<u8>)>,
}

impl Plan {
    /// Every path this plan writes, in order.
    pub fn paths(&self) -> Vec<&str> {
        self.files
            .iter()
            .map(|f| f.path.as_str())
            .chain(self.binaries.iter().map(|(p, _)| p.as_str()))
            .collect()
    }

    /// Split the plan's paths into those that already exist and those that do
    /// not, so a user can be told what an export will replace before it does.
    pub fn survey(&self, output_dir: &Path) -> Survey {
        let mut created = Vec::new();
        let mut replaced = Vec::new();
        // Use exactly the portable paths the writer will publish, including
        // plugin plans containing Windows separators. Invalid plans are
        // diagnosed by `inspect`; surveying them must not follow unsafe paths.
        let paths: Vec<_> = self
            .paths()
            .into_iter()
            .filter_map(|path| crate::staged::validate_path(path).ok())
            .collect();
        for path in &paths {
            if output_dir.join(path).exists() {
                replaced.push(path.clone());
            } else {
                created.push(path.clone());
            }
        }
        Survey {
            created,
            replaced,
            orphans: orphans(
                output_dir,
                &paths.iter().map(String::as_str).collect::<Vec<_>>(),
            ),
        }
    }
}

/// What an export is about to do to a directory.
#[derive(Debug, Default, PartialEq)]
pub struct Survey {
    pub created: Vec<String>,
    pub replaced: Vec<String>,
    /// Files already in the output directory that this export does not write.
    ///
    /// Reported, never deleted — see the module note. A renamed animation leaves
    /// its old file behind, and only the user knows whether that matters.
    pub orphans: Vec<String>,
}

fn orphans(output_dir: &Path, writing: &[&str]) -> Vec<String> {
    if crate::staged::validate_directory(output_dir).is_err() {
        return Vec::new();
    }
    let expected: BTreeSet<String> = writing.iter().map(|p| survey_key(output_dir, p)).collect();
    let mut found = Vec::new();
    walk(output_dir, output_dir, &mut found);
    found
        .into_iter()
        .filter(|p| !expected.contains(&survey_key(output_dir, p)))
        .collect()
}

fn survey_key(_root: &Path, relative: &str) -> String {
    #[cfg(windows)]
    {
        let path = _root.join(relative);
        // Resolve ordinary existing files for case and 8.3 aliases, but never
        // follow an orphan's link/reparse point to another file or directory.
        if std::fs::symlink_metadata(&path).is_ok_and(|m| !crate::staged::is_link(&m))
            && let Ok(canonical) = std::fs::canonicalize(&path)
        {
            return canonical.to_string_lossy().to_lowercase();
        }
        relative.to_lowercase()
    }
    #[cfg(not(windows))]
    relative.to_string()
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let Ok(metadata) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.is_dir() && !crate::staged::is_link(&metadata) {
            walk(root, &path, out);
        } else if let Ok(rel) = path.strip_prefix(root) {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Render a preset into a [`Plan`] without writing anything.
///
/// This is also what the editor's live preview calls, so a preview is the real
/// export rather than an approximation of it — a preview that can disagree with
/// the export is worse than none.
pub fn plan(project: &Project, assets: &AssetDb, preset: &Preset) -> Result<Plan, ExportError> {
    plan_with(project, assets, preset, Images::Encoded)
}

/// Whether a plan carries its atlas pages as encoded PNG bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Images {
    /// Encode every page. Required before [`write()`] — a plan without bytes
    /// writes no images.
    Encoded,
    /// Name the pages but leave their bytes empty.
    ///
    /// For a caller that only displays the plan. PNG encoding a 2048² page costs
    /// tens of milliseconds, and the editor's preview does it only to show a
    /// count and then drop the bytes. The atlas is still **baked** — regions and
    /// UVs reach the context, so the rendered text is byte-identical to a real
    /// export. Only the compression is skipped.
    Named,
}

/// [`plan`], with control over whether atlas pages are encoded.
pub fn plan_with(
    project: &Project,
    assets: &AssetDb,
    preset: &Preset,
    images: Images,
) -> Result<Plan, ExportError> {
    if project.animations.iter().any(|a| {
        a.timelines
            .iter()
            .any(|t| matches!(t, ankhimate_formats::schema::Timeline::PathPosition { .. }))
    }) {
        return Err(TemplateError::Render{template:"Path Position compatibility".into(),reason:"Animated PathPosition is author/CPU-only; native v2 and templates have no supported channel. Use a rendered export.".into()}.into());
    }
    // JSON's Value conversion maps non-finite f32s to null. Reject them before
    // building the context, including headless/preview callers without preflight.
    // Other templates retain their existing contracts and diagnostic policy.
    if crate::preflight::Target::for_preset(preset) == crate::preflight::Target::Runtime
        && let Some(slot) = project.slots.iter().find(|slot| {
            slot.dark_color
                .is_some_and(|dark| dark.iter().any(|value| !value.is_finite()))
        })
    {
        return Err(TemplateError::Render {
            template: preset.templates[0].name.clone(),
            reason: format!(
                "slot {:?} has non-finite dark tint; export aborted",
                slot.name
            ),
        }
        .into());
    }
    let baked: Option<Atlas> = if preset.atlas.enabled {
        Some(atlas::bake(assets, &preset.atlas.settings).map_err(ExportError::Atlas)?)
    } else {
        None
    };

    let engine = Engine::new();
    let mut plan = Plan::default();
    // Path -> the template that claimed it, so a collision names both.
    let mut claimed: Vec<(String, String)> = Vec::new();

    for template in &preset.templates {
        let info = ExportInfo {
            output_dir: preset.output_dir.clone(),
            preset_name: preset.name.clone(),
            template_name: template.name.clone(),
            atlas_stem: preset.atlas.stem.clone(),
        };
        let context = Context::build(project, baked.as_ref(), info);
        if !context.binding_errors.is_empty() {
            return Err(TemplateError::Render {
                template: template.name.clone(),
                reason: context.binding_errors.join("; "),
            }
            .into());
        }

        let renders: Vec<serde_json::Value> = match template.per {
            Cadence::Once => vec![context.to_value()],
            Cadence::Animation => (0..context.animations.len())
                .map(|i| context.with_animation(i))
                .collect(),
        };

        for value in renders {
            let path_name = format!("{} (output path)", template.name);
            let rendered_path = engine.render(&path_name, &template.output_path, &value)?;
            let path = crate::staged::validate_path(&rendered_path)?;

            if let Some((_, first)) = claimed.iter().find(|(p, _)| p == &path) {
                return Err(ExportError::DuplicatePath {
                    path,
                    first: first.clone(),
                    second: template.name.clone(),
                });
            }
            claimed.push((path.clone(), template.name.clone()));

            let contents = engine.render(&template.name, &template.body, &value)?;
            if matches!(
                crate::preflight::Target::for_preset(preset),
                crate::preflight::Target::Runtime | crate::preflight::Target::LegacyRuntime
            ) {
                crate::context::verify_native_bindings(project, &contents).map_err(|reason| {
                    TemplateError::Render {
                        template: template.name.clone(),
                        reason,
                    }
                })?;
            }
            plan.files.push(RenderedFile { path, contents });
        }
    }

    if let Some(atlas) = &baked {
        for page in &atlas.pages {
            let name = atlas::page_filename(&preset.atlas.stem, page.index);
            let bytes = match images {
                Images::Encoded => atlas::encode_png(page).map_err(|e| ExportError::Io {
                    path: name.clone(),
                    reason: e.to_string(),
                })?,
                Images::Named => Vec::new(),
            };
            plan.binaries.push((name, bytes));
        }
    }

    if preset.copy_images {
        let mut names: Vec<&str> = assets.images.iter().map(|(_, a)| a.name.as_str()).collect();
        names.sort_unstable();
        for name in names {
            if let Some(id) = assets.by_name(name) {
                let asset = &assets.images[id];
                // Already-encoded source bytes, so `Named` saves only the clone.
                let bytes = match images {
                    Images::Encoded => asset.bytes.clone(),
                    Images::Named => Vec::new(),
                };
                plan.binaries
                    .push((format!("images/{}.png", asset.name), bytes));
            }
        }
    }

    crate::staged::validate_paths(&plan)?;
    Ok(plan)
}

/// Write a plan to disk.
///
/// Validate all text/image paths, stage all bytes on the destination filesystem,
/// then publish with backups and rollback. See `docs/export-safety.md` for the
/// guarantees, including Windows replacement and crash limitations.
pub fn write(plan: &Plan, output_dir: &Path) -> Result<(), ExportError> {
    write_data(plan, output_dir, None)
}

/// Publish through the same staged writer, refusing a changed replacement list.
/// Consent covers names, not unchanged contents. This is not a filesystem lock
/// against unrelated processes; see `docs/export-preflight.md`.
pub fn write_reviewed(
    plan: &Plan,
    output_dir: &Path,
    replacements: &[String],
) -> Result<(), ExportError> {
    write_data(plan, output_dir, Some(replacements))
}

fn write_data(
    plan: &Plan,
    output_dir: &Path,
    replacements: Option<&[String]>,
) -> Result<(), ExportError> {
    // A plan built with `Images::Named` carries page names and no bytes. Writing
    // it would produce zero-length PNGs beside perfectly good JSON — an export
    // that looks successful and is not, which is the failure this module exists
    // to prevent. Refuse before creating anything.
    if let Some((path, _)) = plan.binaries.iter().find(|(_, bytes)| bytes.is_empty()) {
        return Err(ExportError::Io {
            path: path.clone(),
            reason: "this plan was built for preview and carries no image data; \
                     re-plan with Images::Encoded before writing"
                .into(),
        });
    }

    match replacements {
        Some(replacements) => crate::staged::write_reviewed(plan, output_dir, replacements),
        None => crate::staged::write(plan, output_dir),
    }
}

/// Advisory read-only preflight. Not a permission probe, lock or reservation.
/// `write` remains authoritative and rechecks paths/content before staging.
pub fn inspect(plan: &Plan, output_dir: &Path) -> Result<(), ExportError> {
    crate::staged::inspect(plan, output_dir)
}

/// Stage PNGs one at a time without holding a whole sequence in memory.
/// Text files carry their final contents; binaries are names with empty bytes.
/// Existing destinations are refused. Cancellation during publication rolls
/// back like any other ordinary error; rollback itself cannot be cancelled.
/// See `docs/export-safety.md` for crash and concurrent-writer limitations.
pub fn write_generated_pngs(
    plan: &Plan,
    output_dir: &Path,
    generate: impl FnMut(usize, &str) -> Result<Vec<u8>, String>,
    cancelled: impl Fn() -> bool,
) -> Result<(), ExportError> {
    if plan
        .binaries
        .iter()
        .any(|(name, bytes)| !name.ends_with(".png") || !bytes.is_empty())
    {
        return Err(ExportError::Io {
            path: output_dir.display().to_string(),
            reason: "generated PNG plan must contain .png names with no precomputed bytes".into(),
        });
    }
    crate::staged::write_generated(plan, output_dir, generate, cancelled)
}

/// Generated PNG/JPEG/WebP images use the same staged transaction and refusal of
/// existing names. The callback's bytes must decode as the planned extension.
/// The original PNG-only entry point remains unchanged for historical callers.
pub fn write_generated_images(
    plan: &Plan,
    output_dir: &Path,
    mut generate: impl FnMut(usize, &str) -> Result<Vec<u8>, String>,
    cancelled: impl Fn() -> bool,
) -> Result<(), ExportError> {
    fn format(path: &str) -> Option<image::ImageFormat> {
        match Path::new(path).extension()?.to_str()? {
            "png" => Some(image::ImageFormat::Png),
            "jpg" => Some(image::ImageFormat::Jpeg),
            "webp" => Some(image::ImageFormat::WebP),
            _ => None,
        }
    }
    if plan
        .binaries
        .iter()
        .any(|(name, bytes)| format(name).is_none() || !bytes.is_empty())
    {
        return Err(ExportError::Io {
            path: output_dir.display().to_string(),
            reason: "generated image plan requires .png/.jpg/.webp names and no precomputed bytes"
                .into(),
        });
    }
    crate::staged::write_generated(
        plan,
        output_dir,
        |index, path| {
            let bytes = generate(index, path)?;
            let expected = format(path).expect("validated image suffix");
            let actual = image::guess_format(&bytes)
                .map_err(|e| format!("Invalid generated image {path}: {e}"))?;
            if actual != expected {
                return Err(format!("Image encoding does not match filename {path}"));
            }
            image::load_from_memory_with_format(&bytes, expected)
                .map_err(|e| format!("Invalid generated image {path}: {e}"))?;
            Ok(bytes)
        },
        cancelled,
    )
}

/// Render and write in one step.
pub fn export(
    project: &Project,
    assets: &AssetDb,
    preset: &Preset,
    output_dir: &Path,
) -> Result<Plan, ExportError> {
    let plan = plan(project, assets, preset)?;
    write(&plan, output_dir)?;
    Ok(plan)
}
