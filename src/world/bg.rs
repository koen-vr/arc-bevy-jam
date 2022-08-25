use super::*;

#[derive(Component)]
pub struct Star {
    lifetime: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct StarSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy)]
pub struct StarColor {
    start: Color,
    end: Color,
}

#[derive(Component)]
pub struct StarSpawner {
    time: Timer,
    size: StarSize,
    color: StarColor,
    rate: f32,
    lifetime: f32,
    per_burst: usize,
    canvas_size: Vec2,
}

pub struct BgPlugin;

impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(
        //     SystemSet::on_enter(AppState::MainMenu).with_system(spawn_stars_spawner),
        // );
        app.add_startup_system(spawn_stars_spawner);
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(emit_stars.label("emit")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(update_stars.after("emit")),
        );

        app.add_system_set(
            SystemSet::on_update(AppState::GameLoading).with_system(emit_stars.label("emit")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::GameLoading).with_system(update_stars.after("emit")),
        );

        app.add_system_set(
            SystemSet::on_update(AppState::GamePlay(GameMode::BaseGrid))
                .with_system(emit_stars.label("emit")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::GamePlay(GameMode::BaseGrid))
                .with_system(update_stars.after("emit")),
        );
        app.add_system_set(
            SystemSet::on_inactive_update(AppState::GamePlay(GameMode::BaseGrid))
                .with_system(emit_stars.label("emit")),
        );
        app.add_system_set(
            SystemSet::on_inactive_update(AppState::GamePlay(GameMode::BaseGrid))
                .with_system(update_stars.after("emit")),
        );
    }
}

pub fn emit_stars(
    time: Res<Time>,
    windows: Res<Windows>,
    camera_offset: Res<CameraOffset>,
    mut stars: Query<(&mut Star, &mut Visibility, &mut Transform)>,
    mut spawners: Query<(&Children, &mut StarSpawner)>,
) {
    for (children, mut spawner) in spawners.iter_mut() {
        spawner.time.tick(time.delta());
        if spawner.time.just_finished() {
            for _i in 0..spawner.per_burst {
                for child in children.iter() {
                    if let Ok((mut star, mut visibility, mut transform)) = stars.get_mut(*child) {
                        if !visibility.is_visible {
                            star.lifetime = Timer::from_seconds(spawner.lifetime, false);
                            visibility.is_visible = true;
                            transform.translation = Vec3::new(
                                (spawner.canvas_size.x * (2.0 * rand::random::<f32>() - 1.0))
                                    + camera_offset.value.x,
                                (spawner.canvas_size.y * (2.0 * rand::random::<f32>() - 1.0))
                                    + camera_offset.value.y,
                                0.0,
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_stars_spawner(mut commands: Commands) {
    let rate = 0.2;
    let spawner = StarSpawner {
        time: Timer::from_seconds(rate, true),
        per_burst: 4,
        // Note: Hard Coded: Window Size
        canvas_size: Vec2 {
            x: 1200.0,
            y: 600.0,
        },
        rate: rate,
        lifetime: 12.5,
        size: StarSize {
            start: 0.0,
            end: 2.5,
        },
        color: StarColor {
            start: Color::ORANGE,
            end: Color::WHITE,
        },
    };

    let mut stars = Vec::new();
    let max_stars = (1.1 * spawner.lifetime / spawner.rate) as usize * spawner.per_burst;
    for _i in 0..max_stars {
        stars.push(spawn_star(&mut commands, &spawner));
    }

    commands
        .spawn_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .insert(Name::new("star-spawner"))
        .insert(spawner)
        .push_children(&stars);
}

fn spawn_star(commands: &mut Commands, spawner: &StarSpawner) -> Entity {
    let mut star = SpriteBundle::default();
    star.visibility.is_visible = false;
    star.transform.translation = Vec3::new(
        spawner.canvas_size.x * (2. * rand::random::<f32>() - 1.),
        spawner.canvas_size.y * (2. * rand::random::<f32>() - 1.),
        0.1,
    );
    star.sprite.color = spawner.color.start;
    star.sprite.custom_size = Some(Vec2::splat(spawner.size.start));

    commands
        .spawn()
        .insert(Star {
            lifetime: Timer::from_seconds(spawner.lifetime, false),
        })
        .insert(spawner.color.clone())
        .insert(spawner.size.clone())
        .insert_bundle(star)
        .id()
}

pub fn update_stars(
    time: Res<Time>,
    mut stars: Query<(
        &StarSize,
        &StarColor,
        &mut Star,
        &mut Sprite,
        &mut Visibility,
    )>,
) {
    for (size, color, mut star, mut sprite, mut visibility) in stars.iter_mut() {
        star.lifetime.tick(time.delta());
        if star.lifetime.finished() {
            visibility.is_visible = false;
        }

        let t = star.lifetime.percent();
        let s = lerp_size(size.start, size.end, t);
        sprite.custom_size = Some(Vec2::splat(s));
        sprite.color = lerp_color(color.start, color.end, t);
    }
}

fn lerp(i: f32, j: f32, t: f32) -> f32 {
    return i * (1.0 - t) + j * t;
}

fn lerp_size(i: f32, j: f32, t: f32) -> f32 {
    if t > 0.5 {
        return j * (1.0 - t) + i * t;
    }
    return i * (1.0 - t) + j * t;
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerp(a.r(), b.r(), t),
        lerp(a.g(), b.g(), t),
        lerp(a.b(), b.b(), t),
        lerp(a.a(), b.a(), t),
    )
}
