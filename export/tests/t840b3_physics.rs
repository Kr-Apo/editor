//! Actual native exports and per-tick ReplayV1 trajectories, no rendering.
use ankhimate_core::{
    constraints::Constraint,
    ids::BoneId,
    math::Transform,
    replay_clock::Stop,
    replay_physics::{ReplayError, ReplayFrame, ReplayInputs, ReplayPhysicsInstance},
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{Loaded, ProjectRef, convert};
use serde_json::{Value, json};
fn load(name: &str) -> Loaded {
    let text = match name {
        "force" => include_str!("fixtures/t840b3/independent.ankh.json"),
        "single" => include_str!("fixtures/t840a/single.ankh.json"),
        "parent" => include_str!("fixtures/t840a/parent.ankh.json"),
        _ => include_str!("fixtures/t840a/multiple.ankh.json"),
    };
    ankhimate_formats::from_json(text).unwrap()
}
fn rig(name: &str) -> Loaded {
    let mut d = load(if ["A", "B", "AB", "BA", "ABC"].contains(&name) {
        "force"
    } else if name == "dependent-reversed" {
        "multiple"
    } else {
        name
    });
    if ["A", "B", "AB", "BA", "ABC"].contains(&name) {
        d.skeleton.constraints.retain(|_, c| {
            let Constraint::Physics(p) = c else { panic!() };
            name.contains(p.name.chars().last().unwrap())
        });
        let alive: Vec<_> = d.skeleton.constraints.keys().collect();
        d.skeleton.constraint_order.retain(|c| alive.contains(c));
    }
    if name == "BA" || name == "dependent-reversed" {
        d.skeleton.constraint_order.reverse();
    }
    d
}
fn inputs(d: &Loaded, tick: u64, motion: bool) -> ReplayInputs {
    d.skeleton
        .update_order
        .iter()
        .map(|id| {
            let b = &d.skeleton.bones[*id];
            let mut t = b.local_transform;
            if motion && b.parent.is_none() {
                t.position.x += 50.0 + (tick % 16) as f32 * 0.25;
                t.position.y += -20.0 + ((tick / 4) % 8) as f32 * 0.125;
                t.rotation += ((tick % 8) as f32 - 4.) * 0.01;
            }
            (*id, t)
        })
        .collect()
}
fn input_json(d: &Loaded, v: &ReplayInputs) -> Value {
    json!(v.iter().map(|(id,t)|json!({"name":d.skeleton.bones[*id].name,
    "x":t.position.x,"y":t.position.y,"rotation":t.rotation,"scaleX":t.scale.x,"scaleY":t.scale.y,"shearX":t.shear.x,"shearY":t.shear.y})).collect::<Vec<_>>())
}
fn frame(d: &Loaded, f: &ReplayFrame) -> Value {
    let bones: Vec<_> = d
        .skeleton
        .update_order
        .iter()
        .map(|id| {
            let t = f.pose.locals[*id];
            let w = f.pose.world(*id);
            let values = [
                t.position.x,
                t.position.y,
                t.rotation,
                w.a,
                w.b,
                w.c,
                w.d,
                w.tx,
                w.ty,
            ];
            assert!(values.iter().all(|v| v.is_finite()));
            json!({"name":d.skeleton.bones[*id].name,"values":values})
        })
        .collect();
    let records: Vec<_> = f
        .records
        .iter()
        .map(|r| {
            let Constraint::Physics(p) = &d.skeleton.constraints[r.constraint] else {
                panic!()
            };
            let values = [
                r.rotation,
                r.position.x,
                r.position.y,
                r.rotation_velocity,
                r.position_velocity.x,
                r.position_velocity.y,
                r.anchor.x,
                r.anchor.y,
            ];
            assert!(values.iter().all(|v| v.is_finite()));
            json!({"constraint":p.name,"bone":d.skeleton.bones[r.bone].name,"values":values})
        })
        .collect();
    json!({"tick":f.tick.to_string(),"bones":bones,"records":records})
}
fn native(d: &Loaded) -> String {
    let view = ProjectRef {
        skeleton: &d.skeleton,
        animations: &d.animations,
        animation_order: &[],
        assets: &d.assets,
        name: &d.name,
        fps: d.fps,
        export_presets: &d.export_presets,
        psd_layer_paths: &d.psd_layer_paths,
    };
    let plan = run::plan(
        &convert::to_schema(&view),
        &d.assets,
        &presets::default_preset(),
    )
    .unwrap();
    plan.files
        .iter()
        .find(|f| f.path == "skeleton.json")
        .unwrap()
        .contents
        .clone()
}
fn schedule(hz: u64, hitch: bool) -> Vec<Value> {
    if hitch {
        return vec![
            json!({"op":"time","n":"1","d":"2"}),
            json!({"op":"time","n":"2","d":"4"}),
            json!({"op":"pump","budget":7}),
            json!({"op":"pump","budget":45}),
        ];
    }
    let mut v: Vec<_> = (1..=hz)
        .map(|n| json!({"op":"time","n":n.to_string(),"d":hz.to_string()}))
        .collect();
    v.push(json!({"op":"pump","budget":128}));
    v
}
fn simulate(d: &Loaded, motion: bool, ops: &[Value]) -> (Vec<Value>, Vec<Value>) {
    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &inputs(d, 0, motion)).unwrap();
    let c = s.context();
    assert_eq!(s.clock().completed, 0);
    assert!(
        s.frame().records.iter().all(|r| r.rotation == 0.
            && r.position == glam::Vec2::ZERO
            && r.rotation_velocity == 0.)
    );
    let mut trace = vec![frame(d, s.frame())];
    let mut progress = Vec::new();
    for op in ops {
        let provide =
            |t: ankhimate_core::replay_clock::ScheduledTick| Ok(inputs(d, t.index, motion));
        let observe = |f: &ReplayFrame| trace.push(frame(d, f));
        let p = if op["op"] == "time" {
            s.request_time(
                &c,
                op["n"].as_str().unwrap().parse().unwrap(),
                op["d"].as_str().unwrap().parse().unwrap(),
                provide,
                observe,
            )
        } else {
            s.pump(&c, op["budget"].as_u64().unwrap() as u32, provide, observe)
        }
        .unwrap();
        assert!(!matches!(p.stop, Stop::Failed(_)));
        assert_eq!(s.frame().tick, p.state.completed);
        progress.push(json!({"completed":p.state.completed.to_string(),"requested":p.state.requested.to_string(),"backlog":p.state.backlog.to_string(),"processed":p.processed}));
    }
    (trace, progress)
}
#[test]
fn t840b3_independent_springs_do_not_share_time() {
    let ops = schedule(144, false);
    let results: Vec<_> = ["A", "B", "AB", "BA", "ABC"]
        .iter()
        .map(|n| simulate(&rig(n), false, &ops).0)
        .collect();
    for name in ["spring-A", "spring-B"] {
        let reference = if name == "spring-A" {
            &results[0]
        } else {
            &results[1]
        };
        let rows = |trace: &Vec<Value>| {
            trace
                .iter()
                .filter_map(|row| {
                    row["records"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|r| r["constraint"] == name)
                        .map(|r| r["values"].clone())
                })
                .collect::<Vec<_>>()
        };
        for other in &results[2..] {
            assert_eq!(
                rows(reference),
                rows(other),
                "independent numerical state/anchor at every tick"
            );
        }
        let bone = if name == "spring-A" { "A" } else { "B" };
        let poses = |trace: &Vec<Value>| {
            trace
                .iter()
                .map(|row| {
                    row["bones"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|r| r["name"] == bone)
                        .unwrap()["values"]
                        .clone()
                })
                .collect::<Vec<_>>()
        };
        for other in &results[2..] {
            assert_eq!(
                poses(reference),
                poses(other),
                "independent local/world pose"
            );
        }
    }
}
#[test]
fn t840b3_cadences_hitches_and_real_native_exports() {
    let output = std::env::var_os("T840B3_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &output {
        std::fs::create_dir(p).unwrap();
    }
    let mut cases = Vec::new();
    for name in [
        "A",
        "B",
        "AB",
        "BA",
        "ABC",
        "single",
        "parent",
        "multiple",
        "dependent-reversed",
    ] {
        let d = rig(name);
        let text = native(&d);
        if let Some(p) = &output {
            std::fs::write(p.join(format!("{name}.native.json")), text.as_bytes()).unwrap();
        }
        for motion in [false, true] {
            let baseline = simulate(&d, motion, &schedule(120, false)).0;
            for hz in [30, 60, 120, 144] {
                let ops = schedule(hz, false);
                let (trace, progress) = simulate(&d, motion, &ops);
                assert_eq!(
                    trace, baseline,
                    "{name}/{motion}/{hz} regrouping changed per-tick history"
                );
                let ins: Vec<_> = (0..=120)
                    .map(|t| input_json(&d, &inputs(&d, t, motion)))
                    .collect();
                cases.push(json!({"name":format!("{name}-{motion}-{hz}"),"rig":name,"inputs":ins,"ops":ops,"trace":trace,"progress":progress}));
            }
            let ops = schedule(120, true);
            let (trace, progress) = simulate(&d, motion, &ops);
            assert_eq!(trace, baseline[..61]);
            assert_eq!(progress[0]["backlog"], "52");
            assert_eq!(progress[1]["processed"], 0);
            cases.push(json!({"name":format!("{name}-{motion}-hitch"),"rig":name,"inputs":(0..=60).map(|t|input_json(&d,&inputs(&d,t,motion))).collect::<Vec<_>>(),"ops":ops,"trace":trace,"progress":progress}));
        }
    }
    let a = simulate(&rig("multiple"), true, &schedule(120, false)).0;
    let b = simulate(&rig("dependent-reversed"), true, &schedule(120, false)).0;
    assert_ne!(
        a, b,
        "real parent-child/order effects must remain observable"
    );
    if let Some(p) = output {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&cases).unwrap()).unwrap();
    }
}
#[test]
fn t840b3_context_fault_and_reset_isolation() {
    let d = rig("AB");
    let mut a = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    let mut b = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    let old = a.context();
    let other = b.context();
    let initial = a.clock();
    assert!(matches!(
        a.request_tick(&other, 1, |_| Ok(vec![]), |_| {}),
        Err(ReplayError::Context)
    ));
    assert_eq!(a.clock(), initial);
    assert!(
        a.request_time(&old, u64::MAX, 1, |_| Ok(vec![]), |_| {})
            .is_err()
    );
    assert_eq!(a.clock(), initial);
    assert!(!a.faulted());
    b.request_tick(&other, 3, |_| Ok(vec![]), |_| {}).unwrap();
    let retained = b.clock();
    let p = a
        .request_tick(
            &old,
            4,
            |t| {
                if t.index == 3 {
                    Err("provider failed".into())
                } else {
                    Ok(vec![])
                }
            },
            |_| {},
        )
        .unwrap();
    assert!(matches!(p.stop, Stop::Failed(_)));
    assert_eq!(a.clock().completed, 2);
    assert_eq!(a.frame().tick, 2);
    assert!(a.faulted());
    assert!(matches!(
        a.pump(&old, 8, |_| Ok(vec![]), |_| {}),
        Err(ReplayError::Faulted)
    ));
    a.reset(&d.skeleton, 2, &[]).unwrap();
    assert_eq!(b.clock(), retained);
    assert_eq!(a.frame().tick, 0);
    b.request_tick(&other, 120, |_| Ok(vec![]), |_| {}).unwrap();
    b.pump(&other, 128, |_| Ok(vec![]), |_| {}).unwrap();
    assert_eq!(
        frame(&d, b.frame()),
        simulate(&d, false, &schedule(120, false)).0[120],
        "B continues exactly like an isolated instance after A resets"
    );
    assert!(matches!(
        a.request_tick(&old, 1, |_| Ok(vec![]), |_| {}),
        Err(ReplayError::Context)
    ));
    let recreated = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    assert!(matches!(
        b.request_tick(&recreated.context(), 1, |_| Ok(vec![]), |_| {}),
        Err(ReplayError::Context)
    ));
    let destroyed = a.context();
    drop(a);
    let mut replacement = ReplayPhysicsInstance::new(&d.skeleton, 2, &[]).unwrap();
    assert!(matches!(
        replacement.request_tick(&destroyed, 1, |_| Ok(vec![]), |_| {}),
        Err(ReplayError::Context)
    ));
}
#[test]
fn t840b3_references_singular_inputs_and_numerical_fault_are_protected() {
    let mut d = rig("AB");
    let key = d.skeleton.constraint_order[1];
    let Constraint::Physics(p) = d.skeleton.constraints.get_mut(key).unwrap() else {
        panic!()
    };
    let original = p.bone;
    p.bone = BoneId::default();
    assert!(ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).is_err());
    let Constraint::Physics(p) = d.skeleton.constraints.get_mut(key).unwrap() else {
        panic!()
    };
    p.bone = original;
    p.strength = f32::MAX;
    p.mass = 0.01;
    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    let c = s.context();
    let mut observed = 0;
    let result = s
        .request_tick(&c, 1, |_| Ok(vec![]), |_| observed += 1)
        .unwrap();
    assert!(matches!(result.stop, Stop::Failed(_)));
    assert_eq!(observed, 0);
    assert_eq!(s.frame().tick, 0);
    assert_eq!(s.clock().completed, 0);
    assert!(s.faulted());
    let valid = rig("AB");
    let root = valid.skeleton.update_order[0];
    let mut t = Transform::default();
    t.scale.x = 0.;
    assert!(ReplayPhysicsInstance::new(&valid.skeleton, 1, &[(root, t)]).is_err());
    assert!(s.reset(&d.skeleton, 1, &[]).is_err());
    assert!(s.faulted());
    s.reset(&valid.skeleton, 2, &[]).unwrap();
    assert!(!s.faulted());
}

