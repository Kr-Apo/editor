//! Native FFD keys must obey the documented outgoing-curve context contract.
use ankhimate_core::assets::{AssetDb, ImageAsset};
use ankhimate_export::{
    context::{CONTEXT_VERSION, Context, ExportInfo},
    presets, run,
};
use ankhimate_formats::schema::Project;
use serde_json::{Value, json};

fn fixture(weighted: bool, keys: Value) -> Project {
    let offsets_per_key = if weighted { 16 } else { 8 };
    for key in keys.as_array().unwrap() {
        assert_eq!(key["offsets"].as_array().unwrap().len(), offsets_per_key);
    }
    let mut mesh = json!({
        "type":"mesh", "texture":"flat",
        "vertices":[-32,32,-32,-32,32,-32,32,32],
        "uvs":[0,0,0,1,1,1,1,0], "triangles":[0,1,2,0,2,3]
    });
    if weighted {
        mesh["weights"] = json!(vec![json!([["host", 0.25], ["other", 0.75]]); 4]);
    }
    serde_json::from_value(json!({
        "version":1, "name":"FFD regression", "fps":60,
        "bones":[{"name":"root","length":1},
            {"name":"host","parent":"root","length":16,"tx":8,"ty":4,"rotation":15},
            {"name":"other","parent":"root","length":16,"tx":20,"ty":-8,"rotation":-20}],
        "slots":[{"name":"art","bone":"host","attachment":"shape"}],
        "skins":[{"name":"default","entries":[{"slot":"art","name":"shape","attachment":mesh}]}],
        "default_skin":"default",
        "assets":[{"name":"flat","file":"flat.png","width":16,"height":16}],
        "animations":[{"name":"clip","duration":2,"looping":true,
            "timelines":[{"kind":"deform","slot":"art","attachment":"shape","keys":keys}]}]
    }))
    .unwrap()
}

