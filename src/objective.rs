use bevy::{prelude::*, text::Text2dBounds};

use crate::keyboard_bindings::KeyboardBindings;

const FONT_SIZE: f32 = 24.0;
const BOX_WIDTH: f32 = 360.0;
const BOX_HEIGHT: f32 = FONT_SIZE;
const BOX_CENTER_LEFT: Vec3 = Vec3 {
    x: -BOX_WIDTH / 2.0,
    y: 100.0 - BOX_HEIGHT / 2.0,
    z: 0.0,
};

#[derive(Clone, Component, Copy)]
pub struct Objective;

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_camera: Query<&Transform, With<Camera>>,
) {
    const FONT: &str = "fonts/FiraSans-Bold.ttf";
    const COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
    let text_style = TextStyle {
        font: asset_server.load(FONT),
        font_size: FONT_SIZE,
        color: COLOR,
    };
    let text_alignment = TextAlignment::CENTER_LEFT;

    commands.spawn(Objective).insert(Text2dBundle {
        text: Text::from_section("Mission objective: Eliminate the target", text_style)
            .with_alignment(text_alignment),
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(BOX_WIDTH, BOX_HEIGHT),
        },
        transform: Transform::from_translation(query_camera.single().translation + BOX_CENTER_LEFT),
        ..Default::default()
    });
}

pub fn update_text(
    mut query: Query<(Entity, &mut Text, &mut Transform), With<Objective>>,
    mut commands: Commands,
    mut timer: Local<u32>,
    input: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
    query_camera: Query<&Transform, (With<Camera>, Without<Objective>)>,
) {
    if let Ok((id, mut text, mut transform)) = query.get_single_mut() {
        if input.any_just_pressed([KeyCode::Escape, query_bindings.single().pause()]) {
            commands.entity(id).despawn();
            *timer = 0;
        } else {
            transform.translation = query_camera.single().translation + BOX_CENTER_LEFT;
            const INC: f32 = 0.004;
            if *timer < 125 {
                text.sections[0].style.color += Color::rgb(INC, INC, INC);
                *timer += 1;
            } else if *timer < 250 {
                text.sections[0].style.color += Color::rgb(-INC, -INC, -INC);
                *timer += 1;
            } else {
                commands.entity(id).despawn();
                *timer = 0;
            }
        }
    }
}
