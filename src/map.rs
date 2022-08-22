use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::Inspectable;

use crate::*;

#[derive(Default, Component, Inspectable)]
pub struct HexActive {}

pub struct HexmapPlugin;

impl Plugin for HexmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup);
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_nodes);
        app.add_system(active_node);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_handle = asset_server.load("hex-pointy-64.2.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_handle.clone(),
            ..default()
        })
        .insert(HexActive {});
}

fn spawn_nodes(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize a hex storage component for data
    // let mut node_storage = HexStorage::default();

    // Create a node used to spawn hex grids
    let mut node = HexNode::new(
        Vec2 { x: 0., y: 0. },
        Vec2 { x: 34., y: 34. },
        orient::Style::Pointy,
    );

    // Setup the node entity and spawn the grid
    let name = format!("node-{}:{}", 0, 0);
    let node_id = commands.spawn().insert(Name::new(name)).id();
    let list = node.spawn_entities(node_id, &mut commands, &asset_server);

    // Finalize hex node and entities as children
    commands
        .entity(node_id)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .insert(node)
        .push_children(&list);
}

fn active_node(
    windows: Res<Windows>,
    mut hex_query: Query<(&HexActive, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let (_, mut hex_transform) = hex_query.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let node = HexNode::new(
            Vec2 { x: 0., y: 0. },
            Vec2 { x: 34., y: 34. },
            orient::Style::Pointy,
        );

        // Convert window position to gpu coordinates
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let hex = node.layout.hex_for(Vec2 {
            x: world_pos.x,
            y: world_pos.y,
        });
        let pos = node.layout.center_for(&hex);

        hex_transform.translation.x = pos.x;
        hex_transform.translation.y = pos.y;
    }
}
