use eframe::{
    egui::{Rounding, Sense, Widget},
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

const RED: HsvaGamma = HsvaGamma {
    h: 0.0,
    s: 1.0,
    v: 0.8,
    a: 1.0,
};

const WHITE: HsvaGamma = HsvaGamma {
    h: 0.0,
    s: 0.0,
    v: 1.0,
    a: 1.0,
};

const GROUND: HsvaGamma = HsvaGamma {
    h: 0.36,
    s: 0.95,
    v: 0.64,
    a: 1.0,
};

const SKY: HsvaGamma = HsvaGamma {
    h: 0.61,
    s: 0.8,
    v: 0.6,
    a: 1.0,
};

impl Widget for AttitudeIndicator {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let space = ui.available_size();
        let (response, painter) = ui.allocate_painter(
            Vec2::splat(space.min_elem()),
            Sense::focusable_noninteractive(),
        );
        let bounds = response.rect;

        let size = f32::min(bounds.width(), bounds.height());
        let radius = size / 2.0;

        let (pitch, circle_color, other_color) = if self.pitch > -90.0 && self.pitch <= 90.0 {
            (self.pitch, SKY, GROUND)
        } else {
            (self.pitch + 180.0, GROUND, SKY)
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
                            Stroke::new(radius * 0.01, WHITE),
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
                Stroke::new(radius * 0.01, RED),
            ),
            Shape::line_segment(
                [
                    bounds.center() - Vec2::X * radius * 0.3,
                    bounds.center() - Vec2::X * radius * 0.7,
                ],
                Stroke::new(radius * 0.01, RED),
            ),
            // Red center marker
            Shape::line(
                vec![
                    bounds.center() + Vec2::angled(f32::to_radians(15.0)) * radius * 0.2,
                    bounds.center(),
                    bounds.center() + Vec2::angled(f32::to_radians(180.0 - 15.0)) * radius * 0.2,
                ],
                Stroke::new(radius * 0.01, RED),
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
                        WHITE.into(),
                    ),
                    Shape::text(
                        fonts,
                        bounds.center() - Vec2::X * radius * 0.5,
                        Align2::CENTER_CENTER,
                        format!(
                            "Roll \n{:03.0}°{}",
                            f32::abs(self.roll),
                            if self.roll < 0.0 {
                                'L'
                            } else if self.roll > 0.0 {
                                'R'
                            } else {
                                ' '
                            }
                        ),
                        FontId::monospace(0.1 * radius),
                        WHITE.into(),
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
                        Stroke::new(radius * width, WHITE),
                    )
                })
            });

            let dots = [-45.0, 45.0].iter().map(|angle| {
                Shape::circle_filled(
                    bounds.center() + Vec2::angled(f32::to_radians(angle - 90.0)) * radius * 0.9,
                    radius * 0.02,
                    WHITE,
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
                    RED,
                    Stroke::NONE,
                )
            });
        }

        response
    }
}

pub struct AttitudeIndicatorRectangular {
    pitch: f32,
    roll: f32,
}

impl AttitudeIndicatorRectangular {
    pub fn new(pitch: f32, roll: f32) -> Self {
        // Normalize -180-180
        let pitch = f32::abs((pitch + 180.0).rem_euclid(360.0)) - 180.0;
        let roll = f32::abs((roll + 180.0).rem_euclid(360.0)) - 180.0;

        Self { pitch, roll }
    }
}

impl Widget for AttitudeIndicatorRectangular {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let space = ui.available_size();
        let (response, painter) = ui.allocate_painter(
            Vec2::splat(space.min_elem()),
            Sense::focusable_noninteractive(),
        );
        let bounds = response.rect;

        let size = f32::min(bounds.width(), bounds.height());

        let (pitch, circle_color, other_color) = if self.pitch > -90.0 && self.pitch <= 90.0 {
            (self.pitch, SKY, GROUND)
        } else {
            (self.pitch + 180.0, GROUND, SKY)
        };

        let left_angle = (pitch + self.roll).rem_euclid(360.0);
        let right_angle = (self.roll - pitch + 180.0).rem_euclid(360.0);

        let left_point = match left_angle {
            0.0..=45.0 | 315.0..=365.0 => {
                bounds.left_center()
                    + Vec2::new(0.0, f32::to_radians(left_angle).tan() * size / 2.0)
            }
            45.0..=135.0 => {
                bounds.center_bottom()
                    - Vec2::new(size / (f32::to_radians(left_angle).tan() * 2.0), 0.0)
            }
            135.0..=225.0 => {
                bounds.right_center()
                    - Vec2::new(0.0, f32::to_radians(left_angle).tan() * size / 2.0)
            }
            225.0..=315.0 => {
                bounds.center_top()
                    + Vec2::new(size / (f32::to_radians(left_angle).tan() * 2.0), 0.0)
            }
            _ => unreachable!(),
        };
        let right_point = bounds.center();

        // let right_point =
        //     bounds.right_center() + Vec2::new(0.0, f32::to_radians(right_angle).sin() * size / 2.0);
        let center_point = Pos2::new(
            (left_point.x + right_point.x) / 2.0,
            (left_point.y + right_point.y) / 2.0,
        );

        let tangent = (left_point - right_point).normalized();
        let normal = tangent.rot90();

        painter.extend([
            Shape::rect_filled(bounds, Rounding::same(1.0), circle_color),
            Shape::line_segment([left_point, right_point], Stroke::new(5.0, other_color)),
        ]);

