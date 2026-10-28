use ankhimate_core::{
    animation::{Animation, Axis, Key, Timeline},
    replay_animation::ReplayAnimationInstance,
};

fn author() -> ankhimate_formats::Loaded {
    ankhimate_formats::from_json(include_str!("fixtures/t840b3/independent.ankh.json")).unwrap()
}
fn clip(d: &ankhimate_formats::Loaded) -> Animation {
    let root = d.skeleton.update_order[0];
    let mut a = Animation::new("breathing", 1.0);
    a.timelines.push(Timeline::BoneTranslate {
        bone: root,
        axis: Axis::X,
        keys: vec![Key::linear(0., 0.), Key::linear(1., 10.)],
    });
    a
}
#[test]
fn canonical_clip_supplies_every_executed_tick() {
    let d = author();
    let a = clip(&d);
    let mut s = ReplayAnimationInstance::new(&d.skeleton, &[a], "breathing", 1).unwrap();
    let ctx = s.context();
    let mut ticks = Vec::new();
    s.request_tick(&ctx, 60, |f| ticks.push(f.tick)).unwrap();
    s.pump(&ctx, 120, |f| ticks.push(f.tick)).unwrap();
    assert_eq!(ticks, (1..=60).collect::<Vec<_>>());
    let p = s.sample_tick(60).unwrap();
    assert_eq!(p.locals[d.skeleton.update_order[0]].position.x, 5.);
}

