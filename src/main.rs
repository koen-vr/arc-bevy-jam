use bevy::{prelude::*, render::texture::ImageSettings, window};

mod dev;
use dev::DevPlugin;

mod hexa;
use hexa::*;

mod map;
use map::HexmapPlugin;

mod player;
use player::PlayerPlugin;

mod xorshift;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const MOVE_SPEED: f32 = 6.0;
pub const ROTATE_SPEED: f32 = 24.0;

pub const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    // AssetLoading,
    UniverseMap,
    // SystemMap,
    // Combat,
}

#[derive(Component)]
struct MainCamera;

pub struct AssetsPugin;

pub struct SpaceSheet(pub Handle<TextureAtlas>);

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            resizable: false,
            title: "Arc Explorer".to_string(),
            position: window::WindowPosition::Automatic,
            mode: window::WindowMode::Windowed,
            present_mode: window::PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_state(GameState::UniverseMap)
        .add_plugins(DefaultPlugins)
        .add_plugin(DevPlugin)
        .add_plugin(AssetsPugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(HexmapPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

impl Plugin for AssetsPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_space_sheet);
    }
}

fn load_space_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load("base-space-sheet.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(126.0), 8, 6);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(SpaceSheet(atlas_handle));
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}
