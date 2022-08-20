//use std::collections::HashMap;

use super::*;
use crate::*;

#[derive(Clone, Copy, Debug, Default)]
pub enum Style {
    #[default]
    Flat,
    Pointy,
}

#[derive(Clone, Default, Debug)]
pub struct Layout {
    pub origin: Vec2,
    pub radius: Vec2,
    pub layout: Style,
    pub matrix: Orientation,
}

#[derive(Clone, Default, Debug)]
pub struct Orientation {
    a: f32,
    b: [f32; 4],
    c: [f32; 6],
    f: [f32; 4],
    s: [f32; 6],
}

const PI: f32 = 3.14159265358979323846264338327950288;

impl Orientation {
    pub fn new(style: Style) -> Self {
        let SQ3 = f32::sqrt(3.0);
        match style {
            Style::Flat => Orientation {
                f: [3. / 2., 0., SQ3 / 2., SQ3],
                b: [2. / 3., 0., -1. / 3., SQ3 / 3.],
                a: 0.,
                c: [
                    f32::cos(2. * PI * 0. / 6.),
                    f32::cos(2. * PI * 1. / 6.),
                    f32::cos(2. * PI * 2. / 6.),
                    f32::cos(2. * PI * 3. / 6.),
                    f32::cos(2. * PI * 4. / 6.),
                    f32::cos(2. * PI * 5. / 6.),
                ],
                s: [
                    f32::sin(2. * PI * 0. / 6.),
                    f32::sin(2. * PI * 1. / 6.),
                    f32::sin(2. * PI * 2. / 6.),
                    f32::sin(2. * PI * 3. / 6.),
                    f32::sin(2. * PI * 4. / 6.),
                    f32::sin(2. * PI * 5. / 6.),
                ],
            },
            Style::Pointy => Orientation {
                f: [SQ3, SQ3 / 2., 0., 3. / 2.],
                b: [SQ3 / 3., -1. / 3., 0., 2. / 3.],
                a: 0.5,
                c: [
                    f32::cos(2. * PI * 0.5 / 6.),
                    f32::cos(2. * PI * 1.5 / 6.),
                    f32::cos(2. * PI * 2.5 / 6.),
                    f32::cos(2. * PI * 3.5 / 6.),
                    f32::cos(2. * PI * 4.5 / 6.),
                    f32::cos(2. * PI * 5.5 / 6.),
                ],
                s: [
                    f32::sin(2. * PI * 0.5 / 6.),
                    f32::sin(2. * PI * 1.5 / 6.),
                    f32::sin(2. * PI * 2.5 / 6.),
                    f32::sin(2. * PI * 3.5 / 6.),
                    f32::sin(2. * PI * 4.5 / 6.),
                    f32::sin(2. * PI * 5.5 / 6.),
                ],
            },
        }
    }
}
impl Layout {
    pub fn new(origin: Vec2, radius: Vec2, style: Style) -> Self {
        Layout {
            origin,
            radius,
            layout: style,
            matrix: Orientation::new(style),
        }
    }

    pub fn hex_for(&self, val: Vec2) -> Axial {
        let x = val.x - self.origin.x;
        let y = val.y - self.origin.y;
        let q = (self.matrix.b[0] * x + self.matrix.b[1] * y) / self.radius.x;
        let r = (self.matrix.b[2] * x + self.matrix.b[3] * y) / self.radius.y;
        return f32_to_axial(q, -q - r, r);
    }

    pub fn hex_size(&self) -> Vec2 {
        match self.layout {
            Style::Flat => Vec2 {
                x: 2. * self.radius.x,
                y: f32::sqrt(3.) * self.radius.y,
            },
            Style::Pointy => Vec2 {
                x: f32::sqrt(3.) * self.radius.x,
                y: 2. * self.radius.y,
            },
        }
    }

    pub fn center_for(&self, hex: &Axial) -> Vec2 {
        let (q, r) = (hex.q as f32, hex.r as f32);
        let x = (self.matrix.f[0] * q + self.matrix.f[1] * r) * self.radius.x;
        let y = (self.matrix.f[2] * q + self.matrix.f[3] * r) * self.radius.y;
        return Vec2 {
            x: x + self.origin.x,
            y: y + self.origin.y,
        };
    }

    pub fn area_for(&self, center: Axial, rad: f32) -> HashMap<Axial, bool> {
        let ring = self.ring_for(&center, rad);
        let mut result = HashMap::new();
        for key in ring.keys() {
            result.insert(key.clone(), true);
            for val in Axial::line(key, &center) {
                result.insert(val, true);
            }
        }
        return result;
    }

    pub fn ring_for(&self, center: &Axial, rad: f32) -> HashMap<Axial, bool> {
        let mut result = HashMap::new();
        if rad < self.radius.x && rad < self.radius.y {
            result.insert(center.clone(), true);
            return result;
        }
        let cp = self.center_for(center);

        let mut px = Vec2 { x: rad, y: 0. };
        let mut p = 1. - rad;

        while px.x > px.y {
            if p <= 0. {
                p = p + 2. * px.y + 1.
            } else {
                px.x = px.x - 1.;
                p = p + 2. * px.y - 2. * px.x + 1.
            }

            if px.x < px.y {
                px.y += 1.;
                break;
            }

            let points: [Vec2; 8] = [
                Vec2 {
                    x: px.x + cp.x,
                    y: px.y + cp.y,
                },
                Vec2 {
                    x: -px.x + cp.x,
                    y: px.y + cp.y,
                },
                Vec2 {
                    x: px.x + cp.x,
                    y: -px.y + cp.y,
                },
                Vec2 {
                    x: -px.x + cp.x,
                    y: -px.y + cp.y,
                },
                Vec2 {
                    x: px.y + cp.x,
                    y: px.x + cp.y,
                },
                Vec2 {
                    x: -px.y + cp.x,
                    y: px.x + cp.y,
                },
                Vec2 {
                    x: px.y + cp.x,
                    y: -px.x + cp.y,
                },
                Vec2 {
                    x: -px.y + cp.x,
                    y: -px.x + cp.y,
                },
            ];
            for v in points {
                result.insert(self.hex_for(v), true);
            }
            px.y += 1.;
        }
        return result;
    }
}
