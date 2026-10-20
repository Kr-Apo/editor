//! Explicit, single-region provenance resolution over the exact-path planner.
use super::*;
use crate::snapshot::{Snapshot, snapshot};
use ankhimate_core::ids::{SkinId, SlotId};

#[derive(Clone, Debug)]
pub struct PathPreview {
    pub asset: String,
    pub old_path: String,
    pub new_path: String,
    pub skin: SkinId,
    pub slot: SlotId,
    pub attachment: String,
    pub old_size: (u32, u32),
    pub new_size: (u32, u32),
    pub old_region_size: (f32, f32),
    pub new_region_size: (f32, f32),
    pub same_pixels: bool,
    pub report: Report,
    /// Includes the mapping and every exact-path image update in this batch.
    pub affected_assets: Vec<String>,
}
impl PathPreview {
    pub fn lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("Old path: {}", self.old_path),
            format!("New path: {}", self.new_path),
            format!("Asset retained: {}; region: {}", self.asset, self.attachment),
            format!("Dimensions: {}×{} → {}×{}", self.old_size.0, self.old_size.1, self.new_size.0, self.new_size.1),
            format!("Authored region size: {}×{} → {}×{}", self.old_region_size.0, self.old_region_size.1, self.new_region_size.0, self.new_region_size.1),
            format!("Decoded pixels: {}", if self.same_pixels { "identical; original encoded bytes retained" } else { "different; replace bytes in the same asset" }),
            "Provenance changes to the selected new raw path. No asset or rig renaming.".into(),
            "Size policy: existing T-302 region width/height × new/old image dimensions; pivot, offset, rotation, scale and UV unchanged.".into(),
            "Bones, slots, skins, draw order, constraints, animations, curves and unrelated FFD/meshes are retained.".into(),
            format!("Whole atomic batch (asset names): {}", self.affected_assets.join(", ")),
            "No persistent Photoshop source/layer ID exists. Explicit selection asserts this source; unresolved other missing image provenance is refused.".into(),
        ];
        lines.extend(self.report.lines());
        lines
    }
}

#[derive(Clone)]
pub struct PathChange {
    pub preview: PathPreview,
    before: Snapshot,
    source: Vec<u8>,
    command: ReimportPsd,
}
impl PathChange {
    pub fn validate(&self, doc: &Document, bytes: &[u8]) -> Result<(), String> {
        if self.before != snapshot(doc) || self.source != bytes {
            return Err("PSD path plan is stale; prepare the mapping again".into());
        }
        Ok(())
    }
    /// Consumed only after the caller revalidates document identity, revision,
    /// source-file bytes, author-file preservation, work mode and session locks.
    pub fn into_command(self, doc: &Document, bytes: &[u8]) -> Result<ReimportPsd, String> {
        self.validate(doc, bytes)?;
        Ok(self.command)
    }
}

// Keep processing/inheritance tags, including blocked and unknown tags, literal.
// Moving through an untagged group is safe; changing tag semantics is not.
fn semantics(path: &str) -> Vec<String> {
    path.split('/')
        .filter_map(|part| {
            let tags: String = part.chars().skip_while(|c| *c != '[').collect();
            if part.starts_with(['$', '@']) {
                Some(part.to_owned())
            } else if tags.is_empty() {
                None
            } else {
                Some(tags)
            }
        })
        .collect()
}

