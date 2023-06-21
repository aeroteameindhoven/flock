use eframe::egui::{self, Frame, Slider};

use crate::component::{attitude::AttitudeIndicator, heading::HeadingIndicator};
pub struct MainWindow {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("File", |ui| {});
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("View", |ui| {});
                ui.menu_button("Tools", |ui| {});
                ui.menu_button("Help", |ui| {});
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {});

        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.button("⏪");
                ui.button("⏴");
                ui.button("⏸");
                ui.button("⏵");
                ui.button("⏩");
                ui.scope(|ui| {
                    ui.style_mut().spacing.slider_width = ui.available_width();
                    ui.add(
                        Slider::new(&mut 10, 0..=100)
                            .show_value(false)
                            .trailing_fill(true),
                    )
                });
            });
        });

        egui::Window::new("Attitude Indicator").show(ctx, |ui| {
            ui.add(
                Slider::new(&mut self.roll, -360.0..=360.0)
                    .text("Roll")
                    .integer()
                    .step_by(1.0)
                    .trailing_fill(true),
            );
            ui.horizontal(|ui| {
                ui.add(
                    Slider::new(&mut self.pitch, -360.0..=360.0)
                        .text("Pitch")
                        .integer()
                        .step_by(1.0)
                        .trailing_fill(true)
                        .vertical(),
                );
                Frame::canvas(&ctx.style()).show(ui, |ui| {
                    ui.add(AttitudeIndicator::new(self.pitch, self.roll))
                });
            });
        });

        egui::Window::new("Heading Indicator")
            .constrain(true)
            .show(ctx, |ui| {
                ui.add(
                    Slider::new(&mut self.heading, -360.0..=720.0)
                        .text("Heading")
                        .integer()
                        .step_by(5.0)
                        .trailing_fill(true),
                );
                Frame::canvas(&ctx.style())
                    .show(ui, |ui| ui.add(HeadingIndicator::new(self.heading)));
            });
    }
}
