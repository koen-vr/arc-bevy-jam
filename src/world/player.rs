use super::*;

pub mod resources;
pub use resources::*;

pub const ENERGY_COST: i32 = 4;

#[derive(Component, Default, Inspectable)]
pub struct Player {
    pub active: bool,
    lookat: Vec3,
    jump_range: u8,
    move_speed: f32,
    rotate_speed: f32,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerState {
    pub position: Vec2,
}

#[derive(Component, Default)]
struct PlayerCamera {
    timer: Timer,
    scale: f32,
    target: f32,
}

#[derive(Component, Clone, Copy, Debug, Hash)]
pub struct GridId(pub Entity);

#[derive(Component)]
struct CleanupPlayer;

#[derive(Component)]
struct CleanupPlayerGame;

#[derive(Component)]
struct CleanupPlayerExplore;

pub struct PlayerPlugin;

// TODO System if energy = 0 -> Game Over: Stranded
// TODO System if health = 0 -> Game Over: Destroyed

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let base_mode = AppState::GamePlay(GameMode::BaseGrid);
        let event_mode = AppState::GamePlay(GameMode::EventGrid);
        let explore_mode = AppState::GamePlay(GameMode::ExploreGrid);

        // FixMe: load from saved state
        app.insert_resource(PlayerState {
            ..Default::default()
        });

        if tool::debug::ENABLE_INSPECTOR {
            app.register_inspectable::<Player>();
            app.register_inspectable::<HealthRecource>();
            app.register_inspectable::<EnergyRecource>();
        }

