use std::borrow::Cow;

use eframe::{
    egui::{CentralPanel, RichText},
    epaint::{FontId, Vec2},
};

use super::{analysis::Analysis, live::Live, Application};

pub struct Landing {}

impl Application for Landing {
    fn title(&self) -> Cow<str> {
        Cow::Borrowed("Welcome")
    }

    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Application>> {
        let mut app: Option<Box<dyn Application>> = None;

        CentralPanel::default().show(ctx, |ui| {
            let styles = ui.style_mut();
            styles
                .text_styles
                .entry(eframe::egui::TextStyle::Button)
                .and_modify(|font| font.size = 32.0);
            styles.spacing.button_padding = Vec2::new(64.0, 32.0);

            ui.vertical_centered_justified(|ui| {
                ui.columns(2, |columns| {
                    if columns[0].button("Live View").clicked() {
                        app = Some(Box::new(Live::new()))
                    }
                    if columns[1].button("Post Analysis").clicked() {
                        app = Some(Box::new(Analysis::create()));
                    }
                })
            });

            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("😺☺😎").font(FontId::proportional(100.0)));
            })
        });

        app
    }
}
