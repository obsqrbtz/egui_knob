use core::f32;

use egui::{Align2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Position of the label relative to the knob
pub enum LabelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Visual style of the knob indicator
pub enum KnobStyle {
    /// A line extending from the center to the edge
    Wiper,
    /// A dot on the edge of the knob
    Dot,
}

/// A circular knob widget for egui that can be dragged to change a value
///
/// # Example
/// ```
/// let mut value = 0.5;
/// Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
///     .with_size(50.0)
///     .with_label("Volume", LabelPosition::Bottom)
///     .with_step(0.1);
/// ```
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
    label_format: Box<dyn Fn(f32) -> String>,
    step: Option<f32>,
    min_angle: f32,
    max_angle: f32,
}

impl<'a> Knob<'a> {
    /// Creates a new knob widget
    ///
    /// # Arguments
    /// * `value` - Mutable reference to the value controlled by the knob
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    /// * `style` - Visual style of the knob indicator
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
            label_offset: 1.0,
            label_format: Box::new(|v| format!("{:.2}", v)),
            step: None,
            min_angle: -std::f32::consts::PI,
            max_angle: std::f32::consts::PI * 0.5,
        }
    }

    /// Sets the angular sweep range of the knob
    ///
    /// This controls where the knob starts and how far it can rotate. By default,
    /// knobs start at the bottom-left (135°) and sweep 270° clockwise to bottom-right.
    ///
    /// # Arguments
    /// * `start_angle_normalized` - Starting position as fraction of full circle:
    ///   - `0.0` = bottom (6 o'clock)
    ///   - `0.25` = left (9 o'clock)
    ///   - `0.5` = top (12 o'clock)
    ///   - `0.75` = right (3 o'clock)
    /// * `range` - How far the knob can sweep as fraction of full circle:
    ///   - `0.25` = quarter turn (90°)
    ///   - `0.5` = half turn (180°)
    ///   - `0.75` = three-quarter turn (270°)
    ///   - `1.0` = full turn (360°)
    ///   - Values > 1.0 create multi-turn knobs
    pub fn with_sweep_range(mut self, start_angle_normalized: f32, range: f32) -> Self {
        self.min_angle =
            start_angle_normalized.rem_euclid(1.) * f32::consts::PI * 2. + f32::consts::PI / 2.;

        // A range of 1. represent a full turn
        self.max_angle = self.min_angle + range.max(0.) * 2. * f32::consts::PI;
        self
    }

    /// Sets the size of the knob
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the font size for the label
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the stroke width for the knob's outline and indicator
    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Sets the colors for different parts of the knob
    ///
    /// # Arguments
    /// * `knob_color` - Color of the knob's outline
    /// * `line_color` - Color of the indicator
    /// * `text_color` - Color of the label text
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

    /// Adds a label to the knob
    ///
    /// # Arguments
    /// * `label` - Text to display
    /// * `position` - Position of the label relative to the knob
    pub fn with_label(mut self, label: impl Into<String>, position: LabelPosition) -> Self {
        self.label = Some(label.into());
        self.label_position = position;
        self
    }

    /// Sets the spacing between the knob and its label
    pub fn with_label_offset(mut self, offset: f32) -> Self {
        self.label_offset = offset;
        self
    }

    /// Sets a custom format function for displaying the value
    ///
    /// # Example
    /// ```
    /// # let mut value = 0.5;
    /// Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
    ///     .with_label_format(|v| format!("{:.1}%", v * 100.0));
    /// ```
    pub fn with_label_format(mut self, format: impl Fn(f32) -> String + 'static) -> Self {
        self.label_format = Box::new(format);
        self
    }

    /// Sets the step size for value changes
    ///
    /// When set, the value will snap to discrete steps as the knob is dragged.
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let knob_size = Vec2::splat(self.size);

        let label_size = if let Some(label) = &self.label {
            let font_id = egui::FontId::proportional(self.font_size);
            let max_text = format!("{}: {}", label, (self.label_format)(self.max));
            ui.painter()
                .layout(max_text, font_id, Color32::WHITE, f32::INFINITY)
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
            let step = self.step.unwrap_or(range * 0.005);
            let new_value = (*self.value - delta * step).clamp(self.min, self.max);

            *self.value = if let Some(step) = self.step {
                let steps = ((new_value - self.min) / step).round();
                (self.min + steps * step).clamp(self.min, self.max)
            } else {
                new_value
            };

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
        let angle = self.min_angle
            + (*self.value - self.min) / (self.max - self.min) * (self.max_angle - self.min_angle);

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
            let label_text = format!("{}: {}", label, (self.label_format)(*self.value));
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
