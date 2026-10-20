//! Precise drafts over the existing geometry commands. No persistent selection or new geometry.
use crate::{
    app_state::AppState,
    session::{Tool, WorkMode},
};
use ankhimate_core::{
    attachment::Attachment,
    ids::{SkinId, SlotId},
};
use ankhimate_document::commands::{
    EditCommand,
    attachment_cmds::owning_skin,
    clip_cmds::{ClipEdit, EditBoundingBox, EditClip, EditPath},
    mesh_cmds::{EditMesh, MeshEdit},
};
use eframe::egui;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Position,
    Uv,
}
#[derive(Debug, Clone, PartialEq)]
struct Target {
    active_skin: SkinId,
    owner: SkinId,
    slot: SlotId,
    name: String,
    kind: u8,
    tool: Tool,
    mode: WorkMode,
    editing: bool,
}
#[derive(Debug, Clone)]
pub struct Lease {
    target: Target,
    revision: u64,
    indices: Vec<usize>,
}
fn target(s: &AppState) -> Option<Target> {
    if s.session.selected_slots.len() != 1 {
        return None;
    }
    let slot = s.session.active_slot()?;
    let sl = s.doc.skeleton.slots.get(slot)?;
    let name = sl.attachment.clone()?;
    if let Some(crate::session::Selection::Attachment {
        slot: focused,
        name: focused_name,
    }) = &s.session.selection
        && (*focused != slot || *focused_name != name)
    {
        return None;
    }
    let owner = owning_skin(&s.doc, s.session.active_skin, slot, &name)?;
    let kind = match s.doc.skeleton.skins.get(owner)?.get(slot, &name)? {
        Attachment::Mesh(_) => 0,
        Attachment::BoundingBox(_) => 1,
        Attachment::Clipping(_) => 2,
        Attachment::Path(_) => 3,
        _ => return None,
    };
    Some(Target {
        active_skin: s.session.active_skin,
        owner,
        slot,
        name,
        kind,
        tool: s.session.tool,
        mode: s.session.work_mode,
        editing: s.session.mesh_edit,
    })
}
/// Legacy tools still own the indices. This lease only scopes them; no second selection.
pub fn bind(s: &mut AppState) {
    let current = target(s);
    if s.session.geometry_lease.as_ref().is_none_or(|l| {
        Some(&l.target) != current.as_ref()
            || l.revision != s.revision
            || l.indices != s.session.selected_vertices
    }) {
        s.session.geometry_epoch = s.session.geometry_epoch.wrapping_add(1);
    }
    s.session.geometry_lease = current.map(|target| Lease {
        target,
        revision: s.revision,
        indices: s.session.selected_vertices.clone(),
    });
}
pub fn sync(s: &mut AppState) {
    let current = target(s);
    let stale = s
        .session
        .geometry_lease
        .as_ref()
        .is_some_and(|l| Some(&l.target) != current.as_ref() || l.revision != s.revision);
    if stale || current.is_none() {
        s.session.selected_vertices.clear();
        s.session.hovered_vertex = None;
        s.session.deform_drag = None;
        s.session.dragging_vertex = None;
        s.session.vertex_box_start = None;
        if let Some(p) = s.session.uv_pane.as_mut() {
            p.dragging = None;
        }
    }
    if let Some(p) = s.session.uv_pane.as_ref()
        && !current
            .as_ref()
            .is_some_and(|t| t.owner == p.skin && t.slot == p.slot && t.name == p.name)
    {
        crate::ui::uv::clear(s);
    }
    bind(s);
}
#[derive(Clone, PartialEq)]
pub(crate) struct Draft {
    target: Target,
    revision: u64,
    epoch: u64,
    indices: Vec<usize>,
    domain: Domain,
    snapshot: serde_json::Value,
    binds: Vec<(String, [f32; 6])>,
    points: Vec<glam::Vec2>,
    pub fields: [String; 2],
    pub relative: bool,
}
fn attachment<'a>(s: &'a AppState, t: &Target) -> Result<&'a Attachment, String> {
    s.doc
        .skeleton
        .skins
        .get(t.owner)
        .and_then(|k| k.get(t.slot, &t.name))
        .ok_or_else(|| "Geometry target no longer exists".into())
}
fn binds(a: &Attachment) -> Vec<(String, [f32; 6])> {
    let mut out = if let Attachment::Mesh(m) = a {
        m.inverse_bind_matrices
            .iter()
            .map(|(b, m)| (format!("{b:?}"), [m.a, m.b, m.c, m.d, m.tx, m.ty]))
            .collect()
    } else {
        Vec::new()
    };
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}
fn values(a: &Attachment, domain: Domain) -> Result<&[glam::Vec2], String> {
    match (a, domain) {
        (Attachment::Mesh(m), Domain::Position) => Ok(&m.setup_vertices),
        (Attachment::Mesh(m), Domain::Uv) => Ok(&m.uvs),
        (Attachment::Path(p), Domain::Position) => Ok(&p.vertices),
        (Attachment::BoundingBox(b), Domain::Position) => Ok(&b.vertices),
        (Attachment::Clipping(c), Domain::Position) => Ok(&c.vertices),
        _ => Err("This attachment has no editable UV coordinates".into()),
    }
}
/// Same numeric syntax as the compiled egui DragValue (decimal/scientific, whitespace,
/// Unicode minus). Expressions are not enabled there; no evaluator is introduced here.
fn number(text: &str) -> Result<Option<f32>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let normalized: String = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| if c == '−' { '-' } else { c })
        .collect();
    let value = normalized
        .parse::<f32>()
        .map_err(|_| "Enter finite decimal/scientific numbers; expressions are not supported")?;
    if !value.is_finite() {
        return Err("Coordinates must be finite and representable as f32".into());
    }
    Ok(Some(value))
}
impl Draft {
    pub fn new(
        s: &mut AppState,
        skin: SkinId,
        slot: SlotId,
        name: &str,
        domain: Domain,
    ) -> Result<Self, String> {
        sync(s);
        let t = target(s).ok_or("Select the displayed mesh or polygon")?;
        if t.owner != skin || t.slot != slot || t.name != name {
            return Err("Selection belongs to a different attachment".into());
        }
        let mut indices = s.session.selected_vertices.clone();
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            return Err("Select vertices on the canvas or UV pane".into());
        }
        let a = attachment(s, &t)?;
        let all = values(a, domain)?;
        let points = indices
            .iter()
            .map(|i| {
                all.get(*i)
                    .copied()
                    .ok_or("Selected vertex no longer exists")
            })
            .collect::<Result<Vec<_>, _>>()?;
        if points.iter().any(|p| !p.is_finite()) {
            return Err("Existing coordinates are non-finite".into());
        }
        let common = |axis: usize| {
            let v = points[0][axis];
            if points.iter().all(|p| p[axis].to_bits() == v.to_bits()) {
                v.to_string()
            } else {
                String::new()
            }
        };
        Ok(Self {
            target: t,
            revision: s.revision,
            epoch: s.session.geometry_epoch,
            indices,
            domain,
            snapshot: serde_json::to_value(a).map_err(|e| e.to_string())?,
            binds: binds(a),
            fields: [common(0), common(1)],
            points,
            relative: false,
        })
    }
    fn unchanged(&self, other: &Self) -> bool {
        self.target == other.target
            && self.revision == other.revision
            && self.epoch == other.epoch
            && self.indices == other.indices
            && self.snapshot == other.snapshot
            && self.binds == other.binds
            && self.domain == other.domain
    }
    fn moves(&self) -> Result<Vec<(usize, glam::Vec2)>, String> {
        let axes = [number(&self.fields[0])?, number(&self.fields[1])?];
        if axes.iter().all(Option::is_none) {
            return Err("Enter at least one axis; blank leaves an axis unchanged".into());
        }
        self.indices
            .iter()
            .zip(&self.points)
            .map(|(i, p)| {
                let mut next = *p;
                for axis in 0..2 {
                    if let Some(v) = axes[axis] {
                        next[axis] = if self.relative { p[axis] + v } else { v };
                    }
                }
                if !next.is_finite() {
                    return Err("Coordinate result overflowed; nothing applied".into());
                }
                if self.domain == Domain::Uv
                    && (!(0.0..=1.0).contains(&next.x) || !(0.0..=1.0).contains(&next.y))
                {
                    return Err("UV result must be in [0,1]; no implicit clamp".into());
                }
                Ok((*i, next))
            })
            .collect()
    }
    fn validate_target(&self, s: &AppState) -> Result<(), String> {
        if self.target.mode != WorkMode::Setup || !s.session.can_edit_structure() {
            return Err(
                "Switch to Setup for geometry/UV edits; Animate uses the existing FFD tool".into(),
            );
        }
        let bone = s
            .doc
            .skeleton
            .slots
            .get(self.target.slot)
            .ok_or("Missing slot")?
            .bone;
        if s.session.is_bone_locked(bone) {
            return Err("Slot bone is locked".into());
        }
        if let Attachment::Mesh(m) = attachment(s, &self.target)? {
            if m.linked.is_some() {
                return Err("Linked meshes are read-only for numeric edits".into());
            }
            let owner = &s.doc.skeleton.skins[self.target.owner].name;
            let slot_name = &s.doc.skeleton.slots[self.target.slot].name;
            let default_name = &s.doc.skeleton.skins[s.doc.skeleton.default_skin].name;
            let referenced = s.doc.skeleton.skins.values().any(|skin| {
                skin.entries.values().any(|attachment| {
                    matches!(attachment, Attachment::Mesh(child) if child.linked.as_ref().is_some_and(|link| {
                        link.slot == *slot_name && link.skin.as_ref().unwrap_or(default_name) == owner
                            && link.attachment == self.target.name
                    }))
                })
            });
            if referenced {
                return Err("Mesh referenced by a linked mesh; numeric edits require a separate sharing contract".into());
            }
            let n = m.setup_vertices.len();
            if n < 3
                || m.uvs.len() != n
                || m.triangles.iter().flatten().any(|i| *i as usize >= n)
                || (!m.weights.is_empty() && m.weights.len() != n)
            {
                return Err("Invalid mesh topology/UV/influence alignment".into());
            }
            for weights in &m.weights {
                for w in weights {
                    if !s.doc.skeleton.bones.contains_key(w.bone)
                        || !w.weight.is_finite()
                        || w.weight < 0.0
                    {
                        return Err("Invalid influence reference/weight".into());
                    }
                    let bind = m
                        .inverse_bind_matrices
                        .get(&w.bone)
                        .ok_or("Missing inverse bind; numeric edit refused")?;
                    if ![bind.a, bind.b, bind.c, bind.d, bind.tx, bind.ty]
                        .iter()
                        .all(|v| v.is_finite())
                        || bind.invert().is_none()
                    {
                        return Err("Invalid inverse bind".into());
                    }
                }
            }
            for &i in &self.indices {
                if let Some(weights) = m.weights.get(i)
                    && weights.iter().any(|w| s.session.is_bone_locked(w.bone))
                {
                    return Err("Influencing bone is locked".into());
                }
            }
        }
        Ok(())
    }
    pub fn apply(&self, s: &mut AppState) -> Result<(), String> {
        let current = Self::new(
            s,
            self.target.owner,
            self.target.slot,
            &self.target.name,
            self.domain,
        )?;
        if !self.unchanged(&current) {
            return Err("Geometry selection/revision changed; draft discarded".into());
        }
        self.validate_target(s)?;
        let moves = self.moves()?;
        if self.domain == Domain::Position {
            let slot_bone = s.doc.skeleton.slots[self.target.slot].bone;
            let world = s
                .pose
                .worlds
                .get(slot_bone)
                .ok_or("Missing evaluated slot transform")?;
            let a = attachment(s, &self.target)?;
            let mut candidate = if let Attachment::Mesh(m) = a {
                Some(m.clone())
            } else {
                None
            };
            if let Some(m) = candidate.as_mut() {
                for (i, p) in &moves {
                    m.setup_vertices[*i] = *p;
                }
            }
            for (i, p) in &moves {
                let placed = if let Some(m) = candidate.as_ref() {
                    m.skin_vertex_with_deform(*i, &[], &s.pose, world)
                } else {
                    world.transform_point(*p)
                };
                if !placed.is_finite() {
                    return Err(
                        "Coordinates overflow the evaluated transform; nothing applied".into(),
                    );
                }
            }
        }
        if moves.iter().zip(&self.points).all(|((_, a), b)| a == b) {
            return Ok(());
        }
        let t = &self.target;
        let cmd: Box<dyn EditCommand> = match t.kind {
            0 => Box::new(
                EditMesh::new(
                    t.owner,
                    t.slot,
                    t.name.clone(),
                    if self.domain == Domain::Uv {
                        MeshEdit::MoveUvs(moves)
                    } else {
                        MeshEdit::MoveVertices(moves)
                    },
                )
                .isolated(),
            ),
            1 => Box::new(
                EditBoundingBox::new(
                    t.owner,
                    t.slot,
                    t.name.clone(),
                    ClipEdit::MoveVertices(moves),
                )
                .isolated(),
            ),
            2 => Box::new(
                EditClip::new(
                    t.owner,
                    t.slot,
                    t.name.clone(),
                    ClipEdit::MoveVertices(moves),
                )
                .isolated(),
            ),
            3 => Box::new(
                EditPath::new(
                    t.owner,
                    t.slot,
                    t.name.clone(),
                    ClipEdit::MoveVertices(moves),
                )
                .isolated(),
            ),
            _ => return Err("Unsupported geometry".into()),
        };
        if !s.dispatch(cmd) {
            return Err("Geometry edit refused".into());
        }
        bind(s);
        Ok(())
    }
}
pub fn ui(
    ui: &mut egui::Ui,
    s: &mut AppState,
    skin: SkinId,
    slot: SlotId,
    name: &str,
    domain: Domain,
) {
    let id = ui
        .id()
        .with(("geometry_numbers", skin, slot, name, domain as u8));
    let current = Draft::new(s, skin, slot, name, domain);
    let Ok(current) = current else {
        return;
    };
    let mut draft = ui
        .ctx()
        .data(|d| d.get_temp::<Draft>(id))
        .filter(|d| d.unchanged(&current))
        .unwrap_or(current);
    ui.push_id(id, |ui| {
        ui.group(|ui| {
            let kind = match draft.target.kind { 0 => "Mesh", 1 => "Bounding box", 2 => "Clipping", _ => "Path" };
            ui.label(egui::RichText::new(format!("{kind} {name} — {} selected vertices", draft.indices.len())).strong());
            ui.label(format!("Indices: {:?}{}", &draft.indices[..draft.indices.len().min(8)], if draft.indices.len()>8 {"…"} else {""}));
            ui.label(if domain == Domain::Uv {
                "Source texture U/V [0,1] — origin top-left, V down; no implicit clamp"
            } else {
                "Setup coordinates — slot bone local, X right / Y up; independent of UVs"
            });
            ui.label("Blank axis = unchanged. Set aligns every selected point; Add offsets every selected point. No geometry snapping.");
            ui.horizontal(|ui| {
                let old = draft.relative;
                ui.selectable_value(&mut draft.relative, false, "Set");
                ui.selectable_value(&mut draft.relative, true, "Add");
                if old != draft.relative { draft.fields = [String::new(), String::new()]; }
            });
            egui::Grid::new("geometry_axes").show(ui, |ui| {
                let labels = if domain == Domain::Uv { ["U", "V"] } else { ["Local X", "Local Y"] };
                for (label, value) in labels.into_iter().zip(&mut draft.fields) {
                    let label = ui.label(label);
                    ui.add(egui::TextEdit::singleline(value).desired_width(110.0)).labelled_by(label.id);
                    ui.end_row();
                }
            });
            let valid = draft.moves().and_then(|_| draft.validate_target(s));
            if let Err(error) = &valid { ui.colored_label(ui.visuals().warn_fg_color, error); }
            ui.horizontal(|ui| {
                let label = if domain == Domain::Uv { "Apply UV coordinates" } else { "Apply coordinates" };
                if ui.add_enabled(valid.is_ok(), egui::Button::new(label)).clicked()
                    && let Err(error) = draft.apply(s) {
                    s.session.set_status(error);
                }
                if ui.button("Cancel coordinates").clicked()
                    && let Ok(reset) = Draft::new(s, skin, slot, name, domain) {
                    draft = reset;
                }
            });
        });
    });
    ui.ctx().data_mut(|d| d.insert_temp(id, draft));
}
