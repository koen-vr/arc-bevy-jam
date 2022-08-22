use crate::*;

pub mod gamehud;
pub mod mainmenu;

const TEXT_BUTTON: Color = Color::rgb(0.95, 0.95, 0.95);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);

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
    text: String,
    font: Handle<Font>,
    btn_type: ButtomType,
) -> Entity {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
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
