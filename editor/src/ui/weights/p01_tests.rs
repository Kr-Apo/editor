use super::*;

fn cape() -> AppState {
    let loaded =
        ankhimate_formats::from_json(include_str!("../../../tests/fixtures/p01-cape.json"))
            .unwrap();
    let mut state = AppState {
        doc: ankhimate_document::Document::from_loaded(loaded),
        ..Default::default()
    };
    let slot = state
        .doc
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "cape_slot")
        .unwrap()
        .0;
    state.session.active_skin = state.doc.skeleton.default_skin;
    state.session.select_slot(Some(slot));
    state.refresh_pose();
    state.rebind_meshes();
    state
}

#[test]
fn p01_auto_weight_disconnected_cape_refuses_without_document_or_history_changes() {
    let mut state = cape();
    let (target, mesh) = target(&state).unwrap();
    assert_eq!(mesh.setup_vertices.len(), 116);
    let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    auto_weight_mesh(&mut state, &target, &mesh, &[], &[]);
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        before,
        "Auto Weight must not publish empty influence rows on unreachable islands"
    );
    assert!(!state.history.can_undo());
    let status = state.session.status.as_deref().unwrap();
    for detail in ["default", "cape_slot", "cape", "78", "79", "80"] {
        assert!(status.contains(detail), "missing {detail}: {status}");
    }
    auto_weight_mesh(&mut state, &target, &mesh, &[], &[0]);
    let changed = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    state.undo();
    assert!(state.history.can_redo());
    let (_, restored) = super::target(&state).unwrap();
    auto_weight_mesh(&mut state, &target, &restored, &[], &[]);
    assert!(
        state.history.can_redo(),
        "Refusal must preserve existing Redo"
    );
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        before
    );
    state.redo();
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        changed
    );
}

#[test]
fn p01_auto_weight_selected_connected_vertex_is_one_undoable_edit() {
    let mut state = cape();
    let (target, mesh) = target(&state).unwrap();
    let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    auto_weight_mesh(&mut state, &target, &mesh, &[], &[0]);
    let after = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    assert_ne!(before, after);
    let (_, result) = super::target(&state).unwrap();
    let rows = |m: &MeshAttachment| {
        m.weights[1..]
            .iter()
            .map(|r| r.iter().map(|w| (w.bone, w.weight)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    assert_eq!(rows(&mesh), rows(&result));
    for _ in 0..3 {
        state.undo();
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
        state.redo();
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            after
        );
    }
}
