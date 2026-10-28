//! Headless numerical evidence for the E1 saved region/static-mesh fixtures.
//! Usage: native_bind_reference SOURCE.ankh NEW_OUTPUT [export-only]
//! Default mode additionally samples their ReplayConsumer clips; it does not
//! enable weighted rendering or qualify any graphics adapter. Existing output
//! directories are refused. export-only accepts general author projects.
use ankhimate_core::{
    replay_consumer::{ConsumerBone, ConsumerHealth, ReplayConsumerBridge},
    replay_skin::SkinEvent,
};
use ankhimate_export::{presets, run};
use ankhimate_formats::{ProjectRef, convert};
use serde_json::{Value, json};
use std::{fs, path::Path};
fn view(d: &ankhimate_formats::Loaded) -> ProjectRef<'_> {
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
fn pose(
    d: &ankhimate_formats::Loaded,
    skin: &str,
    bones: &[ConsumerBone],
    native: &Value,
) -> Value {
    let s = &d.skeleton;
    let active = s.skins.iter().find(|(_, v)| v.name == skin).unwrap().0;
    let absent: Vec<_> = s
        .draw_order
        .iter()
        .filter(|id| s.resolve_slot(active, **id).is_none())
        .map(|id| s.slots[*id].name.clone())
        .collect();
    let draws:Vec<_>=s.draw_order.iter().filter_map(|id|{
  let slot=&s.slots[*id];let a=s.resolve_slot(active,*id)?;
  let (texture, local, uvs, triangles, kind)=match a {
 ankhimate_core::attachment::Attachment::Region(a)=>(a.texture.clone(),a.local_corners().to_vec(),vec![[0.,0.],[0.,1.],[1.,1.],[1.,0.]],vec![[0,1,2],[0,2,3]],"region"),
 ankhimate_core::attachment::Attachment::Mesh(a)=>{assert!(a.linked.is_none()&&a.ffd_keyframes.is_empty());(a.texture.clone(),a.setup_vertices.clone(),a.uvs.iter().map(|v|[v.x,v.y]).collect(),a.triangles.clone(),"mesh")},_=>panic!("unqualified")};
  let world=bones.iter().find(|b|b.name==s.bones[slot.bone].name).unwrap().world;
  let native_attachment=&native["skins"][0]["attachments"].as_array().unwrap().iter().find(|e|e["slot"].as_str()==Some(slot.name.as_str())).unwrap()["attachment"];
  let weights=native_attachment.get("weights").cloned().unwrap_or(json!([]));
  let corners:Vec<[f64;2]>=local.iter().enumerate().map(|(i,p)|{
   let row=weights.get(i).and_then(|v|v["bones"].as_array());let mut sum=0.;let mut x=0.;let mut y=0.;
   if let Some(row)=row{for w in row{let m=bones.iter().find(|b|Some(b.name.as_str())==w["bone"].as_str()).unwrap().world;let(px,py,weight)=(w["x"].as_f64().unwrap(),w["y"].as_f64().unwrap(),w["weight"].as_f64().unwrap());x+=(m.a as f64*px+m.c as f64*py+m.tx as f64)*weight;y+=(m.b as f64*px+m.d as f64*py+m.ty as f64)*weight;sum+=weight;}}
   if sum>0.{[x/sum,-y/sum]}else{[world.a as f64*p.x as f64+world.c as f64*p.y as f64+world.tx as f64,-(world.b as f64*p.x as f64+world.d as f64*p.y as f64+world.ty as f64)]}
  }).collect();
  let mut native_pose=ankhimate_core::pose::Pose::new();ankhimate_core::pose::evaluate(s,&[],&mut native_pose);for (id,b)in s.bones.iter(){native_pose.worlds[id]=bones.iter().find(|p|p.name==b.name).unwrap().world;}
  let mut native_max=0f64;let mut native_corners=vec![];
  if let ankhimate_core::attachment::Attachment::Mesh(m)=a{for(i,p)in corners.iter().enumerate(){let c=m.skin_vertex_with_deform(i,&[],&native_pose,&world);native_corners.push([c.x as f64,-c.y as f64]);native_max=native_max.max((c.x as f64-p[0]).abs()).max((-c.y as f64-p[1]).abs());}assert!(native_max.is_finite());}
  Some(json!({"nativeCorners":native_corners,"nativeMax":native_max,"weights":weights,"slot":slot.name,"attachment":slot.attachment,"texture":texture,"type":kind,"local":local.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"uvs":uvs,"triangles":triangles,"corners":corners}))
 }).collect();
    json!({"skin":skin,"absent":absent,"draws":draws,"bones":bones.iter().map(|b|json!({"name":b.name,"world":[b.world.a,b.world.b,b.world.c,b.world.d,b.world.tx,b.world.ty],"local":[b.local.position.x,b.local.position.y,b.local.rotation,b.local.scale.x,b.local.scale.y,b.local.shear.x,b.local.shear.y]})).collect::<Vec<_>>()})
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir(out).unwrap();
    let (d, _) = ankhimate_formats::load(&if input.is_file() {
        input.to_path_buf()
    } else {
        input.join("Character.ankh")
    })
    .unwrap();
    assert!(d.report.is_clean(), "{:?}", d.report);
    let before_schema = serde_json::to_value(convert::to_schema(&view(&d))).unwrap();
    let before_images: std::collections::BTreeMap<_, _> = d
        .assets
        .images
        .values()
        .map(|a| (a.name.clone(), a.bytes.clone()))
        .collect();
    ankhimate_formats::save(&out.join("Character.ankh"), &view(&d), &[]).unwrap();
    let (mut d, _) = ankhimate_formats::load(&out.join("Character.ankh")).unwrap();
    assert!(d.report.is_clean());
    assert_eq!(
        before_schema,
        serde_json::to_value(convert::to_schema(&view(&d))).unwrap(),
        "Save/Reload author schema"
    );
    assert_eq!(
        before_images,
        d.assets
            .images
            .values()
            .map(|a| (a.name.clone(), a.bytes.clone()))
            .collect(),
        "Save/Reload image bytes"
    );
    fs::write(out.join("SAVE-RELOAD.json"),serde_json::to_vec_pretty(&json!({"authorSchemaEqual":true,"imageBytesEqual":true,"images":before_images.len(),"weightedDataPreserved":true})).unwrap()).unwrap();
    let mut setup = ankhimate_core::pose::Pose::new();
    ankhimate_core::pose::evaluate(&d.skeleton, &[], &mut setup);
    let source_meshes:Vec<_> = d.skeleton.skins.iter().flat_map(|(_,skin)| {
        let setup=&setup;
        let skeleton=&d.skeleton;
        skin.entries.iter().filter_map(move |((slot,name),a)| {
            let ankhimate_core::attachment::Attachment::Mesh(m)=a else {return None;};
            let host=setup.world(skeleton.slots[*slot].bone);
            Some(json!({"skin":skin.name,"slot":skeleton.slots[*slot].name,"attachment":name,
                "host":host,"vertices":m.setup_vertices.iter().map(|v|[v.x,v.y]).collect::<Vec<_>>(),
                "sourceWorldF32":m.setup_vertices.iter().map(|v|host.transform_point(*v).to_array()).collect::<Vec<_>>() }))
        })
    }).collect();
    fs::write(out.join("BIND-REFERENCE.json"),serde_json::to_vec_pretty(&json!({
        "bones":d.skeleton.bones.iter().map(|(id,b)|{
            let world=setup.world(id);
            json!({"name":b.name,"world":world,"determinantF32":world.determinant(),"nativeInverse":world.invert()})
        }).collect::<Vec<_>>(),"meshes":source_meshes,"sourcePrecision":"f32"
    })).unwrap()).unwrap();
    let slot_worlds: std::collections::HashMap<_, _> = d
        .skeleton
        .slots
        .iter()
        .map(|(id, s)| (id, setup.world(s.bone)))
        .collect();
    for (_, skin) in d.skeleton.skins.iter_mut() {
        for ((slot, _), a) in skin.entries.iter_mut() {
            if let ankhimate_core::attachment::Attachment::Mesh(m) = a {
                m.bind_to_pose(&setup, slot_worlds[slot]);
            }
        }
    }
    let mut preset = presets::default_preset();
    preset.atlas.settings.max_page = 256;
    preset.atlas.settings.trim = input.file_name().unwrap() == "mixed";
    let plan = run::plan(&convert::to_schema(&view(&d)), &d.assets, &preset).unwrap();
    run::write(&plan, out).unwrap();
    let native: Value =
        serde_json::from_slice(&fs::read(out.join("skeleton.json")).unwrap()).unwrap();
    if args.get(3).is_some_and(|v| v == "export-only") {
        return;
    }
    let clips: Vec<_> = d.animations.values().cloned().collect();
    for clip in clips.iter().map(|a| a.name.as_str()) {
        let name = clip;
        let events = vec![SkinEvent {
            tick: 0,
            skin: "default".into(),
        }];
        let mut b = ReplayConsumerBridge::new(&d.skeleton, &clips, clip, 1, Some(&events)).unwrap();
        b.play(&b.context(), clip, false).unwrap();
        let mut frames = vec![];
        for requested in 0..=240 {
            if requested > 0 {
                b.request_tick(&b.context(), requested, |_| {}).unwrap();
            }
            let f = b.current_frame().unwrap();
            let mut v = pose(&d, &f.skin, &f.bones, &native);
            v["tick"] = json!(f.pose_tick);
            v["requested"] = json!(f.requested);
            v["completed"] = json!(f.completed);
            v["backlog"] = json!(f.backlog);
            v["health"] = json!(format!("{:?}", f.health).to_lowercase());
            v["active"] = json!(
                f.records
                    .iter()
                    .map(|r| d.skeleton.constraints[r.constraint].name())
                    .collect::<Vec<_>>()
            );
            v["records"]=json!(f.records.iter().map(|r|json!({"constraint":d.skeleton.constraints[r.constraint].name(),"bone":d.skeleton.bones[r.bone].name,"rotation":r.rotation,"x":r.position.x,"y":r.position.y,"rotationVelocity":r.rotation_velocity,"velocityX":r.position_velocity.x,"velocityY":r.position_velocity.y,"anchorX":r.anchor.x,"anchorY":r.anchor.y})).collect::<Vec<_>>());
            frames.push(v);
            if f.health == ConsumerHealth::Faulted {
                break;
            }
        }
        fs::write(
            out.join(format!("rust-{name}.json")),
            serde_json::to_vec(&frames).unwrap(),
        )
        .unwrap();
        let mut previews = vec![];
        if clip != "fault_at_20" {
            for tick in [
                0, 1, 8, 19, 20, 21, 30, 39, 40, 60, 119, 120, 121, 239, 240, 241, 600,
            ] {
                b.seek_tick(&b.context(), tick).unwrap();
                let f = b.preview_frame().unwrap().unwrap();
                let mut v = pose(&d, &f.skin, &f.bones, &native);
                v["tick"] = json!(tick);
                previews.push(v);
            }
            fs::write(
                out.join(format!("rust-preview-{name}.json")),
                serde_json::to_vec(&previews).unwrap(),
            )
            .unwrap();
        }
    }
    let mut max_error = 0f64;
    let mut frame_count = 0usize;
    for file in fs::read_dir(out).unwrap() {
        let file = file.unwrap();
        if !file.file_name().to_string_lossy().starts_with("rust-") {
            continue;
        }
        let frames: Value = serde_json::from_slice(&fs::read(file.path()).unwrap()).unwrap();
        for frame in frames.as_array().unwrap() {
            frame_count += 1;
            for draw in frame["draws"].as_array().unwrap() {
                max_error = max_error.max(draw["nativeMax"].as_f64().unwrap());
            }
        }
    }
    fs::write(
        out.join("NUMERICAL-RESULT.json"),
        serde_json::to_vec_pretty(&json!({
            "frames": frame_count, "maxNativeSkinningError": max_error,
            "tolerance": 0.01, "graphics": "NOT_RUN"
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(
        max_error <= 0.01,
        "native skinning discrepancy: {max_error} > 0.01"
    );
    println!("Native bind reconstruction evidence complete; no graphics qualification");
}
