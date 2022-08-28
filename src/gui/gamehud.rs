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
pub struct HudNavigate;

#[derive(Component)]
struct HudCleanup;

#[derive(Component)]
struct HudInitCleanup;

pub struct GamehudPlugin;

impl Plugin for GamehudPlugin {
    fn build(&self, app: &mut App) {
        let base_mode = AppState::GamePlay(GameMode::BaseGrid);
        let event_mode = AppState::GamePlay(GameMode::EventGrid);
        let explore_mode = AppState::GamePlay(GameMode::ExploreGrid);

        // app.add_system_set(SystemSet::on_exit(base_mode).with_system(exit_hud));
        // app.add_system_set(SystemSet::on_enter(base_mode).with_system(enter_hud));

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
            SystemSet::on_update(base_mode).with_system(button_update.label("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(event_mode).with_system(button_update.label("gui-update")),
        );
        app.add_system_set(
            SystemSet::on_update(explore_mode).with_system(button_update.label("gui-update")),
        );
    }
}

// fn exit_hud(mut commands: Commands, query: Query<Entity, With<HudInitCleanup>>) {
//     log::info!("exit_hud");
//     for e in query.iter() {
//         commands.entity(e).despawn_recursive();
//     }
// }

// fn enter_hud(mut commands: Commands) {
//     log::info!("enter_hud");
// }

fn exit_state(mut commands: Commands, query: Query<Entity, With<HudCleanup>>) {
    log::info!("exit_state");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn button_update(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut hex_event: EventWriter<EndHexEvent>,
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
                let navi = match navi_query.get_single() {
                    Ok(navi) => Some(navi),
                    Err(_) => None,
                };
                handle_btn_update_click(
                    &mut commands,
                    button_type.key,
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
            if let Some(children) = navi_children {
                for entity in children {
                    commands.entity(entity.clone()).despawn_recursive();
                }
            }
            hex_event.send(EndHexEvent { enter: false });
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

fn enter_event_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
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
            key: ButtonKey::EventExit,
        },
    ));

    commands.entity(menu).push_children(&list);

    commands.entity(root).push_children(&[menu]);
}

fn enter_explore_gameplay(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    world_assets: Res<WorldAssets>,
) {
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

    let body = explore_mode_dialog(&mut commands, &app_assets, &world_assets);

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
                justify_content: JustifyContent::FlexStart,
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

    let stats = explore_mode_stats(&mut commands, &app_assets.gui_font);

    commands.entity(navi).push_children(&[]);
    commands.entity(menu).push_children(&[exit, navi, stats]);
    commands.entity(root).push_children(&[menu, body]);
}
