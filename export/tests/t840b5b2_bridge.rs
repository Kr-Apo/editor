use ankhimate_core::{
    pose::Pose,
    replay_animation::ReplayAnimationInstance,
    replay_consumer::{ConsumerHealth, ReplayConsumerBridge},
    replay_physics::ReplayFrame,
    replay_skin::SkinEvent,
    replay_transport::{TransportContext, TransportState},
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
use serde_json::{Value, json};
use std::collections::HashMap;
fn clips(d: &ankhimate_formats::Loaded) -> Vec<ankhimate_core::animation::Animation> {
    d.animations.values().cloned().collect()
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
fn pose(d: &ankhimate_formats::Loaded, p: &Pose) -> Value {
    json!(d.skeleton.update_order.iter().map(|id|{let t=p.locals[*id];let w=p.world(*id);let tip=p.world_tip(&d.skeleton,*id);
        json!({"name":d.skeleton.bones[*id].name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty,tip.x,tip.y]})}).collect::<Vec<_>>())
}
fn frame(d: &ankhimate_formats::Loaded, f: &ReplayFrame) -> Value {
    json!({"tick":f.tick.to_string(),"bones":pose(d,&f.pose),"records":f.records.iter().map(|r|json!({"constraint":d.skeleton.constraints[r.constraint].name(),"values":[r.rotation,r.position.x,r.position.y,r.rotation_velocity,r.position_velocity.x,r.position_velocity.y,r.anchor.x,r.anchor.y]})).collect::<Vec<_>>()})
}
fn events() -> Vec<SkinEvent> {
    vec![
        SkinEvent {
            tick: 0,
            skin: "on".into(),
        },
        SkinEvent {
            tick: 120,
            skin: "off".into(),
        },
        SkinEvent {
            tick: 240,
            skin: "on".into(),
        },
    ]
}
fn fixture(name: &str) -> ankhimate_formats::Loaded {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/t840b5b2/{name}.ankh.json"));
    let d = ankhimate_formats::from_json(&std::fs::read_to_string(p).unwrap()).unwrap();
    assert!(d.report.is_clean());
    let reopened =
        ankhimate_formats::from_json(&ankhimate_formats::to_json(&view(&d)).unwrap()).unwrap();
    assert!(reopened.report.is_clean());
    reopened
}
fn bridge(d: &ankhimate_formats::Loaded) -> ReplayConsumerBridge {
    ReplayConsumerBridge::new(&d.skeleton, &clips(d), "breathing", 1, Some(&events())).unwrap()
}
fn snapshot(d: &ankhimate_formats::Loaded, b: &ReplayConsumerBridge) -> Value {
    let Ok(f) = b.current_frame() else {
        return Value::Null;
    };
    json!({"identity":[f.epoch.to_string(),f.pose_tick.to_string()],"epoch":f.epoch.to_string(),"clip":f.clip,"requested":f.requested.to_string(),"completed":f.completed.to_string(),"backlog":f.backlog.to_string(),"poseTick":f.pose_tick.to_string(),"poseTime":[f.pose_time.numerator.to_string(),f.pose_time.denominator.to_string()],"clipTime":f.clip_time,"skin":f.skin,"state":state(f.state),"health":if f.health==ConsumerHealth::Healthy{"healthy"}else{"faulted"},"targetReached":f.target_reached,
 "bones":f.bones.iter().map(|b|{let t=b.local;let w=b.world;json!({"name":b.name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty]})}).collect::<Vec<_>>(),
 "records":f.records.iter().map(|r|json!({"constraint":d.skeleton.constraints[r.constraint].name(),"values":[r.rotation,r.position.x,r.position.y,r.rotation_velocity,r.position_velocity.x,r.position_velocity.y,r.anchor.x,r.anchor.y]})).collect::<Vec<_>>()})
}
fn preview(b: &ReplayConsumerBridge) -> Value {
    b.preview_frame().ok().flatten().map(|f|json!({"epoch":f.epoch.to_string(),"tick":f.tick.to_string(),"skin":f.skin,"poseTime":[f.pose_time.numerator.to_string(),f.pose_time.denominator.to_string()],"clipTime":f.clip_time,"bones":f.bones.iter().map(|b|{let t=b.local;let w=b.world;json!({"name":b.name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty]})}).collect::<Vec<_>>()})).unwrap_or(Value::Null)
}
fn status(b: &ReplayConsumerBridge) -> Value {
    let s = b.transport_status();
    json!({"state":state(s.state),"epoch":b.context().epoch().to_string(),"position":s.position.map(|n|n.to_string()),"kind":s.pose_kind.map(|k|format!("{k:?}")),"replayComplete":s.replay_complete})
}
fn simulate(d: &ankhimate_formats::Loaded, ops: &[Value]) -> Value {
    let mut b = bridge(d);
    let pure =
        ReplayAnimationInstance::with_skins(&d.skeleton, &clips(d), "breathing", 1, &events())
            .unwrap();
    let mut tokens: HashMap<String, TransportContext> = HashMap::new();
    let mut steps = vec![];
    let mut frames = vec![];
    for (index, o) in ops.iter().enumerate() {
        let c = o["context"]
            .as_str()
            .map(|s| tokens[s].clone())
            .unwrap_or_else(|| b.context());
        let before = (snapshot(d, &b), preview(&b), status(&b));
        let tick = |key: &str| o[key].as_str().unwrap().parse::<u64>().unwrap();
        let audit = |f: &ReplayFrame| {
            frames.push(json!({"op":index,"pre":pose(d,&pure.sample_tick(f.tick).unwrap()),"post":frame(d,f)}))
        };
        let result = match o["op"].as_str().unwrap() {
            "capture" => {
                tokens.insert(o["name"].as_str().unwrap().into(), c);
                Ok(())
            }
            "play" => b.play(&c, "breathing", false),
            "pause" => b.pause(&c),
            "resume" => b.resume(&c),
            "stop" => b.stop(&c),
            "reset" => b.reset(&c, tick("epoch")),
            "seek" => b.seek_tick(&c, tick("tick")),
            "replay" => b.replay_from_origin(&c, tick("tick")),
            "request" => b.request_time(&c, tick("n"), tick("d"), audit).map(|_| ()),
            "pump" => b
                .pump(&c, o["budget"].as_u64().unwrap() as u32, audit)
                .map(|_| ()),
            "observe" => {
                for _ in 0..100 {
                    assert_eq!((snapshot(d, &b), preview(&b), status(&b)), before);
                    b.present(|_| Ok(())).ok();
                }
                Ok(())
            }
            "present-fail" => {
                assert!(b.present(|_| Err("renderer failed".into())).is_err());
                assert_eq!((snapshot(d, &b), preview(&b), status(&b)), before);
                Ok(())
            }
            "audit-fail" => {
                assert!(
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| b.request_tick(
                        &c,
                        tick("tick"),
                        |_| panic!("precommit audit")
                    )))
                    .is_err()
                );
                assert_eq!(b.transport_status().state, TransportState::Faulted);
                Ok(())
            }
            "dispose" => b.dispose(&c),
            _ => panic!("unknown op"),
        };
        assert_eq!(
            result.is_err(),
            o["error"].as_bool().unwrap_or(false),
            "{index} {o} {result:?}"
        );
        if result.is_err() {
            assert_eq!((snapshot(d, &b), preview(&b), status(&b)), before);
        }
        let f = snapshot(d, &b);
        if !f.is_null() {
            assert_eq!(f["poseTick"], f["completed"]);
            let n = f["poseTick"].as_str().unwrap().parse::<u64>().unwrap();
            let expected = if !(120..240).contains(&n) {
                "on"
            } else {
                "off"
            };
            assert_eq!(f["skin"], expected);
            // Independent transport oracle: a mislabeled future pose must fail
            // even if both language bridges made the same packaging mistake.
            let oracle = if n == 0 {
                frame(d, pure.frame())
            } else {
                frames.last().expect("completed audit frame")["post"].clone()
            };
            assert_eq!(oracle["tick"], f["poseTick"]);
            assert_eq!(oracle["records"], f["records"]);
            let mut bones = oracle["bones"].clone();
            for b in bones.as_array_mut().unwrap() {
                b["values"].as_array_mut().unwrap().truncate(13);
            }
            assert_eq!(bones, f["bones"]);
        }
        steps.push(json!({"snapshot":f,"preview":preview(&b),"status":status(&b),"outcome":if result.is_err(){"error"}else{"ok"}}));
    }
    json!({"steps":steps,"frames":frames})
}
#[test]
fn consumer_coherence_real_exports_and_shared_vectors() {
    let dir = std::env::var_os("T840B5B2_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &dir {
        std::fs::create_dir(p).unwrap();
    }
    let vectors: Value =
        serde_json::from_str(include_str!("fixtures/t840b5b2/consumer-vectors.json")).unwrap();
    let mut rows = vec![];
    for name in ["rig-5", "rig-32", "rig-128"] {
        let d = fixture(name);
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
            std::fs::write(
                p.join(format!("{name}.ankh.json")),
                ankhimate_formats::to_json(&view(&d)).unwrap(),
            )
            .unwrap();
        }
        let mut baseline = None;
        for case in vectors
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["rig"] == name)
        {
            let result = simulate(&d, case["ops"].as_array().unwrap());
            if case["name"].as_str().unwrap().contains('/') {
                let trace: Vec<_> = result["frames"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f["post"].clone())
                    .collect();
                if let Some(b) = &baseline {
                    assert_eq!(&trace, b)
                } else {
                    baseline = Some(trace)
                }
            }
            let mut row = case.clone();
            row["result"] = result;
            rows.push(row);
        }
    }
    if let Some(p) = dir {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}
