//! Exercise the actual Edit-menu button, then its existing operator dispatch.
use super::operator_button;
use crate::{app_state::AppState, keymap::Keymap, registry::Registry};
use eframe::egui;
#[test]
fn edit_menu_buttons_invoke_one_pose_transaction() {
    for id in ["edit.paste", "edit.paste_mirrored"] {
        let loaded = ankhimate_formats::load(std::path::Path::new("../samples/walker.ankh"))
            .unwrap()
            .0;
        let mut s = AppState::default();
        s.replace_document(ankhimate_document::Document::from_loaded(loaded));
        s.session.selected_bones = s
            .doc
            .skeleton
            .update_order
            .iter()
            .take(4)
            .copied()
            .collect();
        s.copy_pose();
        let before = ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap();
        let ctx = egui::Context::default();
        let ops = Registry::with_builtins();
        let keys = Keymap::builtin();
        let mut draw = |events| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(600., 300.),
                    )),
                    events,
                    ..Default::default()
                },
                |root| {
                    egui::CentralPanel::default().show_inside(root, |ui| {
                        if operator_button(ui, &ops, &keys, &s, id, None) {
                            ops.invoke(id, &mut s);
                        }
                    });
                },
            )
        };
        let label = ops.get(id).unwrap().label().to_owned();
        let output = draw(vec![]);
        let pos = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(t) if t.galley.text() == label => {
                    Some(t.pos + t.galley.rect.center().to_vec2())
                }
                _ => None,
            })
            .expect("real Edit-menu button missing");
        for pressed in [true, false] {
            draw(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ]);
        }
        assert_eq!(s.history.undo_depth(), 1);
        s.undo();
        assert_eq!(
            ankhimate_formats::to_json(&s.doc.as_project_ref()).unwrap(),
            before
        );
    }
}
