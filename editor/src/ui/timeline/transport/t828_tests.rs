use super::*;
fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            ui.horizontal(|ui| super::ui(ui, s));
        },
    )
}
fn numeric(ctx: &egui::Context, s: &mut AppState, text: &str) -> egui::Pos2 {
    let out = frame(ctx, s, vec![]);
    out.shapes
        .iter()
        .find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                (t.galley.text() == text).then_some(t.pos + t.galley.size() * 0.5)
            } else {
                None
            }
        })
        .expect("visible fps number")
}
fn key(k: egui::Key) -> egui::Event {
    egui::Event::Key {
        key: k,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }
}
fn edit(ctx: &egui::Context, s: &mut AppState, text: &str) {
    let p = numeric(ctx, s, &s.doc.meta.fps.to_string());
    frame(ctx, s, vec![egui::Event::PointerMoved(p)]);
    for pressed in [true, false] {
        frame(
            ctx,
            s,
            vec![egui::Event::PointerButton {
                pos: p,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
    frame(ctx, s, vec![]);
    frame(ctx, s, vec![egui::Event::Text(text.into())]);
}
#[test]
fn t828_keyboard_fps_commit_enters_native_history() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    edit(&ctx, &mut s, "60");
    frame(&ctx, &mut s, vec![key(egui::Key::Enter)]);
    assert_eq!(s.doc.meta.fps, 60);
    assert_eq!(
        s.history.undo_depth(),
        1,
        "FPS change must be a native Undo entry"
    );
    s.undo();
    assert_eq!(s.doc.meta.fps, 30);
    s.redo();
    assert_eq!(s.doc.meta.fps, 60);
}

use ankhimate_document::{commands::document_cmds::SetProjectFps, snapshot::snapshot};
fn set(s: &mut AppState, fps: u32) {
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, fps).unwrap().unwrap()));
}
fn pointer(p: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: p,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Default::default(),
    }
}
fn finish(ctx: &egui::Context, s: &mut AppState) {
    frame(ctx, s, vec![key(egui::Key::Enter)]);
    frame(
        ctx,
        s,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
}
#[test]
fn t828_draft_is_not_document_and_escape_preserves_redo() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    set(&mut s, 60);
    s.undo();
    let rev = s.revision;
    let old = snapshot(&s.doc);
    edit(&ctx, &mut s, "120");
    assert!(snapshot(&s.doc) == old);
    assert_eq!(s.revision, rev);
    frame(&ctx, &mut s, vec![key(egui::Key::Escape)]);
    frame(&ctx, &mut s, vec![]);
    assert!(snapshot(&s.doc) == old);
    assert_eq!(s.revision, rev);
    assert_eq!(s.history.undo_depth(), 0);
    assert!(s.history.can_redo());
    assert!(numeric(&ctx, &mut s, "30").x.is_finite());
}
#[test]
fn t828_invalid_keyboard_values_do_not_clamp_or_clear_redo() {
    for value in ["0", "-1", "241", "4294967296", "NaN", "inf", "29.5", "oops"] {
        let mut s = AppState::default();
        let ctx = egui::Context::default();
        set(&mut s, 60);
        s.undo();
        let rev = s.revision;
        let old = snapshot(&s.doc);
        edit(&ctx, &mut s, value);
        finish(&ctx, &mut s);
        assert!(snapshot(&s.doc) == old, "{value}");
        assert_eq!(s.revision, rev, "{value}");
        assert!(s.history.can_redo(), "{value}");
        assert_eq!(s.history.undo_depth(), 0);
    }
}
#[test]
fn t828_blur_commits_once_and_independent_edits_stay_separate() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    edit(&ctx, &mut s, "60");
    let outside = egui::pos2(1400., 400.);
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(outside), pointer(outside, true)],
    );
    frame(&ctx, &mut s, vec![pointer(outside, false)]);
    for _ in 0..3 {
        frame(&ctx, &mut s, vec![]);
    }
    assert_eq!(s.doc.meta.fps, 60);
    assert_eq!(s.history.undo_depth(), 1);
    assert_eq!(s.revision, 1);
    edit(&ctx, &mut s, "120");
    finish(&ctx, &mut s);
    assert_eq!(s.history.undo_depth(), 2);
    assert_eq!(s.revision, 2);
    s.undo();
    assert_eq!(s.doc.meta.fps, 60);
    s.undo();
    assert_eq!(s.doc.meta.fps, 30);
}
#[test]
fn t828_same_value_and_idle_legacy_display_are_read_only() {
    for fps in [0, 30, 1000, u32::MAX] {
        let mut s = AppState::default();
        s.doc.meta.fps = fps;
        let ctx = egui::Context::default();
        let old = snapshot(&s.doc);
        for _ in 0..20 {
            frame(&ctx, &mut s, vec![]);
        }
        assert!(snapshot(&s.doc) == old);
        assert_eq!(s.revision, 0);
        assert_eq!(s.history.undo_depth(), 0);
    }
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    edit(&ctx, &mut s, "30");
    finish(&ctx, &mut s);
    assert_eq!(s.history.undo_depth(), 0);
    assert_eq!(s.revision, 0);
}
#[test]
fn t828_undo_and_document_replacement_invalidate_keyboard_drafts() {
    for replace in [false, true] {
        let mut s = AppState::default();
        let ctx = egui::Context::default();
        set(&mut s, 60);
        edit(&ctx, &mut s, "120");
        if replace {
            s.replace_document(ankhimate_document::Document::new());
        } else {
            s.undo();
        }
        let old = snapshot(&s.doc);
        let rev = s.revision;
        let depth = s.history.undo_depth();
        finish(&ctx, &mut s);
        assert!(snapshot(&s.doc) == old);
        assert_eq!(s.revision, rev);
        assert_eq!(s.history.undo_depth(), depth);
        if !replace {
            assert!(s.history.can_redo());
        }
    }
}
fn drag(ctx: &egui::Context, s: &mut AppState, moves: &[f32], cancel: bool) {
    let p = numeric(ctx, s, &s.doc.meta.fps.to_string());
    frame(ctx, s, vec![egui::Event::PointerMoved(p)]);
    frame(ctx, s, vec![pointer(p, true)]);
    for x in moves {
        frame(
            ctx,
            s,
            vec![egui::Event::PointerMoved(p + egui::vec2(*x, 0.))],
        );
        assert_eq!(s.history.undo_depth(), 0);
        assert_eq!(s.doc.meta.fps, 30);
    }
    if cancel {
        frame(ctx, s, vec![key(egui::Key::Escape)]);
    }
    frame(
        ctx,
        s,
        vec![pointer(p + egui::vec2(*moves.last().unwrap(), 0.), false)],
    );
    frame(ctx, s, vec![]);
}
#[test]
fn t828_continuous_drag_commits_once_on_release() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    drag(&ctx, &mut s, &[10., 20., 30.], false);
    assert!(s.doc.meta.fps > 30);
    assert_eq!(s.history.undo_depth(), 1);
    assert_eq!(s.revision, 1);
    let actual = s.doc.meta.fps;
    s.undo();
    assert_eq!(s.doc.meta.fps, 30);
    s.redo();
    assert_eq!(s.doc.meta.fps, actual);
}
#[test]
fn t828_drag_escape_and_return_to_start_create_no_entry() {
    for (moves, cancel) in [(vec![10., 20.], true), (vec![10., 20., 0.], false)] {
        let mut s = AppState::default();
        let ctx = egui::Context::default();
        drag(&ctx, &mut s, &moves, cancel);
        assert_eq!(s.doc.meta.fps, 30);
        assert_eq!(s.history.undo_depth(), 0);
        assert_eq!(s.revision, 0);
    }
}
#[test]
fn t828_window_focus_loss_cancels_and_hidden_control_discards_draft() {
    for hidden in [false, true] {
        let mut s = AppState::default();
        let ctx = egui::Context::default();
        edit(&ctx, &mut s, "60");
        if hidden {
            for _ in 0..2 {
                let _ = ctx.run_ui(Default::default(), |_| {});
            }
        } else {
            let _ = ctx.run_ui(
                egui::RawInput {
                    focused: false,
                    ..Default::default()
                },
                |ui| {
                    ui.horizontal(|ui| super::ui(ui, &mut s));
                },
            );
        }
        finish(&ctx, &mut s);
        assert_eq!(s.doc.meta.fps, 30);
        assert_eq!(s.history.undo_depth(), 0);
        assert_eq!(s.revision, 0);
    }
}
#[test]
fn t828_arrow_increment_decrement_are_one_keyboard_gesture() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    edit(&ctx, &mut s, "30");
    frame(&ctx, &mut s, vec![key(egui::Key::ArrowUp)]);
    frame(&ctx, &mut s, vec![key(egui::Key::ArrowDown)]);
    frame(&ctx, &mut s, vec![key(egui::Key::ArrowUp)]);
    assert_eq!(s.doc.meta.fps, 30);
    finish(&ctx, &mut s);
    assert_eq!(s.doc.meta.fps, 31);
    assert_eq!(s.history.undo_depth(), 1);
}

