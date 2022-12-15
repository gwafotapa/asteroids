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
    // GameOver,
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

pub fn game_over(
    query: Query<With<spaceship::Spaceship>>,
    mut keyboard_activity: EventReader<KeyboardInput>,
    mut commands: Commands,
) {
    if query.get_single().is_err()
        && keyboard_activity
            .iter()
            .any(|key| key.state == ButtonState::Pressed)
    {
        commands.insert_resource(NextState(GameState::TurnDownLight))
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
