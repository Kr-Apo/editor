//! Fold a PSD import into the document as one undoable step (T-302).
//!
//! One command, not one per bone. An import that undoes in ninety steps is an
//! import nobody dares run twice, and "I imported the wrong file" is exactly the
//! mistake this has to make cheap.

use super::EditCommand;
use crate::WorkMode;
use crate::doc::Document;
use ankhimate_core::attachment::Attachment;
use ankhimate_core::ids::BoneId;
use ankhimate_formats::psd::PsdImport;
use std::collections::HashMap;

pub struct ImportPsd {
    imported: Option<PsdImport>,
    /// Replace the document outright, rather than merging under a new bone.
    replace: bool,
    name: String,
    /// The inactive document, exchanged on undo/redo like ReplaceDocument.
    ///
    /// Coarse on purpose. A merge touches bones, slots, skins, assets and the
    /// draw order at once, and reconstructing the inverse of all of that is far
    /// more code — and far more places to be subtly wrong — than retaining one
    /// owned document copy, including pixels and the original entity IDs.
    other: Option<Box<Document>>,
}

impl ImportPsd {
    pub fn new(imported: PsdImport, replace: bool, name: impl Into<String>) -> Self {
        Self {
            imported: Some(imported),
            replace,
            name: name.into(),
            other: None,
        }
    }
}

impl EditCommand for ImportPsd {
    fn replaces_rig(&self) -> bool {
        self.replace
    }

    fn apply(&mut self, doc: &mut Document) {
        if let Some(other) = self.other.as_mut() {
            std::mem::swap(doc, other);
            return;
        }
        let Some(imported) = self.imported.take() else {
            return;
        };
        self.other = Some(Box::new(doc.clone()));

        if self.replace {
            doc.skeleton = imported.skeleton;
            doc.assets = imported.assets;
            doc.psd_layer_paths = imported.layer_paths;
            // A PSD has no animation clips. Keeping the previous rig's keys
            // would point at unrelated (or missing) bones in this skeleton.
            doc.animations.clear();
            return;
        }

        // Merge: the imported rig arrives under a bone of its own so it can be
        // moved as a unit, and so two imports cannot collide at the root.
        let anchor = doc.skeleton.add_bone(ankhimate_core::skeleton::Bone {
            name: ankhimate_core::skeleton::unique_name(
                &self.name,
                doc.skeleton.bones.iter().map(|(_, b)| b.name.as_str()),
            ),
            parent: None,
            length: 1.0,
            local_transform: Default::default(),
            inherit: Default::default(),
            color: ankhimate_core::skeleton::Bone::default_color(),
        });

        let mut bone_map: HashMap<BoneId, BoneId> = HashMap::new();
        // Imported bones come out parent-before-child, so one pass maps them.
        for id in imported.skeleton.update_order.iter() {
            let Some(bone) = imported.skeleton.bones.get(*id) else {
                continue;
            };
            let parent = match bone.parent {
                Some(p) => bone_map.get(&p).copied().unwrap_or(anchor),
                None => anchor,
            };
            let new = doc.skeleton.add_bone(ankhimate_core::skeleton::Bone {
                name: ankhimate_core::skeleton::unique_name(
                    &bone.name,
                    doc.skeleton.bones.iter().map(|(_, b)| b.name.as_str()),
                ),
                parent: Some(parent),
                ..bone.clone()
            });
            bone_map.insert(*id, new);
        }

        let mut asset_names = HashMap::new();
        for asset in imported.assets.images.values() {
            let id = doc.assets.add(asset.clone());
            asset_names.insert(asset.name.clone(), doc.assets.images[id].name.clone());
        }

        let mut slot_map = HashMap::new();
        for slot_id in imported.skeleton.draw_order.iter() {
            let Some(slot) = imported.skeleton.slots.get(*slot_id) else {
                continue;
            };
            let Some(&bone) = bone_map.get(&slot.bone) else {
                continue;
            };
            let new = doc.skeleton.add_slot(ankhimate_core::slot::Slot {
                bone,
                name: ankhimate_core::skeleton::unique_name(
                    &slot.name,
                    doc.skeleton.slots.iter().map(|(_, s)| s.name.as_str()),
                ),
                ..slot.clone()
            });
            slot_map.insert(*slot_id, new);
        }

        let default_skin = doc.skeleton.default_skin;
        for (source_skin_id, source_skin) in imported.skeleton.skins.iter() {
            // An imported non-default skin keeps its name; its entries never
            // merge into the open document's default skin, or a `@skin:` group
            // would silently become base art.
            let target = if source_skin_id == imported.skeleton.default_skin {
                default_skin
            } else {
                match doc
                    .skeleton
                    .skins
                    .iter()
                    .find(|(_, s)| s.name == source_skin.name)
                    .map(|(id, _)| id)
                {
                    Some(id) => id,
                    None => doc
                        .skeleton
                        .add_skin(ankhimate_core::skin::Skin::new(source_skin.name.clone())),
                }
            };
            for ((slot, name), attachment) in source_skin.entries.iter() {
                let Some(&slot) = slot_map.get(slot) else {
                    continue;
                };
                let mut attachment = attachment.clone();
                remap_assets(&mut attachment, &asset_names);
                if let Attachment::Mesh(mesh) = &mut attachment {
                    mesh.weights.iter_mut().flatten().for_each(|w| {
                        if let Some(&mapped) = bone_map.get(&w.bone) {
                            w.bone = mapped;
                        }
                    });
                }
                doc.skeleton.skins[target].set(slot, name.clone(), attachment);
            }
        }

        for (_, constraint) in imported.skeleton.constraints.iter() {
            let mut constraint = constraint.clone();
            remap_constraint(&mut constraint, &bone_map);
            doc.skeleton.add_constraint(constraint);
        }

        for (asset_name, layer_path) in imported.layer_paths {
            let target = asset_names.get(&asset_name).cloned().unwrap_or(asset_name);
            doc.psd_layer_paths.insert(target, layer_path);
        }

        doc.skeleton.rebuild_update_order();
    }

