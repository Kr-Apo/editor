use super::*;
use ankhimate_document::commands::{
    animation_order::{Direction, MoveAnimation},
    key_cmds::{PreparedBoneOffset, RetimeAnimation},
};
#[allow(dead_code, reason = "Shared synthetic weighted FFD/Path fixture")]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t835_fixture.rs"
    ));
}
#[test]
fn t835_app_save_save_as_reopen_failure_dirty_assets_and_export() {
    let (d, ids) = rig::fixture();
    let mut app = AnkhimateApp::default();
    app.state.replace_document(d);
    app.state.session.active_animation = Some(ids[0]);
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("before.ankh");
    app.save_project_to(&first);
    assert!(!app.is_dirty());
    let before = std::fs::read(&first).unwrap();
    let payload = rig::payloads(&app.state.doc);
    let c = MoveAnimation::prepare(&app.state.doc, ids[1], Direction::Up)
        .unwrap()
        .unwrap();
    app.state.dispatch(Box::new(c));
    assert!(app.is_dirty());
    assert_eq!(rig::payloads(&app.state.doc), payload);
    app.state.undo();
    assert!(!app.is_dirty());
    app.state.redo();
    assert!(app.is_dirty());
    app.save_project_to(dir.path());
    assert!(app.is_dirty());
    assert_eq!(app.current_path, Some(first.clone()));
    assert_eq!(std::fs::read(&first).unwrap(), before);
    let next = dir.path().join("Ordre-鍵.ankh");
    app.save_project_to(&next);
    assert!(!app.is_dirty());
    let expected = ankhimate_formats::convert::to_schema(&app.state.doc.as_project_ref());
    let assets: Vec<_> = app
        .state
        .doc
        .assets
        .images
        .values()
        .map(|a| a.bytes.clone())
        .collect();
    let mut reopened = AnkhimateApp::default();
    reopened.open_initial(Some(next));
    assert!(!reopened.is_dirty());
    assert_eq!(
        serde_json::to_value(ankhimate_formats::convert::to_schema(
            &reopened.state.doc.as_project_ref()
        ))
        .unwrap(),
        serde_json::to_value(expected).unwrap()
    );
    assert_eq!(
        assets,
        reopened
            .state
            .doc
            .assets
            .images
            .values()
            .map(|a| a.bytes.clone())
            .collect::<Vec<_>>()
    );
    let active = reopened.state.doc.animation_ids()[0];
    reopened.state.session.active_animation = Some(active);
    let c = MoveAnimation::prepare(&reopened.state.doc, active, Direction::Down)
        .unwrap()
        .unwrap();
    reopened.state.dispatch(Box::new(c));
    reopened.execute_file_action(FileAction::Save);
    assert!(!reopened.is_dirty());
    let file = reopened.current_path.clone().unwrap();
    let order = rig::names(&reopened.state.doc);
    let mut twice = AnkhimateApp::default();
    twice.open_initial(Some(file));
    assert_eq!(rig::names(&twice.state.doc), order);
    if let Some(dest) = std::env::var_os("T835_FIXTURE_DIR") {
        let dest = std::path::PathBuf::from(dest);
        assert!(!dest.exists());
        std::fs::create_dir(&dest).unwrap();
        app.save_project_to(&dest.join("Reordered-Path-FFD.ankh"));
        app.state.undo();
        app.save_project_to(&dest.join("Before-Path-FFD.ankh"));
        let mut export = app.state.doc.clone();
        for a in export.animations.values_mut() {
            a.timelines
                .retain(|t| !matches!(t, ankhimate_core::animation::Timeline::PathPosition { .. }));
        }
        let preset = ankhimate_export::presets::builtin()
            .into_iter()
            .find(|p| p.name.contains("Ankhimate"))
            .unwrap();
        let plan = ankhimate_export::run::plan(
            &ankhimate_formats::convert::to_schema(&export.as_project_ref()),
            &export.assets,
            &preset,
        )
        .unwrap();
        ankhimate_export::run::write(&plan, &dest.join("Native-v2-without-Path")).unwrap();
    }
}
#[test]
fn t835_retime_selected_offset_and_cpu_pixels_stay_on_logical_clip() {
    let (d, ids) = rig::fixture();
    let mut app = AnkhimateApp::default();
    app.state.replace_document(d);
    app.state.session.active_animation = Some(ids[0]);
    app.state.set_work_mode(crate::session::WorkMode::Animate);
    let request = ankhimate_render::FrameRequest {
        animation: Some(app.state.doc.animations[ids[0]].name.clone()),
        time: 0.25,
        width: 64,
        height: 64,
        background: [0; 4],
        ..Default::default()
    };
    let png = ankhimate_render::render_frame(&app.state.doc, &request)
        .unwrap()
        .bytes;
    let original = app.state.doc.animations[ids[1]].clone();
    let c = MoveAnimation::prepare(&app.state.doc, ids[0], Direction::Down)
        .unwrap()
        .unwrap();
    app.state.dispatch(Box::new(c));
    assert_eq!(
        ankhimate_render::render_frame(&app.state.doc, &request)
            .unwrap()
            .bytes,
        png
    );
    assert_eq!(app.state.session.active_animation, Some(ids[0]));
    let active = app.state.session.active_animation.unwrap();
    let plan =
        RetimeAnimation::prepare_numeric(active, &app.state.doc.animations[active], 1., 0.25)
            .unwrap();
    app.state.dispatch(Box::new(plan.command));
    assert_eq!(app.state.doc.animations[ids[1]], original);
    app.state.undo();
    let bone=app.state.doc.skeleton.slots.values().find(|s|s.attachment.as_deref()==Some("art")&&app.state.doc.animations[active].timelines.iter().any(|t|matches!(t,ankhimate_core::animation::Timeline::BoneRotate{bone,..}if *bone==s.bone))).unwrap().bone;
    let p = PreparedBoneOffset::prepare(&app.state.doc, active, &[bone], 3.).unwrap();
    app.state
        .dispatch(p.into_command(&app.state.doc).unwrap().unwrap());
    assert_eq!(app.state.doc.animations[ids[1]], original);
    assert_eq!(app.state.session.active_animation, Some(active));
}

