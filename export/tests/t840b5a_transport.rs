use ankhimate_core::replay_transport::{ReplayTransport, TransportState};

fn author() -> ankhimate_formats::Loaded {
    ankhimate_formats::from_json(include_str!("fixtures/t840b4/loop.ankh.json")).unwrap()
}
#[test]
fn transport_pause_preserves_requested_debt_without_work() {
    let d = author();
    let clips: Vec<_> = d.animations.values().cloned().collect();
    let mut t = ReplayTransport::new(&d.skeleton, &clips, "breathing", 1).unwrap();
    let c = t.context();
    assert_eq!(t.snapshot().state, TransportState::Stopped);
    t.play(&c, "breathing", false).unwrap();
    t.request_time(&c, 1, 2, |_| {}).unwrap();
    t.pause(&c).unwrap();
    let before = t.snapshot();
    assert_eq!(before.clock.unwrap().completed, 8);
    assert_eq!(before.clock.unwrap().backlog, 52);
    assert_eq!(before, t.snapshot());
    t.pause(&c).unwrap();
    assert_eq!(before, t.snapshot());
    assert!(t.pump(&c, 52, |_| {}).is_err());
    assert_eq!(before, t.snapshot());
    t.resume(&c).unwrap();
    t.pump(&c, 52, |_| {}).unwrap();
    assert_eq!(t.snapshot().clock.unwrap().completed, 60);
}