#[test]
fn independent_consumers_owned_snapshots_and_invalid_initialization() {
    let d = fixture("rig-32");
    let mut a = bridge(&d);
    let mut b = bridge(&d);
    assert!(ReplayConsumerBridge::new(&d.skeleton, &clips(&d), "missing", 1, None).is_err());
    assert!(a.play(&b.context(), "breathing", false).is_err());
    let old = a.context();
    for t in [&mut a, &mut b] {
        t.play(&t.context(), "breathing", false).unwrap();
    }
    a.request_time(&a.context(), 1, 2, |_| {}).unwrap();
    let saved = snapshot(&d, &a);
    a.request_time(&a.context(), 2, 4, |_| {}).unwrap();
    assert_eq!(snapshot(&d, &a), saved);
    a.pause(&a.context()).unwrap();
    b.request_tick(&b.context(), 360, |_| {}).unwrap();
    b.pump(&b.context(), 352, |_| {}).unwrap();
    let owned = a.current_frame().unwrap();
    a.reset(&a.context(), 2).unwrap();
    assert_eq!(owned.completed, 8);
    assert!(a.play(&old, "breathing", false).is_err());
    assert_eq!(b.current_frame().unwrap().completed, 360);
    let before = snapshot(&d, &b);
    assert!(b.present(|_| Err("presentation failure".into())).is_err());
    assert_eq!(snapshot(&d, &b), before);
    a.dispose(&a.context()).unwrap();
    assert!(a.current_frame().is_err());
    assert_eq!(b.current_frame().unwrap().completed, 360);
}

