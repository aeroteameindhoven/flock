use std::borrow::Cow;

use eframe::{
    egui::{self, RawInput, Slider, TextureOptions},
    egui_glow::{self, check_for_gl_error},
    epaint::{Color32, ColorImage, Pos2, Rect, Vec2},
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

        let encoder_thread = std::thread::spawn(move || crate::recorder::recorder(image_receiver));

        crate::SPAWNED_THREADS.write().unwrap().push(encoder_thread);

        let gl = frame.gl().unwrap().clone();

        let framebuffer = unsafe {
            let framebuffer = gl.create_framebuffer().unwrap();
            // gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            framebuffer
        };

        let renderbuffer = unsafe {
            let renderbuffer = gl.create_renderbuffer().unwrap();
            // gl.bind_renderbuffer(glow::RENDERBUFFER, Some(renderbuffer));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::RGB32F, 1920, 1080);
            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::RENDERBUFFER,
                Some(renderbuffer),
            );
            renderbuffer
        };

        let status = unsafe { gl.check_framebuffer_status(glow::FRAMEBUFFER) };

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
            painter: egui_glow::Painter::new(gl, "", None).unwrap(),
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

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("AA");
                });
            },
        );
        let clipped_primitives = offscreen_context.tessellate(offscreen_output.shapes);

        unsafe {
            let gl = self.painter.gl();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(self.renderbuffer));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::RGBA32F, 1920, 1080);
            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::RENDERBUFFER,
                Some(self.renderbuffer),
            );

            let status = unsafe { gl.check_framebuffer_status(glow::FRAMEBUFFER) };

            if status != glow::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete");
            }

            gl.viewport(0, 0, 1920, 1080);
            gl.color_mask(true, true, true, true);
            gl.clear_color(0.0, 1.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
        self.painter
            .paint_primitives([1920, 1080], 1.0, &clipped_primitives);
        unsafe {
            let gl = self.painter.gl();
            gl.finish();
        }
        let pixels = self.painter.read_screen_rgba([1920, 1080]);
        unsafe {
            let gl = self.painter.gl();
            let screen_rect = ctx.screen_rect();
            gl.bind_framebuffer(glow::FRAMEBUFFER, self.painter.intermediate_fbo());
            gl.viewport(
                screen_rect.min.x.round() as _,
                screen_rect.min.y.round() as _,
                screen_rect.width().round() as _,
                screen_rect.height().round() as _,
            );
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
