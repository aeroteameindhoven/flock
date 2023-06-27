use std::{io::Write, path::PathBuf};

use eframe::epaint::ColorImage;
use nix::unistd::Pid;
use video_rs::Encoder;

pub fn recorder(image_receiver: std::sync::mpsc::Receiver<ColorImage>) {
    ffmpeg_sidecar::download::auto_download().expect("ffmpeg should install");

    let filename = "./video.mkv";

    let (window_width, window_height) = (1920, 1080);

    let mut encoder = Encoder::new(
        &video_rs::Locator::Path(PathBuf::from(filename)),
        video_rs::EncoderSettings::for_h264_yuv420p(window_width, window_height, false),
    )
    .expect("encoder should initialize");

    let duration = video_rs::Time::from_nth_of_a_second(10);
    let mut position = video_rs::Time::zero();

    let mut command = ffmpeg_sidecar::command::FfmpegCommand::new()
        .hide_banner()
        .create_no_window()
        .pix_fmt("rgba")
        .format("rawvideo")
        .size(1920, 1080)
        .input("pipe:0")
        .no_audio()
        .filter("pad=1920:1080:(ow-iw)/2:(oh-ih)/2")
        .codec_video("libx264rgb")
        .pix_fmt("rgba")
        .overwrite()
        .output(filename)
        .print_command()
        .spawn()
        .expect("ffmpeg should spawn");

    let mut stdout = command.take_stdout().expect("stdout should be available");
    let mut stderr = command.take_stderr().expect("stdout should be available");

    std::thread::spawn(move || std::io::copy(&mut stdout, &mut std::io::stdout()));
    std::thread::spawn(move || std::io::copy(&mut stderr, &mut std::io::stdout()));

    let mut stdin = command.take_stdin().expect("stdin should be available");

    for image in image_receiver.iter() {
        let [width, height] = image.size;

        stdin.write_all(image.as_raw()).unwrap();

        // let mut pixels =
        //     ffmpeg_next::frame::Video::new(Pixel::RGBA, window_width as u32, window_height as u32);
        // pixels.data_mut(0).copy_from_slice(image.as_raw());
        // pixels.set_pts(position.clone().into_value());
        // // pixels.scaler(window_width, window_height, ffmpeg_next::software::scaling::Flags::)

        // encoder.encode_raw(pixels).unwrap();

        // pixels.slice_mut(s![..height, ..width, ..]).assign(
        //     &Array3::from_shape_vec(
        //         (height, width, 3),
        //         image
        //             .pixels
        //             .into_iter()
        //             .flat_map(|pixel| [pixel.r(), pixel.g(), pixel.b()])
        //             .collect(),
        //     )
        //     .unwrap(),
        // );

        // // for y in 0..width {
        // //     for x in 0..height {
        // //         let color = image.pixels[x + y * width];

        // //         pixels[Ix3(y, x, 0)] = color.r();
        // //         pixels[Ix3(y, x, 1)] = color.g();
        // //         pixels[Ix3(y, x, 2)] = color.b();
        // //     }
        // // }

        // // let pixels =

        // dbg!(pixels.dim());

        // encoder.encode(&pixels, &position).unwrap();

        position = position.aligned_with(&duration).add();
    }

    // encoder.finish().unwrap();

    drop(stdin);
    nix::sys::signal::kill(
        Pid::from_raw(command.as_inner().id() as _),
        nix::sys::signal::SIGTERM,
    )
    .unwrap();
    command.wait().unwrap();
}