#[test]
fn t835_sequence_spritesheet_and_existing_batch_order_contracts_are_preserved() {
    use ankhimate_render::{
        encoding::FrameEncoding,
        framing::FrameOptions,
        sequence::{Sequence, SequenceRequest},
        spritesheet::{Layout, render_page},
    };
    let (mut d, ids) = rig::fixture();
    let request = SequenceRequest {
        animation: d.animations[ids[0]].name.clone(),
        fps: 4,
        width: 16,
        height: 16,
    };
    let pixels = |d: &ankhimate_document::Document| {
        let seq = Sequence::prepare(d, &request, |_, _| true).unwrap();
        let pngs = (0..seq.frame_count())
            .map(|i| seq.render(i).unwrap().bytes)
            .collect::<Vec<_>>();
        let layout = Layout::new(seq.frame_count(), 16, 16, 64, 2).unwrap();
        let pages = (0..layout.pages().len())
            .map(|i| {
                render_page(&seq, &layout, i as u32, |_| true)
                    .unwrap()
                    .bytes
            })
            .collect::<Vec<_>>();
        (pngs, pages)
    };
    let before = pixels(&d);
    let c = MoveAnimation::prepare(&d, ids[1], Direction::Up)
        .unwrap()
        .unwrap();
    ankhimate_document::History::default().push(Box::new(c), &mut d);
    assert_eq!(pixels(&d), before);
    let batch = crate::batch_export::Batch::new(
        &d,
        &[ids[1], ids[0]],
        crate::batch_export::Settings {
            fps: 4,
            width: 16,
            height: 16,
            encoding: FrameEncoding::Png,
            framing: FrameOptions::default(),
            max_page: None,
        },
    )
    .unwrap();
    // Existing batch intentionally enumerates occupied SlotMap indices, not the
    // selected slice or the new author-list order. Do not change its folder contract.
    assert_eq!(
        batch.clips.iter().map(|c| c.id).collect::<Vec<_>>(),
        vec![ids[0], ids[1]]
    );
    assert_eq!(batch.clips[0].source_index, 0);
    assert_eq!(batch.clips[1].source_index, 1);
}
