//! Safe preparation around the existing tracer; no new triangulator or skinning.
use crate::{
    Document,
    commands::mesh_cmds::TraceMesh,
    meshgen,
    snapshot::{Snapshot, snapshot},
};
use ankhimate_core::{
    animation::Timeline,
    attachment::{Attachment, MeshAttachment},
    ids::{SkinId, SlotId},
    pose::{Pose, evaluate},
};
use glam::Vec2;
use std::collections::BTreeSet;

/// Conservative operation limits, not format limits. Never prune to fit them.
pub const MAX_COMPARISONS: usize = 16_000_000;
pub const MAX_INFLUENCES: usize = crate::commands::weight_cmds::DEFAULT_MAX_BONES;

#[derive(Clone, Debug)]
pub struct Summary {
    pub old_vertices: usize,
    pub new_vertices: usize,
    pub old_triangles: usize,
    pub new_triangles: usize,
    pub old_influences: usize,
    pub new_influences: usize,
    pub binds: usize,
    pub unsampled_source_vertices: usize,
    pub source_indices: Vec<usize>,
}
impl Summary {
    pub fn lines(&self) -> Vec<String> {
        vec![format!("Vertices: {} → {}; triangles: {} → {}",self.old_vertices,self.new_vertices,self.old_triangles,self.new_triangles),
            format!("Influences: {} → {}; inverse binds retained: {}",self.old_influences,self.new_influences,self.binds),
            "FFD: absent; linked/shared ownership: absent (otherwise refused)".into(),
            if self.old_influences==0 { "Rigid mesh; no weight transfer needed".into() } else { format!("Copy nearest source rows in slot-bone-local Setup space; exact ties use lowest source index. {} source vertices are not sampled. All source BoneIds retained; rows are not normalized or pruned.",self.unsampled_source_vertices) },
            "Nearest-vertex transfer does not guarantee identical artistic deformation. Review the new shape and paint weights if needed. No animation is deleted.".into()]
    }
}
pub struct Plan {
    skin: SkinId,
    slot: SlotId,
    name: String,
    before: Snapshot,
    old: MeshAttachment,
    after: MeshAttachment,
    pub summary: Summary,
    noop: bool,
}

