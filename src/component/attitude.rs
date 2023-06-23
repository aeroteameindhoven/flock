use eframe::{
    egui::{Sense, TextFormat, Widget},
    emath::{Align, Align2},
    epaint::{
        text::{LayoutJob, LayoutSection, TextWrapping},
        Color32, CubicBezierShape, FontId, HsvaGamma, Pos2, Shape, Stroke, TextShape, Vec2,
    },
};

pub struct AttitudeIndicator {
    pitch: f32,
    roll: f32,
}

impl AttitudeIndicator {
    pub fn new(pitch: f32, roll: f32) -> Self {
        // Normalize -180-180
        let pitch = f32::abs((pitch + 180.0).rem_euclid(360.0)) - 180.0;
        let roll = f32::abs((roll + 180.0).rem_euclid(360.0)) - 180.0;

        Self { pitch, roll }
    }
}

impl Widget for AttitudeIndicator {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let space = ui.available_size();
        let (response, painter) =
            ui.allocate_painter(Vec2::splat(space.min_elem()), Sense::hover());
        let bounds = response.rect;

        let size = f32::min(bounds.width(), bounds.height());
        let radius = size / 2.0;

        painter.circle_filled(
            bounds.center(),
            radius,
            HsvaGamma {
                h: 0.61,
                s: 0.8,
                v: 0.6,
                a: 1.0,
            },
        );

        if (-90.0..=90.0).contains(&self.pitch) {
            let (y_left, x_left) = f32::sin_cos(f32::to_radians(self.pitch + self.roll));
            let (y_right, x_right) = f32::sin_cos(f32::to_radians(self.pitch - self.roll));

            painter.extend([
                Shape::LineSegment {
                    points: [
                        bounds.center() + Vec2::new(x_right, y_right) * radius,
                        bounds.center() + Vec2::new(-x_left, y_left) * radius,
                    ],
                    stroke: Stroke::new(2.0, Color32::DEBUG_COLOR),
                },
                painter.fonts(|fonts| {
                    let text = format!("p{:03.0}°\nr{:03.0}°", self.pitch, self.roll);
                    let galley = fonts.layout_job(LayoutJob {
                        sections: vec![LayoutSection {
                            byte_range: 0..text.bytes().len(),
                            format: TextFormat {
                                font_id: FontId::monospace(bounds.height() * 0.05),
                                color: Color32::DEBUG_COLOR,
                                ..Default::default()
                            },
                            leading_space: 0.0,
                        }],
                        halign: Align::Center,
                        text,
                        ..Default::default()
                    });

                    Shape::Text({
                        TextShape {
                            pos: bounds.center()
                                + Vec2::new((x_right - x_left) / 2.0, (y_left + y_right) / 2.0)
                                    * radius,
                            galley,
                            underline: Stroke::NONE,
                            override_text_color: None,
                            angle: f32::to_radians(-self.roll),
                        }
                    })
                }),
            ]);
        }
        // Shape::CubicBezier(CubicBezierShape::from_points_stroke([], closed, fill, stroke))

        response
    }
}
