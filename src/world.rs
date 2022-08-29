use crate::*;

mod bg;
use bg::*;

mod grid;
use grid::*;

pub mod mode;
pub use mode::*;

pub mod player;
pub use player::*;

pub const TILE_SIZE: f32 = 64.0;

pub const ROTATE_SPEED: f32 = 24.0;

pub const CAMERA_ZOOM_EVENT: f32 = 0.9;
pub const CAMERA_ZOOM_EXPLORE: f32 = 1.8;

pub struct GameOverEvent {
    pub message: String,
}

#[derive(AssetCollection)]
pub struct PlayerAssets {}

#[derive(AssetCollection)]
pub struct WorldAssets {
    #[asset(texture_atlas(
        tile_size_x = 128.,
        tile_size_y = 128.,
        columns = 8,
        rows = 6,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "base-space-sheet.png")]
    base_space_sheet: Handle<TextureAtlas>,
    #[asset(path = "hex-pointy-64.1.png")]
    pointy_hex64_a: Handle<Image>,
    #[asset(path = "hex-pointy-64.2.png")]
    pointy_hex64_b: Handle<Image>,
    #[asset(path = "Ship-7.png")]
    ship_7: Handle<Image>,
    #[asset(path = "Ship-17.png")]
    ship_17: Handle<Image>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Add game loading state
        app.add_loading_state(
            LoadingState::new(AppState::GameLoading)
                .continue_to_state(AppState::GamePlay(GameMode::BaseGrid))
                .with_collection::<PlayerAssets>()
                .with_collection::<WorldAssets>(),
        );

        app.add_event::<GameOverEvent>();

        app.add_plugin(BgPlugin);
        app.add_plugin(GridPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(BaseModePlugin);
        app.add_plugin(EventModePlugin);
        app.add_plugin(ExploreModePlugin);
    }
}