use ankhimate_core::{
    animation::Interp, constraints::Constraint, pose::Pose, replay_animation::canonical_time,
    replay_physics::ReplayFrame,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
use serde_json::{Value, json};

fn fixture(name: &str) -> ankhimate_formats::Loaded {
    let mut d = author();
    d.name = format!("B4 {name}");
    let root = d.skeleton.update_order[0];
    let mut torso = d.skeleton.bones[root].clone();
    torso.name = "torso".into();
    torso.parent = Some(root);
    torso.local_transform.position = glam::vec2(0., 20.);
    if name != "idle" {
        torso.local_transform.rotation = 0.4;
        torso.local_transform.scale = glam::vec2(1.75, 0.6);
    }
    let torso = d.skeleton.bones.insert(torso);
    let children = d.skeleton.update_order[1..].to_vec();
    for id in &children {
        d.skeleton.bones[*id].parent = Some(torso);
        d.skeleton.bones[*id].length = 300.;
    }
    d.skeleton.update_order.insert(1, torso);
    if name == "parent" {
        d.skeleton.bones[root].local_transform.scale = glam::vec2(3., 1.5);
    }
    if ["A", "B", "AB", "ABC", "CBA"].contains(&name) {
        d.skeleton.constraints.retain(|_, c| {
            let Constraint::Physics(p) = c else { panic!() };
            name.contains(p.name.chars().last().unwrap())
        });
    } else {
        d.skeleton.constraints.retain(|_, c| {
            let Constraint::Physics(p) = c else { panic!() };
            p.name == "spring-A"
        });
    }
    d.skeleton
        .constraint_order
        .retain(|c| d.skeleton.constraints.contains_key(*c));
    if name == "CBA" {
        d.skeleton.constraint_order.reverse();
    }
    let mut a = Animation::new("breathing", if name == "loop" { 0.83 } else { 1. });
    a.looping = name != "parent";
    let mut tx = vec![
        Key::linear(0., 0.),
        Key::linear(0.25, 1.5),
        Key::linear(0.75, -0.5),
        Key::linear(1., 0.),
    ];
    tx[1].interp = Interp::Bezier {
        out_handle: glam::vec2(0.25, -0.2),
        in_handle: glam::vec2(0.75, 1.2),
    };
    a.timelines = vec![
        Timeline::BoneTranslate {
            bone: torso,
            axis: Axis::X,
            keys: tx,
        },
        Timeline::BoneTranslate {
            bone: torso,
            axis: Axis::Y,
            keys: vec![
                Key::linear(0., 0.),
                Key::stepped(0.5, 0.4),
                Key::stepped(0.8, -0.2),
                Key::linear(1., 0.),
            ],
        },
        Timeline::BoneRotate {
            bone: torso,
            keys: vec![
                Key::linear(0., 0.),
                Key::linear(0.5, 3.),
                Key::linear(1., 0.),
            ],
        },
        Timeline::BoneScale {
            bone: torso,
            axis: Axis::X,
            keys: vec![Key::linear(0.1, 1.), Key::linear(0.6, 1.02)],
        },
        Timeline::BoneScale {
            bone: torso,
            axis: Axis::Y,
            keys: vec![Key::linear(0., 1.), Key::linear(0.7, 0.98)],
        },
        Timeline::BoneShear {
            bone: torso,
            axis: Axis::X,
            keys: vec![Key::linear(0., 0.), Key::linear(0.75, 1.)],
        },
        Timeline::BoneShear {
            bone: torso,
            axis: Axis::Y,
            keys: vec![Key::linear(0., 0.), Key::linear(0.9, -1.)],
        },
    ];
    if name != "idle" {
        a.timelines.push(Timeline::BoneTranslate {
            bone: root,
            axis: Axis::X,
            keys: vec![
                Key::linear(0., 50.),
                Key::linear(0.5, 52.),
                Key::linear(1., 50.),
            ],
        });
        a.timelines.push(Timeline::BoneRotate {
            bone: root,
            keys: vec![
                Key::linear(0., 0.),
                Key::linear(0.5, 5.),
                Key::linear(1., 0.),
            ],
        });
    }
    if name == "angles" {
        let Timeline::BoneRotate { keys, .. } = a
            .timelines
            .iter_mut()
            .find(|t| matches!(t,Timeline::BoneRotate{bone,..}if *bone==root))
            .unwrap()
        else {
            panic!()
        };
        *keys = vec![
            Key::linear(0., 0.),
            Key::linear(0.25, -180.),
            Key::linear(0.5, 0.),
            Key::linear(0.75, 180.),
            Key::linear(1., 0.),
        ];
    }
    d.animations.clear();
    d.animations.insert(a);
    d
}
fn view(d: &ankhimate_formats::Loaded) -> ProjectRef<'_> {
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
fn clips(d: &ankhimate_formats::Loaded) -> Vec<Animation> {
    d.animations.values().cloned().collect()
}
fn pose_json(d: &ankhimate_formats::Loaded, p: &Pose) -> Value {
    json!(d.skeleton.update_order.iter().map(|id|{
        let t=p.locals[*id];let w=p.world(*id);let tip=p.world_tip(&d.skeleton,*id);
        json!({"name":d.skeleton.bones[*id].name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty,tip.x,tip.y]})
    }).collect::<Vec<_>>())
}
fn frame_json(d: &ankhimate_formats::Loaded, f: &ReplayFrame) -> Value {
    json!({"tick":f.tick.to_string(),"bones":pose_json(d,&f.pose),"records":f.records.iter().map(|r|json!({"constraint":d.skeleton.constraints[r.constraint].name(),"values":[r.rotation,r.position.x,r.position.y,r.rotation_velocity,r.position_velocity.x,r.position_velocity.y,r.anchor.x,r.anchor.y]})).collect::<Vec<_>>()})
}
fn schedule(hz: u64) -> Vec<Value> {
    let mut v = if hz == 0 {
        (1..=8)
            .flat_map(|n| {
                [
                    json!({"op":"time","n":n.to_string(),"d":"2"}),
                    json!({"op":"time","n":(2*n).to_string(),"d":"4"}),
                    json!({"op":"pump","budget":7}),
                    json!({"op":"pump","budget":45}),
                ]
            })
            .collect()
    } else {
        (1..=4 * hz)
            .map(|n| json!({"op":"time","n":n.to_string(),"d":hz.to_string()}))
            .collect::<Vec<_>>()
    };
    v.push(json!({"op":"pump","budget":1024}));
    v
}
fn simulate(d: &ankhimate_formats::Loaded, ops: &[Value]) -> (Vec<Value>, Vec<Value>) {
    let mut s = ReplayAnimationInstance::new(&d.skeleton, &clips(d), "breathing", 1).unwrap();
    let c = s.context();
    let mut trace = vec![frame_json(d, s.frame())];
    let mut progress = vec![];
    for o in ops {
        let observer = |f: &ReplayFrame| trace.push(frame_json(d, f));
        let p = if o["op"] == "time" {
            s.request_time(
                &c,
                o["n"].as_str().unwrap().parse().unwrap(),
                o["d"].as_str().unwrap().parse().unwrap(),
                observer,
            )
        } else {
            s.pump(&c, o["budget"].as_u64().unwrap() as u32, observer)
        }
        .unwrap();
        assert!(!matches!(
            p.stop,
            ankhimate_core::replay_clock::Stop::Failed(_)
        ));
        progress.push(json!({"completed":p.state.completed.to_string(),"requested":p.state.requested.to_string(),"backlog":p.state.backlog.to_string(),"processed":p.processed}));
    }
    assert_eq!(s.clock().completed, 480);
    (trace, progress)
}
#[test]
fn author_native_tick_parity_campaign() {
    let dir = std::env::var_os("T840B4_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &dir {
        std::fs::create_dir(p).unwrap();
    }
    let mut cases = vec![];
    for name in [
        "idle", "parent", "loop", "A", "B", "AB", "ABC", "CBA", "angles",
    ] {
        let d = fixture(name);
        let bytes = ankhimate_formats::to_json(&view(&d)).unwrap();
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("tests/fixtures/t840b4/{name}.ankh.json"));
        if source.exists() {
            assert_eq!(
                bytes,
                std::fs::read_to_string(source).unwrap(),
                "author fixture drift"
            );
        }
        let d = ankhimate_formats::from_json(&bytes).unwrap();
        assert!(d.report.dangling.is_empty() && d.report.lossy.is_empty());
        let native = run::plan(
            &convert::to_schema(&view(&d)),
            &d.assets,
            &presets::default_preset(),
        )
        .unwrap()
        .files
        .into_iter()
        .find(|f| f.path == "skeleton.json")
        .unwrap()
        .contents;
        if let Some(p) = &dir {
            std::fs::write(p.join(format!("{name}.ankh.json")), bytes).unwrap();
            std::fs::write(p.join(format!("{name}.native.json")), native).unwrap();
        }
        let s = ReplayAnimationInstance::new(&d.skeleton, &clips(&d), "breathing", 1).unwrap();
        let pure: Vec<_> = (0..=480)
            .map(|t| pose_json(&d, &s.sample_tick(t).unwrap()))
            .collect();
        let mut reference = None;
        for hz in [30, 60, 120, 144, 0] {
            let ops = schedule(hz);
            let (trace, progress) = simulate(&d, &ops);
            if let Some(old) = &reference {
                assert_eq!(&trace, old, "grouping must not change any pose");
            } else {
                reference = Some(trace.clone());
            }
            cases.push(json!({"name":format!("{name}/{hz}"),"rig":name,"ops":ops,"pure":pure,"trace":trace,"progress":progress}));
        }
        // Independent end-to-end check of the optimized provider: feeding the
        // full pure evaluator's locals to B3 must reproduce every wrapper frame.
        let pure_inputs = |tick| {
            let p = s.sample_tick(tick).unwrap();
            d.skeleton
                .update_order
                .iter()
                .map(|id| (*id, p.locals[*id]))
                .collect::<Vec<_>>()
        };
        let mut direct = ankhimate_core::replay_physics::ReplayPhysicsInstance::new(
            &d.skeleton,
            1,
            &pure_inputs(0),
        )
        .unwrap();
        let ctx = direct.context();
        let mut actual = vec![frame_json(&d, direct.frame())];
        direct
            .request_tick(
                &ctx,
                480,
                |t| Ok(pure_inputs(t.index)),
                |f| actual.push(frame_json(&d, f)),
            )
            .unwrap();
        direct
            .pump(
                &ctx,
                1024,
                |t| Ok(pure_inputs(t.index)),
                |f| actual.push(frame_json(&d, f)),
            )
            .unwrap();
        assert_eq!(
            Some(actual),
            reference,
            "optimized provider must equal full pure pose inputs"
        );
    }
    if let Some(p) = &dir {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&cases).unwrap()).unwrap();
    }
}
#[test]
fn independent_animated_springs_reset_repeat_and_context() {
    let ops = schedule(144);
    let results: Vec<_> = ["A", "B", "AB", "ABC", "CBA"]
        .iter()
        .map(|n| simulate(&fixture(n), &ops).0)
        .collect();
    for (t, _) in results[0].iter().enumerate() {
        for (record, reference) in [("spring-A", 0), ("spring-B", 1)] {
            let get = |i: usize| {
                results[i][t]["records"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["constraint"] == record)
                    .cloned()
                    .unwrap()
            };
            for i in 2..5 {
                assert_eq!(get(i), get(reference));
            }
        }
    }
    let d = fixture("loop");
    let a = clips(&d);
    let mut s = ReplayAnimationInstance::new(&d.skeleton, &a, "breathing", 1).unwrap();
    let old = s.context();
    let mut before = vec![];
    s.request_tick(&old, 480, |f| before.push(frame_json(&d, f)))
        .unwrap();
    s.pump(&old, 1024, |f| before.push(frame_json(&d, f)))
        .unwrap();
    s.reset(&d.skeleton, &a, "breathing", 2).unwrap();
    assert!(s.request_tick(&old, 1, |_| {}).is_err());
    let ctx = s.context();
    let mut after = vec![];
    s.request_tick(&ctx, 480, |f| after.push(frame_json(&d, f)))
        .unwrap();
    s.pump(&ctx, 1024, |f| after.push(frame_json(&d, f)))
        .unwrap();
    assert_eq!(before, after);
}
#[test]
fn boundaries_holds_setup_and_validation() {
    assert_eq!(canonical_time(120, 1., true).unwrap(), 0.);
    assert_eq!(canonical_time(121, 1., false).unwrap(), 1.);
    assert_eq!(canonical_time(5, 0., true).unwrap(), 0.);
    assert!(canonical_time(1 << 23, 1., true).is_err());
    assert_eq!(
        canonical_time(100, 0.83, true).unwrap(),
        ((100f64 / 120.) % f64::from(0.83f32)) as f32
    );
    let d = author();
    let good = clip(&d);
    let check =
        |a: Animation| ReplayAnimationInstance::new(&d.skeleton, &[a], "breathing", 1).is_err();
    assert!(ReplayAnimationInstance::new(&d.skeleton, &[], "missing", 1).is_err());
    assert!(
        ReplayAnimationInstance::new(&d.skeleton, &[good.clone(), good.clone()], "breathing", 1)
            .is_err()
    );
    let mut bad = good.clone();
    bad.duration = f32::NAN;
    assert!(check(bad));
    for value in [f32::NAN, f32::INFINITY] {
        let mut bad = good.clone();
        let Timeline::BoneTranslate { keys, .. } = &mut bad.timelines[0] else {
            panic!()
        };
        keys[1].value = value;
        assert!(check(bad));
    }
    let mut bad = good.clone();
    bad.timelines.push(bad.timelines[0].clone());
    assert!(check(bad));
    let mut bad = good.clone();
    let Timeline::BoneTranslate { keys, .. } = &mut bad.timelines[0] else {
        panic!()
    };
    keys[1].interp = Interp::Bezier {
        out_handle: glam::vec2(-1., 0.),
        in_handle: glam::vec2(1., 1.),
    };
    assert!(check(bad));
    let mut bad = good.clone();
    bad.bone_offsets.push((d.skeleton.update_order[0], 0.1));
    assert!(check(bad));
    let mut bad = good.clone();
    bad.timelines.push(Timeline::DrawOrder { keys: vec![] });
    assert!(check(bad));
    let mut bad = good.clone();
    let Timeline::BoneTranslate { bone, .. } = &mut bad.timelines[0] else {
        panic!()
    };
    *bone = Default::default();
    assert!(check(bad));
    let mut s = ReplayAnimationInstance::new(&d.skeleton, &[good], "breathing", 1).unwrap();
    let ctx = s.context();
    let p = s.sample_tick(0).unwrap();
    assert!(s.request_tick(&ctx, 1 << 23, |_| {}).is_err());
    assert_eq!(s.clock().requested, 0);
    for id in &d.skeleton.update_order {
        assert_eq!(p.locals[*id], d.skeleton.bones[*id].local_transform);
    }
    let other = ReplayAnimationInstance::new(&d.skeleton, &[clip(&d)], "breathing", 1).unwrap();
    assert!(s.request_tick(&other.context(), 1, |_| {}).is_err());
    // Observer panic cannot falsely commit a tick or replace last good pose.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        s.request_tick(&ctx, 1, |_| panic!("observer"))
    }));
    assert!(result.is_err());
    assert!(s.faulted());
    assert_eq!(s.clock().completed, 0);
    assert_eq!(s.frame().tick, 0);
    assert!(s.request_tick(&ctx, 1, |_| {}).is_err());
}

