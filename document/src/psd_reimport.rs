//! Exact-path image refresh. Never rebuild the imported rig or delete missing art.
use crate::{Document, WorkMode, commands::EditCommand};
use ankhimate_core::attachment::Attachment;
use ankhimate_formats::psd::{self, ImportOptions, ReimportReport};
use std::collections::{BTreeMap, BTreeSet};

mod path_change;
pub use path_change::{PathChange, PathPreview};

#[derive(Clone, Debug, Default)]
pub struct Report {
    pub layers: ReimportReport,
    pub unchanged_paths: Vec<String>,
    pub conflicts: Vec<String>,
    pub notes: Vec<String>,
}
impl Report {
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![format!(
            "PSD image update: {} unchanged, {} changed, {} added, {} missing, {} conflicts",
            self.layers.unchanged,
            self.layers.changed.len(),
            self.layers.added.len(),
            self.layers.removed.len(),
            self.conflicts.len()
        )];
        for (label, paths) in [
            ("Unchanged", &self.unchanged_paths),
            ("Update pixels", &self.layers.changed),
            ("Added — import manually", &self.layers.added),
            ("Missing — existing content retained", &self.layers.removed),
            ("REFUSED", &self.conflicts),
        ] {
            out.extend(paths.iter().map(|path| format!("{label}: {path}")));
        }
        out.extend(self.notes.clone());
        out.push("Exact raw layer paths only (including tags); renamed/moved layers are added + missing. No bones, slots or layers are created/deleted. Source visibility is not applied. Region pivots and authored placement stay fixed; source layer movement does not reposition the rig.".into());
        out
    }
}

