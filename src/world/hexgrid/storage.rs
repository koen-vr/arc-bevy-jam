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
    pub hexgrid: HashMap<Axial, Option<Entity>>,
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
            hexgrid: HashMap::new(),
        }
    }

    pub fn spawn_entities(
        &mut self,
        node_id: Entity,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Vec<Entity> {
        // Collections setup
        let mut list = Vec::new();
        let mut grid = HashMap::new();

        let sprite = asset_server.load("hex-pointy-64.1.png");

        // Spawn hex grid entities
        let radius = 12;
        for q in -radius..(radius + 1) {
            let r1 = i32::max(-radius, -q - radius);
            let r2 = i32::min(radius, -q + radius);
            for r in r1..(r2 + 1) {
                let hex = Axial { q, r };
                let name = format!("hex-{}:{}", q, r);
                let pos = self.layout.center_for(&hex);
                let entity = commands
                    .spawn_bundle(SpriteBundle {
                        texture: sprite.clone(),
                        transform: Transform {
                            translation: Vec3::new(pos.x, pos.y, 10.0),
                            ..Default::default()
                        },
                        ..default()
                    })
                    .insert_bundle(HexBundle {
                        position: hex.into(),
                        texture: HexTexture(0),
                        node: HexNodeId(node_id),
                    })
                    .insert(Name::new(name))
                    .id();

                grid.insert(hex, Some(entity));
                list.push(entity);
            }
        }

        // Save the hex grid for updates
        self.hexgrid = grid;
        list
    }
}
