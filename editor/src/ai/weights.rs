//! Bounded adapters over native weight operations; no topology/FFD authoring.
use super::tools::{Action, Snapshot, ToolError};
use ankhimate_core::{
    attachment::{Attachment, MeshAttachment},
    ids::{SkinId, SlotId},
    transforms::Affine2,
};
use ankhimate_document::{
    commands::{
        EditCommand,
        weight_cmds::{self, BrushMode, SetWeights},
    },
    doc::Document,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
pub const PAGE: usize = 32;
pub const MAX_VERTICES: usize = 4096;
pub const MAX_TRIANGLES: usize = 8192;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedWeight {
    pub bone: String,
    pub weight: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VertexWeights {
    pub index: usize,
    pub influences: Vec<NamedWeight>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadMesh {
    pub skin: String,
    pub slot: String,
    pub attachment: String,
    pub start: usize,
    pub count: usize,
}
fn identity(a: &Action) -> (&str, &str, &str) {
    match a {
        Action::MeshSet {
            skin,
            slot,
            attachment,
            ..
        }
        | Action::MeshNormalize {
            skin,
            slot,
            attachment,
            ..
        }
        | Action::MeshSmooth {
            skin,
            slot,
            attachment,
            ..
        } => (skin, slot, attachment),
        _ => unreachable!(),
    }
}
pub fn selection(a: &Action) -> Vec<usize> {
    match a {
        Action::MeshSet { vertices, .. } => vertices.iter().map(|v| v.index).collect(),
        Action::MeshNormalize { vertices, .. } | Action::MeshSmooth { vertices, .. } => {
            vertices.clone()
        }
        _ => vec![],
    }
}
fn target<'a>(
    doc: &'a Document,
    skin: &str,
    slot: &str,
    name: &str,
) -> Result<(SkinId, SlotId, &'a MeshAttachment), String> {
    let slots: Vec<_> = doc
        .skeleton
        .slots
        .iter()
        .filter(|(_, s)| s.name == slot)
        .map(|(id, _)| id)
        .collect();
    let skins: Vec<_> = doc
        .skeleton
        .skins
        .iter()
        .filter(|(_, s)| s.name == skin)
        .map(|(id, _)| id)
        .collect();
    let ([slot], [skin]) = (slots.as_slice(), skins.as_slice()) else {
        return Err("Skin or slot is missing/ambiguous; use exact owned identities".into());
    };
    match doc.skeleton.skins[*skin].get(*slot, name) {
        Some(Attachment::Mesh(m)) => Ok((*skin, *slot, m)),
        Some(_) => Err("Attachment is not a mesh; regions are not weight tables".into()),
        None => Err("No exact owned mesh; implicit fallback edits are forbidden".into()),
    }
}
fn matrix(m: Affine2) -> [f32; 6] {
    [m.a, m.b, m.c, m.d, m.tx, m.ty]
}
fn valid_matrix(m: Affine2) -> bool {
    matrix(m).iter().all(|n| n.is_finite()) && m.invert().is_some()
}
fn guard<'a>(
    doc: &'a Document,
    source: &Snapshot,
    skin: &str,
    slot: &str,
    name: &str,
) -> Result<(SkinId, SlotId, &'a MeshAttachment), String> {
    let (sk, sl, m) = target(doc, skin, slot, name)?;
    let s = &doc.skeleton;
    super::rig::check_hierarchy(s).map_err(str::to_owned)?;
    if m.setup_vertices.len() < 3
        || m.setup_vertices.len() > MAX_VERTICES
        || m.triangles.is_empty()
        || m.triangles.len() > MAX_TRIANGLES
    {
        return Err(
            "Mesh size/topology outside the tool budget (3–4096 vertices, 1–8192 triangles)".into(),
        );
    }
    if m.linked.is_some()||s.skins.values().flat_map(|s|s.entries.values()).any(|a|matches!(a,Attachment::Mesh(other) if other.linked.as_ref().is_some_and(|l|l.slot==slot&&l.attachment==name&&(l.skin.is_none()||l.skin.as_deref()==Some(skin))))) {return Err("Linked mesh or shared linked source is read-only".into())}
    if s.skins
        .values()
        .map(|s| s.names_for_slot(sl).count())
        .sum::<usize>()
        != 1
    {
        return Err("Shared slot across attachments/skins is protected".into());
    }
    if !s.constraints.is_empty() {
        return Err("Constrained rigs require bind qualification; weights remain read-only".into());
    }
    if m.uvs.len() != m.setup_vertices.len()
        || m.weights.len() != m.setup_vertices.len()
        || m.setup_vertices
            .iter()
            .chain(&m.uvs)
            .any(|p| !p.is_finite() || p.abs().max_element() > 1e6)
    {
        return Err("Invalid vertex/UV/weight array or coordinates".into());
    }
    let mut triangles = std::collections::BTreeSet::new();
    for triangle in &m.triangles {
        let mut key = *triangle;
        key.sort();
        if key[2] as usize >= m.setup_vertices.len()
            || key[0] == key[1]
            || key[1] == key[2]
            || !triangles.insert(key)
        {
            return Err("Invalid, duplicate or out-of-range triangle".into());
        }
        let [a, b, c] = triangle.map(|i| m.setup_vertices[i as usize]);
        if (b - a).perp_dot(c - a).abs() <= 1e-8 {
            return Err("Degenerate triangle topology".into());
        }
    }
    if m.edges.iter().any(|[a, b]| {
        a == b || *a as usize >= m.setup_vertices.len() || *b as usize >= m.setup_vertices.len()
    }) {
        return Err("Invalid mesh edge".into());
    }
    let mut bones = std::collections::BTreeSet::new();
    for row in &m.weights {
        if row.is_empty() || row.len() > weight_cmds::DEFAULT_MAX_BONES {
            return Err("Existing weighted vertices need 1–4 influences (T-808 budget uses native default 4)".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for w in row {
            if !s.bones.contains_key(w.bone)
                || !seen.insert(w.bone)
                || !w.weight.is_finite()
                || w.weight <= 1e-4
                || w.weight > 1.
            {
                return Err("Invalid, duplicate, absent or nonpositive influence".into());
            }
            bones.insert(w.bone);
        }
    }
    let slot_bone = s.slots[sl].bone;
    if !s.bones.contains_key(slot_bone) || source.locked.contains(&slot_bone) {
        return Err("Slot's bone missing or locked".into());
    }
    for start in bones.iter().copied().chain(std::iter::once(slot_bone)) {
        let mut next = Some(start);
        while let Some(id) = next {
            if s.skins.values().any(|sk| sk.bones.contains(&id)) {
                return Err("Skin-dependent bone chain is protected".into());
            }
            next = s.bones[id].parent;
        }
    }
    let mut pose = ankhimate_core::pose::Pose::new();
    ankhimate_core::pose::evaluate(s, &[], &mut pose);
    let space = pose.world(slot_bone);
    if !valid_matrix(space) {
        return Err("Invalid slot Setup frame".into());
    }
    if m.inverse_bind_matrices.len() != bones.len() {
        return Err("Missing or extraneous inverse binds; manual repair required".into());
    }
    for bone in bones {
        if s.bones
            .values()
            .filter(|b| b.name == s.bones[bone].name)
            .count()
            != 1
        {
            return Err("Influencing bone name is ambiguous".into());
        }
        if !valid_matrix(pose.world(bone)) {
            return Err("Nonfinite or singular influencing Setup frame".into());
        }
        let Some(bind) = m
            .inverse_bind_matrices
            .get(&bone)
            .copied()
            .filter(|m| valid_matrix(*m))
        else {
            return Err("Invalid or missing inverse bind".into());
        };
        let Some(inv) = pose.world(bone).invert() else {
            return Err("Singular influencing bone Setup frame".into());
        };
        let expected = inv.mul(&space);
        if !valid_matrix(expected)
            || matrix(bind)
                .iter()
                .zip(matrix(expected))
                .any(|(a, b)| (*a - b).abs() > 1e-4)
        {
            return Err("Bind does not match the native Setup pose; no implicit rebind".into());
        }
    }
    let count = m.influence_count();
    let offsets_ok = |v: &[glam::Vec2]| {
        (v.len() == count || v.len() == m.setup_vertices.len()) && v.iter().all(|p| p.is_finite())
    };
    if m.ffd_keyframes
        .iter()
        .any(|k| !k.time.is_finite() || k.time < 0. || !offsets_ok(&k.vertex_offsets))
    {
        return Err("Invalid inline FFD indexing; geometry stays read-only".into());
    }
    for a in doc.animations.values() {
        for t in &a.timelines {
            if let ankhimate_core::animation::Timeline::Deform {
                slot,
                attachment,
                keys,
            } = t
                && *slot == sl
                && attachment == name
                && keys
                    .iter()
                    .any(|k| !k.time.is_finite() || k.time < 0. || !offsets_ok(&k.value))
            {
                return Err("Invalid animation FFD indexing; operation refused".into());
            }
        }
    }
    Ok((sk, sl, m))
}
fn rows(doc: &Document, m: &MeshAttachment, indices: &[usize]) -> Value {
    json!(indices.iter().filter_map(|i|m.weights.get(*i).map(|row|json!({"index":i,"influences":row.iter().take(8).map(|w|json!({"bone":doc.skeleton.bones.get(w.bone).map(|b|&b.name),"weight":w.weight})).collect::<Vec<_>>(),"sum":row.iter().map(|w|w.weight).sum::<f32>(),"influence_count":row.len(),"influences_truncated":row.len()>8}))).collect::<Vec<_>>())
}
pub fn review(doc: &Document, a: &Action) -> Value {
    let (skin, slot, attachment) = identity(a);
    let indices = selection(a);
    match target(doc, skin, slot, attachment) {
        Ok((_, _, m)) => {
            json!({"type":"mesh","skin":skin,"slot":slot,"attachment":attachment,"vertices":rows(doc,m,&indices),"affected_vertices":indices.len()})
        }
        Err(_) => Value::Null,
    }
}
pub fn apply(
    doc: &mut Document,
    source: &Snapshot,
    a: &Action,
    index: usize,
) -> Result<Box<dyn EditCommand>, Box<ToolError>> {
    let err = |code: &str, field: &str, cause: &str| {
        Box::new(ToolError{code:code.into(),operation:a.operation().into(),action_index:Some(index),field:Some(field.into()),cause:cause.into(),correction:"Read the exact owned weighted mesh page. Use <=32 explicit distinct vertices and the SAME ordered existing influences; finite positive coefficients. No linked/shared meshes, topology/FFD or bind edits. Repair invalid geometry/binds manually.".into(),document_changed:false,revision:source.revision})
    };
    let (skin, slot, name) = identity(a);
    let (sk, sl, m) =
        guard(doc, source, skin, slot, name).map_err(|e| err("unsafe_mesh", "mesh", &e))?;
    let selected = selection(a);
    let unique: std::collections::BTreeSet<_> = selected.iter().copied().collect();
    if selected.is_empty()
        || selected.len() > PAGE
        || selected.len() != unique.len()
        || selected.iter().any(|i| *i >= m.setup_vertices.len())
    {
        return Err(err(
            "invalid_vertices",
            "vertices",
            "Use 1–32 unique existing vertex indices",
        ));
    }
    if selected
        .iter()
        .flat_map(|i| &m.weights[*i])
        .any(|w| source.locked.contains(&w.bone) || source.weight_locked.contains(&w.bone))
    {
        return Err(err(
            "locked_influence",
            "vertices",
            "An affected vertex has a locked influence; no redistribution around locks is implicit",
        ));
    }
    let mut after = m.weights.clone();
    match a {
        Action::MeshSet { vertices, .. } => {
            for v in vertices {
                let old = &m.weights[v.index];
                if v.influences.len() != old.len() {
                    return Err(err(
                        "influence_membership",
                        "influences",
                        "Cannot add/remove influences or alter per-influence FFD indexing",
                    ));
                }
                let mut seen = std::collections::BTreeSet::new();
                for w in &v.influences {
                    if !seen.insert(&w.bone)
                        || !w.weight.is_finite()
                        || w.weight < 0.
                        || w.weight > 1.
                    {
                        return Err(err(
                            "invalid_weight",
                            "influences",
                            "Unique bone names and finite coefficients in [0,1] required",
                        ));
                    }
                }
                for w in &mut after[v.index] {
                    let name = &doc.skeleton.bones[w.bone].name;
                    let matches: Vec<_> = doc
                        .skeleton
                        .bones
                        .values()
                        .filter(|b| b.name == *name)
                        .collect();
                    if matches.len() != 1 {
                        return Err(err(
                            "bone_not_unique",
                            "bone",
                            "Influencing bone name is ambiguous",
                        ));
                    }
                    let Some(input) = v.influences.iter().find(|v| v.bone == *name) else {
                        return Err(err(
                            "influence_membership",
                            "bone",
                            "Every existing influence must be named; new/missing bones are refused",
                        ));
                    };
                    w.weight = input.weight;
                }
            }
        }
        Action::MeshNormalize { .. } => {}
        Action::MeshSmooth {
            bone,
            strength,
            iterations,
            ..
        } => {
            if !strength.is_finite()
                || *strength <= 0.
                || *strength > 1.
                || !(1..=4).contains(iterations)
            {
                return Err(err(
                    "invalid_smoothing",
                    "strength/iterations",
                    "Strength (0,1], iterations 1–4 required",
                ));
            }
            let ids: Vec<_> = doc
                .skeleton
                .bones
                .iter()
                .filter(|(_, b)| b.name == *bone)
                .map(|(id, _)| id)
                .collect();
            let [bone] = ids.as_slice() else {
                return Err(err(
                    "bone_not_unique",
                    "bone",
                    "Missing/ambiguous smoothing bone",
                ));
            };
            if selected
                .iter()
                .any(|i| !m.weights[*i].iter().any(|w| w.bone == *bone))
            {
                return Err(err(
                    "influence_membership",
                    "bone",
                    "Smoothing bone must already influence every selected vertex",
                ));
            }
            let distances: Vec<_> = (0..m.setup_vertices.len())
                .map(|i| {
                    if unique.contains(&i) {
                        0.
                    } else {
                        f32::INFINITY
                    }
                })
                .collect();
            let mut scratch = m.clone();
            for _ in 0..*iterations {
                scratch.weights = weight_cmds::brush(
                    &scratch,
                    *bone,
                    BrushMode::Smooth,
                    &distances,
                    1.,
                    *strength,
                    1.,
                    &[],
                );
            }
            after = scratch.weights;
        }
        _ => unreachable!(),
    }
    for i in &selected {
        weight_cmds::normalize(&mut after[*i]);
        let row = &after[*i];
        if row.len() != m.weights[*i].len()
            || row.iter().zip(&m.weights[*i]).any(|(a, b)| {
                a.bone != b.bone || !a.weight.is_finite() || a.weight <= 1e-4 || a.weight > 1.
            })
            || (row.iter().map(|w| w.weight).sum::<f32>() - 1.).abs() > 1e-4
        {
            return Err(err(
                "influence_membership",
                "influences",
                "Native normalization would prune/reorder influences or leave invalid sums; no FFD remapping is implicit",
            ));
        }
    }
    let mut command = SetWeights::new(sk, sl, name, after, "AI Weights Edit").preserving_bindings();
    command.apply(doc);
    Ok(Box::new(command))
}
pub fn read(source: &Snapshot, args: &ReadMesh) -> Result<Value, String> {
    if args.count == 0 || args.count > PAGE {
        return Err("Read count must be 1–32".into());
    }
    let doc = &source.doc;
    let (skin, slot, m) = target(doc, &args.skin, &args.slot, &args.attachment)?;
    let n = m.setup_vertices.len();
    if args.start > n || (args.start == n && n != 0) {
        return Err("Start is outside the existing vertices".into());
    }
    let end = args.start.saturating_add(args.count).min(n);
    let indices: Vec<_> = (args.start..end).collect();
    let protection = guard(doc, source, &args.skin, &args.slot, &args.attachment).err();
    let mut used = std::collections::BTreeSet::new();
    for row in m.weights.get(args.start..end).unwrap_or(&[]) {
        for w in row.iter().take(8) {
            used.insert(w.bone);
        }
    }
    // Never serialize the complete mesh or its texture/source reference.
    let influences = rows(doc, m, &indices);
    let vertices:Vec<_>=indices.iter().enumerate().map(|(at,i)|json!({"index":i,"position_local":m.setup_vertices[*i].to_array(),"uv":m.uvs.get(*i).map(|v|v.to_array()),"influences":influences[at]["influences"],"sum":influences[at]["sum"],"influence_count":influences[at]["influence_count"],"influences_truncated":influences[at]["influences_truncated"],"normalized":m.weights.get(*i).is_some_and(|w|(w.iter().map(|w|w.weight).sum::<f32>()-1.).abs()<=1e-4)})).collect();
    let result = json!({"skin":args.skin,"slot":args.slot,"attachment":args.attachment,"type":if m.linked.is_some(){"linked_mesh"}else if m.weights.is_empty(){"rigid_mesh"}else{"weighted_mesh"},"vertices_count":n,"triangles_count":m.triangles.len(),"influence_count":m.influence_count(),"start":args.start,"count":end-args.start,"next_start":(end<n).then_some(end),"vertices":vertices,"binds":used.into_iter().take(PAGE*4).map(|id|json!({"bone":doc.skeleton.bones.get(id).map(|b|&b.name),"locked":source.locked.contains(&id)||source.weight_locked.contains(&id),"inverse_bind":m.inverse_bind_matrices.get(&id),"valid":m.inverse_bind_matrices.get(&id).is_some_and(|m|valid_matrix(*m))})).collect::<Vec<_>>(),"slot_bone":doc.skeleton.bones.get(doc.skeleton.slots[slot].bone).map(|b|&b.name),"linked":m.linked,"inline_ffd_keys":m.ffd_keyframes.len(),"ffd_tracks":doc.animations.values().flat_map(|a|&a.timelines).filter(|t|matches!(t,ankhimate_core::animation::Timeline::Deform{slot:s,attachment,..}if *s==slot&&*attachment==args.attachment)).count(),"owned_skin":doc.skeleton.skins[skin].name,"editable":source.setup&&protection.is_none(),"write_limitation":protection,"edit_requires_mode":"setup","policy":"Same per-vertex influence IDs/order; existing valid Setup binds unchanged. Max 4 influences, 32 vertices/action, 128/lot. Geometry and FFD read-only. No artistic guarantee."});
    if result.to_string().len() > 32768 {
        return Err("Mesh page exceeds 32 KiB; reduce count".into());
    }
    Ok(result)
}
pub fn read_definition() -> Value {
    json!({"name":"ankhimate_read_mesh","description":"Read one exact owned mesh page, no images/paths. Vertex-local coordinates, weights, normalization, bind facts and limits. Linked geometry is raw owned data, not resolved art. Read before editing.","parameters":{"type":"object","properties":{"skin":{"type":"string"},"slot":{"type":"string"},"attachment":{"type":"string"},"start":{"type":"integer","minimum":0},"count":{"type":"integer","minimum":1,"maximum":32}},"required":["skin","slot","attachment","start","count"],"additionalProperties":false}})
}
pub fn definition() -> Value {
    let identity = json!({"op":{"const":"mesh.set_weights"},"skin":{"type":"string"},"slot":{"type":"string"},"attachment":{"type":"string"},"vertices":{"type":"array","minItems":1,"maxItems":32,"items":{"type":"object","properties":{"index":{"type":"integer","minimum":0},"influences":{"type":"array","minItems":1,"maxItems":4,"items":{"type":"object","properties":{"bone":{"type":"string"},"weight":{"type":"number","minimum":0,"maximum":1}},"required":["bone","weight"],"additionalProperties":false}}},"required":["index","influences"],"additionalProperties":false}}});
    let set = json!({"type":"object","properties":identity,"required":["op","skin","slot","attachment","vertices"],"additionalProperties":false});
    let mut normal = set.clone();
    normal["properties"]["op"]["const"] = json!("mesh.normalize_weights");
    normal["properties"]["vertices"]["items"] = json!({"type":"integer","minimum":0});
    let mut smooth = normal.clone();
    smooth["properties"]["op"]["const"] = json!("mesh.smooth_weights");
    smooth["properties"]["bone"] = json!({"type":"string"});
    smooth["properties"]["strength"] = json!({"type":"number","exclusiveMinimum":0,"maximum":1});
    smooth["properties"]["iterations"] = json!({"type":"integer","minimum":1,"maximum":4});
    smooth["required"] = json!([
        "op",
        "skin",
        "slot",
        "attachment",
        "vertices",
        "bone",
        "strength",
        "iterations"
    ]);
    json!({"name":"ankhimate_weights","description":"Propose a separate Setup weight lot on existing weighted meshes. Exact owned skin/slot/attachment; 1–32 explicit vertices/action, <=16 actions and <=128 vertices/lot. set_weights uses all existing influences per selected row and normalizes via native rules; never add/remove/reorder influences. smooth_weights uses the native one-bone neighbour brush. Valid binds and FFD indices remain unchanged. Shared/linked/ambiguous/locked/invalid meshes refused. Approval required; artistic quality not certified.","parameters":{"type":"object","properties":{"summary":{"type":"string","minLength":1,"maxLength":1000},"actions":{"type":"array","minItems":1,"maxItems":16,"items":{"oneOf":[set,normal,smooth]}}},"required":["summary","actions"],"additionalProperties":false}})
}
