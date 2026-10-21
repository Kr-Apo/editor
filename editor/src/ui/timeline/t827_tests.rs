use super::*;
use ankhimate_document::{
    Document,
    commands::key_cmds::{BoneProperty, KeyRef, TimelineAddr},
};
#[allow(
    dead_code,
    reason = "Shared differential fixture cases are unused by the display-only observer"
)]
mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t827_fixture.rs"
    ));
}
fn show(ctx: &egui::Context, s: &mut AppState, mode: usize) -> egui::FullOutput {
    show_input(ctx, s, mode, vec![], Default::default())
}
fn show_input(
    ctx: &egui::Context,
    s: &mut AppState,
    mode: usize,
    events: Vec<egui::Event>,
    modifiers: egui::Modifiers,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            events,
            modifiers,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            ..Default::default()
        },
        |ui| {
            let a = s.session.active_animation.unwrap();
            let model = model::TimelineModel::build(s, a);
            let mut view = ViewState {
                normalized: mode == 1,
                ..Default::default()
            };
            let layout = Layout {
                sheet_x0: 30.,
                px_per_sec: 260.,
                scroll_sec: 0.,
                fps: 30,
            };
            let rect = egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(600., 500.));
            let theme = crate::theme::Theme::default();
            let registry = crate::registry::Registry::with_builtins();
            let style = Style {
                theme: &theme,
                text: 12.,
                operators: &registry,
            };
            if mode < 2 {
                graph::ui(ui, s, a, &model, &view, &layout, rect, style);
            } else {
                sheet::ui(ui, s, a, &model, &mut view, &layout, rect, style);
            }
        },
    )
}
#[test]
fn t827_optional_display_comparison() {
    let Some(path) = std::env::var_os("T827_DISPLAY_RESULT") else {
        return;
    };
    let mut output = vec![];
    for n in [100, 1000, 10000] {
        let (mut d, a, r) = fixture::fixture(n, 1, false);
        if let ankhimate_core::animation::Timeline::BoneRotate { keys, .. } =
            &mut d.animations[a].timelines[0]
        {
            for k in keys {
                k.time = k.time / (n - 1) as f32 * 2.;
            }
        }
        let mut s = AppState::default();
        s.replace_document(d);
        s.session.active_animation = Some(a);
        s.session.select_bone(s.doc.skeleton.bones.keys().next());
        s.set_work_mode(crate::session::WorkMode::Animate);
        for selected in [false, true] {
            for (mode, name) in [(0, "raw"), (1, "normalize"), (2, "sheet")] {
                s.session.key_selection = Default::default();
                if selected {
                    crate::key_selection::set(&mut s, a, r.clone());
                }
                let ctx = egui::Context::default();
                std::hint::black_box(show(&ctx, &mut s, mode));
                let mut times = vec![];
                let mut signatures = vec![];
                for _ in 0..3 {
                    let t = std::time::Instant::now();
                    let out = show(&ctx, &mut s, mode);
                    times.push(t.elapsed().as_secs_f64() * 1000.);
                    let circles = out
                        .shapes
                        .iter()
                        .filter(|x| matches!(&x.shape, egui::Shape::Circle(_)))
                        .count();
                    signatures.push(circles);
                }
                output.push(serde_json::json!({"keys":n,"selected":selected,"mode":name,"displayMs":times,"logicalCircleCounts":signatures}));
            }
        }
    }
    std::fs::write(path, serde_json::to_vec_pretty(&output).unwrap()).unwrap();
}

fn input_point(p: egui::Pos2, pressed: bool, ctrl: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(p),
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers {
                ctrl,
                ..Default::default()
            },
        },
    ]
}
fn animated(n: usize) -> (AppState, ankhimate_core::ids::AnimationId, Vec<KeyRef>) {
    let (mut d, a, r) = fixture::fixture(n, 1, false);
    if let ankhimate_core::animation::Timeline::BoneRotate { keys, .. } =
        &mut d.animations[a].timelines[0]
    {
        for k in keys {
            k.time = k.time / (n - 1) as f32 * 2.;
        }
    }
    let mut s = AppState::default();
    s.replace_document(d);
    s.session.active_animation = Some(a);
    let bone = s.doc.skeleton.bones.keys().next();
    s.session.select_bone(bone);
    s.set_work_mode(crate::session::WorkMode::Animate);
    (s, a, r)
}
#[test]
fn t827_large_marquee_raw_normalize_and_ctrl_union_share_exact_refs() {
    for mode in [0, 1] {
        let (mut s, a, r) = animated(1000);
        let ctx = egui::Context::default();
        let rev = s.revision;
        show(&ctx, &mut s, mode);
        for additive in [false, true] {
            let mods = egui::Modifiers {
                ctrl: additive,
                ..Default::default()
            };
            show_input(
                &ctx,
                &mut s,
                mode,
                input_point(egui::pos2(16., 51.), true, additive),
                mods,
            );
            show_input(
                &ctx,
                &mut s,
                mode,
                vec![egui::Event::PointerMoved(egui::pos2(614., 549.))],
                mods,
            );
            show_input(
                &ctx,
                &mut s,
                mode,
                input_point(egui::pos2(614., 549.), false, additive),
                mods,
            );
            assert_eq!(crate::key_selection::read(&mut s, a), r);
            assert_eq!(s.revision, rev);
            assert_eq!(s.history.undo_depth(), 0);
        }
    }
}
#[test]
fn t827_large_marquee_stale_document_and_revision_cannot_publish() {
    for change in 0..3 {
        let (mut s, a, _) = animated(1000);
        let ctx = egui::Context::default();
        show(&ctx, &mut s, 1);
        show_input(
            &ctx,
            &mut s,
            1,
            input_point(egui::pos2(16., 51.), true, false),
            Default::default(),
        );
        show_input(
            &ctx,
            &mut s,
            1,
            vec![egui::Event::PointerMoved(egui::pos2(614., 549.))],
            Default::default(),
        );
        match change {
            0 => s.revision += 1,
            1 => {
                let d = s.doc.clone();
                s.replace_document(d);
                s.session.active_animation = Some(a);
            }
            _ => {
                let clip = s.doc.animations[a].clone();
                let b = s.doc.animations.insert(clip);
                s.session.active_animation = Some(b);
            }
        }
        show_input(
            &ctx,
            &mut s,
            1,
            input_point(egui::pos2(614., 549.), false, false),
            Default::default(),
        );
        let current = s.session.active_animation.unwrap();
        assert!(crate::key_selection::read(&mut s, current).is_empty());
    }
}
