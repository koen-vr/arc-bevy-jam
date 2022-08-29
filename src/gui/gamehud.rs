use super::*;

use crate::gui;
use crate::world::*;

const KEY_CLICKED: &str = "ButtonKey::clicked:";
const FAILED_TO_SET_STATE: &str = "Failed to set game state";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonKey {
    BaseExit,
    EventExit,
    ExploreExit,
    EnterEvent,
    LeaveEvent,
    GameOver,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogKey {
    DialogTitle,
    DialogDescribe,
}

#[derive(Component)]
pub struct ButtonType {
    pub key: ButtonKey,
}

#[derive(Component)]
pub struct DialogType {
    pub key: DialogKey,
}

#[derive(Component)]
pub struct HudDialog;

#[derive(Component)]
pub struct HudNavigate;

#[derive(Component)]
pub struct HudCleanup;

pub struct GamehudPlugin;

impl Plugin for GamehudPlugin {
    fn build(&self, app: &mut App) {
        let game_over = AppState::GamePlay(GameMode::GameOver);

        let base_mode = AppState::GamePlay(GameMode::BaseGrid);
        let event_mode = AppState::GamePlay(GameMode::EventGrid);
        let explore_mode = AppState::GamePlay(GameMode::ExploreGrid);

        app.add_system_set(SystemSet::on_exit(game_over).with_system(exit_state));
        app.add_system_set(SystemSet::on_exit(game_over).with_system(exit_gameover));
        app.add_system_set(SystemSet::on_enter(game_over).with_system(enter_gameove));

        app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(base_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(base_mode).with_system(enter_base_gameplay));
        app.add_system_set(SystemSet::on_resume(base_mode).with_system(enter_base_gameplay));

        app.add_system_set(SystemSet::on_exit(event_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(event_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(event_mode).with_system(enter_event_gameplay));
        app.add_system_set(SystemSet::on_resume(event_mode).with_system(enter_event_gameplay));

        app.add_system_set(SystemSet::on_exit(explore_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(explore_mode).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(explore_mode).with_system(enter_explore_gameplay));
        app.add_system_set(SystemSet::on_resume(explore_mode).with_system(enter_explore_gameplay));

        app.add_system_set(
            SystemSet::on_update(game_over).with_system(button_update.label("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(base_mode).with_system(button_update.label("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(event_mode).with_system(button_update.label("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(explore_mode).with_system(button_update.label("gui-update")),
        );

        app.add_system_set(SystemSet::on_update(event_mode).with_system(on_gameover));
        app.add_system_set(SystemSet::on_update(explore_mode).with_system(on_gameover));
    }
}

fn exit_gameover(mut commands: Commands, query: Query<Entity, With<HudCleanup>>) {
    log::info!("exit_hud");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_gameove(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    mut player_query: Query<&mut Player>,
) {
    log::info!("enter_gameove");
    let player = player_query.single_mut();

    let node = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        120.,
        true,
        "exit".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::GameOver,
        },
    ));

    list.push(
        commands
            .spawn_bundle(TextBundle::from_section(
                player.message.clone(),
                TextStyle {
                    font: app_assets.gui_font.clone(),
                    font_size: 24.0,
                    color: gui::TEXT_BUTTON,
                },
            ))
            .id(),
    );

    list.push(
        commands
            .spawn_bundle(TextBundle::from_section(
                "Gameover",
                TextStyle {
                    font: app_assets.gui_font.clone(),
                    font_size: 42.0,
                    color: gui::TEXT_BUTTON,
                },
            ))
            .id(),
    );

    commands.entity(node).push_children(&list);
    //commands.entity(entity).push_children(&[node]);
}

fn exit_state(mut commands: Commands, query: Query<Entity, With<HudCleanup>>) {
    log::info!("exit_state");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn on_gameover(
    mut state: ResMut<State<AppState>>,
    mut game_over: EventReader<GameOverEvent>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();
    if player.active {
        return;
    }
    for ev in game_over.iter() {
        player.message = ev.message.clone();
        log::info!("on_gameover");
        state
            .push(AppState::GamePlay(GameMode::GameOver))
            .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
    }
}

fn button_update(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut hex_event: EventWriter<EndHexEvent>,
    diag_query: Query<&Children, With<HudDialog>>,
    navi_query: Query<&Children, With<HudNavigate>>,
    mut button_query: Query<
        (&Interaction, &ButtonType, &mut UiColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button_type, mut color, mut transform) in &mut button_query {
        match *interaction {
            Interaction::Clicked => {
                buttons.clear();
                transform.scale *= 1.05;
                *color = gui::PRESSED_BUTTON.into();
                let diag = match diag_query.get_single() {
                    Ok(diag) => Some(diag),
                    Err(_) => None,
                };
                let navi = match navi_query.get_single() {
                    Ok(navi) => Some(navi),
                    Err(_) => None,
                };
                handle_btn_update_click(
                    &mut commands,
                    button_type.key,
                    diag,
                    navi,
                    &mut state,
                    &mut hex_event,
                );
            }
            Interaction::Hovered => {
                transform.scale *= 0.95;
                *color = gui::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                transform.scale *= 1.05;
                *color = gui::NORMAL_BUTTON.into();
            }
        }
    }
}

fn handle_btn_update_click(
    commands: &mut Commands,
    button_key: ButtonKey,
    diag_children: Option<&Children>,
    navi_children: Option<&Children>,
    state: &mut ResMut<State<AppState>>,
    hex_event: &mut EventWriter<EndHexEvent>,
) {
    match button_key {
        ButtonKey::BaseExit => {
            log::info!("{} {}", KEY_CLICKED, "BaseExit");
            state
                .set(AppState::MainLoading)
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
        ButtonKey::EventExit => {
            log::info!("{} {}", KEY_CLICKED, "EventExit");
            state
                .pop()
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
        ButtonKey::ExploreExit => {
            log::info!("{} {}", KEY_CLICKED, "ExploreExit");
            // state
            //     .pop()
            //     .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
            state
                .set(AppState::MainLoading)
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
        ButtonKey::EnterEvent => {
            log::info!("{} {}", KEY_CLICKED, "EnterEvent");
            // TODO: Hook Up Leave event
            // player.active = true;
            state
                .push(AppState::GamePlay(GameMode::EventGrid))
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
            hex_event.send(EndHexEvent { enter: true });
        }
        ButtonKey::LeaveEvent => {
            log::info!("{} {}", KEY_CLICKED, "LeaveEvent");
            // TODO despawn dialog
            // TODO: Hook Up Leave event
            // player.active = true;
            if let Some(children) = diag_children {
                for entity in children {
                    commands.entity(entity.clone()).despawn_recursive();
                }
            }
            if let Some(children) = navi_children {
                for entity in children {
                    commands.entity(entity.clone()).despawn_recursive();
                }
            }
            hex_event.send(EndHexEvent { enter: false });
        }
        ButtonKey::GameOver => {
            log::info!("{} {}", KEY_CLICKED, "GameOver");
            state
                .set(AppState::MainLoading)
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
    }
}

fn enter_base_gameplay(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    world_assets: Res<WorldAssets>,
) {
    log::info!("enter_base_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let body = base_mode_select(&mut commands, &app_assets, &world_assets);

    let menu = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        65.,
        true,
        "<<".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::BaseExit,
        },
    ));

    commands.entity(menu).push_children(&list);

    commands.entity(root).push_children(&[menu, body]);
}

fn enter_event_gameplay(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    world_assets: Res<WorldAssets>,
) {
    log::info!("enter_event_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let body = event_mode_dialog(&mut commands, &app_assets, &world_assets);

    let menu = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let navi = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("hex-menu"))
        .insert(HudCleanup)
        .insert(HudNavigate)
        .id();

    let left = gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        65.,
        true,
        "<<".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::EventExit,
        },
    );

    let stats = event_mode_stats(&mut commands, &app_assets.gui_font);

    commands.entity(menu).push_children(&[left, navi, stats]);

    commands.entity(root).push_children(&[menu, body]);
}

fn enter_explore_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_explore_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(HudCleanup)
        .id();

    let body = explore_mode_dialog(&mut commands);

    let menu = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("hex-menu"))
        .insert(HudCleanup)
        .id();

    let navi = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("hex-menu"))
        .insert(HudCleanup)
        .insert(HudNavigate)
        .id();

    let exit = gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        65.,
        true,
        "<<".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::ExploreExit,
        },
    );
    let left = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(TILE_SIZE * 8.), Val::Percent(100.)),
                padding: UiRect::new(Val::Px(0.), Val::Px(12.), Val::Px(0.), Val::Px(0.)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .push_children(&[exit])
        .id();

    let stats = explore_mode_stats(&mut commands, &app_assets.gui_font);

    commands.entity(navi).push_children(&[]);
    commands.entity(menu).push_children(&[left, navi, stats]);
    commands.entity(root).push_children(&[menu, body]);
}