/// Prepared immediately before dispatch. Owns both states; redo never reads a file.
#[derive(Clone)]
pub struct ReimportPsd {
    other: Box<Document>,
}
impl ReimportPsd {
    pub fn prepare(doc: &Document, bytes: &[u8]) -> Result<(Option<Self>, Report), String> {
        let nodes = psd::layer_tree(bytes).map_err(|e| e.to_string())?;
        let mut report = Report::default();
        let mut paths = BTreeSet::new();
        for node in &nodes {
            if node.name.contains(['/', '\\']) || !paths.insert(node.path.clone()) {
                report
                    .conflicts
                    .push(format!("{}: ambiguous source path", node.path));
            }
        }
        let mut known = BTreeMap::new();
        for (asset, path) in &doc.psd_layer_paths {
            if doc.assets.by_name(asset).is_some()
                && known.insert(path.clone(), asset.clone()).is_some()
            {
                report.conflicts.push(format!(
                    "{path}: multiple document assets have this source path"
                ));
            }
        }
        if known.is_empty() {
            report
                .conflicts
                .push("No image-layer provenance in the current document".into());
        }
        if !report.conflicts.is_empty() {
            return Ok((None, report));
        }
        // Reuse the existing pixel extraction, tags and sequence rules. Its temporary
        // rig is never installed in the document. Catch the decoder's crop failures.
        let imported = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            psd::import(
                bytes,
                &ImportOptions {
                    skip_hidden: false,
                    ..Default::default()
                },
            )
        }))
        .map_err(|_| "PSD image extraction failed; document unchanged".to_string())?
        .map_err(|e| e.to_string())?;
        let incoming: BTreeMap<_, _> = imported
            .layer_paths
            .iter()
            .filter_map(|(name, path)| {
                imported
                    .assets
                    .by_name(name)
                    .map(|id| (path.clone(), &imported.assets.images[id]))
            })
            .collect();
        for path in known.keys() {
            if !incoming.contains_key(path) && nodes.iter().any(|n| &n.path == path) {
                report.conflicts.push(format!("{path}: source still present but no decodable image candidate; existing content retained"));
            }
        }
        for node in nodes
            .iter()
            .filter(|n| !n.is_group && !incoming.contains_key(&n.path))
        {
            report.notes.push(format!(
                "Not an image candidate (marker, empty or unsupported): {}",
                node.path
            ));
        }
        report.layers.added = incoming
            .keys()
            .filter(|p| !known.contains_key(*p))
            .cloned()
            .collect();
        report.layers.removed = known
            .keys()
            .filter(|p| !incoming.contains_key(*p) && !nodes.iter().any(|n| &n.path == *p))
            .cloned()
            .collect();
        let mut next = doc.clone();
        for (path, name) in known {
            let Some(new) = incoming.get(&path) else {
                continue;
            };
            let id = doc.assets.by_name(&name).expect("validated asset");
            let old = &doc.assets.images[id];
            if old.bytes == new.bytes && old.width == new.width && old.height == new.height {
                report.layers.unchanged += 1;
                report.unchanged_paths.push(path);
                continue;
            }
            // Bytes can differ solely because a PNG was re-encoded. Classify
            // changes by decoded RGBA and verify metadata before using its scale.
            let old_pixels = image::load_from_memory(&old.bytes).ok();
            let new_pixels = image::load_from_memory(&new.bytes).ok();
            let valid = old_pixels
                .as_ref()
                .is_some_and(|p| p.width() == old.width && p.height() == old.height)
                && new_pixels
                    .as_ref()
                    .is_some_and(|p| p.width() == new.width && p.height() == new.height);
            if !valid {
                report
                    .conflicts
                    .push(format!("{path}: image bytes/dimensions cannot be verified"));
                continue;
            }
            if old_pixels.unwrap().to_rgba8() == new_pixels.unwrap().to_rgba8() {
                report.layers.unchanged += 1;
                report.unchanged_paths.push(path);
                continue;
            }
            report.layers.changed.push(path.clone());
            if nodes.iter().any(|n| {
                !n.visible && (n.path == path || path.starts_with(&format!("{}/", n.path)))
            }) {
                report
                    .conflicts
                    .push(format!("{path}: hidden layer/group; existing art retained"));
                continue;
            }
            if !nodes.iter().any(|n| n.path == path && !n.is_group) {
                report.conflicts.push(format!(
                    "{path}: flattened/group image cannot be refreshed safely"
                ));
                continue;
            }
            let mut owners = vec![];
            let mut unsafe_ref = false;
            for (skin_id, skin) in doc.skeleton.skins.iter() {
                for ((slot, attachment), value) in &skin.entries {
                    match value {
                        Attachment::Region(r) => {
                            if r.sequence
                                .as_ref()
                                .is_some_and(|s| s.frames.contains(&name))
                            {
                                unsafe_ref = true;
                            }
                            if r.texture == name {
                                if r.sequence.is_some() {
                                    unsafe_ref = true;
                                }
                                owners.push((skin_id, *slot, attachment.clone()));
                            }
                        }
                        Attachment::Mesh(m)
                            if m.texture == name
                                || m.sequence
                                    .as_ref()
                                    .is_some_and(|s| s.frames.contains(&name)) =>
                        {
                            unsafe_ref = true
                        }
                        _ => {}
                    }
                }
            }
            if unsafe_ref
                || owners.len() != 1
                || old.width == 0
                || old.height == 0
                || new.width == 0
                || new.height == 0
            {
                report.conflicts.push(format!("{path}: mesh, sequence, shared/unreferenced image or invalid dimensions; no automatic update"));
                continue;
            }
            let (skin, slot, attachment) = &owners[0];
            if doc.animations.values().any(|a|a.timelines.iter().any(|t| matches!(t,ankhimate_core::animation::Timeline::Deform{slot:s,attachment:n,..} if s==slot && n==attachment))) {
                report.conflicts.push(format!("{path}: FFD references this attachment; no automatic update"));continue;
            }
            let Attachment::Region(region) = next.skeleton.skins[*skin]
                .entries
                .get_mut(&(*slot, attachment.clone()))
                .expect("region owner")
            else {
                unreachable!()
            };
            let width = region.width * (new.width as f32 / old.width as f32);
            let height = region.height * (new.height as f32 / old.height as f32);
            if !width.is_finite() || !height.is_finite() || width <= 0. || height <= 0. {
                report
                    .conflicts
                    .push(format!("{path}: nonrepresentable region dimensions"));
                continue;
            }
            region.width = width;
            region.height = height;
            let target = &mut next.assets.images[id];
            target.bytes.clone_from(&new.bytes);
            target.width = new.width;
            target.height = new.height;
        }
        if !report.conflicts.is_empty() || report.layers.changed.is_empty() {
            return Ok((None, report));
        }
        Ok((
            Some(Self {
                other: Box::new(next),
            }),
            report,
        ))
    }
}
impl EditCommand for ReimportPsd {
    fn apply(&mut self, doc: &mut Document) {
        std::mem::swap(doc, &mut self.other);
    }
    fn revert(&mut self, doc: &mut Document) {
        self.apply(doc);
    }
    fn label(&self) -> &str {
        "Reimport PSD images"
    }
    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Setup)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
