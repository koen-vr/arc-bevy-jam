use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::*;

#[derive(Component, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Inspectable)]
pub struct HexPosition {
    pub x: i32,
    pub y: i32,
}

impl Into<Axial> for &HexPosition {
    fn into(self) -> Axial {
        Axial {
            q: self.x,
            r: self.y,
        }
    }
}

impl From<Axial> for HexPosition {
    fn from(v: Axial) -> Self {
        Self { x: v.q, y: v.r }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Hash, Inspectable)]
pub struct HexTexture(pub u32);

#[derive(Component, Clone, Copy, Debug, Hash, Inspectable, Reflect)]
pub struct HexNodeId(pub Entity);

impl Default for HexNodeId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

#[derive(Bundle, Clone, Copy, Debug, Default)]
pub struct HexBundle {
    pub position: HexPosition,
    pub texture: HexTexture,
    pub node: HexNodeId,
}

#[derive(Component, Clone, Debug, Default)]
pub struct HexNode {
    pub layout: Layout,
    pub tilemap: HashMap<Axial, Option<Entity>>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct HexStorage {
    pub nodes: Vec<HexNode>,
}

impl HexNode {
    pub fn new(origin: Vec2, radius: Vec2, style: orient::Style) -> Self {
        Self {
            layout: Layout {
                origin,
                radius,
                layout: style.clone(),
                matrix: Orientation::new(style),
            },
            tilemap: HashMap::new(),
        }
    }
}
