use super::*;

use bevy::ui::FocusPolicy;

const FAILED_TO_SET_STATE: &str = "Failed to set game state";

#[derive(Clone, Copy, Default)]
pub enum ShipKey {
    #[default]
    Ship1,
    Ship2,
    // Ship3,
    // Ship4,
    // Ship5,
    // Ship6,
    // Ship7,
    // Ship8,
    // Ship9,
    // Ship10,
    // Ship11,
    // Ship12,
    // Ship13,
    // Ship14,
    // Ship15,
    // Ship16,
}

#[derive(Component, Clone, Copy)]
pub struct ShipInfo {
    pub key: ShipKey,
    pub jump: u8,
    pub speed: f32,
    pub health: u16,
    pub energy: u16,
}

pub struct BaseModePlugin;

#[derive(Component)]
struct CleanupBaseMode;

impl Plugin for BaseModePlugin {
    fn build(&self, app: &mut App) {
        let base_grid = AppState::GamePlay(GameMode::BaseGrid);

        app.insert_resource(ShipInfo {
            key: ShipKey::Ship1,
            jump: 3,
            speed: 8.,
            health: 8,
            energy: 12,
        });

        // TODO Generate the map data while in this state, remove transition delay

        app.add_system_set(SystemSet::on_update(base_grid).with_system(button_update));
    }
}

fn button_update(
    mut info: ResMut<ShipInfo>,
    mut state: ResMut<State<AppState>>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut button_query: Query<
        (&Interaction, &ShipInfo, &mut UiColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, ship, mut color, mut transform) in &mut button_query {
        match *interaction {
            Interaction::Clicked => {
                buttons.clear();
                transform.scale *= 1.05;
                *color = gui::PRESSED_BUTTON.into();
                handle_btn_update_click(ship, &mut info, &mut state);
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
    bnt: &ShipInfo,
    info: &mut ResMut<ShipInfo>,
    state: &mut ResMut<State<AppState>>,
) {
    log::info!("SelectKey::clicked: Ship Selection");

    info.key = bnt.key;
    info.jump = bnt.jump;
    info.speed = bnt.speed;
    info.health = bnt.health;
    info.energy = bnt.energy;

    state
        .push(AppState::GamePlay(GameMode::ExploreGrid))
        .unwrap_or_else(|error| log::error!("{}: {}", FAILED_TO_SET_STATE, error));
}

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

    // Note: We pulled the ships out of the spritesheet for this
    // Using spritesheets in UI seems to be a bit to complicated.

    // Spawn Ship Selection
    list.push(spawn_ship_stats(
        commands,
        ShipInfo {
            key: ShipKey::Ship1,
            jump: 3,
            speed: 6.,
            health: 8,
            energy: 10,
        },
        app_assets.gui_font.clone(),
        world_assets.ship_7.clone(),
    ));
    list.push(spawn_ship_stats(
        commands,
        ShipInfo {
            key: ShipKey::Ship2,
            jump: 5,
            speed: 8.,
            health: 6,
            energy: 12,
        },
        app_assets.gui_font.clone(),
        world_assets.ship_17.clone(),
    ));

    commands.entity(root).push_children(&list);
    root
}

fn spawn_ship_stats(
    commands: &mut Commands,
    info: ShipInfo,
    font: Handle<Font>,
    ship: Handle<Image>,
) -> Entity {
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
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(36.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .insert(info)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "energy:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent.spawn_bundle(TextBundle::from_section(
                        info.energy.to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(36.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .insert(info)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "speed:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent.spawn_bundle(TextBundle::from_section(
                        (info.speed as u8).to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(36.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .insert(info)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "jump:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent.spawn_bundle(TextBundle::from_section(
                        info.jump.to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(36.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .insert(info)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "hp:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent.spawn_bundle(TextBundle::from_section(
                        info.health.to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                });
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
                .insert(info)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "select",
                        TextStyle {
                            font: font.clone(),
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
                ..Default::default()
            });
        })
        .id()
}

impl ShipInfo {
    pub fn get_index(&self) -> usize {
        match self.key {
            ShipKey::Ship1 => 7,
            ShipKey::Ship2 => 17,
            // ShipKey::Ship3 => 0,
            // ShipKey::Ship4 => 0,
            // ShipKey::Ship5 => 0,
            // ShipKey::Ship6 => 0,
            // ShipKey::Ship7 => 0,
            // ShipKey::Ship8 => 0,
            // ShipKey::Ship9 => 0,
            // ShipKey::Ship10 => 0,
            // ShipKey::Ship11 => 0,
            // ShipKey::Ship12 => 0,
            // ShipKey::Ship13 => 0,
            // ShipKey::Ship14 => 0,
            // ShipKey::Ship15 => 0,
            // ShipKey::Ship16 => 0,
        }
    }
}
