//! Characterization only: call the existing solvers, never replace their math.
use ankhimate_core::{
    constraints::Constraint,
    physics::PhysicsState,
    pose::{Pose, evaluate, evaluate_with},
    skin::Skin,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{Loaded, ProjectRef, convert};
use serde::{Deserialize, Serialize};
use serde_json::json;

fn load(name: &str) -> Loaded {
    let text = match name {
        "single" => include_str!("fixtures/t840a/single.ankh.json"),
        "multiple" => include_str!("fixtures/t840a/multiple.ankh.json"),
        "parent" => include_str!("fixtures/t840a/parent.ankh.json"),
        _ => panic!("unknown fixture"),
    };
    let loaded = ankhimate_formats::from_json(text).unwrap();
    assert!(loaded.report.is_clean());
    loaded
}

fn view(d: &Loaded) -> ProjectRef<'_> {
    ProjectRef {
        skeleton: &d.skeleton,
        animations: &d.animations,
        animation_order: &[],
        assets: &d.assets,
        name: &d.name,
        fps: d.fps,
        export_presets: &d.export_presets,
        psd_layer_paths: &d.psd_layer_paths,
    }
}

fn context(d: &mut Loaded) {
    let mut clip = d.animations.values().next().unwrap().clone();
    clip.name = "other".into();
    d.animations.insert(clip);
    d.skeleton.skins[d.skeleton.default_skin].constraints = d.skeleton.constraints.keys().collect();
    d.skeleton.skins.insert(Skin::new("inactive"));
}

#[derive(Clone, Serialize, Deserialize)]
struct Step {
    dt: f32,
    time: f32,
    action: String,
}
#[derive(Clone, Serialize, Deserialize)]
struct Scenario {
    name: String,
    context: bool,
    steps: Vec<Step>,
}

fn scenarios() -> Vec<Scenario> {
    let mut out = Vec::new();
    // Test-only common cadence: prime at t=0 and sample each integer tick.
    let mut canonical = vec![Step {
        dt: 0.0,
        time: 0.0,
        action: "advance".into(),
    }];
    canonical.extend((1..=120).map(|i| Step {
        dt: 1.0 / 120.0,
        time: i as f32 / 120.0,
        action: "advance".into(),
    }));
    out.push(Scenario {
        name: "canonical-120".into(),
        context: false,
        steps: canonical,
    });
    for hz in [30, 60, 120, 144] {
        let mut t = 0.0;
        let steps = (0..hz)
            .map(|_| {
                let dt = 1.0 / hz as f32;
                t += dt;
                Step {
                    dt,
                    time: t,
                    action: "advance".into(),
                }
            })
            .collect();
        out.push(Scenario {
            name: format!("rate-{hz}"),
            context: false,
            steps,
        });
    }
    for (name, deltas) in [
        ("fractional", [0.001, 0.002, 0.003, 0.004].repeat(50)),
        (
            "variable",
            [0.003, 0.017, 0.011, 0.027, 0.008, 0.034].repeat(20),
        ),
        ("hitch-zero", vec![0.0, 0.5, 0.0, 0.0, 1.0 / 60.0]),
        ("repeated-hitches", vec![0.5; 10]),
        ("long-10000", vec![1.0 / 60.0; 10_000]),
    ] {
        let mut t = 0.0;
        let steps = deltas
            .into_iter()
            .map(|dt| {
                t += dt;
                Step {
                    dt,
                    time: t,
                    action: "advance".into(),
                }
            })
            .collect();
        out.push(Scenario {
            name: name.into(),
            context: false,
            steps,
        });
    }
    for action in [
        "mix-off",
        "inactive",
        "pause",
        "seek",
        "play",
        "crossfade",
        "reset",
        "replace",
    ] {
        let mut steps = Vec::new();
        let mut t = 0.0;
        for _ in 0..20 {
            t += 1.0 / 60.0;
            steps.push(Step {
                dt: 1.0 / 60.0,
                time: t,
                action: "advance".into(),
            });
        }
        if ["seek", "play", "crossfade", "replace"].contains(&action) {
            t = 0.0;
        }
        steps.push(Step {
            dt: 0.0,
            time: t,
            action: action.into(),
        });
        for _ in 0..20 {
            let paused = action == "pause";
            if !paused {
                t += 1.0 / 60.0;
            }
            steps.push(Step {
                dt: 1.0 / 60.0,
                time: t,
                action: if paused { "paused" } else { "advance" }.into(),
            });
        }
        steps.push(Step {
            dt: 0.0,
            time: t,
            action: "resume".into(),
        });
        for _ in 0..20 {
            t += 1.0 / 60.0;
            steps.push(Step {
                dt: 1.0 / 60.0,
                time: t,
                action: "advance".into(),
            });
        }
        out.push(Scenario {
            name: action.into(),
            context: true,
            steps,
        });
    }
    let mut instance = out.iter().find(|s| s.name == "rate-144").unwrap().clone();
    instance.name = "instances-interleaved".into();
    out.push(instance);
    let mut steps = Vec::new();
    for _ in 0..3 {
        steps.push(Step {
            dt: 0.0,
            time: 0.0,
            action: "reset".into(),
        });
        steps.extend((1..=20).map(|i| Step {
            dt: 1.0 / 60.0,
            time: i as f32 / 60.0,
            action: "advance".into(),
        }));
    }
    out.push(Scenario {
        name: "repeated-reset".into(),
        context: false,
        steps,
    });
    out
}

