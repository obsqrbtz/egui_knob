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
    #[deprecated(
        since = "0.4.0",
        note = "use with_knob_color, with_line_color and with_text_color instead"
    )]
    pub fn with_colors(
        self,
        knob_color: Color32,
        line_color: Color32,
        text_color: Color32,
    ) -> Self {
        self.with_knob_color(knob_color)
            .with_line_color(line_color)
            .with_text_color(text_color)
    }

    /// Sets the color of the knob's outline
    pub fn with_knob_color(mut self, color: Color32) -> Self {
        self.config.colors.knob_color = color;
        self
    }

    /// Sets the color of the indicator
    pub fn with_line_color(mut self, color: Color32) -> Self {
        self.config.colors.line_color = color;
        self
    }

    /// Sets the color of the label text
    pub fn with_text_color(mut self, color: Color32) -> Self {
        self.config.colors.text_color = color;
        self
    }

    /// Sets the knob's fill color, overriding the default derived from the knob color
    pub fn with_fill_color(mut self, color: Color32) -> Self {
        self.config.colors.fill_color = color;
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

    /// Sets the step size for value changes, in the same units as `min` and `max`
    /// Non-finite or non-positive steps are ignored.
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
    /// Moves one `step` per wheel event, or scales by config.drag_sensitivity if no step is set
    pub fn with_middle_scroll(mut self) -> Self {
        self.config.allow_scroll = true;
        self
    }
    pub fn with_logarithmic_scaling(mut self) -> Self {
        self.config.logarithmic_scaling = true;
        self
    }
}

fn value_to_raw(value: f32, min: f32, max: f32, logarithmic: bool) -> f32 {
    if min == max {
        return 0.0;
    }

    if logarithmic {
        remap(value, min..=max, 1.0..=10.0).log(10.0)
    } else {
        remap(value, min..=max, 0.0..=1.0)
    }
}

fn raw_to_value(raw: f32, min: f32, max: f32, logarithmic: bool) -> f32 {
    if logarithmic {
        remap(10f32.powf(raw), 1.0..=10.0, min..=max)
    } else {
        remap(raw, 0.0..=1.0, min..=max)
    }
}

fn snap_to_step(value: f32, min: f32, max: f32, step: Option<f32>) -> f32 {
    match step {
        Some(step) if step.is_finite() && step > 0.0 => {
            let steps = ((value - min) / step).round();
            let (lo, hi) = (min.min(max), min.max(max));
            (min + steps * step).max(lo).min(hi)
        }
        _ => value,
    }
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.value.is_nan() {
            *self.value = self.min;
        }

        let logarithmic = self.config.logarithmic_scaling;
        let (min, max, step) = (self.min, self.max, self.config.step);
        let previous = *self.value;
        let mut raw = value_to_raw(*self.value, self.min, self.max, logarithmic);

        let renderer = KnobRenderer::new(&self.config, *self.value, raw, self.min, self.max);
        let adjusted_size = renderer.calculate_size(ui);

        let (rect, response) = ui.allocate_exact_size(adjusted_size, Sense::click_and_drag());

        let mut response = response;
        let mut moved = false;

        let base = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(response.id))
            .filter(|prev| {
                snap_to_step(raw_to_value(*prev, min, max, logarithmic), min, max, step) == previous
            })
            .unwrap_or(raw);

        if response.dragged() {
            let mut sensitivity = self.config.drag_sensitivity;

            if let Some(step) = step.filter(|step| step.is_finite() && *step > 0.0) {
                let one_step =
                    (value_to_raw(*self.value + step, min, max, logarithmic) - raw).abs();
                if one_step > 0.0 {
                    sensitivity = sensitivity.min(one_step);
                }
            }

            raw = (base - response.drag_delta().y * sensitivity).clamp(0.0, 1.0);
            moved = true;
        }  else if response.hovered() & self.config.allow_scroll && let Some(scoll) = ui.input(|input| {
                input.events.iter().find_map(|e| match e {
                    egui::Event::MouseWheel { delta, .. } if delta.y != 0.0 => Some(*delta),
                    _ => None,
                })
            }) {
            raw = match step.filter(|step| step.is_finite() && *step > 0.0) {
                Some(step) => {
                    let index = (*self.value - min) / step;
                    let next = if snap_to_step(*self.value, min, max, Some(step)) == *self.value {
                        index.round() + scoll.y.signum()
                    } else if scoll.y > 0.0 {
                        index.ceil()
                    } else {
                        index.floor()
                    };
                    value_to_raw(min + next * step, min, max, logarithmic).clamp(0.0, 1.0)
                }
                None => (base + scoll.y * self.config.drag_sensitivity).clamp(0.0, 1.0),
            };
            moved = true;
        }

        // Hide the wheel from parent ScrollAreas while adjusting knob
        if self.config.allow_scroll && response.hovered() {
            ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
        }

        if moved {
            ui.ctx().data_mut(|data| data.insert_temp(response.id, raw));
            *self.value = snap_to_step(raw_to_value(raw, min, max, logarithmic), min, max, step);
        }

        if response.drag_stopped() {
            ui.ctx().data_mut(|data| data.remove::<f32>(response.id));
        }

        if response.double_clicked()
            && let Some(reset_value) = self.config.reset_value {
                *self.value = reset_value
            }

        if *self.value != previous {
            response.mark_changed();
        }

        let raw = value_to_raw(*self.value, self.min, self.max, logarithmic).clamp(0.0, 1.0);

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
