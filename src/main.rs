use std::{sync::RwLock, thread};

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod application;
pub mod component;
pub mod recorder;
pub mod window;

pub static SPAWNED_THREADS: RwLock<Vec<thread::JoinHandle<()>>> = RwLock::new(Vec::new());

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::filter_fn(|meta| {
            meta.target().starts_with(env!("CARGO_PKG_NAME"))
        }))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    video_rs::init().expect("video_rs should initialize");

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            ..Default::default()
        },
        Box::new(|_ctx| Box::new(window::MainWindow::new())),
    )?;

    for thread in SPAWNED_THREADS.write().unwrap().drain(..) {
        thread.join().unwrap();
    }

    Ok(())
}
