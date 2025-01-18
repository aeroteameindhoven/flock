use std::{net::TcpListener, sync::Arc, thread};

use eframe::egui::mutex::Mutex;
use tracing::{trace, warn};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use tungstenite::Message;
use window::Attitude;

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
        Box::new(|ctx| {
            let attitude = Arc::new(Mutex::new(Attitude::default()));

            thread::spawn({
                let attitude = attitude.clone();
                let request_repaint = {
                    let ctx = ctx.egui_ctx.clone();

                    move || ctx.request_repaint()
                };

                || websocket_thread(attitude, request_repaint)
            });

            Ok(Box::new(window::MainWindow::new(attitude)))
        }),
    )?;

    Ok(())
}

fn websocket_thread(attitude_mutex: Arc<Mutex<Attitude>>, request_repaint: impl Fn()) {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in server.incoming() {
        trace!("new TCP connection");
        let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
        trace!("TCP upgraded to websocket connection");

        loop {
            let msg = websocket.read().unwrap();

            if msg.is_close() {
                break;
            }

            trace!(?msg, "websocket message");

            if msg.is_binary() {
                warn!("invalid message type: binary");
                break;
            }

            if let Message::Text(msg) = msg {
                let attitude: Attitude = serde_json::from_str(msg.as_str()).unwrap();

                *attitude_mutex.lock() = attitude;
                request_repaint();
            }
        }

        trace!("websocket connection closed");
    }
}
