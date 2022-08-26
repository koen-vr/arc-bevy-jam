use super::*;

const ITERATIONS: i32 = 96;
const MAX_CHECKS: i32 = 32;

const MIN_RADIUS: f32 = 192. + 64.;
const AVR_RADIUS: f32 = 256. + 64.;
const MAX_RADIUS: f32 = 384. + 64.;

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

    fn find_nearest(&mut self, target: &Point, start: &Point, list: &Vec<Point>) -> (Point, f32) {
        let mut nc = start;
        let mut dc = nc.distance(target);
        for n in list {
            let d = n.distance(target);
            if d < dc {
                (dc, nc) = (d, n);
            }
        }
        return (nc.clone(), dc);
    }

    pub fn spawn_points(
        &mut self,
        seed: i64,
        commands: &mut Commands,
        world_assets: &Res<WorldAssets>,
    ) {
        let mut rng = Shift64::new(seed);

        let scale = self.layout.hex_size();
        let hexes = (i32::abs(-self.radius - self.radius) + 1) as f32;

        // 0.1 Size of the point grid for generation and offset
        let width = (scale.x * hexes - (34. * 0.25)) as i32;
        let height = (scale.y * hexes - (34. * 0.25)) as i32;
        let offset_x = width as f32 * 0.5;
        let offset_y = height as f32 * 0.5;

        let mut list: Vec<Point> = Vec::new();
        let mut mapped: Vec<Vec<Option<Point>>> = vec![vec![None; width as usize]; height as usize];

        let rx = width / 4;
        let ry = height / 4;
        let root = Point {
            x: (rng.i32(rx) - (rx / 2)) + (width / 2),
            y: (rng.i32(ry) - (ry / 2)) + (height / 2),
        };

        list.push(root);
        mapped[root.x as usize][root.y as usize] = Some(root);

        for _x in 0..ITERATIONS {
            let mut rn: i32 = 0;
            let mut ds: f32 = 0.;
            let mut na: Point = Point::default();
            let mut nb: Point = Point::default();

            // 1.  Get random & nearest node (&& distance)
            // 2.  While min radius > distance between (&& max checks)
            // 2.1  goto: 1
            while MAX_RADIUS > ds && MAX_CHECKS > rn {
                rn += 1;
                let x = rng.i32(width);
                let y = rng.i32(height);

                na = Point { x, y };
                (nb, ds) = self.find_nearest(&na, &root, &list);
            }
            // 3.  max radius < distance between
            // 3.1  change to max distance
            if MAX_RADIUS < ds {
                let xc = (na.x - nb.x) as f32 / ds;
                let yc = (na.y - nb.y) as f32 / ds;
                // let xi = (AVR_RADIUS*xc + (nb.x) as f32) as i32;
                // let yi = (AVR_RADIUS*yc + (nb.y) as f32) as i32;
                na = Point {
                    x: (AVR_RADIUS * xc + (nb.x as f32)) as i32,
                    y: (AVR_RADIUS * yc + (nb.y as f32)) as i32,
                };
            }
            // 4.  min radius < distance between
            // 4.1  check ifnode is inside the map
            // 4.1.1  add node and link up to graph
            if self.layout.is_in_range(
                Vec2 {
                    x: ((na.x as f32) - offset_x) + self.layout.origin.x,
                    y: ((na.y as f32) - offset_y) + self.layout.origin.y,
                },
                self.radius,
            ) && MIN_RADIUS <= ds
            {
                list.push(na);
                // if (rn % 2) > 0 {
                //     na.value = 0.4
                // } else {
                //     na.value = 0.2
                // }
            }
        }

        // Setup the node entity and spawn the grid
        let name = format!("points-{}:{}", 0, 0);
        let node_id = commands.spawn().insert(Name::new(name)).id();
        let mut points = Vec::new();

        // Spawn Points of Intrest
        let mut sprite = TextureAtlasSprite::new(34);
        sprite.color = Color::rgb(0.9, 0.8, 1.0);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE * 0.25));

        for pnt in list {
            // let hex = layout.hex_for(Vec2 {
            //     x: ((pnt.x as f32) - offset_x) + layout.origin.x,
            //     y: ((pnt.y as f32) - offset_y) + layout.origin.y,
            // });
            // let pos = layout.center_for(&hex);

            let pos = Vec2 {
                x: ((pnt.x as f32) - offset_x) + self.layout.origin.x,
                y: ((-pnt.y as f32) - offset_y) + self.layout.origin.y,
            };
            points.push(
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas: world_assets.base_space_sheet.clone(),
                        transform: Transform {
                            translation: Vec3::new(pos.x, pos.y, 9.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .id(),
            );
        }

        // Finalize hex node and entities as children
        commands
            .entity(node_id)
            .insert_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(CleanupGrid)
            // Keep the points or hide?
            //.insert(GridRoot)
            .push_children(&points);
    }
}
