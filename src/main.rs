use std::thread;

use eframe::epaint::ColorImage;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod application;
pub mod component;
pub mod window;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::filter_fn(|meta| {
            meta.target().starts_with(env!("CARGO_PKG_NAME"))
        }))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let (image_sender, image_receiver) = std::sync::mpsc::channel();

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            ..Default::default()
        },
        Box::new(|ctx| Box::new(window::MainWindow::new(ctx, image_sender))),
    )?;

    thread::spawn(move || recorder(image_receiver));

    Ok(())
}

fn recorder(image_receiver: std::sync::mpsc::Receiver<ColorImage>) {
    for image in image_receiver.iter() {
        dbg!(image.size);
    }
}
