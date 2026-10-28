//! T-840C9-A2: the expected contract, deliberately red against E1.
#[path = "support/native_fidelity.rs"]
mod support;
use ankhimate_export::{
    context::{Context, ExportInfo},
    preflight::Target,
    preset::Preset,
    presets, run,
};
use ankhimate_formats::schema;
use serde_json::{Value, json};

fn outgoing(keys: &[Value], curves: Vec<Value>) {
    assert_eq!(keys.len(), curves.len());
    for (k, expected) in keys.iter().zip(curves) {
        assert_eq!(k["curve"], expected, "outgoing segment at {}", k["time"]);
    }
}
fn expected(interp: schema::Interp) -> Value {
    match interp {
        schema::Interp::Linear => json!("linear"),
        schema::Interp::Stepped => json!("stepped"),
        schema::Interp::Bezier { handles } => {
            json!({"type":"bezier","handles":handles,"out_x":handles[0],"out_y":handles[1],"in_x":handles[2],"in_y":handles[3]})
        }
    }
}
#[test]
fn slot_color_segments_match_arrival_keys_after_save_reopen() {
    for name in [
        "color-linear",
        "color-stepped",
        "color-bezier",
        "color-multiple",
        "color-constant",
    ] {
        let tmp = tempfile::tempdir().unwrap();
        let (_, p, plan) = support::saved_export(name, tmp.path());
        let w = support::wire(&plan);
        let schema::Timeline::SlotColor { slot, keys } = &p.animations[0].timelines[0] else {
            panic!()
        };
        let native = &w["animations"]["clip"]["slots"][0];
        assert_eq!(native["name"], *slot);
        let wire = native["keys"].as_array().unwrap();
        outgoing(
            wire,
            keys.iter()
                .enumerate()
                .map(|(i, _)| {
                    keys.get(i + 1)
                        .map_or(json!("linear"), |k| expected(k.interp))
                })
                .collect(),
        );
        for (k, n) in keys.iter().zip(wire) {
            assert_eq!(n["time"], json!(k.time));
            assert_eq!(n["color"], json!(k.value));
        }
    }
}
#[test]
fn transform_mix_segments_preserve_every_axis_and_curve_after_save_reopen() {
    for name in [
        "transform-linear",
        "transform-stepped",
        "transform-bezier",
        "transform-multiple",
        "transform-constant",
    ] {
        let tmp = tempfile::tempdir().unwrap();
        let (_, p, plan) = support::saved_export(name, tmp.path());
        let w = support::wire(&plan);
        let schema::Timeline::TransformConstraintMix { constraint, keys } =
            &p.animations[0].timelines[0]
        else {
            panic!()
        };
        let track = &w["animations"]["clip"]["transform"][0];
        assert_eq!(track["constraint"], *constraint);
        let wire = track["keys"].as_array().unwrap();
        outgoing(
            wire,
            keys.iter()
                .enumerate()
                .map(|(i, _)| {
                    keys.get(i + 1)
                        .map_or(json!("linear"), |k| expected(k.interp))
                })
                .collect(),
        );
        for (k, n) in keys.iter().zip(wire) {
            assert_eq!(n["time"], json!(k.time));
            let m = serde_json::to_value(k.value).unwrap();
            for field in [
                "rotate",
                "translate_x",
                "translate_y",
                "scale_x",
                "scale_y",
                "shear_x",
                "shear_y",
            ] {
                assert_eq!(
                    n[field],
                    m.get(field).cloned().unwrap_or(json!(0.0)),
                    "{field}"
                );
            }
        }
    }
}
#[test]
fn native_duration_recovers_the_author_f32_bits() {
    for name in [
        "duration-30",
        "duration-60",
        "duration-083",
        "duration-2",
        "duration-f32",
        "duration-courier",
    ] {
        let tmp = tempfile::tempdir().unwrap();
        let (_, p, plan) = support::saved_export(name, tmp.path());
        let wire = support::wire(&plan);
        let received = wire["animations"]["clip"]["duration"].as_f64().unwrap() as f32;
        assert_eq!(
            received.to_bits(),
            p.animations[0].duration.to_bits(),
            "{name}: native duration cannot change author modulo"
        );
    }
}
#[test]
fn historical_e1_presets_remain_recognized_without_rewriting_them() {
    let original = include_str!("fixtures/native_e1_preset.json");
    let saved = Preset::from_json(original).unwrap();
    let before = saved.clone();
    assert_eq!(Target::for_preset(&saved), Target::Runtime);
    assert_eq!(saved, before);
    let tmp = tempfile::tempdir().unwrap();
    let (d, p, _) = support::saved_export("duration-30", tmp.path());
    let old = run::plan(&p, &d.assets, &saved).unwrap();
    assert_ne!(
        support::wire(&old)["animations"]["clip"]["duration"]
            .as_f64()
            .unwrap() as f32,
        p.animations[0].duration
    );
    let mut dark = saved.clone();
    dark.templates[0].body = dark.templates[0].body.replace(
        r#"{{#if dark_color}},"dark_color":{{json dark_color}}{{/if}}"#,
        "",
    );
    assert_eq!(Target::for_preset(&dark), Target::LegacyRuntime);
    let mut custom = saved.clone();
    custom.templates[0].body.push(' ');
    assert_eq!(Target::for_preset(&custom), Target::Custom);
    let ctx = Context::build(&p, None, ExportInfo::default());
    assert_eq!(ctx.context_version, 1);
    assert_eq!(presets::default_preset().templates.len(), 1);
}
