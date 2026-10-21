use super::*;
use crate::snapshot::snapshot;
mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/support/t827_fixture.rs"
    ));
}
mod reference {
    include!("t827_reference.rs");
}
use reference::reference_capture;
fn same(d: &Document, a: AnimationId, r: &[KeyRef]) {
    assert_eq!(KeyClip::capture(d, a, r), reference_capture(d, a, r));
}
#[test]
fn t827_exact_capture_matches_original_cases() {
    for (name, d, a, r) in fixture::cases() {
        assert_eq!(
            KeyClip::capture(&d, a, &r),
            reference_capture(&d, a, &r),
            "{name}"
        );
    }
}
#[test]
fn t827_ten_thousand_duplicates_keep_first_occurrence_order() {
    let (d, a, r) = fixture::fixture(5000, 1, false);
    let input = r.iter().rev().chain(r.iter()).cloned().collect::<Vec<_>>();
    same(&d, a, &input);
    assert_eq!(KeyClip::capture(&d, a, &input).unwrap().keys.len(), 5000);
}
#[test]
fn t827_large_capture_resolves_once_and_checks_each_key() {
    let (d, a, r) = fixture::fixture(10000, 1, false);
    let (out, c) = capture_batch::measured(|| {
        capture_batch::capture_with(
            &d,
            a,
            &r,
            std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default(),
        )
    });
    assert_eq!(out.unwrap().keys.len(), 10000);
    assert_eq!(c.timeline_indexed, 1);
    assert_eq!(c.names_indexed, 1);
    assert_eq!(c.track_resolutions, 1);
    assert_eq!(c.candidates_checked, 1);
    assert_eq!(c.validated_keys, 10000);
    assert!(c.duplicate_comparisons < 200000, "{c:?}");
}
#[test]
fn t827_many_small_tracks_are_indexed_once() {
    let (d, a, r) = fixture::fixture(1, 1000, false);
    let (out, c) = capture_batch::measured(|| {
        capture_batch::capture_with(
            &d,
            a,
            &r,
            std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default(),
        )
    });
    assert_eq!(out.unwrap().keys.len(), 1000);
    assert_eq!(c.timeline_indexed, 1000);
    assert_eq!(c.names_indexed, 1000);
    assert_eq!(c.track_resolutions, 1000);
    assert_eq!(c.candidates_checked, 1000);
    assert_eq!(c.validated_keys, 1000);
}
#[test]
fn t827_ten_thousand_multi_channel_refs_keep_types_and_values() {
    let (d, a, r) = fixture::fixture(1250, 1, true);
    let (out, c) = capture_batch::measured(|| {
        capture_batch::capture_with(
            &d,
            a,
            &r,
            std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default(),
        )
    });
    assert_eq!(out.unwrap(), reference_capture(&d, a, &r).unwrap());
    assert_eq!(c.track_resolutions, 8);
    assert_eq!(c.candidates_checked, 8);
    assert_eq!(c.validated_keys, 10000);
}
#[derive(Clone, Default)]
struct Collision;
struct Zero;
impl std::hash::Hasher for Zero {
    fn write(&mut self, _: &[u8]) {}
    fn finish(&self) -> u64 {
        0
    }
}
impl std::hash::BuildHasher for Collision {
    type Hasher = Zero;
    fn build_hasher(&self) -> Zero {
        Zero
    }
}
#[test]
fn t827_hash_collisions_use_full_typed_equality() {
    for (_, d, a, r) in fixture::cases()
        .into_iter()
        .filter(|(_, _, _, r)| r.len() < 200)
    {
        assert_eq!(
            capture_batch::capture_with(&d, a, &r, Collision),
            reference_capture(&d, a, &r)
        );
    }
}
#[test]
fn t827_small_batch_boundary_has_identical_errors_and_order() {
    for n in [1, 10, 16, 17, 32] {
        let (mut d, a, mut r) = fixture::fixture(n, 1, false);
        r.reverse();
        same(&d, a, &r);
        r[n / 2].index = n + 1;
        same(&d, a, &r);
        let duplicate = d.animations[a].timelines[0].clone();
        d.animations[a].timelines.push(duplicate);
        same(&d, a, &r);
    }
}
#[test]
fn t827_unselected_invalid_key_does_not_change_capture_policy() {
    let (mut d, a, mut r) = fixture::fixture(100, 1, false);
    if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
        keys[99].time = f32::NAN;
    }
    r.truncate(20);
    same(&d, a, &r);
    assert!(KeyClip::capture(&d, a, &r).is_ok());
}
#[test]
fn t827_duplicate_source_names_and_missing_ids_still_refuse() {
    for mode in [6, 7, 8, 14] {
        let (_, d, a, r) = fixture::cases()
            .into_iter()
            .find(|(n, ..)| n == &format!("case_{mode}"))
            .unwrap();
        same(&d, a, &r);
        assert!(KeyClip::capture(&d, a, &r).is_err());
    }
}
#[test]
fn t827_first_invalid_key_diagnostic_precedes_later_track_error() {
    let (mut d, a, mut r) = fixture::fixture(30, 2, false);
    r[12].index = 999;
    let second = d.animations[a].timelines[1].clone();
    d.animations[a].timelines.push(second);
    same(&d, a, &r);
    assert_eq!(
        KeyClip::capture(&d, a, &r).unwrap_err(),
        "Missing key index or unsupported timeline"
    );
    r.rotate_left(30);
    same(&d, a, &r);
    assert_eq!(
        KeyClip::capture(&d, a, &r).unwrap_err(),
        "Source track is missing or ambiguous"
    );
}
#[test]
fn t827_general_ffd_refusal_preserves_owner_and_match_diagnostics() {
    let (mut d, a, r) = fixture::fixture(20, 1, true);
    let slot = d.skeleton.slots.keys().next().unwrap();
    d.animations[a].timelines.push(Timeline::Deform {
        slot,
        attachment: "mesh".into(),
        keys: vec![Key::linear(0., vec![])],
    });
    for owner in [None, Some(d.skeleton.default_skin)] {
        let mut refs = r.clone();
        refs.insert(
            3,
            KeyRef {
                addr: TimelineAddr::Deform {
                    slot,
                    attachment: "mesh".into(),
                    owner,
                },
                index: 0,
            },
        );
        same(&d, a, &refs);
        assert_eq!(
            KeyClip::capture(&d, a, &refs).unwrap_err(),
            "FFD vectors are excluded from the general key clipboard"
        );
    }
}
#[test]
fn t827_capture_never_mutates_document_or_assets() {
    let (d, a, r) = fixture::fixture(1000, 2, true);
    let before = snapshot(&d);
    same(&d, a, &r);
    assert!(snapshot(&d) == before);
    let mut bad = r;
    bad.last_mut().unwrap().index = usize::MAX;
    assert!(KeyClip::capture(&d, a, &bad).is_err());
    assert!(snapshot(&d) == before);
}
#[test]
fn t827_many_clips_resolve_only_requested_animation() {
    let (mut d, a, r) = fixture::fixture(30, 1, false);
    let template = d.animations[a].clone();
    for i in 0..2000 {
        let mut clip = template.clone();
        clip.name = format!("other_{i}");
        d.animations.insert(clip);
    }
    same(&d, a, &r);
    d.animations.remove(a);
    same(&d, a, &r);
    assert_eq!(
        KeyClip::capture(&d, a, &r).unwrap_err(),
        "Source animation no longer exists"
    );
}
#[test]
fn t827_missing_attachment_is_checked_by_destination_not_invented_at_copy() {
    let (mut d, a, r) = fixture::fixture(20, 1, true);
    let slot = d.skeleton.slots.keys().next().unwrap();
    d.animations[a].timelines.push(Timeline::SlotAttachment {
        slot,
        keys: (0..20)
            .map(|i| Key::stepped(i as f32, Some("missing".into())))
            .collect(),
    });
    let refs = (0..20)
        .map(|index| KeyRef {
            addr: TimelineAddr::SlotAttachment { slot },
            index,
        })
        .collect::<Vec<_>>();
    same(&d, a, &refs);
    let copied = KeyClip::capture(&d, a, &refs).unwrap();
    assert!(copied.resolve(&d, a, 0., &[]).is_err());
    assert_eq!(r.len(), 160);
}
#[test]
fn t827_angle_boundaries_and_bezier_payload_are_exact() {
    let (mut d, a, r) = fixture::fixture(30, 1, false);
    if let Timeline::BoneRotate { keys, .. } = &mut d.animations[a].timelines[0] {
        for (i, k) in keys.iter_mut().enumerate() {
            k.value = if i % 2 == 0 { -180. } else { 180. };
        }
    }
    same(&d, a, &r);
}

