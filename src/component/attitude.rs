use eframe::{
    egui::{Sense, Widget},
    epaint::{Color32, CubicBezierShape, HsvaGamma, Shape, Stroke, Vec2},
};

pub struct AttitudeIndicator {
    pitch: f32,
    roll: f32,
}

impl AttitudeIndicator {
    pub fn new(pitch: f32, roll: f32) -> Self {
        // Normalize -180 to 180
        // let pitch = f32::abs(pitch.rem_euclid(360.0));
        // let roll = f32::abs(roll.rem_euclid(360.0));

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

        painter.add(Shape::CubicBezier(CubicBezierShape {
            points: [
                bounds.center() + Vec2::ZERO,
                bounds.center() + Vec2::ZERO,
                bounds.center() + Vec2::ZERO,
                bounds.center() + Vec2::ZERO,
            ],
            closed: true,
            fill: Color32::DEBUG_COLOR,
            stroke: Stroke::NONE,
        }));

        // Shape::CubicBezier(CubicBezierShape::from_points_stroke([], closed, fill, stroke))

        response
    }
}
