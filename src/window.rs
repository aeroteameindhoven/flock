use eframe::{
    egui::{self, Frame, Sense, Slider},
    epaint::{Color32, Vec2},
};

use crate::component::heading::HeadingIndicator;
pub struct MainWindow {
    pub heading: f32,
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
            Frame::canvas(&ctx.style()).show(ui, |ui| {
                let space = ui.available_size();
                let (response, painter) =
                    ui.allocate_painter(Vec2::splat(space.min_elem()), Sense::hover());
                let bounds = response.rect;

                let size = f32::min(bounds.width(), bounds.height());
                let radius = size / 2.0;

                painter.circle_filled(bounds.center(), radius, Color32::LIGHT_BLUE);

                // Shape::CubicBezier(CubicBezierShape::from_points_stroke([], closed, fill, stroke))
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
