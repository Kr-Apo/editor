use super::*;
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    attachment::{Attachment, FfdKeyframe, LinkedMesh, MeshAttachment},
};
use ankhimate_document::snapshot::snapshot;
#[allow(
    dead_code,
    reason = "Shared native fixture helpers also cover geometry-only integration cases"
)]
mod support {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/retrace_fixture.rs"
    ));
}
fn fixture(weighted: bool) -> (AppState, PendingTrace, meshgen::Traced) {
    let (d, sk, sl) = support::fixture(weighted);
    let mut s = AppState::default();
    s.replace_document(d);
    s.session.active_skin = sk;
    s.session.select_slot(Some(sl));
    s.session.mesh_edit = true;
    s.session.selected_vertices = vec![0, 1];
    crate::geometry_edit::bind(&mut s);
    let p = PendingTrace::new(&s, sk, sl, "mesh".into());
    let t = meshgen::Traced {
        contours: vec![vec![
            glam::vec2(0.1, 0.1),
            glam::vec2(0.1, 0.9),
            glam::vec2(0.9, 0.9),
            glam::vec2(0.9, 0.1),
        ]],
        interior: vec![glam::Vec2::splat(0.5)],
    };
    (s, p, t)
}
#[test]
fn t823_editor_apply_has_one_undo_and_three_exact_cycles() {
    let (mut s, p, t) = fixture(true);
    let before = snapshot(&s.doc);
    assert!(apply_trace(&mut s, &p, &t).unwrap());
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 1);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t823_editor_second_application_is_rejected_without_history_change() {
    let (mut s, p, t) = fixture(true);
    apply_trace(&mut s, &p, &t).unwrap();
    let before = snapshot(&s.doc);
    assert!(apply_trace(&mut s, &p, &t).is_err());
    assert_eq!(s.history.undo_depth(), 1);
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t823_editor_locks_on_slot_or_influence_refuse_before_dispatch() {
    for index in 0..2 {
        let (mut s, p, t) = fixture(true);
        let bone = support::mesh(&s.doc, p.skin, p.slot).weights[0][index].bone;
        s.session.locked_bones.insert(bone, true);
        let before = snapshot(&s.doc);
        assert!(apply_trace(&mut s, &p, &t).is_err());
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.history.undo_depth(), 0);
    }
}
#[test]
fn t823_editor_mode_skin_selection_and_identity_changes_make_preview_stale() {
    for case in 0..6 {
        let (mut s, p, t) = fixture(true);
        match case {
            0 => s.session.work_mode = ankhimate_document::WorkMode::Animate,
            1 => s.session.selected_slots.clear(),
            2 => s.session.active_skin = Default::default(),
            3 => s.replace_document(s.doc.clone()),
            4 => s.doc.meta.name = "newer".into(),
            _ => {
                s.session.selection = None;
            }
        }
        let before = snapshot(&s.doc);
        assert!(apply_trace(&mut s, &p, &t).is_err(), "case {case}");
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.history.undo_depth(), 0);
    }
}
#[test]
fn t823_editor_invalid_geometry_refusal_preserves_document_and_redo() {
    let (mut s, p, t) = fixture(true);
    apply_trace(&mut s, &p, &t).unwrap();
    s.undo();
    let before = snapshot(&s.doc);
    let p = PendingTrace::new(&s, p.skin, p.slot, "mesh".into());
    let t = meshgen::Traced {
        contours: vec![],
        interior: vec![],
    };
    assert!(apply_trace(&mut s, &p, &t).is_err());
    assert!(s.history.can_redo());
    assert!(snapshot(&s.doc) == before);
    s.redo();
}
#[test]
fn t823_geometry_draft_cannot_reuse_old_vertex_indices_after_retrace() {
    let (mut s, p, t) = fixture(true);
    let mut draft = crate::geometry_edit::Draft::new(
        &mut s,
        p.skin,
        p.slot,
        "mesh",
        crate::geometry_edit::Domain::Position,
    )
    .unwrap();
    draft.fields[0] = "9".into();
    apply_trace(&mut s, &p, &t).unwrap();
    assert!(s.session.selected_vertices.is_empty());
    let before = snapshot(&s.doc);
    assert!(draft.apply(&mut s).is_err());
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.history.undo_depth(), 1);
}
#[test]
fn t823_painting_after_retrace_uses_existing_command_and_native_binds() {
    let (mut s, p, t) = fixture(true);
    apply_trace(&mut s, &p, &t).unwrap();
    let before = snapshot(&s.doc);
    let mut weights = support::mesh(&s.doc, p.skin, p.slot).weights.clone();
    weights[0][0].weight = 0.6;
    weights[0][1].weight = 0.4;
    assert!(s.dispatch(Box::new(
        ankhimate_document::commands::weight_cmds::PaintWeights::new(
            p.skin, p.slot, "mesh", weights
        )
    )));
    let after = snapshot(&s.doc);
    assert_eq!(s.history.undo_depth(), 2);
    for _ in 0..3 {
        s.undo();
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert!(snapshot(&s.doc) == after);
    }
}
#[test]
fn t823_diagnostics_ticket_is_stale_after_retrace_and_refreshes() {
    let (mut s, p0, t) = fixture(true);
    s.doc
        .skeleton
        .add_skin(ankhimate_core::skin::Skin::new("empty"));
    let p = PendingTrace::new(&s, p0.skin, p0.slot, "mesh".into());
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::ZERO);
    let i = s
        .diagnostics
        .issues
        .iter()
        .position(|i| i.code == "empty-skin")
        .unwrap();
    let ticket = s.diagnostics.ticket(i).unwrap();
    apply_trace(&mut s, &p, &t).unwrap();
    assert!(!crate::diagnostics::navigate(&mut s, &ticket));
    s.diagnostics
        .poll(&s.doc, s.revision, std::time::Duration::from_secs(1));
    assert!(!s.diagnostics.stale());
}
fn frame(ctx: &egui::Context, s: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    crate::ui::motion::configure(ctx, true);
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 1600.),
            )),
            events,
            ..Default::default()
        },
        |ui| super::ui(ui.ctx(), s, &crate::theme::Theme::default()),
    )
}
fn click(ctx: &egui::Context, s: &mut AppState, text: &str) {
    for _ in 0..30 {
        frame(ctx, s, vec![]);
    }
    let mut pos = None;
    for _ in 0..5 {
        let out = frame(ctx, s, vec![]);
        pos = out.shapes.iter().find_map(|sh| {
            if let egui::Shape::Text(t) = &sh.shape {
                let p = t.pos + t.galley.size() * 0.5;
                (t.galley.text() == text && sh.clip_rect.contains(p)).then_some(p)
            } else {
                None
            }
        });
        if pos.is_some() {
            break;
        }
        frame(
            ctx,
            s,
            vec![
                egui::Event::PointerMoved(egui::pos2(900., 800.)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    phase: egui::TouchPhase::Move,
                    delta: egui::vec2(0., -500.),
                    modifiers: Default::default(),
                },
            ],
        );
        for _ in 0..30 {
            frame(ctx, s, vec![]);
        }
    }
    let pos = pos.unwrap_or_else(|| panic!("Missing visible action {text}"));
    frame(ctx, s, vec![egui::Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        frame(
            ctx,
            s,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            }],
        );
    }
}
#[test]
fn t823_egui_preview_is_readonly_and_cancel_preserves_selection_and_redo() {
    let (mut s, p, t) = fixture(true);
    apply_trace(&mut s, &p, &t).unwrap();
    s.undo();
    let p = PendingTrace::new(&s, p.skin, p.slot, "mesh".into());
    s.session.pending_trace = Some(p);
    let before = snapshot(&s.doc);
    let selected = s.session.selected_vertices.clone();
    let ctx = egui::Context::default();
    for _ in 0..30 {
        frame(&ctx, &mut s, vec![]);
    }
    assert!(snapshot(&s.doc) == before);
    assert!(s.history.can_redo());
    click(&ctx, &mut s, "Cancel");
    assert!(s.session.pending_trace.is_none());
    assert!(snapshot(&s.doc) == before);
    assert_eq!(s.session.selected_vertices, selected);
    assert!(s.history.can_redo());
}
#[test]
fn t823_egui_actual_ok_click_commits_topology_and_copied_weights() {
    let (mut s, p, _) = fixture(true);
    s.session.pending_trace = Some(p.clone());
    let before = snapshot(&s.doc);
    let ctx = egui::Context::default();
    click(&ctx, &mut s, "OK");
    assert!(s.session.pending_trace.is_none());
    assert_eq!(s.history.undo_depth(), 1);
    assert!(!support::mesh(&s.doc, p.skin, p.slot).weights.is_empty());
    assert!(snapshot(&s.doc) != before);
    s.undo();
    assert!(snapshot(&s.doc) == before);
}
#[test]
fn t823_egui_ffd_refusal_has_visible_reason_and_no_apply_action() {
    for weighted in [false, true] {
        let (mut s, p0, _) = fixture(weighted);
        support::mesh_mut(&mut s.doc, p0.skin, p0.slot)
            .ffd_keyframes
            .push(FfdKeyframe {
                time: 0.,
                vertex_offsets: vec![],
            });
        let p = PendingTrace::new(&s, p0.skin, p0.slot, "mesh".into());
        s.session.pending_trace = Some(p);
        let before = snapshot(&s.doc);
        let ctx = egui::Context::default();
        for _ in 0..30 {
            frame(&ctx, &mut s, vec![]);
        }
        let out = frame(&ctx, &mut s, vec![]);
        let texts: Vec<_> = out
            .shapes
            .iter()
            .filter_map(|sh| {
                if let egui::Shape::Text(t) = &sh.shape {
                    Some(t.galley.text())
                } else {
                    None
                }
            })
            .collect();
        assert!(
            texts
                .iter()
                .any(|t| t.contains("REFUSED") && t.contains("FFD"))
        );
        assert!(!texts.contains(&"OK"));
        assert!(snapshot(&s.doc) == before);
        click(&ctx, &mut s, "Cancel");
        assert!(s.session.pending_trace.is_none());
    }
}
#[test]
fn t823_egui_linked_source_and_three_deform_keys_are_readonly_refusals() {
    for link in [false, true] {
        let (mut s, p0, _) = fixture(true);
        if link {
            s.doc.skeleton.skins[p0.skin].set(
                p0.slot,
                "linked",
                Attachment::Mesh(MeshAttachment {
                    linked: Some(LinkedMesh {
                        skin: None,
                        slot: "slot".into(),
                        attachment: "mesh".into(),
                        inherit_deform: true,
                    }),
                    ..Default::default()
                }),
            );
        } else {
            let mut a = Animation::new("FFD", 3.);
            a.timelines.push(Timeline::Deform {
                slot: p0.slot,
                attachment: "mesh".into(),
                keys: vec![
                    Key::linear(0., vec![]),
                    Key::linear(1., vec![glam::Vec2::ONE; 8]),
                    Key::stepped(3., vec![]),
                ],
            });
            s.doc.animations.insert(a);
        }
        let p = PendingTrace::new(&s, p0.skin, p0.slot, "mesh".into());
        s.session.pending_trace = Some(p);
        let before = snapshot(&s.doc);
        let ctx = egui::Context::default();
        for _ in 0..30 {
            frame(&ctx, &mut s, vec![]);
        }
        let out = frame(&ctx, &mut s, vec![]);
        assert!(out.shapes.iter().any(
            |sh| matches!(&sh.shape,egui::Shape::Text(t)if t.galley.text().contains("REFUSED"))
        ));
        assert!(snapshot(&s.doc) == before);
        assert_eq!(s.history.undo_depth(), 0);
    }
}
