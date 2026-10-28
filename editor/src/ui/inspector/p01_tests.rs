use super::*;

#[test]
fn p01_animate_mesh_checkbox_can_activate_existing_ffd_tool_via_egui() {
    let loaded =
        ankhimate_formats::from_json(include_str!("../../../tests/fixtures/p01-cape.json"))
            .unwrap();
    let mut state = AppState {
        doc: ankhimate_document::Document::from_loaded(loaded),
        ..Default::default()
    };
    let slot = state.doc.skeleton.slots.keys().next().unwrap();
    state.session.select_slot(Some(slot));
    state.create_animation();
    state.set_work_mode(crate::session::WorkMode::Animate);
    state.session.mesh_edit = false;
    let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    let ctx = egui::Context::default();
    let frame = |state: &mut AppState, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(650., 1800.),
                )),
                events,
                ..Default::default()
            },
            |ui| super::attachment_inspector(ui, state, slot),
        )
    };
    let out = frame(&mut state, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                (t.galley.text() == "Edit vertices").then_some(t.visual_bounding_rect().center())
            } else {
                None
            }
        })
        .expect("existing mesh control");
    frame(&mut state, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut state,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    assert!(
        state.session.mesh_edit,
        "Animate must be able to enable the FFD vertex tool without first switching to Setup"
    );
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        before,
        "activation is session state, not topology editing"
    );
    let out = frame(&mut state, vec![]);
    let pos = out
        .shapes
        .iter()
        .find_map(|s| {
            if let egui::Shape::Text(t) = &s.shape {
                (t.galley.text() == "Key setup FFD").then_some(t.visual_bounding_rect().center())
            } else {
                None
            }
        })
        .expect("explicit neutral FFD key control");
    frame(&mut state, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            &mut state,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    let id = state.session.active_animation.unwrap();
    assert!(state.doc.animations[id].timelines.iter().any(|t|matches!(t,ankhimate_core::animation::Timeline::Deform{keys,..} if keys.len()==1 && keys[0].value.len()>116 && keys[0].value.iter().all(|v|*v==glam::Vec2::ZERO))));
    let after = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
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
