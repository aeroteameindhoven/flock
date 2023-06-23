use eframe::{
    egui::{Sense, Widget},
    emath::Align2,
    epaint::{FontId, HsvaGamma, Pos2, Shape, Stroke, Vec2},
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

        let red = HsvaGamma {
            h: 0.0,
            s: 1.0,
            v: 0.8,
            a: 1.0,
        };

        let white = HsvaGamma {
            h: 0.0,
            s: 0.0,
            v: 1.0,
            a: 1.0,
        };

        let ground = HsvaGamma {
            h: 0.36,
            s: 0.95,
            v: 0.64,
            a: 1.0,
        };

        let sky: HsvaGamma = HsvaGamma {
            h: 0.61,
            s: 0.8,
            v: 0.6,
            a: 1.0,
        };

        let (pitch, circle_color, other_color) = if self.pitch > -90.0 && self.pitch <= 90.0 {
            (self.pitch, sky, ground)
        } else {
            (self.pitch + 180.0, ground, sky)
        };

        let left_angle = (pitch + self.roll).rem_euclid(360.0);
        let right_angle = (self.roll - pitch + 180.0).rem_euclid(360.0);

        let left_point = bounds.center() + Vec2::angled(f32::to_radians(left_angle)) * radius;
        let right_point = bounds.center() + Vec2::angled(f32::to_radians(right_angle)) * radius;
        let center_point = Pos2::new(
            (left_point.x + right_point.x) / 2.0,
            (left_point.y + right_point.y) / 2.0,
        );

        let tangent = (left_point - right_point).normalized();
        let normal = tangent.rot90();

        painter.extend([
            // Background
            Shape::circle_filled(bounds.center(), radius, circle_color),
            // Horizon
            Shape::convex_polygon(
                [left_point, right_point]
                    .into_iter()
                    .chain(
                        std::iter::successors(Some(right_angle), |angle| {
                            let angle = (angle - 5.0).rem_euclid(360.0);

                            if f32::abs(angle - left_angle) <= 5.0 {
                                None
                            } else {
                                Some(angle)
                            }
                        })
                        .skip(1)
                        .map(|angle| {
                            bounds.center() + Vec2::angled(f32::to_radians(angle)) * radius
                        }),
                    )
                    .collect::<Vec<_>>(),
                other_color,
                Stroke::NONE,
            ),
            // Horizon degrees markers
            Shape::Vec(
                [-20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0]
                    .into_iter()
                    .map(|angle| {
                        Shape::line_segment(
                            {
                                let upwards = normal * radius * f32::sin(f32::to_radians(angle));

                                let half_width = if angle % 10.0 == 0.0 { 0.2 } else { 0.1 };

                                let inwards = tangent * radius * half_width;

                                [
                                    center_point + upwards - inwards,
                                    center_point + upwards + inwards,
                                ]
                            },
                            Stroke::new(radius * 0.01, white),
                        )
                    })
                    .collect(),
            ),
            // Red level markers
            Shape::line_segment(
                [
                    bounds.center() + Vec2::X * radius * 0.3,
                    bounds.center() + Vec2::X * radius * 0.7,
                ],
                Stroke::new(radius * 0.01, red),
            ),
            Shape::line_segment(
                [
                    bounds.center() - Vec2::X * radius * 0.3,
                    bounds.center() - Vec2::X * radius * 0.7,
                ],
                Stroke::new(radius * 0.01, red),
            ),
            // Red center marker
            Shape::line(
                vec![
                    bounds.center() + Vec2::angled(f32::to_radians(15.0)) * radius * 0.2,
                    bounds.center(),
                    bounds.center() + Vec2::angled(f32::to_radians(180.0 - 15.0)) * radius * 0.2,
                ],
                Stroke::new(radius * 0.01, red),
            ),
            // Pitch
            painter.fonts(|fonts| {
                Shape::Vec(vec![
                    Shape::text(
                        fonts,
                        bounds.center() + Vec2::X * radius * 0.5,
                        Align2::CENTER_CENTER,
                        format!("Pitch\n{:+04.0}°", self.pitch),
                        FontId::monospace(0.1 * radius),
                        white.into(),
                    ),
                    Shape::text(
                        fonts,
                        bounds.center() - Vec2::X * radius * 0.5,
                        Align2::CENTER_CENTER,
                        format!("Roll \n{:+04.0}°", self.roll),
                        FontId::monospace(0.1 * radius),
                        white.into(),
                    ),
                ])
            }),
        ]);

        {
            let markings: &[(&[f32], std::ops::RangeInclusive<f32>, f32)] = &[
                (&[-20.0, -10.0, 10.0, 20.0], 0.8..=0.9, 0.01),
                (
                    &[-90.0, -60.0, -30.0, 0.0, 30.0, 60.0, 90.0],
                    0.8..=1.0,
                    0.02,
                ),
            ];

            let markings = markings.iter().flat_map(|(angle, length, width)| {
                angle.iter().map(move |angle| {
                    Shape::line_segment(
                        [
                            bounds.center()
                                + Vec2::angled(f32::to_radians(angle - 90.0))
                                    * radius
                                    * *length.end(),
                            bounds.center()
                                + Vec2::angled(f32::to_radians(angle - 90.0))
                                    * radius
                                    * *length.start(),
                        ],
                        Stroke::new(radius * width, white),
                    )
                })
            });

            let dots = [-45.0, 45.0].iter().map(|angle| {
                Shape::circle_filled(
                    bounds.center() + Vec2::angled(f32::to_radians(angle - 90.0)) * radius * 0.9,
                    radius * 0.02,
                    white,
                )
            });

            painter.extend(markings.chain(dots));

            // Roll Indicator
            painter.add({
                let top = Vec2::angled(f32::to_radians(self.roll - 90.0));
                let bottom_left = Vec2::angled(f32::to_radians(self.roll - 95.0));
                let bottom_right = Vec2::angled(f32::to_radians(self.roll - 85.0));

                Shape::convex_polygon(
                    vec![
                        bounds.center() + top * radius * 0.85,
                        bounds.center() + bottom_right * radius * 0.7,
                        bounds.center() + bottom_left * radius * 0.7,
                    ],
                    red,
                    Stroke::NONE,
                )
            });
        }

        response
    }
}
