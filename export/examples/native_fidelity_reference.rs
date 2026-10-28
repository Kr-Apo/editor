//! Headless author/save/reopen/native-export reference for T-840C9-A2.
//! Creates a NEW output tree only. No GUI, runtime mutation or historical fixture rewrite.
#[path = "../tests/support/native_fidelity.rs"]
mod support;
use ankhimate_core::{
    pose::{self, Pose},
    replay_animation::canonical_time,
};
use ankhimate_export::run;
use ankhimate_formats::convert;
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn pose_at(d: &ankhimate_formats::Loaded, time: f32) -> Value {
    let a = d.animations.values().next().unwrap();
    let mut p = Pose::new();
    pose::evaluate(&d.skeleton, &[(a, time, 1.0)], &mut p);
    let bones=d.skeleton.bones.iter().map(|(id,b)|{let t=p.locals[id];let w=p.worlds[id];json!({"name":b.name,"x":t.position.x,"y":t.position.y,"rotation":t.rotation,"scaleX":t.scale.x,"scaleY":t.scale.y,"shearX":t.shear.x,"shearY":t.shear.y,"world":{"a":w.a,"b":w.b,"c":w.c,"d":w.d,"tx":w.tx,"ty":w.ty}})}).collect::<Vec<_>>();
    let slots = d
        .skeleton
        .slots
        .iter()
        .map(|(id, s)| json!({"name":s.name,"color":p.slot_colors[id]}))
        .collect::<Vec<_>>();
    let mixes=p.transform_mix.iter().map(|(id,m)|json!({"constraint":d.skeleton.constraints[id].name(),"rotate":m.rotate,"translate_x":m.translate.x,"translate_y":m.translate.y,"scale_x":m.scale.x,"scale_y":m.scale.y,"shear_x":m.shear.x,"shear_y":m.shear.y})).collect::<Vec<_>>();
    json!({"time":time,"bones":bones,"slots":slots,"mixes":mixes})
}
fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("NEW_OUTPUT"));
    fs::create_dir(&out).expect("refuse existing output");
    let mut cases = vec![];
    for label in [
        "color-linear",
        "color-stepped",
        "color-bezier",
        "color-multiple",
        "color-constant",
        "transform-linear",
        "transform-stepped",
        "transform-bezier",
        "transform-multiple",
        "transform-constant",
        "duration-30",
        "duration-60",
        "duration-083",
        "duration-2",
        "duration-f32",
        "duration-courier",
    ] {
        let dir = out.join(label);
        let (d, p, plan) = support::saved_export(label, &dir);
        let native = support::wire(&plan);
        fs::write(
            dir.join("author.json"),
            serde_json::to_vec_pretty(&p).unwrap(),
        )
        .unwrap();
        let schema_after = convert::to_schema(&support::view(&d));
        assert_eq!(p, schema_after);
        let second = run::plan(
            &schema_after,
            &d.assets,
            &ankhimate_export::presets::default_preset(),
        )
        .unwrap();
        assert_eq!(plan.files, second.files);
        assert_eq!(plan.binaries, second.binaries);
        let export = dir.join("native");
        run::write(&plan, &export).unwrap();
        assert_eq!(
            fs::read_to_string(export.join("skeleton.json")).unwrap(),
            plan.files[0].contents
        );
        let a = d.animations.values().next().unwrap();
        let duration = a.duration;
        let mut samples = vec![];
        let mut ticks = vec![];
        let mut continuous = vec![];
        if label.starts_with("duration-") {
            let mut checkpoints = vec![0, 1, 2, 3, 4, 119, 120, 121, 239, 240, 241, 480];
            for loops in [1, 3, 7] {
                let near = (f64::from(duration) * 120.0 * f64::from(loops)).floor() as u64;
                checkpoints.extend([near.saturating_sub(1), near, near + 1]);
            }
            checkpoints.sort();
            checkpoints.dedup();
            for tick in checkpoints {
                let time = canonical_time(tick, duration, true).unwrap();
                ticks.push(json!({"tick":tick,"canonicalTime":time,"pose":pose_at(&d,time)}));
            }
            let dur = f64::from(duration);
            let epsilon = dur * 1e-6;
            for absolute in [
                0.0,
                dur - epsilon,
                dur,
                dur + epsilon,
                3.0 * dur - epsilon,
                3.0 * dur,
                3.0 * dur + epsilon,
                7.0 * dur,
                1.0,
                2.0,
            ] {
                let time = (absolute % dur) as f32;
                continuous.push(json!({"absoluteSeconds":absolute,"canonicalTime":time,"pose":pose_at(&d,time)}));
            }
        } else {
            for time in [
                0.0, 0.25, 0.4999, 0.5, 0.5001, 0.75, 0.9999, 1.0, 1.0001, 1.25, 1.5, 1.75, 2.0,
            ] {
                samples.push(pose_at(&d, time));
            }
        }
        let reference = json!({"case":label,"duration":duration,"durationBits":duration.to_bits(),"looping":a.looping,"samples":samples,"ticks":ticks,"continuous":continuous,"saveReloadExact":true,"assetsExact":true,"twoExportsByteIdentical":true,"nativeFormatVersion":native["format_version"]});
        fs::write(
            dir.join("rust.json"),
            serde_json::to_vec_pretty(&reference).unwrap(),
        )
        .unwrap();
        cases.push(label);
    }
    fs::write(
        out.join("CASES.json"),
        serde_json::to_vec_pretty(&cases).unwrap(),
    )
    .unwrap();
    println!("{} real Save/reopen/export cases prepared", cases.len());
}
