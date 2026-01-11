use crate::style::{KnobColors, KnobStyle, LabelPosition};

pub struct KnobConfig {
    pub(crate) size: f32,
    pub(crate) font_size: f32,
    pub(crate) stroke_width: f32,
    pub(crate) colors: KnobColors,
    pub(crate) label: Option<String>,
    pub(crate) label_position: LabelPosition,
    pub(crate) style: KnobStyle,
    pub(crate) label_offset: f32,
    pub(crate) label_format: Box<dyn Fn(f32) -> String>,
    pub(crate) step: Option<f32>,
    pub(crate) drag_sensitivity: f32,
    pub(crate) show_background_arc: bool,
    pub(crate) show_filled_segments: bool,
    pub(crate) min_angle: f32,
    pub(crate) max_angle: f32,
    pub(crate) reset_value: Option<f32>,
}

impl KnobConfig {
    pub fn new(style: KnobStyle) -> Self {
        Self {
            size: 40.0,
            font_size: 12.0,
            stroke_width: 2.0,
            colors: KnobColors::default(),
            label: None,
            label_position: LabelPosition::Bottom,
            style,
            label_offset: 1.0,
            label_format: Box::new(|v| format!("{:.2}", v)),
            step: None,
            min_angle: -std::f32::consts::PI,
            max_angle: std::f32::consts::PI * 0.5,
            drag_sensitivity: 0.005,
            show_background_arc: true,
            show_filled_segments: true,
            reset_value: None,
        }
    }
}
