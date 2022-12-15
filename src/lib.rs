#![allow(clippy::type_complexity)]
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use iyes_loopless::prelude::*;

use crate::collision::math::{triangle::TriangleXY, Collider, Topology};

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod camera;
pub mod collision;
pub mod compass;
pub mod fire;
pub mod light;
pub mod map;
pub mod spaceship;
pub mod ui;
pub mod wreckage;

const PLANE_Z: f32 = 500.0;
// pub const WINDOW_WIDTH: f32 = 1920.0;
// pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
// pub const WINDOW_WIDTH: f32 = 800.0;
// pub const WINDOW_HEIGHT: f32 = 600.0;
// const SHINE_FACTOR: f32 = 1.0 / DIM_FACTOR;

#[derive(Component, Clone, Copy)]
pub struct Velocity(Vec3);

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Health(i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    MainMenu,
    GameSetup,
    InGame,
    Paused,
    GameOver,
    TurnDownLight,
    TurnUpLight,
}

pub fn exit_game_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::TurnUpLight));
}

// // Warning: Should generate some double despawn (with debris::update for example)
// pub fn exit_game(
//     mut commands: Commands,
//     query_all: Query<Entity, Without<Camera>>,
//     // query_all: Query<Entity>,
//     // mut query_camera: Query<&mut UiCameraConfig>,
// ) {
//     for id in &query_all {
//         commands.entity(id).despawn();
//     }
//     // query_camera.single_mut().show_ui = true;
// }

#[derive(Clone, Component, Copy)]
pub struct GameOver;

pub fn game_over_spawn_text(
    query_spaceship: Query<&Health, With<spaceship::Spaceship>>,
    mut query_camera: Query<&mut UiCameraConfig>,
    mut query_pause_menu: Query<&mut Style, With<ui::pause_menu::PauseMenu>>,
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

            query_pause_menu.single_mut().display = Display::None;
            query_camera.single_mut().show_ui = true;
        }
    }
}

pub fn game_over_update_text(
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

pub fn spaceship_exists(query: Query<With<spaceship::Spaceship>>) -> bool {
    !query.is_empty()
}

pub fn ingame_or_paused(game_state: Res<CurrentState<GameState>>) -> bool {
    game_state.0 == GameState::InGame || game_state.0 == GameState::Paused
}

// pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health)>) {
//     for (id, health) in query.iter() {
//         if health.0 <= 0 {
//             commands.entity(id).despawn();
//         }
//     }
// }

pub fn despawn_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_recursive_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
