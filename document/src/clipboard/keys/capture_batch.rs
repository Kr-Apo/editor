//! Operation-local acceleration of the same immutable native capture contract.
use super::*;
use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    hash::{BuildHasher, Hash, Hasher},
};

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Counts {
    pub timeline_indexed: usize,
    pub names_indexed: usize,
    pub track_resolutions: usize,
    pub candidates_checked: usize,
    pub duplicate_comparisons: usize,
    pub validated_keys: usize,
}
#[cfg(test)]
thread_local! {static COUNTS:std::cell::Cell<Counts>=std::cell::Cell::new(Counts::default());}
macro_rules! count {
    ($field:ident) => {
        #[cfg(test)]
        COUNTS.with(|c| {
            let mut n = c.get();
            n.$field += 1;
            c.set(n);
        });
    };
}
#[cfg(test)]
pub(super) fn measured<T>(f: impl FnOnce() -> T) -> (T, Counts) {
    COUNTS.with(|c| c.set(Counts::default()));
    let value = f();
    (value, COUNTS.with(|c| c.get()))
}

// Optional test-only phase clocks; no clocks or per-phase state in the product.
#[cfg(test)]
thread_local! {static PHASES:std::cell::Cell<[f64;5]>=const{std::cell::Cell::new([0.;5])};}
macro_rules! begin_phase {
    ($start:ident) => {
        #[cfg(test)]
        let $start = PROFILE.then(std::time::Instant::now);
    };
}
macro_rules! end_phase {
    ($slot:expr,$start:ident) => {
        #[cfg(test)]
        if let Some(t) = $start {
            PHASES.with(|p| {
                let mut phases = p.get();
                phases[$slot] += t.elapsed().as_secs_f64() * 1000.;
                p.set(phases);
            });
        }
    };
}
#[cfg(test)]
pub(super) fn profiled(
    doc: &Document,
    anim: AnimationId,
    refs: &[KeyRef],
) -> (Result<KeyClip, String>, [f64; 5]) {
    PHASES.with(|p| p.set([0.; 5]));
    let value = capture_impl::<_, true>(
        doc,
        anim,
        refs,
        std::collections::hash_map::RandomState::new(),
    );
    (value, PHASES.with(|p| p.get()))
}

// Hash chooses a bucket; complete native KeyRef equality decides duplication.
struct Selected<'a>(&'a KeyRef);
impl PartialEq for Selected<'_> {
    fn eq(&self, other: &Self) -> bool {
        count!(duplicate_comparisons);
        self.0 == other.0
    }
}
impl Eq for Selected<'_> {}
impl Hash for Selected<'_> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.addr.hash(h);
        self.0.index.hash(h);
    }
}

// Preserve native matching BEFORE validating malformed axis/owner addresses.
fn matching_address(addr: &TimelineAddr) -> TimelineAddr {
    let mut key = addr.clone();
    match &mut key {
        TimelineAddr::Bone {
            property: BoneProperty::Rotate,
            axis,
            ..
        } => *axis = None,
        TimelineAddr::Deform { owner, .. } => *owner = None,
        _ => {}
    }
    key
}
fn indexed_address(t: &Timeline) -> Option<TimelineAddr> {
    address(t).ok().or_else(|| {
        if let Timeline::Deform {
            slot, attachment, ..
        } = t
        {
            Some(TimelineAddr::Deform {
                slot: *slot,
                attachment: attachment.clone(),
                owner: None,
            })
        } else {
            None
        }
    })
}
pub(super) fn capture(
    doc: &Document,
    anim: AnimationId,
    refs: &[KeyRef],
) -> Result<KeyClip, String> {
    capture_with(
        doc,
        anim,
        refs,
        std::collections::hash_map::RandomState::new(),
    )
}
pub(super) fn capture_with<S: BuildHasher + Clone>(
    doc: &Document,
    anim: AnimationId,
    refs: &[KeyRef],
    hash: S,
) -> Result<KeyClip, String> {
    capture_impl::<S, false>(doc, anim, refs, hash)
}
fn capture_impl<S: BuildHasher + Clone, const PROFILE: bool>(
    doc: &Document,
    anim: AnimationId,
    refs: &[KeyRef],
    hash: S,
) -> Result<KeyClip, String> {
    if refs.is_empty() {
        return Err("No selected keys".into());
    }
    let animation = doc
        .animations
        .get(anim)
        .ok_or("Source animation no longer exists")?;
    begin_phase!(index_start);
    let mut timelines: HashMap<TimelineAddr, Vec<&Timeline>, S> =
        HashMap::with_hasher(hash.clone());
    for t in &animation.timelines {
        count!(timeline_indexed);
        if let Some(addr) = indexed_address(t) {
            timelines.entry(addr).or_default().push(t);
        }
    }
    let mut bones: HashMap<&str, usize, S> = HashMap::with_hasher(hash.clone());
    let mut slots: HashMap<&str, usize, S> = HashMap::with_hasher(hash.clone());
    for b in doc.skeleton.bones.values() {
        count!(names_indexed);
        *bones.entry(b.name.as_str()).or_default() += 1;
    }
    for s in doc.skeleton.slots.values() {
        count!(names_indexed);
        *slots.entry(s.name.as_str()).or_default() += 1;
    }
    end_phase!(0, index_start);
    let mut seen = HashSet::with_hasher(hash.clone());
    let mut resolved: HashMap<&TimelineAddr, (&Timeline, NamedTrack), S> =
        HashMap::with_hasher(hash);
    let mut keys = Vec::new();
    for r in refs {
        begin_phase!(dedup_start);
        let duplicate = !seen.insert(Selected(r));
        end_phase!(1, dedup_start);
        if duplicate {
            continue;
        }
        begin_phase!(resolve_start);
        let (timeline, track) = match resolved.entry(&r.addr) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                count!(track_resolutions);
                let mut found = None;
                for t in timelines
                    .get(&matching_address(&r.addr))
                    .into_iter()
                    .flatten()
                {
                    count!(candidates_checked);
                    if r.addr.matches_timeline(t) {
                        if found.is_some() {
                            return Err("Source track is missing or ambiguous".into());
                        }
                        found = Some(*t);
                    }
                }
                let t = found.ok_or("Source track is missing or ambiguous")?;
                let named = NamedTrack::capture_using(doc, &r.addr, |named| {
                    let (counts, kind) = if matches!(named.kind, TrackKind::Bone(..)) {
                        (&bones, "Bone")
                    } else {
                        (&slots, "Slot")
                    };
                    if counts.get(named.name.as_str()) == Some(&1) {
                        Ok(())
                    } else {
                        Err(format!("{kind} '{}' is missing or ambiguous", named.name))
                    }
                })?;
                e.insert((t, named))
            }
        };
        end_phase!(2, resolve_start);
        begin_phase!(key_start);
        let key = captured_key(timeline, r.index)?;
        validate(&track.kind, &key)?;
        count!(validated_keys);
        end_phase!(3, key_start);
        begin_phase!(append_start);
        keys.push(CopiedKey {
            track: track.clone(),
            key,
        });
        end_phase!(4, append_start);
    }
    Ok(KeyClip { keys })
}
