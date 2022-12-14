use std::fmt::Display;

use super::*;

#[derive(Clone, Copy)]
pub enum Diagonal {
    QNeg, // {-2,  1,  1},
    QPos, // { 2, -1, -1},
    RNeg, // { 1, -2,  1},
    RPos, // {-1,  2, -1},
    SNeg, // { 1,  1, -2},
    SPos, // {-1, -1,  2},
    None, // { 0,  0,  0},
}

#[derive(Clone, Copy)]
pub enum Direction {
    QNeg, // {-1,  0,  1},
    QPos, // { 1,  0, -1},
    RNeg, // { 1, -1,  0},
    RPos, // {-1,  1,  0},
    SNeg, // { 0,  1, -1},
    SPos, // { 0, -1,  1},
    None, // { 0,  0,  0},
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Cuboid {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Diagonal {
    pub fn delta(&self) -> Cuboid {
        match self {
            Diagonal::QNeg => Cuboid { q: -2, r: 1, s: 1 },
            Diagonal::QPos => Cuboid { q: 2, r: -1, s: -1 },
            Diagonal::RNeg => Cuboid { q: 1, r: -2, s: 1 },
            Diagonal::RPos => Cuboid { q: -1, r: 2, s: -1 },
            Diagonal::SNeg => Cuboid { q: 1, r: 1, s: -2 },
            Diagonal::SPos => Cuboid { q: -1, r: -1, s: 2 },
            Diagonal::None => Cuboid { q: 0, r: 0, s: 0 },
        }
    }
}

impl Direction {
    pub fn delta(&self) -> Cuboid {
        match self {
            Direction::QNeg => Cuboid { q: -1, r: 0, s: 1 },
            Direction::QPos => Cuboid { q: 1, r: 0, s: -1 },
            Direction::RNeg => Cuboid { q: 1, r: -1, s: 0 },
            Direction::RPos => Cuboid { q: -1, r: 1, s: 0 },
            Direction::SNeg => Cuboid { q: 0, r: 1, s: -1 },
            Direction::SPos => Cuboid { q: 0, r: -1, s: 1 },
            Direction::None => Cuboid { q: 0, r: 0, s: 0 },
        }
    }
}

impl Axial {
    pub fn delta(&self, rh: &Axial) -> Cuboid {
        let q = self.q - rh.q;
        let r = self.r - rh.r;
        Cuboid {
            q: q,
            r: r,
            s: -q - r,
        }
    }

    pub fn distance(&self, other: &Axial) -> i32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;
        return (i32::abs(dq) + i32::abs(dr) + i32::abs(dr + dq)) >> 1;
    }

    pub fn neighbor(&self, d: Direction) -> Axial {
        let dir = d.delta();
        Axial {
            q: self.q + dir.q,
            r: self.r + dir.r,
        }
    }

    pub fn line(a: &Axial, b: &Axial) -> Vec<Axial> {
        let delta = a.delta(b);

        let n = delta.length();
        let dir = delta.direction();

        let mut result = Vec::new();
        let mut visited = HashMap::new();

        let (ax, ay, az) = (a.q as f32, (-a.q - a.r) as f32, a.r as f32);
        let (bx, by, bz) = (b.q as f32, (-b.q - b.r) as f32, b.r as f32);
        let (x, y, z) = (bx - ax, by - ay, bz - az);

        let step = 1. / (n as f32);
        for h in 0..n {
            let t = step * (h as f32);
            let mut pnt = f32_to_axial(ax + x * t, ay + y * t, az + z * t);
            while let Some(_v) = visited.get(&pnt) {
                pnt = pnt.neighbor(dir)
            }
            result.push(pnt);
            visited.insert(pnt, true);
        }

        if let None = visited.get(b) {
            result.push(b.clone());
        }

        return result;
    }
}

impl Display for Axial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.q, self.r)
    }
}

impl Cuboid {
    pub fn abs(&self) -> Cuboid {
        Cuboid {
            q: i32::abs(self.q),
            r: i32::abs(self.r),
            s: i32::abs(self.s),
        }
    }

    pub fn length(&self) -> i32 {
        (i32::abs(self.q) + i32::abs(self.r) + i32::abs(self.s)) >> 1
    }

    pub fn direction(&self) -> Direction {
        let abs = self.abs();
        if abs.q >= abs.r && abs.q >= abs.s {
            if self.q < 0 {
                return Direction::QNeg;
            }
            return Direction::QPos;
        }
        if abs.r >= abs.s {
            if self.r < 0 {
                return Direction::RNeg;
            }
            return Direction::RPos;
        }
        if self.s < 0 {
            return Direction::SNeg;
        }
        return Direction::SPos;
    }
}

impl Point {
    pub fn distance(&self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        return f64::sqrt((dx * dx) + (dy * dy)) as f32;
    }
}

pub(crate) fn f32_to_axial(x: f32, y: f32, z: f32) -> Axial {
    let mut rx = f32::round(x);
    let ry = f32::round(y);
    let mut rz = f32::round(z);

    let dx = f32::abs(rx - x);
    let dy = f32::abs(ry - y);
    let dz = f32::abs(rz - z);

    if dx > dz && dx > dy {
        rx = -rz - ry
    } else if dz > dy {
        rz = -rx - ry
    }

    return Axial {
        q: f32::round(rx) as i32,
        r: f32::round(rz) as i32,
    };
}
