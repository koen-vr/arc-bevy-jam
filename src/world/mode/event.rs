use super::*;

#[derive(Component, Default)]
pub struct Laser {
    timeout: Timer,
    direction: Vec3,
}

#[derive(Component, Default)]
pub struct Enemy {
    pub hp: i32,
}

#[derive(Component, Default)]
pub struct PlayerLaser;

#[derive(Component, Default)]
pub struct EnemyLaser;

#[derive(Component, Default)]
pub struct CleanupEvent;

pub struct EventModePlugin;

// TODO Energy consumption for lasers
// TODO Energy consumption for movement

impl Plugin for EventModePlugin {
    fn build(&self, app: &mut App) {
        let event_grid = AppState::GamePlay(GameMode::EventGrid);

        app.add_system_set(SystemSet::on_exit(event_grid).with_system(exit_event_gameplay));
        app.add_system_set(SystemSet::on_enter(event_grid).with_system(enter_event_gameplay));

        app.add_system_set(SystemSet::on_update(event_grid).with_system(player_fire_system));

        app.add_system_set(
            SystemSet::on_update(event_grid)
                .with_system(lasers_movement)
                .after("player-move")
                .label("laser-move"),
        );
        app.add_system_set(
            SystemSet::on_update(event_grid)
                .with_system(lasers_enemy_hits)
                .after("laser-move"),
        );
        app.add_system_set(
            SystemSet::on_update(event_grid)
                .with_system(lasers_player_hits)
                .after("laser-move"),
        );
    }
}

fn exit_event_gameplay(mut commands: Commands, query: Query<Entity, With<CleanupEvent>>) {
    log::info!("exit_event_gameplay");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_event_gameplay(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    world_assets: Res<WorldAssets>,
    mut player_query: Query<(&mut Player, &Transform)>,
) {
    log::info!("enter_event_gameplay");
    let (mut player, transform) = player_query.single_mut();

    match grid.key {
        EventKey::None => (),
        EventKey::Combat => spawn_combat_event(
            &mut commands,
            &world_assets,
            &grid.get_event_combat(),
            Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
        ),
        EventKey::Energy => (),
        EventKey::Mining => spawn_mining_event(&mut commands, &grid.get_event_mining()),
    }

    // TODO Spawn Enemies
}

fn spawn_combat_event(
    commands: &mut Commands,
    assets: &Res<WorldAssets>,
    action: &CombatAction,
    center: Vec2,
) {
    let mut sprite = TextureAtlasSprite::new(12);
    sprite.color = Color::rgb(0.9, 0.6, 0.9);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE * 0.5));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: assets.base_space_sheet.clone(),
            transform: Transform {
                translation: Vec3::new(center.x + (TILE_SIZE * 5.), center.y, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy { hp: 5 })
        .insert(CleanupEvent);
}

fn spawn_mining_event(commands: &mut Commands, action: &MiningAction) {
    // TODO Implement
}

////////////////////////
/// Gamehud Extentions
////////////////////////

pub(crate) fn event_mode_dialog(
    commands: &mut Commands,
    app_assets: &Res<AppAssets>,
    world_assets: &Res<WorldAssets>,
) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexStart,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-dialog"))
        .id();

    root
}

pub(crate) fn lasers_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Laser, &mut Transform), With<CleanupEvent>>,
) {
    for (entity, mut laser, mut transform) in query.iter_mut() {
        laser.timeout.tick(time.delta());
        if laser.timeout.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            let move_speed = 12. * time.delta_seconds() * TILE_SIZE * 0.5;
            transform.translation = transform.translation + (laser.direction * move_speed);
        }
    }
}

pub(crate) fn lasers_enemy_hits(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Laser, &mut Transform), With<EnemyLaser>>,
) {
    for (entity, mut laser, mut transform) in query.iter_mut() {
        // TODO Implement
    }
}

pub(crate) fn lasers_player_hits(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Laser, &mut Transform), With<PlayerLaser>>,
    mut enemy_query: Query<
        (Entity, &mut Enemy, &mut Transform),
        (With<CleanupEvent>, Without<PlayerLaser>),
    >,
) {
    let max_dist = TILE_SIZE * 0.25;
    for (l_entity, mut laser, mut l_transform) in laser_query.iter_mut() {
        let pos = Vec2 {
            x: l_transform.translation.x,
            y: l_transform.translation.y,
        };
        for (e_entity, mut enemy, mut e_transform) in enemy_query.iter_mut() {
            let dist = pos.distance(Vec2 {
                x: e_transform.translation.x,
                y: e_transform.translation.y,
            });

            if max_dist > dist {
                commands.entity(l_entity).despawn_recursive();

                enemy.hp = enemy.hp - 1;
                if enemy.hp < 1 {
                    commands.entity(e_entity).despawn_recursive();
                    // TODO Update Combat stats
                }
            }
        }
    }
}

pub(crate) fn player_fire_system(
    mut commands: Commands,
    windows: Res<Windows>,
    world_assets: Res<WorldAssets>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<PlayerCamera>, Without<Player>)>,
) {
    let (player, mut player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        log::info!("... event fire ...");
        buttons.clear();
        // FIXMe: This code is in 3 spot: player fire player rotate, move_explore_grid
        // Get the primary window the camera renders to.
        let (camera, camera_transform) = camera_query.single();
        let wnd = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };
        if let Some(screen_pos) = wnd.cursor_position() {
            let mut laser = TextureAtlasSprite::new(47);
            laser.custom_size = Some(Vec2 {
                x: TILE_SIZE * 0.05,
                y: TILE_SIZE * 0.5,
            });

            // Convert window position to gpu coordinates
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let delta_x = world_pos.x - player_transform.translation.x;
            let delta_y = world_pos.y - player_transform.translation.y;
            let delta = delta_x.atan2(delta_y);

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: laser,
                    texture_atlas: world_assets.base_space_sheet.clone(),
                    transform: Transform {
                        rotation: Quat::from_axis_angle(-Vec3::Z, delta),
                        translation: Vec3 {
                            x: player_transform.translation.x,
                            y: player_transform.translation.y,
                            z: 4.0,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CleanupEvent)
                .insert(PlayerLaser)
                .insert(Laser {
                    direction: Vec3::new(delta_x, delta_y, 0.0).normalize(),
                    timeout: Timer::from_seconds(1.6, false),
                });
        }
    }
}