use ankhimate_core::{
    animation::EventKey, pose::Pose, replay_animation::ReplayAnimationInstance,
    replay_physics::ReplayFrame, replay_transport::TransportContext,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
use serde_json::{Value, json};
use std::collections::HashMap;
fn fixture(name: &str) -> ankhimate_formats::Loaded {
    let mut d = ankhimate_formats::from_json(
        &std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("tests/fixtures/t840b4/{name}.ankh.json")),
        )
        .unwrap(),
    )
    .unwrap();
    if name == "loop" {
        let mut a = d.animations.values().next().unwrap().clone();
        a.name = "held".into();
        a.looping = false;
        a.duration = 1.;
        d.animations.insert(a.clone());
        a.name = "with-event".into();
        a.events.push(EventKey {
            time: 0.25,
            name: "hit".into(),
            int_value: 0,
            float_value: 0.,
            string_value: String::new(),
            audio: String::new(),
            volume: 1.,
            balance: 0.,
        });
        d.animations.insert(a);
    }
    d
}
fn clips(d: &ankhimate_formats::Loaded) -> Vec<ankhimate_core::animation::Animation> {
    d.animations.values().cloned().collect()
}
fn transport(d: &ankhimate_formats::Loaded) -> ReplayTransport {
    ReplayTransport::new(&d.skeleton, &clips(d), "breathing", 1).unwrap()
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
fn state(s: TransportState) -> &'static str {
    match s {
        TransportState::Stopped => "stopped",
        TransportState::Playing => "playing",
        TransportState::Paused => "paused",
        TransportState::SeekPreview => "seek_preview",
        TransportState::Replaying => "replaying",
        TransportState::Faulted => "faulted",
        TransportState::Disposed => "disposed",
    }
}
fn snapshot(t: &ReplayTransport) -> Value {
    let s = t.snapshot();
    json!({"state":state(s.state),"clip":s.clip,"position":s.position.map(|v|v.to_string()),"clipTime":s.clip_time,
        "poseKind":s.pose_kind.map(|k|match k{ankhimate_core::replay_transport::PoseKind::Pure=>"pure",_=>"simulated"}),
        "previewTime":s.preview_time.map(|x|vec![x.numerator.to_string(),x.denominator.to_string()]),"resumeState":s.resume_state.map(state),"replayTarget":s.replay_target.map(|v|v.to_string()),"replayComplete":s.replay_complete,
        "clock":s.clock.map(|c|json!({"epoch":c.epoch.to_string(),"requested":c.requested.to_string(),"completed":c.completed.to_string(),"backlog":c.backlog.to_string(),"next":c.next.map(|v|v.to_string()),"time":c.time.map(|v|vec![v.numerator.to_string(),v.denominator.to_string()])}))})
}
fn pose(d: &ankhimate_formats::Loaded, p: &Pose) -> Value {
    json!(d.skeleton.update_order.iter().map(|id|{let t=p.locals[*id];let w=p.world(*id);let tip=p.world_tip(&d.skeleton,*id);
        json!({"name":d.skeleton.bones[*id].name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty,tip.x,tip.y]})}).collect::<Vec<_>>())
}
fn frame(d: &ankhimate_formats::Loaded, f: &ReplayFrame) -> Value {
    json!({"tick":f.tick.to_string(),"bones":pose(d,&f.pose),"records":f.records.iter().map(|r|json!({"constraint":d.skeleton.constraints[r.constraint].name(),"values":[r.rotation,r.position.x,r.position.y,r.rotation_velocity,r.position_velocity.x,r.position_velocity.y,r.anchor.x,r.anchor.y]})).collect::<Vec<_>>()})
}
fn current(d: &ankhimate_formats::Loaded, t: &ReplayTransport) -> Value {
    t.frame().map(|f| frame(d, f)).unwrap_or(Value::Null)
}
fn integer(o: &Value, key: &str) -> u64 {
    o[key].as_str().unwrap().parse().unwrap()
}
fn simulate(d: &ankhimate_formats::Loaded, ops: &[Value]) -> Value {
    let mut t = transport(d);
    let mut tokens: HashMap<String, TransportContext> = HashMap::new();
    let mut steps = vec![];
    let mut frames = vec![];
    for (index, o) in ops.iter().enumerate() {
        let c = o["context"]
            .as_str()
            .map(|n| tokens[n].clone())
            .unwrap_or_else(|| t.context());
        let before = (snapshot(&t), current(d, &t));
        let selected = t.snapshot().clip;
        let pure = ReplayAnimationInstance::new(&d.skeleton, &clips(d), &selected, 1).unwrap();
        let observer = |f: &ReplayFrame| {
            frames.push(json!({"op":index,"pre":pose(d,&pure.sample_tick(f.tick).unwrap()),"post":frame(d,f)}))
        };
        let result = match o["op"].as_str().unwrap() {
            "play" => t.play(
                &c,
                o["clip"].as_str().unwrap(),
                o["restart"].as_bool().unwrap(),
            ),
            "pause" => t.pause(&c),
            "resume" => t.resume(&c),
            "stop" => t.stop(&c),
            "reset" => t.reset(&c, integer(o, "epoch")),
            "seek" => t.seek_time(&c, integer(o, "n"), integer(o, "d")),
            "replay" => t.replay_to(&c, integer(o, "tick")),
            "request" => t
                .request_time(&c, integer(o, "n"), integer(o, "d"), observer)
                .map(|_| ()),
            "pump" => t
                .pump(&c, o["budget"].as_u64().unwrap() as u32, observer)
                .map(|_| ()),
            "capture" => {
                tokens.insert(o["name"].as_str().unwrap().into(), c);
                Ok(())
            }
            "observe" => {
                for _ in 0..20 {
                    assert_eq!(snapshot(&t), before.0);
                    assert_eq!(current(d, &t), before.1);
                }
                Ok(())
            }
            "crossfade" => t.cross_fade(&c, "held", 0.25),
            "skin" => t.set_skin(&c, "default"),
            "rest_start" => t.start_from_preview(&c),
            "dispose" => t.dispose(&c),
            _ => panic!("unknown op"),
        };
        assert_eq!(
            result.is_err(),
            o["error"].as_bool().unwrap_or(false),
            "{index}: {o} {result:?}"
        );
        if result.is_err() {
            assert_eq!(
                (snapshot(&t), current(d, &t)),
                before,
                "validation error must not mutate"
            );
        }
        let s = t.snapshot();
        if s.pose_kind == Some(ankhimate_core::replay_transport::PoseKind::Simulated) {
            assert_eq!(t.frame().unwrap().tick, s.clock.unwrap().completed);
        }
        let pre = s.position.map(|p| pose(d, &t.sample_tick(p).unwrap()));
        steps.push(json!({"snapshot":snapshot(&t),"frame":current(d,&t),"pre":pre,"outcome":if result.is_err(){"error"}else{"ok"}}));
    }
    json!({"steps":steps,"frames":frames})
}
fn schedule(hz: u64) -> Vec<Value> {
    let mut ops = vec![json!({"op":"play","clip":"breathing","restart":false})];
    if hz == 0 {
        for n in 1..=10 {
            ops.extend([
                json!({"op":"request","n":n.to_string(),"d":"2"}),
                json!({"op":"pause"}),
                json!({"op":"observe"}),
                json!({"op":"resume"}),
                json!({"op":"pump","budget":52}),
            ]);
        }
    } else {
        for n in 1..=5 * hz {
            ops.push(json!({"op":"request","n":n.to_string(),"d":hz.to_string()}));
        }
    }
    ops.push(json!({"op":"pump","budget":1024}));
    ops
}
#[test]
fn shared_vectors_real_native_exports_and_per_tick_campaign() {
    let dir = std::env::var_os("T840B5A_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &dir {
        std::fs::create_dir(p).unwrap();
    }
    let shared: Value =
        serde_json::from_str(include_str!("fixtures/t840b5a/transport-vectors.json")).unwrap();
    let mut rows = vec![];
    for name in [
        "idle", "parent", "loop", "A", "B", "AB", "ABC", "CBA", "angles",
    ] {
        let d = fixture(name);
        let author = ankhimate_formats::to_json(&view(&d)).unwrap();
        let d = ankhimate_formats::from_json(&author).unwrap();
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
            std::fs::write(p.join(format!("{name}.native.json")), native).unwrap();
            if name == "loop" {
                std::fs::write(p.join("multiclip.ankh.json"), author).unwrap();
            }
        }
        let mut baseline = None;
        for hz in [30, 60, 120, 144, 0] {
            let ops = schedule(hz);
            let result = simulate(&d, &ops);
            let per_tick: Vec<_> = result["frames"]
                .as_array()
                .unwrap()
                .iter()
                .map(|f| f["post"].clone())
                .collect();
            assert_eq!(per_tick.len(), 600);
            if let Some(b) = &baseline {
                assert_eq!(&per_tick, b);
            } else {
                baseline = Some(per_tick);
            }
            rows.push(json!({"name":format!("{name}/{hz}"),"rig":name,"ops":ops,"result":result}));
        }
    }
    let d = fixture("loop");
    for c in shared["cases"].as_array().unwrap() {
        let ops = c["ops"].as_array().unwrap();
        rows.push(json!({"name":c["name"],"rig":"loop","ops":ops,"result":simulate(&d,ops)}));
    }
    if let Some(p) = &dir {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}