#[test]
fn t827_optional_native_measurement_and_external_oracle() {
    let Some(path) = std::env::var_os("T827_RESULT") else {
        return;
    };
    let cases=fixture::cases().into_iter().map(|(name,d,a,r)|serde_json::json!({"name":name,"refs":format!("{r:?}"),"result":format!("{:?}",KeyClip::capture(&d,a,&r))})).collect::<Vec<_>>();
    let mut measures = vec![];
    for n in [1, 10, 100, 1000, 2000, 5000, 10000, 20000] {
        let t = std::time::Instant::now();
        let (d, a, r) = fixture::fixture(n, 1, false);
        let construct = t.elapsed().as_secs_f64() * 1000.;
        std::hint::black_box(KeyClip::capture(&d, a, &r).unwrap());
        let mut times = vec![];
        for _ in 0..3 {
            let t = std::time::Instant::now();
            std::hint::black_box(KeyClip::capture(&d, a, &r).unwrap());
            times.push(t.elapsed().as_secs_f64() * 1000.);
        }
        let (_, counts) = capture_batch::measured(|| KeyClip::capture(&d, a, &r));
        let (profiled, phases) = capture_batch::profiled(&d, a, &r);
        assert_eq!(profiled.unwrap(), KeyClip::capture(&d, a, &r).unwrap());
        measures.push(serde_json::json!({"keys":n,"constructMs":construct,"captureMs":times,"counts":format!("{counts:?}"),"phaseMs":{"indexConstruction":phases[0],"dedup":phases[1],"cachedTrackResolution":phases[2],"keyCloneAndValidation":phases[3],"resultAppend":phases[4]}}));
    }
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&serde_json::json!({"cases":cases,"measures":measures})).unwrap(),
    )
    .unwrap();
}