fn topology(vertices: &[Vec2], uvs: &[Vec2], triangles: &[[u32; 3]]) -> Result<(), String> {
    if vertices.len() < 3
        || vertices.len() != uvs.len()
        || triangles.is_empty()
        || vertices.iter().chain(uvs).any(|p| !p.is_finite())
    {
        return Err("Invalid mesh vertices/UVs/triangles".into());
    }
    for t in triangles {
        if t.iter().any(|i| *i as usize >= vertices.len())
            || t[0] == t[1]
            || t[1] == t[2]
            || t[0] == t[2]
        {
            return Err("Invalid triangle indices".into());
        }
        let [a, b, c] = t.map(|i| vertices[i as usize].as_dvec2());
        if (b - a).perp_dot(c - a).abs() <= 1e-12 {
            return Err("Degenerate mesh triangle".into());
        }
    }
    Ok(())
}
/// Validate source references before calling any geometry generation.
pub fn source<'a>(
    doc: &'a Document,
    skin: SkinId,
    slot: SlotId,
    name: &str,
) -> Result<&'a MeshAttachment, String> {
    if crate::ffd_keys::owner(doc, slot, name)
        .map_err(|_| "Mesh slot/attachment must have exactly one owning skin")?
        != skin
    {
        return Err("Mesh skin owner is ambiguous or changed".into());
    }
    let sl = doc.skeleton.slots.get(slot).ok_or("Mesh slot is absent")?;
    if !doc.skeleton.bones.contains_key(sl.bone) {
        return Err("Mesh slot bone is absent".into());
    }
    let sk = doc.skeleton.skins.get(skin).ok_or("Mesh skin is absent")?;
    let Some(Attachment::Mesh(m)) = sk.get(slot, name) else {
        return Err("Trace target is not an owned mesh".into());
    };
    if m.linked.is_some() || m.sequence.is_some() {
        return Err("Linked meshes and image sequences cannot be retraced".into());
    }
    if !m.ffd_keyframes.is_empty()
        || doc.animations.values().any(|a| {
            a.timelines.iter().any(
                |t| matches!(t,Timeline::Deform{slot:s,attachment:n,..} if *s==slot && n==name),
            )
        })
    {
        return Err("FFD inline keys or Deform timelines depend on this topology; retrace refused without deleting keys".into());
    }
    doc.skeleton
        .skins
        .get(doc.skeleton.default_skin)
        .ok_or("Default skin is absent")?;
    for s in doc.skeleton.skins.values() {
        for a in s.entries.values() {
            if let Attachment::Mesh(other) = a
                && let Some(l) = &other.linked
            {
                // Unpinned links resolve through the active/owning skin, then
                // fallback. Conservatively protect every matching unique source.
                if l.slot == sl.name
                    && l.attachment == name
                    && l.skin.as_deref().is_none_or(|src| src == sk.name)
                {
                    return Err("Source of a linked mesh is shared; retrace refused".into());
                }
            }
        }
    }
    if !m.edges.is_empty() {
        return Err(
            "Pinned edges have no safe index remapping; remove explicitly before retracing".into(),
        );
    }
    topology(&m.setup_vertices, &m.uvs, &m.triangles)?;
    if m.weights.is_empty() {
        if !m.inverse_bind_matrices.is_empty() {
            return Err("Rigid mesh has unexpected bind data".into());
        }
        return Ok(m);
    }
    if m.weights.len() != m.setup_vertices.len() {
        return Err("Weight rows do not match vertices".into());
    }
    let mut bones = BTreeSet::new();
    for row in &m.weights {
        let sum: f32 = row.iter().map(|w| w.weight).sum();
        if row.len() > MAX_INFLUENCES
            || (!row.is_empty() && (!sum.is_finite() || (sum - 1.).abs() > 0.0001))
        {
            return Err("Weight rows must already be normalized, with at most four influences in this retrace scope".into());
        }
        let mut unique = BTreeSet::new();
        for w in row {
            if !doc.skeleton.bones.contains_key(w.bone)
                || !unique.insert(w.bone)
                || !w.weight.is_finite()
                || !(0.0..=1.0).contains(&w.weight)
            {
                return Err("Invalid, duplicate or missing bone influence".into());
            }
            bones.insert(w.bone);
        }
    }
    if bones.is_empty() || m.inverse_bind_matrices.len() != bones.len() {
        return Err("Weighted mesh bind coverage is incomplete or ambiguous".into());
    }
    let mut pose = Pose::new();
    evaluate(&doc.skeleton, &[], &mut pose);
    let space = pose.world(sl.bone);
    if space.invert().is_none() {
        return Err("Slot Setup frame is singular".into());
    }
    for bone in bones {
        let b = m
            .inverse_bind_matrices
            .get(&bone)
            .ok_or("Required inverse bind is missing")?;
        if [b.a, b.b, b.c, b.d, b.tx, b.ty]
            .iter()
            .any(|v| !v.is_finite())
            || b.invert().is_none()
        {
            return Err("Inverse bind is non-finite or singular".into());
        }
        let expected = pose
            .world(bone)
            .invert()
            .ok_or("Influence Setup frame is singular")?
            .mul(&space);
        if *b != expected {
            return Err("Bind differs from native Setup convention; cannot promise Save/Reload preservation".into());
        }
    }
    // The existing trace mapper uses one affine UV frame. Non-affine authored
    // meshes cannot be reprojected by that mapper without an approximation.
    let (linear, translation) =
        meshgen::uv_to_local_affine(m).ok_or("Weighted mesh has no valid affine UV frame")?;
    if m.uvs
        .iter()
        .zip(&m.setup_vertices)
        .any(|(uv, p)| (linear * *uv + translation - *p).abs().max_element() > 0.0001)
    {
        return Err("Weighted mesh UV-to-Setup frame is non-affine; retrace refused".into());
    }
    Ok(m)
}

