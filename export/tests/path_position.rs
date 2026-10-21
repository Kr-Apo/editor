#[allow(dead_code, reason = "Shared synthetic native path")]
mod rig {
    include!("../../core/tests/support/path_position_fixture.rs");
}
use ankhimate_core::{assets::AssetDb, slotmap::SlotMap};
use ankhimate_export::{
    preflight::{self, Target},
    presets, run,
};
fn project() -> ankhimate_formats::schema::Project {
    let (s, a, _, _) = rig::fixture(false, true);
    let mut animations = SlotMap::with_key();
    animations.insert(a);
    ankhimate_formats::convert::to_schema(&ankhimate_formats::ProjectRef {
        skeleton: &s,
        animations: &animations,
        assets: &AssetDb::new(),
        name: "Path",
        fps: 30,
        export_presets: &[],
        psd_layer_paths: &Default::default(),
    })
}
#[test]
fn t831_native_generic_custom_and_plugin_preflight_refuse_path_animation() {
    let p = project();
    let assets = AssetDb::new();
    for target in [
        Target::Runtime,
        Target::LegacyRuntime,
        Target::Generic,
        Target::PhaserAtlas,
        Target::Custom,
    ] {
        let report = preflight::inspect(&p, &assets, target, None);
        assert!(report.has_errors());
        assert!(
            report
                .items
                .iter()
                .any(|d| d.code == "path-position-runtime-unsupported")
        );
    }
    for preset in presets::builtin() {
        let err = run::plan(&p, &assets, &preset).unwrap_err();
        assert!(err.to_string().contains("PathPosition"));
    }
}
#[test]
fn t831_rendered_preflight_accepts_valid_path_but_rejects_invalid_identity() {
    let mut p = project();
    let assets = AssetDb::new();
    let report = preflight::inspect(&p, &assets, Target::Rendered, None);
    assert!(!report.has_errors(), "{}", report.errors());
    assert!(report.items.iter().any(|d| d.code == "path-unqualified"));
    p.constraints[0].kind = "ik".into();
    let report = preflight::inspect(&p, &assets, Target::Rendered, None);
    assert!(
        report
            .items
            .iter()
            .any(|d| d.code == "invalid-path-position")
    );
}
#[test]
fn t831_static_native_export_remains_available_without_new_track() {
    let mut p = project();
    p.animations[0].timelines.clear();
    let preset = presets::default_preset();
    assert!(run::plan(&p, &AssetDb::new(), &preset).is_ok());
}
