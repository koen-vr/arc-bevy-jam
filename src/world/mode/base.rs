use super::*;

use bevy::ui::FocusPolicy;
pub struct BaseModePlugin;

#[derive(Component)]
struct CleanupBaseMode;

impl Plugin for BaseModePlugin {
    fn build(&self, app: &mut App) {
        //let base_grid = AppState::GamePlay(GameMode::BaseGrid);

        // app.add_system_set(SystemSet::on_exit(base_grid).with_system(exit_base_mode));
        // app.add_system_set(SystemSet::on_enter(base_grid).with_system(enter_base_mode));
    }
}

// fn exit_base_mode(mut commands: Commands, query: Query<Entity, With<CleanupBaseMode>>) {
//     log::info!("exit_base_mode");
//     for e in query.iter() {
//         commands.entity(e).despawn_recursive();
//     }
// }

pub(crate) fn base_mode_select(
    commands: &mut Commands,
    app_assets: &Res<AppAssets>,
    world_assets: &Res<WorldAssets>,
) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(40.0)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("main-menu"))
        .insert(CleanupBaseMode)
        .id();

    let mut list = Vec::new();

    // Note: We pulled the hips out of the spritesheet for this
    // Using spritesheets in UI seems to be a bit to complicated.

    // Spawn Ship Selection
    list.push(spawn_ship_stats(
        commands,
        app_assets.gui_font.clone(),
        world_assets.ship_7.clone(),
    ));
    list.push(spawn_ship_stats(
        commands,
        app_assets.gui_font.clone(),
        world_assets.ship_17.clone(),
    ));

    commands.entity(root).push_children(&list);
    root
}

fn spawn_ship_stats(commands: &mut Commands, font: Handle<Font>, ship: Handle<Image>) -> Entity {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Px(TILE_SIZE * 3.), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    focus_policy: FocusPolicy::Block,
                    style: Style {
                        size: Size::new(Val::Px(TILE_SIZE * 3.), Val::Px(64.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(24.), Val::Px(24.)),
                        ..default()
                    },
                    color: gui::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Select",
                        TextStyle {
                            font: font,
                            font_size: 40.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(TILE_SIZE * 2.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::from(ship),
                //color: Color::rgba(0., 0., 0., 0.5).into(),
                //visible: Visible{is_visible:false, is_transparent:true},
                ..Default::default()
            });
        })
        .id()
}
