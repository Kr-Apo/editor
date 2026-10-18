//! M001a reference: the real native template and Rust poses for the same rig.
//! Emits JSON to stdout. No files or authored documents are modified.
//! See docs/milestones/M001a-validation.md for the cross-repository gate.

use ankhimate_core::pose::{Pose, evaluate};
use ankhimate_export::{
    context::{Context, ExportInfo},
    presets,
    template::Engine,
};
use ankhimate_formats::{convert, schema::Project};
use serde_json::{Value, json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project: Project =
        serde_json::from_str(include_str!("../tests/fixtures/runtime_axes.ankh.json"))?;
    let loaded = convert::from_schema(&project);
    assert!(
        loaded.report.is_clean(),
        "the parity fixture must load losslessly"
    );
    let context = Context::build(&project, None, ExportInfo::default());
    let preset = presets::default_preset();
    let rig: Value = serde_json::from_str(&Engine::new().render(
        "runtime axes parity",
        &preset.templates[0].body,
        &context.to_value(),
    )?)?;
    let mut cases = Vec::new();
    let times = [
        -0.25, 0.0, 0.15, 0.25, 0.5, 0.75, 0.8, 1.0, 1.25, 1.6, 1.8, 2.0,
    ];
    for animation in &project.animations {
        for time in times {
            cases.push(reference_case(
                &loaded,
                vec![json!({"animation": animation.name, "time": time, "alpha": 1.0})],
            ));
        }
    }
    for alpha in [0.0, 0.35, 0.5, 1.0] {
        cases.push(reference_case(
            &loaded,
            vec![json!({"animation": "independent_bezier", "time": 0.65, "alpha": alpha})],
        ));
        cases.push(reference_case(
            &loaded,
            vec![
                json!({"animation": "independent_step", "time": 0.75, "alpha": 1.0-alpha}),
                json!({"animation": "independent_bezier", "time": 0.65, "alpha": alpha}),
            ],
        ));
    }
    println!(
        "{}",
        serde_json::to_string(&json!({
            "source": "export/tests/fixtures/runtime_axes.ankh.json",
            "tolerance": 0.01,
            "rig": rig,
            "cases": cases,
        }))?
    );
    Ok(())
}

fn reference_case(loaded: &convert::Loaded, layers: Vec<Value>) -> Value {
    let animations: Vec<_> = layers
        .iter()
        .map(|layer| {
            let name = layer["animation"].as_str().unwrap();
            let animation = loaded
                .animations
                .values()
                .find(|animation| animation.name == name)
                .expect("the fixture names an existing clip");
            (
                animation,
                layer["time"].as_f64().unwrap() as f32,
                layer["alpha"].as_f64().unwrap() as f32,
            )
        })
        .collect();
    let mut pose = Pose::new();
    evaluate(&loaded.skeleton, &animations, &mut pose);
    let bones: Vec<_> = loaded
        .skeleton
        .update_order
        .iter()
        .map(|id| {
            let local = pose.locals[*id];
            let world = pose.world(*id);
            json!({
                "name": loaded.skeleton.bones[*id].name,
                "local": [local.position.x, local.position.y, local.rotation,
                    local.scale.x, local.scale.y, local.shear.x, local.shear.y],
                "world": [world.a, world.b, world.c, world.d, world.tx, world.ty],
            })
        })
        .collect();
    json!({"layers": layers, "bones": bones})
}