#[test]
fn replay_600_matches_every_continuous_frame_and_is_interruptible() {
    let d = fixture("loop");
    let mut t = transport(&d);
    let c = t.context();
    t.play(&c, "breathing", false).unwrap();
    let mut normal = vec![];
    for n in 1..=600 {
        t.request_tick(&c, n, |f| normal.push(frame(&d, f)))
            .unwrap();
    }
    t.replay_to(&c, 600).unwrap();
    let c = t.context();
    assert_eq!(t.snapshot().clock.unwrap().completed, 0);
    assert_eq!(t.snapshot().clock.unwrap().backlog, 600);
    t.pause(&c).unwrap();
    let snapshot = t.snapshot();
    assert!(t.pump(&c, 1, |_| {}).is_err());
    assert_eq!(snapshot, t.snapshot());
    t.resume(&c).unwrap();
    let mut replay = vec![];
    for n in 1..=600 {
        t.pump(&c, 1, |f| replay.push(frame(&d, f))).unwrap();
        assert_eq!(t.snapshot().clock.unwrap().completed, n);
        if n == 137 {
            t.pause(&c).unwrap();
            t.resume(&c).unwrap();
        }
    }
    assert_eq!(replay, normal);
    assert_eq!(t.snapshot().state, TransportState::Paused);
    assert!(t.snapshot().replay_complete);
    assert!(t.pump(&c, 1, |_| {}).is_err());
    t.resume(&c).unwrap();
    t.request_tick(&c, 601, |_| {}).unwrap();
    assert_eq!(t.snapshot().clock.unwrap().completed, 601);
}
#[test]
fn instances_contexts_disposal_rejections_and_faults_are_isolated() {
    let d = fixture("loop");
    let mut a = transport(&d);
    let mut b = transport(&d);
    let ca = a.context();
    let cb = b.context();
    assert!(a.play(&cb, "breathing", false).is_err());
    a.play(&ca, "breathing", false).unwrap();
    b.play(&cb, "breathing", false).unwrap();
    a.request_tick(&ca, 60, |_| {}).unwrap();
    a.pause(&ca).unwrap();
    let frozen = (snapshot(&a), current(&d, &a));
    for n in 1..=600 {
        b.request_tick(&cb, n, |_| {}).unwrap();
    }
    assert_eq!((snapshot(&a), current(&d, &a)), frozen);
    a.resume(&ca).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        a.pump(&ca, 52, |f| {
            if f.tick == 11 {
                panic!("observer failure")
            }
        })
    }));
    assert!(result.is_err());
    assert_eq!(a.snapshot().state, TransportState::Faulted);
    assert_eq!(a.snapshot().clock.unwrap().completed, 10);
    assert_eq!(a.frame().unwrap().tick, 10);
    assert!(a.resume(&ca).is_err());
    assert!(a.stop(&ca).is_err());
    assert!(a.replay_to(&ca, 60).is_err());
    let before = current(&d, &b);
    a.reset(&ca, 2).unwrap();
    assert!(a.play(&ca, "breathing", false).is_err());
    a.dispose(&a.context()).unwrap();
    assert_eq!(a.snapshot().state, TransportState::Disposed);
    assert!(a.frame().is_err());
    assert!(a.reset(&ca, 3).is_err());
    assert_eq!(before, current(&d, &b));
    let mut alone = transport(&d);
    let c = alone.context();
    alone.play(&c, "breathing", false).unwrap();
    for n in 1..=600 {
        alone.request_tick(&c, n, |_| {}).unwrap();
    }
    assert_eq!(current(&d, &alone), current(&d, &b));
}

