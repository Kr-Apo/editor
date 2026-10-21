use super::*;
use ankhimate_document::{commands::document_cmds::SetProjectFps, snapshot::snapshot};
#[test]
fn t828_onion_frames_follow_native_fps_undo_redo_without_document_edits() {
    let mut s = AppState::default();
    let before = snapshot(&s.doc);
    let times = |fps| frame_onion_times(0.5, 1., fps, 2);
    let old = times(s.doc.meta.fps);
    s.dispatch(Box::new(SetProjectFps::new(&s.doc, 60).unwrap().unwrap()));
    let new = times(s.doc.meta.fps);
    assert_ne!(old, new);
    assert_eq!(
        new,
        vec![
            (0.5 - 1. / 60., OnionSide::Past, 1),
            (0.5 + 1. / 60., OnionSide::Future, 1),
            (0.5 - 2. / 60., OnionSide::Past, 2),
            (0.5 + 2. / 60., OnionSide::Future, 2)
        ]
    );
    let after = snapshot(&s.doc);
    for _ in 0..3 {
        s.undo();
        assert_eq!(times(s.doc.meta.fps), old);
        assert!(snapshot(&s.doc) == before);
        s.redo();
        assert_eq!(times(s.doc.meta.fps), new);
        assert!(snapshot(&s.doc) == after);
    }
    assert!(frame_onion_times(0., 0., s.doc.meta.fps, 5).is_empty());
}
