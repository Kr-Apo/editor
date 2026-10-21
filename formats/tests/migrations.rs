//! Authentic historical compatibility, not fabricated v0 migration.
use ankhimate_formats::{self as formats, Loaded, ProjectRef, schema};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn manifest() -> Value {
    serde_json::from_str(include_str!("fixtures/format-versions.json")).unwrap()
}
fn project(p: &Loaded) -> ProjectRef<'_> {
    ProjectRef {
        animation_order: &[],
        skeleton: &p.skeleton,
        animations: &p.animations,
        assets: &p.assets,
        name: &p.name,
        fps: p.fps,
        export_presets: &p.export_presets,
        psd_layer_paths: &p.psd_layer_paths,
    }
}
fn snapshot(p: &Loaded) -> Value {
    serde_json::from_str(&formats::to_json(&project(p)).unwrap()).unwrap()
}
fn compare(a: &Value, b: &Value, at: &str) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            assert!((x - y).abs() <= 0.0001, "{at}: {x} != {y}");
        }
        (Value::Array(x), Value::Array(y)) => {
            assert_eq!(x.len(), y.len(), "{at}");
            for (i, (a, b)) in x.iter().zip(y).enumerate() {
                compare(a, b, &format!("{at}[{i}]"));
            }
        }
        (Value::Object(x), Value::Object(y)) => {
            assert_eq!(x.len(), y.len(), "{at}");
            for (k, v) in x {
                compare(
                    v,
                    y.get(k).unwrap_or_else(|| panic!("missing {at}.{k}")),
                    &format!("{at}.{k}"),
                );
            }
        }
        _ => assert_eq!(a, b, "{at}"),
    }
}
fn tree(path: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    if !path.exists() {
        return vec![];
    }
    let mut out = vec![];
    for e in fs::read_dir(path).unwrap() {
        let e = e.unwrap();
        if e.path().is_dir() {
            out.extend(tree(&e.path()));
        } else {
            out.push((e.path(), fs::read(e.path()).unwrap()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

#[test]
fn authentic_v1_fixtures_save_reopen_all_profiles_without_touching_original_assets() {
    for f in manifest()["fixtures"].as_array().unwrap() {
        let source = root().join(f["path"].as_str().unwrap());
        let bytes = fs::read(&source).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            f["sha256"].as_str().unwrap()
        );
        let original_assets = tree(&formats::container::asset_root(&source));
        let (loaded, _) = formats::load(&source).unwrap();
        assert!(
            loaded.report.dangling.is_empty(),
            "{}: {:?}",
            source.display(),
            loaded.report
        );
        let before = snapshot(&loaded);
        for suffix in [".ankh", ".ankh.json", ".ankh.min.json"] {
            let dir = tempfile::tempdir().unwrap();
            let saved = dir.path().join(format!("roundtrip{suffix}"));
            let backup = dir.path().join("user-backup.ankh");
            fs::write(&backup, &bytes).unwrap();
            formats::save(&saved, &project(&loaded), &[]).unwrap();
            let (again, _) = formats::load(&saved).unwrap();
            assert!(
                again.report.dangling.is_empty(),
                "reopened {:?}",
                again.report
            );
            compare(&before, &snapshot(&again), f["id"].as_str().unwrap());
            for (_, asset) in loaded.assets.images.iter() {
                let id = again.assets.by_name(&asset.name).unwrap();
                assert_eq!(
                    asset.bytes, again.assets.images[id].bytes,
                    "asset {}",
                    asset.name
                );
            }
            assert_eq!(fs::read(backup).unwrap(), bytes, "existing backup modified");
        }
        assert_eq!(
            fs::read(&source).unwrap(),
            bytes,
            "historical source modified"
        );
        assert_eq!(
            tree(&formats::container::asset_root(&source)),
            original_assets,
            "original assets modified"
        );
    }
}

#[test]
fn authentic_zip_prototype_is_rejected_not_mislabeled_as_v0_or_current_v1() {
    let data = manifest();
    let fixture = &data["unsupported_prototype"];
    let path = root().join(fixture["path"].as_str().unwrap());
    let bytes = fs::read(&path).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        fixture["sha256"].as_str().unwrap()
    );
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    let value: Value = serde_json::from_reader(zip.by_name("project.json").unwrap()).unwrap();
    assert_eq!(value["version"], 1);
    assert!(data["v0_fixture"].is_null());
    assert!(formats::load(&path).is_err());
    assert_eq!(fs::read(path).unwrap(), bytes);
}

#[test]
fn unknown_author_versions_and_malformed_payloads_preserve_inputs_and_backups() {
    let dir = tempfile::tempdir().unwrap();
    let backup = dir.path().join("existing.backup");
    fs::write(&backup, b"valid backup").unwrap();
    for version in [0, 2, u32::MAX] {
        let path = dir.path().join(format!("unknown-{version}.ankh.json"));
        let bytes =
            serde_json::to_vec(&json!({"version":version,"name":"future","fps":60})).unwrap();
        fs::write(&path, &bytes).unwrap();
        assert!(
            matches!(formats::load(&path),Err(formats::Error::Version{found}) if found==version)
        );
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    let original = fs::read(root().join("formats/tests/fixtures/minimal.ankh")).unwrap();
    let mut corrupt = original.clone();
    corrupt[12] ^= 1;
    let mut future = original.clone();
    future[4] = 2;
    for (name, bytes) in [
        ("crc", corrupt),
        ("future-envelope", future),
        ("truncated", original[..10].to_vec()),
        ("json", b"{bad json".to_vec()),
    ] {
        let path = dir.path().join(format!("{name}.ankh"));
        fs::write(&path, &bytes).unwrap();
        assert!(formats::load(&path).is_err(), "{name}");
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    assert_eq!(fs::read(backup).unwrap(), b"valid backup");
}

#[test]
fn unknown_fields_survive_schema_only_and_editable_load_refuses_loss_except_opaque_presets() {
    let raw = json!({"version":1,"name":"unknown fields","fps":60,"future_root":{"a":7},
        "bones":[{"name":"root","length":1,"future_bone":[1,2]}],
        "export_presets":[{"future_preset":{"nested":["é",{"future":true}]}}]});
    let typed: schema::Project = serde_json::from_value(raw.clone()).unwrap();
    let direct = serde_json::to_value(&typed).unwrap();
    assert_eq!(direct["future_root"], raw["future_root"]);
    assert_eq!(
        direct["bones"][0]["future_bone"],
        raw["bones"][0]["future_bone"]
    );
    assert!(matches!(
        formats::from_json(&raw.to_string()),
        Err(formats::Error::UnsupportedMember { .. })
    ));
    let opaque =
        json!({"version":1,"name":"opaque","fps":60,"export_presets":raw["export_presets"]});
    let loaded = formats::from_json(&opaque.to_string()).unwrap();
    let compact = formats::to_min_json(&project(&loaded)).unwrap();
    assert_eq!(
        snapshot(&formats::from_json(&compact).unwrap())["export_presets"],
        raw["export_presets"]
    );
    let group: schema::Group = serde_json::from_value(json!({"name":"g","future":42})).unwrap();
    assert!(serde_json::to_value(group).unwrap().get("future").is_none());
}

#[test]
fn missing_assets_report_dangling_and_traversal_is_rejected_without_writes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.ankh.json");
    let mut raw = json!({"version":1,"name":"missing","fps":60,"assets":[{"name":"image","file":"aa/missing.png","width":1,"height":1}]});
    fs::write(&path, raw.to_string()).unwrap();
    let (loaded, _) = formats::load(&path).unwrap();
    assert!(
        loaded
            .report
            .dangling
            .iter()
            .any(|(kind, name)| *kind == "asset image" && name == "image")
    );
    for bad in [
        "../outside.png",
        "C:/outside.png",
        "/outside.png",
        "..\\outside.png",
    ] {
        raw["assets"][0]["file"] = json!(bad);
        fs::write(&path, raw.to_string()).unwrap();
        let before = tree(dir.path());
        assert!(formats::load(&path).is_err(), "{bad}");
        assert_eq!(tree(dir.path()), before);
    }
}

#[test]
fn failed_save_keeps_previous_project_assets_and_user_backup() {
    let source = root().join("demos/m0070-crapaud-pharaon/projet/Crapaud-Pharaon.ankh");
    let (loaded, _) = formats::load(&source).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join("rig.ankh");
    formats::save(&dest, &project(&loaded), &[]).unwrap();
    let backup = dir.path().join("rig.user-backup");
    fs::copy(&dest, &backup).unwrap();
    let before = tree(dir.path());
    // Writer lock is exclusive and belongs to another operation: never replace it.
    let lock = dir.path().join(".rig.ankh.ankh-write-lock");
    fs::write(&lock, b"another writer").unwrap();
    assert!(formats::save(&dest, &project(&loaded), &[]).is_err());
    assert_eq!(fs::read(&lock).unwrap(), b"another writer");
    for (path, bytes) in before {
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    let (again, _) = formats::load(&dest).unwrap();
    compare(&snapshot(&loaded), &snapshot(&again), "failed save");
}

#[test]
fn dangling_references_are_reported_and_legacy_paired_keys_are_not_migrated() {
    let p = formats::from_json(
        &json!({"version":1,"name":"dangling","fps":60,"slots":[{"name":"x","bone":"missing"}]})
            .to_string(),
    )
    .unwrap();
    assert!(!p.report.dangling.is_empty());
    let legacy = json!({"version":1,"name":"paired","fps":60,"bones":[{"name":"root","length":1}],"animations":[{"name":"a","duration":1,"timelines":[{"kind":"bone_translate","bone":"root","keys":[{"time":0,"x":1,"y":2,"curve":"linear"}]}]}]});
    assert!(formats::from_json(&legacy.to_string()).is_err());
}
