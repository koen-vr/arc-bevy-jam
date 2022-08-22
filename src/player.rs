use crate::*;

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    render::{camera::RenderTarget, render_resource::encase::rts_array::Length},
};
use bevy_inspector_egui::Inspectable;

#[derive(Default, Component, Inspectable)]
pub struct Player {
    moved: bool,
    active: bool,
    lookat: Vec3,
    move_speed: f32,
    rotate_speed: f32,
}

#[derive(Default, Component, Inspectable)]
pub struct PlayerMoveTo {
    target: Vec2,
}

impl Player {
    pub fn is_active(&self) -> bool {
        self.active
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system_set(
            SystemSet::on_update(GameState::UniverseMap)
                .with_system(mouse_movement_system.label("universe_move"))
                .with_system(key_movement_system.after("universe_move"))
                .with_system(rotate_system.after("universe_move"))
                .with_system(camera_system.after("universe_move")),
        );
    }
}

fn key_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut PlayerMoveTo, &mut Transform)>,
) {
    let (mut player, mut move_to, mut transform) = player_query.single_mut();
    player.moved = false;
    if !player.active {
        return;
    }

    let move_speed = player.move_speed * time.delta_seconds() * TILE_SIZE;

    let mut target_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        //player.current_direction = FacingDirection::Up;
        target_y = move_speed;
        player.moved = true;
    }
    if keyboard.pressed(KeyCode::S) {
        //player.current_direction = FacingDirection::Down;
        target_y = -move_speed;
        player.moved = true;
    }

    let mut target_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        //player.current_direction = FacingDirection::Left;
        target_x = -move_speed;
        player.moved = true;
    }
    if keyboard.pressed(KeyCode::D) {
        //player.current_direction = FacingDirection::Right;
        target_x = move_speed;
        player.moved = true;
    }

    let pos = Vec2 {
        x: transform.translation.x,
        y: transform.translation.y,
    };
    if player.moved {
        transform.translation = transform.translation + Vec3::new(target_x, target_y, 0.0);

        move_to.target = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        };
    } else if move_to.target.distance(pos) > 0.25 {
        let distance = move_to.target - pos;
        let direction = (distance / distance.length()) * move_speed;
        let target = transform.translation + Vec3::new(direction.x, direction.y, 0.0);
        if move_to.target.distance(pos) > move_speed {
            transform.translation = target.clone();
        } else {
            transform.translation =
                Vec3::new(move_to.target.x, move_to.target.y, transform.translation.z);
        }
    }
}

fn mouse_movement_system(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_query: Query<(&mut Player, &mut PlayerMoveTo)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let (mut player, mut move_to) = player_query.single_mut();
    if !player.active {
        return;
    }

    // Get the primary window the camera renders to.
    let (camera, camera_transform) = camera_query.single();
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

        move_to.target = node.layout.center_for(&hex);
    }
}

fn rotate_system(
    time: Res<Time>,
    windows: Res<Windows>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let (player, mut player_transform) = player_query.single_mut();
    if !player.active {
        return;
    }

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        // Convert window position to gpu coordinates
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let delta_x = world_pos.x - player_transform.translation.x;
        let delta_y = world_pos.y - player_transform.translation.y;
        let delta = delta_x.atan2(delta_y);

        // Rotate the sprite to look at the mouse position
        let q = Quat::from_axis_angle(-Vec3::Z, delta);
        player_transform.rotation = player_transform
            .rotation
            .slerp(q, player.rotate_speed * time.delta_seconds());
    }
}

fn camera_system(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut cam_transform = camera_query.single_mut();
    let (_, player_transform) = player_query.single();

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}

pub fn spawn_player(mut commands: Commands, space_sheet: Res<SpaceSheet>) {
    let mut sprite = TextureAtlasSprite::new(7);
    sprite.color = Color::rgb(0.9, 0.8, 1.0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: space_sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            active: true,
            move_speed: MOVE_SPEED,
            rotate_speed: ROTATE_SPEED,
            ..Default::default()
        })
        .insert(PlayerMoveTo {
            ..Default::default()
        });
}
