//! Shared author fixtures for export fidelity tests and the headless reference example.
use ankhimate_core::assets::ImageAsset;
use ankhimate_export::{presets, run};
use ankhimate_formats::{Loaded, ProjectRef, convert, schema};
use image::{ImageFormat, Rgba, RgbaImage};
use serde_json::{Value, json};
use std::path::Path;

pub fn view(d: &Loaded) -> ProjectRef<'_> {
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

pub fn curve(kind: &str) -> schema::Interp {
    match kind {
        "stepped" => schema::Interp::Stepped,
        "bezier" => schema::Interp::Bezier {
            handles: [0.2, 0.0, 0.8, 0.0],
        },
        "overshoot" => schema::Interp::Bezier {
            handles: [0.2, -0.5, 0.8, 1.5],
        },
        _ => schema::Interp::Linear,
    }
}

pub fn author(label: &str) -> Loaded {
    let mut value = json!({"version":1,"name":label,"fps":60,
        "bones":[{"name":"root","length":10}],
        "slots":[{"name":"art","bone":"root","attachment":"image","color":[1,1,1,1]}],
        "skins":[{"name":"default","entries":[{"slot":"art","name":"image","attachment":{"type":"region","texture":"image","width":4,"height":4,"uv":[0,0,1,1]}}]}],
        "default_skin":"default","draw_order":["art"],
        "animations":[{"name":"clip","duration":2,"looping":true,"timelines":[]}]});
    if label.starts_with("color-") {
        let keys = if label == "color-multiple" {
            vec![
                (0.0, [1.0; 4], curve("overshoot")),
                (0.5, [0.2, 0.4, 0.6, 0.8], curve("stepped")),
                (1.0, [0.8, 0.6, 0.4, 0.2], curve("overshoot")),
                (1.5, [0.5; 4], curve("linear")),
            ]
        } else if label == "color-constant" {
            vec![(0.0, [0.4; 4], curve("bezier"))]
        } else {
            vec![
                (0.0, [1.0; 4], curve("linear")),
                (1.0, [0.0; 4], curve(label.strip_prefix("color-").unwrap())),
            ]
        };
        let p: &mut Value = &mut value["animations"][0]["timelines"];
        *p = json!([{"kind":"slot_color","slot":"art","keys":keys.into_iter().map(|(time,value,interp)|schema::ColorKey{time,value,interp}).collect::<Vec<_>>()}]);
    } else if label.starts_with("transform-") {
        value["bones"].as_array_mut().unwrap().push(json!({"name":"target","length":1,"tx":10,"ty":6,"rotation":30,"sx":1.2,"sy":0.8,"shear_x":5,"shear_y":-3}));
        // Local mode exercises all seven supported mixes without decomposition ambiguity.
        value["constraints"] = json!([{"name":"follow","type":"transform","target":"target","bones":["root"],"local":true,"transform_mix":{}}]);
        value["constraint_order"] = json!(["follow"]);
        let mix = |factor: f32| schema::TransformMix {
            rotate: 0.6 * factor,
            translate_x: factor,
            translate_y: 0.7 * factor,
            scale_x: 0.6 * factor,
            scale_y: 0.3 * factor,
            shear_x: 0.9 * factor,
            shear_y: 0.5 * factor,
        };
        let values = if label == "transform-multiple" {
            vec![
                (0.0, 0.0, curve("bezier")),
                (0.5, 0.8, curve("stepped")),
                (1.0, 0.2, curve("overshoot")),
                (1.5, 1.0, curve("linear")),
            ]
        } else if label == "transform-constant" {
            vec![(0.0, 0.5, curve("bezier"))]
        } else {
            vec![
                (0.0, 0.0, curve("linear")),
                (1.0, 1.0, curve(label.strip_prefix("transform-").unwrap())),
            ]
        };
        let keys = values
            .into_iter()
            .map(|(time, value, interp)| schema::MixKey {
                time,
                value: mix(value),
                interp,
                extra: Default::default(),
            })
            .collect::<Vec<_>>();
        value["animations"][0]["timelines"] =
            json!([{"kind":"transform_constraint_mix","constraint":"follow","keys":keys}]);
    } else if label.starts_with("duration-") {
        let duration = duration(label);
        value["animations"][0]["duration"] = json!(duration);
        value["animations"][0]["timelines"] = json!([{"kind":"bone_translate","bone":"root","axis":"x","keys":[{"time":0,"value":0,"curve":"linear"},{"time":duration,"value":10,"curve":"linear"}]}]);
    }
    let project: schema::Project = serde_json::from_value(value).unwrap();
    let mut d = convert::from_schema(&project);
    assert!(d.report.dangling.is_empty());
    let img = RgbaImage::from_pixel(4, 4, Rgba([100, 160, 220, 180]));
    let mut bytes = std::io::Cursor::new(Vec::new());
    img.write_to(&mut bytes, ImageFormat::Png).unwrap();
    d.assets
        .add(ImageAsset::new("image", bytes.into_inner(), 4, 4));
    d
}

pub fn duration(label: &str) -> f32 {
    match label {
        "duration-30" => 1.0 / 30.0,
        "duration-60" => 1.0 / 60.0,
        "duration-083" => 0.83,
        "duration-2" => 2.0,
        "duration-f32" => f32::from_bits(0x3eaaaaab),
        "duration-courier" => 3.0,
        _ => panic!("unknown duration fixture"),
    }
}

pub fn saved_export(label: &str, directory: &Path) -> (Loaded, schema::Project, run::Plan) {
    std::fs::create_dir_all(directory).unwrap();
    let path = directory.join("Character.ankh");
    let d = author(label);
    let before = convert::to_schema(&view(&d));
    ankhimate_formats::save(&path, &view(&d), &[]).unwrap();
    let (after, extra) = ankhimate_formats::load(&path).unwrap();
    assert!(extra.is_empty());
    assert!(after.report.dangling.is_empty());
    assert_eq!(
        before,
        convert::to_schema(&view(&after)),
        "save/reload author data"
    );
    for asset in d.assets.images.values() {
        let id = after.assets.by_name(&asset.name).unwrap();
        assert_eq!(
            asset.bytes, after.assets.images[id].bytes,
            "asset round trip"
        );
    }
    let schema = convert::to_schema(&view(&after));
    let plan = run::plan(&schema, &after.assets, &presets::default_preset()).unwrap();
    (after, schema, plan)
}

pub fn wire(plan: &run::Plan) -> Value {
    serde_json::from_str(
        &plan
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents,
    )
    .unwrap()
}