#[test]
fn t828_stepping_snapping_playback_keep_seconds_and_clips_intact() {
    use ankhimate_core::animation::{Animation, Key, Timeline};
    let mut s = AppState::default();
    let mut a = Animation::new("clip", 2.);
    a.timelines.push(Timeline::DrawOrder {
        keys: vec![
            Key::linear(-0.1, vec![]),
            Key::linear(0.137, vec![]),
            Key::linear(3., vec![]),
        ],
    });
    let clip = s.doc.animations.insert(a);
    let zero = s.doc.animations.insert(Animation::new("zero", 0.));
    s.session.active_animation = Some(clip);
    s.session.looping = true;
    let before = snapshot(&s.doc);
    let test_clock = |s: &mut AppState, fps: u32| {
        assert_eq!(s.doc.meta.fps, fps);
        s.set_playhead(0.);
        s.step_frames(1);
        assert!((s.session.playhead - 1. / fps as f32).abs() < 1e-7);
        s.step_frames(-2);
        assert_eq!(s.session.playhead, 0.);
        s.set_playhead(2.);
        s.step_frames(1);
        assert_eq!(s.session.playhead, 2.);
        let layout = super::super::Layout {
            sheet_x0: 0.,
            px_per_sec: 100.,
            scroll_sec: 0.,
            fps,
        };
        assert_eq!(
            layout.snap_time(0.137),
            (0.137 * fps as f32).round() / fps as f32
        );
        s.session.playing = false;
        s.set_playhead(0.137);
        assert!(!s.advance_playback(0.25));
        assert_eq!(s.session.playhead, 0.137);
        s.session.playing = true;
        s.advance_playback(0.25);
        assert!((s.session.playhead - 0.387).abs() < 1e-6);
        s.set_playhead(1.9);
        s.advance_playback(0.25);
        assert!((s.session.playhead - 0.15).abs() < 1e-6);
        s.session.looping = false;
        s.set_playhead(1.9);
        s.session.playing = true;
        s.advance_playback(0.25);
        assert_eq!(s.session.playhead, 2.);
        assert!(!s.session.playing);
        s.session.looping = true;
        s.session.active_animation = Some(zero);
        s.set_playhead(0.);
        s.step_frames(1);
        assert_eq!(s.session.playhead, 0.);
        s.session.playing = true;
        assert!(!s.advance_playback(1.));
        s.session.active_animation = Some(clip);
        s.session.playing = false;
    };
    test_clock(&mut s, 30);
    assert!(snapshot(&s.doc) == before);
    set(&mut s, 60);
    let after = snapshot(&s.doc);
    test_clock(&mut s, 60);
    assert!(snapshot(&s.doc) == after);
    s.undo();
    test_clock(&mut s, 30);
    assert!(snapshot(&s.doc) == before);
    s.redo();
    test_clock(&mut s, 60);
    assert!(snapshot(&s.doc) == after);
    let mut only_fps = s.doc.clone();
    only_fps.meta.fps = 30;
    assert!(snapshot(&only_fps) == before);
}
#[test]
fn t828_idle_transport_never_rescans_diagnostics_or_changes_selection() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::ZERO);
    let before = snapshot(&s.doc);
    let selected = s.session.selected_bones.clone();
    for t in 1..100 {
        frame(&ctx, &mut s, vec![]);
        s.diagnostics
            .poll(&s.doc, s.revision, std::time::Duration::from_millis(t));
    }
    assert_eq!(s.diagnostics.scans, 1);
    assert_eq!(s.revision, 0);
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.session.selected_bones, selected);
}
#[test]
fn t828_pointer_departure_cancels_drag_without_late_release_commit() {
    let mut s = AppState::default();
    let ctx = egui::Context::default();
    let p = numeric(&ctx, &mut s, "30");
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(p), pointer(p, true)],
    );
    frame(
        &ctx,
        &mut s,
        vec![egui::Event::PointerMoved(p + egui::vec2(20., 0.))],
    );
    frame(&ctx, &mut s, vec![egui::Event::PointerGone]);
    frame(&ctx, &mut s, vec![pointer(p, false)]);
    assert_eq!(s.doc.meta.fps, 30);
    assert_eq!(s.history.undo_depth(), 0);
    assert_eq!(s.revision, 0);
}
