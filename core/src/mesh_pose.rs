//! Shared geometry and deform ownership. No sampler, mutation, renderer or format.
use crate::{
    attachment::{Attachment, MeshAttachment},
    ids::{BoneId, SkinId, SlotId},
    pose::Pose,
    skeleton::Skeleton,
};
use glam::Vec2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshOwner<'a> {
    pub skin: SkinId,
    pub slot: SlotId,
    pub attachment: &'a str,
}
#[derive(Debug)]
pub struct MeshPose<'a> {
    pub displayed: MeshOwner<'a>,
    pub geometry_owner: MeshOwner<'a>,
    pub deform_owner: MeshOwner<'a>,
    pub geometry: &'a MeshAttachment,
    pub texture: &'a str,
    pub host: BoneId,
    pub offsets: Option<&'a [Vec2]>,
    linked: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshPoseError(pub String);
impl std::fmt::Display for MeshPoseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for MeshPoseError {}
fn check(ok: bool, msg: &str) -> Result<(), MeshPoseError> {
    if ok {
        Ok(())
    } else {
        Err(MeshPoseError(msg.into()))
    }
}
fn finite(m: crate::transforms::Affine2) -> bool {
    [m.a, m.b, m.c, m.d, m.tx, m.ty]
        .iter()
        .all(|v| v.is_finite())
}
impl MeshPose<'_> {
    /// Preserve native per-influence packing and empty-row host fallback. Offsets
    /// pass through the source inverse bind's linear part, never its translation alone.
    pub fn vertices(&self, pose: &Pose) -> Result<Vec<Vec2>, MeshPoseError> {
        let g = self.geometry;
        let host = pose.world(self.host);
        let values = (0..g.setup_vertices.len())
            .map(|i| {
                let offsets = self
                    .offsets
                    .map(|d| {
                        if g.weights.is_empty() {
                            d.get(i..i + 1).unwrap_or_default()
                        } else {
                            d.get(g.influence_range(i)).unwrap_or_default()
                        }
                    })
                    .unwrap_or_default();
                g.skin_vertex_with_deform(i, offsets, pose, &host)
            })
            .collect::<Vec<_>>();
        if self.linked {
            check(
                values.iter().all(|v| v.is_finite()),
                "Linked mesh produced nonfinite vertices",
            )?;
        }
        Ok(values)
    }
}
impl Skeleton {
    /// Exact owner lookup. Nonlinked behavior is unchanged. Linked FFD is qualified
    /// only in one fixed default skin, with a direct source and immutable topology.
    /// `false` permits a native own track; `true` never falls back to that own track.
    pub fn resolve_mesh_pose<'a>(
        &'a self,
        active: &[SkinId],
        pose: &'a Pose,
        slot: SlotId,
        name: &'a str,
    ) -> Result<MeshPose<'a>, MeshPoseError> {
        let host = self
            .slots
            .get(slot)
            .ok_or_else(|| MeshPoseError("Displayed slot missing".into()))?
            .bone;
        let skin = active
            .iter()
            .chain(std::iter::once(&self.default_skin))
            .copied()
            .find(|id| {
                self.skins
                    .get(*id)
                    .is_some_and(|s| s.get(slot, name).is_some())
            })
            .ok_or_else(|| MeshPoseError("Displayed mesh owner missing".into()))?;
        let Some(Attachment::Mesh(displayed)) = self.skins[skin].get(slot, name) else {
            return Err(MeshPoseError("Displayed attachment is not a mesh".into()));
        };
        let displayed_owner = MeshOwner {
            skin,
            slot,
            attachment: name,
        };
        let Some(link) = &displayed.linked else {
            return Ok(MeshPose {
                displayed: displayed_owner,
                geometry_owner: displayed_owner,
                deform_owner: displayed_owner,
                geometry: displayed,
                texture: &displayed.texture,
                host,
                offsets: pose.deforms.get(&(slot, name.into())).map(Vec::as_slice),
                linked: false,
            });
        };
        check(
            self.skins.len() == 1
                && skin == self.default_skin
                && active.iter().all(|s| *s == self.default_skin),
            "Linked FFD requires one fixed default skin; ambiguous owners are unsupported",
        )?;
        let default = &self.skins[self.default_skin];
        check(
            link.skin.as_ref().is_none_or(|s| *s == default.name),
            "Linked source skin missing or outside fixed default",
        )?;
        let slots = self
            .slots
            .iter()
            .filter(|(_, s)| s.name == link.slot)
            .collect::<Vec<_>>();
        check(slots.len() == 1, "Linked source slot missing or ambiguous")?;
        let source_slot = slots[0].0;
        let Some(Attachment::Mesh(g)) = default.get(source_slot, &link.attachment) else {
            return Err(MeshPoseError(
                "Linked source is missing or not a mesh".into(),
            ));
        };
        check(
            g.linked.is_none(),
            "Linked chains and cycles are unsupported",
        )?;
        check(
            g.setup_vertices.len() >= 3
                && g.uvs.len() == g.setup_vertices.len()
                && !g.triangles.is_empty()
                && g.setup_vertices.iter().chain(&g.uvs).all(|v| v.is_finite())
                && g.triangles
                    .iter()
                    .flatten()
                    .all(|i| (*i as usize) < g.setup_vertices.len()),
            "Linked source topology/UVs invalid",
        )?;
        check(
            g.uvs
                .iter()
                .all(|v| (0.0..=1.0).contains(&v.x) && (0.0..=1.0).contains(&v.y)),
            "Linked source UVs outside normalized bounds",
        )?;
        check(
            g.triangles.iter().all(|t| {
                t[0] != t[1]
                    && t[0] != t[2]
                    && t[1] != t[2]
                    && (g.setup_vertices[t[1] as usize] - g.setup_vertices[t[0] as usize])
                        .perp_dot(g.setup_vertices[t[2] as usize] - g.setup_vertices[t[0] as usize])
                        .abs()
                        > 1e-12
            }),
            "Linked source triangles degenerate",
        )?;
        check(
            self.bones.contains_key(self.slots[source_slot].bone),
            "Linked source host is missing",
        )?;
        check(
            g.weights.is_empty() || g.weights.len() == g.setup_vertices.len(),
            "Linked influence topology invalid",
        )?;
        check(
            self.bones.contains_key(host)
                && pose.worlds.contains_key(host)
                && finite(pose.world(host)),
            "Linked displayed host invalid",
        )?;
        for row in &g.weights {
            let mut total = 0.;
            let mut bones = std::collections::HashSet::new();
            for w in row {
                check(
                    self.bones.contains_key(w.bone)
                        && bones.insert(w.bone)
                        && w.weight.is_finite()
                        && w.weight >= 0.,
                    "Linked influence missing, duplicate or invalid",
                )?;
                let bind = g
                    .inverse_bind_matrices
                    .get(&w.bone)
                    .ok_or_else(|| MeshPoseError("Linked source bind absent or singular".into()))?;
                check(
                    finite(*bind)
                        && bind.invert().is_some()
                        && pose.worlds.contains_key(w.bone)
                        && finite(pose.world(w.bone)),
                    "Linked source bind/world invalid or singular",
                )?;
                total += w.weight;
            }
            check(
                total.is_finite() && (row.is_empty() || total > 0.),
                "Linked weight total invalid",
            )?;
        }
        let geometry_owner = MeshOwner {
            skin: self.default_skin,
            slot: source_slot,
            attachment: &link.attachment,
        };
        let deform_owner = if link.inherit_deform {
            geometry_owner
        } else {
            displayed_owner
        };
        let offsets = pose
            .deforms
            .get(&(deform_owner.slot, deform_owner.attachment.into()))
            .map(Vec::as_slice);
        if let Some(d) = offsets {
            let expected = if g.weights.is_empty() {
                g.setup_vertices.len()
            } else {
                g.influence_count()
            };
            check(
                (d.is_empty() || d.len() == expected) && d.iter().all(|v| v.is_finite()),
                "Linked FFD offsets have invalid dimensions or nonfinite values",
            )?;
        }
        Ok(MeshPose {
            displayed: displayed_owner,
            geometry_owner,
            deform_owner,
            geometry: g,
            texture: &displayed.texture,
            host,
            offsets,
            linked: true,
        })
    }
}
