use std::{io::Write, path::PathBuf};

use eframe::{
    egui,
    egui_wgpu::{self, renderer::ScreenDescriptor},
    epaint::{ColorImage, Pos2, Rect, Vec2},
    wgpu::{self, TextureViewDescriptor},
};
use nix::unistd::Pid;
use pollster::FutureExt;
use video_rs::Encoder;

pub fn renderer() {
    let (width, height) = (1920, 1080);

    let gpu = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: wgpu::Dx12Compiler::default(),
    });

    let adapter = gpu
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("flock overlay renderer"),
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .block_on()
        .unwrap();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("flock overlay render target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 0,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
    });
    let texture_view = texture.create_view(&TextureViewDescriptor {
        label: Some("flock overlay render view"),
        ..Default::default()
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("flock overlay render output buffer"),
        size: ((std::mem::size_of::<u32>() as u32) * width * height) as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: true,
    });

    let mut renderer =
        egui_wgpu::renderer::Renderer::new(&device, wgpu::TextureFormat::Rgba8Unorm, None, 0);
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let context = egui::Context::default();
    let mut textures_delta = context.tex_manager().write().take_delta();

    let screen_descriptor = ScreenDescriptor {
        size_in_pixels: [width, height],
        pixels_per_point: 1.0,
    };

    let output = context.run(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0.0, 0.0),
                Vec2::new(
                    screen_descriptor.size_in_pixels[0] as _,
                    screen_descriptor.size_in_pixels[1] as _,
                ) * screen_descriptor.pixels_per_point,
            )),
            ..Default::default()
        },
        |ctx| {
            egui::TopBottomPanel::top("eepee").show(ctx, |ui| {
                ui.label("test");
            });

            // egui::CentralPanel::default().show(ctx, |ui| {
            //     ui.label("AA");
            // });
        },
    );
    let clipped_primitives = context.tessellate(output.shapes);
    textures_delta.append(context.tex_manager().write().take_delta());

    let command_buffers = renderer.update_buffers(
        &device,
        &queue,
        &mut command_encoder,
        &clipped_primitives,
        &screen_descriptor,
    );

    queue.submit(command_buffers);
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    });
    renderer.render(&mut render_pass, &clipped_primitives, &screen_descriptor);
    command_encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: u32_size * texture_size,
                rows_per_image: texture_size,
            },
        },
        texture_desc.size,
    );

    // TODO: render,
}

pub fn encoder(image_receiver: std::sync::mpsc::Receiver<ColorImage>) {
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
