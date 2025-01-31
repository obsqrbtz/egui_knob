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
        Self { value: 0.5 }
    }
}

impl eframe::App for KnobExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Knob::new(&mut self.value, 0.0, 1.0));
            ui.label(format!("Value: {:.2}", self.value));
        });
    }
}
