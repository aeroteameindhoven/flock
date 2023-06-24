use std::borrow::Cow;

use eframe::egui;

pub mod analysis;
pub mod live;
pub mod start;

pub trait Application {
    fn title(&self) -> Cow<str>;

    #[must_use]
    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Application>>;
}
