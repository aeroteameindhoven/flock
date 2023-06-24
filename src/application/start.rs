use std::borrow::Cow;

use eframe::{egui::CentralPanel, epaint::Vec2};

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
                        app = Some(Box::new(Live {
                            heading: 0.0,
                            pitch: 0.0,
                            roll: 0.0,
                        }))
                    }
                    if columns[1].button("Post Analysis").clicked() {
                        app = Some(Box::new(Analysis {
                            heading: 0.0,
                            pitch: 0.0,
                            roll: 0.0,

                            timeline_progress: 5.0,
                        }));
                    }
                })
            });
        });

        app
    }
}
