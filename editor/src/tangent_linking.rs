//! Session policy for native tangent edits; no author metadata.
use crate::{app_state::AppState, key_selection};
use ankhimate_core::{
    animation::{Interp, Timeline},
    ids::AnimationId,
};
pub use ankhimate_document::tangent_linking::{Mode, Side};
use ankhimate_document::{
    clipboard::KeyClip,
    commands::key_cmds::{KeyRef, SetInterp, TimelineAddr},
    tangent_linking::Junction,
};
use eframe::egui;
#[derive(Clone, PartialEq)]
pub struct Capture {
    identity: u64,
    revision: u64,
    anim: AnimationId,
    selection: Vec<KeyRef>,
    refs: Vec<KeyRef>,
    data: KeyClip,
}
impl Capture {
    pub fn new(s: &mut AppState, anim: AnimationId, addr: &TimelineAddr) -> Result<Self, String> {
        let selection = key_selection::read(s, anim);
        if selection.is_empty() {
            return Err("Select one central Bezier key".into());
        }
        let a = s.doc.animations.get(anim).ok_or("Animation is absent")?;
        let t = a
            .timelines
            .iter()
            .find(|t| addr.matches_timeline(t))
            .ok_or("Track is absent")?;
        let n = match t {
            Timeline::BoneTranslate { keys, .. }
            | Timeline::BoneRotate { keys, .. }
            | Timeline::BoneScale { keys, .. }
            | Timeline::BoneShear { keys, .. } => keys.len(),
            Timeline::SlotColor { keys, .. } => keys.len(),
            _ => return Err("Read-only graph channel".into()),
        };
        let refs = (0..n)
            .map(|index| KeyRef {
                addr: addr.clone(),
                index,
            })
            .collect::<Vec<_>>();
        let data = KeyClip::capture(&s.doc, anim, &refs)?;
        Ok(Self {
            identity: s.diagnostics.document_identity(),
            revision: s.revision,
            anim,
            selection,
            refs,
            data,
        })
    }
    pub fn matches(&self, s: &mut AppState) -> bool {
        self.identity == s.diagnostics.document_identity()
            && self.revision == s.revision
            && s.session.active_animation == Some(self.anim)
            && key_selection::read(s, self.anim) == self.selection
            && KeyClip::capture(&s.doc, self.anim, &self.refs)
                .ok()
                .as_ref()
                == Some(&self.data)
    }
}
#[derive(Default)]
pub struct LinkState {
    mode: Mode,
    scope: Option<Capture>,
}
pub fn mode(s: &mut AppState) -> Mode {
    let scope = s.session.tangent_linking.scope.clone();
    if scope.as_ref().is_some_and(|c| !c.matches(s)) {
        s.session.tangent_linking = Default::default();
    }
    s.session.tangent_linking.mode
}
pub fn selected(s: &mut AppState) -> Result<Junction, String> {
    let anim = s
        .session
        .active_animation
        .ok_or("Select an animation and one central Bezier key")?;
    let refs = key_selection::read(s, anim);
    if refs.len() != 1 {
        return Err("Linked modes require exactly one central key; multi-key Broken editing remains available".into());
    }
    Junction::read(&s.doc, anim, refs[0].addr.clone(), refs[0].index)
}
pub fn choose(s: &mut AppState, value: Mode) -> Result<(), String> {
    s.session.tangent_linking = Default::default();
    if value == Mode::Broken {
        return Ok(());
    }
    let j = selected(s)?;
    editable(s, &j.addr)?;
    let scope = Capture::new(s, j.anim, &j.addr)?;
    s.session.tangent_linking = LinkState {
        mode: value,
        scope: Some(scope),
    };
    Ok(())
}
fn rebase(s: &mut AppState, m: Mode, anim: AnimationId, refs: Vec<KeyRef>) {
    key_selection::set(s, anim, refs);
    if m != Mode::Broken
        && let Err(e) = choose(s, m)
    {
        s.session.set_status(e);
    }
}
pub fn editable(s: &AppState, addr: &TimelineAddr) -> Result<(), String> {
    if !s.session.is_animating() {
        return Err("Switch to Animate mode to edit tangents".into());
    }
    let bone = match addr {
        TimelineAddr::Bone { bone, .. } => *bone,
        TimelineAddr::SlotColor { slot } => {
            s.doc
                .skeleton
                .slots
                .get(*slot)
                .ok_or("Slot is absent")?
                .bone
        }
        _ => return Err("Read-only graph channel".into()),
    };
    if !s.doc.skeleton.bones.contains_key(bone) || s.session.is_bone_locked(bone) {
        return Err("Track target is missing or locked".into());
    }
    Ok(())
}
pub fn mode_ui(ui: &mut egui::Ui, s: &mut AppState) -> [egui::Rect; 3] {
    let current = mode(s);
    let available = selected(s).and_then(|j| editable(s, &j.addr));
    let mut rects = [egui::Rect::NOTHING; 3];
    ui.horizontal(|ui|{
        ui.label("Tangent link:");
        for (i,(m,text)) in [(Mode::Broken,"Broken"),(Mode::Aligned,"Aligned"),(Mode::Mirrored,"Mirrored")].into_iter().enumerate(){
            let r=ui.add_enabled(m==Mode::Broken || available.is_ok(),egui::Button::selectable(current==m,text));rects[i]=r.rect;
            let r=r.on_hover_text(if m==Mode::Broken{"Independent segment handles; session mode"}else{"Single central key; seconds/native-value metric. Applies on the next edit; mode is not saved."});
            if r.clicked() && let Err(e)=choose(s,m){s.session.set_status(e);}
        }
    });
    if let Err(e) = available {
        ui.label(e);
    }
    rects
}
#[derive(Clone)]
pub struct Numeric {
    pub fields: [String; 4],
    pub side: Side,
    pub mode: Mode,
    capture: Capture,
    pub junction: Junction,
}
impl Numeric {
    pub fn new(s: &mut AppState, side: Side) -> Result<Self, String> {
        let mode = mode(s);
        if mode == Mode::Broken {
            return Err("Choose a linked mode".into());
        }
        let junction = selected(s)?;
        let capture = Capture::new(s, junction.anim, &junction.addr)?;
        let i = junction.handle(Side::Incoming);
        let o = junction.handle(Side::Outgoing);
        Ok(Self {
            fields: [i.x, i.y, o.x, o.y].map(|v| v.to_string()),
            side,
            mode,
            capture,
            junction,
        })
    }
    pub fn valid(&self, s: &mut AppState) -> bool {
        self.capture.matches(s) && mode(s) == self.mode
    }
    pub fn pair(&self) -> Result<[Interp; 2], String> {
        let n = if self.side == Side::Incoming { 0 } else { 2 };
        let parse = |i: usize| {
            self.fields[i]
                .trim()
                .parse::<f32>()
                .map_err(|_| "Enter finite handle numbers".to_string())
        };
        self.junction
            .solve(self.mode, self.side, glam::vec2(parse(n)?, parse(n + 1)?))
    }
    pub fn apply(&self, s: &mut AppState) -> Result<(), String> {
        editable(s, &self.junction.addr)?;
        if !self.valid(s) {
            return Err("Tangent draft is stale; select again".into());
        }
        let pair = self.pair()?;
        if let Some(cmd) = self.junction.command(&s.doc, pair, None)? {
            if !s.dispatch(Box::new(cmd)) {
                return Err("Tangent command refused".into());
            }
            rebase(
                s,
                self.mode,
                self.junction.anim,
                self.capture.selection.clone(),
            );
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct Gesture {
    capture: Capture,
    baseline: Option<Junction>,
    mode: Mode,
    side: Side,
    end: KeyRef,
    token: u64,
    valid: bool,
}
impl Gesture {
    pub fn begin(
        s: &mut AppState,
        anim: AnimationId,
        end: KeyRef,
        side: Side,
        token: u64,
    ) -> Result<Self, String> {
        editable(s, &end.addr)?;
        let mode = mode(s);
        let baseline = if mode == Mode::Broken {
            None
        } else {
            let j = selected(s)?;
            let expected = j.index + usize::from(side == Side::Outgoing);
            if end.addr != j.addr || end.index != expected {
                return Err("Drag only the selected junction's incoming or outgoing handle".into());
            }
            Some(j)
        };
        let capture = Capture::new(s, anim, &end.addr)?;
        Ok(Self {
            capture,
            baseline,
            mode,
            side,
            end,
            token,
            valid: true,
        })
    }
    pub fn update(&mut self, s: &mut AppState, moved: glam::Vec2) -> Result<(), String> {
        if !self.valid || !self.capture.matches(s) || mode(s) != self.mode {
            self.valid = false;
            return Err("Tangent gesture stopped: selection or document changed".into());
        }
        editable(s, &self.end.addr)?;
        if let Some(baseline) = &self.baseline {
            let pair = baseline.solve(self.mode, self.side, moved)?;
            let current =
                Junction::read(&s.doc, baseline.anim, baseline.addr.clone(), baseline.index)?;
            if let Some(cmd) = current.command(&s.doc, pair, Some(self.token))? {
                if !s.dispatch(Box::new(cmd)) {
                    return Err("Tangent gesture refused".into());
                }
                rebase(
                    s,
                    self.mode,
                    self.capture.anim,
                    self.capture.selection.clone(),
                );
            }
        } else {
            let k = KeyClip::capture(&s.doc, self.capture.anim, std::slice::from_ref(&self.end))?;
            let (mut out_handle, mut in_handle) = match k.keys[0].key.interp {
                Interp::Bezier {
                    out_handle,
                    in_handle,
                } => (out_handle, in_handle),
                Interp::Linear => (
                    glam::vec2(1.0 / 3.0, 1.0 / 3.0),
                    glam::vec2(2.0 / 3.0, 2.0 / 3.0),
                ),
                _ => return Err("Stepped has no editable handle".into()),
            };
            if !moved.is_finite() || !(0.0..=1.0).contains(&moved.x) {
                return Err("Invalid handle".into());
            }
            match self.side {
                Side::Incoming => in_handle = moved,
                Side::Outgoing => out_handle = moved,
            }
            let interp = Interp::Bezier {
                out_handle,
                in_handle,
            };
            if interp != k.keys[0].key.interp {
                s.dispatch(Box::new(
                    SetInterp::new(self.capture.anim, vec![self.end.clone()], interp)
                        .with_gesture(self.token),
                ));
                key_selection::set(s, self.capture.anim, self.capture.selection.clone());
            }
        }
        self.capture = Capture::new(s, self.capture.anim, &self.end.addr)?;
        Ok(())
    }
}
#[cfg(test)]
pub(crate) mod tests;
