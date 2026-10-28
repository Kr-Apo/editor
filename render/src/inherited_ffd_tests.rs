use super::*;
use ankhimate_core::{animation::Timeline, replay_consumer::ReplayConsumerBridge};
use serde_json::json;
mod fixture {
    include!("../../editor/tests/support/linked_frog.rs");
}
fn output(name: &str) -> (Option<tempfile::TempDir>, std::path::PathBuf) {
    if let Ok(p) = std::env::var("A10_OUTPUT") {
        let p = std::path::PathBuf::from(p).join(name);
        std::fs::create_dir(&p).unwrap();
        (None, p)
    } else {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().to_path_buf();
        (Some(tmp), p)
    }
}
#[test]
fn a10_continuous_independent_reference() {
    let d = fixture::fixture();
    let full = d.animations.values().next().unwrap();
    let mut bones_only = full.clone();
    bones_only.timelines.retain(|t| {
        !matches!(
            t,
            Timeline::Deform { .. }
                | Timeline::SlotColor { .. }
                | Timeline::SlotVisible { .. }
                | Timeline::SlotAttachment { .. }
                | Timeline::DrawOrder { .. }
        )
    });
    bones_only.events.clear();
    let mut replay =
        ReplayConsumerBridge::new(&d.skeleton, &[bones_only], "linked_frog", 1, None).unwrap();
    let context = replay.context();
    replay.play(&context, "linked_frog", false).unwrap();
    let (_tmp, out) = output("reference");
    std::fs::create_dir(out.join("source")).unwrap();
    for image in d.assets.images.values() {
        std::fs::write(
            out.join("source").join(format!("{}.png", image.name)),
            &image.bytes,
        )
        .unwrap();
    }
    let focus = ResolvedFocus::new(&d, None).unwrap();
    let textures = decode_textures(&d);
    let camera = ResolvedCamera {
        center: Vec2::ZERO,
        zoom: 3.,
    };
    let mut frames = vec![];
    let mut maximum = 0.0_f64;
    let mut offset_error = 0.0_f32;
    for tick in 0..=720 {
        if tick > 0 {
            replay.request_tick(&context, tick, |_| {}).unwrap();
        }
        let accepted = replay.current_frame().unwrap();
        assert_eq!(accepted.completed, tick);
        let mut p = fixture::pose(&d, accepted.clip_time);
        for (id, b) in &d.skeleton.bones {
            let state = accepted.bones.iter().find(|v| v.name == b.name).unwrap();
            p.locals[id] = state.local;
            p.worlds[id] = state.world;
        }
        let actual = build_draws(&d, &p, &focus).unwrap();
        let mut expected = vec![];
        let mut owners = vec![];
        for &slot in &p.draw_order {
            if p.slot_visible.get(slot) == Some(&false) {
                continue;
            }
            let name = p.attachment_name(&d.skeleton, slot);
            let Some(name) = name else {
                continue;
            };
            let Some(a) = d
                .skeleton
                .resolve_posed(&[d.skeleton.default_skin], &p, slot)
            else {
                continue;
            };
            let host = p.world(d.skeleton.slots[slot].bone);
            let (texture, points, uv, indices, terms) = match a {
                Attachment::Mesh(m) => {
                    let (source, owner) = match name {
                        "linked_royal" => ("body", "body"),
                        "linked_plain" => ("body", "linked_plain"),
                        "linked_cape" => ("cape", "linked_cape"),
                        _ => (name, name),
                    };
                    let (points, terms) =
                        fixture::independent(&d, &p, slot, source, owner, accepted.clip_time);
                    let Attachment::Mesh(g) = d.skeleton.skins[d.skeleton.default_skin]
                        .get(slot, source)
                        .unwrap()
                    else {
                        panic!()
                    };
                    let resolved = d
                        .skeleton
                        .resolve_mesh_pose(&[d.skeleton.default_skin], &p, slot, name)
                        .unwrap();
                    assert_eq!(resolved.geometry_owner.attachment, source);
                    assert_eq!(resolved.deform_owner.attachment, owner);
                    let independent = fixture::offsets(full, slot, owner, accepted.clip_time);
                    let actual_offsets = resolved.offsets.unwrap_or_default();
                    assert_eq!(independent.len(), actual_offsets.len());
                    for (a, b) in independent.iter().zip(actual_offsets) {
                        offset_error = offset_error.max((*a - *b).abs().max_element());
                        assert!((*a - *b).abs().max_element() < 0.001);
                    }
                    owners.push(json!({"skin":"default","slot":d.skeleton.slots[slot].name,"displayed":name,"geometrySource":source,"deformOwner":owner,"texture":m.texture,"offsets":independent.iter().map(|v|v.to_array()).collect::<Vec<_>>()}));
                    (
                        m.texture.clone(),
                        points,
                        g.uvs.iter().map(|v| v.to_array()).collect::<Vec<_>>(),
                        g.triangles.iter().flatten().copied().collect::<Vec<_>>(),
                        terms,
                    )
                }
                Attachment::Region(reg) => {
                    let points = reg
                        .local_corners()
                        .map(|v| host.transform_point(v))
                        .map(|v| [f64::from(v.x), f64::from(v.y)])
                        .to_vec();
                    let u = &reg.uv_rect;
                    (
                        reg.texture.clone(),
                        points,
                        vec![
                            [u.x, u.y],
                            [u.x, u.y + u.h],
                            [u.x + u.w, u.y + u.h],
                            [u.x + u.w, u.y],
                        ],
                        vec![0, 1, 2, 0, 2, 3],
                        vec![],
                    )
                }
                _ => panic!("outside A10"),
            };
            let index = expected.len();
            let actual = &actual[index];
            assert_eq!(actual.texture, texture);
            assert_eq!(actual.indices, indices);
            assert_eq!(actual.vertices.len(), points.len());
            for (i, (v, p)) in actual.vertices.iter().zip(&points).enumerate() {
                assert_eq!(v.uv.to_array(), uv[i]);
                let error = (f64::from(v.world.x) - p[0])
                    .abs()
                    .max((f64::from(v.world.y) - p[1]).abs());
                maximum = maximum.max(error);
                assert!(error <= 0.01, "tick{tick} {name} {i} {error}");
            }
            expected.push(json!({"slot":d.skeleton.slots[slot].name,"attachment":name,"texture":texture,"world":points,"positions":points.iter().flat_map(|p|[p[0],-p[1]]).collect::<Vec<_>>(),"sourceUV":uv,"indices":indices,"contributions":terms,"color":p.slot_colors[slot]}));
        }
        assert_eq!(actual.len(), expected.len());
        if [
            0, 15, 30, 47, 48, 59, 60, 89, 90, 95, 96, 97, 120, 131, 132, 179, 180, 181, 360, 720,
        ]
        .contains(&tick)
        {
            let mut canvas = Canvas::new(640, 480, [0; 4]);
            let trail = [(accepted.clip_time, &p)];
            let scene = PaintContext {
                doc: &d,
                focus: &focus,
                camera,
                trail_poses: &trail,
                textures: &textures,
            };
            paint_scene(
                &mut canvas,
                Viewport::new(0, 0, 640, 480),
                &p,
                &actual,
                &scene,
            );
            std::fs::write(out.join(format!("tick{tick}.png")), canvas.png().unwrap()).unwrap();
            std::fs::write(out.join(format!("tick{tick}.rgba")), canvas.image.as_raw()).unwrap();
        }
        frames.push(json!({"tick":tick,"clipTime":accepted.clip_time,"preview":false,"owners":owners,"ffdSources":full.timelines.iter().filter_map(|t|match t{Timeline::Deform{slot,attachment,..}=>Some(json!({"skin":"default","slot":d.skeleton.slots[*slot].name,"attachment":attachment,"offsets":fixture::offsets(full,*slot,attachment,accepted.clip_time).iter().map(|v|v.to_array()).collect::<Vec<_>>()})),_=>None}).collect::<Vec<_>>(),"draws":expected,"bones":accepted.bones.iter().map(|b|json!({"name":b.name,"world":[b.world.a,b.world.b,b.world.c,b.world.d,b.world.tx,b.world.ty]})).collect::<Vec<_>>(),"records":accepted.records.iter().map(|r|json!({"constraint":r.constraint,"rotation":r.rotation,"position":r.position,"rotationVelocity":r.rotation_velocity})).collect::<Vec<_>>()}));
    }
    assert!(maximum > 0.);
    std::fs::write(out.join("oracle.json"),serde_json::to_vec(&json!({"source":"Explicit author identity table, independent f64 affine point+linear-vector algebra and Bezier bisection; matrices are accepted native Replay Physics. No corrected lookup or renderer vertices used as expected data.","maximum":maximum,"offsetError":offset_error,"frames":frames})).unwrap()).unwrap();
    println!("A10 721 accepted reference states maxWorld={maximum} offsetError={offset_error}");
}
#[test]
fn a10_save_reload_native_export_and_own_track() {
    let mut d = fixture::fixture();
    let (_tmp, out) = output("author");

    let before = ankhimate_formats::convert::to_schema(&view(&d));
    ankhimate_formats::save(&out.join("Linked-Frog.ankh"), &view(&d), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&out.join("Linked-Frog.ankh")).unwrap();
    let mut reopened = Document::from_loaded(loaded);
    ankhimate_document::rebind::rebind_meshes(&mut reopened);
    assert_eq!(
        before,
        ankhimate_formats::convert::to_schema(&view(&reopened))
    );
    let mut preset = ankhimate_export::presets::default_preset();
    preset.atlas.settings.trim = false;
    preset.atlas.settings.max_page = 96;
    let plan = ankhimate_export::run::plan(&before, &d.assets, &preset).unwrap();
    ankhimate_export::run::write(&plan, &out.join("native")).unwrap();
    for (name, bytes) in [
        (
            "skeleton.json",
            include_bytes!("../../editor/tests/fixtures/inherited_ffd/native/skeleton.json")
                .as_slice(),
        ),
        (
            "atlas.png",
            include_bytes!("../../editor/tests/fixtures/inherited_ffd/native/atlas.png").as_slice(),
        ),
        (
            "atlas_2.png",
            include_bytes!("../../editor/tests/fixtures/inherited_ffd/native/atlas_2.png")
                .as_slice(),
        ),
        (
            "atlas_3.png",
            include_bytes!("../../editor/tests/fixtures/inherited_ffd/native/atlas_3.png")
                .as_slice(),
        ),
    ] {
        assert_eq!(
            std::fs::read(out.join("native").join(name)).unwrap(),
            bytes,
            "A2 native bytes unchanged {name}"
        );
    }
    let slot = d
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "body")
        .unwrap()
        .0;
    let a = d.animations.values_mut().next().unwrap();
    let mut own = a
        .timelines
        .iter()
        .find_map(|t| match t {
            Timeline::Deform { keys, .. } => Some(keys.clone()),
            _ => None,
        })
        .unwrap();
    for key in &mut own {
        for value in &mut key.value {
            *value *= -0.35;
        }
    }
    a.timelines.push(Timeline::Deform {
        slot,
        attachment: "linked_plain".into(),
        keys: own,
    });
    let p = fixture::pose(&d, 0.5);
    let r = d
        .skeleton
        .resolve_mesh_pose(&[d.skeleton.default_skin], &p, slot, "linked_plain")
        .unwrap();
    assert_eq!(r.deform_owner.attachment, "linked_plain");
    assert!(!r.offsets.unwrap().is_empty());
    let (oracle, _) = fixture::independent(&d, &p, slot, "body", "linked_plain", 0.5);
    for (a, b) in r.vertices(&p).unwrap().iter().zip(oracle) {
        assert!((f64::from(a.x) - b[0]).abs() < 0.01 && (f64::from(a.y) - b[1]).abs() < 0.01);
    }
    // Own-track persistence/export is also native; no JSON patch.
    let owned_png = render_frame(
        &d,
        &FrameRequest {
            animation: Some("linked_frog".into()),
            time: 0.5,
            ..Default::default()
        },
    )
    .unwrap();
    let plain_png = render_frame(
        &fixture::fixture(),
        &FrameRequest {
            animation: Some("linked_frog".into()),
            time: 0.5,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(owned_png.bytes != plain_png.bytes);
    let own_schema = ankhimate_formats::convert::to_schema(&view(&d));
    ankhimate_formats::save(&out.join("Own-Track.ankh"), &view(&d), &[]).unwrap();
    let (loaded, _) = ankhimate_formats::load(&out.join("Own-Track.ankh")).unwrap();
    let loaded = Document::from_loaded(loaded);
    assert_eq!(
        own_schema,
        ankhimate_formats::convert::to_schema(&view(&loaded))
    );
    let plan = ankhimate_export::run::plan(&own_schema, &d.assets, &preset).unwrap();
    ankhimate_export::run::write(&plan, &out.join("own-native")).unwrap();
    fixture::flag(&mut d, false);
    assert!(!fixture::pose(&d, 0.75).deforms.is_empty());
}

fn view(d: &Document) -> ankhimate_formats::ProjectRef<'_> {
    ankhimate_formats::ProjectRef {
        skeleton: &d.skeleton,
        animations: &d.animations,
        animation_order: &[],
        assets: &d.assets,
        name: &d.meta.name,
        fps: d.meta.fps,
        export_presets: &d.export_presets,
        psd_layer_paths: &d.psd_layer_paths,
    }
}

#[test]
fn a10_cpu_rejects_invalid_link_and_sampled_ffd() {
    for case in 0..3 {
        let mut d = fixture::fixture();
        let slot = d
            .skeleton
            .slots
            .iter()
            .find(|(_, s)| s.name == "body")
            .unwrap()
            .0;
        let skin = d.skeleton.default_skin;
        if case == 0 {
            let Attachment::Mesh(m) = d.skeleton.skins[skin]
                .entries
                .get_mut(&(slot, "linked_royal".into()))
                .unwrap()
            else {
                panic!()
            };
            m.linked.as_mut().unwrap().attachment = "missing".into();
        } else {
            let a = d.animations.values_mut().next().unwrap();
            for t in &mut a.timelines {
                if let Timeline::Deform { keys, .. } = t {
                    for k in keys {
                        if case == 1 {
                            k.value.truncate(2);
                        } else {
                            k.value[0].x = f32::NAN;
                        }
                    }
                }
            }
        }
        let p = fixture::pose(&d, 0.75);
        let focus = ResolvedFocus::new(&d, None).unwrap();
        assert!(build_draws(&d, &p, &focus).is_err());
    }
}
