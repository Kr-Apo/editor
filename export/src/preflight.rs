//! Read-only diagnostics over the existing export schema. No publication here.
use ankhimate_core::assets::AssetDb;
use ankhimate_formats::schema::{self, Attachment, Project};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}
#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub subject: String,
    pub message: String,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub items: Vec<Diagnostic>,
}
impl Report {
    pub fn push(
        &mut self,
        severity: Severity,
        code: &str,
        subject: &str,
        message: impl Into<String>,
    ) {
        self.items.push(Diagnostic {
            severity,
            code: code.into(),
            subject: subject.into(),
            message: message.into(),
        });
    }
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }
    pub fn has_warnings(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Warning)
    }
    pub fn allowed(&self, accepted_warnings: bool) -> bool {
        !self.has_errors() && (!self.has_warnings() || accepted_warnings)
    }
    pub fn errors(&self) -> String {
        self.items
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .map(|d| format!("{}: {}", d.subject, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// Author-document integrity only, never a publication capability profile.
    Document,
    Runtime,
    /// Exact native template shipped before static dark RGBA transmission.
    LegacyRuntime,
    Generic,
    PhaserAtlas,
    Rendered,
    Custom,
}
impl Target {
    /// A label is not a capability declaration. Edited templates are custom.
    pub fn for_preset(preset: &crate::preset::Preset) -> Self {
        for (text, target) in [
            (crate::presets::ANKHIMATE_RUNTIME, Self::Runtime),
            (crate::presets::GENERIC_JSON, Self::Generic),
            (crate::presets::PHASER_ATLAS, Self::PhaserAtlas),
        ] {
            if crate::preset::Preset::from_json(text).is_ok_and(|p| p.templates == preset.templates)
            {
                return target;
            }
        }
        let mut legacy = crate::presets::default_preset();
        for template in &mut legacy.templates {
            template.body = template.body.replace(crate::presets::DARK_TINT_FIELD, "");
        }
        if legacy.templates == preset.templates {
            return Self::LegacyRuntime;
        }
        Self::Custom
    }

    fn is_runtime(self) -> bool {
        matches!(self, Self::Runtime | Self::LegacyRuntime)
    }
}

fn has_attachment(project: &Project, slot: &str, name: &str) -> bool {
    project.skins.iter().any(|skin| {
        skin.entries
            .iter()
            .any(|entry| entry.slot == slot && entry.name == name)
    })
}
fn mesh<'a>(
    project: &'a Project,
    skin: &str,
    slot: &str,
    name: &str,
    seen: &mut BTreeSet<(String, String, String)>,
) -> Option<&'a schema::Mesh> {
    if !seen.insert((skin.into(), slot.into(), name.into())) {
        return None;
    }
    let entry = project
        .skins
        .iter()
        .find(|s| s.name == skin)?
        .entries
        .iter()
        .find(|e| e.slot == slot && e.name == name)?;
    let Attachment::Mesh(m) = &entry.attachment else {
        return None;
    };
    if let Some(link) = &m.linked {
        let default = project.skins.first().map(|s| s.name.as_str()).unwrap_or("");
        let default = if project.default_skin.is_empty() {
            default
        } else {
            &project.default_skin
        };
        mesh(
            project,
            link.skin.as_deref().unwrap_or(default),
            &link.slot,
            &link.attachment,
            seen,
        )
    } else {
        Some(m)
    }
}
fn contains_null(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Array(v) => v.iter().any(contains_null),
        Value::Object(v) => v.values().any(contains_null),
        _ => false,
    }
}

fn inherits_animated_source(
    project: &Project,
    clip: &schema::Animation,
    link: &schema::LinkedMesh,
    seen: &mut BTreeSet<(String, String, String)>,
) -> bool {
    if !link.inherit_deform {
        return false;
    }
    let skin = link.skin.as_deref().unwrap_or(&project.default_skin);
    if !seen.insert((skin.into(), link.slot.clone(), link.attachment.clone())) {
        return false;
    }
    if clip.timelines.iter().any(|t|matches!(t,schema::Timeline::Deform{slot,attachment,keys} if slot==&link.slot&&attachment==&link.attachment&&keys.iter().any(|k|!k.offsets.is_empty()))) {return true;}
    project.skins.iter().find(|s|s.name==skin).and_then(|s|s.entries.iter().find(|e|e.slot==link.slot&&e.name==link.attachment))
        .is_some_and(|e|matches!(&e.attachment,Attachment::Mesh(m) if m.linked.as_ref().is_some_and(|next|inherits_animated_source(project,clip,next,seen))))
}

