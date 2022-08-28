use crate::*;

use bevy::ui::FocusPolicy;

pub mod gamehud;
pub mod mainmenu;

pub const TEXT_BUTTON: Color = Color::rgb(0.95, 0.95, 0.95);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gamehud::GamehudPlugin);
        app.add_plugin(mainmenu::MainMenuPlugin);
    }
}

pub(crate) fn create_button<ButtomType: Component>(
    commands: &mut Commands,
    fg: Color,
    bg: Color,
    width: f32,
    visible: bool,
    text: String,
    font: Handle<Font>,
    btn_type: ButtomType,
) -> Entity {
    commands
        .spawn_bundle(ButtonBundle {
            focus_policy: FocusPolicy::Block,
            visibility: Visibility {
                is_visible: visible,
            },
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(65.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: bg.into(),
            ..default()
        })
        .insert(btn_type)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                text,
                TextStyle {
                    font: font,
                    font_size: 40.0,
                    color: fg,
                },
            ));
        })
        .id()
}

pub(crate) fn create_image_button<ButtomType: Component>(
    commands: &mut Commands,
    fg: Color,
    bg: Color,
    width: f32,
    text: String,
    font: Handle<Font>,
    btn_type: ButtomType,
) -> Entity {
    commands
        .spawn_bundle(ButtonBundle {
            focus_policy: FocusPolicy::Block,
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(65.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: bg.into(),
            ..default()
        })
        .insert(btn_type)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                text,
                TextStyle {
                    font: font,
                    font_size: 40.0,
                    color: fg,
                },
            ));
        })
        .id()
}
