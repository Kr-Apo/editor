use ankhimate_formats::{self as formats, Loaded, ProjectRef};
use serde_json::{Value, json};
use std::fs;

fn project(p: &Loaded) -> ProjectRef<'_> {
    ProjectRef {
        skeleton: &p.skeleton,
        animations: &p.animations,
        assets: &p.assets,
        name: &p.name,
        fps: p.fps,
        export_presets: &p.export_presets,
        psd_layer_paths: &p.psd_layer_paths,
    }
}
fn fixture() -> Value {
    json!({"version":1,"name":"reliability","fps":60,
        "bones":[{"name":"root","length":1}],
        "slots":[{"name":"body","bone":"root","attachment":"body"}],
        "skins":[{"name":"default","entries":[{"slot":"body","name":"body","attachment":{"type":"region","texture":"body","width":1,"height":1}}]}],
        "assets":[{"name":"body","file":"original/image.png","width":1,"height":1}],
        "groups":[{"name":"g"}],
        "animations":[{"name":"a","duration":1,"events":[{"time":0.5,"name":"hit"}],
            "timelines":[{"kind":"bone_rotate","bone":"root","keys":[{"time":0,"value":0,"curve":"linear"}]}]}]})
}

#[test]
fn t813_missing_image_save_refuses_without_changing_primary_or_save_as() {
    for suffix in [".ankh", ".ankh.json", ".ankh.min.json"] {
        let dir = tempfile::tempdir().unwrap();
        let primary = dir.path().join("source.ankh.json");
        let bytes = fixture().to_string().into_bytes();
        fs::write(&primary, &bytes).unwrap();
        let (mut control, _) = formats::load(&primary).unwrap();
        for asset in control.assets.images.values_mut() {
            asset.bytes = vec![1, 2, 3];
        }
        formats::save(
            &dir.path().join("write-control.ankh"),
            &project(&control),
            &[],
        )
        .expect("positive write control must succeed before testing refusal");
        let (loaded, _) = formats::load(&primary).unwrap();
        assert!(!loaded.report.dangling.is_empty());
        let dest = dir.path().join(format!("épreuve dossier{suffix}"));
        fs::write(&dest, b"previous valid output sentinel").unwrap();
        let result = formats::save(&dest, &project(&loaded), &[]);
        assert!(
            result.is_err(),
            "Missing image must block Save, not replace its URI with an empty hash"
        );
        assert_eq!(fs::read(&primary).unwrap(), bytes);
        assert_eq!(fs::read(&dest).unwrap(), b"previous valid output sentinel");
        assert!(!formats::container::asset_root(&dest).exists());
        assert!(formats::save(&primary, &project(&loaded), &[]).is_err());
        assert_eq!(fs::read(primary).unwrap(), bytes);
    }
}

#[test]
fn t813_unknown_members_are_refused_before_lossy_model_conversion() {
    for scope in [
        "",
        "/bones/0",
        "/slots/0",
        "/skins/0",
        "/skins/0/entries/0",
        "/skins/0/entries/0/attachment",
        "/assets/0",
        "/groups/0",
        "/animations/0",
        "/animations/0/events/0",
        "/animations/0/timelines/0",
        "/animations/0/timelines/0/keys/0",
    ] {
        let mut raw = fixture();
        raw.pointer_mut(scope)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(
                "future_member".into(),
                json!({"private_value_must_not_appear_in_errors":true}),
            );
        let before = raw.to_string();
        let result = formats::from_json(&before);
        assert!(
            result.is_err(),
            "Unknown member silently accepted at {scope}"
        );
        assert_eq!(raw.to_string(), before);
    }
}

#[test]
fn t813_unknown_envelope_flags_are_not_interpreted_as_current_format() {
    let payload = b"valid payload";
    for bit in [2, 4, 128] {
        let mut bytes = formats::container::encode(payload).unwrap();
        bytes[7] |= bit;
        assert!(
            formats::container::decode(&bytes).is_err(),
            "reserved flag {bit} must be rejected"
        );
    }
}

#[test]
fn t813_opaque_presets_and_recognized_explicit_defaults_still_load() {
    let mut raw = fixture();
    raw["export_presets"] = json!([{"future":{"version":99,"arbitrary":[1,"é",null]}}]);
    raw["groups"][0]["parent"] = json!("");
    raw["skins"][0]["bones"] = json!([]);
    raw["skins"][0]["constraints"] = json!([]);
    raw["skins"][0]["entries"][0]["attachment"]["sequence"] = Value::Null;
    raw["animations"][0]["events"][0]["volume"] = json!(1);
    raw["animations"][0]["events"][0]["int_value"] = json!(0);
    let loaded = formats::from_json(&raw.to_string()).unwrap();
    let saved: Value = serde_json::from_str(&formats::to_json(&project(&loaded)).unwrap()).unwrap();
    assert_eq!(saved["export_presets"], raw["export_presets"]);
}

#[test]
fn t813_nonfinite_serialization_must_not_replace_the_last_readable_project() {
    for suffix in [".ankh", ".ankh.json", ".ankh.min.json"] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(format!("finite{suffix}"));
        let mut raw = fixture();
        raw["assets"] = json!([]);
        let mut loaded = formats::from_json(&raw.to_string()).unwrap();
        formats::save(&path, &project(&loaded), &[]).unwrap();
        let before = fs::read(&path).unwrap();
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            loaded.skeleton.bones.values_mut().next().unwrap().length = invalid;
            assert!(
                formats::save(&path, &project(&loaded), &[]).is_err(),
                "Non-finite float serialized as null must be rejected before publication"
            );
            assert_eq!(fs::read(&path).unwrap(), before);
            assert!(formats::load(&path).is_ok());
        }
    }
}

