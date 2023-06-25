use eframe::{
    egui::{self},
    epaint::ColorImage,
};

use crate::application::{self, Application};

pub struct MainWindow {
    application: Box<dyn Application>,

    image_sender: std::sync::mpsc::Sender<ColorImage>,
}

impl MainWindow {
    pub fn new(
        ctx: &eframe::CreationContext,
        image_sender: std::sync::mpsc::Sender<ColorImage>,
    ) -> Self {
        Self {
            application: Box::new(application::start::Landing {}),
            image_sender,
        }
    }
}

impl eframe::App for MainWindow {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
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

    fn post_rendering(&mut self, window_size_px: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            self.image_sender
                .send(screenshot)
                .expect("image sender should always be listening");
        }
    }
}
