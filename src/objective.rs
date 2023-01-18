use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use crate::{
    boss::Boss, game_state::GameState, keyboard::KeyboardBindings, spaceship::Spaceship, Health,
};

#[derive(Clone, Component, Copy)]
pub struct Objective;

pub fn spawn_text(
    mut commands: Commands,
    mut query_camera: Query<&mut UiCameraConfig>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Objective).insert(TextBundle {
        text: Text::from_section(
            "Mission objective: Eliminate the target",
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

    query_camera.single_mut().show_ui = true;
}

pub fn update_text(
    mut query: Query<(Entity, &mut Text), With<Objective>>,
    mut commands: Commands,
    mut timer: Local<u32>,
    input: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
) {
    if let Ok((id, mut text)) = query.get_single_mut() {
        if input.any_just_pressed([KeyCode::Escape, query_bindings.single().pause()]) {
            commands.entity(id).despawn();
            *timer = 0;
        } else {
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
