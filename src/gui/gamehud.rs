use crate::gui;
use crate::*;

#[derive(AssetCollection)]
struct GameAssets {}

#[derive(Component)]
struct HexGameCleanup;

#[derive(Component)]
struct HexGameBtnExit;

#[derive(Component)]
struct EventGameCleanup;

#[derive(Component)]
struct EventGameBtnExit;

pub struct GamehudPlugin;

impl Plugin for GamehudPlugin {
    fn build(&self, app: &mut App) {
        // Add game loading state
        app.add_loading_state(
            LoadingState::new(AppState::GameLoading)
                .continue_to_state(AppState::GamePlay(GameMode::HexGrid))
                .with_collection::<GameAssets>(),
        );

        let hex_grind = AppState::GamePlay(GameMode::HexGrid);
        let event_grind = AppState::GamePlay(GameMode::EventGrid);
        app.add_system_set(SystemSet::on_update(hex_grind).with_system(update_buttons));
        app.add_system_set(SystemSet::on_update(event_grind).with_system(update_buttons));

        app.add_system_set(SystemSet::on_update(hex_grind).with_system(hex_update_exit));
        app.add_system_set(SystemSet::on_update(event_grind).with_system(event_update_exit));

        app.add_system_set(SystemSet::on_enter(hex_grind).with_system(enter_hex_gameplay));
        app.add_system_set(SystemSet::on_enter(event_grind).with_system(enter_event_gameplay));

        app.add_system_set(SystemSet::on_exit(hex_grind).with_system(exit_hex_gameplay));
        app.add_system_set(SystemSet::on_exit(event_grind).with_system(exit_event_gameplay));
    }
}

fn update_buttons(
    mut button_query: Query<
        (&Interaction, &mut UiColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut transform) in &mut button_query {
        match *interaction {
            Interaction::Clicked => {
                *color = gui::PRESSED_BUTTON.into();
                transform.scale *= 1.05;
            }
            Interaction::Hovered => {
                *color = gui::HOVERED_BUTTON.into();
                transform.scale *= 0.95;
            }
            Interaction::None => {
                *color = gui::NORMAL_BUTTON.into();
                transform.scale *= 1.05;
            }
        }
    }
}

fn hex_update_exit(
    mut state: ResMut<State<AppState>>,
    mut exit_btn_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<HexGameBtnExit>),
    >,
) {
    for interaction in &mut exit_btn_query {
        match *interaction {
            Interaction::Clicked => {
                log::info!("hex_update_exit::clicked");
                state
                    .set(AppState::MainLoading)
                    .unwrap_or_else(|error| log::error!("Failed to set game state {}", error));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn event_update_exit(
    mut state: ResMut<State<AppState>>,
    mut exit_btn_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<EventGameBtnExit>),
    >,
) {
    for interaction in &mut exit_btn_query {
        match *interaction {
            Interaction::Clicked => {
                log::info!("event_update_exit::clicked");
                state
                    .set(AppState::GamePlay(GameMode::HexGrid))
                    .unwrap_or_else(|error| log::error!("Failed to set game state {}", error));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn enter_hex_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_hex_gameplay");

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("hex-camera"))
        .insert(HexGameCleanup);

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                //size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(100.0),
                    bottom: Val::Percent(0.0),
                }, // UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("hex-menu"))
        .insert(HexGameCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Back".into(),
        app_assets.gui_font.clone(),
        HexGameBtnExit,
    ));

    commands.entity(root).push_children(&list);
}

fn exit_hex_gameplay(mut commands: Commands, query: Query<Entity, With<HexGameCleanup>>) {
    log::info!("exit_hex_gameplay");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn enter_event_gameplay(mut commands: Commands, app_assets: Res<AppAssets>) {
    log::info!("enter_event_gameplay");

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("event-camera"))
        .insert(EventGameCleanup);

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                //size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(100.0),
                    bottom: Val::Percent(0.0),
                }, // UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("event-menu"))
        .insert(EventGameCleanup)
        .id();

    let mut list = Vec::new();

    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "Back".into(),
        app_assets.gui_font.clone(),
        EventGameBtnExit,
    ));

    commands.entity(root).push_children(&list);
}

fn exit_event_gameplay(mut commands: Commands, query: Query<Entity, With<EventGameCleanup>>) {
    log::info!("exit_event_gameplay");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
