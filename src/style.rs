use egui::Color32;

/// Visual style of the knob indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnobStyle {
    /// A line extending from the center to the edge
    Wiper,
    /// A dot on the edge of the knob
    Dot,
}

/// Position of the label relative to the knob
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelPosition {
    /// Label appears above the knob
    Top,
    /// Label appears below the knob
    Bottom,
    /// Label appears to the left of the knob
    Left,
    /// Label appears to the right of the knob
    Right,
}

/// Color configuration for the knob widget
#[derive(Debug, Clone, Copy)]
pub struct KnobColors {
    /// Color of the knob's outline
    pub knob_color: Color32,
    /// Color of the indicator (wiper or dot)
    pub line_color: Color32,
    /// Color of the label text
    pub text_color: Color32,
}

impl Default for KnobColors {
    fn default() -> Self {
        Self {
            knob_color: Color32::GRAY,
            line_color: Color32::GRAY,
            text_color: Color32::WHITE,
        }
    }
}
