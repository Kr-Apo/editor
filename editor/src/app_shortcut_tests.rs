//! B04: drive the production dispatch seam with actual egui key events.
use super::*;

fn key(modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key: egui::Key::S,
        physical_key: Some(egui::Key::S),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

#[test]
fn settings_capture_must_receive_save_chords_without_opening_a_file_dialog() {
    for modifiers in [
        egui::Modifiers::CTRL,
        egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
    ] {
        let app = AnkhimateApp::default();
        let ctx = egui::Context::default();
        ctx.memory_mut(|m| {
            m.data
                .insert_temp(egui::Id::new("keymap_capturing"), "tool.rotate".to_owned())
        });
        let _ = ctx.run_ui(
            egui::RawInput {
                modifiers,
                events: vec![key(modifiers)],
                ..Default::default()
            },
            |ui| {
                assert_eq!(app.file_shortcut(ui.ctx()), None);
                assert!(
                    ui.input(|i| i.key_pressed(egui::Key::S)),
                    "the Settings recorder still owns the key"
                );
            },
        );
    }
}

#[test]
fn save_uses_the_key_event_modifiers_even_if_control_was_released_before_the_frame() {
    for (modifiers, expected) in [
        (egui::Modifiers::CTRL, FileAction::Save),
        (
            egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
            FileAction::SaveAs,
        ),
    ] {
        let app = AnkhimateApp::default();
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                modifiers: egui::Modifiers::NONE,
                events: vec![key(modifiers)],
                ..Default::default()
            },
            |ui| {
                assert_eq!(app.file_shortcut(ui.ctx()), Some(expected.clone()));
                assert!(
                    ui.input(|i| app.config.keymap.resolve(i, false).is_empty()),
                    "Save must not also activate Scale"
                );
                assert_eq!(app.file_shortcut(ui.ctx()), None, "one press fires once");
            },
        );
    }
}

#[test]
fn pressing_control_after_a_plain_s_or_using_alt_does_not_save() {
    for modifiers in [
        egui::Modifiers::NONE,
        egui::Modifiers::CTRL | egui::Modifiers::ALT,
    ] {
        let app = AnkhimateApp::default();
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                modifiers: egui::Modifiers::CTRL,
                events: vec![key(modifiers)],
                ..Default::default()
            },
            |ui| {
                assert_eq!(app.file_shortcut(ui.ctx()), None);
            },
        );
    }
}

#[test]
fn save_chords_work_with_canvas_text_and_numeric_focus_without_editing_the_field() {
    for focus in [None, Some("bone_name"), Some("numeric_value")] {
        for (modifiers, expected) in [
            (egui::Modifiers::CTRL, FileAction::Save),
            (
                egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
                FileAction::SaveAs,
            ),
        ] {
            let app = AnkhimateApp::default();
            let ctx = egui::Context::default();
            let mut name = "unchanged".to_owned();
            let mut number = 12.0_f32;
            let _ = ctx.run_ui(
                egui::RawInput {
                    modifiers,
                    events: vec![key(modifiers)],
                    ..Default::default()
                },
                |ui| {
                    if let Some(focus) = focus {
                        ui.memory_mut(|m| m.request_focus(egui::Id::new(focus)));
                    }
                    assert_eq!(app.file_shortcut(ui.ctx()), Some(expected.clone()));
                    ui.add(egui::TextEdit::singleline(&mut name).id(egui::Id::new("bone_name")));
                    ui.push_id("numeric_value", |ui| {
                        ui.add(egui::DragValue::new(&mut number));
                    });
                    assert!(ui.input(|i| app.config.keymap.resolve(i, focus.is_some()).is_empty()));
                },
            );
            assert_eq!(name, "unchanged");
            assert_eq!(number, 12.0);
        }
    }
}

#[test]
fn a_save_shortcut_keeps_pending_confirmation_and_uses_the_existing_safe_save() {
    let mut app = AnkhimateApp::default();
    app.state.doc.meta.name = "dirty".into();
    app.run_file_action(FileAction::New);
    let pending = app.pending_file_action.clone();
    let ctx = egui::Context::default();
    let input = || egui::RawInput {
        modifiers: egui::Modifiers::CTRL,
        events: vec![key(egui::Modifiers::CTRL)],
        ..Default::default()
    };
    let _ = ctx.run_ui(input(), |ui| assert_eq!(app.file_shortcut(ui.ctx()), None));
    assert_eq!(app.pending_file_action, pending);
    assert!(app.confirm_unsaved);
    app.confirm_file_action(UnsavedChoice::Cancel, None);
    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::S,
                physical_key: Some(egui::Key::S),
                pressed: false,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |_| {},
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("shortcut.ankh");
    app.current_path = Some(path.clone());
    let _ = ctx.run_ui(input(), |ui| {
        let action = app.file_shortcut(ui.ctx());
        assert_eq!(action, Some(FileAction::Save));
        app.resolve_frame(action, false, false, None);
    });
    assert!(path.exists());
    assert!(!app.is_dirty());
    assert_eq!(app.state.doc.meta.name, "dirty");
    assert_eq!(
        app.pending_file_action, pending,
        "Save does not resume suspended New"
    );
}
