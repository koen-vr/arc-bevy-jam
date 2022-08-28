use super::*;

pub mod data;
pub use data::*;

pub mod math;
pub use math::*;

pub mod orient;
pub use orient::*;

pub mod storage;
pub use storage::*;

pub mod utilities;
pub use utilities::*;

pub struct Grid {
    pub radius: i32,
    pub layout: Layout,
    pub nodes: HashMap<Axial, HexNode>,
}

#[derive(Component)]
struct GridRoot;

#[derive(Component)]
struct CleanupGrid;

#[derive(Component)]
struct CleanupGridGame;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        let base_mode = AppState::GamePlay(GameMode::BaseGrid);
        let explore_mode = AppState::GamePlay(GameMode::ExploreGrid);

        if tool::debug::ENABLE_INSPECTOR {
            app.register_inspectable::<GridTarget>();
            app.register_inspectable::<GridMovement>();
        }

        let radius = 38;
        let layout = Layout {
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            style: orient::Style::Pointy,
            origin: Vec2 { x: 0., y: 0. },
            matrix: Orientation::new(orient::Style::Pointy),
        };
        app.insert_resource(Grid {
            radius: radius,
            layout: layout,
            nodes: HashMap::new(),
        });

        // FixMe: Execution of systems is not ordered, this is broken.
        // Verification depends on fixed execution to repeatable values.
        app.insert_resource(Shift64::new(rand::random::<i64>()));

        app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_grid_game));
        // app.add_system_set(SystemSet::on_enter(base_mode).with_system(enter_grid_game));

        app.add_system_set(SystemSet::on_exit(explore_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(explore_mode).with_system(spawn_explore_movement));

        app.add_system_set(SystemSet::on_pause(explore_mode).with_system(pause_explore_movement));
        app.add_system_set(SystemSet::on_resume(explore_mode).with_system(resume_explore_movement));

        app.add_system_set(SystemSet::on_enter(explore_mode).with_system(spawn_grid_nodes));
        app.add_system_set(SystemSet::on_pause(explore_mode).with_system(pause_grid_nodes));
        app.add_system_set(SystemSet::on_resume(explore_mode).with_system(resume_grid_nodes));
    }
}

fn exit_state(mut commands: Commands, query: Query<Entity, With<CleanupGrid>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

////////////////////////////////
/// Game Setup - Shared Objects
////////////////////////////////

fn exit_grid_game(mut commands: Commands, query: Query<Entity, With<CleanupGridGame>>) {
    log::info!("exit_grid_game");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// fn enter_grid_game(mut commands: Commands) {
//     // TODO: Seed selection gui
//     log::info!("enter_grid_game");
// }

fn spawn_explore_movement(mut commands: Commands, world_assets: Res<WorldAssets>) {
    log::info!("spawn_explore_movement");
    let mut hex = SpriteBundle {
        texture: world_assets.pointy_hex64_b.clone(),
        ..default()
    };
    hex.visibility.is_visible = true;
    hex.transform.translation = Vec3::new(0.0, 0.0, 1.0);
    commands
        .spawn_bundle(hex)
        .insert(Name::new("hex-target"))
        .insert(GridTargetHex)
        .insert(CleanupGrid);
}

fn pause_explore_movement(mut grid_root: Query<&mut Visibility, With<GridRoot>>) {
    for mut visibility in grid_root.iter_mut() {
        visibility.is_visible = false;
    }
}

fn resume_explore_movement(mut grid_root: Query<&mut Visibility, With<GridRoot>>) {
    for mut visibility in grid_root.iter_mut() {
        visibility.is_visible = true;
    }
}

fn spawn_grid_nodes(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut rng: ResMut<Shift64>,
    world_assets: Res<WorldAssets>,
) {
    // Main Grid nodes
    let mut root = HexMap::new(
        grid.layout.size,
        grid.layout.style,
        grid.layout.origin,
        grid.radius,
    );
    _spawn_grid_node(
        &mut root,
        &mut commands,
        &world_assets,
        false,
        rng.shift(),
        Color::rgba(0.6, 0.4, 0.6, 0.3),
    );

    // TODO Grid should have a map of all tiles
    // all points in nodes should be stored in grid

    // Sub grid nodes
    let offset = 24;
    let mut node_a = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial { q: -offset, r: 0 }),
        12,
    );
    _spawn_grid_node(
        &mut node_a,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_b = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial { q: 0, r: -offset }),
        12,
    );
    _spawn_grid_node(
        &mut node_b,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_c = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial {
            q: -offset,
            r: offset,
        }),
        12,
    );
    _spawn_grid_node(
        &mut node_c,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_d = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        Vec2 { x: 0., y: 0. },
        12,
    );
    _spawn_grid_node(
        &mut node_d,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_e = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial { q: offset, r: 0 }),
        12,
    );
    _spawn_grid_node(
        &mut node_e,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_f = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial { q: 0, r: offset }),
        12,
    );
    _spawn_grid_node(
        &mut node_f,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    let mut node_g = HexMap::new(
        Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        orient::Style::Pointy,
        grid.layout.center_for(&Axial {
            q: offset,
            r: -offset,
        }),
        12,
    );
    _spawn_grid_node(
        &mut node_g,
        &mut commands,
        &world_assets,
        true,
        rng.shift(),
        Color::rgba(0.8, 0.6, 0.8, 0.2),
    );

    grid.nodes = root.nodes;
}

fn _spawn_grid_node(
    node: &mut HexMap,
    commands: &mut Commands,
    world_assets: &Res<WorldAssets>,
    run: bool,
    seed: i64,
    color: Color,
) {
    // Setup the node entity and spawn the grid
    let name = format!("node-{}:{}", 0, 0);
    let node_id = commands.spawn().insert(Name::new(name)).id();
    let list = node.spawn_entities(color, node_id, commands, &world_assets);

    if run {
        node.spawn_points(seed, commands, world_assets);
    }

    // Finalize hex node and entities as children
    commands
        .entity(node_id)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .insert(CleanupGrid)
        .insert(GridRoot)
        // .insert(node)
        .push_children(&list);
}

fn pause_grid_nodes(mut nodes_query: Query<&mut Visibility, With<GridTargetHex>>) {
    for mut visibility in nodes_query.iter_mut() {
        visibility.is_visible = false;
    }
}

fn resume_grid_nodes(mut nodes_query: Query<&mut Visibility, With<GridTargetHex>>) {
    for mut visibility in nodes_query.iter_mut() {
        visibility.is_visible = true;
    }
}

impl Grid {
    pub fn on_grid(&self, hex: &Axial) -> bool {
        self.radius >= hex.distance(&Axial { q: 0, r: 0 })
    }

    pub fn get_hex(&mut self, position: Vec2) -> Axial {
        self.layout.hex_for(position)
    }

    pub fn get_value(&mut self, hex: Axial) -> i32 {
        if let Some(node) = self.nodes.get(&hex) {
            return node.value;
        }
        0
    }

    pub fn get(&mut self, hex: Axial) -> Option<Entity> {
        if let Some(node) = self.nodes.get(&hex) {
            return Some(node.entity);
        }
        None
    }

    pub fn get_event_key(&mut self, hex: Axial) -> EventKey {
        if let Some(node) = self.nodes.get(&hex) {
            return node.key;
        }
        EventKey::Combat
    }

    pub fn roll_event_table(&mut self, seed: i64, position: Vec2) -> EventKey {
        let hex = self.layout.hex_for(position);
        if let Some(node) = self.nodes.get(&hex) {
            // TODO Roll the actual table in to an event
            return node.key;
        }
        // TODO Roll combat table in to an event
        EventKey::Combat
    }
}
