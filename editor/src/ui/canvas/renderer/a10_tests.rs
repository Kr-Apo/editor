use super::*;
mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/support/linked_frog.rs"
    ));
}
#[test]
fn a10_viewport_sprite_uses_independent_source_ffd_and_texture() {
    let mut s = AppState::default();
    s.replace_document(fixture::fixture());
    let slot = s
        .doc
        .skeleton
        .slots
        .iter()
        .find(|(_, v)| v.name == "body")
        .unwrap()
        .0;
    for t in [0., 0.25, 0.5, 0.75, 0.8, 1.1, 1.5] {
        s.pose = fixture::pose(&s.doc, t);
        prepare_textures(&mut s);
        let name = s.pose.attachment_name(&s.doc.skeleton, slot).unwrap();
        let (source, owner) = match name {
            "linked_royal" => ("body", "body"),
            "linked_plain" => ("body", "linked_plain"),
            _ => (name, name),
        };
        let (points, _) = fixture::independent(&s.doc, &s.pose, slot, source, owner, t);
        let draw = sprite_for_slot(&s, &s.pose, slot, None).unwrap();
        assert_eq!(draw.vertices.len(), points.len());
        for (v, p) in draw.vertices.iter().zip(points) {
            assert!(
                (f64::from(v.position[0]) - p[0]).abs() < 0.01
                    && (f64::from(v.position[1]) - p[1]).abs() < 0.01
            );
        }
        let Attachment::Mesh(m) = s
            .doc
            .skeleton
            .resolve_posed(&s.session.skin_stack(), &s.pose, slot)
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(
            draw.key,
            s.session.texture_keys[s.doc.assets.by_name(&m.texture).unwrap()]
        );
    }
    s.pose = fixture::pose(&s.doc, 0.75);
    s.pose.slot_visible.insert(slot, false);
    assert!(sprite_for_slot(&s, &s.pose, slot, None).is_none());
    s.pose.slot_visible.insert(slot, true);
    s.pose.slot_attachments.insert(slot, None);
    assert!(sprite_for_slot(&s, &s.pose, slot, None).is_none());
}
#[test]
fn a10_viewport_refuses_invalid_link_and_reports_during_preparation() {
    let mut s = AppState::default();
    s.replace_document(fixture::fixture());
    s.pose = fixture::pose(&s.doc, 0.75);
    let slot = s
        .doc
        .skeleton
        .slots
        .iter()
        .find(|(_, v)| v.name == "body")
        .unwrap()
        .0;
    let skin = s.doc.skeleton.default_skin;
    let Attachment::Mesh(m) = s.doc.skeleton.skins[skin]
        .entries
        .get_mut(&(slot, "linked_royal".into()))
        .unwrap()
    else {
        panic!()
    };
    m.linked.as_mut().unwrap().attachment = "missing".into();
    prepare_textures(&mut s);
    assert!(sprite_for_slot(&s, &s.pose, slot, None).is_none());
    assert!(
        s.session
            .status
            .as_deref()
            .is_some_and(|s| s.contains("source"))
    );
    fixture::flag(&mut s.doc, false);
}
