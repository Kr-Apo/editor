//! Authentic native-export fixtures and controlled replay references for T-839.
//! Optional emission writes only to a new, explicitly supplied evidence folder.
use ankhimate_core::{
    physics::PhysicsState,
    pose::{Pose, evaluate_with},
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{
    convert::{self, Loaded, ProjectRef},
    schema::Project,
};
use serde_json::{Value, json};

fn source(multiple: bool, rotated: bool) -> Loaded {
    let mut project = json!({"version":1,"name":"T839 physics","fps":60,
        "bones":[{"name":"root","length":1,"rotation":if rotated {30.0} else {0.0},"sx":if rotated {1.5} else {1.0},"sy":if rotated {0.75} else {1.0}},
                 {"name":"tassel","parent":"root","length":10,"tx":10},
                 {"name":"tip","parent":"tassel","length":2,"tx":10}],
        "constraints":[{"name":"sway","type":"physics","target":"tassel","bones":[],"mix":1,
            "physics":[0.5,40,0.5,1],"forces":[10,0,0,-9.8],"channels":[true,true]}],
        "constraint_order":["sway"],
        "animations":[{"name":"move","duration":2,"looping":true,"timelines":[
            {"kind":"bone_translate","bone":"root","axis":"x","keys":[
                {"time":0,"value":0,"curve":"linear"},{"time":2,"value":24,"curve":"linear"}]}]}]});
    if multiple {
        let mut second = project["constraints"][0].clone();
        second["name"] = json!("tip-sway");
        second["target"] = json!("tip");
        project["constraints"].as_array_mut().unwrap().push(second);
        project["constraint_order"]
            .as_array_mut()
            .unwrap()
            .push(json!("tip-sway"));
    }
    let loaded = convert::from_schema(&serde_json::from_value::<Project>(project).unwrap());
    assert!(loaded.report.is_clean());
    loaded
}

fn view(loaded: &Loaded) -> ProjectRef<'_> {
    ProjectRef {
        skeleton: &loaded.skeleton,
        animations: &loaded.animations,
        animation_order: &[],
        assets: &loaded.assets,
        name: &loaded.name,
        fps: loaded.fps,
        export_presets: &loaded.export_presets,
        psd_layer_paths: &loaded.psd_layer_paths,
    }
}

fn replay(loaded: &Loaded, deltas: &[f32]) -> Value {
    let mut physics = PhysicsState::new();
    let mut pose = Pose::new();
    let clip = loaded.animations.values().next().unwrap();
    let mut time = 0.0;
    let mut samples = Vec::new();
    for &delta in deltas {
        time += delta;
        evaluate_with(
            &loaded.skeleton,
            &[(clip, time, 1.0)],
            &mut physics,
            delta,
            &mut pose,
        );
        let bones: Vec<_> = loaded.skeleton.update_order.iter().map(|&id| {
            let local = pose.locals[id];
            json!({"name":loaded.skeleton.bones[id].name,"x":local.position.x,"y":local.position.y,
                "rotation":local.rotation,"world":pose.world(id)})
        }).collect();
        samples.push(json!({"dt":delta,"time":time,"bones":bones}));
    }
    json!(samples)
}

#[test]
fn native_physics_target_and_controlled_replays_are_reproducible() {
    let output = std::env::var_os("T839_FIXTURE_DIR").map(std::path::PathBuf::from);
    if let Some(path) = &output {
        // Never overwrite an earlier reference or a project folder.
        std::fs::create_dir(path).unwrap();
    }
    for (name, multiple, rotated) in [
        ("single", false, false),
        ("multiple", true, false),
        ("parent", false, true),
    ] {
        let loaded = source(multiple, rotated);
        let project = convert::to_schema(&view(&loaded));
        assert_eq!(project.constraints[0].target, "tassel");
        assert!(project.constraints[0].bones.is_empty());
        let plan = run::plan(&project, &loaded.assets, &presets::default_preset()).unwrap();
        let native = &plan
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents;
        let parsed: Value = serde_json::from_str(native).unwrap();
        assert_eq!(parsed["format_version"], 2);
        assert_eq!(parsed["constraints"][0]["target"], "tassel");
        assert_eq!(parsed["constraints"][0]["bones"], json!([]));
        let mut references = Vec::new();
        for (label, deltas) in [
            ("60fps", vec![1.0 / 60.0; 60]),
            ("144fps", vec![1.0 / 144.0; 144]),
            ("hitch", vec![0.0, 0.5, 0.0, 1.0 / 60.0]),
            ("zero-dt", vec![1.0 / 60.0, 0.0, 0.0, 1.0 / 60.0]),
        ] {
            let samples = replay(&loaded, &deltas);
            assert_eq!(
                samples,
                replay(&loaded, &deltas),
                "reset replay: {name}/{label}"
            );
            references.push(json!({"label":label,"samples":samples}));
        }
        let author = ankhimate_formats::to_json(&view(&loaded)).unwrap();
        let reopened = ankhimate_formats::from_json(&author).unwrap();
        assert_eq!(
            replay(&loaded, &[1.0 / 60.0; 60]),
            replay(&reopened, &[1.0 / 60.0; 60])
        );
        if let Some(path) = &output {
            std::fs::write(path.join(format!("{name}.ankh.json")), author).unwrap();
            std::fs::write(path.join(format!("{name}.native.json")), native).unwrap();
            std::fs::write(
                path.join(format!("{name}.reference.json")),
                serde_json::to_vec_pretty(&references).unwrap(),
            )
            .unwrap();
        }
    }
}
