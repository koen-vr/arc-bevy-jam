use bevy::app::AppExit;

use crate::gui;
use crate::world;
use crate::*;

#[derive(Component)]
struct MainMenuCleanup;

#[derive(Component)]
struct MainMenuBtnExit;

#[derive(Component)]
struct MainMenuBtnEnter;

pub struct MainMenuPlugin;

#[cfg(not(target_arch = "wasm32"))]
const ENABLE_EXIT: bool = true;
#[cfg(target_arch = "wasm32")]
const ENABLE_EXIT: bool = false;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_buttons));
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_btn_exit));
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_btn_enter));

        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(enter_mainmenu));
        app.add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_mainmenu));
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

fn update_btn_enter(
    mut state: ResMut<State<AppState>>,
    mut enter_btn_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<MainMenuBtnEnter>),
    >,
) {
    for interaction in &mut enter_btn_query {
        match *interaction {
            Interaction::Clicked => {
                log::info!("update_btn_enter::clicked");
                state
                    .set(AppState::GameLoading)
                    .unwrap_or_else(|error| log::error!("Failed to set game state {}", error));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn update_btn_exit(
    mut exit: EventWriter<AppExit>,
    mut exit_btn_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<MainMenuBtnExit>),
    >,
) {
    for interaction in &mut exit_btn_query {
        match *interaction {
            Interaction::Clicked => {
                log::info!("update_btn_exit::clicked");
                exit.send(AppExit);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn enter_mainmenu(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    mut camera_offset: ResMut<CameraOffset>,
) {
    log::info!("enter_mainmenu");
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = world::CAMERA_ZOOM_EXPLORE;
    commands
        .spawn_bundle(camera_bundle)
        .insert(Name::new("gui-camera"))
        .insert(MainMenuCleanup)
        .insert(MainCamera);

    // Reset the camera offset
    camera_offset.value.x = 0.;
    camera_offset.value.y = 0.;

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(40.0)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("main-menu"))
        .insert(MainMenuCleanup)
        .id();

    let mut list = Vec::new();

    if ENABLE_EXIT {
        list.push(gui::create_button(
            &mut commands,
            gui::TEXT_BUTTON,
            gui::NORMAL_BUTTON,
            "Exit".into(),
            app_assets.gui_font.clone(),
            MainMenuBtnExit,
        ));
    }
    list.push(gui::create_button(
        &mut commands,
        gui::TEXT_BUTTON,
        gui::NORMAL_BUTTON,
        "start".into(),
        app_assets.gui_font.clone(),
        MainMenuBtnEnter,
    ));

    commands.entity(root).push_children(&list);
}

fn exit_mainmenu(mut commands: Commands, query: Query<Entity, With<MainMenuCleanup>>) {
    log::info!("exit_mainmenu");
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