fn flat(d: &Loaded, pose: &Pose) -> Vec<f32> {
    d.skeleton
        .update_order
        .iter()
        .flat_map(|&id| {
            let p = pose.locals[id];
            let w = pose.world(id);
            [
                p.position.x,
                p.position.y,
                p.rotation,
                w.a,
                w.b,
                w.c,
                w.d,
                w.tx,
                w.ty,
            ]
        })
        .collect()
}

fn replay(name: &str, scenario: &Scenario) -> Vec<Vec<f32>> {
    let mut d = load(name);
    if scenario.context {
        context(&mut d);
    }
    let mut physics = PhysicsState::new();
    let mut other_instance = PhysicsState::new();
    let mut pose = Pose::new();
    let mut traces = Vec::new();
    let mut current = 0;
    let mut outgoing = None;
    let mut fade = 0.0;
    let mut last_time = 0.0;
    for step in &scenario.steps {
        match step.action.as_str() {
            "mix-off" | "resume" => {
                for c in d.skeleton.constraints.values_mut() {
                    if let Constraint::Physics(p) = c {
                        p.mix = if step.action == "mix-off" { 0.0 } else { 1.0 };
                    }
                }
                physics.paused = false;
            }
            "pause" => physics.paused = true,
            "play" => {
                current = 1;
                outgoing = None;
            }
            "crossfade" => {
                outgoing = Some(last_time);
                current = 1;
                fade = 0.0;
            }
            "reset" => physics.reset(),
            "replace" => {
                d = load(name);
                context(&mut d);
                current = 0;
                outgoing = None;
            }
            _ => {}
        }
        let clips: Vec<_> = d.animations.values().collect();
        if scenario.name == "instances-interleaved" {
            evaluate_with(
                &d.skeleton,
                &[(clips[0], 0.75, 1.0)],
                &mut other_instance,
                0.5,
                &mut Pose::new(),
            );
        }
        let mut layers = vec![(clips[current], step.time, 1.0)];
        if let Some(t) = outgoing {
            fade += step.dt;
            let t = t + step.dt;
            outgoing = Some(t);
            let alpha = (fade / 0.5_f32).min(1.0);
            layers = vec![(clips[0], t, 1.0 - alpha), (clips[1], step.time, alpha)];
        }
        // Current Editor seek/play/replacement preview takes its pure branch.
        // Other actions call the actual stateful core. Stateful skin gating has
        // no Rust entry point: inactive intentionally reveals that API gap.
        if ["seek", "play", "replace"].contains(&step.action.as_str()) {
            evaluate(&d.skeleton, &layers, &mut pose);
        } else {
            evaluate_with(&d.skeleton, &layers, &mut physics, step.dt, &mut pose);
        }
        traces.push(flat(&d, &pose));
        last_time = step.time;
    }
    traces
}

