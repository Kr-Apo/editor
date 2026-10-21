use super::*;
use ankhimate_document::commands::animation_order::{Direction, MoveAnimation};
#[allow(
    dead_code,
    reason = "Shared synthetic weighted mesh, FFD and PathPosition fixture"
)]
mod rig {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../document/tests/support/t835_fixture.rs"
    ));
}
#[test]
fn t835_reorder_invalidates_prepared_ffd_paste_not_owned_payload_or_path() {
    let (d, ids) = rig::fixture();
    let a = ids[0];
    let path = d.animations[a].timelines[0].clone();
    let ankhimate_core::animation::Timeline::Deform {
        slot, attachment, ..
    } = &d.animations[a].timelines[1]
    else {
        panic!()
    };
    let refs: Vec<_> = (0..3)
        .map(|index| KeyRef {
            addr: TimelineAddr::Deform {
                slot: *slot,
                attachment: attachment.clone(),
                owner: Some(d.skeleton.default_skin),
            },
            index,
        })
        .collect();
    let mut s = AppState::default();
    s.replace_document(d);
    s.session.active_animation = Some(a);
    s.set_work_mode(crate::session::WorkMode::Animate);
    s.set_playhead(0.);
    key_selection::set_sheet(&mut s, a, refs.clone());
    copy(&mut s, a, &refs).unwrap();
    let Clipboard::Deform(payload) = s.session.clipboard.clone() else {
        panic!()
    };
    let keys = payload.keys().to_vec();
    s.set_playhead(3.);
    key_selection::set_sheet(&mut s, a, refs.clone());
    let mut old = Draft::new(&mut s, a, &refs).unwrap();
    old.ack = true;
    let c = MoveAnimation::prepare(&s.doc, a, Direction::Down)
        .unwrap()
        .unwrap();
    s.dispatch(Box::new(c));
    let before = ankhimate_document::snapshot::snapshot(&s.doc);
    assert!(!old.apply(&mut s));
    assert!(ankhimate_document::snapshot::snapshot(&s.doc) == before);
    assert_eq!(payload.keys(), keys);
    assert_eq!(s.doc.animations[a].timelines[0], path);
    assert_eq!(s.session.active_animation, Some(a));
    assert!(key_selection::read(&mut s, a).is_empty());
    key_selection::set_sheet(&mut s, a, refs.clone());
    let mut fresh = Draft::new(&mut s, a, &refs).unwrap();
    fresh.ack = true;
    assert!(fresh.apply(&mut s));
    assert!(!fresh.apply(&mut s));
    assert_eq!(s.doc.animations[a].timelines[0], path);
    assert_eq!(payload.keys(), keys);
}
