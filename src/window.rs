use eframe::egui;

use crate::application::{self, Application};

pub struct MainWindow {
    application: Box<dyn Application>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            application: Box::new(application::start::Landing {}),
        }
    }
}

impl eframe::App for MainWindow {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [1.0, 0.0, 0.0, 1.0]
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("File", |ui| {});
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("View", |ui| {});
                ui.menu_button("Tools", |ui| {});
                ui.menu_button("Help", |ui| {});
                ui.label(format!("Frame: {}", ctx.frame_nr()));
            })
        });

        if let Some(application) = self.application.update(ctx, frame) {
            frame.set_window_title(application.title().as_ref());
            self.application = application;
        }
    }
}