pub fn inspect(
    project: &Project,
    assets: &AssetDb,
    target: Target,
    selected: Option<&str>,
) -> Report {
    use Severity::*;
    let mut out = Report::default();
    let bones: BTreeSet<_> = project.bones.iter().map(|b| b.name.as_str()).collect();
    let slots: BTreeSet<_> = project.slots.iter().map(|s| s.name.as_str()).collect();
    for (kind, names) in [
        (
            "bone",
            project
                .bones
                .iter()
                .map(|b| b.name.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "slot",
            project.slots.iter().map(|s| s.name.as_str()).collect(),
        ),
        (
            "skin",
            project.skins.iter().map(|s| s.name.as_str()).collect(),
        ),
        (
            "clip",
            project.animations.iter().map(|a| a.name.as_str()).collect(),
        ),
        (
            "constraint",
            project
                .constraints
                .iter()
                .map(|c| c.name.as_str())
                .collect(),
        ),
    ] {
        let mut used = BTreeSet::new();
        for name in names {
            if name.is_empty() || !used.insert(name) {
                out.push(
                    Error,
                    "invalid-name",
                    name,
                    format!("Empty or duplicate {kind} name"),
                );
            }
        }
    }
    for bone in &project.bones {
        if (!bone.parent.is_empty() && !bones.contains(bone.parent.as_str()))
            || bone.parent == bone.name
        {
            out.push(Error, "missing-bone", &bone.name, "Invalid parent bone");
        }
        let mut seen = BTreeSet::new();
        let mut cursor = bone;
        while let Some(parent) = project.bones.iter().find(|b| b.name == cursor.parent) {
            if !seen.insert(&parent.name) {
                out.push(
                    Error,
                    "bone-cycle",
                    &bone.name,
                    "Bone hierarchy contains a cycle",
                );
                break;
            }
            cursor = parent;
        }
        if ![
            bone.length,
            bone.tx,
            bone.ty,
            bone.rotation,
            bone.sx,
            bone.sy,
            bone.shear_x,
            bone.shear_y,
        ]
        .iter()
        .all(|v| v.is_finite())
        {
            out.push(
                Error,
                "invalid-number",
                &bone.name,
                "Non-finite bone transform",
            );
        }
    }
    if !project.default_skin.is_empty()
        && !project.skins.iter().any(|s| s.name == project.default_skin)
    {
        out.push(
            Error,
            "missing-skin",
            &project.default_skin,
            "Default skin does not exist",
        );
    }
    for asset in &project.assets {
        if assets.by_name(&asset.name).is_none() {
            out.push(
                Error,
                "missing-image",
                &asset.name,
                "Image bytes are absent",
            );
        }
    }
    for asset in assets.images.values() {
        match image::load_from_memory(&asset.bytes) {
            Ok(image) if image.width() == asset.width && image.height() == asset.height => {}
            _ => out.push(
                Error,
                "invalid-image",
                &asset.name,
                "Image bytes cannot be decoded or dimensions disagree",
            ),
        }
    }
    for slot in &project.slots {
        if !bones.contains(slot.bone.as_str()) {
            out.push(
                Error,
                "missing-bone",
                &slot.name,
                "Slot references a missing bone",
            );
        }
        if let Some(name) = &slot.attachment
            && !has_attachment(project, &slot.name, name)
        {
            out.push(
                Error,
                "unknown-attachment",
                &slot.name,
                format!("Setup attachment `{name}` is absent"),
            );
        }
        if !slot
            .color
            .iter()
            .chain(slot.dark_color.iter().flatten())
            .all(|v| v.is_finite())
        {
            out.push(Error, "invalid-number", &slot.name, "Non-finite slot tint");
        }
        if slot.dark_color.is_some() {
            match target {
                Target::Runtime => out.push(
                    Warning,
                    "dark-tint-consumer-unknown",
                    &slot.name,
                    "Static dark RGBA is serialized. Normal blending is qualified with local runtime snapshot m0061-8888e0e (core 0.1.1 / adapter 0.1.0 / Phaser 3.90.0, exact archive hashes required). The consumer is unknown here; old or other consumers may ignore dark tint. Explicit consent is still required",
                ),
                Target::LegacyRuntime => out.push(
                    Warning,
                    "dark-tint-loss",
                    &slot.name,
                    "Known loss: this saved native template omits dark tint. Use the current Ankhimate runtime preset to transmit static dark RGBA, then use the qualified runtime snapshot; changing the consumer cannot recover omitted data",
                ),
                Target::Rendered => out.push(
                    Warning,
                    "dark-tint-unqualified",
                    &slot.name,
                    "CPU dark tint code exists, but this export feature has not been visually qualified",
                ),
                _ => {}
            }
        }
        if !slot.blend_mode.is_empty() && slot.blend_mode != "normal" {
            if target == Target::Rendered {
                out.push(
                    Error,
                    "rendered-blend",
                    &slot.name,
                    "Transparent rendered export supports Normal blending only",
                );
            } else if target.is_runtime() {
                out.push(Warning,"blend-unqualified",&slot.name,"Known CPU/Phaser composition differences exist for Add/Multiply/Screen on transparent or opaque backgrounds. Only Normal blending is in the qualified alpha scope");
            }
        }
    }
    for name in &project.draw_order {
        if !slots.contains(name.as_str()) {
            out.push(
                Error,
                "missing-slot",
                name,
                "Draw order references a missing slot",
            );
        }
    }
    for skin in &project.skins {
        if skin.bones.iter().any(|b| !bones.contains(b.as_str()))
            || skin.constraints.iter().any(|name| {
                !project
                    .constraints
                    .iter()
                    .any(|c| name == &c.name || *name == format!("{}:{}", c.kind, c.name))
            })
        {
            out.push(
                Error,
                "invalid-skin",
                &skin.name,
                "Skin gating references a missing bone or constraint",
            );
        }
        let mut names = BTreeSet::new();
        for entry in &skin.entries {
            let subject = format!("{}/{}/{}", skin.name, entry.slot, entry.name);
            if !slots.contains(entry.slot.as_str()) || !names.insert((&entry.slot, &entry.name)) {
                out.push(
                    Error,
                    "invalid-attachment",
                    &subject,
                    "Attachment slot is missing or its name is duplicated in this skin",
                );
            }
            let (texture, sequence) = match &entry.attachment {
                Attachment::Region(r) => {
                    if ![
                        r.width, r.height, r.offset_x, r.offset_y, r.rotation, r.scale_x, r.scale_y,
                    ]
                    .iter()
                    .all(|v| v.is_finite())
                        || r.width <= 0.0
                        || r.height <= 0.0
                    {
                        out.push(
                            Error,
                            "invalid-region",
                            &subject,
                            "Region geometry must be finite with positive dimensions",
                        );
                    }
                    (Some(r.texture.as_str()), r.sequence.as_ref())
                }
                Attachment::Mesh(m) => {
                    if let Some(link) = &m.linked {
                        let default_skin = if project.default_skin.is_empty() {
                            project.skins.first().map(|s| s.name.as_str()).unwrap_or("")
                        } else {
                            &project.default_skin
                        };
                        let source_skin = link.skin.as_deref().unwrap_or(default_skin);
                        let chained = project.skins.iter().find(|s| s.name == source_skin)
                            .and_then(|s| s.entries.iter().find(|e| e.slot == link.slot && e.name == link.attachment))
                            .is_some_and(|e| matches!(&e.attachment, Attachment::Mesh(source) if source.linked.is_some()));
                        if chained && (target.is_runtime() || target == Target::Rendered) {
                            out.push(Warning, "linked-chain-incompatible", &subject,
                                "Known limitation: current Rust/Phaser renderers resolve one linked-mesh level; chains may lose geometry or deformation. Linked-mesh chains are excluded from the alpha scope");
                        }
                    }
                    if let Some(g) = mesh(
                        project,
                        &skin.name,
                        &entry.slot,
                        &entry.name,
                        &mut BTreeSet::new(),
                    ) {
                        let count = g.vertices.len() / 2;
                        if g.vertices.len() % 2 != 0
                            || g.uvs.len() != g.vertices.len()
                            || g.triangles.len() % 3 != 0
                            || g.triangles.iter().any(|i| *i as usize >= count)
                            || !g.vertices.iter().chain(&g.uvs).all(|v| v.is_finite())
                            || (!g.weights.is_empty() && g.weights.len() != count)
                        {
                            out.push(
                                Error,
                                "invalid-mesh",
                                &subject,
                                "Mesh vertices, UVs, triangles or weight counts disagree",
                            );
                        }
                        for weights in &g.weights {
                            if (weights.is_empty() && target != Target::Document)
                                || weights.iter().any(|(bone, w)| {
                                    !bones.contains(bone.as_str()) || !w.is_finite() || *w < 0.0
                                })
                                || (!weights.is_empty()
                                    && weights.iter().map(|(_, w)| w).sum::<f32>() <= 0.0)
                            {
                                out.push(Error,"invalid-weights",&subject,"Missing bone, negative/non-finite weight or empty influence set");
                                break;
                            }
                        }
                    } else {
                        out.push(
                            Error,
                            "invalid-linked-mesh",
                            &subject,
                            "Linked geometry is absent, not a mesh or cyclic",
                        );
                    }
                    (Some(m.texture.as_str()), m.sequence.as_ref())
                }
                Attachment::Clipping(c) => {
                    if let Some(end) = &c.end_slot
                        && !slots.contains(end.as_str())
                    {
                        out.push(
                            Error,
                            "missing-slot",
                            &subject,
                            "Clipping end slot does not exist",
                        );
                    }
                    if target != Target::Document {
                        out.push(Warning,"clipping-unqualified",&subject,"Clipping code exists; complex clipping fidelity is not qualified for this target");
                    }
                    (None, None)
                }
                _ => (None, None),
            };
            if let Some(texture) = texture {
                for name in std::iter::once(texture).chain(
                    sequence
                        .into_iter()
                        .flat_map(|s| s.frames.iter().map(String::as_str)),
                ) {
                    if assets.by_name(name).is_none() {
                        out.push(
                            Error,
                            "missing-image",
                            &subject,
                            format!("Texture `{name}` is absent"),
                        );
                    }
                }
            }
        }
    }
    for constraint in &project.constraints {
        if constraint.kind != "path" && !bones.contains(constraint.target.as_str()) {
            out.push(
                Error,
                "missing-bone",
                &constraint.name,
                "Constraint target bone is missing",
            );
        }
        if constraint.kind == "path"
            && constraint
                .slot
                .as_ref()
                .is_none_or(|s| !slots.contains(s.as_str()))
        {
            out.push(
                Error,
                "missing-slot",
                &constraint.name,
                "Path constraint target slot is missing",
            );
        }
        if ![
            constraint.bend_direction,
            constraint.mix,
            constraint.softness,
            constraint.stretch_limit,
            constraint.stiffness,
        ]
        .iter()
        .chain(constraint.offsets.iter().flatten())
        .chain(constraint.physics.iter().flatten())
        .chain(constraint.forces.iter().flatten())
        .chain(constraint.path.iter().flatten())
        .all(|v| v.is_finite())
        {
            out.push(
                Error,
                "invalid-number",
                &constraint.name,
                "Constraint parameters must be finite",
            );
        }
        if constraint.bones.iter().any(|b| !bones.contains(b.as_str())) {
            out.push(
                Error,
                "missing-bone",
                &constraint.name,
                "Constraint references a missing bone",
            );
        }
        if constraint.kind == "physics" {
            if target == Target::Rendered {
                out.push(
                    Error,
                    "rendered-physics",
                    &constraint.name,
                    "Rendered export does not simulate physics",
                );
            } else if target.is_runtime() {
                out.push(Warning,"runtime-physics-incompatible",&constraint.name,"Known incompatibility: the native export uses target with an empty bones list, while the current TypeScript runtime reads bones[0]. Physics does not activate on real native exports; it is excluded from the alpha runtime scope");
            }
        }
        if constraint.kind == "path" && target != Target::Document {
            out.push(
                Warning,
                "path-unqualified",
                &constraint.name,
                "Advanced path fidelity is outside the alpha scope. Rust/TypeScript divergences are known at exact segment junctions and with alternate-skin path selection; do not assume portable poses",
            );
        }
        if !["ik", "transform", "physics", "path"].contains(&constraint.kind.as_str()) {
            out.push(
                Warning,
                "unknown-constraint",
                &constraint.name,
                format!(
                    "Constraint type `{}` has no established target support",
                    constraint.kind
                ),
            );
        }
    }
    if target == Target::Rendered && project.skins.len() > 1 {
        out.push(
            Warning,
            "default-skin-only",
            "skins",
            "Known scope: only the default skin is rendered; alternate skins are not exported",
        );
    }
    if target.is_runtime()
        && project
            .skins
            .iter()
            .any(|s| !s.bones.is_empty() || !s.constraints.is_empty())
    {
        out.push(Warning,"conditional-skins-unqualified","skins","Skin gating is serialized and has evaluator code; combined game-side fidelity is not qualified");
    }
    if let Some(name) = selected
        && !project.animations.iter().any(|a| a.name == name)
    {
        out.push(Error, "missing-clip", name, "Selected clip does not exist");
    }
    for clip in project
        .animations
        .iter()
        .filter(|a| selected.is_none_or(|name| name == a.name))
    {
        if target == Target::Rendered {
            for entry in project.skins.iter().flat_map(|s| &s.entries) {
                if let Attachment::Mesh(m) = &entry.attachment
                    && let Some(link) = &m.linked
                    && inherits_animated_source(project, clip, link, &mut BTreeSet::new())
                {
                    out.push(Error,"rendered-linked-deform",&clip.name,format!("Linked mesh `{}` inherits animated source deforms. The CPU renderer looks up the current attachment's deform only; this export cannot preserve that inheritance",entry.name));
                }
            }
        }
        if !clip.duration.is_finite()
            || clip.duration < 0.0
            || (target == Target::Rendered && clip.duration == 0.0)
        {
            out.push(Error,"invalid-clip",&clip.name,"Clip duration must be finite and positive for rendered export, non-negative for data");
        } else if clip.duration == 0.0 {
            out.push(
                Warning,
                "zero-duration",
                &clip.name,
                "Zero-duration data clip is readable but cannot play a timed loop",
            );
        }
        if clip.timelines.is_empty() && clip.events.is_empty() {
            out.push(
                Info,
                "static-clip",
                &clip.name,
                "Empty clip: exports the static setup pose for its duration",
            );
        }
        for offset in &clip.bone_offsets {
            if !offset.offset.is_finite() || !bones.contains(offset.bone.as_str()) {
                out.push(
                    Error,
                    "invalid-offset",
                    &clip.name,
                    "Bone time offset is non-finite or targets a missing bone",
                );
            }
        }
        for event in &clip.events {
            if !event.time.is_finite()
                || !event.float_value.is_finite()
                || !event.volume.is_finite()
                || !event.balance.is_finite()
            {
                out.push(
                    Error,
                    "invalid-event",
                    &clip.name,
                    "Event contains a non-finite number",
                );
            }
        }
        for timeline in &clip.timelines {
            let value = serde_json::to_value(timeline).expect("schema timeline serializes");
            if let Some(bone) = value["bone"].as_str()
                && !bones.contains(bone)
            {
                out.push(
                    Error,
                    "missing-bone",
                    &clip.name,
                    format!("Timeline targets missing bone `{bone}`"),
                );
            }
            if let Some(slot) = value["slot"].as_str()
                && !slots.contains(slot)
            {
                out.push(
                    Error,
                    "missing-slot",
                    &clip.name,
                    format!("Timeline targets missing slot `{slot}`"),
                );
            }
            if let Some(name) = value["constraint"].as_str()
                && !project.constraints.iter().any(|c| c.name == name)
            {
                out.push(
                    Error,
                    "missing-constraint",
                    &clip.name,
                    format!("Timeline targets missing constraint `{name}`"),
                );
            }
            let keys = value["keys"]
                .as_array()
                .expect("all timeline variants have keys");
            let mut previous = f64::NEG_INFINITY;
            for key in keys {
                match key["time"].as_f64() {
                    Some(time) if time.is_finite() && time >= previous => {
                        previous = time;
                        if time < 0.0 || time > f64::from(clip.duration) + 0.000001 {
                            out.push(Warning,"key-outside-clip",&clip.name,"Key outside the clip duration; rendered sampling will not visit it");
                        }
                    }
                    _ => out.push(
                        Error,
                        "invalid-key-time",
                        &clip.name,
                        "Key times must be finite and sorted",
                    ),
                }
                for field in [
                    "value",
                    "offsets",
                    "handles",
                    "rotate",
                    "translate_x",
                    "translate_y",
                    "scale_x",
                    "scale_y",
                    "shear_x",
                    "shear_y",
                ] {
                    if let Some(sample) = key.get(field)
                        && !(field == "value" && value["kind"] == "slot_attachment")
                        && contains_null(sample)
                    {
                        out.push(
                            Error,
                            "invalid-key-value",
                            &clip.name,
                            "Non-finite key value",
                        );
                    }
                }
                if let Some(handles) = key["handles"].as_array()
                    && (handles.len() != 4
                        || handles.iter().any(|v| v.as_f64().is_none())
                        || [0, 2].iter().any(|i| {
                            handles
                                .get(*i)
                                .and_then(Value::as_f64)
                                .is_none_or(|v| !(0.0..=1.0).contains(&v))
                        }))
                {
                    out.push(
                        Error,
                        "invalid-curve",
                        &clip.name,
                        "Bezier handles must be finite, with X in [0,1]",
                    );
                }
                if value["kind"] == "slot_attachment"
                    && let Some(name) = key["value"].as_str()
                    && !has_attachment(project, value["slot"].as_str().unwrap_or(""), name)
                {
                    out.push(
                        Error,
                        "unknown-attachment",
                        &clip.name,
                        format!("Attachment key `{name}` cannot resolve"),
                    );
                }
            }
            if let schema::Timeline::DrawOrder { keys } = timeline {
                for key in keys {
                    if key
                        .offsets
                        .iter()
                        .any(|(slot, _)| !slots.contains(slot.as_str()))
                    {
                        out.push(
                            Error,
                            "missing-slot",
                            &clip.name,
                            "Animated draw order references a missing slot",
                        );
                    }
                }
            }
            if let schema::Timeline::Deform {
                slot,
                attachment,
                keys,
            } = timeline
            {
                let geometries: Vec<_> = project
                    .skins
                    .iter()
                    .filter_map(|s| mesh(project, &s.name, slot, attachment, &mut BTreeSet::new()))
                    .collect();
                if geometries.is_empty() {
                    out.push(
                        Error,
                        "invalid-deform",
                        &clip.name,
                        "Deform track does not resolve to mesh geometry",
                    );
                }
                for geometry in geometries {
                    let expected = if geometry.weights.is_empty() {
                        geometry.vertices.len()
                    } else {
                        geometry.weights.iter().map(Vec::len).sum::<usize>() * 2
                    };
                    if keys
                        .iter()
                        .any(|k| !k.offsets.is_empty() && k.offsets.len() != expected)
                    {
                        out.push(
                            Error,
                            "invalid-deform",
                            &clip.name,
                            format!(
                                "Deform offsets require {expected} components (or an empty reset)"
                            ),
                        );
                    }
                }
                if target != Target::Document {
                    out.push(Warning,"deform-unqualified",&clip.name,"Deformation code exists; complex/weighted/linked game or rendered parity has not been qualified");
                }
            }
        }
        if target == Target::Rendered && !clip.events.is_empty() {
            out.push(
                Info,
                "events-not-rendered",
                &clip.name,
                "Images carry visual frames, not game event callbacks",
            );
        }
    }
    match target {
        Target::Generic=>out.push(Warning,"partial-format","Generic JSON","Known loss: example templates omit setup scale/shear/inheritance, slot color/dark tint/blending, skins, geometry, constraints and slot/deform/draw-order tracks; this is not a complete playable rig"),
        Target::PhaserAtlas=>out.push(Warning,"atlas-only","Phaser 3 atlas","Known scope: source texture regions only; dark tint, blending, rig/skin/constraint animation and poses are not baked. No animation playback. Use Spritesheet for baked animation"),
        Target::Custom=>out.push(Warning,"custom-unqualified","Custom/plugin templates","Target capabilities cannot be inferred from arbitrary templates or plugin labels; inspect and qualify their output"),
        _=>{},
    }
    out
}
