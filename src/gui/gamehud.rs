use crate::gui;
use crate::*;

const KEY_CLICKED: &str = "ButtonKey::clicked:";
const FAILED_TO_SET_STATE: &str = "Failed to set game state";

#[derive(Component, Clone, Copy)]
enum ButtonKey {
    BaseExit,
    BaseEnter,
    EventExit,
    ExploreExit,
    ExploreEnter,
}

#[derive(Component)]
struct ButtonType {
    key: ButtonKey,
}

#[derive(Component)]
struct HudCleanup;

#[derive(Component)]
struct GamehudCleanup;

pub struct GamehudPlugin;

impl Plugin for GamehudPlugin {
    fn build(&self, app: &mut App) {
        let base_grid = AppState::GamePlay(GameMode::BaseGrid);
        let event_grid = AppState::GamePlay(GameMode::EventGrid);
        let explore_grid = AppState::GamePlay(GameMode::ExploreGrid);

        app.add_system_set(SystemSet::on_exit(base_grid).with_system(exit_hud));
        app.add_system_set(SystemSet::on_enter(base_grid).with_system(enter_hud));

        app.add_system_set(SystemSet::on_exit(base_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(base_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(base_grid).with_system(enter_base_gameplay));
        app.add_system_set(SystemSet::on_resume(base_grid).with_system(enter_base_gameplay));

        app.add_system_set(SystemSet::on_exit(event_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(event_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(event_grid).with_system(enter_event_gameplay));
        app.add_system_set(SystemSet::on_resume(event_grid).with_system(enter_event_gameplay));

        app.add_system_set(SystemSet::on_exit(explore_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_pause(explore_grid).with_system(exit_state));
        app.add_system_set(SystemSet::on_enter(explore_grid).with_system(enter_explore_gameplay));
        app.add_system_set(SystemSet::on_resume(explore_grid).with_system(enter_explore_gameplay));

        app.add_system_set(SystemSet::on_update(base_grid).with_system(button_update));
        app.add_system_set(SystemSet::on_update(event_grid).with_system(button_update));
        app.add_system_set(SystemSet::on_update(explore_grid).with_system(button_update));
    }
}

fn exit_hud(mut commands: Commands, query: Query<Entity, With<HudCleanup>>) {
    log::info!("exit_state");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_hud(mut commands: Commands) {
    log::info!("enter_gamehud");

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("gamehud-camera"))
        .insert(HudCleanup);
}

fn exit_state(mut commands: Commands, query: Query<Entity, With<GamehudCleanup>>) {
    log::info!("exit_state");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn button_update(
    mut state: ResMut<State<AppState>>,
    mut button_query: Query<
        (&Interaction, &ButtonType, &mut UiColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button_type, mut color, mut transform) in &mut button_query {
        match *interaction {
            Interaction::Clicked => {
                *color = gui::PRESSED_BUTTON.into();
                transform.scale *= 1.05;
                handle_btn_update_click(button_type.key, &mut state);
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

fn handle_btn_update_click(button_key: ButtonKey, state: &mut ResMut<State<AppState>>) {
    match button_key {
        ButtonKey::BaseExit => {
            log::info!("{} {}", KEY_CLICKED, "BaseExit");
            state
                .set(AppState::MainLoading)
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
        ButtonKey::BaseEnter => {
            log::info!("{} {}", KEY_CLICKED, "BaseEnter");
            state
                .push(AppState::GamePlay(GameMode::ExploreGrid))
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
            state
                .pop()
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
        ButtonKey::ExploreEnter => {
            log::info!("{} {}", KEY_CLICKED, "ExploreEnter");
            state
                .push(AppState::GamePlay(GameMode::EventGrid))
                .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
        }
    }
}

//////////

fn enter_base_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_base_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(100.0),
                    bottom: Val::Percent(0.0),
                },
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(GamehudCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Back".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::BaseExit,
        },
    ));
    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Enter".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::BaseEnter,
        },
    ));

    commands.entity(root).push_children(&list);
}

fn enter_event_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_event_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(100.0),
                    bottom: Val::Percent(0.0),
                },
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(GamehudCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Back".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::EventExit,
        },
    ));

    commands.entity(root).push_children(&list);
}

fn enter_explore_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_explore_gameplay");

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(100.0),
                    bottom: Val::Percent(0.0),
                },
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("hex-menu"))
        .insert(GamehudCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Back".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::ExploreExit,
        },
    ));
    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Enter".into(),
        app_assets.gui_font.clone(),
        ButtonType {
            key: ButtonKey::ExploreEnter,
        },
    ));

    commands.entity(root).push_children(&list);
}
