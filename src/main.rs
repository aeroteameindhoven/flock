use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use eframe::egui::mutex::Mutex;
use tracing::{debug, trace, warn};
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
            let attitude = Arc::new(Mutex::new(Attitude {
                heading: 200.0,
                pitch: 10.0,
                roll: 10.0,
            }));
            let connection = Arc::new(AtomicBool::new(false));

            thread::spawn({
                let attitude = attitude.clone();
                let connection = connection.clone();
                let request_repaint = {
                    let ctx = ctx.egui_ctx.clone();

                    move || ctx.request_repaint()
                };

                || websocket_thread(attitude, connection, request_repaint)
            });

            Ok(Box::new(window::MainWindow::new(attitude, connection)))
        }),
    )?;

    Ok(())
}

fn websocket_thread(
    attitude_mutex: Arc<Mutex<Attitude>>,
    connection: Arc<AtomicBool>,
    request_repaint: impl Fn(),
) {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in server.incoming() {
        trace!("new TCP connection");
        let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
        trace!("TCP upgraded to websocket connection");
        connection.store(true, Ordering::Release);
        request_repaint();

        loop {
            match websocket.read() {
                Ok(Message::Close(_)) => {
                    break;
                }
                Ok(Message::Binary(_)) => {
                    warn!("invalid message type: binary");
                    break;
                }
                Ok(Message::Text(text)) => {
                    let attitude: Attitude = serde_json::from_str(text.as_str()).unwrap();

                    // trace!(?attitude, "websocket message");

                    *attitude_mutex.lock() = attitude;
                    request_repaint();
                }
                Ok(_) => {}
                Err(tungstenite::Error::ConnectionClosed) => {
                    break;
                }
                Err(tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    debug!("reset without closing handshake. Likely iOS tab unfocused");

                    break;
                }
                Err(error) => {
                    warn!(%error, "error in websocket connection");
                    break;
                }
            }
        }

        connection.store(false, Ordering::Release);
        request_repaint();

        trace!("websocket connection closed");
    }
}