        app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_player_game));
        app.add_system_set(SystemSet::on_enter(base_mode).with_system(enter_player_game));

        app.add_system_set(SystemSet::on_exit(event_mode).with_system(exit_player_event));
        app.add_system_set(SystemSet::on_exit(event_mode).with_system(exit_event_camera));
        app.add_system_set(SystemSet::on_enter(event_mode).with_system(enter_event_camera));

        // -- FixMe: mess to exit state without poping in to base_mode --------------------
        app.add_system_set(SystemSet::on_exit(explore_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_exit(explore_mode).with_system(exit_player_game));
        // --------------------------------------------------------------------------------

        app.add_system_set(SystemSet::on_exit(explore_mode).with_system(exit_player_explore));
        app.add_system_set(SystemSet::on_enter(explore_mode).with_system(enter_player_explore));

        // Player Movement Systems
        app.add_system_set(
            SystemSet::on_update(event_mode)
                .with_system(move_event_grid.after("gui-update").label("player-move")),
        );
        app.add_system_set(
            SystemSet::on_update(explore_mode)
                .with_system(move_explore_grid.after("gui-update").label("player-move")),
        );

        app.add_system_set(
            SystemSet::on_update(event_mode)
                .with_system(move_player_camera.after("gui-update").after("player-move")),
        );
        app.add_system_set(
            // Note: Keep rotation on path when moving
            SystemSet::on_update(explore_mode)
                .with_system(move_player_camera.after("gui-update").after("player-move")),
        );

        app.add_system_set(
            SystemSet::on_update(event_mode).with_system(
                player_rotate_system
                    .after("gui-update")
                    .after("player-move"),
            ),
        );
        app.add_system_set(
            // Note: Keep rotation on path when moving
            SystemSet::on_update(explore_mode).with_system(
                player_rotate_system
                    .after("gui-update")
                    .after("player-move"),
            ),
        );

        // Camera Scale
        app.add_system_set(
            SystemSet::on_update(event_mode).with_system(update_scale_camera.after("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(explore_mode).with_system(update_scale_camera.after("gui-update")),
        );
    }
}

fn exit_state(mut commands: Commands, query: Query<Entity, With<CleanupPlayer>>) {
    log::info!("exit_state");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

////////////////////////////////
/// Game Setup - Shared Objects
////////////////////////////////

fn exit_player_game(mut commands: Commands, query: Query<Entity, With<CleanupPlayerGame>>) {
    log::info!("exit_player_game");
    // TODO Reset camera offset
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_player_game(mut commands: Commands) {
    log::info!("enter_player_game");
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 1.8;
    commands
        .spawn_bundle(camera_bundle)
        .insert(Name::new("game-camera"))
        .insert(CleanupPlayerGame)
        .insert(PlayerCamera {
            timer: Timer::default(),
            scale: CAMERA_ZOOM_EXPLORE,
            target: CAMERA_ZOOM_EXPLORE,
        })
        .insert(MainCamera);
}

fn exit_event_camera(mut camera_query: Query<&mut PlayerCamera, Without<Player>>) {
    let mut camera = camera_query.single_mut();
    camera.timer = Timer::from_seconds(1., false);
    camera.target = CAMERA_ZOOM_EXPLORE;
}

fn enter_event_camera(mut camera_query: Query<&mut PlayerCamera, Without<Player>>) {
    let mut camera = camera_query.single_mut();
    camera.timer = Timer::from_seconds(1., false);
    camera.target = CAMERA_ZOOM_EVENT
}

fn update_scale_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut PlayerCamera, &mut OrthographicProjection), Without<Player>>,
) {
    let (mut camera, mut projection) = camera_query.single_mut();
    if camera.scale == camera.target {
        return;
    }

    camera.timer.tick(time.delta());
    if camera.timer.finished() {
        projection.scale = camera.target;
        camera.scale = camera.target;
        return;
    }

    let t = camera.timer.percent();
    projection.scale = lerp(camera.scale, camera.target, t);
}

////////////////////////////////
/// Player Setup - Base Objects
////////////////////////////////

fn exit_player_event(
    player_state: Res<PlayerState>,
    mut player_query: Query<(&mut Transform, &mut GridTarget), With<Player>>,
) {
    let (mut transform, mut move_to) = player_query.single_mut();
    move_to.moving = false;
    move_to.target.x = player_state.position.x;
    move_to.target.y = player_state.position.y;
    transform.translation.x = player_state.position.x;
    transform.translation.y = player_state.position.y;
}

fn exit_player_explore(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    query: Query<Entity, With<CleanupPlayerExplore>>,
) {
    log::info!("exit_player_explore");
    player_state.position = Vec2::default();
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_player_explore(
    mut commands: Commands,
    ship_info: Res<ShipInfo>,
    player_state: Res<PlayerState>,
    world_assets: Res<WorldAssets>,
) {
    log::info!("enter_player_explore");

    let mut sprite = TextureAtlasSprite::new(ship_info.get_index());
    sprite.color = Color::rgb(0.9, 0.8, 1.0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE * 0.5));

    // Note: restore position from saved state
    let position = player_state.position;

    // Spawn the players root entity
    let mut player = commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: world_assets.base_space_sheet.clone(),
        transform: Transform {
            translation: Vec3::new(position.x, position.y, 10.0),
            ..Default::default()
        },
        ..Default::default()
    });
    player
        .insert(Name::new("Player"))
        .insert(CleanupPlayerExplore)
        .insert(Player {
            active: true,
            jump_range: ship_info.jump,
            move_speed: ship_info.speed,
            rotate_speed: ROTATE_SPEED,
            ..Default::default()
        });

    // Resource Setup
    player.insert(HealthRecource {
        max: ship_info.health * 10,
        value: ship_info.health * 10,
    });
    player.insert(EnergyRecource {
        max: ship_info.energy * 10,
        value: ship_info.energy * 10,
    });

    // Movement Setup
    player
        .insert(GridTarget {
            mouse: Vec2::new(0.0, 0.0),
            target: Vec2::new(0.0, 0.0),
            moving: false,
        })
        .insert(GridMovement {
            cost: 0_25,
            speed: 6_00,
            distance: 4_00,
        });
}

////////////////////////////////
/// Player Movement Systems
////////////////////////////////

fn move_event_grid(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut GridTarget, &mut Transform)>,
) {
    let (player, mut move_to, mut transform) = player_query.single_mut();

    if !player.active {
        return;
    }

    let move_speed = player.move_speed * time.delta_seconds() * TILE_SIZE * 0.5;

    let mut moved = false;
    let mut target_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        target_y = 1.;
        moved = true;
    }
    if keyboard.pressed(KeyCode::S) {
        target_y = -1.;
        moved = true;
    }

    let mut target_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        target_x = -1.;
        moved = true;
    }
    if keyboard.pressed(KeyCode::D) {
        target_x = 1.;
        moved = true;
    }

    if moved {
        transform.translation =
            transform.translation + (Vec3::new(target_x, target_y, 0.0).normalize() * move_speed);

        move_to.target = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        };
    }
}