impl ReimportPsd {
    pub fn prepare_path_change(
        doc: &Document,
        bytes: &[u8],
        asset: &str,
        new_path: &str,
    ) -> Result<PathChange, String> {
        let (_, initial) = Self::prepare(doc, bytes)?;
        if !initial.conflicts.is_empty() {
            return Err(initial.lines().join("\n"));
        }
        let old_path = doc
            .psd_layer_paths
            .get(asset)
            .ok_or("Old asset has no known PSD path")?;
        if !initial.layers.removed.contains(old_path)
            || !initial.layers.added.contains(&new_path.to_owned())
        {
            return Err(
                "Select one actually missing old path and one actually added new image path".into(),
            );
        }
        let mut claimed = BTreeSet::new();
        for (name, path) in &doc.psd_layer_paths {
            if !claimed.insert(path) || path == new_path {
                return Err("Duplicate/claimed PSD provenance; cannot distinguish sources".into());
            }
            if doc.assets.by_name(name).is_none() && !name.ends_with("_marker") {
                return Err("PSD provenance references a missing image asset".into());
            }
        }
        // There is no source-file UUID in the current schema. Never silently
        // combine this source with other missing, potentially separate imports.
        if initial.layers.removed.iter().any(|p| p != old_path) {
            return Err("Other missing image provenance makes PSD source affiliation ambiguous; resolve outside this bounded operation".into());
        }
        if semantics(old_path) != semantics(new_path) {
            return Err(
                "Path tags/legacy markers change semantics; no structural conversion allowed"
                    .into(),
            );
        }
        let nodes = psd::layer_tree(bytes).map_err(|e| e.to_string())?;
        if !nodes.iter().any(|n| n.path == new_path && !n.is_group)
            || nodes.iter().any(|n| {
                !n.visible && (n.path == new_path || new_path.starts_with(&format!("{}/", n.path)))
            })
        {
            return Err("New path must be a visible, non-flattened image layer".into());
        }
        let imported = psd::import(
            bytes,
            &ImportOptions {
                skip_hidden: false,
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        let incoming_names: Vec<_> = imported
            .layer_paths
            .iter()
            .filter(|(_, p)| *p == new_path)
            .map(|(n, _)| n)
            .collect();
        if incoming_names.len() != 1 {
            return Err("New image path is ambiguous or undecodable".into());
        }
        let incoming_name = incoming_names[0];
        // Reject inferred sequences and [mesh] requests too: temporary import
        // regions must not disguise a different authoring type.
        if !imported.trace_requests.is_empty()
            && imported
                .trace_requests
                .iter()
                .any(|r| r.attachment == *incoming_name)
        {
            return Err("New path requests mesh conversion".into());
        }
        let mut incoming_owners = 0;
        for skin in imported.skeleton.skins.values() {
            for a in skin.entries.values() {
                match a {
                    Attachment::Region(r)
                        if r.texture == *incoming_name && r.sequence.is_none() =>
                    {
                        incoming_owners += 1
                    }
                    Attachment::Region(r)
                        if r.sequence
                            .as_ref()
                            .is_some_and(|s| s.frames.contains(incoming_name)) =>
                    {
                        return Err("New path belongs to an image sequence".into());
                    }
                    Attachment::Mesh(m) if m.texture == *incoming_name => {
                        return Err("New path is a mesh".into());
                    }
                    _ => {}
                }
            }
        }
        if incoming_owners != 1 {
            return Err("New path is not one simple region image".into());
        }
        let ids: Vec<_> = doc
            .assets
            .images
            .iter()
            .filter(|(_, a)| a.name == asset)
            .map(|(id, _)| id)
            .collect();
        if ids.len() != 1 {
            return Err("Old image asset is absent or ambiguous".into());
        }
        let id = ids[0];
        let old = &doc.assets.images[id];
        let new_id = imported
            .assets
            .by_name(incoming_name)
            .ok_or("New image has no pixels")?;
        let new = &imported.assets.images[new_id];
        let decode = |a: &ankhimate_core::assets::ImageAsset| -> Result<image::RgbaImage, String> {
            let p = image::load_from_memory(&a.bytes)
                .map_err(|_| "Image bytes cannot be decoded")?
                .to_rgba8();
            if a.width == 0 || a.height == 0 || p.dimensions() != (a.width, a.height) {
                return Err("Image dimensions do not match decoded bytes".into());
            }
            Ok(p)
        };
        let same_pixels = decode(old)? == decode(new)?;
        let mut owners = Vec::new();
        for (skin, s) in &doc.skeleton.skins {
            for ((slot, name), a) in &s.entries {
                match a {
                    Attachment::Region(r) => {
                        if r.sequence
                            .as_ref()
                            .is_some_and(|s| s.frames.iter().any(|n| n == asset))
                        {
                            return Err("Image sequence is not eligible".into());
                        }
                        if r.texture == asset {
                            if r.sequence.is_some() {
                                return Err("Image sequence is not eligible".into());
                            }
                            if r.width <= 0.
                                || r.height <= 0.
                                || !r.width.is_finite()
                                || !r.height.is_finite()
                                || !r.pivot.is_finite()
                                || !r.local_offset.is_finite()
                                || !r.local_rotation.is_finite()
                                || !r.local_scale.is_finite()
                                || ![r.uv_rect.x, r.uv_rect.y, r.uv_rect.w, r.uv_rect.h]
                                    .iter()
                                    .all(|v| v.is_finite())
                            {
                                return Err("Region geometry is not representable".into());
                            }
                            owners.push((skin, *slot, name.clone()));
                        }
                    }
                    Attachment::Mesh(m)
                        if m.texture == asset
                            || m.sequence
                                .as_ref()
                                .is_some_and(|s| s.frames.iter().any(|n| n == asset)) =>
                    {
                        return Err("Mesh/linked image is not eligible".into());
                    }
                    _ => {}
                }
            }
        }
        if owners.len() != 1 {
            return Err("Image must have exactly one simple region owner".into());
        }
        let (skin, slot, attachment) = owners.remove(0);
        if !doc.skeleton.slots.contains_key(slot)
            || !doc
                .skeleton
                .bones
                .contains_key(doc.skeleton.slots[slot].bone)
        {
            return Err("Region slot/bone reference is invalid".into());
        }
        // UI flattening has no dedicated persisted type. Its importer-created
        // owner is a group-named bone/slot/attachment. Refuse this ambiguous
        // pattern too, even if its old path no longer exists in the source.
        let owner_bone = &doc.skeleton.bones[doc.skeleton.slots[slot].bone];
        let old_leaf = old_path.rsplit('/').next().unwrap_or(old_path);
        if owner_bone.name == ankhimate_formats::psd_tags::Tags::parse(old_leaf).name
            && doc.skeleton.slots[slot].name == format!("{}_slot", owner_bone.name)
            && attachment == old_leaf
        {
            return Err("Old owner may be a flattened group; source type is ambiguous".into());
        }
        for (sid, s) in &doc.skeleton.skins {
            if s.entries.contains_key(&(slot, attachment.clone())) && sid != skin {
                return Err("Skin override makes attachment ownership ambiguous".into());
            }
            for a in s.entries.values() {
                if let Attachment::Mesh(m) = a
                    && m.linked.as_ref().is_some_and(|l| {
                        l.slot == doc.skeleton.slots[slot].name && l.attachment == attachment
                    })
                {
                    return Err("Linked mesh references this attachment".into());
                }
            }
        }
        if doc.animations.values().any(|a| a.timelines.iter().any(|t|matches!(t, ankhimate_core::animation::Timeline::Deform{slot:s,attachment:n,..} if *s == slot && *n == attachment))) { return Err("FFD references this attachment; no path remapping".into()); }
        let mut next = doc.clone();
        next.psd_layer_paths
            .insert(asset.to_owned(), new_path.to_owned());
        // The existing planner is the ONLY pixel/dimension update algorithm.
        let (pixels, report) = Self::prepare(&next, bytes)?;
        if !report.conflicts.is_empty() {
            return Err(report.lines().join("\n"));
        }
        if let Some(pixels) = pixels {
            next = *pixels.other;
        }
        let region_size = |d: &Document| {
            let Attachment::Region(r) = d.skeleton.skins[skin]
                .get(slot, &attachment)
                .expect("validated region")
            else {
                unreachable!()
            };
            (r.width, r.height)
        };
        let mut affected_assets: Vec<_> = report
            .layers
            .changed
            .iter()
            .filter_map(|p| {
                next.psd_layer_paths
                    .iter()
                    .find(|(_, v)| *v == p)
                    .map(|(n, _)| n.clone())
            })
            .collect();
        affected_assets.push(asset.to_owned());
        affected_assets.sort();
        affected_assets.dedup();
        let old_region_size = region_size(doc);
        let new_region_size = region_size(&next);
        Ok(PathChange {
            preview: PathPreview {
                asset: asset.into(),
                old_path: old_path.clone(),
                new_path: new_path.into(),
                skin,
                slot,
                attachment,
                old_size: (old.width, old.height),
                new_size: (new.width, new.height),
                old_region_size,
                new_region_size,
                same_pixels,
                report,
                affected_assets,
            },
            before: snapshot(doc),
            source: bytes.to_vec(),
            command: Self {
                other: Box::new(next),
            },
        })
    }
}