#[test]
fn t840b3_exact_step_priming_and_static_skin_membership() {
    let mut d = rig("AB");
    let b = d.skeleton.constraint_order[1];
    let mut skin = ankhimate_core::skin::Skin::new("inactive");
    skin.constraints.push(b);
    d.skeleton.skins.insert(skin);
    let a = d.skeleton.constraint_order[0];
    let Constraint::Physics(p) = d.skeleton.constraints.get_mut(a).unwrap() else {
        panic!()
    };
    p.mix = 0.;
    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &inputs(&d, 0, true)).unwrap();
    assert_eq!(s.frame().records.len(), 1);
    assert_eq!(s.frame().records[0].anchor, glam::vec2(50., -20.));
    let Constraint::Physics(parameters) = &d.skeleton.constraints[a] else {
        panic!()
    };
    let push = s
        .frame()
        .pose
        .world(d.skeleton.update_order[0])
        .invert()
        .unwrap()
        .transform_vector(parameters.wind + parameters.gravity);
    let c = s.context();
    s.request_tick(&c, 1, |_| Ok(inputs(&d, 0, true)), |_| {})
        .unwrap();
    let r = &s.frame().records[0];
    let expected = (push.x / 120.) * (-0.5_f32 * 12. / 120.).exp();
    assert!((r.rotation_velocity - expected).abs() < 1e-7);
    assert!(r.rotation > 0.);
    assert_eq!(s.frame().pose.locals[r.bone].rotation, 0.);
    let mut schema: Value =
        serde_json::from_str(include_str!("fixtures/t840b3/independent.ankh.json")).unwrap();
    schema["constraints"][0]["type"] = json!("ik");
    let mixed = ankhimate_formats::from_json(&schema.to_string()).unwrap();
    assert!(ReplayPhysicsInstance::new(&mixed.skeleton, 1, &[]).is_err());
}