fn move_explore_grid(
    grid: Res<Grid>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut shift: ResMut<Shift64>,
    // mut event: ResMut<EventState>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut hex_event: EventWriter<StartHexEvent>,
    mut player_query: Query<(
        &Player,
        &mut GridTarget,
        &mut Transform,
        &mut EnergyRecource,
    )>,
    mut active_query: Query<(&mut Sprite, &mut Transform), (With<GridTargetHex>, Without<Player>)>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<PlayerCamera>, Without<Player>)>,
) {
    let (player, mut move_to, mut transform, mut energy) = player_query.single_mut();
    if !player.active {
        return;
    }

    // Get the primary window the camera renders to.
    let (camera, camera_transform) = camera_query.single();
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // Update the current target
    move_to.update_current(window, camera, camera_transform);

    let pos = Vec2 {
        x: transform.translation.x,
        y: transform.translation.y,
    };

    // Update the path to the targets Not sure if this is the way
    // It is here because we compute the mouse position in move to
    let (mut active_sprite, mut active_transform) = active_query.single_mut();
    active_transform.translation.x = move_to.mouse.x;
    active_transform.translation.y = move_to.mouse.y;

    // Note: Duplicated/Revesed math here
    // see move_to.update_current call above
    let hex = &grid.layout.hex_for(move_to.mouse);
    let dist = hex.distance(&grid.layout.hex_for(pos));
    let cost = (dist * ENERGY_COST) as u16;

    let can_jump = dist <= player.jump_range as i32;

    // Note: There is no need to rest this
    if can_jump && !move_to.moving && energy.value >= cost && grid.on_grid(hex) {
        active_sprite.color = Color::rgb(1., 1., 1.);
        if buttons.just_pressed(MouseButton::Left) {
            // FixMe: Start roling event dice,
            // cuting corners no dice animations.

            move_to.moving = true;
            move_to.set_current();
            buttons.clear();

            // Note Cleanup, Subtract cost

            if energy.value > cost {
                energy.value = energy.value - cost;
            } else {
                energy.value = 0;
            }
        }
    } else {
        active_sprite.color = Color::rgb(1., 0., 0.);
    }

    // TODO Move Above or replace with method call below
    // grid_movement.update_current(&active_entity, &move_to, &transform);

    if move_to.target.distance(pos) > 0.25 {
        let distance = move_to.target - pos;

        let move_speed = player.move_speed * time.delta_seconds() * TILE_SIZE;

        let direction = (distance / distance.length()) * move_speed;
        let target = transform.translation + Vec3::new(direction.x, direction.y, 0.0);
        if move_to.target.distance(pos) > move_speed {
            transform.translation = target.clone();
        } else {
            transform.translation =
                Vec3::new(move_to.target.x, move_to.target.y, transform.translation.z);
        }
    } else if move_to.moving {
        move_to.moving = false;
        hex_event.send(StartHexEvent {
            seed: shift.shift(),
        });
    }
}

fn move_player_camera(
    player_query: Query<(&Player, &Transform)>,
    mut camera_offset: ResMut<CameraOffset>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let mut cam_transform = camera_query.single_mut();
    let (_, player_transform) = player_query.single();

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
    camera_offset.value.x = player_transform.translation.x;
    camera_offset.value.y = player_transform.translation.y;
}

fn player_rotate_system(
    time: Res<Time>,
    windows: Res<Windows>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<PlayerCamera>, Without<Player>)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let (player, mut player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        // Convert window position to gpu coordinates
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let delta_x = world_pos.x - player_transform.translation.x;
        let delta_y = world_pos.y - player_transform.translation.y;
        let delta = delta_x.atan2(delta_y);

        // Rotate the sprite to look at the mouse position
        let q = Quat::from_axis_angle(-Vec3::Z, delta);
        player_transform.rotation = player_transform
            .rotation
            .slerp(q, player.rotate_speed * time.delta_seconds());
    }
}
