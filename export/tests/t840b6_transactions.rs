use ankhimate_core::{replay_clock::Stop, replay_physics::ReplayPhysicsInstance};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
fn fixture(name: &str) -> ankhimate_formats::Loaded {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/t840b6/{name}.ankh.json"));
    let d = ankhimate_formats::from_json(&std::fs::read_to_string(p).unwrap()).unwrap();
    assert!(d.report.is_clean());
    ankhimate_formats::from_json(&ankhimate_formats::to_json(&view(&d)).unwrap()).unwrap()
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
#[test]
fn second_constraint_failure_restores_private_records_not_only_public_frame() {
    let d = fixture("independent");
    assert!(d.report.is_clean());
    if let Some(dir) = std::env::var_os("T840B6_RED_REFERENCE") {
        std::fs::create_dir(&dir).unwrap();
        let bytes = run::plan(
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
        std::fs::write(
            std::path::PathBuf::from(dir).join("independent.native.json"),
            bytes,
        )
        .unwrap();
    }
    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    let before = format!("{:?}", s.numerical_state());
    let frame = format!("{:?}", s.frame());
    let pivot = d
        .skeleton
        .update_order
        .iter()
        .find(|b| d.skeleton.bones[**b].name == "pivot-B")
        .unwrap();
    let mut t = d.skeleton.bones[*pivot].local_transform;
    t.scale.x = 0.;
    let p = s
        .request_tick(
            &s.context(),
            1,
            |_| Ok(vec![(*pivot, t)]),
            |_| panic!("refused frame must not be observed"),
        )
        .unwrap();
    assert!(matches!(p.stop, Stop::Failed(_)));
    assert_eq!(s.clock().completed, 0);
    assert_eq!(s.clock().backlog, 1);
    assert_eq!(format!("{:?}", s.frame()), frame);
    let after = format!("{:?}", s.numerical_state());
    println!("BEFORE {before}\nAFTER {after}");
    assert_eq!(
        after, before,
        "private state changed after A succeeded and B failed"
    );
}
#[test]
fn measured_transaction_workloads() {
    let Some(path) = std::env::var_os("T840B6_PERFORMANCE_FILE") else {
        return;
    };
    let mut rows = vec![];
    for name in ["single", "independent", "dependent", "medium", "many"] {
        let d = fixture(name);
        let all: Vec<_> = d.animations.values().cloned().collect();
        for operation in [
            "one-tick",
            "advance-120",
            "advance-1200",
            "snapshot-100",
            "failure-second",
            "failure-observer",
            "eight-owners",
        ] {
            if name == "single" && operation == "failure-second" {
                continue;
            }
            let mut samples = vec![];
            for repeat in 0..13 {
                let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
                let start = std::time::Instant::now();
                match operation {
                    "one-tick" => {
                        std::hint::black_box(
                            s.request_tick(&s.context(), 1, |_| Ok(vec![]), |_| {})
                                .unwrap(),
                        );
                    }
                    "snapshot-100" => {
                        for _ in 0..100 {
                            std::hint::black_box(s.numerical_state());
                        }
                    }
                    "failure-second" => {
                        let id = d
                            .skeleton
                            .update_order
                            .iter()
                            .find(|b| d.skeleton.bones[**b].name == "pivot-B")
                            .unwrap();
                        let mut t = d.skeleton.bones[*id].local_transform;
                        t.scale.x = 0.;
                        std::hint::black_box(
                            s.request_tick(&s.context(), 1, |_| Ok(vec![(*id, t)]), |_| {})
                                .unwrap(),
                        );
                    }
                    "failure-observer" => {
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            s.request_tick(
                                &s.context(),
                                1,
                                |_| Ok(vec![]),
                                |_| panic!("benchmark audit"),
                            )
                        }));
                    }
                    _ => {
                        let count = if operation == "eight-owners" { 8 } else { 1 };
                        let mut owners: Vec<_> = (0..count)
                            .map(|_| {
                                ankhimate_core::replay_consumer::ReplayConsumerBridge::new(
                                    &d.skeleton,
                                    &all,
                                    "breathing",
                                    1,
                                    None,
                                )
                                .unwrap()
                            })
                            .collect();
                        for b in &mut owners {
                            b.play(&b.context(), "breathing", false).unwrap();
                            let ticks = if operation == "advance-1200" {
                                1200
                            } else {
                                120
                            };
                            b.request_tick(&b.context(), ticks, |_| {}).unwrap();
                            while b.transport_status().clock.unwrap().backlog > 0 {
                                b.pump(&b.context(), 128, |_| {}).unwrap();
                            }
                        }
                    }
                }
                if repeat >= 3 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(serde_json::json!({"rig":name,"bones":d.skeleton.bones.len(),"constraints":d.skeleton.constraints.len(),"operation":operation,"medianMs":samples[samples.len()/2],"minMs":samples[0],"maxMs":samples.last(),"samplesMs":samples}));
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
#[test]
fn genuine_transaction_fixtures() {
    let Some(dir) = std::env::var_os("T840B6_FIXTURE_DIR") else {
        return;
    };
    std::fs::create_dir(&dir).unwrap();
    for name in ["single", "independent", "dependent", "medium", "many"] {
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
        std::fs::write(
            std::path::PathBuf::from(&dir).join(format!("{name}.native.json")),
            native,
        )
        .unwrap();
        std::fs::write(
            std::path::PathBuf::from(&dir).join(format!("{name}.ankh.json")),
            ankhimate_formats::to_json(&view(&d)).unwrap(),
        )
        .unwrap();
    }
}
fn events() -> Vec<ankhimate_core::replay_skin::SkinEvent> {
    use ankhimate_core::replay_skin::SkinEvent;
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
fn inputs(
    d: &ankhimate_formats::Loaded,
    p: &ankhimate_core::pose::Pose,
) -> ankhimate_core::replay_physics::ReplayInputs {
    d.skeleton
        .update_order
        .iter()
        .map(|b| (*b, p.locals[*b]))
        .collect()
}
fn records(
    d: &ankhimate_formats::Loaded,
    rs: &[ankhimate_core::replay_physics::ReplayRecord],
) -> serde_json::Value {
    serde_json::json!(rs.iter().map(|r|serde_json::json!({"constraint":d.skeleton.constraints[r.constraint].name(),"values":[r.rotation,r.position.x,r.position.y,r.rotation_velocity,r.position_velocity.x,r.position_velocity.y,r.anchor.x,r.anchor.y]})).collect::<Vec<_>>())
}
fn state(d: &ankhimate_formats::Loaded, s: &ReplayPhysicsInstance) -> serde_json::Value {
    let n = s.numerical_state();
    let f = s.frame();
    let c = s.clock();
    serde_json::json!({"epoch":c.epoch.to_string(),"requested":c.requested.to_string(),"completed":c.completed.to_string(),"backlog":c.backlog.to_string(),"faulted":s.faulted(),"skin":s.skin_at(f.tick).unwrap().map(|x|x.skin).unwrap_or_else(||d.skeleton.skins[d.skeleton.default_skin].name.clone()),
 "active":n.active.iter().map(|id|d.skeleton.constraints[*id].name()).collect::<Vec<_>>(),"private":records(d,&n.records),"frame":{"tick":f.tick.to_string(),"records":records(d,&f.records),"bones":d.skeleton.update_order.iter().map(|id|{let t=f.pose.locals[*id];let w=f.pose.world(*id);serde_json::json!({"name":d.skeleton.bones[*id].name,"values":[t.position.x,t.position.y,t.rotation,t.scale.x,t.scale.y,t.shear.x,t.shear.y,w.a,w.b,w.c,w.d,w.tx,w.ty]})}).collect::<Vec<_>>()}})
}
fn bits(s: &ReplayPhysicsInstance) -> Vec<(ankhimate_core::ids::ConstraintId, [u32; 8])> {
    s.numerical_state()
        .records
        .iter()
        .map(|r| {
            (
                r.constraint,
                [
                    r.rotation,
                    r.position.x,
                    r.position.y,
                    r.rotation_velocity,
                    r.position_velocity.x,
                    r.position_velocity.y,
                    r.anchor.x,
                    r.anchor.y,
                ]
                .map(f32::to_bits),
            )
        })
        .collect()
}
#[test]
fn native_fault_campaign_exact_private_rollback_and_reset_replay() {
    use ankhimate_core::replay_animation::ReplayAnimationInstance;
    use serde_json::json;
    let dir = std::env::var_os("T840B6_REFERENCE_DIR").map(std::path::PathBuf::from);
    if let Some(p) = &dir {
        std::fs::create_dir(p).unwrap();
    }
    let mut rows = vec![];
    for name in ["single", "independent", "dependent", "medium", "many"] {
        let d = fixture(name);
        let source = ankhimate_formats::to_json(&view(&d)).unwrap();
        let all: Vec<_> = d.animations.values().cloned().collect();
        let e = events();
        let pure =
            ReplayAnimationInstance::with_skins(&d.skeleton, &all, "breathing", 1, &e).unwrap();
        let initial = inputs(&d, &pure.sample_tick(0).unwrap());
        if let Some(p) = &dir {
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
            std::fs::write(p.join(format!("{name}.native.json")), native).unwrap();
            std::fs::write(p.join(format!("{name}.ankh.json")), &source).unwrap();
        }
        let scenarios: Vec<_> = if ["medium", "many"].contains(&name) {
            vec![("second", 9), ("observer", 9)]
        } else {
            let mut v = vec![
                ("observer", 9),
                ("observer", 120),
                ("observer", 240),
                ("provider", 9),
                ("nonfinite", 9),
                ("unknown", 9),
                ("cancel", 9),
            ];
            if name != "single" {
                v.extend([
                    ("second", 1),
                    ("second", 9),
                    ("second", 120),
                    ("second", 240),
                ]);
            }
            v
        };
        for (fault, tick) in scenarios {
            let mut s = ReplayPhysicsInstance::with_skins(&d.skeleton, 1, &initial, &e).unwrap();
            let other = ReplayPhysicsInstance::with_skins(&d.skeleton, 1, &initial, &e).unwrap();
            let unaffected = state(&d, &other);
            for n in 1..tick {
                s.request_tick(
                    &s.context(),
                    n,
                    |t| Ok(inputs(&d, &pure.sample_tick(t.index).unwrap())),
                    |_| {},
                )
                .unwrap();
            }
            let before = state(&d, &s);
            let before_bits = bits(&s);
            let active = s.numerical_state().active;
            let target = tick + 51;
            if fault == "observer" {
                let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    s.request_tick(
                        &s.context(),
                        target,
                        |t| Ok(inputs(&d, &pure.sample_tick(t.index).unwrap())),
                        |_| panic!("rejected precommit candidate"),
                    )
                }));
                assert!(panic.is_err());
            } else if fault == "cancel" {
                let p = s.queue_time(&s.context(), target, 120).unwrap();
                assert!(matches!(p.stop, Stop::Cancelled));
            } else {
                let p = s
                    .request_tick(
                        &s.context(),
                        target,
                        |t| {
                            if fault == "provider" {
                                return Err("sampler error".into());
                            }
                            let mut v = inputs(&d, &pure.sample_tick(t.index).unwrap());
                            if fault == "second" {
                                let (id, tr) = v
                                    .iter_mut()
                                    .find(|(id, _)| d.skeleton.bones[*id].name == "pivot-B")
                                    .unwrap();
                                let _ = id;
                                tr.scale.x = 0.;
                            }
                            if fault == "nonfinite" {
                                v[0].1.position.x = f32::NAN;
                            }
                            if fault == "unknown" {
                                v.push((
                                    ankhimate_core::ids::BoneId::default(),
                                    ankhimate_core::math::Transform::default(),
                                ));
                            }
                            Ok(v)
                        },
                        |_| panic!("failed computation must not notify"),
                    )
                    .unwrap();
                assert!(matches!(p.stop, Stop::Failed(_)));
            }
            let after = state(&d, &s);
            assert_eq!(
                bits(&s),
                before_bits,
                "bitwise private rollback {name}/{fault}/{tick}"
            );
            assert_eq!(s.numerical_state().active, active);
            assert_eq!(before["frame"], after["frame"]);
            assert_eq!(s.clock().completed, tick - 1);
            assert_eq!(s.clock().backlog, 52);
            assert_eq!(s.faulted(), fault != "cancel");
            assert_eq!(state(&d, &other), unaffected);
            assert_eq!(ankhimate_formats::to_json(&view(&d)).unwrap(), source);
            if fault != "cancel" {
                assert!(s.pump(&s.context(), 1, |_| Ok(vec![]), |_| {}).is_err());
            }
            let old = s.context();
            s.reset(&d.skeleton, 2, &initial).unwrap();
            assert!(s.request_tick(&old, 1, |_| Ok(vec![]), |_| {}).is_err());
            // Public B3 reset intentionally selects its fixed-default legacy membership.
            let mut control = ReplayPhysicsInstance::new(&d.skeleton, 2, &initial).unwrap();
            for n in 1..=360 {
                let provider = |t: ankhimate_core::replay_clock::ScheduledTick| {
                    Ok(inputs(&d, &pure.sample_tick(t.index).unwrap()))
                };
                s.request_tick(&s.context(), n, provider, |_| {}).unwrap();
                control
                    .request_tick(&control.context(), n, provider, |_| {})
                    .unwrap();
                assert_eq!(state(&d, &s), state(&d, &control));
            }
            rows.push(json!({"name":format!("{name}/{fault}/{tick}"),"rig":name,"fault":fault,"tick":tick.to_string(),"target":target.to_string(),"before":before,"after":after,"reconstructed":state(&d,&s),"rollbackExact":true}));
        }
    }
    if let Some(p) = dir {
        std::fs::write(p.join("cases.json"), serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}
#[test]
fn successful_prefix_inside_one_request_is_retained() {
    let d = fixture("independent");
    let mut s = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    let mut control = ReplayPhysicsInstance::new(&d.skeleton, 1, &[]).unwrap();
    control
        .request_tick(&control.context(), 3, |_| Ok(vec![]), |_| {})
        .unwrap();
    let pivot = d
        .skeleton
        .update_order
        .iter()
        .find(|b| d.skeleton.bones[**b].name == "pivot-B")
        .unwrap();
    let mut bad = d.skeleton.bones[*pivot].local_transform;
    bad.scale.x = 0.;
    let p = s
        .request_tick(
            &s.context(),
            60,
            |tick| {
                Ok(if tick.index == 4 {
                    vec![(*pivot, bad)]
                } else {
                    vec![]
                })
            },
            |_| {},
        )
        .unwrap();
    assert!(matches!(p.stop, Stop::Failed(_)));
    assert_eq!(p.processed, 3);
    assert_eq!(s.clock().requested, 60);
    assert_eq!(s.clock().completed, 3);
    assert_eq!(s.clock().backlog, 57);
    assert!(s.faulted());
    assert_eq!(bits(&s), bits(&control));
    assert_eq!(state(&d, &s)["frame"], state(&d, &control)["frame"]);
}