fn exported(project: &Project) -> Value {
    let before = project.clone();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(16, 16, image::Rgba([64, 128, 192, 128]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("flat", bytes.into_inner(), 16, 16));
    let preset = presets::default_preset();
    let plan = run::plan(project, &assets, &preset).unwrap();
    assert_eq!(project, &before, "export must not edit the authoring keys");
    let text = &plan
        .files
        .iter()
        .find(|f| f.path == "skeleton.json")
        .unwrap()
        .contents;
    let data: Value = serde_json::from_str(text).unwrap();
    assert_eq!(data["format_version"], 2);
    assert_eq!(data["context_version"], CONTEXT_VERSION);
    assert_eq!(CONTEXT_VERSION, 1);
    assert_eq!(serde_json::to_value(&preset).unwrap()["preset_version"], 1);
    data
}

fn regression(curve: &str, weighted: bool) {
    // These are the offsets and handles used by the M005.4a real-export oracle.
    let end = if weighted {
        vec![
            12., 0., -4., 16., 0., 8., 4., -8., -12., 4., 16., 0., 8., -4., 0., 12.,
        ]
    } else {
        vec![-8., 12., 4., -8., 20., 4., 0., 16.]
    };
    let mut last = json!({"time":1,"offsets":end,"curve":curve});
    if curve == "bezier" {
        last["handles"] = json!([0.2, 0.8, 0.75, 0.1]);
    }
    let project = fixture(
        weighted,
        json!([
            {"time":0,"offsets":vec![0.;end.len()],"curve":"linear"}, last
        ]),
    );
    let data = exported(&project);
    let keys = &data["animations"]["clip"]["deform"][0]["keys"];
    assert_eq!(keys[0]["time"], 0.);
    assert_eq!(keys[1]["time"], 1.);
    assert_eq!(keys[0]["offsets"], json!(vec![0.; end.len()]));
    assert_eq!(keys[1]["offsets"], json!(end));
    if curve == "bezier" {
        assert_eq!(keys[0]["curve"]["type"], "bezier");
        let handles: [f32; 4] =
            serde_json::from_value(keys[0]["curve"]["handles"].clone()).unwrap();
        assert_eq!(handles, [0.2, 0.8, 0.75, 0.1]);
    } else {
        assert_eq!(keys[0]["curve"], curve);
    }
    assert_eq!(
        keys[1]["curve"], "linear",
        "no outgoing segment on the last key"
    );
    let mesh = &data["skins"][0]["attachments"][0]["attachment"];
    assert_eq!(mesh["vertex_count"], 4);
    assert_eq!(mesh["triangles"], json!([0, 1, 2, 0, 2, 3]));
    assert_eq!(
        mesh["weights"].as_array().unwrap().len(),
        if weighted { 4 } else { 0 }
    );
    if weighted {
        for weights in mesh["weights"].as_array().unwrap() {
            assert_eq!(weights["count"], 2);
            assert_eq!(weights["bones"][0]["bone"], "host");
            assert_eq!(weights["bones"][0]["weight"], 0.25);
            assert_eq!(weights["bones"][1]["bone"], "other");
            assert_eq!(weights["bones"][1]["weight"], 0.75);
        }
    }
}

#[test]
fn native_ffd_stepped_uses_the_destination_keys_curve() {
    regression("stepped", false);
}
#[test]
fn native_ffd_rigid_bezier_uses_the_destination_keys_curve() {
    regression("bezier", false);
}
#[test]
fn native_ffd_weighted_bezier_uses_the_destination_keys_curve() {
    regression("bezier", true);
}
#[test]
fn native_ffd_linear_preserves_rigid_and_per_influence_offsets() {
    regression("linear", false);
    regression("linear", true);
}

#[test]
fn sparse_ffd_keys_keep_times_values_and_the_first_and_last_boundaries() {
    let project = fixture(
        false,
        json!([
            {"time":0.25,"offsets":[1,2,3,4,5,6,7,8],"curve":"stepped"},
            {"time":0.5,"offsets":[2,4,6,8,10,12,14,16],"curve":"bezier","handles":[0.2,0.8,0.75,0.1]},
            {"time":1.5,"offsets":[4,8,12,16,20,24,28,32],"curve":"stepped"}
        ]),
    );
    let schema = serde_json::to_value(&project).unwrap();
    let authored = schema["animations"][0]["timelines"][0]["keys"]
        .as_array()
        .unwrap();
    let context = Context::build(&project, None, ExportInfo::default());
    let data = exported(&project);
    for keys in [
        &context.animations[0]["deform"][0]["keys"],
        &data["animations"]["clip"]["deform"][0]["keys"],
    ] {
        assert_eq!(keys.as_array().unwrap().len(), authored.len());
        for (i, key) in authored.iter().enumerate() {
            assert_eq!(keys[i]["time"], key["time"]);
            assert_eq!(keys[i]["offsets"], key["offsets"]);
        }
        assert_eq!(keys[0]["curve"]["type"], "bezier");
        assert_eq!(keys[1]["curve"], "stepped");
        assert_eq!(keys[2]["curve"], "linear");
    }
}

#[test]
fn empty_and_single_key_ffd_tracks_do_not_invent_frames() {
    for keys in [
        json!([]),
        json!([{"time":0.5,"offsets":[1,2,3,4,5,6,7,8],"curve":"stepped"}]),
    ] {
        let data = exported(&fixture(false, keys.clone()));
        let actual = &data["animations"]["clip"]["deform"][0]["keys"];
        assert_eq!(
            actual.as_array().unwrap().len(),
            keys.as_array().unwrap().len()
        );
        if !keys.as_array().unwrap().is_empty() {
            assert_eq!(actual[0]["time"], keys[0]["time"]);
            let values =
                |key: &Value| serde_json::from_value::<Vec<f32>>(key["offsets"].clone()).unwrap();
            assert_eq!(values(&actual[0]), values(&keys[0]));
            assert_eq!(actual[0]["curve"], "linear");
        }
    }
}
