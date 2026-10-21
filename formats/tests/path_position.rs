use ankhimate_formats::schema;
#[test]
fn t831_path_position_wire_variant_is_available() {
    let raw = serde_json::json!({"kind":"path_position","constraint":"path","keys":[{"time":0.,"value":0.25,"curve":"linear"}]});
    let result = serde_json::from_value::<schema::Timeline>(raw);
    assert!(
        result.is_ok(),
        "Missing native PathPosition author timeline: {result:?}"
    );
}

#[allow(dead_code, reason = "Shared native polyline fixture")]
mod rig {
    include!("../../core/tests/support/path_position_fixture.rs");
}
use ankhimate_core::{
    animation::Timeline,
    assets::AssetDb,
    pose::{Pose, evaluate},
    slotmap::SlotMap,
};
use ankhimate_formats::{ProjectRef, convert, from_json, load, save};
fn project() -> schema::Project {
    let (s, a, _, _) = rig::fixture(false, true);
    let mut animations = SlotMap::with_key();
    animations.insert(a);
    convert::to_schema(&ProjectRef {
        animation_order: &[],
        skeleton: &s,
        animations: &animations,
        assets: &AssetDb::new(),
        name: "Path é",
        fps: 30,
        export_presets: &[],
        psd_layer_paths: &Default::default(),
    })
}
fn as_ref(p: &ankhimate_formats::Loaded) -> ProjectRef<'_> {
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
#[test]
fn t831_author_v1_three_profiles_roundtrip_curves_values_and_poses() {
    let mut source = project();
    if let schema::Timeline::PathPosition { keys, .. } = &mut source.animations[0].timelines[0] {
        keys[1].interp = schema::Interp::Bezier {
            handles: [0.2, -0.1, 0.8, 1.2],
        };
    }
    let loaded = from_json(&serde_json::to_string(&source).unwrap()).unwrap();
    let dir = tempfile::tempdir().unwrap();
    for suffix in ["ankh", "ankh.json", "ankh.min.json"] {
        let path = dir.path().join(format!("chemin-é.{suffix}"));
        save(&path, &as_ref(&loaded), &[]).unwrap();
        let (back, _) = load(&path).unwrap();
        assert_eq!(convert::to_schema(&as_ref(&back)), source);
        for t in [0., 0.25, 0.5, 0.75, 1.] {
            let mut p = Pose::new();
            let mut q = Pose::new();
            let a = loaded.animations.values().next().unwrap();
            let b = back.animations.values().next().unwrap();
            evaluate(&loaded.skeleton, &[(a, t, 1.)], &mut p);
            evaluate(&back.skeleton, &[(b, t, 1.)], &mut q);
            for (x, y) in p.worlds.values().zip(q.worlds.values()) {
                assert!((x.tx - y.tx).abs() < 0.0001 && (x.ty - y.ty).abs() < 0.0001);
            }
        }
    }
    assert_eq!(schema::CURRENT_VERSION, 1);
    assert_eq!(ankhimate_formats::container::VERSION, 1);
}
#[test]
fn t831_author_old_project_without_path_track_has_no_artificial_timeline() {
    let mut source = project();
    source.animations[0].timelines.clear();
    let text = serde_json::to_string(&source).unwrap();
    let loaded = from_json(&text).unwrap();
    assert!(loaded.animations.values().all(|a| {
        !a.timelines
            .iter()
            .any(|t| matches!(t, Timeline::PathPosition { .. }))
    }));
    assert_eq!(convert::to_schema(&as_ref(&loaded)), source);
}
#[test]
fn t831_author_refuses_ambiguous_wrong_type_duplicate_missing_and_invalid_tracks() {
    for case in 0..10 {
        let mut p = project();
        match case {
            0 => p.constraints.push(p.constraints[0].clone()),
            1 => p.constraints[0].kind = "ik".into(),
            2 => p.constraints.clear(),
            3 => {
                let t = p.animations[0].timelines[0].clone();
                p.animations[0].timelines.push(t);
            }
            4 => p.slots.clear(),
            5 => p.bones.retain(|b| b.name != "tail0"),
            6 => p.skins.clear(),
            7 => {
                if let schema::Timeline::PathPosition { keys, .. } =
                    &mut p.animations[0].timelines[0]
                {
                    keys[1].time = 0.;
                }
            }
            8 => {
                if let schema::Timeline::PathPosition { keys, .. } =
                    &mut p.animations[0].timelines[0]
                {
                    keys[1].interp = schema::Interp::Bezier {
                        handles: [-0.1, 0., 0.8, 1.],
                    };
                }
            }
            _ => {
                let mut raw = serde_json::to_value(&p).unwrap();
                raw["animations"][0]["timelines"][0]["unknown"] = serde_json::json!(1);
                assert!(from_json(&raw.to_string()).is_err());
                continue;
            }
        }
        assert!(
            from_json(&serde_json::to_string(&p).unwrap()).is_err(),
            "case {case}"
        );
    }
}
#[test]
fn t831_save_invalid_path_keeps_previous_primary_and_assets() {
    let source = project();
    let mut loaded = from_json(&serde_json::to_string(&source).unwrap()).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("safe.ankh");
    save(&path, &as_ref(&loaded), &[]).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let id = loaded.skeleton.constraints.keys().next().unwrap();
    loaded.skeleton.remove_constraint(id);
    assert!(save(&path, &as_ref(&loaded), &[]).is_err());
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}

#[test]
fn t831_author_rejects_ambiguous_skin_slot_bone_and_path_entry() {
    for case in 0..3 {
        let mut p = project();
        match case {
            0 => p.skins.push(p.skins[0].clone()),
            1 => p.bones.push(p.bones[0].clone()),
            _ => {
                let e = p.skins[0].entries[0].clone();
                p.skins[0].entries.push(e);
            }
        }
        assert!(
            from_json(&serde_json::to_string(&p).unwrap()).is_err(),
            "ambiguity {case}"
        );
    }
}

#[test]
fn t831_author_path_ancestry_must_not_be_dropped_or_rebound() {
    for case in 0..3 {
        let mut p = project();
        let root = p.bones.iter().position(|b| b.name == "path").unwrap();
        match case {
            0 => p.bones[root].parent = "missing ancestor".into(),
            1 => p.bones[root].parent = "path".into(),
            _ => {
                let mut ancestor = p.bones[root].clone();
                ancestor.name = "ancestor".into();
                p.bones[root].parent = ancestor.name.clone();
                p.bones.push(ancestor.clone());
                p.bones.push(ancestor);
            }
        }
        assert!(
            from_json(&serde_json::to_string(&p).unwrap()).is_err(),
            "ancestry {case}"
        );
    }
}
