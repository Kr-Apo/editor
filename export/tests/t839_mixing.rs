//! Pose-only mixing audit. No rendering, source assets or mixing rule changes.
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    assets::AssetDb,
    pose::{Pose, evaluate},
    slot::Slot,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert, schema::Project};
use serde_json::json;

#[test]
fn ordered_draw_order_mixing_reference_is_explicit() {
    let project: Project =
        serde_json::from_value(json!({"version":1,"name":"T839 mixing","fps":60,
        "bones":[{"name":"root","length":1}]}))
        .unwrap();
    let mut loaded = convert::from_schema(&project);
    let root = loaded.skeleton.update_order[0];
    let a = loaded.skeleton.add_slot(Slot::new("a".into(), root));
    loaded.skeleton.add_slot(Slot::new("b".into(), root));
    let c = loaded.skeleton.add_slot(Slot::new("c".into(), root));
    for (name, slot, offset) in [("first", a, 2), ("second", c, -2)] {
        let mut clip = Animation::new(name, 1.0);
        clip.timelines.push(Timeline::DrawOrder {
            keys: vec![Key::linear(0.0, vec![(slot, offset)])],
        });
        loaded.animations.insert(clip);
    }
    let clips: Vec<_> = loaded.animations.values().collect();
    let mut pose = Pose::new();
    evaluate(
        &loaded.skeleton,
        &[(clips[0], 0.5, 0.4), (clips[1], 0.5, 0.6)],
        &mut pose,
    );
    let order: Vec<_> = pose
        .draw_order
        .iter()
        .map(|id| loaded.skeleton.slots[*id].name.as_str())
        .collect();
    assert_eq!(
        order,
        ["c", "b", "a"],
        "record current Rust semantics, not a future mixing contract"
    );
    if let Some(output) = std::env::var_os("T839_MIXING_DIR") {
        let output = std::path::PathBuf::from(output);
        std::fs::create_dir(&output).unwrap();
        let assets = AssetDb::new();
        let project = convert::to_schema(&ProjectRef {
            skeleton: &loaded.skeleton,
            animations: &loaded.animations,
            animation_order: &[],
            assets: &assets,
            name: &loaded.name,
            fps: loaded.fps,
            export_presets: &[],
            psd_layer_paths: &Default::default(),
        });
        let plan = run::plan(&project, &assets, &presets::default_preset()).unwrap();
        let native = &plan
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents;
        std::fs::write(output.join("mixing.native.json"), native).unwrap();
        std::fs::write(output.join("mixing.reference.json"), serde_json::to_vec_pretty(&json!({
            "layers":[{"animation":"first","time":0.5,"alpha":0.4},{"animation":"second","time":0.5,"alpha":0.6}],
            "rustDrawOrder":order})).unwrap()).unwrap();
    }
}
