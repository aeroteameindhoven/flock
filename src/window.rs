use std::any::{Any, TypeId};

use eframe::egui;
use egui_dock::{node, Node, Tree};
use egui_tracing::{ui, EventCollector};

pub struct MainWindow {
    pub tracing_collector: EventCollector,
    pub tree: Tree<Box<dyn Tab>>,
}

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

        egui_dock::DockArea::new(&mut self.tree)
            .show_add_buttons(true)
            .show_add_popup(true)
            .show(ctx, &mut TabViewer { tabs_to_add })
    }
}

pub struct LogsTab {
    tracing_collector: EventCollector,
}

impl Tab for LogsTab {
    fn title(&self) -> egui::WidgetText {
        "Logs".into()
    }

    fn ui(&self, ui: &mut egui::Ui) {
        ui.add(egui_tracing::Logs::new(self.tracing_collector.clone()));
    }
}

pub trait Tab: Any {
    fn title(&self) -> egui::WidgetText;
    fn ui(&self, ui: &mut egui::Ui);
}

pub struct BasicTab {
    pub title: egui::WidgetText,
    pub ui: fn(ui: &mut egui::Ui),
}

impl Tab for BasicTab {
    fn title(&self) -> egui::WidgetText {
        self.title.clone()
    }

    fn ui(&self, ui: &mut egui::Ui) {
        (self.ui)(ui)
    }
}

struct TabViewer {
    tabs_to_add: Vec<Box<dyn Tab>>,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Box<dyn Tab>;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title()
    }

    fn add_popup(&mut self, ui: &mut egui::Ui, node: egui_dock::NodeIndex) {
        if ui.button("Logs View").clicked() {
            self.tabs_to_add.push(Box::new(LogsTab {
                tracing_collector: self.tracing_collector.clone(),
            }))
        };
    }
}
