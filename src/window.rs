use eframe::egui;
pub struct MainWindow {}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {});
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("View", |ui| {});
                ui.menu_button("Tools", |ui| {});
                ui.menu_button("Help", |ui| {});
            })
        });
    }
}
