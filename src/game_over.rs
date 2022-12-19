use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

#[derive(Clone, Component, Copy)]
pub struct GameOver;

pub fn spawn_text(
    query_spaceship: Query<&super::Health, With<super::spaceship::Spaceship>>,
    mut query_camera: Query<&mut UiCameraConfig>,
    // mut query_pause_menu: Query<&mut Style, With<super::ui::pause_menu::PauseMenu>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(health) = query_spaceship.get_single() {
        if health.0 == 0 {
            commands.spawn(GameOver).insert(TextBundle {
                text: Text::from_section(
                    "Press space to go back to the main menu",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                ),
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            });

            // query_pause_menu.single_mut().display = Display::None;
            query_camera.single_mut().show_ui = true;
        }
    }
}

pub fn update_text(
    mut query: Query<&mut Text, With<GameOver>>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if text.sections[0].style.color.r() < 0.5 {
            text.sections[0].style.color += Color::rgb(0.002, 0.002, 0.002);
        }
        if input.just_pressed(KeyCode::Space) {
            commands.insert_resource(NextState(GameState::TurnDownLight));
        }
    }
}
