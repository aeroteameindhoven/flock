use std::{env::temp_dir, path::PathBuf, thread};

use eframe::epaint::ColorImage;
use ndarray::{s, Array3, Ix3};
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use video_rs::Encoder;

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

    video_rs::init().expect("video_rs should initialize");

    let (image_sender, image_receiver) = std::sync::mpsc::channel();

    let recorder = thread::spawn(move || recorder(image_receiver));

    eframe::run_native(
        "Aero Flock",
        eframe::NativeOptions {
            // icon_data: (),
            app_id: Some("nl.aeroteameindhoven.Flock".to_string()),
            ..Default::default()
        },
        Box::new(|ctx| Box::new(window::MainWindow::new(ctx, image_sender))),
    )?;

    recorder.join().unwrap();

    Ok(())
}

fn recorder(image_receiver: std::sync::mpsc::Receiver<ColorImage>) {
    let filename = PathBuf::from("./video.mkv");

    let (window_width, window_height) = (1920, 1080);

    let mut encoder = Encoder::new(
        &video_rs::Locator::Path(filename),
        video_rs::EncoderSettings::for_h264_yuv420p(window_width, window_height, false),
    )
    .expect("encoder should initialize");

    let duration = video_rs::Time::from_nth_of_a_second(10);
    let mut position = video_rs::Time::zero();

    let mut pixels: Array3<u8> = Array3::zeros((window_height, window_width, 3));

    for image in image_receiver.iter() {
        let [width, height] = dbg!(image.size);

        pixels.slice_mut(s![..height, ..width, ..]).assign(
            &Array3::from_shape_vec(
                (height, width, 3),
                image
                    .pixels
                    .into_iter()
                    .flat_map(|pixel| [pixel.r(), pixel.g(), pixel.b()])
                    .collect(),
            )
            .unwrap(),
        );

        // for y in 0..width {
        //     for x in 0..height {
        //         let color = image.pixels[x + y * width];

        //         pixels[Ix3(y, x, 0)] = color.r();
        //         pixels[Ix3(y, x, 1)] = color.g();
        //         pixels[Ix3(y, x, 2)] = color.b();
        //     }
        // }

        // let pixels =

        dbg!(pixels.dim());

        encoder.encode(&pixels, &position).unwrap();

        position = position.aligned_with(&duration).add();
    }

    encoder.finish().unwrap();
}
