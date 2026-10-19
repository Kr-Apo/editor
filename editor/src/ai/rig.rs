//! Setup-only proposals use existing document commands on an asset-free scratch.
use super::tools::{Action, Snapshot, ToolError};
use ankhimate_core::{
    ids::BoneId,
    math::Transform,
    skeleton::{Bone, Skeleton},
};
use ankhimate_document::{
    commands::{
        EditCommand,
        bone_cmds::{CreateBone, RenameBone, SetBoneTransform},
    },
    doc::Document,
};
use serde_json::{Value, json};

#[cfg(test)]
pub fn value(v: &impl serde::Serialize) -> Value {
    serde_json::to_value(v).expect("Native scalar snapshot serialization")
}
pub fn same_bones(a: &Option<Skeleton>, b: &Option<Skeleton>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => super::assembly::fingerprint(a) == super::assembly::fingerprint(b),
        _ => false,
    }
}
pub fn check_hierarchy(s: &Skeleton) -> Result<(), &'static str> {
    let mut done = std::collections::HashSet::new();
    for start in s.bones.keys() {
        let mut path = std::collections::HashSet::new();
        let mut next = Some(start);
        while let Some(id) = next {
            if done.contains(&id) {
                break;
            }
            if !path.insert(id) {
                return Err("Cyclic hierarchy");
            }
            next = s.bones.get(id).ok_or("Missing parent in hierarchy")?.parent;
        }
        done.extend(path);
    }
    Ok(())
}
pub fn apply(
    doc: &mut Document,
    source: &Snapshot,
    a: &Action,
    index: usize,
) -> Result<Box<dyn EditCommand>, Box<ToolError>> {
    if a.is_weights() {
        return super::weights::apply(doc, source, a, index);
    }
    if a.is_assembly() {
        return super::assembly::apply(doc, source, a, index);
    }
    let err = |code: &str, field: &str, cause: &str| {
        Box::new(ToolError{code:code.into(),operation:a.operation().into(),action_index:Some(index),field:Some(field.into()),cause:cause.into(),correction:"Read exact bone properties; use a unique name, an existing unlocked parent and finite parent-local Setup values. No reparenting is available.".into(),document_changed:false,revision:source.revision})
    };
    let resolve = |s: &Skeleton, name: &str| -> Result<BoneId, Box<ToolError>> {
        let ids: Vec<_> = s
            .bones
            .iter()
            .filter(|(_, b)| b.name == name)
            .map(|(id, _)| id)
            .collect();
        let [id] = ids.as_slice() else {
            return Err(err("bone_not_unique", "bone", "Missing or ambiguous bone"));
        };
        if source.locked.contains(id) {
            return Err(err("bone_locked", "bone", "Bone is locked"));
        }
        Ok(*id)
    };
    let name_ok = |s: &Skeleton, name: &str| -> Result<(), Box<ToolError>> {
        if name.trim().is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
            return Err(err(
                "invalid_name",
                "name",
                "Nonempty name, at most 128 UTF-8 bytes and no control characters required",
            ));
        }
        if s.bones.values().any(|b| b.name == name) {
            return Err(err(
                "bone_exists",
                "name",
                "Bone name already exists; no automatic renaming",
            ));
        }
        Ok(())
    };
    let transform =
        |position: &[f32; 2], angle: f32, scale: &[f32; 2]| -> Result<Transform, Box<ToolError>> {
            if position.iter().any(|v| !v.is_finite() || v.abs() > 1e6)
                || !angle.is_finite()
                || angle.abs() > 360000.0
                || scale
                    .iter()
                    .any(|v| !v.is_finite() || v.abs() < 0.0001 || v.abs() > 100.0)
            {
                return Err(err(
                    "invalid_transform",
                    "position/rotation_degrees/scale",
                    "Out-of-range or nonfinite Setup transform",
                ));
            }
            Ok(Transform {
                position: glam::Vec2::from_array(*position),
                rotation: angle.to_radians(),
                scale: glam::Vec2::from_array(*scale),
                ..Default::default()
            })
        };
    let mut command: Box<dyn EditCommand> = match a {
        Action::BoneCreate {
            name,
            parent,
            position,
            rotation_degrees,
            length,
        } => {
            name_ok(&doc.skeleton, name)?;
            if !length.is_finite() || !(0.0..=1e6).contains(length) {
                return Err(err(
                    "invalid_length",
                    "length",
                    "Length must be finite, 0–1000000",
                ));
            }
            let parent = parent
                .as_ref()
                .map(|p| resolve(&doc.skeleton, p))
                .transpose()?;
            let bone = Bone {
                name: name.clone(),
                parent,
                length: *length,
                local_transform: transform(position, *rotation_degrees, &[1.0, 1.0])?,
                inherit: Default::default(),
                color: Bone::default_color(),
            };
            Box::new(CreateBone::new(bone))
        }
        Action::BoneSetup {
            bone,
            position,
            rotation_degrees,
            scale,
        } => {
            let id = resolve(&doc.skeleton, bone)?;
            let mut after = transform(position, *rotation_degrees, scale)?;
            after.shear = doc.skeleton.bones[id].local_transform.shear;
            Box::new(SetBoneTransform::new(id, after))
        }
        Action::BoneRename { bone, name } => {
            let id = resolve(&doc.skeleton, bone)?;
            if doc.skeleton.bones[id].name != *name {
                name_ok(&doc.skeleton, name)?;
            }
            Box::new(RenameBone::new(id, name))
        }
        _ => return Err(err("invalid_rig_action", "op", "Not a rigging action")),
    };
    command.apply(doc);
    check_hierarchy(&doc.skeleton).map_err(|why| err("invalid_hierarchy", "parent", why))?;
    Ok(command)
}

