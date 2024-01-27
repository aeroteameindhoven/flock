use std::borrow::Cow;

use eframe::egui::{self, Frame, Slider, Ui, ViewportBuilder, ViewportId, WidgetText};

use crate::component::{attitude::AttitudeIndicator, heading::HeadingIndicator};

use super::Application;

pub struct Live {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,

    attitude_display: DisplayState,
    heading_display: DisplayState,
}

impl Live {
    pub fn new() -> Self {
        Live {
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,

            attitude_display: DisplayState::Viewport,
            heading_display: DisplayState::Viewport,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayState {
    Closed,
    Window,
    Viewport,
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
            ui.heading("Windows");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    window_controls(ui, &mut self.attitude_display);
                    ui.label("Attitude");
                });
                ui.group(|ui| {
                    window_controls(ui, &mut self.heading_display);
                    ui.label("Heading");
                });
            });

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

        window(
            ctx,
            &mut self.attitude_display,
            "Attitude Indicator",
            |ui| {
                Frame::canvas(&ctx.style()).show(ui, |ui| {
                    ui.add(AttitudeIndicator::new(self.pitch, self.roll));
                });
            },
        );

        window(ctx, &mut self.heading_display, "Heading Indicator", |ui| {
            Frame::canvas(&ctx.style()).show(ui, |ui| ui.add(HeadingIndicator::new(self.heading)));
        });

        None
    }
}

fn window_controls(ui: &mut Ui, display: &mut DisplayState) {
    ui.selectable_value(display, DisplayState::Closed, "🗕");
    ui.selectable_value(display, DisplayState::Window, "🗖");
    ui.selectable_value(display, DisplayState::Viewport, "⬈");
}

fn window(
    ctx: &egui::Context,
    display: &mut DisplayState,
    title: impl Into<WidgetText>,
    add_contents: impl FnOnce(&mut Ui),
) {
    let title: WidgetText = title.into();
    let title_text = title.text();

    match display {
        DisplayState::Closed => {}
        DisplayState::Window => {
            egui::Window::new(title).collapsible(false).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    window_controls(ui, display);
                });

                add_contents(ui);
            });
        }
        DisplayState::Viewport => {
            ctx.show_viewport_immediate(
                ViewportId::from_hash_of(title_text),
                ViewportBuilder::default()
                    .with_title(title_text)
                    .with_resizable(false),
                |ctx, class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if ui.input(|i| i.viewport().close_requested()) {
                            *display = DisplayState::Window;
                        }

                        ui.horizontal(|ui| {
                            window_controls(ui, display);
                        });

                        add_contents(ui);
                    })
                },
            );
        }
    }
}
