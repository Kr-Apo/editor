use super::super::t829_probe::animated;
use super::*;
use ankhimate_core::{animation::Axis, skin::Skin};
use ankhimate_document::commands::key_cmds::BoneProperty;
use std::hash::{BuildHasherDefault, Hasher};
#[derive(Default)]
struct Collision;
impl Hasher for Collision {
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, _: &[u8]) {}
}
#[test]
fn t829_membership_small_large_and_duplicate_equivalence() {
    let (_, _, refs) = animated(100, 2, true);
    for n in [0, 1, 2, 8, 16, 64, 65, 100, 1000] {
        let mut selected = refs[..n].to_vec();
        selected.extend(selected.clone());
        let before = selected.clone();
        let m = Membership::new(&selected);
        for key in &refs {
            assert_eq!(m.contains(&selected, key), selected.contains(key));
        }
        assert_eq!(selected, before);
    }
}
#[test]
fn t829_membership_full_identity_survives_forced_hash_collisions() {
    let (mut s, _, mut refs) = animated(5, 2, true);
    let slot = s.doc.skeleton.slots.keys().next().unwrap();
    let skin = s.doc.skeleton.default_skin;
    let other = s.doc.skeleton.skins.insert(Skin::new("other"));
    for owner in [None, Some(skin), Some(other)] {
        for name in ["mesh", "different"] {
            refs.push(KeyRef {
                addr: TimelineAddr::Deform {
                    slot,
                    attachment: name.into(),
                    owner,
                },
                index: 0,
            });
        }
    }
    let mut selected = refs.iter().step_by(2).cloned().collect::<Vec<_>>();
    selected.extend(selected.clone());
    let m = Membership::with_hasher(&selected, BuildHasherDefault::<Collision>::default());
    assert!(m.index.is_some());
    for key in &refs {
        assert_eq!(
            m.contains(&selected, key),
            selected.contains(key),
            "{key:?}"
        );
    }
    for key in refs {
        let missing = KeyRef {
            index: usize::MAX,
            ..key
        };
        assert!(!m.contains(&selected, &missing));
    }
}
#[test]
fn t829_membership_does_not_confuse_axes_properties_bones_or_slots() {
    let (s, _, r) = animated(2, 2, true);
    let bone = s.doc.skeleton.bones.keys().next().unwrap();
    let slot = s.doc.skeleton.slots.keys().next().unwrap();
    let mut selected = vec![r[0].clone(); 65];
    selected.push(KeyRef {
        addr: TimelineAddr::Bone {
            bone,
            property: BoneProperty::Translate,
            axis: Some(Axis::X),
        },
        index: 1,
    });
    let m = Membership::new(&selected);
    for addr in [
        TimelineAddr::Bone {
            bone,
            property: BoneProperty::Translate,
            axis: Some(Axis::Y),
        },
        TimelineAddr::Bone {
            bone,
            property: BoneProperty::Scale,
            axis: Some(Axis::X),
        },
        TimelineAddr::SlotColor { slot },
        TimelineAddr::SlotVisible { slot },
    ] {
        assert!(!m.contains(&selected, &KeyRef { addr, index: 1 }));
    }
}
#[test]
fn t829_membership_is_not_a_validator_or_a_selection_store() {
    let (_, _, mut r) = animated(100, 1, false);
    r[3].index = usize::MAX;
    let index = Membership::new(&r);
    assert!(index.contains(&r, &r[3])); // Exactly like Vec membership; validity belongs to KeySelection.
    let changed = vec![r[0].clone()];
    let fresh = Membership::new(&changed);
    assert!(!fresh.contains(&changed, &r[3]));
    assert_eq!(changed.len(), 1);
}
#[derive(Clone, Default)]
struct CountHash(std::sync::Arc<std::sync::atomic::AtomicUsize>);
struct CountHasher {
    counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    inner: std::collections::hash_map::DefaultHasher,
}
impl std::hash::BuildHasher for CountHash {
    type Hasher = CountHasher;
    fn build_hasher(&self) -> Self::Hasher {
        CountHasher {
            counter: self.0.clone(),
            inner: Default::default(),
        }
    }
}
impl Hasher for CountHasher {
    fn finish(&self) -> u64 {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.finish()
    }
    fn write(&mut self, b: &[u8]) {
        self.inner.write(b)
    }
}
#[test]
fn t829_large_lookup_uses_one_hash_probe_per_query_not_a_linear_scan() {
    let (_, _, refs) = animated(10000, 1, false);
    let h = CountHash::default();
    let m = Membership::with_hasher(&refs, h.clone());
    let counter = h.0;
    counter.store(0, std::sync::atomic::Ordering::Relaxed);
    for key in &refs {
        assert!(m.contains(&refs, key));
    }
    assert_eq!(
        counter.load(std::sync::atomic::Ordering::Relaxed),
        refs.len()
    );
    assert_eq!(m.index.as_ref().unwrap().len(), 10000);
    // The old positive Vec path compares 1+...+10000 full references.
    assert_eq!(refs.len() * (refs.len() + 1) / 2, 50_005_000);
}
