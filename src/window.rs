use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex, Frame, Slider, Vec2};

use crate::component::{
    attitude::{AttitudeIndicator, AttitudeIndicatorRectangular},
    heading::HeadingIndicator,
};

pub struct MainWindow {
    attitude: Arc<Mutex<Attitude>>,
}

#[derive(Debug, Default, Clone, Copy, serde::Deserialize)]
pub struct Attitude {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl MainWindow {
    pub fn new(attitude: Arc<Mutex<Attitude>>) -> Self {
        Self { attitude }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        //     ui.horizontal(|ui| {
        //         egui::widgets::global_theme_preference_switch(ui);
        //         ui.menu_button("File", |ui| {});
        //         ui.menu_button("Edit", |ui| {});
        //         ui.menu_button("View", |ui| {});
        //         ui.menu_button("Tools", |ui| {});
        //         ui.menu_button("Help", |ui| {});
        //     })
        // });

        // FIXME: Am i holding this lock for too long?
        // TODO: how to push updates?
        let mut attitude = self.attitude.lock();

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add(
            //     Slider::new(&mut attitude.heading, -360.0..=720.0)
            //         .text("Heading")
            //         .integer()
            //         .step_by(5.0)
            //         .trailing_fill(true),
            // );
            // ui.add(
            //     Slider::new(&mut attitude.roll, -360.0..=360.0)
            //         .text("Roll")
            //         .integer()
            //         .step_by(1.0)
            //         .trailing_fill(true),
            // );
            // ui.add(
            //     Slider::new(&mut attitude.pitch, -360.0..=360.0)
            //         .text("Pitch")
            //         .integer()
            //         .step_by(1.0)
            //         .trailing_fill(true),
            // );

            let height = ui.available_height();

            Frame::canvas(&ctx.style()).show(ui, |ui| {
                ui.add_sized(
                    Vec2::splat(height / 3.0),
                    AttitudeIndicator::new(attitude.pitch, attitude.roll),
                );
            });

            Frame::canvas(&ctx.style()).show(ui, |ui| {
                ui.add_sized(
                    Vec2::splat(height / 3.0),
                    AttitudeIndicatorRectangular::new(attitude.pitch, attitude.roll),
                );
            });

            Frame::canvas(&ctx.style()).show(ui, |ui| {
                ui.add_sized(Vec2::splat(height / 3.0), HeadingIndicator::new(attitude.heading))
            });
        });
    }
}
