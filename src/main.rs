use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::texture::ImageSettings;
use bevy::utils::HashMap;
use bevy::{log, window};

use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::Inspectable;

const GAMENAME: &str = "Arc Raiders";
const GAMECLEAR: Color = Color::rgb(0.03137254902, 0.0, 0.05882352941);

mod gui;

mod tool;
use tool::debug;
use tool::xorshift::*;

mod world;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraOffset {
    value: Vec2,
}

#[derive(Component, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameMode {
    GameOver,
    BaseGrid,
    EventGrid,
    ExploreGrid,
}

#[derive(Component, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    MainLoading,
    MainMenu,
    GameLoading,
    GamePlay(GameMode),
    Credits,
}

#[derive(AssetCollection)]
struct AppAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    gui_font: Handle<Font>,
}

fn main() {
    let mut app = App::new();

    // Setup game engine
    app.insert_resource(ImageSettings::default_nearest());
    app.insert_resource(ClearColor(GAMECLEAR));
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        resizable: false,
        title: GAMENAME.into(),
        position: window::WindowPosition::Automatic,
        mode: window::WindowMode::Windowed,
        present_mode: window::PresentMode::AutoVsync,
        ..Default::default()
    });
    app.insert_resource(CameraOffset {
        value: Vec2::default(),
    });

    // Set state to MainLoading
    app.add_state(AppState::MainLoading).add_loading_state(
        LoadingState::new(AppState::MainLoading)
            .continue_to_state(AppState::MainMenu)
            .with_collection::<AppAssets>(),
    );

    app.add_plugins(DefaultPlugins);
    app.add_plugin(debug::DebugPlugin);
    app.add_plugin(world::WorldPlugin);
    app.add_plugin(gui::GuiPlugin);

    app.run();
}
