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
    basic_value: f32,
    purple_value: f32,
    large_value: f32,
    thick_value: f32,
    red_value: f32,
    green_value: f32,
    blue_value: f32,
}

impl Default for KnobExample {
    fn default() -> Self {
        Self {
            basic_value: 0.0,
            purple_value: 0.0,
            large_value: 0.0,
            thick_value: 0.0,
            red_value: 0.0,
            green_value: 0.0,
            blue_value: 0.0,
        }
    }
}

impl eframe::App for KnobExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(
                        Knob::new(&mut self.basic_value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                            .with_label("Basic", egui_knob::LabelPosition::Right)
                            .with_size(40.0)
                            .with_font_size(10.0)
                            .with_colors(
                                egui::Color32::from_rgb(60, 60, 60),
                                egui::Color32::from_rgb(150, 150, 150),
                                egui::Color32::from_rgb(200, 200, 200),
                            )
                            .with_background_arc(false),
                    )
                    .changed()
                {
                    println!("Basic value changed: {}", self.basic_value);
                }

                ui.add(
                    Knob::new(
                        &mut self.purple_value,
                        0.0,
                        100.0,
                        egui_knob::KnobStyle::Wiper,
                    )
                    .with_label("Purple", egui_knob::LabelPosition::Bottom)
                    .with_colors(
                        egui::Color32::from_rgb(60, 30, 80),
                        egui::Color32::from_rgb(200, 100, 255),
                        egui::Color32::from_rgb(230, 150, 255),
                    )
                    .with_size(50.0)
                    .with_font_size(14.0)
                    .with_stroke_width(3.0)
                    .with_step(0.1)
                    .with_background_arc(false),
                );

                ui.add(
                    Knob::new(&mut self.large_value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                        .with_label("Large", egui_knob::LabelPosition::Bottom)
                        .with_size(60.0)
                        .with_font_size(16.0),
                );

                ui.add(
                    Knob::new(
                        &mut self.thick_value,
                        0.0,
                        100.0,
                        egui_knob::KnobStyle::Wiper,
                    )
                    .with_label("Thick", egui_knob::LabelPosition::Bottom)
                    .with_size(50.0)
                    .with_stroke_width(4.0),
                );

                ui.add(
                    Knob::new(&mut self.red_value, 0.0, 100.0, egui_knob::KnobStyle::Dot)
                        .with_label("Red", egui_knob::LabelPosition::Bottom)
                        .with_colors(
                            egui::Color32::from_rgb(80, 30, 30),
                            egui::Color32::from_rgb(220, 50, 50),
                            egui::Color32::from_rgb(255, 100, 100),
                        )
                        .with_size(50.0),
                );

                ui.add(
                    Knob::new(
                        &mut self.green_value,
                        0.0,
                        100.0,
                        egui_knob::KnobStyle::Wiper,
                    )
                    .with_label("Leftandlongtext", egui_knob::LabelPosition::Left)
                    .with_colors(
                        egui::Color32::from_rgb(30, 80, 30),
                        egui::Color32::from_rgb(50, 220, 50),
                        egui::Color32::from_rgb(100, 255, 100),
                    )
                    .with_size(50.0)
                    .with_label_format(|v| format!("{:.2}%", v))
                    .with_sweep_range(1. / 8., 0.75),
                );

                ui.add(
                    Knob::new(&mut self.blue_value, 0.0, 100., egui_knob::KnobStyle::Dot)
                        .with_label("Top", egui_knob::LabelPosition::Top)
                        .with_colors(
                            egui::Color32::from_rgb(30, 30, 80),
                            egui::Color32::from_rgb(50, 50, 220),
                            egui::Color32::from_rgb(100, 100, 255),
                        )
                        .with_size(50.0)
                        .with_sweep_range(0., 2.),
                );
            });
        });
    }
}