        // painter.extend([
        //     // Background
        //     Shape::circle_filled(bounds.center(), radius, circle_color),
        //     // Horizon
        //     Shape::convex_polygon(
        //         [left_point, right_point]
        //             .into_iter()
        //             .chain(
        //                 std::iter::successors(Some(right_angle), |angle| {
        //                     let angle = (angle - 5.0).rem_euclid(360.0);

        //                     if f32::abs(angle - left_angle) <= 5.0 {
        //                         None
        //                     } else {
        //                         Some(angle)
        //                     }
        //                 })
        //                 .skip(1)
        //                 .map(|angle| {
        //                     bounds.center() + Vec2::angled(f32::to_radians(angle)) * radius
        //                 }),
        //             )
        //             .collect::<Vec<_>>(),
        //         other_color,
        //         Stroke::NONE,
        //     ),
        //     // Horizon degrees markers
        //     Shape::Vec(
        //         [-20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0]
        //             .into_iter()
        //             .map(|angle| {
        //                 Shape::line_segment(
        //                     {
        //                         let upwards = normal * radius * f32::sin(f32::to_radians(angle));

        //                         let half_width = if angle % 10.0 == 0.0 { 0.2 } else { 0.1 };

        //                         let inwards = tangent * radius * half_width;

        //                         [
        //                             center_point + upwards - inwards,
        //                             center_point + upwards + inwards,
        //                         ]
        //                     },
        //                     Stroke::new(radius * 0.01, white),
        //                 )
        //             })
        //             .collect(),
        //     ),
        //     // Red level markers
        //     Shape::line_segment(
        //         [
        //             bounds.center() + Vec2::X * radius * 0.3,
        //             bounds.center() + Vec2::X * radius * 0.7,
        //         ],
        //         Stroke::new(radius * 0.01, red),
        //     ),
        //     Shape::line_segment(
        //         [
        //             bounds.center() - Vec2::X * radius * 0.3,
        //             bounds.center() - Vec2::X * radius * 0.7,
        //         ],
        //         Stroke::new(radius * 0.01, red),
        //     ),
        //     // Red center marker
        //     Shape::line(
        //         vec![
        //             bounds.center() + Vec2::angled(f32::to_radians(15.0)) * radius * 0.2,
        //             bounds.center(),
        //             bounds.center() + Vec2::angled(f32::to_radians(180.0 - 15.0)) * radius * 0.2,
        //         ],
        //         Stroke::new(radius * 0.01, red),
        //     ),
        //     // Pitch
        //     painter.fonts(|fonts| {
        //         Shape::Vec(vec![
        //             Shape::text(
        //                 fonts,
        //                 bounds.center() + Vec2::X * radius * 0.5,
        //                 Align2::CENTER_CENTER,
        //                 format!("Pitch\n{:+04.0}°", self.pitch),
        //                 FontId::monospace(0.1 * radius),
        //                 WHITE.into(),
        //             ),
        //             Shape::text(
        //                 fonts,
        //                 bounds.center() - Vec2::X * radius * 0.5,
        //                 Align2::CENTER_CENTER,
        //                 format!(
        //                     "Roll \n{:03.0}°{}",
        //                     f32::abs(self.roll),
        //                     if self.roll < 0.0 {
        //                         'L'
        //                     } else if self.roll > 0.0 {
        //                         'R'
        //                     } else {
        //                         ' '
        //                     }
        //                 ),
        //                 FontId::monospace(0.1 * radius),
        //                 WHITE.into(),
        //             ),
        //         ])
        //     }),
        // ]);

        // {
        //     let markings: &[(&[f32], std::ops::RangeInclusive<f32>, f32)] = &[
        //         (&[-20.0, -10.0, 10.0, 20.0], 0.8..=0.9, 0.01),
        //         (
        //             &[-90.0, -60.0, -30.0, 0.0, 30.0, 60.0, 90.0],
        //             0.8..=1.0,
        //             0.02,
        //         ),
        //     ];

        //     let markings = markings.iter().flat_map(|(angle, length, width)| {
        //         angle.iter().map(move |angle| {
        //             Shape::line_segment(
        //                 [
        //                     bounds.center()
        //                         + Vec2::angled(f32::to_radians(angle - 90.0))
        //                             * radius
        //                             * *length.end(),
        //                     bounds.center()
        //                         + Vec2::angled(f32::to_radians(angle - 90.0))
        //                             * radius
        //                             * *length.start(),
        //                 ],
        //                 Stroke::new(radius * width, WHITE),
        //             )
        //         })
        //     });

        //     let dots = [-45.0, 45.0].iter().map(|angle| {
        //         Shape::circle_filled(
        //             bounds.center() + Vec2::angled(f32::to_radians(angle - 90.0)) * radius * 0.9,
        //             radius * 0.02,
        //             WHITE,
        //         )
        //     });

        //     painter.extend(markings.chain(dots));

        //     // Roll Indicator
        //     painter.add({
        //         let top = Vec2::angled(f32::to_radians(self.roll - 90.0));
        //         let bottom_left = Vec2::angled(f32::to_radians(self.roll - 95.0));
        //         let bottom_right = Vec2::angled(f32::to_radians(self.roll - 85.0));

        //         Shape::convex_polygon(
        //             vec![
        //                 bounds.center() + top * radius * 0.85,
        //                 bounds.center() + bottom_right * radius * 0.7,
        //                 bounds.center() + bottom_left * radius * 0.7,
        //             ],
        //             RED,
        //             Stroke::NONE,
        //         )
        //     });
        // }

        response
    }
}
