//! Reviewed, bounded transfer. Live publication is one existing native swap.
use crate::{
    Document,
    clipboard::BoneClip,
    commands::{
        EditCommand, asset_cmds::AddAssets, bone_cmds::PasteBones, document_cmds::ReplaceDocument,
    },
    snapshot::{Snapshot, snapshot},
};
use ankhimate_core::{
    attachment::Attachment,
    constraints::Constraint,
    ids::{BoneId, ConstraintId, SlotId},
    skeleton::unique_name,
};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_BONES: usize = 512;
pub const MAX_SLOTS: usize = 1024;
pub const MAX_IMAGE_BYTES: usize = 64 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetMapping {
    pub source: String,
    pub destination: String,
    pub reused: bool,
    pub bytes: usize,
}
#[derive(Clone, Debug, Default)]
pub struct Summary {
    pub bones: Vec<(String, String)>,
    pub slots: Vec<(String, String)>,
    pub constraints: Vec<(String, String)>,
    pub clips: Vec<(String, String)>,
    pub images: Vec<AssetMapping>,
    pub attachments: usize,
    pub keys: usize,
    pub notes: Vec<String>,
}
pub struct Plan {
    before: Snapshot,
    after: Document,
    pub summary: Summary,
    pub root: BoneId,
}
fn integrity(doc: &Document) -> Result<(), String> {
    // The exact typed projection guard used by T813 Save, without any writes.
    // Global diagnostics are complementary, not a transaction authorization.
    let project = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let value = serde_json::to_value(project).map_err(|e| e.to_string())?;
    let _: ankhimate_formats::schema::Project =
        serde_json::from_value(value).map_err(|e| format!("Author serialization guard: {e}"))?;
    let mut asset_names = BTreeSet::new();
    if doc
        .assets
        .images
        .values()
        .any(|a| a.name.is_empty() || !asset_names.insert(a.name.as_str()))
    {
        return Err(
            "Image logical names must be nonempty and unique; ambiguous assets cannot transfer"
                .into(),
        );
    }
    let issues = crate::diagnostics::inspect(doc);
    let errors: Vec<_> = issues
        .iter()
        .filter(|i| i.severity == crate::diagnostics::Severity::Error)
        .map(|i| format!("{} / {}: {}", i.code, i.subject, i.message))
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().take(8).collect::<Vec<_>>().join("\n"))
    }
}
fn slots_for(doc: &Document, bones: &BTreeSet<BoneId>) -> BTreeSet<SlotId> {
    doc.skeleton
        .slots
        .iter()
        .filter(|(_, s)| bones.contains(&s.bone))
        .map(|(id, _)| id)
        .collect()
}
fn affected_track(
    t: &ankhimate_core::animation::Timeline,
    slots: &BTreeSet<SlotId>,
    constraints: &BTreeSet<ConstraintId>,
) -> bool {
    use ankhimate_core::animation::Timeline as T;
    match t {
        T::SlotColor { slot, .. }
        | T::SlotVisible { slot, .. }
        | T::SlotAttachment { slot, .. }
        | T::Deform { slot, .. } => slots.contains(slot),
        T::DrawOrder { keys } => keys
            .iter()
            .flat_map(|k| &k.value)
            .any(|(s, _)| slots.contains(s)),
        T::IkMix { constraint, .. }
        | T::IkSoftness { constraint, .. }
        | T::IkBendDirection { constraint, .. }
        | T::TransformConstraintMix { constraint, .. } => constraints.contains(constraint),
        _ => false,
    }
}
impl Plan {
    pub fn prepare(
        source: &Document,
        destination: &Document,
        root: BoneId,
        parent: Option<BoneId>,
    ) -> Result<Self, String> {
        Self::prepare_checked(source, destination, root, parent, integrity)
    }
    fn prepare_checked(
        source: &Document,
        destination: &Document,
        root: BoneId,
        parent: Option<BoneId>,
        validate: impl FnOnce(&Document) -> Result<(), String>,
    ) -> Result<Self, String> {
        integrity(source).map_err(|e| format!("Source integrity: {e}"))?;
        integrity(destination).map_err(|e| format!("Destination integrity: {e}"))?;
        let s = &source.skeleton;
        if !s.bones.contains_key(root) {
            return Err("Source root is absent".into());
        }
        if parent.is_some_and(|b| !destination.skeleton.bones.contains_key(b)) {
            return Err("Destination parent is absent".into());
        }
        let source_skin = s
            .skins
            .get(s.default_skin)
            .ok_or("Source default skin is absent")?;
        let dest_skin = destination
            .skeleton
            .skins
            .get(destination.skeleton.default_skin)
            .ok_or("Destination default skin is absent")?;
        let ids: Vec<_> = s
            .update_order
            .iter()
            .copied()
            .filter(|id| *id == root || s.is_descendant(*id, root))
            .collect();
        if ids.is_empty() || ids.len() > MAX_BONES {
            return Err(format!("Transfer supports 1–{MAX_BONES} bones"));
        }
        let bones: BTreeSet<_> = ids.iter().copied().collect();
        let slots = slots_for(source, &bones);
        if slots.len() > MAX_SLOTS {
            return Err(format!("Transfer supports at most {MAX_SLOTS} slots"));
        }
        let mut internal = BTreeSet::new();
        for (id, c) in &s.constraints {
            let touches = c.affected_bones().iter().any(|b| bones.contains(b))
                || c.target().is_some_and(|b| bones.contains(&b));
            if !touches {
                continue;
            }
            if !matches!(c, Constraint::Ik(_) | Constraint::Transform(_)) {
                return Err(format!(
                    "Unsupported path/physics constraint `{}`",
                    c.name()
                ));
            }
            if c.affected_bones().is_empty()
                || c.affected_bones().iter().any(|b| !bones.contains(b))
                || c.target().is_some_and(|b| !bones.contains(&b))
            {
                return Err(format!("External constraint dependency `{}`", c.name()));
            }
            if s.constraint_order.iter().filter(|c| **c == id).count() != 1 {
                return Err("Constraint order is not representable safely".into());
            }
            internal.insert(id);
        }
        for (id, skin) in &s.skins {
            if skin.bones.iter().any(|b| bones.contains(b))
                || skin.constraints.iter().any(|c| internal.contains(c))
            {
                return Err("Conditional skin gating is outside region transfer v1".into());
            }
            for ((slot, _), a) in &skin.entries {
                if !slots.contains(slot) {
                    continue;
                }
                if id != s.default_skin {
                    return Err("Alternate skin overrides are outside region transfer v1".into());
                }
                if !matches!(a, Attachment::Region(_)) {
                    return Err("Only region attachments (including image sequences) can transfer; mesh/linked/FFD/clipping/path are refused".into());
                }
            }
        }
        for a in source.animations.values() {
            if a.timelines
                .iter()
                .any(|t| affected_track(t, &slots, &internal))
            {
                return Err(format!(
                    "Clip `{}` has unsupported slot/FFD/constraint/draw-order tracks for this subtree",
                    a.name
                ));
            }
            if a.timelines
                .iter()
                .any(|t| t.bone().is_some_and(|b| bones.contains(&b)))
                && !a.events.is_empty()
            {
                return Err(format!(
                    "Clip `{}` has global events; v1 refuses to drop them",
                    a.name
                ));
            }
        }
        let mut clip = BoneClip::capture(source, ids.clone());
        if !clip.dropped_constraints.is_empty() {
            return Err("External constraints cannot be dropped during transfer".into());
        }
        let mut summary = Summary::default();
        summary.notes.push("Local setup transforms and key offsets are retained. Source-world placement under a different ancestor is NOT preserved.".into());
        summary.notes.push("Scope: chosen subtree, source default-skin regions/sequences, internal IK/transform and bone tracks. Unrelated objects, editor folders/export presets and PSD provenance are not imported.".into());
        summary.notes.push(format!("Skin mapping: {} → {}. Imported slots append after existing slots in their source relative setup draw order.",source_skin.name,dest_skin.name));
        let mut effective = s.draw_order.clone();
        for id in s.slots.keys() {
            if !effective.contains(&id) {
                effective.push(id);
            }
        }
        clip.slots.sort_by_key(|c| {
            effective
                .iter()
                .position(|id| s.slots.get(*id).is_some_and(|sl| sl.name == c.slot.name))
                .unwrap_or(usize::MAX)
        });
        clip.constraints.sort_by_key(|c| {
            s.constraint_order
                .iter()
                .position(|id| s.constraints.get(*id).is_some_and(|cc| cc.name() == c.name))
                .unwrap_or(usize::MAX)
        });
        let mut used = BTreeSet::new();
        for c in &clip.slots {
            for (_, _, a) in &c.entries {
                if let Attachment::Region(r) = a {
                    used.insert(r.texture.clone());
                    if let Some(seq) = &r.sequence {
                        used.extend(seq.frames.iter().cloned());
                    }
                }
            }
        }
        let mut staged = destination.clone();
        let mut names = BTreeMap::new();
        let mut total = 0usize;
        for name in used {
            let id = source
                .assets
                .by_name(&name)
                .ok_or_else(|| format!("Required source image `{name}` is missing"))?;
            let image = source
                .assets
                .get(id)
                .ok_or("Required source image ID is missing")?;
            if image.bytes.is_empty() {
                return Err(format!("Required source image `{name}` has no bytes"));
            }
            total = total
                .checked_add(image.bytes.len())
                .ok_or("Image size overflow")?;
            if total > MAX_IMAGE_BYTES {
                return Err("Transfer image budget exceeds 64 MiB of encoded bytes".into());
            }
            let mut identical: Vec<_> = staged
                .assets
                .images
                .values()
                .filter(|a| {
                    a.bytes == image.bytes && a.width == image.width && a.height == image.height
                })
                .map(|a| a.name.clone())
                .collect();
            identical.sort();
            let reused = !identical.is_empty();
            let dest = if let Some(n) = identical.first() {
                n.clone()
            } else {
                let mut asset = image.clone();
                asset.name = unique_name(
                    &name,
                    staged.assets.images.values().map(|a| a.name.as_str()),
                );
                asset.source_path = None;
                let result = asset.name.clone();
                AddAssets::new(vec![asset]).apply(&mut staged);
                result
            };
            summary.images.push(AssetMapping {
                source: name.clone(),
                destination: dest.clone(),
                reused,
                bytes: image.bytes.len(),
            });
            names.insert(name, dest);
        }
        for c in &mut clip.slots {
            for (skin, _, a) in &mut c.entries {
                *skin = dest_skin.name.clone();
                if let Attachment::Region(r) = a {
                    r.texture = names[&r.texture].clone();
                    if let Some(seq) = &mut r.sequence {
                        for frame in &mut seq.frames {
                            *frame = names[frame].clone();
                        }
                    }
                }
            }
        }
        let mut reserved: Vec<_> = destination
            .animations
            .values()
            .map(|a| a.name.clone())
            .collect();
        for animation in &mut clip.animations {
            let old = animation.name.clone();
            animation.name = unique_name(&old, reserved.iter().map(String::as_str));
            reserved.push(animation.name.clone());
            summary.clips.push((old, animation.name.clone()));
            summary.keys += animation
                .timelines
                .iter()
                .map(|(_, t)| {
                    use ankhimate_core::animation::Timeline as T;
                    match t {
                        T::BoneTranslate { keys, .. }
                        | T::BoneRotate { keys, .. }
                        | T::BoneScale { keys, .. }
                        | T::BoneShear { keys, .. } => keys.len(),
                        _ => 0,
                    }
                })
                .sum::<usize>();
        }
        summary.attachments = clip.slots.iter().map(|s| s.entries.len()).sum();
        let old_bone_names: Vec<_> = clip.bones.iter().map(|b| b.bone.name.clone()).collect();
        let old_slot_names: Vec<_> = clip.slots.iter().map(|s| s.slot.name.clone()).collect();
        let old_constraint_names: Vec<_> =
            clip.constraints.iter().map(|c| c.name.clone()).collect();
        let mut paste = PasteBones::new(clip, parent);
        paste.apply(&mut staged);
        if paste.created_bone_ids().len() != ids.len()
            || paste.created_slot_ids().len() != slots.len()
            || paste.created_constraint_ids().len() != internal.len()
        {
            return Err("Native paste produced incomplete result maps".into());
        }
        for (old, id) in old_bone_names.iter().zip(paste.created_bone_ids()) {
            summary
                .bones
                .push((old.clone(), staged.skeleton.bones[*id].name.clone()));
        }
        for (old, id) in old_slot_names.iter().zip(paste.created_slot_ids()) {
            summary
                .slots
                .push((old.clone(), staged.skeleton.slots[*id].name.clone()));
        }
        for (old, id) in old_constraint_names
            .iter()
            .zip(paste.created_constraint_ids())
        {
            summary
                .constraints
                .push((old.clone(), staged.skeleton.constraints[*id].name().into()));
        }
        let mapping: BTreeMap<_, _> = ids
            .iter()
            .copied()
            .zip(paste.created_bone_ids().iter().copied())
            .collect();
        for (old, new) in &summary.clips {
            let source = source
                .animations
                .values()
                .find(|a| &a.name == old)
                .ok_or("Source clip lost")?;
            let target = staged
                .animations
                .values_mut()
                .find(|a| &a.name == new)
                .ok_or("Transferred clip lost")?;
            target.looping = source.looping;
            target.markers = source.markers.clone();
            target.bone_offsets = source
                .bone_offsets
                .iter()
                .filter_map(|(id, t)| mapping.get(id).map(|b| (*b, *t)))
                .collect();
        }
        validate(&staged)
            .map_err(|e| format!("Prepared result rejected before publication: {e}"))?;
        let root = *paste.created_bone_ids().first().ok_or("No root created")?;
        Ok(Self {
            before: snapshot(destination),
            after: staged,
            summary,
            root,
        })
    }
    pub fn matches(&self, destination: &Document) -> bool {
        self.before == snapshot(destination)
    }
    pub fn into_command(self, destination: &Document) -> Result<ReplaceDocument, String> {
        if !self.matches(destination) {
            return Err("Destination changed; prepare transfer again".into());
        }
        Ok(ReplaceDocument::new(self.after))
    }
}
#[cfg(test)]
mod tests;
