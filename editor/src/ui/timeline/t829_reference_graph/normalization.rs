//! View-only projection of existing scalar channels, never author data.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Range {
    pub min: f64,
    pub max: f64,
    pub count: usize,
}
impl Range {
    pub fn collect(values: impl IntoIterator<Item = f32>) -> Option<Self> {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut count = 0;
        for value in values {
            if value.is_finite() {
                let v = f64::from(value);
                min = min.min(v);
                max = max.max(v);
                count += 1;
            }
        }
        (count > 0).then_some(Self { min, max, count })
    }
    pub fn relative(self, value: f64) -> f64 {
        if !value.is_finite() {
            return f64::NAN;
        }
        if self.min == self.max {
            0.5
        } else {
            (value - self.min) / (self.max - self.min)
        }
    }
    pub fn legend(self) -> String {
        format!(
            "native [{}, {}]{}; {} finite key(s)",
            self.min as f32,
            self.max as f32,
            if self.min == self.max {
                " (constant → 0.5)"
            } else {
                ""
            },
            self.count
        )
    }
}
/// Indexed only within this one immutable visible-row traversal, never key identity.
pub(super) fn ranges(rows: &[VisibleRow<'_>], view: &ViewState) -> Vec<Vec<Option<Range>>> {
    rows.iter()
        .map(|row| {
            if !row.is_soloed(&view.soloed) {
                return Vec::new();
            }
            let VisibleRow::Property { data, .. } = row else {
                return Vec::new();
            };
            if data.read_only {
                return Vec::new();
            }
            data.channels
                .iter()
                .map(|ch| {
                    Range::collect(ch.values.iter().enumerate().filter_map(|(i, &v)| {
                        data.keys.get(i).filter(|k| k.time.is_finite()).map(|_| v)
                    }))
                })
                .collect()
        })
        .collect()
}
pub(super) fn label(s: &AppState, addr: &TimelineAddr, property: &str) -> String {
    let target = match addr {
        TimelineAddr::Bone { bone, .. } => s.doc.skeleton.bones.get(*bone).map(|b| b.name.as_str()),
        TimelineAddr::SlotColor { slot } => {
            s.doc.skeleton.slots.get(*slot).map(|b| b.name.as_str())
        }
        _ => None,
    };
    let units = match addr {
        TimelineAddr::Bone {
            property: BoneProperty::Rotate | BoneProperty::Shear,
            ..
        } => "degrees offset",
        TimelineAddr::Bone {
            property: BoneProperty::Scale,
            ..
        } => "factor",
        TimelineAddr::Bone { .. } => "position offset",
        TimelineAddr::SlotColor { .. } => "alpha",
        _ => "native",
    };
    format!(
        "{} / {property} ({units})",
        target.unwrap_or("Missing target")
    )
}
/// Mode changes stop old pointer captures; they cannot resume after toggling back.
pub(super) fn blocked(ui: &egui::Ui, normalized: bool) -> bool {
    let id = ui.id().with("graph_projection_mode");
    let down = ui.input(|i| i.pointer.primary_down());
    let previous = ui.ctx().data(|d| d.get_temp::<(bool, bool)>(id));
    let mut block = previous.is_some_and(|p| p.1);
    if previous.is_some_and(|p| p.0 != normalized) {
        block = down;
        ui.ctx().stop_dragging();
        ui.ctx().data_mut(|d| {
            d.remove::<crate::tangent_linking::Gesture>(ui.id().with("graph_tangent_capture"))
        });
    }
    if !down {
        block = false;
    }
    ui.ctx()
        .data_mut(|d| d.insert_temp(id, (normalized, block)));
    block
}
pub(super) fn axis(
    painter: &egui::Painter,
    rect: egui::Rect,
    to_y: &impl Fn(f64) -> f32,
    visuals: &egui::Visuals,
) {
    for (v, text) in [(0., "0"), (0.5, "0.5"), (1., "1")] {
        let y = to_y(v);
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(
                1.,
                visuals
                    .widgets
                    .noninteractive
                    .bg_stroke
                    .color
                    .gamma_multiply(0.3),
            ),
        );
        painter.text(
            egui::pos2(rect.left() + 3., y - 1.),
            egui::Align2::LEFT_BOTTOM,
            text,
            egui::FontId::proportional(9.),
            visuals.weak_text_color(),
        );
    }
}
