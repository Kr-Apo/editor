use super::*;
use crate::{constraints::PhysicsConstraint, replay_skin::SkinEvent, skeleton::Bone, skin::Skin};
fn rig() -> Skeleton {
    let mut r = Skeleton::new();
    let root = r.add_bone(bone("root", None));
    let a = r.add_bone(bone("A", Some(root)));
    let b = r.add_bone(bone("B", Some(root)));
    let c = r.add_bone(bone("C", Some(a)));
    for (name, id) in [("A", a), ("B", b), ("C", c)] {
        let mut p = PhysicsConstraint::sway(name, id);
        p.wind = Vec2::new(10., 0.);
        p.translate = true;
        r.add_constraint(Constraint::Physics(p));
    }
    let ids = r.constraint_order.clone();
    r.skins[r.default_skin].constraints = ids.clone();
    let mut on = Skin::new("on");
    on.constraints = ids.clone();
    r.add_skin(on);
    let mut off = Skin::new("off");
    off.constraints = vec![ids[1]];
    r.add_skin(off);
    r
}
fn bone(name: &str, parent: Option<BoneId>) -> Bone {
    Bone {
        name: name.into(),
        parent,
        length: 10.,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}
fn bits(s: &ReplayPhysicsInstance) -> Vec<(ConstraintId, [u32; 8])> {
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
fn returned_errors_and_unwinds_drop_all_candidate_mutations() {
    for transition in ["stable", "remove", "prime"] {
        let r = rig();
        let calendar = vec![
            SkinEvent {
                tick: 0,
                skin: if transition == "prime" { "off" } else { "on" }.into(),
            },
            SkinEvent {
                tick: 9,
                skin: if transition == "remove" { "off" } else { "on" }.into(),
            },
        ];
        let points = if transition == "remove" { 1 } else { 3 };
        for point in 0..points {
            for field in 0..6 {
                for unwind in [false, true] {
                    let mut s = ReplayPhysicsInstance::with_skins(&r, 1, &[], &calendar).unwrap();
                    s.request_tick(&s.context(), 8, |_| Ok(vec![]), |_| {})
                        .unwrap();
                    let before = bits(&s);
                    let active = s.active.clone();
                    let frame = format!("{:?}", s.frame);
                    let mut count = 0;
                    let mut step = |rig: &Skeleton,
                                    pose: &mut Pose,
                                    p: &PhysicsConstraint,
                                    record: &mut Record,
                                    anchor: Vec2,
                                    inverse: Affine2| {
                        integrate(rig, pose, p, record, anchor, inverse)?;
                        let current = count;
                        count += 1;
                        if current == point {
                            match field {
                                0 => record.sim.position.x = f32::NAN,
                                1 => record.sim.position_velocity.y = f32::INFINITY,
                                2 => record.sim.rotation = f32::NAN,
                                3 => record.sim.rotation_velocity = f32::NEG_INFINITY,
                                4 => record.anchor.x = f32::NAN,
                                _ => {
                                    pose.locals[p.bone].position.x = f32::NAN;
                                    pose.worlds[p.bone].tx = f32::INFINITY;
                                }
                            }
                            if unwind {
                                panic!("test-only failure after native integration");
                            }
                            return Err(ReplayError::Numerical);
                        }
                        Ok(())
                    };
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let ReplayPhysicsInstance {
                            rig,
                            clock,
                            records,
                            frame,
                            faulted,
                            active,
                            calendar,
                            ..
                        } = &mut s;
                        clock
                            .request_time(1, 60, 120, |tick| {
                                run_with(
                                    rig,
                                    records,
                                    frame,
                                    faulted,
                                    active,
                                    calendar.as_ref(),
                                    tick,
                                    &mut |_| Ok(vec![]),
                                    &mut |_| panic!("no rejected candidate publication"),
                                    &mut step,
                                )
                            })
                            .unwrap()
                    }));
                    assert_eq!(outcome.is_err(), unwind);
                    if let Ok(p) = outcome {
                        assert!(matches!(p.stop, crate::replay_clock::Stop::Failed(_)));
                    }
                    assert_eq!(count, point + 1);
                    assert_eq!(bits(&s), before);
                    assert_eq!(s.active, active);
                    assert_eq!(format!("{:?}", s.frame), frame);
                    assert!(s.faulted());
                    assert_eq!(s.clock().completed, 8);
                    assert_eq!(s.clock().backlog, 52);
                    assert!(s.pump(&s.context(), 1, |_| Ok(vec![]), |_| {}).is_err());
                }
            }
        }
    }
}
#[test]
fn real_finite_force_overflow_in_second_spring_restores_first() {
    let mut r = rig();
    let id = r.constraint_order[1];
    let Constraint::Physics(p) = &mut r.constraints[id] else {
        unreachable!()
    };
    p.wind.x = f32::MAX;
    p.mass = 0.01;
    let mut s = ReplayPhysicsInstance::new(&r, 1, &[]).unwrap();
    let before = bits(&s);
    let frame = format!("{:?}", s.frame());
    let p = s
        .request_tick(
            &s.context(),
            1,
            |_| Ok(vec![]),
            |_| panic!("nonfinite candidate must not publish"),
        )
        .unwrap();
    assert!(matches!(
        p.stop,
        crate::replay_clock::Stop::Failed(ReplayError::Numerical)
    ));
    assert_eq!(bits(&s), before);
    assert_eq!(format!("{:?}", s.frame()), frame);
    assert_eq!(s.clock().completed, 0);
    assert!(s.faulted());
}
#[test]
fn candidate_copy_and_accept_component_measurements() {
    let Some(path) = std::env::var_os("T840B6_COMPONENT_FILE") else {
        return;
    };
    let mut rows = vec![];
    for n in [1, 3, 64] {
        let mut r = rig();
        let first = r.constraint_order[0];
        if n == 1 {
            r.constraints.retain(|id, _| id == first);
            r.constraint_order = vec![first];
            for s in r.skins.values_mut() {
                s.constraints.retain(|c| *c == first);
            }
        }
        while r.constraints.len() < n {
            let mut p = match &r.constraints[first] {
                Constraint::Physics(p) => p.clone(),
                _ => unreachable!(),
            };
            p.name = format!("extra-{}", r.constraints.len());
            r.add_constraint(Constraint::Physics(p));
        }
        let mut s = ReplayPhysicsInstance::new(&r, 1, &[]).unwrap();
        for operation in ["candidate", "accept", "discard"] {
            let mut samples = vec![];
            for repeat in 0..23 {
                let mut prepared = Vec::new();
                if operation != "candidate" {
                    for _ in 0..1000 {
                        prepared.push((candidate(&s.records, &s.active), s.frame.clone()));
                    }
                }
                let start = std::time::Instant::now();
                if operation == "candidate" {
                    for _ in 0..1000 {
                        std::hint::black_box(candidate(&s.records, &s.active));
                    }
                } else if operation == "discard" {
                    drop(prepared);
                } else {
                    for (next, f) in prepared {
                        accept(&mut s.records, &mut s.active, &mut s.frame, next, f);
                    }
                }
                if repeat >= 3 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.);
                }
            }
            samples.sort_by(f64::total_cmp);
            rows.push(format!("{{\"constraints\":{n},\"operation\":\"{operation}\",\"iterations\":1000,\"medianMs\":{},\"minMs\":{},\"maxMs\":{}}}",samples[samples.len()/2],samples[0],samples[samples.len()-1]));
        }
    }
    use std::io::Write;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap()
        .write_all(format!("[{}]", rows.join(",")).as_bytes())
        .unwrap();
}
