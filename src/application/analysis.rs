use std::borrow::Cow;

use eframe::{
    egui::{self, Slider},
    epaint::Vec2,
};

use crate::component::{attitude::AttitudeIndicator, heading::HeadingIndicator};

use super::Application;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Analysis {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,

    pub timeline_progress: f32,
}

impl Application for Analysis {
    fn title(&self) -> Cow<str> {
        Cow::Borrowed("Analysis")
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Application>> {
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
                        Slider::new(&mut self.timeline_progress, 0.0..=100.0)
                            .show_value(false)
                            .trailing_fill(true),
                    )
                });
            });

            egui::Area::new("Attitude").show(ctx, |ui| {
                ui.add_sized(
                    Vec2::splat(50.0),
                    AttitudeIndicator::new(self.pitch, self.roll),
                );
            });

            egui::Area::new("Heading").constrain(true).show(ctx, |ui| {
                ui.add_sized(Vec2::splat(50.0), HeadingIndicator::new(self.heading));
            });
        });

        None
    }
}
