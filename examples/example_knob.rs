use eframe::egui;
use egui_knob::{Knob, KnobStyle, LabelPosition};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui_knob demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(KnobDemo::default()))),
    )
}

struct KnobDemo {
    values: [f32; 6],
    show_bg_arc: bool,
    show_filled: bool,
    use_step: bool,
    knob_color: egui::Color32,
    line_color: egui::Color32,
    text_color: egui::Color32,
}

impl Default for KnobDemo {
    fn default() -> Self {
        Self {
            values: [f32::NAN; 6],
            show_bg_arc: true,
            show_filled: true,
            use_step: false,
            knob_color: egui::Color32::DARK_GRAY,
            line_color: egui::Color32::LIGHT_BLUE,
            text_color: egui::Color32::WHITE,
        }
    }
}

impl eframe::App for KnobDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎛 egui-knob example");
            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_bg_arc, "Show background arc");
                ui.checkbox(&mut self.show_filled, "Show filled segment");
                ui.checkbox(&mut self.use_step, "Enable step (0.1)");
            });

            ui.horizontal(|ui| {
                ui.label("Knob Colors:");
                ui.color_edit_button_srgba(&mut self.knob_color);
                ui.color_edit_button_srgba(&mut self.line_color);
                ui.color_edit_button_srgba(&mut self.text_color);
            });

            ui.separator();
            ui.label("👇 Scroll or drag knobs to interact:");

            ui.add_space(10.0);
            egui::Grid::new("knob_grid")
                .num_columns(3)
                .spacing([30.0, 20.0])
                .show(ui, |ui| {
                    for (i, (label, config)) in [
                        ("Basic Dot", KnobStyle::Dot),
                        ("Wiper, Sweep", KnobStyle::Wiper),
                        ("Thick Stroke", KnobStyle::Wiper),
                        ("360° Sweep", KnobStyle::Wiper),
                        ("Multi-Turn", KnobStyle::Dot),
                        ("Large Font", KnobStyle::Wiper),
                    ]
                    .iter()
                    .enumerate()
                    {
                        ui.vertical(|ui| {
                            ui.label(*label);
                            let mut knob = Knob::new(&mut self.values[i], 0.0, 1.0, *config)
                                .with_label(*label, LabelPosition::Bottom)
                                .with_background_arc(self.show_bg_arc)
                                .with_show_filled_segments(self.show_filled)
                                .with_colors(self.knob_color, self.line_color, self.text_color)
                                .with_step(self.use_step.then_some(0.1));

                            if *label == "Wiper, Sweep" {
                                knob = knob.with_sweep_range(0.25, 0.75).with_size(50.0);
                            }
                            if *label == "Thick Stroke" {
                                knob = knob.with_stroke_width(4.0).with_size(60.0);
                            }
                            if *label == "360° Sweep" {
                                knob = knob.with_sweep_range(0.5, 1.0);
                            }
                            if *label == "Multi-Turn" {
                                knob = knob.with_sweep_range(0.0, 2.5);
                            }
                            if *label == "Large Font" {
                                knob = knob.with_size(70.0).with_font_size(18.0);
                            }

                            ui.add(knob);
                        });

                        if (i + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(10.0);
            ui.separator();
        });
    }
}
