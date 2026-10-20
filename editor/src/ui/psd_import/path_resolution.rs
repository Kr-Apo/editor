//! Session-only explicit mapping in the existing PSD preview.
use super::*;
use ankhimate_document::{
    WorkMode,
    psd_reimport::{PathChange, ReimportPsd},
};

#[derive(Clone, Default)]
pub struct Resolution {
    pub open: bool,
    old: Option<String>,
    new: Option<String>,
    plan: Option<std::sync::Arc<Lease>>,
    error: Option<String>,
}
#[derive(Clone)]
struct Lease {
    identity: u64,
    revision: u64,
    source_path: String,
    plan: PathChange,
}

fn locks(state: &AppState, plan: &PathChange) -> Result<(), String> {
    if state.session.work_mode != WorkMode::Setup {
        return Err("Switch to Setup mode to resolve a PSD path".into());
    }
    for skin in state.doc.skeleton.skins.values() {
        for ((slot, _), a) in &skin.entries {
            if let ankhimate_core::attachment::Attachment::Region(r) = a
                && plan.preview.affected_assets.contains(&r.texture)
                && state
                    .session
                    .is_bone_locked(state.doc.skeleton.slots[*slot].bone)
            {
                return Err("A region/slot bone in this batch is locked".into());
            }
        }
    }
    Ok(())
}

pub(super) fn prepare(state: &AppState, pending: &mut PendingPsd) -> Result<(), String> {
    pending.path_resolution.plan = None;
    if let Some(e) = &pending.preservation_error {
        return Err(e.clone());
    }
    let asset = pending
        .path_resolution
        .old
        .as_deref()
        .ok_or("Select an old image explicitly")?;
    let path = pending
        .path_resolution
        .new
        .as_deref()
        .ok_or("Select a new layer path explicitly")?;
    let source_path = pending
        .source_path
        .clone()
        .ok_or("Read the PSD file again to verify its source")?;
    if std::fs::read(&source_path).map_err(|e| format!("Cannot verify PSD source: {e}"))?
        != pending.bytes
    {
        return Err("PSD source changed; reload its preview".into());
    }
    let plan = ReimportPsd::prepare_path_change(&state.doc, &pending.bytes, asset, path)?;
    locks(state, &plan)?;
    pending.path_resolution.plan = Some(std::sync::Arc::new(Lease {
        identity: state.diagnostics.document_identity(),
        revision: state.revision,
        source_path,
        plan,
    }));
    Ok(())
}

pub(super) fn apply(state: &mut AppState, pending: &PendingPsd) -> Result<String, String> {
    let lease = pending
        .path_resolution
        .plan
        .clone()
        .ok_or("Prepare and review the explicit mapping before Apply")?;
    if lease.identity != state.diagnostics.document_identity()
        || lease.revision != state.revision
        || pending.source_path.as_ref() != Some(&lease.source_path)
        || pending.path_resolution.old.as_ref() != Some(&lease.plan.preview.asset)
        || pending.path_resolution.new.as_ref() != Some(&lease.plan.preview.new_path)
    {
        return Err("PSD path mapping is stale; prepare it again".into());
    }
    lease.plan.validate(&state.doc, &pending.bytes)?;
    locks(state, &lease.plan)?;
    let current =
        std::fs::read(&lease.source_path).map_err(|e| format!("Cannot verify PSD source: {e}"))?;
    if current != pending.bytes {
        return Err("PSD source changed after preview; reload and prepare again".into());
    }
    let lines = lease.plan.preview.lines();
    let command = lease.plan.clone().into_command(&state.doc, &current)?;
    if !state.dispatch(Box::new(command)) {
        return Err("PSD path mapping refused in the current mode".into());
    }
    state.invalidate_asset_caches();
    state.session.import_summary = Some(lines);
    Ok("PSD path change and disclosed image batch applied; rig retained".into())
}

/// Selection and Prepare are session operations; they never dispatch a command.
/// The existing preview's final Apply button owns the confirmation.
pub(super) fn ui(ui: &mut egui::Ui, state: &AppState, pending: &mut PendingPsd) {
    if ui.button("Resolve path change…").clicked() {
        pending.path_resolution = Resolution {
            open: true,
            ..Default::default()
        };
    }
    if !pending.path_resolution.open {
        return;
    }
    ui.separator();
    ui.label("One explicit simple-region mapping. Choose both paths, then Prepare. The entire exact-path image batch is included and reviewed before Apply.");
    let Some(report) = pending.reimport_report.clone() else {
        return;
    };
    egui::ScrollArea::vertical()
        .id_salt("psd_path_mapping")
        .max_height(300.)
        .show(ui, |ui| {
            ui.label("Old missing image (no automatic selection):");
            let mut old: Vec<_> = state
                .doc
                .psd_layer_paths
                .iter()
                .filter(|(_, p)| report.layers.removed.contains(p))
                .collect();
            old.sort();
            for (asset, path) in old {
                let response = ui.selectable_value(
                    &mut pending.path_resolution.old,
                    Some(asset.clone()),
                    format!("{asset} — {path}"),
                );
                if response.changed() {
                    pending.path_resolution.plan = None;
                    pending.path_resolution.error = None;
                }
            }
            ui.label("New added image path (no automatic selection):");
            for path in &report.layers.added {
                if ui
                    .selectable_value(&mut pending.path_resolution.new, Some(path.clone()), path)
                    .changed()
                {
                    pending.path_resolution.plan = None;
                    pending.path_resolution.error = None;
                }
            }
            if ui.button("Prepare path mapping").clicked() {
                pending.path_resolution.error = prepare(state, pending).err();
            }
            if let Some(error) = &pending.path_resolution.error {
                ui.colored_label(ui.visuals().error_fg_color, error);
            }
            if let Some(lease) = &pending.path_resolution.plan {
                let preview = &lease.plan.preview;
                ui.label(format!(
                    "Skin: {}; slot: {}",
                    state
                        .doc
                        .skeleton
                        .skins
                        .get(preview.skin)
                        .map(|s| s.name.as_str())
                        .unwrap_or("missing (stale plan)"),
                    state
                        .doc
                        .skeleton
                        .slots
                        .get(preview.slot)
                        .map(|s| s.name.as_str())
                        .unwrap_or("missing (stale plan)")
                ));
                for line in preview.lines() {
                    ui.label(line);
                }
            }
        });
    let response = ui.button("Cancel path mapping");
    if response.clicked() {
        pending.path_resolution = Default::default();
    }
}

pub(super) fn ready(pending: &PendingPsd) -> bool {
    pending.path_resolution.open && pending.path_resolution.plan.is_some()
}

#[cfg(test)]
mod tests;
