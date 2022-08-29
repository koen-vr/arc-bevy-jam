use super::*;

use crate::gui::gamehud::*;

#[derive(Component, Default)]
pub struct Laser {
    timeout: Timer,
    direction: Vec3,
}

#[derive(Component, Default)]
pub struct Enemy {
    pub hp: i32,
    pub timeout: Timer,
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
        let game_over = AppState::GamePlay(GameMode::GameOver);
        let event_mode = AppState::GamePlay(GameMode::EventGrid);

        app.add_system_set(SystemSet::on_exit(game_over).with_system(exit_event_gameplay));

        app.add_system_set(SystemSet::on_exit(event_mode).with_system(exit_event_gameplay));
        app.add_system_set(SystemSet::on_enter(event_mode).with_system(enter_event_gameplay));

        app.add_system_set(SystemSet::on_update(event_mode).with_system(enemy_fire_system));
        app.add_system_set(SystemSet::on_update(event_mode).with_system(player_fire_system));

        app.add_system_set(
            SystemSet::on_update(event_mode)
                .with_system(lasers_movement)
                .after("player-move")
                .label("laser-move"),
        );
        app.add_system_set(
            SystemSet::on_update(event_mode)
                .with_system(lasers_enemy_hits)
                .after("laser-move"),
        );
        app.add_system_set(
            SystemSet::on_update(event_mode)
                .with_system(lasers_player_hits)
                .after("laser-move"),
        );

        app.add_system_set(SystemSet::on_update(event_mode).with_system(update_health_text));
        app.add_system_set(SystemSet::on_update(event_mode).with_system(update_energy_text));
    }
}

fn update_health_text(
    stats_query: Query<&HealthRecource, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let stat = stats_query.single();
    let mut text = text_query.single_mut();

    if text.sections.len() == 0 {
        return;
    }
    let max = stat.max.to_string();
    let value = stat.value.to_string();
    text.sections[0].value = format!("{value}/{max}");
}
fn update_energy_text(
    stats_query: Query<&EnergyRecource, With<Player>>,
    mut text_query: Query<&mut Text, With<EnergyText>>,
) {
    let stat = stats_query.single();
    let mut text = text_query.single_mut();

    if text.sections.len() == 0 {
        return;
    }
    let max = stat.max.to_string();
    let value = stat.value.to_string();
    text.sections[0].value = format!("{value}/{max}");
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
    let mut rng = Shift64::new(rand::random());
    match grid.key {
        EventKey::None => (),
        EventKey::Combat => spawn_combat_event(
            &mut commands,
            &mut rng,
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
    rng: &mut Shift64,
    assets: &Res<WorldAssets>,
    action: &CombatAction,
    center: Vec2,
) {
    let mut rng = Shift64::new(rand::random());
    let mut small_ship = TextureAtlasSprite::new(12);
    small_ship.color = Color::rgb(0.9, 0.7, 0.9);
    small_ship.custom_size = Some(Vec2::splat(TILE_SIZE * 0.5));

    let mut big_ship = TextureAtlasSprite::new(14);
    big_ship.color = Color::rgb(0.9, 0.7, 0.9);
    big_ship.custom_size = Some(Vec2::splat(TILE_SIZE * 0.6));

    let mut enemies = action.enemies;
    if action.is_large {
        enemies = enemies - 1;
        let position = Vec2 {
            x: center.x + rng.f32(TILE_SIZE * 12.) - (TILE_SIZE * 6.),
            y: center.y + rng.f32(TILE_SIZE * 12.) - (TILE_SIZE * 6.),
        };
        spawn_combat_event_big(
            commands,
            position,
            assets.base_space_sheet.clone(),
            big_ship.clone(),
        );
    }
    for _ in 0..enemies {
        let position = Vec2 {
            x: center.x + rng.f32(TILE_SIZE * 12.) - (TILE_SIZE * 6.),
            y: center.y + rng.f32(TILE_SIZE * 12.) - (TILE_SIZE * 6.),
        };
        spawn_combat_event_small(
            commands,
            (0.01 * rng.f32(240.)),
            position,
            assets.base_space_sheet.clone(),
            small_ship.clone(),
        );
    }
}

fn spawn_combat_event_small(
    commands: &mut Commands,
    time: f32,
    position: Vec2,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas,
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy {
            hp: 4,
            timeout: Timer::from_seconds(1.2 + time, false),
        })
        .insert(CleanupEvent);
}

fn spawn_combat_event_big(
    commands: &mut Commands,
    position: Vec2,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas,
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy {
            hp: 8,
            timeout: Timer::from_seconds(1.2, false),
        })
        .insert(CleanupEvent);
}