#[test]
fn measured_consumer_boundary() {
    let Some(path) = std::env::var_os("T840B5B2_PERFORMANCE_FILE") else {
        return;
    };
    let mut rows = vec![];
    for name in ["rig-5", "rig-32", "rig-128"] {
        let d = fixture(name);
        let clips = clips(&d);
        let e = events();
        for operation in [
            "request-unchanged",
            "status",
            "snapshot",
            "snapshot-copy",
            "bridge-without-physics",
            "transport-physics",
            "bridge-physics",
            "visual-skin-physics",
            "eight-instances",
            "catch-up",
        ] {
            let b = bridge(&d);
            let owned = b.current_frame().unwrap();
            let mut samples = vec![];
            for repeat in 0..18 {
                let start = std::time::Instant::now();
                match operation {
                    "request-unchanged" => {
                        let mut b = bridge(&d);
                        b.play(&b.context(), "breathing", false).unwrap();
                        for _ in 0..1000 {
                            std::hint::black_box(b.request_tick(&b.context(), 0, |_| {}).unwrap());
                        }
                    }
                    "status" => {
                        for _ in 0..1000 {
                            std::hint::black_box(b.transport_status());
                        }
                    }
                    "snapshot" => {
                        for _ in 0..100 {
                            std::hint::black_box(b.current_frame().unwrap());
                        }
                    }
                    "snapshot-copy" => {
                        for _ in 0..100 {
                            std::hint::black_box((owned.bones.clone(), owned.records.clone()));
                        }
                    }
                    "bridge-without-physics" => {
                        let mut b = bridge(&d);
                        b.play(&b.context(), "breathing", false).unwrap();
                        for _ in 0..100 {
                            b.request_tick(&b.context(), 0, |_| {}).unwrap();
                            std::hint::black_box(b.current_frame().unwrap());
                        }
                    }
                    "transport-physics" => {
                        let mut t = ankhimate_core::replay_transport::ReplayTransport::with_skins(
                            &d.skeleton,
                            &clips,
                            "breathing",
                            1,
                            &e,
                        )
                        .unwrap();
                        t.play(&t.context(), "breathing", false).unwrap();
                        t.request_tick(&t.context(), 120, |_| {}).unwrap();
                        t.pump(&t.context(), 112, |_| {}).unwrap();
                    }
                    _ => {
                        let count = if operation == "eight-instances" { 8 } else { 1 };
                        for _ in 0..count {
                            let visual = [
                                SkinEvent {
                                    tick: 0,
                                    skin: "on".into(),
                                },
                                SkinEvent {
                                    tick: 120,
                                    skin: "visual".into(),
                                },
                                SkinEvent {
                                    tick: 240,
                                    skin: "on".into(),
                                },
                            ];
                            let mut b = ReplayConsumerBridge::new(
                                &d.skeleton,
                                &clips,
                                "breathing",
                                1,
                                Some(if operation == "visual-skin-physics" {
                                    &visual
                                } else {
                                    &e
                                }),
                            )
                            .unwrap();
                            b.play(&b.context(), "breathing", false).unwrap();
                            let ticks = if operation == "catch-up" { 600 } else { 120 };
                            b.request_tick(&b.context(), ticks, |_| {}).unwrap();
                            while b.transport_status().clock.unwrap().backlog > 0 {
                                b.pump(&b.context(), 64, |_| {}).unwrap();
                                std::hint::black_box(b.current_frame().unwrap());
                            }
                        }
                    }
                }
                if repeat >= 3 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(json!({"rig":name,"operation":operation,"medianMs":samples[samples.len()/2],"minMs":samples[0],"maxMs":samples.last(),"samplesMs":samples}));
        }
    }
    use std::io::Write;
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap()
        .write_all(&serde_json::to_vec_pretty(&rows).unwrap())
        .unwrap();
}
