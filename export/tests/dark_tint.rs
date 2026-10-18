//! M005.3a: preserve authored static dark RGBA without claiming visual playback.
use ankhimate_core::assets::{AssetDb, ImageAsset};
use ankhimate_export::{
    preflight::{self, Severity, Target},
    preset::Preset,
    presets, run,
};
use ankhimate_formats::schema::Project;
use serde_json::{Value, json};

fn fixture() -> Project {
    serde_json::from_value(json!({
        "version": 1, "name": "rig", "fps": 60,
        "bones": [{"name": "root", "length": 1}],
        "slots": [{"name": "slot", "bone": "root"}]
    }))
    .unwrap()
}

fn legacy() -> Preset {
    Preset::from_json(include_str!("fixtures/runtime_v2_before_dark_tint.json")).unwrap()
}

fn exported(project: &Project, preset: &Preset) -> (String, Value) {
    let plan = run::plan(project, &AssetDb::new(), preset).unwrap();
    let text = plan
        .files
        .iter()
        .find(|f| f.path == "skeleton.json")
        .unwrap()
        .contents
        .clone();
    let value = serde_json::from_str(&text).unwrap();
    (text, value)
}

#[test]
fn static_dark_rgba_survives_native_json_exactly_including_zero_alpha() {
    let mut project = fixture();
    for dark in [
        [0.12345679, 0.2345679, 0.9876543, 0.375],
        [0.0; 4],
        [1.0; 4],
        [1.0, 0.0, 0.5, 0.0],
        [0.0, 1.0, 0.0, 0.5],
        [-0.25, 2.0, 0.5, 1.0], // finite authored values are not clamped
    ] {
        project.slots[0].dark_color = Some(dark);
        let before = project.clone();
        let authoring_json = serde_json::to_string(&project).unwrap();
        let roundtrip: Project = serde_json::from_str(&authoring_json).unwrap();
        assert_eq!(roundtrip, project);
        let loaded = ankhimate_formats::convert::from_schema(&roundtrip);
        assert_eq!(
            loaded.skeleton.slots.values().next().unwrap().dark_color,
            Some(dark)
        );
        let (_, data) = exported(&project, &presets::default_preset());
        let actual: [f32; 4] =
            serde_json::from_value(data["slots"][0]["dark_color"].clone()).unwrap();
        assert_eq!(actual, dark);
        assert_eq!(data["format_version"], 2);
        assert_eq!(data["context_version"], 1);
        assert_eq!(project, before);
        assert_eq!(data["animations"], json!({})); // no invented dark animation
    }
}

#[test]
fn no_dark_tint_is_byte_identical_to_the_accepted_native_preset() {
    let project = fixture();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([200, 100, 50, 128]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("art", bytes.into_inner(), 2, 2));
    let old = run::plan(&project, &assets, &legacy()).unwrap();
    let current = run::plan(&project, &assets, &presets::default_preset()).unwrap();
    assert_eq!(old.files, current.files);
    assert!(!current.binaries.is_empty());
    assert_eq!(old.binaries, current.binaries);
    let (_, value) = exported(&project, &presets::default_preset());
    assert!(value["slots"][0].get("dark_color").is_none());
}

#[test]
fn tint_extension_preserves_json_escaping_and_existing_light_blend_fields() {
    let mut project = fixture();
    let name = "slot\"\\\n\t\u{0001}é日本🦴<&>";
    project.slots[0].name = name.into();
    project.slots[0].color = [0.5, 0.25, 1.0, 0.5];
    project.slots[0].blend_mode = "additive".into();
    let (_, before) = exported(&project, &legacy());
    project.slots[0].dark_color = Some([0.2, 0.3, 0.4, 0.5]);
    let (text, mut after) = exported(&project, &presets::default_preset());
    assert_eq!(after["slots"][0]["name"], name);
    assert_eq!(
        after["slots"][0]["dark_color"],
        json!(project.slots[0].dark_color.unwrap())
    );
    after["slots"][0]
        .as_object_mut()
        .unwrap()
        .remove("dark_color");
    assert_eq!(before, after);
    assert_eq!(text, exported(&project, &presets::default_preset()).0);
}

