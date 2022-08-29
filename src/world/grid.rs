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
    pub key: EventKey,
    pub radius: i32,
    pub layout: Layout,
    pub hexmap: HashMap<Axial, HexNode>,
    pub combat: Option<EventInfo<CombatAction>>,
    pub energy: Option<EventInfo<EnergyAction>>,
    pub mining: Option<EventInfo<MiningAction>>,
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
            key: EventKey::None,
            radius: radius,
            layout: layout,
            hexmap: HashMap::new(),
            combat: None,
            energy: None,
            mining: None,
        });
        GridEvents::load_data(app);

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
    root.collect(&mut node_a);

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
    root.collect(&mut node_b);

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
    root.collect(&mut node_c);

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
    root.collect(&mut node_d);

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
    root.collect(&mut node_e);

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
    root.collect(&mut node_f);

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
    root.collect(&mut node_g);

    grid.hexmap = root.nodes;
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
        if let Some(node) = self.hexmap.get(&hex) {
            return node.value;
        }
        0
    }

    pub fn get_entity(&mut self, hex: Axial) -> Option<Entity> {
        if let Some(node) = self.hexmap.get(&hex) {
            return Some(node.entity);
        }
        None
    }

    pub fn get_event_key(&mut self, hex: Axial) -> EventKey {
        if let Some(node) = self.hexmap.get(&hex) {
            return node.key;
        }
        EventKey::Combat
    }

    pub fn get_event_data(&self) -> EventData {
        match self.key {
            EventKey::None => EventData::default(),
            EventKey::Combat => {
                if let Some(event) = &self.combat {
                    return event.data.clone();
                }
                EventData::default()
            }
            EventKey::Energy => {
                if let Some(event) = &self.energy {
                    return event.data.clone();
                }
                EventData::default()
            }
            EventKey::Mining => {
                if let Some(event) = &self.mining {
                    return event.data.clone();
                }
                EventData::default()
            }
        }
    }

    // FixMe: Rust must have cleaner way to do this setup
    pub fn clr_event(&mut self) {
        self.key = EventKey::None;
        self.combat = None;
        self.energy = None;
        self.mining = None;
    }

    pub fn set_event_combat(&mut self, event: EventInfo<CombatAction>) {
        self.key = EventKey::Combat;
        self.combat = Some(event);
        self.energy = None;
        self.mining = None;
    }

    pub fn set_event_energy(&mut self, event: EventInfo<EnergyAction>) {
        self.key = EventKey::Energy;
        self.combat = None;
        self.energy = Some(event);
        self.mining = None;
    }

    pub fn set_event_mining(&mut self, event: EventInfo<MiningAction>) {
        self.key = EventKey::Mining;
        self.combat = None;
        self.energy = None;
        self.mining = Some(event);
    }
}
