use ankhimate_core::{
    pose::Pose,
    replay_animation::ReplayAnimationInstance,
    replay_physics::{ReplayFrame, ReplayPhysicsInstance},
    replay_skin::SkinEvent,
    replay_transport::{ReplayTransport, TransportContext, TransportState},
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
use serde_json::{Value, json};
use std::collections::HashMap;
fn fixture(name: &str) -> ankhimate_formats::Loaded {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("tests/fixtures/t840b5b1/{name}.ankh.json")),
    )
    .unwrap();
    let d = ankhimate_formats::from_json(&text).unwrap();
    assert!(d.report.is_clean(), "{:?}", d.report);
    let saved = ankhimate_formats::to_json(&view(&d)).unwrap();
    let reopened = ankhimate_formats::from_json(&saved).unwrap();
    assert!(reopened.report.is_clean());
    reopened
}
fn events(visual: bool) -> Vec<SkinEvent> {
    vec![
        SkinEvent {
            tick: 0,
            skin: "on".into(),
        },
        SkinEvent {
            tick: 120,
            skin: if visual { "visual" } else { "off" }.into(),
        },
        SkinEvent {
            tick: 240,
            skin: "on".into(),
        },
        SkinEvent {
            tick: 300,
            skin: "partial".into(),
        },
        SkinEvent {
            tick: 330,
            skin: "on".into(),
        },
    ]
}
fn transport(d: &ankhimate_formats::Loaded, e: &[SkinEvent]) -> ReplayTransport {
    ReplayTransport::with_skins(&d.skeleton, &clips(d), "breathing", 1, e).unwrap()
}
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
fn snapshot(t: &ReplayTransport) -> Value {
    let s = t.snapshot();
    let skin = s.position.and_then(|n| t.skin_at(n).unwrap());
    json!({"skin":skin.as_ref().map(|s|s.skin.clone()),"state":state(s.state),"clip":s.clip,"position":s.position.map(|v|v.to_string()),"clipTime":s.clip_time,
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
fn simulate(d: &ankhimate_formats::Loaded, ops: &[Value], events: &[SkinEvent]) -> Value {
    let mut t = transport(d, events);
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
        let pure =
            ReplayAnimationInstance::with_skins(&d.skeleton, &clips(d), &selected, 1, events)
                .unwrap();
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
        for n in 1..=6 {
            ops.push(json!({"op":"request","n":n.to_string(),"d":"2"}));
            ops.push(json!({"op":"pause"}));
            ops.push(json!({"op":"observe"}));
            ops.push(json!({"op":"resume"}));
            ops.push(json!({"op":"pump","budget":52}));
        }
    } else {
        for n in 1..=hz * 3 {
            ops.push(json!({"op":"request","n":n.to_string(),"d":hz.to_string()}));
        }
    }
    ops
}
#[test]
fn native_calendars_cadences_backlog_and_real_export() {
    let dir = std::env::var_os("T840B5B1_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &dir {
        std::fs::create_dir(p).unwrap();
    }
    let mut rows = vec![];
    for name in [
        "independent",
        "dependent",
        "zero-mix",
        "default-only",
        "control",
    ] {
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
        for visual in [false, true] {
            let e = events(visual);
            let mut baseline = None;
            for hz in [30, 60, 120, 144, 0] {
                let ops = schedule(hz);
                let result = simulate(&d, &ops, &e);
                let per_tick: Vec<_> = result["frames"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f["post"].clone())
                    .collect();
                if let Some(b) = &baseline {
                    assert_eq!(&per_tick, b);
                } else {
                    baseline = Some(per_tick);
                }
                rows.push(json!({"name":format!("{name}/{visual}/{hz}"),"rig":name,"visual":visual,"ops":ops,"result":result}));
            }
            let mut ops = schedule(60);
            ops.push(json!({"op":"capture","name":"past"}));
            for epoch in [100, 200, 300] {
                ops.extend([
                    json!({"op":"seek","n":"3","d":"2"}),
                    json!({"op":"skin","error":true}),
                    json!({"op":"replay","tick":"360"}),
                    json!({"op":"pause"}),
                    json!({"op":"observe"}),
                    json!({"op":"resume"}),
                ]);
                for _ in 0..6 {
                    ops.push(json!({"op":"pump","budget":64}));
                }
                ops.push(json!({"op":"reset","epoch":epoch.to_string()}));
            }
            ops.extend([json!({"op":"play","clip":"breathing","restart":false,"context":"past","error":true}),json!({"op":"dispose"}),json!({"op":"play","clip":"breathing","restart":false,"error":true})]);
            let result = simulate(&d, &ops, &e);
            rows.push(json!({"name":format!("{name}-lifecycle-{visual}"),"rig":name,"visual":visual,"ops":ops,"result":result}));
        }
    }
    if let Some(p) = dir {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}

#[test]
fn mix_zero_and_invalid_native_members_do_not_expand_the_contract() {
    let d = fixture("independent");
    let z = fixture("zero-mix");
    let mut a = transport(&d, &events(true));
    let mut b = transport(&z, &events(true));
    a.play(&a.context(), "breathing", false).unwrap();
    b.play(&b.context(), "breathing", false).unwrap();
    for n in 1..300 {
        a.request_tick(&a.context(), n, |_| {}).unwrap();
        b.request_tick(&b.context(), n, |_| {}).unwrap();
        assert_eq!(
            frame(&d, a.frame().unwrap())["records"][0],
            frame(&z, b.frame().unwrap())["records"][0]
        );
    }
    let p = b.sample_tick(299).unwrap();
    let id = z
        .skeleton
        .update_order
        .iter()
        .find(|b| z.skeleton.bones[**b].name == "A")
        .unwrap();
    assert_eq!(p.locals[*id], b.frame().unwrap().pose.locals[*id]);
    let mut bad = d.skeleton.clone();
    let duplicate = bad.skins.values().find(|s| s.name == "on").unwrap().clone();
    bad.add_skin(duplicate);
    assert!(ReplayTransport::with_skins(&bad, &clips(&d), "breathing", 1, &events(false)).is_err());
    let mut bad = d.skeleton.clone();
    let c = bad.constraint_order[0];
    bad.constraints.remove(c);
    assert!(ReplayTransport::with_skins(&bad, &clips(&d), "breathing", 1, &events(false)).is_err());
    let mut bad = d.skeleton.clone();
    let c = bad.constraint_order[0];
    bad.constraint_order.remove(0);
    bad.constraints.remove(c);
    assert!(ReplayTransport::with_skins(&bad, &clips(&d), "breathing", 1, &events(false)).is_err());
}

#[test]
fn activation_drops_state_primes_current_parent_and_preserves_others() {
    let d = fixture("independent");
    let e = events(false);
    let mut t = transport(&d, &e);
    let mut visual = transport(&d, &events(true));
    let c = t.context();
    let cv = visual.context();
    t.play(&c, "breathing", false).unwrap();
    visual.play(&cv, "breathing", false).unwrap();
    let pure = ReplayAnimationInstance::new(&d.skeleton, &clips(&d), "breathing", 1).unwrap();
    for n in 1..=360 {
        t.request_tick(&c, n, |_| {}).unwrap();
        visual.request_tick(&cv, n, |_| {}).unwrap();
        let f = t.frame().unwrap();
        let names: Vec<_> = f
            .records
            .iter()
            .map(|r| d.skeleton.constraints[r.constraint].name())
            .collect();
        assert_eq!(names.contains(&"spring-A"), !(120..240).contains(&n));
        let b = f
            .records
            .iter()
            .find(|r| d.skeleton.constraints[r.constraint].name() == "spring-B")
            .unwrap();
        let vb = visual
            .frame()
            .unwrap()
            .records
            .iter()
            .find(|r| r.constraint == b.constraint)
            .unwrap();
        assert_eq!(format!("{b:?}"), format!("{vb:?}"));
        if n == 240 {
            let p = pure.sample_tick(n).unwrap();
            let inputs: Vec<_> = d
                .skeleton
                .update_order
                .iter()
                .map(|b| (*b, p.locals[*b]))
                .collect();
            let mut fresh = ReplayPhysicsInstance::with_skins(
                &d.skeleton,
                1,
                &inputs,
                &[SkinEvent {
                    tick: 0,
                    skin: "on".into(),
                }],
            )
            .unwrap();
            fresh
                .request_tick(&fresh.context(), 1, |_| Ok(inputs.clone()), |_| {})
                .unwrap();
            let a = f
                .records
                .iter()
                .find(|r| d.skeleton.constraints[r.constraint].name() == "spring-A")
                .unwrap();
            let fa = fresh
                .frame()
                .records
                .iter()
                .find(|r| r.constraint == a.constraint)
                .unwrap();
            assert_eq!(format!("{a:?}"), format!("{fa:?}"));
        }
    }
    // Visual-only change must match an unchanging calendar exactly, before later membership events.
    let mut fixed = transport(
        &d,
        &[SkinEvent {
            tick: 0,
            skin: "on".into(),
        }],
    );
    let mut v = transport(&d, &events(true));
    fixed.play(&fixed.context(), "breathing", false).unwrap();
    v.play(&v.context(), "breathing", false).unwrap();
    for n in 1..300 {
        fixed.request_tick(&fixed.context(), n, |_| {}).unwrap();
        v.request_tick(&v.context(), n, |_| {}).unwrap();
        assert_eq!(
            frame(&d, fixed.frame().unwrap()),
            frame(&d, v.frame().unwrap())
        );
    }
}

#[test]
fn seek_replay_resets_and_errors_preserve_history() {
    let d = fixture("dependent");
    let e = events(false);
    let mut t = transport(&d, &e);
    let c = t.context();
    t.play(&c, "breathing", false).unwrap();
    let mut expected = vec![];
    for n in 1..=360 {
        t.request_tick(&c, n, |f| expected.push(frame(&d, f)))
            .unwrap();
    }
    for _ in 0..3 {
        t.seek_tick(&t.context(), 180).unwrap();
        assert_eq!(t.skin_at(180).unwrap().unwrap().skin, "off");
        assert!(t.frame().unwrap().records.is_empty());
        assert_eq!(t.snapshot().clock.unwrap().completed, 0);
        t.replay_to(&t.context(), 360).unwrap();
        let ctx = t.context();
        t.pause(&ctx).unwrap();
        t.resume(&ctx).unwrap();
        let mut actual = vec![];
        while t.snapshot().clock.unwrap().backlog > 0 {
            t.pump(&ctx, 17, |f| actual.push(frame(&d, f))).unwrap();
        }
        assert_eq!(actual, expected);
        t.reset(&t.context(), t.context().epoch() + 1).unwrap();
    }
    let before = (snapshot(&t), current(&d, &t));
    assert!(t.set_skin(&t.context(), "on").is_err());
    assert_eq!((snapshot(&t), current(&d, &t)), before);
    for bad in [
        vec![],
        vec![SkinEvent {
            tick: 1,
            skin: "on".into(),
        }],
        vec![SkinEvent {
            tick: 0,
            skin: "missing".into(),
        }],
        vec![
            SkinEvent {
                tick: 0,
                skin: "on".into(),
            },
            SkinEvent {
                tick: 0,
                skin: "off".into(),
            },
        ],
        vec![
            SkinEvent {
                tick: 0,
                skin: "on".into(),
            },
            SkinEvent {
                tick: 99999999,
                skin: "off".into(),
            },
        ],
    ] {
        assert!(
            ReplayTransport::with_skins(&d.skeleton, &clips(&d), "breathing", 1, &bad).is_err()
        );
    }
    assert!(t.play(&c, "breathing", false).is_err());
    t.play(&t.context(), "breathing", false).unwrap();
    let before = current(&d, &t);
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        t.request_tick(&t.context(), 1, |_| panic!("observer failure"))
    }));
    assert!(failure.is_err());
    assert_eq!(t.snapshot().state, TransportState::Faulted);
    assert_eq!(current(&d, &t), before);
    assert!(t.pump(&t.context(), 1, |_| {}).is_err());
    t.reset(&t.context(), t.context().epoch() + 1).unwrap();
    t.dispose(&t.context()).unwrap();
    assert!(t.skin_at(0).is_err());
    assert!(t.reset(&t.context(), 99).is_err());
}
#[test]
fn measured_b5a_control_and_skin_costs() {
    let Some(path) = std::env::var_os("T840B5B1_PERFORMANCE_FILE") else {
        return;
    };
    let d = fixture("control");
    let mut rows = vec![];
    for ticks in [120, 600, 1200] {
        for profile in ["B5A", "fixed", "visual", "membership"] {
            let e = match profile {
                "fixed" => vec![SkinEvent {
                    tick: 0,
                    skin: "default".into(),
                }],
                "visual" => events(true),
                _ => events(false),
            };
            let mut samples = vec![];
            for repeat in 0..35 {
                let start = std::time::Instant::now();
                let mut t = if profile == "B5A" {
                    ReplayTransport::new(&d.skeleton, &clips(&d), "breathing", 1).unwrap()
                } else {
                    transport(&d, &e)
                };
                t.play(&t.context(), "breathing", false).unwrap();
                t.request_tick(&t.context(), ticks, |f| {
                    std::hint::black_box(f);
                })
                .unwrap();
                t.pump(&t.context(), ticks as u32, |f| {
                    std::hint::black_box(f);
                })
                .unwrap();
                if repeat >= 5 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(json!({"ticks":ticks,"profile":profile,"medianMs":samples[samples.len()/2],"samplesMs":samples}));
        }
    }
    let t = transport(&d, &events(false));
    for operation in ["skin-resolution", "pure-sampling", "frame-clone"] {
        let mut samples = vec![];
        for repeat in 0..35 {
            let start = std::time::Instant::now();
            for tick in 0..1000 {
                match operation {
                    "skin-resolution" => {
                        std::hint::black_box(t.skin_at(tick).unwrap());
                    }
                    "pure-sampling" => {
                        std::hint::black_box(t.sample_tick(tick).unwrap());
                    }
                    _ => {
                        std::hint::black_box(t.frame().unwrap().clone());
                    }
                }
            }
            if repeat >= 5 {
                samples.push(start.elapsed().as_secs_f64() * 1000.);
            }
        }
        samples.sort_by(f64::total_cmp);
        rows.push(json!({"operation":operation,"iterations":1000,"medianMs":samples[samples.len()/2],"samplesMs":samples}));
    }
    for (operation, tick, profile) in [
        ("no-change", 120, "fixed"),
        ("visual-event", 120, "visual"),
        ("delete-records", 120, "membership"),
        ("prime-records", 240, "membership"),
    ] {
        let mut samples = vec![];
        for repeat in 0..35 {
            let e = if profile == "fixed" {
                vec![SkinEvent {
                    tick: 0,
                    skin: "default".into(),
                }]
            } else {
                events(profile == "visual")
            };
            let mut t = transport(&d, &e);
            t.play(&t.context(), "breathing", false).unwrap();
            t.request_tick(&t.context(), tick - 1, |_| {}).unwrap();
            t.pump(&t.context(), tick as u32, |_| {}).unwrap();
            let start = std::time::Instant::now();
            t.request_tick(&t.context(), tick, |f| {
                std::hint::black_box(f);
            })
            .unwrap();
            if repeat >= 5 {
                samples.push(start.elapsed().as_secs_f64() * 1000.);
            }
        }
        samples.sort_by(f64::total_cmp);
        rows.push(
            json!({"boundary":operation,"medianMs":samples[samples.len()/2],"samplesMs":samples}),
        );
    }
    let bytes = serde_json::to_vec_pretty(&rows).unwrap();
    use std::io::Write;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap()
        .write_all(&bytes)
        .unwrap();
}