fn spawn_mining_event(commands: &mut Commands, action: &MiningAction) {
    // TODO Implement
    log::warn!("not implemented")
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
        .insert(HudDialog)
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
            let move_speed = 12. * time.delta_seconds() * TILE_SIZE * 0.4;
            transform.translation = transform.translation + (laser.direction * move_speed);
        }
    }
}

pub(crate) fn lasers_enemy_hits(
    time: Res<Time>,
    mut commands: Commands,
    mut game_over: EventWriter<GameOverEvent>,
    mut laser_query: Query<
        (Entity, &mut Laser, &mut Transform),
        (With<EnemyLaser>, Without<Player>),
    >,
    mut player_query: Query<
        (&mut Player, &mut HealthRecource, &mut Transform),
        Without<EnemyLaser>,
    >,
) {
    let (mut player, mut player_health, player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }
    let max_dist = TILE_SIZE * 0.25;
    for (entity, mut _laser, transform) in laser_query.iter_mut() {
        let pos = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        };
        let dist = pos.distance(Vec2 {
            x: player_transform.translation.x,
            y: player_transform.translation.y,
        });

        if max_dist > dist {
            commands.entity(entity).despawn_recursive();

            if (player_health.value > 0) {
                player_health.value = player_health.value - 1;
            }
            log::info!("health: {}", player_health.value);
            if player_health.value < 1 {
                player.active = false;
                game_over.send(GameOverEvent {
                    message: "Your ship was destroyed.".to_string(),
                })
            }
        }
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

pub(crate) fn enemy_fire_system(
    time: Res<Time>,
    mut commands: Commands,
    world_assets: Res<WorldAssets>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), Without<Player>>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Enemy>>,
) {
    let mut rng = Shift64::new(rand::random());
    let (player, mut player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }
    let mut laser = TextureAtlasSprite::new(46);
    laser.custom_size = Some(Vec2 {
        x: TILE_SIZE * 0.05,
        y: TILE_SIZE * 0.5,
    });
    for (mut enemy, mut enemy_transform) in enemy_query.iter_mut() {
        // Update timer
        enemy.timeout.tick(time.delta());
        if enemy.timeout.finished() {
            enemy.timeout = Timer::from_seconds(0.6 + (0.01 * rng.f32(160.)), false);
            let from = Vec2 {
                x: enemy_transform.translation.x,
                y: enemy_transform.translation.y,
            };
            spawn_enemy_laser(
                &mut commands,
                from,
                Vec2 {
                    x: player_transform.translation.x - from.x,
                    y: player_transform.translation.y - from.y,
                },
                world_assets.base_space_sheet.clone(),
                laser.clone(),
            )
        }
    }
}

fn spawn_enemy_laser(
    commands: &mut Commands,
    from: Vec2,
    to: Vec2,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
) {
    let delta = to.x.atan2(to.y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas,
            transform: Transform {
                rotation: Quat::from_axis_angle(-Vec3::Z, delta),
                translation: Vec3 {
                    x: from.x,
                    y: from.y,
                    z: 5.0,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanupEvent)
        .insert(EnemyLaser)
        .insert(Laser {
            direction: Vec3::new(to.x, to.y, 0.0).normalize(),
            timeout: Timer::from_seconds(1.6, false),
        });
}

pub(crate) fn player_fire_system(
    mut commands: Commands,
    windows: Res<Windows>,
    world_assets: Res<WorldAssets>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut game_over: EventWriter<GameOverEvent>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut EnergyRecource)>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<PlayerCamera>, Without<Player>)>,
) {
    let (mut player, mut player_transform, mut energy) = player_query.single_mut();
    if !player.active {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        buttons.clear();

        if (energy.value > 0) {
            energy.value = energy.value - 1;
        }
        if energy.value < 1 {
            player.active = false;
            game_over.send(GameOverEvent {
                message: "No more energy, your ship is stranded.".to_string(),
            })
        }

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

pub(crate) fn event_mode_stats(commands: &mut Commands, font: &Handle<Font>) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(TILE_SIZE * 8.), Val::Percent(100.)),
                padding: UiRect::new(Val::Px(0.), Val::Px(12.), Val::Px(0.), Val::Px(0.)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("explore-stats"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 1.8), Val::Px(65.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "hp:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent
                        .spawn_bundle(TextBundle::from_section(
                            format!("80/80"),
                            TextStyle {
                                font: font.clone(),
                                font_size: 24.0,
                                color: gui::TEXT_BUTTON,
                            },
                        ))
                        .insert(HealthText);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.4), Val::Px(65.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "energy:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent
                        .spawn_bundle(TextBundle::from_section(
                            //format!("{0}/{0}"),
                            format!("120/120"),
                            TextStyle {
                                font: font.clone(),
                                font_size: 24.0,
                                color: gui::TEXT_BUTTON,
                            },
                        ))
                        .insert(EnergyText);
                });
        })
        .id();

    root
}
