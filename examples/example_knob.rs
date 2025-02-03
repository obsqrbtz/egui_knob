use eframe::egui;
use egui_knob::Knob;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Knob Example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(KnobExample::default()))),
    )
}

struct KnobExample {
    value: f32,
}

impl Default for KnobExample {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl eframe::App for KnobExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_size(50.0),
                );
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Wiper)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_size(50.0),
                );
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_colors(
                            egui::Color32::DARK_GRAY,
                            egui::Color32::WHITE,
                            egui::Color32::WHITE,
                        )
                        .with_size(50.0),
                );
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Wiper)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_colors(
                            egui::Color32::DARK_GRAY,
                            egui::Color32::WHITE,
                            egui::Color32::LIGHT_BLUE,
                        )
                        .with_size(50.0),
                );
            });
        });
    }
}