#[test]
fn numerical_sampler_and_physics_faults_keep_last_good_public_frame() {
    for sampler in [false, true] {
        let mut d = fixture("loop");
        if sampler {
            use ankhimate_core::animation::{Axis, Key, Timeline};
            let root = d.skeleton.update_order[0];
            let a = d
                .animations
                .values_mut()
                .find(|a| a.name == "breathing")
                .unwrap();
            a.timelines.retain(
                |t| !matches!(t,Timeline::BoneTranslate{bone,axis:Axis::X,..}if *bone==root),
            );
            a.timelines.push(Timeline::BoneTranslate {
                bone: root,
                axis: Axis::X,
                keys: vec![Key::linear(0., f32::MAX), Key::linear(1., -f32::MAX)],
            });
        } else {
            let ankhimate_core::constraints::Constraint::Physics(p) =
                d.skeleton.constraints.values_mut().next().unwrap()
            else {
                panic!()
            };
            p.wind.x = f32::MAX;
            p.mass = 0.01;
        }
        let mut t = transport(&d);
        let c = t.context();
        t.play(&c, "breathing", false).unwrap();
        let before = current(&d, &t);
        let p = t
            .request_tick(&c, 1, |_| panic!("failed tick must not notify"))
            .unwrap();
        assert!(matches!(
            p.stop,
            ankhimate_core::replay_clock::Stop::Failed(_)
        ));
        assert_eq!(t.snapshot().state, TransportState::Faulted);
        assert_eq!(t.snapshot().clock.unwrap().completed, 0);
        assert_eq!(before, current(&d, &t));
        assert!(t.pump(&c, 1, |_| {}).is_err());
        t.reset(&c, 2).unwrap();
        assert_eq!(t.snapshot().state, TransportState::Stopped);
    }
}