#[test]
fn t813_unsupported_string_discriminants_are_refused_instead_of_defaulted() {
    for location in ["blend", "constraint", "sequence"] {
        let mut raw = fixture();
        match location {
            "blend" => raw["slots"][0]["blend_mode"] = json!("future_blend"),
            "constraint" => {
                raw["constraints"] =
                    json!([{"name":"c","type":"future_constraint","target":"root"}])
            }
            _ => {
                raw["skins"][0]["entries"][0]["attachment"]["sequence"] =
                    json!({"frames":["body"],"fps":12,"mode":"future_mode"})
            }
        }
        assert!(
            formats::from_json(&raw.to_string()).is_err(),
            "Unknown {location} value silently defaulted/dropped"
        );
    }
}

#[test]
fn t813_binary_and_compact_json_reject_unknown_members_without_writes() {
    let dir = tempfile::tempdir().unwrap();
    let loaded = formats::from_json(&fixture().to_string()).unwrap();
    let mut compact: Value =
        serde_json::from_str(&formats::to_min_json(&project(&loaded)).unwrap()).unwrap();
    compact["~future_member"] = json!({"secret_value": "do not print"});
    for (name, bytes) in [
        (
            "future.ankh.min.json",
            serde_json::to_vec(&compact).unwrap(),
        ),
        (
            "future.ankh",
            formats::container::encode(&rmp_serde::to_vec(&compact).unwrap()).unwrap(),
        ),
    ] {
        let path = dir.path().join(name);
        fs::write(&path, &bytes).unwrap();
        let err = formats::load(&path)
            .err()
            .expect("Unknown compact field must not be dropped");
        assert!(err.to_string().contains("future_member"));
        assert!(!err.to_string().contains("do not print"));
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
}

#[test]
fn t813_recognized_optional_defaults_and_all_attachment_kinds_are_accepted() {
    for attachment in [
        json!({"type":"region","texture":"body","width":1,"height":1,"sequence":null}),
        json!({"type":"mesh","texture":"body","edges":[],"linked":null,"sequence":null}),
        json!({"type":"mesh","texture":"body","linked":{"skin":null,"slot":"body","attachment":"parent"}}),
        json!({"type":"boundingbox","vertices":[],"weights":[]}),
        json!({"type":"clipping","vertices":[],"end_slot":null}),
        json!({"type":"path","vertices":[],"closed":false}),
        json!({"type":"point","x":0,"y":0,"rotation":0}),
    ] {
        let mut raw = fixture();
        raw["skins"][0]["entries"][0]["attachment"] = attachment.clone();
        raw["psd_layer_paths"] = json!({});
        raw["animations"][0]["events"][0] = json!({"time":0.5,"name":"hit","int_value":0,"float_value":0,"string_value":"","audio":"","volume":1,"balance":0});
        raw["constraints"] = json!([{"name":"c","type":"transform","target":"root","transform_mix":{"rotate":0,"translate_x":0,"translate_y":0,"scale_x":0,"scale_y":0,"shear_x":0,"shear_y":0},"offsets":null,"local":false,"relative":false,"physics":null,"forces":null,"channels":null,"slot":null,"path":null}]);
        raw["animations"][0]["timelines"] = json!([{"kind":"transform_constraint_mix","constraint":"c","keys":[{"time":0,"curve":"linear","rotate":0,"translate_x":0,"translate_y":0,"scale_x":0,"scale_y":0,"shear_x":0,"shear_y":0}]}]);
        assert!(
            formats::from_json(&raw.to_string()).is_ok(),
            "valid attachment/defaults refused: {attachment}"
        );
    }
}

#[test]
fn t813_moved_project_and_assets_remain_readable_in_every_profile() {
    let dir = tempfile::tempdir().unwrap();
    let before = dir.path().join("avant déplacement");
    fs::create_dir(&before).unwrap();
    let after = dir.path().join("après Unicode é");
    let mut loaded = formats::from_json(&fixture().to_string()).unwrap();
    let mut png = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(1, 1, image::Rgba([20, 40, 60, 128]))
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    let pixels = png.into_inner();
    loaded.assets.images.values_mut().next().unwrap().bytes = pixels.clone();
    for suffix in [".ankh", ".ankh.json", ".ankh.min.json"] {
        formats::save(
            &before.join(format!("personnage{suffix}")),
            &project(&loaded),
            &[],
        )
        .unwrap();
    }
    fs::rename(&before, &after).unwrap();
    for suffix in [".ankh", ".ankh.json", ".ankh.min.json"] {
        let path = after.join(format!("personnage{suffix}"));
        let (again, _) = formats::load(&path).unwrap();
        assert!(again.report.is_clean());
        assert_eq!(again.assets.images.values().next().unwrap().bytes, pixels);
        formats::save(&path, &project(&again), &[]).unwrap();
        assert!(formats::load(&path).unwrap().0.report.is_clean());
    }
}

#[test]
fn t813_default_omission_exceptions_are_scoped_and_unknown_nulls_are_not_lost() {
    for (scope, key, value) in [
        ("/bones/0", "events", json!([])),
        ("/animations/0/timelines/0/keys/0", "translate_x", json!(0)),
        ("/skins/0/entries/0/attachment", "weights", json!([])),
        ("/groups/0", "future", Value::Null),
    ] {
        let mut raw = fixture();
        raw.pointer_mut(scope)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(key.into(), value);
        assert!(
            formats::from_json(&raw.to_string()).is_err(),
            "Scoped exception accepted unknown {scope}/{key}"
        );
    }
}
