#![allow(clippy::type_complexity)]
use bevy::prelude::*;

use crate::collision::math::{triangle::TriangleXY, Collider, Topology};

pub use crate::{game_state::GameState, spaceship::Spaceship};

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod camera;
pub mod collision;
pub mod compass;
pub mod fire;
pub mod game_over;
pub mod game_state;
pub mod health_bar;
pub mod keyboard;
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
pub struct Health(i32);

#[derive(Component, Clone, Copy)]
pub struct Mass(f32);

#[derive(Component, Clone, Copy)]
pub struct Velocity(Vec3);

#[derive(Component, Clone, Copy)]
pub struct AngularVelocity(f32); // radians per frame

// #[derive(Component)]
// struct SpawnedTime(Instant);

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

// pub fn spaceship_exists(query: Query<With<spaceship::Spaceship>>) -> bool {
//     !query.is_empty()
// }

// pub fn ingame_or_paused(game_state: Res<CurrentState<GameState>>) -> bool {
//     game_state.0 == GameState::InGame || game_state.0 == GameState::Paused
// }

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

pub fn count_entities(query: Query<Entity>) {
    println!("{}", query.iter().count());
}