/// A native command group rebuilt on redo. Native inverses preserve unrelated
/// data; recreated IDs are broadcast through the existing History mechanism.
pub struct RigEdit {
    plan: super::tools::EditPlan,
    commands: Vec<Box<dyn EditCommand>>,
    created: Vec<BoneId>,
    remap: ankhimate_document::commands::IdRemap,
}
impl RigEdit {
    pub fn new(plan: super::tools::EditPlan) -> Self {
        Self {
            plan,
            commands: vec![],
            created: vec![],
            remap: Default::default(),
        }
    }
}
impl EditCommand for RigEdit {
    fn apply(&mut self, doc: &mut Document) {
        let mut scratch = Document {
            skeleton: doc.skeleton.clone(),
            animations: doc.animations.clone(),
            ..Document::new()
        };
        let source = Snapshot {
            metadata: Value::Null,
            doc: Document::new(),
            locked: vec![],
            weight_locked: vec![],
            revision: 0,
            setup: true,
        };
        let mut commands = vec![];
        let mut created = vec![];
        for (index, a) in self.plan.actions.iter().enumerate() {
            let command = apply(&mut scratch, &source, a, index)
                .expect("Validated native rig transaction must replay");
            if let Some(c) = command.as_any().downcast_ref::<CreateBone>() {
                created.push(c.created_id().expect("Native bone creation"));
            }
            commands.push(command);
        }
        for (old, new) in self.created.iter().zip(&created) {
            if old != new {
                self.remap.remap_bone(*old, *new);
            }
        }
        doc.skeleton = scratch.skeleton;
        self.commands = commands;
        self.created = created;
    }
    fn revert(&mut self, doc: &mut Document) {
        for command in self.commands.iter_mut().rev() {
            command.revert(doc);
        }
    }
    fn label(&self) -> &str {
        if self.plan.actions.iter().any(Action::is_weights) {
            "AI Weights Edit"
        } else if self.plan.actions.iter().any(Action::is_assembly) {
            "AI Assembly Edit"
        } else {
            "AI Rig Edit"
        }
    }
    fn requires_mode(&self) -> Option<ankhimate_document::WorkMode> {
        Some(ankhimate_document::WorkMode::Setup)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn take_remap(&mut self) -> ankhimate_document::commands::IdRemap {
        std::mem::take(&mut self.remap)
    }
    fn apply_remap(&mut self, remap: &ankhimate_document::commands::IdRemap) {
        for command in &mut self.commands {
            command.apply_remap(remap);
        }
        for id in &mut self.created {
            *id = remap.bone(*id);
        }
    }
}
pub fn definition() -> Value {
    let pair = json!({"type":"array","items":{"type":"number"},"minItems":2,"maxItems":2});
    json!({"name":"ankhimate_rig","description":"Propose Setup-only bone creation, local setup transforms or rename. No existing-bone reparenting. Explicit native approval required. Never mix rigging and animation in one batch. Preserve IDs, attachments and animation data; changing setup can visibly change their poses.","parameters":{"type":"object","properties":{"summary":{"type":"string","minLength":1},"actions":{"type":"array","minItems":1,"maxItems":128,"items":{"oneOf":[
        {"type":"object","properties":{"op":{"const":"bone.create"},"name":{"type":"string"},"parent":{"type":["string","null"]},"position":pair,"rotation_degrees":{"type":"number"},"length":{"type":"number","minimum":0,"maximum":1000000}},"required":["op","name","parent","position","rotation_degrees","length"],"additionalProperties":false},
        {"type":"object","properties":{"op":{"const":"bone.setup"},"bone":{"type":"string"},"position":pair,"rotation_degrees":{"type":"number"},"scale":pair},"required":["op","bone","position","rotation_degrees","scale"],"additionalProperties":false},
        {"type":"object","properties":{"op":{"const":"bone.rename"},"bone":{"type":"string"},"name":{"type":"string"}},"required":["op","bone","name"],"additionalProperties":false}
    ]}}},"required":["summary","actions"],"additionalProperties":false}})
}
