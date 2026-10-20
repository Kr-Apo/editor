//! Shared exact native snapshot for revision-guarded read-only plans.
use crate::Document;
/// Exact native identity snapshot, constructed only at scan and explicit click.
/// Asset bytes are compared verbatim, never decoded/serialized each UI frame.
#[derive(Clone, PartialEq)]
pub struct Snapshot {
    native: serde_json::Value,
    assets: Vec<(ankhimate_core::ids::AssetId, Vec<u8>)>,
}
pub fn snapshot(doc: &Document) -> Snapshot {
    let s = &doc.skeleton;
    let skins: Vec<_> = s
        .skins
        .iter()
        .map(|(id, skin)| {
            let mut entries: Vec<_> = skin.entries.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let binds: Vec<_> = entries
                .iter()
                .filter_map(|(key, a)| {
                    if let ankhimate_core::attachment::Attachment::Mesh(m) = a {
                        let mut binds: Vec<_> = m.inverse_bind_matrices.iter().collect();
                        binds.sort_by_key(|(id, _)| **id);
                        Some((key, binds))
                    } else {
                        None
                    }
                })
                .collect();
            serde_json::json!([id, skin.name, skin.bones, skin.constraints, entries, binds])
        })
        .collect();
    Snapshot {
        native: serde_json::json!({"bones":s.bones,"slots":s.slots,"skins":skins,"default_skin":s.default_skin,"constraints":s.constraints,"constraint_order":s.constraint_order,"order":s.update_order,"draw_order":s.draw_order,"groups":s.groups,"group_order":s.group_order,"animations":doc.animations,"project":ankhimate_formats::convert::to_schema(&doc.as_project_ref())}),
        assets: doc
            .assets
            .images
            .iter()
            .map(|(id, a)| (id, a.bytes.clone()))
            .collect(),
    }
}
