use super::*;
use ankhimate_core::{
    animation::{Axis, Key},
    math::Transform,
    skeleton::Bone,
};
pub fn fixture() -> (AppState, AnimationId, Vec<KeyRef>) {
    let mut s = AppState::default();
    let bone = s.doc.skeleton.add_bone(Bone {
        name: "root".into(),
        parent: None,
        length: 10.,
        local_transform: Transform::default(),
        inherit: Default::default(),
        color: Bone::default_color(),
    });
    let mut a = Animation::new("profiles", 2.);
    a.timelines = vec![
        Timeline::BoneTranslate {
            bone,
            axis: Axis::X,
            keys: vec![
                Key::linear(0., 0.),
                Key::linear(1., 250.),
                Key::linear(2., 500.),
            ],
        },
        Timeline::BoneRotate {
            bone,
            keys: vec![
                Key::linear(0., -45.),
                Key::linear(1., 22.5),
                Key::linear(2., 90.),
            ],
        },
        Timeline::BoneScale {
            bone,
            axis: Axis::X,
            keys: vec![
                Key::linear(0., 0.5),
                Key::linear(1., 1.),
                Key::linear(2., 1.5),
            ],
        },
    ];
    let anim = s.doc.animations.insert(a);
    s.session.active_animation = Some(anim);
    s.session.select_bone(Some(bone));
    s.set_work_mode(crate::session::WorkMode::Animate);
    let refs = vec![
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Translate,
                axis: Some(Axis::X),
            },
            index: 0,
        },
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Rotate,
                axis: None,
            },
            index: 0,
        },
        KeyRef {
            addr: TimelineAddr::Bone {
                bone,
                property: BoneProperty::Scale,
                axis: Some(Axis::X),
            },
            index: 0,
        },
    ];
    (s, anim, refs)
}
pub fn rect() -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(15., 50.), egui::vec2(600., 500.))
}
pub fn layout() -> Layout {
    Layout {
        sheet_x0: 30.,
        px_per_sec: 160.,
        scroll_sec: 0.,
        fps: 30,
    }
}
pub fn render(
    ctx: &egui::Context,
    s: &mut AppState,
    view: ViewState,
    lay: Layout,
    events: Vec<egui::Event>,
    mods: egui::Modifiers,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600., 900.),
            )),
            events,
            modifiers: mods,
            ..Default::default()
        },
        |ui| {
            let a = s.session.active_animation.unwrap();
            let model = TimelineModel::build(s, a);
            super::ui(
                ui,
                s,
                a,
                &model,
                &view,
                &lay,
                rect(),
                crate::ui::timeline::Style {
                    theme: &crate::theme::Theme::default(),
                    text: 12.,
                    operators: &crate::registry::Registry::with_builtins(),
                },
            );
        },
    )
}
pub fn pointer(p: egui::Pos2, down: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(p),
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed: down,
            modifiers: Default::default(),
        },
    ]
}
pub fn signature(out: &egui::FullOutput) -> serde_json::Value {
    let shapes = out
        .shapes
        .iter()
        .filter_map(|s| {
            let clip = [
                s.clip_rect.min.x,
                s.clip_rect.min.y,
                s.clip_rect.max.x,
                s.clip_rect.max.y,
            ];
            let value = match &s.shape {
                egui::Shape::Circle(c) => serde_json::json!([
                    "circle",
                    [c.center.x, c.center.y],
                    c.radius,
                    c.fill.to_array(),
                    c.stroke.width,
                    c.stroke.color.to_array(),
                    clip
                ]),
                egui::Shape::LineSegment { points, stroke } => serde_json::json!([
                    "line",
                    points.map(|p| [p.x, p.y]),
                    stroke.width,
                    stroke.color.to_array(),
                    clip
                ]),
                egui::Shape::Path(p) => serde_json::json!([
                    "path",
                    p.points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(),
                    format!("{:?}", p.stroke),
                    clip
                ]),
                egui::Shape::Text(t) => serde_json::json!([
                    "text",
                    t.galley.text(),
                    [t.pos.x, t.pos.y],
                    t.fallback_color.to_array(),
                    clip
                ]),
                egui::Shape::Rect(r) => serde_json::json!([
                    "rect",
                    [r.rect.min.x, r.rect.min.y, r.rect.max.x, r.rect.max.y],
                    r.fill.to_array(),
                    r.stroke.width,
                    r.stroke.color.to_array(),
                    clip
                ]),
                egui::Shape::Noop => serde_json::json!(["noop"]),
                _ => return None,
            };
            Some(value)
        })
        .collect::<Vec<_>>();
    serde_json::json!(shapes)
}
pub fn digest(value: &serde_json::Value) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    value.to_string().hash(&mut h);
    h.finish()
}
pub fn raw_case(case: usize) -> u64 {
    let (mut s, anim, _) = fixture();
    if case == 1 {
        for t in &mut s.doc.animations[anim].timelines {
            if let Timeline::BoneTranslate { keys, .. } = t {
                for k in keys {
                    k.value = -7.;
                }
            }
        }
    }
    if case == 2
        && let Timeline::BoneTranslate { keys, .. } = &mut s.doc.animations[anim].timelines[0]
    {
        keys[1].interp = Interp::Bezier {
            out_handle: glam::vec2(0.2, -2.),
            in_handle: glam::vec2(0.8, 3.),
        };
    }
    let ctx = egui::Context::default();
    digest(&signature(&render(
        &ctx,
        &mut s,
        ViewState::default(),
        layout(),
        vec![],
        Default::default(),
    )))
}
