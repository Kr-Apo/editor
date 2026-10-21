//! Headless CPU observer and exact, independently pinned T828 UI comparison.
use super::*;
use ankhimate_document::{
    Document,
    commands::key_cmds::{BoneProperty, KeyRef, TimelineAddr},
    snapshot::snapshot,
};
use serde_json::json;
use std::time::Instant;
#[allow(dead_code, reason = "Reuse accepted native synthetic fixtures")]
mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t827_fixture.rs"
    ));
}
pub(crate) fn animated(
    n: usize,
    bones: usize,
    channels: bool,
) -> (AppState, ankhimate_core::ids::AnimationId, Vec<KeyRef>) {
    let (mut doc, a, refs) = fixture::fixture(n, bones, channels);
    doc.animations[a].duration = 2.;
    for t in &mut doc.animations[a].timelines {
        macro_rules! times {
            ($keys:expr) => {
                for k in $keys {
                    k.time = k.time / (n.max(2) - 1) as f32 * 2.;
                }
            };
        }
        use ankhimate_core::animation::Timeline as T;
        match t {
            T::BoneRotate { keys, .. }
            | T::BoneTranslate { keys, .. }
            | T::BoneScale { keys, .. }
            | T::BoneShear { keys, .. } => times!(keys),
            T::SlotColor { keys, .. } => times!(keys),
            _ => {}
        }
    }
    let mut s = AppState::default();
    s.replace_document(doc);
    s.session.active_animation = Some(a);
    s.session.select_bone(s.doc.skeleton.bones.keys().next());
    s.set_work_mode(crate::session::WorkMode::Animate);
    (s, a, refs)
}
pub(crate) fn show(
    ctx: &egui::Context,
    s: &mut AppState,
    mode: usize,
    old: bool,
    events: Vec<egui::Event>,
    mods: egui::Modifiers,
    partial: bool,
) -> (egui::FullOutput, Vec<String>, f64) {
    display(ctx, s, mode, old, events, mods, partial, true)
}
fn measure_show(
    ctx: &egui::Context,
    s: &mut AppState,
    mode: usize,
    old: bool,
    events: Vec<egui::Event>,
    mods: egui::Modifiers,
    partial: bool,
) -> (egui::FullOutput, Vec<String>, f64) {
    display(ctx, s, mode, old, events, mods, partial, false)
}
#[allow(
    clippy::too_many_arguments,
    reason = "The oracle can collect hitboxes outside timed render measurements"
)]
fn display(
    ctx: &egui::Context,
    s: &mut AppState,
    mode: usize,
    old: bool,
    events: Vec<egui::Event>,
    mods: egui::Modifiers,
    partial: bool,
    capture_boxes: bool,
) -> (egui::FullOutput, Vec<String>, f64) {
    let mut boxes = vec![];
    let mut model_ms = 0.;
    let out = ctx.run_ui(
        egui::RawInput {
            events,
            modifiers: mods,
            time: Some(ctx.cumulative_frame_nr() as f64 / 60.),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            ..Default::default()
        },
        |ui| {
            let a = s.session.active_animation.unwrap();
            let start = Instant::now();
            let model = model::TimelineModel::build(s, a);
            model_ms = start.elapsed().as_secs_f64() * 1000.;
            let mut view = ViewState {
                normalized: mode == 1,
                ..Default::default()
            };
            let layout = Layout {
                sheet_x0: 30.,
                px_per_sec: if partial { 600. } else { 260. },
                scroll_sec: if partial { 0.5 } else { 0. },
                fps: s.doc.meta.fps,
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
                if old {
                    t829_reference_graph::ui(ui, s, a, &model, &view, &layout, rect, style)
                } else {
                    graph::ui(ui, s, a, &model, &view, &layout, rect, style)
                }
            } else if old {
                t829_reference_sheet::ui(ui, s, a, &model, &mut view, &layout, rect, style)
            } else {
                sheet::ui(ui, s, a, &model, &mut view, &layout, rect, style)
            }
            if capture_boxes {
                for (row, data) in model.visible_rows(&|_| false).iter().enumerate() {
                    if let model::VisibleRow::Property { data, .. } = data {
                        for key in &data.keys {
                            let id = if mode < 2 {
                                ui.id().with((
                                    "graph_key",
                                    data.addr.stable_id(),
                                    data.channels.first().map_or(0, |c| c.axis),
                                    key.index,
                                ))
                            } else {
                                ui.id()
                                    .with(("tl_key", row, data.addr.stable_id(), key.index))
                            };
                            if let Some(r) = ui.ctx().read_response(id) {
                                boxes.push(format!(
                                    "{id:?}|{:?}|{:?}|{:?}|{}",
                                    r.rect,
                                    r.interact_rect,
                                    r.sense,
                                    r.enabled()
                                ));
                            }
                        }
                    }
                }
            }
        },
    );
    (out, boxes, model_ms)
}
fn state_like(s: &AppState) -> AppState {
    let mut out = AppState::default();
    out.replace_document(s.doc.clone());
    out.session.active_animation = s.session.active_animation;
    out.session.selected_bones = s.session.selected_bones.clone();
    out.set_work_mode(crate::session::WorkMode::Animate);
    out
}
pub(super) fn logical(out: &egui::FullOutput) -> String {
    format!("{:?}", out.shapes)
}
pub(super) fn pair(mut s: AppState, refs: Vec<KeyRef>, mode: usize, partial: bool) {
    let a = s.session.active_animation.unwrap();
    let mut old = state_like(&s);
    crate::key_selection::set(&mut s, a, refs.clone());
    crate::key_selection::set(&mut old, a, refs);
    let c = egui::Context::default();
    let b = egui::Context::default();
    for _ in 0..2 {
        let (x, h, _) = show(&c, &mut s, mode, false, vec![], Default::default(), partial);
        let (y, k, _) = show(
            &b,
            &mut old,
            mode,
            true,
            vec![],
            Default::default(),
            partial,
        );
        assert_eq!(logical(&x), logical(&y), "full ordered logical shapes");
        assert_eq!(h, k, "actual registered key hitboxes");
        assert!(!h.is_empty(), "oracle must actually observe key hitboxes");
        assert_eq!(
            crate::key_selection::read(&mut s, a),
            crate::key_selection::read(&mut old, a)
        );
    }
}
#[test]
fn t829_exact_t828_shapes_colors_positions_and_hitboxes() {
    for (n, count, partial) in [
        (12, 0, false),
        (12, 1, false),
        (12, 2, false),
        (100, 16, false),
        (1000, 500, false),
        (10000, 10000, false),
        (1000, 1000, true),
    ] {
        for mode in 0..3 {
            let (s, _, r) = animated(n, 1, false);
            pair(s, r.into_iter().take(count).collect(), mode, partial);
        }
    }
}
#[test]
fn t829_multiple_typed_properties_bones_slots_duplicates_and_hidden_points() {
    for mode in 0..3 {
        let (s, _, r) = animated(20, 2, true);
        let mut selection = r.iter().step_by(3).cloned().collect::<Vec<_>>();
        selection.extend(selection.clone());
        pair(s, selection, mode, true);
    }
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
#[test]
fn t829_optional_paired_cpu_observer() {
    let Some(path) = std::env::var_os("T829_MEASURE") else {
        return;
    };
    let mut rows = vec![];
    for n in [100, 1000, 5000, 10000] {
        for count in [0, 1, n / 10, n / 2, n] {
            for (mode, name) in [(0, "raw"), (1, "normalize"), (2, "sheet")] {
                let (mut s, a, r) = animated(n, 1, false);
                let mut old = state_like(&s);
                let refs = r[..count].to_vec();
                crate::key_selection::set(&mut s, a, refs.clone());
                crate::key_selection::set(&mut old, a, refs.clone());
                let snap = snapshot(&s.doc);
                let rev = s.revision;
                let depth = s.history.undo_depth();
                let new_ctx = egui::Context::default();
                let old_ctx = egui::Context::default();
                measure_show(
                    &new_ctx,
                    &mut s,
                    mode,
                    false,
                    vec![],
                    Default::default(),
                    false,
                );
                measure_show(
                    &old_ctx,
                    &mut old,
                    mode,
                    true,
                    vec![],
                    Default::default(),
                    false,
                );
                let mut before = vec![];
                let mut after = vec![];
                let mut build = vec![];
                let mut capture = vec![];
                let mut read = vec![];
                let mut shapes = 0;
                let mut hits = 0;
                for pass in 0..3 {
                    let t = Instant::now();
                    let result = ankhimate_document::clipboard::KeyClip::capture(&s.doc, a, &refs);
                    assert!(result.is_ok() || refs.is_empty());
                    let _ = std::hint::black_box(result);
                    capture.push(t.elapsed().as_secs_f64() * 1000.);
                    let t = Instant::now();
                    std::hint::black_box(crate::key_selection::read(&mut s, a));
                    read.push(t.elapsed().as_secs_f64() * 1000.);
                    let mut outputs = vec![];
                    for legacy in if pass % 2 == 0 {
                        [true, false]
                    } else {
                        [false, true]
                    } {
                        let t = Instant::now();
                        let o = if legacy {
                            measure_show(
                                &old_ctx,
                                &mut old,
                                mode,
                                true,
                                vec![],
                                Default::default(),
                                false,
                            )
                        } else {
                            measure_show(
                                &new_ctx,
                                &mut s,
                                mode,
                                false,
                                vec![],
                                Default::default(),
                                false,
                            )
                        };
                        let ms = t.elapsed().as_secs_f64() * 1000.;
                        if legacy {
                            before.push(ms)
                        } else {
                            after.push(ms);
                            build.push(o.2);
                            shapes = o.0.shapes.len();
                            hits = o.1.len();
                        }
                        outputs.push(o);
                    }
                    assert_eq!(logical(&outputs[0].0), logical(&outputs[1].0));
                    assert_eq!(outputs[0].1, outputs[1].1);
                }
                assert!(snapshot(&s.doc) == snap);
                assert_eq!(s.revision, rev);
                assert_eq!(s.history.undo_depth(), depth);
                rows.push(json!({"keys":n,"selected":count,"mode":name,"beforeMs":median(before.clone()),"afterMs":median(after.clone()),"beforeSamplesMs":before,"afterSamplesMs":after,"modelMs":median(build),"captureMs":median(capture),"selectionReadMs":median(read),"shapes":shapes,"keyHitboxes":hits,"orderedShapesAndHitboxesExact":true}));
            }
        }
    }
    std::fs::write(path,serde_json::to_vec_pretty(&json!({"profile":"debug","warmup":1,"samples":3,"pairedAlternatingOrder":true,"toolchain":"repository pinned 1.95","rows":rows})).unwrap()).unwrap();
}

#[test]
fn t829_optional_separate_stage_and_small_selection_observer() {
    let Some(path) = std::env::var_os("T829_STAGES") else {
        return;
    };
    let mut rows = vec![];
    let mut thresholds = vec![];
    for n in [100, 1000, 5000, 10000] {
        for count in [0, 1, n / 10, n / 2, n] {
            let (s, _, refs) = animated(n, 1, false);
            let selection = &refs[..count];
            let mut builds = vec![];
            let mut oldq = vec![];
            let mut newq = vec![];
            let mut oldh = vec![];
            let mut newh = vec![];
            let mut coords = vec![];
            let mut capacity = 0;
            for _ in 0..3 {
                let t = Instant::now();
                let lookup = super::membership::Membership::new(selection);
                builds.push(t.elapsed().as_secs_f64() * 1000.);
                capacity = lookup.capacity();
                let t = Instant::now();
                let old_hits = refs
                    .iter()
                    .filter(|r| std::hint::black_box(selection.contains(r)))
                    .count();
                oldq.push(t.elapsed().as_secs_f64() * 1000.);
                let t = Instant::now();
                let hits = refs
                    .iter()
                    .filter(|r| std::hint::black_box(lookup.contains(selection, r)))
                    .count();
                newq.push(t.elapsed().as_secs_f64() * 1000.);
                assert_eq!(old_hits, hits);
                let (before, shapes) = t829_reference_graph::t829_phases::handles(&s, selection);
                let (after, new_shapes) = graph::t829_phases::handles(&s, selection);
                assert_eq!(shapes, new_shapes);
                oldh.push(before);
                newh.push(after);
                let t = Instant::now();
                let model = model::TimelineModel::build(&s, s.session.active_animation.unwrap());
                let layout = Layout {
                    sheet_x0: 30.,
                    px_per_sec: 260.,
                    scroll_sec: 0.,
                    fps: 30,
                };
                for group in &model.groups {
                    for row in &group.rows {
                        for (key, value) in row.keys.iter().zip(&row.channels[0].values) {
                            std::hint::black_box(egui::pos2(
                                layout.time_to_x(key.time),
                                300. - *value,
                            ));
                        }
                    }
                }
                coords.push(t.elapsed().as_secs_f64() * 1000.);
            }
            rows.push(json!({"keys":n,"selected":count,"membershipQueries":n,"oldFullEqualityComparisons":count*(count+1)/2+(n-count)*count,"indexMs":median(builds),"oldQueryMs":median(oldq),"newQueryMs":median(newq),"oldHandlePreparationMs":median(oldh),"newHandlePreparationMs":median(newh),"modelPlusCoordinateProjectionMs":median(coords),"indexCapacity":capacity,"tupleBytes":std::mem::size_of::<(TimelineAddr,usize)>(),"indexPayloadBytesLowerBound":capacity*std::mem::size_of::<(TimelineAddr,usize)>()}));
        }
    }
    for total in [100, 10000] {
        let (_, _, refs) = animated(total, 1, false);
        for count in [0, 1, 2, 8, 16, 17, 32, 64, 100] {
            let selection = &refs[..count];
            let mut linear = vec![];
            let mut indexed = vec![];
            let mut builds = vec![];
            for _ in 0..5 {
                let t = Instant::now();
                let set = selection
                    .iter()
                    .map(|r| (r.addr.clone(), r.index))
                    .collect::<std::collections::HashSet<_>>();
                builds.push(t.elapsed().as_secs_f64() * 1000.);
                let t = Instant::now();
                let a = refs
                    .iter()
                    .filter(|r| std::hint::black_box(selection.contains(r)))
                    .count();
                linear.push(t.elapsed().as_secs_f64() * 1000.);
                let t = Instant::now();
                let b = refs
                    .iter()
                    .filter(|r| std::hint::black_box(set.contains(&(r.addr.clone(), r.index))))
                    .count();
                indexed.push(t.elapsed().as_secs_f64() * 1000.);
                assert_eq!(a, b);
            }
            thresholds.push(json!({"queries":total,"selected":count,"linearMs":median(linear),"forcedIndexBuildMs":median(builds),"forcedIndexQueryMs":median(indexed)}));
        }
    }
    std::fs::write(path,serde_json::to_vec_pretty(&json!({"profile":"debug","samples":3,"microStagesNotAdditiveToFullFrames":true,"handleProjection":"same fixed test projection, actual private handle function","rows":rows,"smallSelectionStudy":thresholds})).unwrap()).unwrap();
}