    fn revert(&mut self, doc: &mut Document) {
        if let Some(other) = self.other.as_mut() {
            std::mem::swap(doc, other);
        }
    }

    fn label(&self) -> &str {
        "Import PSD"
    }

    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Setup)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// AssetDb uniquifies names on insertion; references must follow that rename.
fn remap_assets(attachment: &mut Attachment, names: &HashMap<String, String>) {
    let (texture, sequence) = match attachment {
        Attachment::Region(region) => (&mut region.texture, &mut region.sequence),
        Attachment::Mesh(mesh) => (&mut mesh.texture, &mut mesh.sequence),
        _ => return,
    };
    let remap = |name: &mut String| {
        if let Some(target) = names.get(name) {
            name.clone_from(target);
        }
    };
    remap(texture);
    if let Some(sequence) = sequence {
        sequence.frames.iter_mut().for_each(remap);
    }
}

/// Point a constraint's bone references at their merged copies.
fn remap_constraint(
    constraint: &mut ankhimate_core::constraints::Constraint,
    map: &HashMap<BoneId, BoneId>,
) {
    use ankhimate_core::constraints::Constraint;
    let remap = |id: &mut BoneId| {
        if let Some(&mapped) = map.get(id) {
            *id = mapped;
        }
    };
    match constraint {
        Constraint::Ik(ik) => {
            remap(&mut ik.target);
            ik.bones.iter_mut().for_each(remap);
        }
        Constraint::Transform(tc) => {
            remap(&mut tc.target);
            tc.bones.iter_mut().for_each(remap);
        }
        Constraint::Physics(p) => remap(&mut p.bone),
        Constraint::Path(p) => p.bones.iter_mut().for_each(remap),
    }
}
