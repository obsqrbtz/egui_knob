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
                ui.add_space(15.0);
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_size(50.0),
                );
                ui.add_space(15.0);
                ui.add(
                    Knob::new(&mut self.value, 0.0, 100.0, egui_knob::KnobStyle::Wiper)
                        .with_label("Gain", egui_knob::LabelPosition::Bottom)
                        .with_size(50.0),
                );
            });
        });
    }
}
