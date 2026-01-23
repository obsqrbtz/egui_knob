use egui::{remap, Color32, Response, Sense, Ui, Widget};

use crate::config::KnobConfig;
use crate::render::KnobRenderer;
use crate::style::{KnobStyle, LabelPosition};

pub struct Knob<'a> {
    pub(crate) value: &'a mut f32,
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) config: KnobConfig,
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
            config: KnobConfig::new(style),
        }
    }

    /// Sets the angular sweep range of the knob
    ///
    /// This controls where the knob starts and how far it can rotate. By default,
    /// knobs start at the left (180°) and sweep 270° clockwise to bottom.
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
    ///   - Negative values are clamped to 0.0
    ///
    /// Note: the start angle is offset by PI/2 so that `0.0` is at the bottom (6 o'clock)
    pub fn with_sweep_range(mut self, start_angle_normalized: f32, range: f32) -> Self {
        if start_angle_normalized.is_nan() || range.is_nan() {
            return self;
        }

        self.config.min_angle = 
            start_angle_normalized.rem_euclid(1.0) * std::f32::consts::TAU + std::f32::consts::PI / 2.0;
        self.config.max_angle = self.config.min_angle + range.max(0.0) * std::f32::consts::TAU;
        self
    }

    /// Sets the size of the knob
    pub fn with_size(mut self, size: f32) -> Self {
        self.config.size = size;
        self
    }

    /// Sets the font size for the label
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.config.font_size = size;
        self
    }

    /// Sets the stroke width for the knob's outline and indicator
    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.config.stroke_width = width;
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
        self.config.colors.knob_color = knob_color;
        self.config.colors.line_color = line_color;
        self.config.colors.text_color = text_color;
        self
    }

    /// Adds a label to the knob
    ///
    /// # Arguments
    /// * `label` - Text to display
    /// * `position` - Position of the label relative to the knob
    pub fn with_label(mut self, label: impl Into<String>, position: LabelPosition) -> Self {
        self.config.label = Some(label.into());
        self.config.label_position = position;
        self
    }

    /// Sets the spacing between the knob and its label
    pub fn with_label_offset(mut self, offset: f32) -> Self {
        self.config.label_offset = offset;
        self
    }

    /// Sets a custom format function for displaying the value
    ///
    /// # Example
    /// ```no_run
    /// use egui_knob::{Knob, KnobStyle};
    /// ui.add(
    ///     Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
    ///         .with_label_format(|v| format!("{:.1}%", v * 100.0))
    /// );
    /// ```
    pub fn with_label_format(mut self, format: impl Fn(f32) -> String + 'static) -> Self {
        self.config.label_format = Box::new(format);
        self
    }

    /// Sets the step size for value changes
    pub fn with_step(mut self, step: Option<f32>) -> Self {
        self.config.step = step;
        self
    }

    /// Controls whether to show the background arc indicating the full range
    pub fn with_background_arc(mut self, enabled: bool) -> Self {
        self.config.show_background_arc = enabled;
        self
    }

    /// Controls whether to show the filled segment on the background arc
    ///
    /// When enabled (and background arc is visible), displays a colored segment
    /// from the minimum position to the current value position.
    pub fn with_show_filled_segments(mut self, enabled: bool) -> Self {
        self.config.show_filled_segments = enabled;
        self
    }

    /// Sets the drag sensitivity for mouse interactions
    ///
    /// Default is 0.005.
    pub fn with_drag_sensitivity(mut self, sensitivity: f32) -> Self {
        self.config.drag_sensitivity = sensitivity;
        self
    }

    /// Sets a reset value to return to on doubleclick event.
    pub fn with_double_click_reset(mut self, reset_value: f32) -> Self {
        self.config.reset_value = Some(reset_value);
        self
    }

    /// Allows user to use scroll wheel to change knob value
    /// Uses config.step for the increment value
    pub fn with_middle_scroll(mut self) -> Self {
        self.config.allow_scroll = true;
        self
    }
    pub fn with_logarithmic_scaling(mut self) -> Self {
        self.config.logarithmic_scaling = true;
        self
    }
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.value.is_nan() {
            *self.value = self.min;
        }

        let mut raw = if self.config.logarithmic_scaling {
            remap(*self.value, self.min..=self.max, 1.0..=10.0).log(10.0)
        } else {
            remap(*self.value, self.min..=self.max, 0.0..=1.0)
        };

        let renderer = KnobRenderer::new(&self.config, *self.value, raw, self.min, self.max);
        let adjusted_size = renderer.calculate_size(ui);

        let (rect, response) = ui.allocate_exact_size(adjusted_size, Sense::click_and_drag());

        let mut response = response;
        if response.dragged() {
            let delta = response.drag_delta().y;
            let step = self.config.step.unwrap_or(self.config.drag_sensitivity);
            raw = (raw - delta * step).clamp(0.0,1.0);

            raw = if let Some(step) = self.config.step {
                let steps = (raw / step).round();
                (steps * step).clamp(0.0, 1.0)
            } else {
                raw
            };

            if self.value.is_nan() {
                *self.value = 0.0;
            }

            response.mark_changed();
        }  else if response.hovered() & self.config.allow_scroll {
            if let Some(scoll) = ui.input(|input| {
                input.events.iter().find_map(|e| match e {
                    egui::Event::MouseWheel { delta, .. } => Some(*delta),
                    _ => None,
                })
            }) {
                raw = (raw
                    + scoll.y * self.config.step.unwrap_or(self.config.drag_sensitivity))
                .clamp(0.0, 1.0);
            }
        }

        *self.value = if self.config.logarithmic_scaling {
            remap(10f32.powf(raw), 1.0..=10.0, self.min..=self.max)
        }else {
            remap(raw, 0.0..=1.0, self.min..=self.max)
        };

        if response.double_clicked() {
            if let Some(reset_value) = self.config.reset_value {
                *self.value = reset_value
            }
        }

        let knob_rect = renderer.calculate_knob_rect(rect);
        let center = knob_rect.center();
        let radius = self.config.size / 2.0;

        let updated_renderer = KnobRenderer::new(&self.config, *self.value, raw, self.min, self.max);
        updated_renderer.render_knob(ui.painter(), center, radius, response.hovered());
        updated_renderer.render_label(ui, rect);

        if self.config.label.is_some() && response.hovered() {
            response
                .clone()
                .on_hover_text((self.config.label_format)(*self.value));
        }

        response
    }
}
