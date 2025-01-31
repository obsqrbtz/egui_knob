use egui::{Response, Sense, Ui, Vec2, Widget};

pub struct Knob<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32, min: f32, max: f32) -> Self {
        Self { value, min, max }
    }
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(40.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::drag());

        if response.dragged() {
            let delta = response.drag_delta().y;
            let range = self.max - self.min;
            let step = range * 0.005;
            *self.value = (*self.value - delta * step).clamp(self.min, self.max);
        }

        let painter = ui.painter();
        let center = rect.center();
        let radius = rect.width() / 2.0;
        let angle = (*self.value - self.min) / (self.max - self.min) * std::f32::consts::PI * 1.5
            - std::f32::consts::PI * 0.75;

        painter.circle_stroke(center, radius, egui::Stroke::new(2.0, ui.visuals().text_color()));
        let pointer = center + Vec2::angled(angle) * (radius * 0.7);
        painter.line_segment([center, pointer], egui::Stroke::new(3.0, ui.visuals().text_color()));

        response
    }
}