#[test]
fn old_saved_native_presets_keep_their_bytes_and_report_the_known_omission() {
    let mut project = fixture();
    project.slots[0].dark_color = Some([0.2, 0.3, 0.4, 1.0]);
    let mut preset = legacy();
    preset.name = "my saved native preset".into();
    let before = preset.clone();
    let target = Target::for_preset(&preset);
    assert_eq!(format!("{target:?}"), "LegacyRuntime");
    let report = preflight::inspect(&project, &AssetDb::new(), target, None);
    assert!(
        report
            .items
            .iter()
            .any(|d| d.code == "dark-tint-loss" && d.severity == Severity::Warning)
    );
    assert!(!report.allowed(false));
    assert!(report.allowed(true));
    let (_, value) = exported(&project, &preset);
    assert!(value["slots"][0].get("dark_color").is_none());
    assert_eq!(preset, before);
    preset.templates[0].body.push(' ');
    assert_eq!(Target::for_preset(&preset), Target::Custom);
    project.slots[0].dark_color = None;
    assert!(!preflight::inspect(&project, &AssetDb::new(), target, None).has_warnings());
}

#[test]
fn transmitted_dark_tint_still_requires_consent_for_unknown_consumers() {
    let mut project = fixture();
    project.slots[0].dark_color = Some([0.0; 4]);
    let preset = presets::default_preset();
    let target = Target::for_preset(&preset);
    assert_eq!(target, Target::Runtime);
    let report = preflight::inspect(&project, &AssetDb::new(), target, None);
    let warning = report
        .items
        .iter()
        .find(|d| d.code == "dark-tint-consumer-unknown")
        .unwrap();
    assert_eq!(warning.severity, Severity::Warning);
    assert!(warning.message.contains("Phaser"));
    assert!(warning.message.contains("serialized"));
    assert!(!report.items.iter().any(|d| d.code == "dark-tint-loss"));
    assert!(!report.allowed(false));
    assert!(report.allowed(true));
    // Cancelled preflight is read-only: no destination is even created.
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("cancelled");
    let plan = run::plan(&project, &AssetDb::new(), &preset).unwrap();
    run::inspect(&plan, &destination).unwrap();
    assert!(!destination.exists());
}

#[test]
fn nonfinite_dark_channels_are_blocked_before_context_can_turn_them_into_null() {
    for channel in 0..4 {
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut project = fixture();
            let mut dark = [0.5; 4];
            dark[channel] = invalid;
            project.slots[0].dark_color = Some(dark);
            let report = preflight::inspect(&project, &AssetDb::new(), Target::Runtime, None);
            assert!(!report.allowed(true));
            assert!(
                report
                    .items
                    .iter()
                    .any(|d| d.code == "invalid-number" && d.severity == Severity::Error)
            );
            for images in [run::Images::Named, run::Images::Encoded] {
                let result = run::plan_with(
                    &project,
                    &AssetDb::new(),
                    &presets::default_preset(),
                    images,
                );
                assert!(result.is_err(), "native planning must refuse {dark:?}");
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("non-finite dark tint")
                );
            }
            assert_eq!(
                project.slots[0].dark_color.unwrap().map(f32::to_bits),
                dark.map(f32::to_bits)
            );
        }
    }
}

#[test]
fn authoring_json_requires_four_numeric_dark_channels_and_keeps_old_missing_values() {
    let mut value = serde_json::to_value(fixture()).unwrap();
    for invalid in [
        json!([1, 2, 3]),
        json!([1, 2, 3, 4, 5]),
        json!([1, null, 0, 1]),
        json!("ff0000ff"),
    ] {
        value["slots"][0]["dark_color"] = invalid;
        assert!(serde_json::from_value::<Project>(value.clone()).is_err());
    }
    value["slots"][0]
        .as_object_mut()
        .unwrap()
        .remove("dark_color");
    assert_eq!(
        serde_json::from_value::<Project>(value).unwrap().slots[0].dark_color,
        None
    );
}
