use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod window;
pub mod component;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::filter_fn(|meta| {
            meta.target().starts_with(env!("CARGO_PKG_NAME"))
        }))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            ..Default::default()
        },
        Box::new(|ctx| Box::new(window::MainWindow { heading: 115.0 })),
    )
}
