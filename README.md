# egui_knob

![Crates.io Version](https://img.shields.io/crates/v/egui_knob)

Simple knob widget for egui.

![Knob Widget Screenshot](scrot.png)

## Features

- Adjustable size, font size, and stroke width.
- Customizable colors for the knob, indicator and text.
- Label positions (Top, Bottom, Left, Right).
- Label formatting.
- Two styles: Wiper and Dot.

## Installation

To use the Knob widget in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
egui = "0.32"
egui_knob = "0.3.3"
```

## Usage example

```rust
use egui_knob::{Knob, KnobStyle, LabelPosition};
use eframe::{egui};

struct KnobApp {
    value: f32,
}

impl Default for KnobApp {
    fn default() -> Self {
        Self { value: 0.5 }
    }
}

impl eframe::App for KnobApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let knob = Knob::new(&mut self.value, 0.0, 1.0, KnobStyle::Wiper)
                .with_size(50.0)
                .with_font_size(14.0)
                .with_colors(egui::Color32::GRAY, egui::Color32::WHITE, egui::Color32::WHITE)
                .with_stroke_width(3.0)
                .with_label("Volume", LabelPosition::Top);

            ui.add(knob);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minimal",
        options,
        Box::new(|_cc| Ok(Box::new(KnobApp::default()) as Box<dyn eframe::App>)),
    ).unwrap();
}
```
