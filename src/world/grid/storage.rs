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
    pub radius: i32,
    pub layout: Layout,
    pub hexgrid: HashMap<Axial, Option<Entity>>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct HexStorage {
    pub nodes: Vec<HexNode>,
}

impl HexNode {
    pub fn new(size: Vec2, style: orient::Style, origin: Vec2, radius: i32) -> Self {
        Self {
            radius: radius,
            layout: Layout {
                size: size,
                style: style.clone(),
                origin,
                matrix: Orientation::new(style),
            },
            hexgrid: HashMap::new(),
        }
    }

    pub fn spawn_entities(
        &mut self,
        color: Color,
        node_id: Entity,
        commands: &mut Commands,
        world_assets: &Res<WorldAssets>,
    ) -> Vec<Entity> {
        // Collections setup
        let mut list = Vec::new();
        let mut grid = HashMap::new();

        // Spawn hex grid entities
        for q in -self.radius..(self.radius + 1) {
            let r1 = i32::max(-self.radius, -q - self.radius);
            let r2 = i32::min(self.radius, -q + self.radius);
            for r in r1..(r2 + 1) {
                let hex = Axial { q, r };
                let name = format!("hex-{}:{}", q, r);
                let pos = self.layout.center_for(&hex);
                let bundle = SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        custom_size: Some(Vec2::splat(TILE_SIZE * 2.)),
                        ..Default::default()
                    },
                    texture: world_assets.pointy_hex64_a.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, 10.0),
                        ..Default::default()
                    },
                    ..default()
                };
                let entity = commands
                    .spawn_bundle(bundle)
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
