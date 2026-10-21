use super::*;
pub fn reference_capture(doc: &Document, anim: AnimationId, refs: &[KeyRef]) -> Result<KeyClip, String> {
        if refs.is_empty() {
            return Err("No selected keys".into());
        }
        let animation = doc
            .animations
            .get(anim)
            .ok_or("Source animation no longer exists")?;
        let mut keys = Vec::new();
        let mut seen = Vec::new();
        for r in refs {
            if seen.contains(r) {
                continue;
            }
            seen.push(r.clone());
            let timelines: Vec<_> = animation
                .timelines
                .iter()
                .filter(|t| r.addr.matches_timeline(t))
                .collect();
            if timelines.len() != 1 {
                return Err("Source track is missing or ambiguous".into());
            }
            let track = NamedTrack::capture(doc, &r.addr)?;
            let key = captured_key(timelines[0], r.index)?;
            validate(&track.kind, &key)?;
            keys.push(CopiedKey { track, key });
        }
        Ok(KeyClip { keys })
    }
