use std::borrow::Cow;

use eframe::{
    egui::{self, Slider},
    epaint::Vec2,
};

use crate::component::{attitude::AttitudeIndicator, heading::HeadingIndicator};

use super::Application;

pub struct Analysis {
    heading: f32,
    pitch: f32,
    roll: f32,

    timeline_progress: f32,
}

impl Analysis {
    pub fn create() -> Self {
        Self {
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            timeline_progress: 0.25,
        }
    }
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
