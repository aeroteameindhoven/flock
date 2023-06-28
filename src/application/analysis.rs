use std::borrow::Cow;

use eframe::{
    egui::{self, RawInput, Slider, TextureOptions},
    egui_glow::{self, check_for_gl_error},
    epaint::{Color32, ColorImage, ImageDelta, Pos2, Rect, Vec2},
    glow::{self, HasContext},
};

use crate::{
    component::{attitude::AttitudeIndicator, heading::HeadingIndicator},
    window,
};

use super::Application;

pub struct Analysis {
    heading: f32,
    pitch: f32,
    roll: f32,

    timeline_progress: f32,

    framebuffer: glow::NativeFramebuffer,
    renderbuffer: glow::NativeRenderbuffer,
    painter: egui_glow::Painter,
    image_sender: std::sync::mpsc::Sender<ColorImage>,
}

impl Analysis {
    pub fn create(ctx: &egui::Context, frame: &mut eframe::Frame) -> Self {
        let (image_sender, image_receiver) = std::sync::mpsc::channel();

        let encoder_thread = std::thread::spawn(move || crate::recording::encoder(image_receiver));

        crate::SPAWNED_THREADS.write().unwrap().push(encoder_thread);

        let gl = frame.gl().unwrap().clone();

        let framebuffer = unsafe {
            let framebuffer = gl.create_framebuffer().unwrap();
            check_for_gl_error!(&gl, "create_framebuffer");

            // gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            framebuffer
        };

        let renderbuffer = unsafe {
            let renderbuffer = gl.create_renderbuffer().unwrap();
            check_for_gl_error!(&gl, "create_renderbuffer");

            // gl.bind_renderbuffer(glow::RENDERBUFFER, Some(renderbuffer));
            // gl.renderbuffer_storage(glow::RENDERBUFFER, glow::RGBA8, 1920, 1080);
            // check_for_gl_error!(&gl, "renderbuffer_storage");

            // gl.framebuffer_renderbuffer(
            //     glow::FRAMEBUFFER,
            //     glow::COLOR_ATTACHMENT0,
            //     glow::RENDERBUFFER,
            //     Some(renderbuffer),
            // );
            // check_for_gl_error!(&gl, "framebuffer_renderbuffer");

            renderbuffer
        };

        let status = unsafe { gl.check_framebuffer_status(glow::FRAMEBUFFER) };
        check_for_gl_error!(&gl, "check_framebuffer_status");

        dbg!(
            glow::FRAMEBUFFER_COMPLETE,
            glow::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
            glow::FRAMEBUFFER_INCOMPLETE_DIMENSIONS,
            glow::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
            glow::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
            glow::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
            glow::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
            glow::FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
        );
        dbg!(status);

        if status != glow::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer is not complete");
        }

        Self {
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            timeline_progress: 0.25,

            framebuffer,
            renderbuffer,
            painter: {
                let mut painter = egui_glow::Painter::new(gl, "", None).unwrap();
                // painter.set_texture(
                //     eframe::epaint::TextureId::Managed(0),
                //     &ImageDelta::full(egui::epaint::FontImage::new([0, 0]), Default::default()),
                // );
                painter
            },
            image_sender,
        }
    }
}

impl Drop for Analysis {
    fn drop(&mut self) {
        self.painter.destroy();
    }
}

impl Application for Analysis {
    fn title(&self) -> Cow<str> {
        Cow::Borrowed("Analysis")
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Application>> {
        let offscreen_context = egui::Context::default();

        let mut textures_delta = offscreen_context.tex_manager().write().take_delta();

        // offscreen_context.set_request_repaint_callback(callback)
        let offscreen_output = offscreen_context.run(
            RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0.0, 0.0),
                    Vec2::new(1920.0, 1080.0),
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
        let clipped_primitives = offscreen_context.tessellate(offscreen_output.shapes);
        textures_delta.append(offscreen_context.tex_manager().write().take_delta());

        unsafe {
            let gl = self.painter.gl();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(self.renderbuffer));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::RGBA8, 1920, 1080);
            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::RENDERBUFFER,
                Some(self.renderbuffer),
            );

            let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);

            if status != glow::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete");
            }

            egui_glow::painter::clear(gl, [1920, 1080], [0.0, 0.0, 1.0, 1.0]);
        }
        self.painter.paint_and_update_textures(
            [1920, 1080],
            1.0,
            &clipped_primitives,
            &textures_delta,
        );
        unsafe {
            let gl = self.painter.gl();
            gl.finish();
        }
        let pixels = self.painter.read_screen_rgba([1920, 1080]);

        unsafe {
            let gl = self.painter.gl();
            gl.bind_framebuffer(glow::FRAMEBUFFER, self.painter.intermediate_fbo());
        }
        self.image_sender
            .send(pixels)
            .expect("image sender should always be listening");

        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.button("⏪");
                ui.button("⏴");
                ui.button("⏸");
                ui.button("⏵");
                ui.button("⏩");
                ui.scope(|ui| {
                    ui.style_mut().spacing.slider_width = ui.available_width();
                    ui.add(
                        Slider::new(&mut self.timeline_progress, 0.0..=100.0)
                            .show_value(false)
                            .trailing_fill(true),
                    )
                });
            });

            egui::Area::new("Attitude").show(ctx, |ui| {
                ui.add_sized(
                    Vec2::splat(50.0),
                    AttitudeIndicator::new(self.pitch, self.roll),
                );
            });

            egui::Area::new("Heading").constrain(true).show(ctx, |ui| {
                ui.add_sized(Vec2::splat(50.0), HeadingIndicator::new(self.heading));
            });
        });

        None
    }
}