#[test]
fn t840a_current_physics_replays_are_deterministic_and_emit_real_exports() {
    let plan = scenarios();
    let output = std::env::var_os("T840A_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &output {
        std::fs::create_dir(p).unwrap();
        std::fs::write(p.join("plan.json"), serde_json::to_vec(&plan).unwrap()).unwrap();
    }
    for name in ["single", "multiple", "parent"] {
        let mut results = Vec::new();
        for scenario in &plan {
            let a = replay(name, scenario);
            let b = replay(name, scenario);
            assert_eq!(a, b, "{name}/{} repeat", scenario.name);
            assert!(a.iter().flatten().all(|v| v.is_finite()));
            results.push(json!({"scenario":scenario.name,"trace":a}));
        }
        let trace =
            |label: &str| &results.iter().find(|r| r["scenario"] == label).unwrap()["trace"];
        assert_eq!(
            trace("rate-144"),
            trace("instances-interleaved"),
            "other instance must not alter this pose history"
        );
        let repeated = trace("repeated-reset").as_array().unwrap();
        assert_eq!(&repeated[..21], &repeated[21..42]);
        assert_eq!(&repeated[..21], &repeated[42..63]);
        if let Some(p) = &output {
            std::fs::write(
                p.join(format!("{name}.reference.json")),
                serde_json::to_vec(&results).unwrap(),
            )
            .unwrap();
            for derived in [false, true] {
                let mut d = load(name);
                if derived {
                    context(&mut d);
                }
                let native = run::plan(
                    &convert::to_schema(&view(&d)),
                    &d.assets,
                    &presets::default_preset(),
                )
                .unwrap();
                let text = &native
                    .files
                    .iter()
                    .find(|f| f.path == "skeleton.json")
                    .unwrap()
                    .contents;
                std::fs::write(
                    p.join(format!(
                        "{name}{}.native.json",
                        if derived { "-context" } else { "" }
                    )),
                    text,
                )
                .unwrap();
            }
        }
    }
}

#[test]
fn t840a_shared_remainder_couples_otherwise_independent_constraints() {
    let d = load("multiple");
    let ids: Vec<_> = d
        .skeleton
        .constraints
        .iter()
        .map(|(id, c)| {
            let Constraint::Physics(p) = c else { panic!() };
            (id, p.bone)
        })
        .collect();
    let run = |shared: bool| {
        let mut states = [PhysicsState::new(), PhysicsState::new()];
        for _ in 0..144 {
            for (i, (c, b)) in ids.iter().enumerate() {
                states[if shared { 0 } else { i }].advance(
                    *c,
                    *b,
                    1.0 / 144.0,
                    glam::Vec2::ZERO,
                    glam::vec2(10.0, -9.8),
                    0.5,
                    40.0,
                    0.5,
                    1.0,
                );
            }
        }
        ids.iter()
            .enumerate()
            .map(|(i, (c, b))| states[if shared { 0 } else { i }].offsets(*c, *b))
            .collect::<Vec<_>>()
    };
    let shared = run(true);
    let isolated = run(false);
    assert_ne!(
        shared, isolated,
        "characterize current clock coupling, not desired conformance"
    );
    if let Some(p) = std::env::var_os("T840A_COUPLING_FILE") {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(p)
            .unwrap();
        serde_json::to_writer_pretty(file, &json!({"shared":shared,"isolated":isolated})).unwrap();
    }
}

#[test]
fn t840a_instances_and_explicit_reset_are_independent() {
    let d = load("single");
    let (id, c) = d.skeleton.constraints.iter().next().unwrap();
    let Constraint::Physics(c) = c else { panic!() };
    let mut a = PhysicsState::new();
    let mut b = PhysicsState::new();
    let mut alone = PhysicsState::new();
    for _ in 0..144 {
        a.advance(
            id,
            c.bone,
            0.5,
            glam::Vec2::ZERO,
            glam::vec2(100.0, 0.0),
            0.5,
            40.0,
            0.5,
            1.0,
        );
        for s in [&mut b, &mut alone] {
            s.advance(
                id,
                c.bone,
                1.0 / 144.0,
                glam::Vec2::ZERO,
                glam::vec2(10.0, 0.0),
                0.5,
                40.0,
                0.5,
                1.0,
            );
        }
        assert_eq!(b.offsets(id, c.bone), alone.offsets(id, c.bone));
    }
    for _ in 0..3 {
        a.reset();
        assert_eq!(a.offsets(id, c.bone), (0.0, glam::Vec2::ZERO));
    }
    let before = b.offsets(id, c.bone);
    a.advance(
        id,
        c.bone,
        1.0 / 60.0,
        glam::Vec2::ZERO,
        glam::vec2(f32::INFINITY, 0.0),
        0.5,
        40.0,
        0.5,
        1.0,
    );
    assert_eq!(
        a.offsets(id, c.bone),
        (0.0, glam::Vec2::ZERO),
        "current Rust nonfinite guard"
    );
    assert_eq!(b.offsets(id, c.bone), before);
}