#[test]
fn descriptive_transport_cpu_cost() {
    let Some(file) = std::env::var_os("T840B5A_PERFORMANCE_FILE") else {
        return;
    };
    let mut rows = vec![];
    for name in ["idle", "ABC"] {
        let d = fixture(name);
        let all = clips(&d);
        for ticks in [120, 600, 1200] {
            for profile in ["B4-advance", "B5A-advance", "B4-replay", "B5A-replay"] {
                let mut samples = vec![];
                for repeat in 0..55 {
                    let start = std::time::Instant::now();
                    if profile.starts_with("B4") {
                        let mut s = ReplayAnimationInstance::new(&d.skeleton, &all, "breathing", 1)
                            .unwrap();
                        if profile.ends_with("replay") {
                            s.reset(&d.skeleton, &all, "breathing", 2).unwrap();
                            let c = s.context();
                            s.queue_time(&c, ticks, 120).unwrap();
                            while s.clock().backlog > 0 {
                                s.pump(&c, 64, |f| {
                                    std::hint::black_box(f);
                                })
                                .unwrap();
                            }
                        } else {
                            let c = s.context();
                            s.request_tick(&c, ticks, |f| {
                                std::hint::black_box(f);
                            })
                            .unwrap();
                            s.pump(&c, ticks as u32, |f| {
                                std::hint::black_box(f);
                            })
                            .unwrap();
                        }
                    } else {
                        let mut t = transport(&d);
                        if profile.ends_with("replay") {
                            t.replay_to(&t.context(), ticks).unwrap();
                            let c = t.context();
                            while t.snapshot().clock.unwrap().backlog > 0 {
                                t.pump(&c, 64, |f| {
                                    std::hint::black_box(f);
                                })
                                .unwrap();
                            }
                        } else {
                            let c = t.context();
                            t.play(&c, "breathing", false).unwrap();
                            t.request_tick(&c, ticks, |f| {
                                std::hint::black_box(f);
                            })
                            .unwrap();
                            t.pump(&c, ticks as u32, |f| {
                                std::hint::black_box(f);
                            })
                            .unwrap();
                        }
                    }
                    if repeat >= 5 {
                        samples.push(start.elapsed().as_secs_f64() * 1000.);
                    }
                }
                samples.sort_by(f64::total_cmp);
                rows.push(json!({"rig":name,"metric":profile,"iterations":1,"ticks":ticks,"warmup":5,"runs":50,"medianMs":samples[25],"p95Ms":samples[47],"minMs":samples[0],"maxMs":samples[49],"medianUsPerTick":samples[25]*1000./ticks as f64}));
            }
        }
        for metric in [
            "play-same",
            "pause-resume-pair",
            "snapshot",
            "frame-borrow",
            "frame-owned-copy",
            "pure-pose-query",
            "restart",
            "stop",
            "seek",
            "reset",
        ] {
            let mut t = transport(&d);
            t.play(&t.context(), "breathing", false).unwrap();
            let mut samples = vec![];
            for repeat in 0..55 {
                let start = std::time::Instant::now();
                for _ in 0..100 {
                    let c = t.context();
                    match metric {
                        "play-same" => t.play(&c, "breathing", false).unwrap(),
                        "pause-resume-pair" => {
                            t.pause(&c).unwrap();
                            t.resume(&c).unwrap();
                        }
                        "snapshot" => {
                            std::hint::black_box(t.snapshot());
                        }
                        "frame-borrow" => {
                            std::hint::black_box(t.frame().unwrap());
                        }
                        "frame-owned-copy" => {
                            std::hint::black_box(t.frame().unwrap().clone());
                        }
                        "pure-pose-query" => {
                            std::hint::black_box(t.sample_tick(30).unwrap());
                        }
                        "restart" => t.play(&c, "breathing", true).unwrap(),
                        "stop" => t.stop(&c).unwrap(),
                        "seek" => t.seek_tick(&c, 30).unwrap(),
                        "reset" => t.reset(&c, c.epoch() + 1).unwrap(),
                        _ => unreachable!(),
                    }
                }
                if repeat >= 5 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(json!({"rig":name,"metric":metric,"iterations":100,"ticks":0,"warmup":5,"runs":50,"medianMs":samples[25],"p95Ms":samples[47],"minMs":samples[0],"maxMs":samples[49],"medianUsPerCommand":samples[25]*10.}));
        }
    }
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
        .unwrap();
    serde_json::to_writer_pretty(file, &rows).unwrap();
}
