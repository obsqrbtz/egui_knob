use egui::{Align2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

pub enum LabelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

pub enum KnobStyle {
    Wiper,
    Dot,
}

pub struct Knob<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    size: f32,
    font_size: f32,
    stroke_width: f32,
    knob_color: Color32,
    line_color: Color32,
    text_color: Color32,
    label: Option<String>,
    label_position: LabelPosition,
    style: KnobStyle,
    label_offset: f32,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32, min: f32, max: f32, style: KnobStyle) -> Self {
        Self {
            value,
            min,
            max,
            size: 40.0,
            font_size: 12.0,
            stroke_width: 2.0,
            knob_color: Color32::GRAY,
            line_color: Color32::GRAY,
            text_color: Color32::WHITE,
            label: None,
            label_position: LabelPosition::Bottom,
            style,
            label_offset: 2.0,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn with_colors(
        mut self,
        knob_color: Color32,
        line_color: Color32,
        text_color: Color32,
    ) -> Self {
        self.knob_color = knob_color;
        self.line_color = line_color;
        self.text_color = text_color;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>, position: LabelPosition) -> Self {
        self.label = Some(label.into());
        self.label_position = position;
        self
    }

    pub fn with_label_offset(mut self, offset: f32) -> Self {
        self.label_offset = offset;
        self
    }
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let knob_size = Vec2::splat(self.size);

        let label_size = if let Some(label) = &self.label {
            let font_id = egui::FontId::proportional(self.font_size);
            ui.painter()
                .layout(
                    format!("{}: {:.2}", label, self.value),
                    font_id,
                    Color32::WHITE,
                    f32::INFINITY,
                )
                .size()
        } else {
            Vec2::ZERO
        };

        let label_padding = 2.0;

        let adjusted_size = match self.label_position {
            LabelPosition::Top | LabelPosition::Bottom => Vec2::new(
                knob_size.x.max(label_size.x + label_padding * 2.0),
                knob_size.y + label_size.y + label_padding * 2.0 + self.label_offset,
            ),
            LabelPosition::Left | LabelPosition::Right => Vec2::new(
                knob_size.x + label_size.x + label_padding * 2.0 + self.label_offset,
                knob_size.y.max(label_size.y + label_padding * 2.0),
            ),
        };

        let (rect, mut response) = ui.allocate_exact_size(adjusted_size, Sense::drag());

        if response.dragged() {
            let delta = response.drag_delta().y;
            let range = self.max - self.min;
            let step = range * 0.005;
            *self.value = (*self.value - delta * step).clamp(self.min, self.max);
            response.mark_changed();
        }

        let painter = ui.painter();
        let knob_rect = match self.label_position {
            LabelPosition::Left => {
                Rect::from_min_size(rect.right_top() + Vec2::new(-knob_size.x, 0.0), knob_size)
            }
            LabelPosition::Right => Rect::from_min_size(rect.left_top(), knob_size),
            LabelPosition::Top => Rect::from_min_size(
                rect.left_bottom() + Vec2::new((rect.width() - knob_size.x) / 2.0, -knob_size.y),
                knob_size,
            ),
            LabelPosition::Bottom => Rect::from_min_size(
                rect.left_top() + Vec2::new((rect.width() - knob_size.x) / 2.0, 0.0),
                knob_size,
            ),
        };

        let center = knob_rect.center();
        let radius = knob_size.x / 2.0;
        let angle = (*self.value - self.min) / (self.max - self.min) * std::f32::consts::PI * 1.5
            - std::f32::consts::PI;

        painter.circle_stroke(
            center,
            radius,
            Stroke::new(self.stroke_width, self.knob_color),
        );

        match self.style {
            KnobStyle::Wiper => {
                let pointer = center + Vec2::angled(angle) * (radius * 0.7);
                painter.line_segment(
                    [center, pointer],
                    Stroke::new(self.stroke_width * 1.5, self.line_color),
                );
            }
            KnobStyle::Dot => {
                let dot_pos = center + Vec2::angled(angle) * (radius * 0.7);
                painter.circle_filled(dot_pos, self.stroke_width * 1.5, self.line_color);
            }
        }

        if let Some(label) = self.label {
            let label_text = format!("{label}: {:.2}", self.value);
            let font_id = egui::FontId::proportional(self.font_size);

            let (label_pos, alignment) = match self.label_position {
                LabelPosition::Top => (
                    Vec2::new(
                        rect.center().x,
                        rect.min.y - self.label_offset + label_padding,
                    ),
                    Align2::CENTER_TOP,
                ),
                LabelPosition::Bottom => (
                    Vec2::new(rect.center().x, rect.max.y + self.label_offset),
                    Align2::CENTER_BOTTOM,
                ),
                LabelPosition::Left => (
                    Vec2::new(rect.min.x - self.label_offset, rect.center().y),
                    Align2::LEFT_CENTER,
                ),
                LabelPosition::Right => (
                    Vec2::new(rect.max.x + self.label_offset, rect.center().y),
                    Align2::RIGHT_CENTER,
                ),
            };

            ui.painter().text(
                label_pos.to_pos2(),
                alignment,
                label_text,
                font_id,
                self.text_color,
            );
        }

        // Draw the bounding rect
        //painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::RED));

        response
    }
}
