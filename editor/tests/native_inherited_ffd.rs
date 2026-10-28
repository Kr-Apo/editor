mod support {
    include!("support/linked_frog.rs");
}
use ankhimate_core::attachment::Attachment;
#[test]
fn a10_red_a9_diagnostic_and_cpu_inheritance() {
    let mut d = support::fixture();
    for (t, expected) in [(0.75, 14.059692), (0.8, 3.5680866)] {
        let p = support::pose(&d, t);
        let (slot, s) = d
            .skeleton
            .slots
            .iter()
            .find(|(_, s)| s.name == "body")
            .unwrap();
        let Some(Attachment::Mesh(displayed)) =
            d.skeleton
                .resolve_posed(&[d.skeleton.default_skin], &p, slot)
        else {
            panic!()
        };
        let source = d
            .skeleton
            .resolve_linked_mesh(&[d.skeleton.default_skin], displayed);
        assert!(!p.deforms.contains_key(&(slot, "linked_royal".into())));
        let offsets = &p.deforms[&(slot, "body".into())];
        let delta = (0..source.setup_vertices.len())
            .map(|i| {
                let a = source.skin_vertex_with_deform(i, &[], &p, &p.world(s.bone));
                let b = source.skin_vertex_with_deform(
                    i,
                    &offsets[source.influence_range(i)],
                    &p,
                    &p.world(s.bone),
                );
                (a - b).abs().max_element()
            })
            .fold(0., f32::max);
        println!("A9 displayed lookup missing t={t}: {delta} units");
        assert!((delta - expected).abs() < 0.0001);
    }
    let req = ankhimate_render::FrameRequest {
        animation: Some("linked_frog".into()),
        time: 0.75,
        width: 640,
        height: 480,
        background: [0; 4],
        camera: ankhimate_render::Camera {
            center: Some([0., 0.]),
            zoom: Some(3.),
            ..Default::default()
        },
        ..Default::default()
    };
    let p = support::pose(&d, 0.75);
    let (id, _) = d
        .skeleton
        .slots
        .iter()
        .find(|(_, s)| s.name == "body")
        .unwrap();
    let resolved = d
        .skeleton
        .resolve_mesh_pose(&[d.skeleton.default_skin], &p, id, "linked_royal")
        .unwrap();
    println!(
        "resolved {:?}, offsets {:?}, positions {:?}, visible {:?}, color {:?}",
        resolved.deform_owner,
        resolved.offsets,
        resolved.vertices(&p),
        p.slot_visible.get(id),
        p.slot_colors.get(id)
    );
    let (independent, _) = support::independent(&d, &p, id, "body", "body", 0.75);
    for (actual, expected) in resolved.vertices(&p).unwrap().iter().zip(independent) {
        assert!(
            (f64::from(actual.x) - expected[0]).abs() < 0.01
                && (f64::from(actual.y) - expected[1]).abs() < 0.01
        );
    }
    let inherited = ankhimate_render::render_frame(&d, &req).unwrap();
    support::flag(&mut d, false);
    let p = support::pose(&d, 0.75);
    let resolved = d
        .skeleton
        .resolve_mesh_pose(&[d.skeleton.default_skin], &p, id, "linked_royal")
        .unwrap();
    println!(
        "plain owner {:?}, offsets {:?}, positions {:?}",
        resolved.deform_owner,
        resolved.offsets,
        resolved.vertices(&p)
    );
    let plain = ankhimate_render::render_frame(&d, &req).unwrap();
    if let Ok(out) = std::env::var("A10_DEBUG") {
        std::fs::write(format!("{out}/true.png"), &inherited.bytes).unwrap();
        std::fs::write(format!("{out}/false.png"), &plain.bytes).unwrap();
    }
    assert!(
        inherited.bytes != plain.bytes,
        "inherit flag must change CPU pixels for the source FFD"
    );
}
