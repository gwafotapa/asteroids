use bevy::{app::AppExit, prelude::*, text::Text2dBounds};
use iyes_loopless::prelude::*;

use crate::{
    boss::{Boss, Indestructible},
    game_state::GameState,
    spaceship::Spaceship,
    Health,
};

#[derive(Clone, Component, Copy)]
pub struct GameOver;

const FONT_SIZE: f32 = 24.0;
const BOX_WIDTH: f32 = 530.0;
const BOX_HEIGHT: f32 = FONT_SIZE;
const BOX_CENTER_LEFT: Vec3 = Vec3 {
    x: -BOX_WIDTH / 2.0,
    y: 100.0 - BOX_HEIGHT / 2.0,
    z: 0.0,
};

pub fn spawn_text(
    query_spaceship: Query<(Entity, &Health), With<Spaceship>>,
    query_boss: Query<&Health, With<Boss>>,
    query_camera: Query<&Transform, With<Camera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const FONT: &str = "fonts/FiraSans-Bold.ttf";
    const COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
    let text_style = TextStyle {
        font: asset_server.load(FONT),
        font_size: FONT_SIZE,
        color: COLOR,
    };
    let text_alignment = TextAlignment::CENTER_LEFT;

    if let Ok((spaceship, health)) = query_spaceship.get_single() {
        if health.0 == 0 {
            commands.spawn(GameOver).insert(Text2dBundle {
                text: Text::from_section(
                    "Mission failed. Press Enter to go back to the main menu",
                    text_style,
                )
                .with_alignment(text_alignment),
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(BOX_WIDTH, BOX_HEIGHT),
                },
                transform: Transform::from_translation(
                    query_camera.single().translation + BOX_CENTER_LEFT,
                ),
                ..Default::default()
            });
        } else if let Ok(health) = query_boss.get_single() {
            if health.0 == 0 {
                commands.spawn(GameOver).insert(Text2dBundle {
                    text: Text::from_section(
                        "Mission cleared. Press Enter to go back to the main menu",
                        text_style,
                    )
                    .with_alignment(text_alignment),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(BOX_WIDTH, BOX_HEIGHT),
                    },
                    transform: Transform::from_translation(
                        query_camera.single().translation + BOX_CENTER_LEFT,
                    ),
                    ..Default::default()
                });

                commands.entity(spaceship).insert(Indestructible);
            }
        }
    }
}

pub fn update_text(
    mut query: Query<(&mut Text, &mut Transform), With<GameOver>>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    input: Res<Input<KeyCode>>,
    query_camera: Query<&Transform, (With<Camera>, Without<GameOver>)>,
) {
    if let Ok((mut text, mut transform)) = query.get_single_mut() {
        transform.translation = query_camera.single().translation + BOX_CENTER_LEFT;
        if text.sections[0].style.color.r() < 0.5 {
            const INC: f32 = 0.002;
            text.sections[0].style.color += Color::rgb(INC, INC, INC);
        }
        if input.just_pressed(KeyCode::Return) {
            commands.insert_resource(NextState(GameState::TurnDownLight));
        } else if input.just_pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }
    }
}