#[test]
fn t840b3_descriptive_cpu_cost() {
    let Some(path) = std::env::var_os("T840B3_PERFORMANCE_FILE") else {
        return;
    };
    let mut rows = Vec::new();
    for name in ["single", "ABC"] {
        let d = rig(name);
        for replay in [false, true] {
            let mut samples = Vec::new();
            for sample in 0..55 {
                let start = std::time::Instant::now();
                if replay {
                    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
                    let c = s.context();
                    s.request_tick(&c, 120, |_| Ok(vec![]), |_| {}).unwrap();
                    s.pump(&c, 120, |_| Ok(vec![]), |_| {}).unwrap();
                    std::hint::black_box(s.frame());
                } else {
                    let mut state = ankhimate_core::physics::PhysicsState::new();
                    let mut pose = ankhimate_core::pose::Pose::new();
                    for _ in 0..120 {
                        ankhimate_core::pose::evaluate_with(
                            &d.skeleton,
                            &[],
                            &mut state,
                            1. / 120.,
                            &mut pose,
                        );
                    }
                    std::hint::black_box(pose);
                }
                if sample >= 5 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(json!({"rig":name,"profile":if replay{"ReplayV1"}else{"LegacyT839"},"runs":50,"ticksPerRun":120,"medianMs":samples[25],"minMs":samples[0],"maxMs":samples[49]}));
        }
    }
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    serde_json::to_writer_pretty(file, &rows).unwrap();
}
