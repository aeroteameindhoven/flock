use egui_dock::Tree;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use window::BasicTab;

pub mod window;

fn main() -> Result<(), eframe::Error> {
    let tracing_collector = egui_tracing::EventCollector::default();
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_collector.clone())
        .init();

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            ..Default::default()
        },
        Box::new(|ctx| {
            Box::new(window::MainWindow {
                tracing_collector,
                tree: Tree::new(vec![
                    Box::new(BasicTab {
                        title: "A".into(),
                        ui: |ui| {},
                    }),
                    Box::new(BasicTab {
                        title: "B".into(),
                        ui: |ui| {
                            ui.label("peepee");
                        },
                    }),
                    Box::new(BasicTab {
                        title: "C".into(),
                        ui: |ui| {
                            ui.label("poo poo");
                        },
                    }),
                ]),
            })
        }),
    )
}