#[test]
fn descriptive_cpu_cost() {
    let Some(file) = std::env::var_os("T840B4_PERFORMANCE_FILE") else {
        return;
    };
    let mut rows = vec![];
    for name in ["idle", "ABC"] {
        let d = fixture(name);
        let a = clips(&d);
        let s = ReplayAnimationInstance::new(&d.skeleton, &a, "breathing", 1).unwrap();
        let direct: Vec<_> = (0..=120)
            .map(|i| {
                let p = s.sample_tick(i).unwrap();
                d.skeleton
                    .update_order
                    .iter()
                    .map(|id| (*id, p.locals[*id]))
                    .collect::<Vec<_>>()
            })
            .collect();
        for profile in ["B3-direct", "B4-clip", "pure-clip"] {
            let mut samples = vec![];
            for repeat in 0..55 {
                let start = std::time::Instant::now();
                match profile {
                    "B3-direct" => {
                        let mut s = ankhimate_core::replay_physics::ReplayPhysicsInstance::new(
                            &d.skeleton,
                            1,
                            &direct[0],
                        )
                        .unwrap();
                        let c = s.context();
                        s.request_tick(
                            &c,
                            120,
                            |t| Ok(direct[t.index as usize].clone()),
                            |f| {
                                std::hint::black_box(f);
                            },
                        )
                        .unwrap();
                        s.pump(
                            &c,
                            120,
                            |t| Ok(direct[t.index as usize].clone()),
                            |f| {
                                std::hint::black_box(f);
                            },
                        )
                        .unwrap();
                    }
                    "B4-clip" => {
                        let mut s =
                            ReplayAnimationInstance::new(&d.skeleton, &a, "breathing", 1).unwrap();
                        let c = s.context();
                        s.request_tick(&c, 120, |f| {
                            std::hint::black_box(f);
                        })
                        .unwrap();
                        s.pump(&c, 120, |f| {
                            std::hint::black_box(f);
                        })
                        .unwrap();
                    }
                    _ => {
                        for tick in 1..=120 {
                            std::hint::black_box(s.sample_tick(tick).unwrap());
                        }
                    }
                }
                if repeat >= 5 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(json!({"rig":name,"profile":profile,"warmup":5,"runs":50,"ticks":120,"minMs":samples[0],"medianMs":samples[25],"p95Ms":samples[47],"maxMs":samples[49]}));
        }
    }
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
        .unwrap();
    serde_json::to_writer_pretty(file, &rows).unwrap();
}
