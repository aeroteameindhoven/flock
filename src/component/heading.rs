use eframe::{
    egui::{Sense, Widget},
    emath::Align2,
    epaint::{FontId, Hsva, HsvaGamma, Rect, RectShape, Rounding, Shape, Stroke, Vec2},
};

pub struct HeadingIndicator {
    heading: f32,
}

impl HeadingIndicator {
    pub fn new(heading: f32) -> Self {
        let heading = f32::abs(heading.rem_euclid(360.0));

        HeadingIndicator { heading }
    }
}

impl Widget for HeadingIndicator {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let space = ui.available_size();
        let (response, painter) =
            ui.allocate_painter(Vec2::splat(space.min_elem()), Sense::hover());
        let bounds = response.rect;

        let size = f32::min(bounds.width(), bounds.height());
        let radius = size / 2.0;

        let background_color = Hsva::new(0.0, 0.0, 0.02, 1.0);
        let border_color = Hsva::new(0.0, 0.0, 1.0, 1.0);

        let front = {
            let (y, x) = f32::sin_cos(f32::to_radians(self.heading - 90.0));
            Vec2::new(x, y)
        };

        let rear_right = {
            let (y, x) = f32::sin_cos(f32::to_radians(self.heading + 150.0 - 90.0));
            Vec2::new(x, y)
        };

        let rear_left = {
            let (y, x) = f32::sin_cos(f32::to_radians(self.heading + 210.0 - 90.0));
            Vec2::new(x, y)
        };

        painter.extend([
            // Circle background
            Shape::circle_filled(bounds.center(), radius, Hsva::new(0.0, 0.0, 0.02, 1.0)),
        ]);

        let degrees = std::iter::successors(Some(0), |current| {
            let next = current + 10;

            (next < 360).then_some(next)
        });

        // Heading Markers
        painter.extend(degrees.map(|degree| {
            let (y, x) = f32::sin_cos(f32::to_radians(degree as _));
            let stroke_width = size / 100.0;

            if degree % 90 == 0 {
                Shape::line_segment(
                    [
                        bounds.center() + Vec2::new(x, y) * radius,
                        bounds.center() + Vec2::new(x, y) * radius * 0.8,
                    ],
                    (
                        stroke_width,
                        HsvaGamma {
                            h: 0.0,
                            s: 0.0,
                            v: 1.0,
                            a: 1.0,
                        },
                    ),
                )
            } else if degree % 30 == 0 {
                Shape::line_segment(
                    [
                        bounds.center() + Vec2::new(x, y) * radius,
                        bounds.center() + Vec2::new(x, y) * radius * 0.85,
                    ],
                    (
                        stroke_width,
                        HsvaGamma {
                            h: 0.0,
                            s: 0.0,
                            v: 0.8,
                            a: 1.0,
                        },
                    ),
                )
            } else {
                Shape::line_segment(
                    [
                        bounds.center() + Vec2::new(x, y) * radius,
                        bounds.center() + Vec2::new(x, y) * radius * 0.9,
                    ],
                    (
                        stroke_width,
                        HsvaGamma {
                            h: 0.0,
                            s: 0.0,
                            v: 0.4,
                            a: 1.0,
                        },
                    ),
                )
            }
        }));

        // Cardinal Directions
        let direction_color = HsvaGamma {
            h: 0.0,
            s: 0.0,
            v: 1.0,
            a: 1.0,
        };
        let direction_font = FontId::monospace(radius / 5.0);
        painter.extend(painter.fonts(|fonts| {
            [
                Shape::text(
                    fonts,
                    bounds.center() + -Vec2::Y * radius * 0.75,
                    Align2::CENTER_TOP,
                    "N",
                    direction_font.clone(),
                    direction_color.into(),
                ),
                Shape::text(
                    fonts,
                    bounds.center() + Vec2::Y * radius * 0.75,
                    Align2::CENTER_BOTTOM,
                    "S",
                    direction_font.clone(),
                    direction_color.into(),
                ),
                Shape::text(
                    fonts,
                    bounds.center() + Vec2::X * radius * 0.75,
                    Align2::RIGHT_CENTER,
                    "E",
                    direction_font.clone(),
                    direction_color.into(),
                ),
                Shape::text(
                    fonts,
                    bounds.center() + -Vec2::X * radius * 0.75,
                    Align2::LEFT_CENTER,
                    "W",
                    direction_font.clone(),
                    direction_color.into(),
                ),
            ]
        }));

        painter.extend([
            // Arrow heading indicator
            Shape::convex_polygon(
                vec![
                    bounds.center() + front * radius / 2.0,
                    bounds.center() + rear_left * radius / 2.0, // Vec2::new(-size / 6.0, size / 4.0),
                    bounds.center() + front * -radius / 4.0,
                ],
                Hsva::new(0.0, 1.0, 0.8, 1.0),
                Stroke::NONE,
            ),
            Shape::convex_polygon(
                vec![
                    bounds.center() + front * radius / 2.0,
                    bounds.center() + rear_right * radius / 2.0,
                    bounds.center() + front * -radius / 4.0,
                ],
                Hsva::new(0.0, 1.0, 0.8, 1.0),
                Stroke::NONE,
            ),
            Shape::line(
                vec![
                    bounds.center() + front * -radius / 4.0,
                    bounds.center() + rear_left * radius / 2.0,
                    bounds.center() + front * radius / 2.0,
                    bounds.center() + rear_right * radius / 2.0,
                    bounds.center() + front * -radius / 4.0,
                    bounds.center() + front * radius,
                ],
                (size / 100.0, Hsva::new(0.0, 0.9, 0.5, 1.0)),
            ),
            // Heading text box
            Shape::Rect(RectShape {
                rect: Rect::from_center_size(bounds.center(), Vec2::new(size * 0.40, size * 0.25)),
                rounding: Rounding::same(size * 0.05),
                fill: Hsva {
                    a: 0.75,
                    ..background_color
                }
                .into(),
                stroke: (
                    0.5,
                    Hsva {
                        a: 0.75,
                        ..border_color
                    },
                )
                    .into(),
            }),
            painter.fonts(|fonts| {
                Shape::text(
                    fonts,
                    bounds.center(),
                    Align2::CENTER_CENTER,
                    format!("{:03.0}°", self.heading),
                    FontId::monospace(bounds.height() * 0.15),
                    Hsva {
                        a: 0.75,
                        ..border_color
                    }
                    .into(),
                )
            }),
            Shape::circle_stroke(bounds.center(), radius, (radius / 150.0, border_color)),
        ]);

        response
    }
}
