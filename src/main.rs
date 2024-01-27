use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod application;
pub mod component;
pub mod window;

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

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            viewport: eframe::egui::ViewportBuilder::default()
                .with_app_id("nl.aeroteameindhoven.Flock"),
            ..Default::default()
        },
        Box::new(|_ctx| Box::new(window::MainWindow::new())),
    )?;

    Ok(())
}
