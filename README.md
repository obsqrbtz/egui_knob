# egui_knob

[![Crates.io](https://img.shields.io/crates/v/egui_knob)](https://crates.io/crates/egui_knob)
[![Documentation](https://docs.rs/egui_knob/badge.svg)](https://docs.rs/egui_knob)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A simple, customizable knob widget for egui.

![Knob Widget Screenshot](scrot.png)

## Features

- Adjustable size, font size, and stroke width
- Customizable colors for the knob, indicator, and text
- Label positions (Top, Bottom, Left, Right)
- Custom label formatting
- Two visual styles: Wiper and Dot
- Configurable sweep range
- Background arc with filled segments
- Adjustable drag sensitivity
- Logarithmic scaling

## Installation

To use the Knob widget in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
egui = "0.33"
eframe = "0.33"
egui_knob = "0.3.9"
```

## Usage

### Basic Example

```rust
use egui_knob::{Knob, KnobStyle, LabelPosition};
use eframe::egui;

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

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Knob Example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(KnobApp::default()))),
    )
}
```

### Advanced Examples

#### Custom Sweep Range
```rust
// 270° sweep starting from the left (9 o'clock position)
Knob::new(&mut value, 0.0, 100.0, KnobStyle::Wiper)
    .with_sweep_range(0.25, 0.75)
    .with_label("Gain", LabelPosition::Bottom);
```

#### Multi-Turn Knob
```rust
// 2.5 full rotations
Knob::new(&mut value, 0.0, 1.0, KnobStyle::Dot)
    .with_sweep_range(0.0, 2.5);
```

#### Stepped Values
```rust
// Snap to 0.1 increments
Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
    .with_step(Some(0.1))
    .with_label_format(|v| format!("{:.1}", v));
```

#### Custom Formatting
```rust
// Display as percentage
Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
    .with_label_format(|v| format!("{:.0}%", v * 100.0));
```

#### Logarithmic Knobs
```rust
// Enable logarithmic scaling
Knob::new(&mut value, 0.0, 1.0, KnobStyle::Wiper)
    .with_logarithmic_scaling();
```

## Running demo app

```bash
cargo run --example example_knob
```

Demo app is available at [examples/example_knob.rs](examples/example_knob.rs).

## Contributing

Contributions are welcome. Feel free to open an issue or submit a PR.