impl Plan {
    pub fn from_trace(
        doc: &Document,
        skin: SkinId,
        slot: SlotId,
        name: &str,
        traced: &meshgen::Traced,
    ) -> Result<Self, String> {
        let m = source(doc, skin, slot, name)?;
        let (vertices, uvs, triangles) = meshgen::mesh_from_trace_on_mesh(traced, m);
        Self::prepare(doc, skin, slot, name, vertices, uvs, triangles)
    }
    /// Coordinates are the same slot-bone-local Setup frame as setup_vertices.
    pub fn prepare(
        doc: &Document,
        skin: SkinId,
        slot: SlotId,
        name: &str,
        vertices: Vec<Vec2>,
        uvs: Vec<Vec2>,
        triangles: Vec<[u32; 3]>,
    ) -> Result<Self, String> {
        let m = source(doc, skin, slot, name)?;
        topology(&vertices, &uvs, &triangles)?;
        let mut after = m.clone();
        after.setup_vertices = vertices;
        after.uvs = uvs;
        after.triangles = triangles;
        let mut source_indices = Vec::new();
        if !m.weights.is_empty() {
            if m.setup_vertices
                .len()
                .checked_mul(after.setup_vertices.len())
                .is_none_or(|n| n > MAX_COMPARISONS)
            {
                return Err("Nearest-vertex comparison budget exceeded; retrace refused".into());
            }
            for p in &after.setup_vertices {
                let index = m
                    .setup_vertices
                    .iter()
                    .enumerate()
                    .min_by(|(i, a), (j, b)| {
                        a.as_dvec2()
                            .distance_squared(p.as_dvec2())
                            .total_cmp(&b.as_dvec2().distance_squared(p.as_dvec2()))
                            .then(i.cmp(j))
                    })
                    .expect("validated source vertices")
                    .0;
                source_indices.push(index);
            }
            after.weights = source_indices
                .iter()
                .map(|i| m.weights[*i].clone())
                .collect();
            let bone_set = |mesh: &MeshAttachment| {
                mesh.weights
                    .iter()
                    .flatten()
                    .map(|w| w.bone)
                    .collect::<BTreeSet<_>>()
            };
            if bone_set(m) != bone_set(&after) {
                return Err(
                    "Nearest-vertex transfer would lose a source bone influence; retrace refused"
                        .into(),
                );
            }
        }
        let summary = Summary {
            old_vertices: m.setup_vertices.len(),
            new_vertices: after.setup_vertices.len(),
            old_triangles: m.triangles.len(),
            new_triangles: after.triangles.len(),
            old_influences: m.influence_count(),
            new_influences: after.influence_count(),
            binds: m.inverse_bind_matrices.len(),
            unsampled_source_vertices: if m.weights.is_empty() {
                0
            } else {
                m.setup_vertices.len() - source_indices.iter().collect::<BTreeSet<_>>().len()
            },
            source_indices,
        };
        let noop = serde_json::to_value(m).map_err(|e| e.to_string())?
            == serde_json::to_value(&after).map_err(|e| e.to_string())?;
        Ok(Self {
            skin,
            slot,
            name: name.into(),
            before: snapshot(doc),
            old: m.clone(),
            after,
            summary,
            noop,
        })
    }
    pub fn result(&self) -> &MeshAttachment {
        &self.after
    }
    pub fn into_command(self, doc: &Document) -> Result<Option<TraceMesh>, String> {
        if self.before != snapshot(doc) {
            return Err("Trace plan is stale; reopen the preview".into());
        }
        if self.noop {
            return Ok(None);
        }
        Ok(Some(TraceMesh::prepared(
            self.skin, self.slot, self.name, self.old, self.after,
        )))
    }
}
