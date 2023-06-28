use std::{sync::RwLock, thread};

use eframe::Renderer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod application;
pub mod component;
pub mod recording;
pub mod window;

pub static SPAWNED_THREADS: RwLock<Vec<thread::JoinHandle<()>>> = RwLock::new(Vec::new());

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::filter_fn(|meta| {
            // if *meta.level() == tracing::Level::ERROR {
            //     println!("{}", std::backtrace::Backtrace::force_capture());
            //     unreachable!();
            // }

            meta.target().starts_with(env!("CARGO_PKG_NAME"))
                || (tracing::level_filters::LevelFilter::WARN >= *meta.level())
        }))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    video_rs::init().expect("video_rs should initialize");

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            renderer: Renderer::Wgpu,
            ..Default::default()
        },
        Box::new(|_ctx| Box::new(window::MainWindow::new())),
    )?;

    for thread in SPAWNED_THREADS.write().unwrap().drain(..) {
        thread.join().unwrap();
    }

    Ok(())
}
