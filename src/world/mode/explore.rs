use super::*;

use crate::gui::gamehud::*;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct EnergyText;

#[derive(Clone, Copy, Default, Debug)]
pub struct EndHexEvent {
    pub enter: bool,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct StartHexEvent {
    pub seed: i64,
}

pub struct ExploreModePlugin;

impl Plugin for ExploreModePlugin {
    fn build(&self, app: &mut App) {
        let explore_mode = AppState::GamePlay(GameMode::ExploreGrid);

        app.add_event::<EndHexEvent>();
        app.add_event::<StartHexEvent>();

        // app.add_system_set(SystemSet::on_exit(explore_grid).with_system(exit_explore_gameplay));
        // app.add_system_set(SystemSet::on_enter(explore_grid).with_system(enter_explore_gameplay));

        app.add_system_set(SystemSet::on_update(explore_mode).with_system(update_health_text));
        app.add_system_set(SystemSet::on_update(explore_mode).with_system(update_energy_text));

        // Event Systems
        app.add_system_set(
            SystemSet::on_update(explore_mode)
                .with_system(on_end_hex_event.after("gui-update").after("player-move")),
        );
        app.add_system_set(
            SystemSet::on_update(explore_mode)
                .with_system(on_start_hex_event.after("gui-update").after("player-move")),
        );
    }
}

// fn exit_explore_gameplay(mut commands: Commands) {
//     log::info!("exit_explore_gameplay");
// }

// fn enter_explore_gameplay(mut commands: Commands) {
//     log::info!("enter_explore_gameplay");
// }

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

////////////////////////////////
/// Handle Exploration Events
////////////////////////////////

fn on_end_hex_event(
    mut commands: Commands,
    mut game_over: EventWriter<GameOverEvent>,
    mut end_hex_event: EventReader<EndHexEvent>,
    mut grid: ResMut<Grid>,
    player_state: Res<PlayerState>,
    mut player_query: Query<(&mut Player, &mut EnergyRecource)>,
) {
    for ev in end_hex_event.iter() {
        // TODO: Destroy the resource if found
        let (mut player, mut energy) = player_query.single_mut();
        let hex = grid.get_hex(player_state.position);

        if !ev.enter && EventKey::Energy == grid.get_event_key(hex) {
            let event = grid.get_event_energy();
            energy.value = energy.value + event.energy;
            energy.value = energy.value.clamp(0, energy.max);
            if energy.value < 1 {
                player.active = false;
                game_over.send(GameOverEvent {
                    message: "No more energy, your ship is stranded.".to_string(),
                })
            }
        }

        if let Some(entity) = grid.clr_node(&hex) {
            commands.entity(entity).despawn_recursive();
        }
        player.active = true;
    }
}

fn on_start_hex_event(
    mut commands: Commands,
    assets: Res<AppAssets>,
    events: Res<GridEvents>,
    mut grid: ResMut<Grid>,
    mut player_state: ResMut<PlayerState>,
    mut start_hex_event: EventReader<StartHexEvent>,
    mut player_query: Query<(&mut Player, &Transform)>,
    dialog_query: Query<Entity, With<HudDialog>>,
    navigate_query: Query<Entity, With<HudNavigate>>,
) {
    for ev in start_hex_event.iter() {
        let (mut player, transform) = player_query.single_mut();
        player.active = false;
        player_state.position = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        };
        let dialog = dialog_query.single();
        let navigate = navigate_query.single();

        // FixMe This is messy and does not scale
        let hex = grid.get_hex(player_state.position);
        match grid.get_event_key(hex) {
            EventKey::None => grid.clr_event(),
            EventKey::Combat => grid.set_event_combat(events.roll_combat_table(ev.seed)),
            EventKey::Energy => grid.set_event_energy(events.roll_energy_table(ev.seed)),
            EventKey::Mining => grid.set_event_mining(events.roll_mining_table(ev.seed)),
        };

        handle_enter_hex_event(&mut commands, &grid, &assets, dialog, navigate);
    }
}

fn handle_enter_hex_event(
    commands: &mut Commands,
    grid: &ResMut<Grid>,
    assets: &Res<AppAssets>,
    dialog: Entity,
    navigate: Entity,
) {
    let act = GridEvents::get_actions(grid.key);
    let data = grid.get_event_data();
    log::info!(data.title);

    let title = commands
        .spawn_bundle(TextBundle::from_section(
            data.title,
            TextStyle {
                font: assets.gui_font.clone(),
                font_size: 40.0,
                color: gui::TEXT_BUTTON,
            },
        ))
        .insert(HudCleanup)
        .id();
    let descr = commands
        .spawn_bundle(TextBundle::from_section(
            data.descr,
            TextStyle {
                font: assets.gui_font.clone(),
                font_size: 20.0,
                color: gui::TEXT_BUTTON,
            },
        ))
        .insert(HudCleanup)
        .id();

    commands.entity(dialog).push_children(&[descr, title]);

    // FixMe Ugly cheat to fix empty events
    let mut text = act.leave;
    if !data.enter && grid.key != EventKey::Energy {
        text = "leave".to_string();
    }
    let leave = gui::create_button(
        commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        130.,
        true,
        text,
        assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::LeaveEvent,
        },
    );

    if data.enter {
        let enter = gui::create_button(
            commands,
            gui::TEXT_BUTTON,
            gui::NORMAL_BUTTON,
            130.,
            true,
            act.enter,
            assets.gui_font.clone(),
            ButtonType {
                key: ButtonKey::EnterEvent,
            },
        );
        commands.entity(navigate).push_children(&[enter, leave]);
    } else {
        commands.entity(navigate).push_children(&[leave]);
    }
}

////////////////////////
/// Gamehud Extentions
////////////////////////

pub(crate) fn explore_mode_stats(commands: &mut Commands, font: &Handle<Font>) -> Entity {
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

pub(crate) fn explore_mode_dialog(commands: &mut Commands) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                margin: UiRect::all(Val::Auto),
                padding: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(128.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("explore-dialog"))
        .insert(HudDialog)
        .id();

    root
}
