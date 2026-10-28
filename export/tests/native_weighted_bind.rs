use ankhimate_core::{
    pose::{self, Pose},
    transforms::Affine2,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{Loaded, ProjectRef, convert};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
fn view(d: &Loaded) -> ProjectRef<'_> {
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
fn load() -> Loaded {
    ankhimate_formats::load(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/native_bind_near_singular/Character.ankh"),
    )
    .unwrap()
    .0
}
fn worlds(d: &Loaded) -> BTreeMap<String, Affine2> {
    let mut p = Pose::new();
    pose::evaluate(&d.skeleton, &[], &mut p);
    d.skeleton
        .bones
        .iter()
        .map(|(id, b)| (b.name.clone(), p.world(id)))
        .collect()
}
fn point(m: Affine2, x: f64, y: f64) -> [f64; 2] {
    [
        f64::from(m.a) * x + f64::from(m.c) * y + f64::from(m.tx),
        f64::from(m.b) * x + f64::from(m.d) * y + f64::from(m.ty),
    ]
}
fn exported(d: &Loaded) -> Value {
    let mut p = presets::default_preset();
    p.atlas.settings.max_page = 256;
    p.atlas.settings.trim = false;
    let plan = run::plan(&convert::to_schema(&view(d)), &d.assets, &p).unwrap();
    let again = run::plan(&convert::to_schema(&view(d)), &d.assets, &p).unwrap();
    assert_eq!(plan.files, again.files);
    assert_eq!(plan.binaries, again.binaries);
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
#[test]
fn saved_native_near_singular_binding_reconstructs_within_point_zero_one() {
    let d = load();
    assert!(d.report.is_clean());
    let before = serde_json::to_value(convert::to_schema(&view(&d))).unwrap();
    let dir = tempfile::tempdir().unwrap();
    ankhimate_formats::save(&dir.path().join("Character.ankh"), &view(&d), &[]).unwrap();
    let (d, _) = ankhimate_formats::load(&dir.path().join("Character.ankh")).unwrap();
    assert_eq!(
        before,
        serde_json::to_value(convert::to_schema(&view(&d))).unwrap()
    );
    let j = exported(&d);
    let ws = worlds(&d);
    let head = &j["skins"][0]["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["slot"] == "head")
        .unwrap()["attachment"];
    let p = &head["vertices"];
    let mut max = 0f64;
    let mut raw_max = 0f64;
    let mut measured = vec![];
    for (i, row) in head["weights"].as_array().unwrap().iter().enumerate() {
        let source = ws["head"].transform_point(glam::Vec2::new(
            p[i * 2].as_f64().unwrap() as f32,
            p[i * 2 + 1].as_f64().unwrap() as f32,
        ));
        let (mut x, mut y, mut sum) = (0., 0., 0.);
        for w in row["bones"].as_array().unwrap() {
            let q = point(
                ws[w["bone"].as_str().unwrap()],
                w["x"].as_f64().unwrap(),
                w["y"].as_f64().unwrap(),
            );
            let weight = w["weight"].as_f64().unwrap();
            x += q[0] * weight;
            y += q[1] * weight;
            sum += weight;
            assert!(q.iter().all(|v| v.is_finite()));
        }
        let out = [x / sum, y / sum];
        let err = (out[0] as f32 - source.x)
            .abs()
            .max((out[1] as f32 - source.y).abs()) as f64;
        raw_max = raw_max
            .max((out[0] - f64::from(source.x)).abs())
            .max((out[1] - f64::from(source.y)).abs());
        max = max.max(err);
        measured.push(json!({"vertex":i,"source":[source.x,source.y],"reconstructed":out,"error_f32":err,"weights":row}));
    }
    eprintln!(
        "{}",
        json!({"matrix":ws["torso"],"native_threshold":1e-12,"old_export_threshold":f32::EPSILON,"max_error":max,"max_unrounded_error":raw_max,"vertices":measured})
    );
    assert!(
        max <= 0.01 && raw_max <= 0.01,
        "native bind fidelity: {max} exceeds 0.01; WORLD must never replace bone LOCAL"
    );
}
#[test]
fn native_tiny_bones_keep_parameters_and_local_geometry() {
    let mut d = load();
    let id = d
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    d.skeleton.bones[id].local_transform.scale = glam::Vec2::new(1e-6, 2e-6);
    let source = convert::to_schema(&view(&d));
    let j = exported(&d);
    let b = j["bones"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["name"] == "torso")
        .unwrap();
    assert_eq!(b["scaleX"].as_f64().unwrap() as f32, 1e-6);
    assert_eq!(b["scaleY"].as_f64().unwrap() as f32, 2e-6);
    let m = &j["skins"][0]["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["slot"] == "head")
        .unwrap()["attachment"];
    let original = source.skins[0]
        .entries
        .iter()
        .find(|e| e.slot == "head")
        .unwrap();
    let ankhimate_formats::schema::Attachment::Mesh(original) = &original.attachment else {
        panic!()
    };
    assert_eq!(
        m["uvs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect::<Vec<_>>(),
        original.uvs
    );
    assert_eq!(m["triangles"], json!(original.triangles));
    assert_eq!(
        m["vertices"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect::<Vec<_>>(),
        original.vertices
    );
    for (i, row) in m["weights"].as_array().unwrap().iter().enumerate() {
        assert_eq!(
            row["count"].as_u64().unwrap() as usize,
            original.weights[i].len()
        );
        for (k, w) in row["bones"].as_array().unwrap().iter().enumerate() {
            assert_eq!(w["bone"], original.weights[i][k].0);
            assert_eq!(w["weight"], json!(original.weights[i][k].1));
        }
    }
}
#[test]
fn impossible_native_bind_is_explicit_and_plan_does_not_touch_output() {
    for scale in [0., 1e-7, f32::INFINITY, f32::NAN] {
        let mut d = load();
        let id = d
            .skeleton
            .bones
            .iter()
            .find(|(_, b)| b.name == "torso")
            .unwrap()
            .0;
        d.skeleton.bones[id].local_transform.scale = glam::Vec2::splat(scale);
        let project = convert::to_schema(&view(&d));
        let context = ankhimate_export::context::Context::build(&project, None, Default::default());
        assert!(!context.binding_errors.is_empty());
        assert!(context.to_value()["binding_errors"].is_array());
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("old-export.json"), b"last valid output").unwrap();
        let result = run::plan(&project, &d.assets, &presets::default_preset());
        assert!(result.is_err(), "{scale}");
        assert!(result.unwrap_err().to_string().contains("weighted bind"));
        assert_eq!(
            std::fs::read(dir.path().join("old-export.json")).unwrap(),
            b"last valid output"
        );
    }
}
#[test]
fn three_influences_nonunit_sums_and_zero_coefficients_keep_native_structure() {
    let mut d = load();
    let ids: BTreeMap<_, _> = d
        .skeleton
        .bones
        .iter()
        .map(|(id, b)| (b.name.clone(), id))
        .collect();
    for (_, skin) in d.skeleton.skins.iter_mut() {
        for a in skin.entries.values_mut() {
            if let ankhimate_core::attachment::Attachment::Mesh(m) = a {
                for row in &mut m.weights {
                    *row = vec![
                        ankhimate_core::attachment::VertexWeight {
                            bone: ids["head"],
                            weight: 2.,
                        },
                        ankhimate_core::attachment::VertexWeight {
                            bone: ids["torso"],
                            weight: 3.,
                        },
                        ankhimate_core::attachment::VertexWeight {
                            bone: ids["root"],
                            weight: 0.,
                        },
                    ];
                }
            }
        }
    }
    let j = exported(&d);
    let rows = &j["skins"][0]["attachments"][0]["attachment"]["weights"];
    for row in rows.as_array().unwrap() {
        assert_eq!(row["count"], 3);
        assert_eq!(row["bones"][0]["weight"], 2.);
        assert_eq!(row["bones"][1]["weight"], 3.);
        assert_eq!(row["bones"][2]["weight"], 0.);
    }
}
#[test]
fn old_saved_rounding_preset_is_recognized_and_cannot_silently_collapse_binds() {
    let mut d = load();
    let id = d
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    d.skeleton.bones[id].local_transform.scale = glam::Vec2::new(1e-6, 2e-6);
    let root = d
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "root")
        .unwrap()
        .0;
    let slot = d
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "head")
        .unwrap()
        .0;
    d.skeleton.slots[slot].bone = root;
    let project = convert::to_schema(&view(&d));
    let legacy = ankhimate_export::preset::Preset::from_json(include_str!(
        "fixtures/runtime_v2_before_dark_tint.json"
    ))
    .unwrap();
    assert_eq!(
        ankhimate_export::preflight::Target::for_preset(&legacy),
        ankhimate_export::preflight::Target::LegacyRuntime
    );
    let before = legacy.to_json();
    let result = run::plan(&project, &d.assets, &legacy);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("precision loses fidelity")
    );
    assert_eq!(before, legacy.to_json());
}

#[test]
fn native_serialized_bind_preserves_single_multi_influence_transformed_setups() {
    for (name, scale, rotation, root_position, single) in [
        ("identity-single", [1., 1.], 0., [0., 0.], true),
        ("translated-parent", [1., 1.], 0., [83., -42.], false),
        ("rotated-parent", [1., 1.], 0.71, [18., -37.], false),
        ("uniform", [2., 2.], -0.34, [9., 12.], true),
        ("nonuniform", [0.25, 4.], 0.52, [21., -9.], false),
        ("reflected", [-2., 0.5], -0.71, [-8., 15.], false),
        ("det-near-1e-7", [0.0005, 0.0002], 0., [10000., 25.], false),
        ("det-near-1e-8", [0.0001, 0.0001], 0., [10000., 25.], false),
        (
            "det-above-native-boundary",
            [0.000001, 0.00000101],
            0.,
            [0., 0.],
            false,
        ),
    ] {
        let mut d = load();
        let ids: BTreeMap<_, _> = d
            .skeleton
            .bones
            .iter()
            .map(|(id, b)| (b.name.clone(), id))
            .collect();
        d.skeleton.bones[ids["root"]].local_transform.position = glam::Vec2::from(root_position);
        d.skeleton.bones[ids["root"]].local_transform.rotation = rotation;
        d.skeleton.bones[ids["torso"]].local_transform.scale = glam::Vec2::from(scale);
        if single {
            for (_, skin) in d.skeleton.skins.iter_mut() {
                for a in skin.entries.values_mut() {
                    if let ankhimate_core::attachment::Attachment::Mesh(m) = a {
                        for row in &mut m.weights {
                            *row = vec![ankhimate_core::attachment::VertexWeight {
                                bone: ids["torso"],
                                weight: 1.,
                            }];
                        }
                    }
                }
            }
        }
        let source = worlds(&d);
        let j = exported(&d);
        let mut serialized = convert::to_schema(&view(&d));
        for b in &mut serialized.bones {
            let entry = j["bones"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["name"] == b.name)
                .unwrap();
            b.tx = entry["x"].as_f64().unwrap() as f32;
            b.ty = entry["y"].as_f64().unwrap() as f32;
            b.rotation = entry["rotation"].as_f64().unwrap() as f32;
            b.sx = entry["scaleX"].as_f64().unwrap() as f32;
            b.sy = entry["scaleY"].as_f64().unwrap() as f32;
            b.shear_x = entry["shearX"].as_f64().unwrap() as f32;
            b.shear_y = entry["shearY"].as_f64().unwrap() as f32;
        }
        let loaded = convert::from_schema(&serialized);
        let placed = worlds(&loaded);
        let mut max = 0f64;
        for a in j["skins"][0]["attachments"].as_array().unwrap() {
            let m = &a["attachment"];
            if m["weights"].as_array().is_none_or(|w| w.is_empty()) {
                continue;
            }
            let host = d
                .skeleton
                .slots
                .iter()
                .find(|(_, s)| Some(s.name.as_str()) == a["slot"].as_str())
                .unwrap()
                .1;
            let host = &source[&d.skeleton.bones[host.bone].name];
            for (i, row) in m["weights"].as_array().unwrap().iter().enumerate() {
                let original = host.transform_point(glam::Vec2::new(
                    m["vertices"][2 * i].as_f64().unwrap() as f32,
                    m["vertices"][2 * i + 1].as_f64().unwrap() as f32,
                ));
                let (mut x, mut y, mut total) = (0., 0., 0.);
                for w in row["bones"].as_array().unwrap() {
                    let q = point(
                        placed[w["bone"].as_str().unwrap()],
                        w["x"].as_f64().unwrap(),
                        w["y"].as_f64().unwrap(),
                    );
                    let weight = w["weight"].as_f64().unwrap();
                    x += q[0] * weight;
                    y += q[1] * weight;
                    total += weight;
                }
                max = max
                    .max((x / total - original.x as f64).abs())
                    .max((y / total - original.y as f64).abs());
            }
        }
        eprintln!("{name}: independent actual-serialized WORLD residual={max}");
        assert!(max <= 0.01, "{name}: {max}");
    }
}

#[test]
fn unresolved_influence_and_malformed_weight_dimensions_are_refused() {
    for missing_bone in [true, false] {
        let d = load();
        let mut project = convert::to_schema(&view(&d));
        for skin in &mut project.skins {
            for entry in &mut skin.entries {
                if let ankhimate_formats::schema::Attachment::Mesh(m) = &mut entry.attachment {
                    if missing_bone {
                        m.weights[0][0].0 = "missing influence".into();
                    } else {
                        m.weights.push(m.weights[0].clone());
                    }
                }
            }
        }
        let result = run::plan(&project, &d.assets, &presets::default_preset());
        assert!(result.is_err());
        let message = result.err().unwrap().to_string();
        assert!(
            message.contains(if missing_bone {
                "unresolved"
            } else {
                "dimensions disagree"
            }),
            "{message}"
        );
    }
}
