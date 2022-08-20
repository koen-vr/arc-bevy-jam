//use std::collections::HashMap;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub mod orient;
pub(crate) use orient::*;

mod storage;
pub(crate) use storage::*;

#[derive(Clone, Copy)]
enum Diagonal {
    QNeg, // {-2,  1,  1},
    QPos, // { 2, -1, -1},
    RNeg, // { 1, -2,  1},
    RPos, // {-1,  2, -1},
    SNeg, // { 1,  1, -2},
    SPos, // {-1, -1,  2},
    None, // { 0,  0,  0},
}

#[derive(Clone, Copy)]
enum Direction {
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Diagonal {
    fn delta(&self) -> Cuboid {
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
    fn delta(&self) -> Cuboid {
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
    fn delta(&self, rh: &Axial) -> Cuboid {
        let q = self.q - rh.q;
        let r = self.r - rh.r;
        Cuboid {
            q: q,
            r: r,
            s: -q - r,
        }
    }

    fn neighbor(&self, d: Direction) -> Axial {
        let dir = d.delta();
        Axial {
            q: self.q + dir.q,
            r: self.r + dir.r,
        }
    }

    fn line(a: &Axial, b: &Axial) -> Vec<Axial> {
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
            while let Some(v) = visited.get(&pnt) {
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

impl Cuboid {
    fn abs(&self) -> Cuboid {
        Cuboid {
            q: i32::abs(self.q),
            r: i32::abs(self.r),
            s: i32::abs(self.s),
        }
    }

    fn length(&self) -> i32 {
        (i32::abs(self.q) + i32::abs(self.r) + i32::abs(self.s)) >> 1
    }

    fn direction(&self) -> Direction {
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

impl Point {}

fn f32_to_axial(x: f32, y: f32, z: f32) -> Axial {
    let mut rx = f32::round(x);
    let mut ry = f32::round(y);
    let mut rz = f32::round(z);

    let dx = f32::abs(rx - x);
    let dy = f32::abs(ry - y);
    let dz = f32::abs(rz - z);

    if dx > dz && dx > dy {
        rx = -rz - ry
    } else if dz > dy {
        rz = -rx - ry
    } else {
        ry = -rx - rz
    }

    return Axial {
        q: f32::round(rx) as i32,
        r: f32::round(rz) as i32,
    };
}
