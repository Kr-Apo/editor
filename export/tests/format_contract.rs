use ankhimate_export::{
    context::{CONTEXT_VERSION, Context, ExportInfo},
    presets,
    template::Engine,
};
use ankhimate_formats::{ProjectRef, schema};
use serde_json::Value;

fn render(project: &schema::Project) -> Value {
    let context = Context::build(project, None, ExportInfo::default());
    serde_json::from_str(
        &Engine::new()
            .render(
                "T-605 native contract",
                &presets::default_preset().templates[0].body,
                &context.to_value(),
            )
            .unwrap(),
    )
    .unwrap()
}
fn fields(value: &Value, expected: &[&str], label: &str) {
    let mut actual: Vec<_> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    actual.sort_unstable();
    let mut wanted = expected.to_vec();
    wanted.sort_unstable();
    assert_eq!(actual, wanted, "{label} native fields changed; review spec");
    let spec = include_str!("../../docs/format-spec.md");
    let row = spec
        .split("## Native runtime JSON")
        .nth(1)
        .unwrap()
        .lines()
        .find(|l| l.starts_with(&format!("| {label} |")))
        .unwrap_or_else(|| panic!("missing normative table for {label}"));
    for key in expected {
        assert!(row.contains(key), "missing documented native {label}.{key}");
    }
}
#[test]
fn real_native_axes_export_matches_versioned_root_and_animation_tables() {
    let project: schema::Project =
        serde_json::from_str(include_str!("fixtures/runtime_axes.ankh.json")).unwrap();
    let native = render(&project);
    assert_eq!(native["format"], "ankhimate-runtime");
    assert_eq!(native["format_version"], 2);
    assert_eq!(native["context_version"], CONTEXT_VERSION);
    assert_eq!(CONTEXT_VERSION, 1);
    assert_eq!(ankhimate_export::preset::PRESET_VERSION, 1);
    fields(
        &native,
        &[
            "format",
            "format_version",
            "context_version",
            "name",
            "fps",
            "bones",
            "slots",
            "drawOrder",
            "defaultSkin",
            "skins",
            "constraints",
            "constraintOrder",
            "animations",
            "atlas",
        ],
        "Root",
    );
    fields(
        &native["bones"][0],
        &[
            "name",
            "parent",
            "length",
            "x",
            "y",
            "rotation",
            "scaleX",
            "scaleY",
            "shearX",
            "shearY",
            "inheritRotation",
            "inheritScale",
            "inheritReflect",
        ],
        "Bone",
    );
    for animation in native["animations"].as_object().unwrap().values() {
        fields(
            animation,
            &[
                "duration",
                "looping",
                "bones",
                "slots",
                "deform",
                "drawOrder",
                "ik",
                "transform",
                "events",
            ],
            "Animation",
        );
        for track in animation["bones"].as_array().unwrap() {
            for (key, keys) in track.as_object().unwrap() {
                if key == "name" {
                    continue;
                }
                assert!(
                    [
                        "rotate",
                        "translate_x",
                        "translate_y",
                        "scale_x",
                        "scale_y",
                        "shear_x",
                        "shear_y"
                    ]
                    .contains(&key.as_str()),
                    "unexpected v2 channel {key}"
                );
                assert!(!keys.as_array().unwrap().is_empty());
                for k in keys.as_array().unwrap() {
                    fields(
                        k,
                        &[
                            "time",
                            "value",
                            "curve",
                            "has_next",
                            "points",
                            "line_points",
                            "is_bezier",
                        ],
                        "ScalarKey",
                    );
                }
            }
        }
    }
    assert!(native["atlas"].is_null());
}

#[test]
fn historical_crapaud_native_attachment_fields_match_documented_projection() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../demos/m0070-crapaud-pharaon/projet/Crapaud-Pharaon.ankh");
    let (p, _) = ankhimate_formats::load(&path).unwrap();
    let view = ProjectRef {
        skeleton: &p.skeleton,
        animations: &p.animations,
        assets: &p.assets,
        name: &p.name,
        fps: p.fps,
        export_presets: &p.export_presets,
        psd_layer_paths: &p.psd_layer_paths,
    };
    let native = render(&ankhimate_formats::convert::to_schema(&view));
    let mut count = 0;
    for skin in native["skins"].as_array().unwrap() {
        fields(
            skin,
            &["name", "bones", "constraints", "attachments"],
            "Skin",
        );
        for entry in skin["attachments"].as_array().unwrap() {
            let a = &entry["attachment"];
            if a["type"] == "region" {
                let expected = [
                    "type",
                    "texture",
                    "x",
                    "y",
                    "rotation",
                    "scale_x",
                    "scale_y",
                    "width",
                    "height",
                    "source_width",
                    "source_height",
                    "uv",
                    "pivot_x",
                    "pivot_y",
                    "sequence",
                ];
                let mut got: Vec<_> = a.as_object().unwrap().keys().map(String::as_str).collect();
                got.sort_unstable();
                let mut want = expected.to_vec();
                want.sort_unstable();
                assert_eq!(got, want);
                let spec = include_str!("../../docs/format-spec.md");
                let row = spec.lines().find(|l| l.starts_with("| region |")).unwrap();
                for key in expected.into_iter().filter(|k| *k != "type") {
                    assert!(row.contains(key), "undocumented region.{key}");
                }
                count += 1;
            }
        }
    }
    assert!(count > 0);
}
