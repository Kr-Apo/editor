#[allow(dead_code, reason = "Shared synthetic author project")]
mod rig {
    include!("../../document/tests/support/path_document_fixture.rs");
}
use ankhimate_core::constraints::Constraint;
use ankhimate_render::{
    Camera, Focus, FocusMode, FrameRequest, render_frame,
    sequence::{Sequence, SequenceRequest},
    spritesheet::{Layout, render_page},
};
#[test]
fn t831_cpu_sequence_pixels_match_static_path_poses_at_known_times() {
    let (doc, _, c) = rig::path_document();
    let before = ankhimate_document::snapshot::snapshot(&doc);
    let request = SequenceRequest {
        animation: "progress".into(),
        fps: 4,
        width: 128,
        height: 128,
    };
    let seq = Sequence::prepare(&doc, &request, |_, _| true).unwrap();
    assert_eq!(seq.frame_count(), 4);
    let camera = seq.camera();
    let mut images = Vec::new();
    for i in 0..4 {
        let rgba = seq.render_rgba(i).unwrap();
        let mut reference = doc.clone();
        if let Constraint::Path(p) = &mut reference.skeleton.constraints[c] {
            p.position = seq.time(i).unwrap() * 0.5;
        }
        let frame = render_frame(
            &reference,
            &FrameRequest {
                animation: None,
                background: [0; 4],
                width: 128,
                height: 128,
                camera: Camera {
                    center: Some(camera.center.to_array()),
                    zoom: Some(camera.zoom),
                    ..Default::default()
                },
                focus: Some(Focus {
                    mode: FocusMode::ArtOnly,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();
        let expected = image::load_from_memory(&frame.bytes).unwrap().into_rgba8();
        assert!(
            rgba == expected,
            "frame {i}: {} differing pixels",
            rgba.pixels()
                .zip(expected.pixels())
                .filter(|(a, b)| a != b)
                .count()
        );
        assert!(rgba.pixels().any(|p| p[3] > 0));
        images.push(rgba);
    }
    assert_ne!(images[0], images[3]);
    assert!(ankhimate_document::snapshot::snapshot(&doc) == before);
}
#[test]
fn t831_spritesheet_reconstruction_matches_path_sequence_pixels() {
    let (doc, _, _) = rig::path_document();
    let req = SequenceRequest {
        animation: "progress".into(),
        fps: 4,
        width: 64,
        height: 64,
    };
    let seq = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
    let layout = Layout::new(4, 64, 64, 256, 2).unwrap();
    for page in layout.pages() {
        let png = render_page(&seq, &layout, page.index, |_| true).unwrap();
        let rgba = image::load_from_memory(&png.bytes).unwrap().into_rgba8();
        for frame in layout.frames().iter().filter(|f| f.page == page.index) {
            let r = frame.rect;
            let cropped = image::imageops::crop_imm(&rgba, r.x, r.y, r.w, r.h).to_image();
            assert_eq!(cropped, seq.render_rgba(frame.index).unwrap());
        }
    }
}
#[test]
fn t831_rendered_invalid_path_refuses_before_render_and_legacy_exclusions_remain() {
    let (mut doc, _, c) = rig::path_document();
    let request = SequenceRequest {
        animation: "progress".into(),
        fps: 4,
        width: 64,
        height: 64,
    };
    doc.skeleton.remove_constraint(c);
    assert!(request.frame_count(&doc).is_err());
    let (doc, _, _) = rig::path_document();
    let seq = Sequence::prepare(&doc, &request, |_, _| false);
    assert!(seq.is_err());
}
