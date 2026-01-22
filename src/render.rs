use egui::{Align2, Color32, Painter, Pos2, Rect, Stroke, Ui, Vec2};

use crate::config::KnobConfig;
use crate::style::{KnobStyle, LabelPosition};

pub(crate) struct KnobRenderer<'a> {
    config: &'a KnobConfig,
    value: f32,
    raw: f32,
    min: f32,
    max: f32,
}

impl<'a> KnobRenderer<'a> {
    pub fn new(config: &'a KnobConfig, value: f32, raw: f32, min: f32, max: f32) -> Self {
        Self {
            config,
            value,
            raw,
            min,
            max,
        }
    }

    pub fn compute_angle(&self) -> f32 {
        if self.min == self.max || self.raw.is_nan() {
            self.config.min_angle
        } else {
            self.config.min_angle
                + self.raw * (self.config.max_angle - self.config.min_angle)
        }
    }

    pub fn render_knob(&self, painter: &Painter, center: Pos2, radius: f32, hovered: bool) {
        let knob_color = if hovered {
            self.config.colors.knob_color.linear_multiply(1.15)
        } else {
            self.config.colors.knob_color
        };

        // TODO: make an option
        painter.circle_filled(
            center,
            radius - self.config.stroke_width / 2.0,
            self.config.colors.knob_color.gamma_multiply(0.15),
        );

        painter.circle_stroke(
            center,
            radius,
            Stroke::new(self.config.stroke_width, knob_color),
        );

        if self.config.show_background_arc {
            self.render_background_arc(painter, center, radius);
        }

        let angle = self.compute_angle();
        match self.config.style {
            KnobStyle::Wiper => {
                let pointer = center + Vec2::angled(angle) * (radius * 0.65);
                painter.line_segment(
                    [center, pointer],
                    Stroke::new(
                        self.config.stroke_width * 1.2,
                        self.config.colors.line_color,
                    ),
                );
            }
            KnobStyle::Dot => {
                let dot_pos = center + Vec2::angled(angle) * (radius * 0.7);
                painter.circle_filled(
                    dot_pos,
                    self.config.stroke_width * 1.8,
                    self.config.colors.line_color,
                );
            }
        }
    }

    fn render_background_arc(&self, painter: &Painter, center: Pos2, radius: f32) {
        let arc_start = self.config.min_angle;
        let arc_end = self.config.max_angle;
        let segments = 128;
        let arc_color = self.config.colors.knob_color.gamma_multiply(0.35);
        let arc_radius = radius * 0.85;

        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = arc_start + (arc_end - arc_start) * t;
            let pos = center + Vec2::angled(angle) * arc_radius;
            points.push(pos);
        }

        painter.add(egui::Shape::line(
            points,
            Stroke::new(self.config.stroke_width, arc_color),
        ));

        if self.config.show_filled_segments {
            let filled_segments = (segments as f32
                * self.raw.clamp(0.0, 1.0))
                as usize;

            if filled_segments > 0 {
                let mut fill_points = Vec::with_capacity(filled_segments + 1);
                for i in 0..=filled_segments {
                    let t = i as f32 / segments as f32;
                    let angle = arc_start + (arc_end - arc_start) * t;
                    let pos = center + Vec2::angled(angle) * arc_radius;
                    fill_points.push(pos);
                }

                painter.add(egui::Shape::line(
                    fill_points,
                    Stroke::new(
                        self.config.stroke_width * 1.2,
                        self.config.colors.line_color,
                    ),
                ));
            }
        }
    }

    pub fn render_label(&self, ui: &Ui, rect: Rect) {
        if let Some(label) = &self.config.label {
            let label_text = format!("{}: {}", label, (self.config.label_format)(self.value));
            let font_id = egui::FontId::proportional(self.config.font_size);
            let label_padding = 4.0;

            let (label_pos, alignment) = match self.config.label_position {
                LabelPosition::Top => (
                    Vec2::new(rect.center().x, rect.min.y + label_padding),
                    Align2::CENTER_TOP,
                ),
                LabelPosition::Bottom => (
                    Vec2::new(rect.center().x, rect.max.y - label_padding),
                    Align2::CENTER_BOTTOM,
                ),
                LabelPosition::Left => (
                    Vec2::new(rect.min.x + label_padding, rect.center().y),
                    Align2::LEFT_CENTER,
                ),
                LabelPosition::Right => (
                    Vec2::new(rect.max.x - label_padding, rect.center().y),
                    Align2::RIGHT_CENTER,
                ),
            };

            ui.painter().text(
                label_pos.to_pos2(),
                alignment,
                label_text,
                font_id,
                self.config.colors.text_color,
            );
        }
    }

    pub fn calculate_size(&self, ui: &Ui) -> Vec2 {
        let knob_size = Vec2::splat(self.config.size + self.config.stroke_width * 2.0);

        let label_size = if let Some(label) = &self.config.label {
            let font_id = egui::FontId::proportional(self.config.font_size);
            let max_text = format!("{}: {}", label, (self.config.label_format)(self.max));
            ui.painter()
                .layout(max_text, font_id, Color32::WHITE, f32::INFINITY)
                .size()
        } else {
            Vec2::ZERO
        };

        let label_padding = 8.0;

        match self.config.label_position {
            LabelPosition::Top | LabelPosition::Bottom => Vec2::new(
                knob_size.x.max(label_size.x + label_padding * 2.0),
                knob_size.y + label_size.y + label_padding + self.config.label_offset,
            ),
            LabelPosition::Left | LabelPosition::Right => Vec2::new(
                knob_size.x + label_size.x + label_padding + self.config.label_offset,
                knob_size.y.max(label_size.y + label_padding),
            ),
        }
    }

    pub fn calculate_knob_rect(&self, rect: Rect) -> Rect {
        let knob_size = Vec2::splat(self.config.size);

        match self.config.label_position {
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
        }
    }
}
