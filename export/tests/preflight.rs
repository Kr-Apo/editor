use ankhimate_core::assets::{AssetDb, ImageAsset};
use ankhimate_export::preflight::{self, Severity, Target};
use ankhimate_formats::schema::Project;
use serde_json::json;

fn fixture() -> (Project, AssetDb) {
    let project = serde_json::from_value(json!({"version":1,"name":"rig","fps":60,
        "assets":[{"name":"art","file":"art.png","width":2,"height":2}],
        "bones":[{"name":"root","length":1}],"slots":[{"name":"slot","bone":"root","attachment":"art"}],
        "skins":[{"name":"default","entries":[{"slot":"slot","name":"art","attachment":{"type":"region","texture":"art","width":2,"height":2}}]}],"default_skin":"default",
        "animations":[{"name":"move","duration":1,"looping":true,"timelines":[{"kind":"bone_translate","bone":"root","axis":"x","keys":[{"time":0,"value":0,"curve":"linear"},{"time":1,"value":10,"curve":"linear"}]}]}]
    })).unwrap();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([200, 100, 50, 128]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("art", bytes.into_inner(), 2, 2));
    (project, assets)
}
fn has(report: &preflight::Report, code: &str, severity: Severity) -> bool {
    report
        .items
        .iter()
        .any(|item| item.code == code && item.severity == severity)
}

#[test]
fn valid_runtime_has_no_unjustified_warning_and_preserves_its_input() {
    let (project, assets) = fixture();
    let before = project.clone();
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    assert!(report.allowed(false));
    assert!(!report.has_warnings());
    assert_eq!(project, before);
}

#[test]
fn alpha_diagnostics_describe_qualified_dark_tint_without_trusting_package_labels() {
    let (mut project, assets) = fixture();
    project.slots[0].dark_color = Some([0.2, 0.3, 0.4, 1.0]);
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    let warning = report
        .items
        .iter()
        .find(|d| d.code == "dark-tint-consumer-unknown")
        .expect("consumer must remain unknown until explicitly qualified");
    assert_eq!(warning.severity, Severity::Warning);
    assert!(warning.message.contains("m0061-8888e0e"));
    assert!(warning.message.contains("Normal"));
    assert!(warning.message.contains("unknown"));
    assert!(!warning.message.contains("Phaser do not apply"));
    assert!(!report.allowed(false));
    assert!(report.allowed(true));
    let legacy = preflight::inspect(&project, &assets, Target::LegacyRuntime, None);
    let loss = legacy
        .items
        .iter()
        .find(|d| d.code == "dark-tint-loss")
        .unwrap();
    assert!(loss.message.contains("omits"));
    assert!(!loss.message.contains("Phaser still does not apply"));
    assert!(!legacy.allowed(false));
}

#[test]
fn native_physics_diagnostic_identifies_real_incompatibility_and_keeps_rendered_refusal() {
    let (mut project, assets) = fixture();
    project.constraints.push(
        serde_json::from_value(
            json!({"name":"spring","type":"physics","target":"root","bones":[]}),
        )
        .unwrap(),
    );
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    let warning = report
        .items
        .iter()
        .find(|d| d.code == "runtime-physics-incompatible")
        .expect("native physics is incompatible, not merely unqualified");
    assert_eq!(warning.severity, Severity::Warning);
    assert!(warning.message.contains("does not activate"));
    assert!(warning.message.contains("target"));
    assert!(warning.message.contains("bones"));
    assert!(!report.allowed(false));
    assert!(report.allowed(true));
    assert!(has(
        &preflight::inspect(&project, &assets, Target::Rendered, Some("move")),
        "rendered-physics",
        Severity::Error
    ));
}

