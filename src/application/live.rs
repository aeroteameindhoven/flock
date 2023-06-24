use std::borrow::Cow;

use eframe::egui::{self, Frame, Slider};

use crate::component::{attitude::AttitudeIndicator, heading::HeadingIndicator};

use super::Application;

pub struct Live {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Application for Live {
    fn title(&self) -> Cow<str> {
        Cow::Borrowed("Live")
    }
    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Application>> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                Slider::new(&mut self.heading, -360.0..=720.0)
                    .text("Heading")
                    .integer()
                    .step_by(5.0)
                    .trailing_fill(true),
            );
            ui.add(
                Slider::new(&mut self.roll, -360.0..=360.0)
                    .text("Roll")
                    .integer()
                    .step_by(1.0)
                    .trailing_fill(true),
            );
            ui.add(
                Slider::new(&mut self.pitch, -360.0..=360.0)
                    .text("Pitch")
                    .integer()
                    .step_by(1.0)
                    .trailing_fill(true),
            );
        });

        egui::Window::new("Attitude Indicator").show(ctx, |ui| {
            Frame::canvas(&ctx.style()).show(ui, |ui| {
                ui.add(AttitudeIndicator::new(self.pitch, self.roll))
            });
        });

        egui::Window::new("Heading Indicator")
            .constrain(true)
            .show(ctx, |ui| {
                Frame::canvas(&ctx.style())
                    .show(ui, |ui| ui.add(HeadingIndicator::new(self.heading)));
            });

        None
    }
}
