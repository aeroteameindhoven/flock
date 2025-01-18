use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eframe::egui::{self, mutex::Mutex, Color32, Frame, RichText};

use crate::component::{
    attitude::{AttitudeIndicator, AttitudeIndicatorRectangular},
    heading::HeadingIndicator,
};

pub struct MainWindow {
    attitude: Arc<Mutex<Attitude>>,
    connection: Arc<AtomicBool>,
}

#[derive(Debug, Default, Clone, Copy, serde::Deserialize)]
pub struct Attitude {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl MainWindow {
    pub fn new(attitude: Arc<Mutex<Attitude>>, connection: Arc<AtomicBool>) -> Self {
        Self {
            attitude,
            connection,
        }
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
        let attitude = self.attitude.lock();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Websocket status: ");
                ui.label(if self.connection.load(Ordering::Acquire) {
                    RichText::new("connected").color(Color32::GREEN)
                } else {
                    RichText::new("disconnected").color(Color32::RED)
                });
            });

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

            ui.columns_const(|[one, two, three]| {
                Frame::canvas(&ctx.style()).show(one, |ui| {
                    ui.add(AttitudeIndicator::new(attitude.pitch, attitude.roll));
                });

                Frame::canvas(&ctx.style()).show(two, |ui| {
                    ui.add(AttitudeIndicatorRectangular::new(
                        attitude.pitch,
                        attitude.roll,
                    ));
                });

                Frame::canvas(&ctx.style())
                    .show(three, |ui| ui.add(HeadingIndicator::new(attitude.heading)));
            });
        });
    }
}