#[test]
fn linked_mesh_chains_disclose_known_one_level_renderer_limit() {
    let (mut project, assets) = fixture();
    project.skins[0].entries[0].attachment=serde_json::from_value(json!({"type":"mesh","texture":"art","vertices":[0,0,1,0,0,1],"uvs":[0,0,1,0,0,1],"triangles":[0,1,2]})).unwrap();
    for (name, source) in [("middle", "art"), ("chain", "middle")] {
        project.skins[0].entries.push(serde_json::from_value(json!({"slot":"slot","name":name,"attachment":{"type":"mesh","texture":"art","linked":{"slot":"slot","attachment":source,"inherit_deform":false}}})).unwrap());
    }
    for target in [Target::Runtime, Target::Rendered] {
        let report = preflight::inspect(&project, &assets, target, Some("move"));
        assert!(has(&report, "linked-chain-incompatible", Severity::Warning));
        assert!(!report.allowed(false));
    }
    project.default_skin.clear();
    assert!(has(
        &preflight::inspect(&project, &assets, Target::Runtime, None),
        "linked-chain-incompatible",
        Severity::Warning
    ));
}
#[test]
fn absent_images_and_unknown_attachments_are_blocking() {
    let (mut project, assets) = fixture();
    project.slots[0].attachment = Some("ghost".into());
    let report = preflight::inspect(&project, &AssetDb::new(), Target::Runtime, None);
    assert!(has(&report, "missing-image", Severity::Error));
    assert!(has(&report, "unknown-attachment", Severity::Error));
    assert!(!report.allowed(true));
    assert!(!preflight::inspect(&project, &assets, Target::Rendered, Some("move")).allowed(true));
}
#[test]
fn empty_clips_are_static_information_and_invalid_clips_are_blocking() {
    let (mut project, assets) = fixture();
    project.animations[0].timelines.clear();
    let report = preflight::inspect(&project, &assets, Target::Rendered, Some("move"));
    assert!(has(&report, "static-clip", Severity::Info));
    assert!(report.allowed(false));
    project.animations[0].duration = f32::NAN;
    assert!(!preflight::inspect(&project, &assets, Target::Runtime, None).allowed(true));
    assert!(!preflight::inspect(&project, &assets, Target::Rendered, Some("absent")).allowed(true));
}
#[test]
fn malformed_deformation_mesh_indices_and_curve_numbers_are_refused() {
    let (mut project, assets) = fixture();
    project.skins[0].entries[0].attachment=serde_json::from_value(json!({"type":"mesh","texture":"art","vertices":[0,0,1,0,0,1],"uvs":[0,0,1,0,0,1],"triangles":[0,1,8]})).unwrap();
    project.animations[0].timelines.push(serde_json::from_value(json!({"kind":"deform","slot":"slot","attachment":"art","keys":[{"time":0,"offsets":[1,2],"curve":"linear"}]})).unwrap());
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    assert!(has(&report, "invalid-mesh", Severity::Error));
    assert!(has(&report, "invalid-deform", Severity::Error));
    project.animations[0].timelines[0]=serde_json::from_value(json!({"kind":"bone_translate","bone":"root","keys":[{"time":0,"value":0,"curve":"bezier","handles":[2,0,0.5,1]}]})).unwrap();
    assert!(has(
        &preflight::inspect(&project, &assets, Target::Runtime, None),
        "invalid-curve",
        Severity::Error
    ));
}
#[test]
fn transmitted_dark_tint_requires_explicit_consent_for_unknown_consumers() {
    let (mut project, assets) = fixture();
    project.slots[0].dark_color = Some([0.2, 0.3, 0.4, 1.0]);
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    assert!(has(
        &report,
        "dark-tint-consumer-unknown",
        Severity::Warning
    ));
    assert!(!report.allowed(false));
    assert!(report.allowed(true));
    assert!(preflight::inspect(&project, &assets, Target::Rendered, Some("move")).has_warnings());
    let plan = ankhimate_export::run::plan(
        &project,
        &assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let data: serde_json::Value = serde_json::from_str(&plan.files[0].contents).unwrap();
    assert_eq!(data["format_version"], 2);
    assert_eq!(
        data["slots"][0]["dark_color"],
        json!(project.slots[0].dark_color.unwrap())
    );
    assert!(data["slots"][0].get("darkColor").is_none());
}
#[test]
fn rendered_physics_and_blends_stay_blocked_and_nondefault_skins_are_disclosed() {
    let (mut project, assets) = fixture();
    project.slots[0].blend_mode = "multiply".into();
    project.constraints.push(
        serde_json::from_value(
            json!({"name":"spring","type":"physics","target":"root","bones":["root"]}),
        )
        .unwrap(),
    );
    project.skins.push(project.skins[0].clone());
    project.skins[1].name = "alternate".into();
    let report = preflight::inspect(&project, &assets, Target::Rendered, Some("move"));
    assert!(has(&report, "rendered-physics", Severity::Error));
    assert!(has(&report, "rendered-blend", Severity::Error));
    assert!(has(&report, "default-skin-only", Severity::Warning));
    assert!(!report.allowed(true));
}
#[test]
fn modified_templates_are_not_certified_by_their_name_and_partial_formats_warn() {
    let (project, assets) = fixture();
    let mut preset = ankhimate_export::presets::default_preset();
    assert_eq!(Target::for_preset(&preset), Target::Runtime);
    preset.templates[0].body = "{}".into();
    assert_eq!(Target::for_preset(&preset), Target::Custom);
    assert!(preflight::inspect(&project, &assets, Target::Generic, None).has_warnings());
    assert!(preflight::inspect(&project, &assets, Target::PhaserAtlas, None).has_warnings());
}

#[test]
fn inherited_source_deforms_are_refused_for_rendering_instead_of_silently_lost() {
    let (mut project, assets) = fixture();
    project.skins[0].entries[0].attachment=serde_json::from_value(json!({"type":"mesh","texture":"art","vertices":[0,0,1,0,0,1],"uvs":[0,0,1,0,0,1],"triangles":[0,1,2]})).unwrap();
    project.skins[0].entries.push(serde_json::from_value(json!({"slot":"slot","name":"linked","attachment":{"type":"mesh","texture":"art","linked":{"slot":"slot","attachment":"art","inherit_deform":true}}})).unwrap());
    project.slots[0].attachment = Some("linked".into());
    project.animations[0].timelines.push(serde_json::from_value(json!({"kind":"deform","slot":"slot","attachment":"art","keys":[{"time":0,"offsets":[1,0,0,0,0,0],"curve":"linear"}]})).unwrap());
    let report = preflight::inspect(&project, &assets, Target::Rendered, Some("move"));
    assert!(has(&report, "rendered-linked-deform", Severity::Error));
    assert!(!preflight::inspect(&project, &assets, Target::Runtime, None).has_errors());
}

#[test]
fn constraint_and_animated_draw_order_references_are_validated() {
    let (mut project, assets) = fixture();
    project.constraints.push(
        serde_json::from_value(json!({"name":"ik","type":"ik","target":"ghost","bones":["root"]}))
            .unwrap(),
    );
    project.animations[0].timelines.push(
        serde_json::from_value(
            json!({"kind":"draw_order","keys":[{"time":0,"offsets":[["missing",1]]}]}),
        )
        .unwrap(),
    );
    let report = preflight::inspect(&project, &assets, Target::Runtime, None);
    assert!(has(&report, "missing-bone", Severity::Error));
    assert!(has(&report, "missing-slot", Severity::Error));
}
