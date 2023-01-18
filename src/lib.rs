#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use bevy::prelude::*;

use crate::collision::detection::{triangle::TriangleXY, Collider, Topology};

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
pub mod intercepter;
pub mod keyboard;
pub mod light;
pub mod map;
pub mod spaceship;
pub mod transform;
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

#[derive(Clone, Component, Copy)]
pub struct Part;

#[derive(Clone, Component, Copy)]
pub struct Health(pub u32);

#[derive(Clone, Component, Copy)]
pub struct Mass(pub f32);

#[derive(Clone, Component, Copy)]
pub struct MomentOfInertia(pub f32);

#[derive(Clone, Component, Copy, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Clone, Component, Copy, Debug)]
pub struct AngularVelocity(pub f32); // radians per frame

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
        if health.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_recursive_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn count_entities(query: Query<Entity>) {
    println!("entities: {}", query.iter().count());
}

pub fn count_asteroids(query: Query<&asteroid::Asteroid, Without<Part>>) {
    println!("asteroids: {}", query.iter().count());
}

pub fn count_stars(query: Query<&map::star::Star>) {
    println!("stars: {}", query.iter().count());
}

pub fn count_intercepters(query: Query<&intercepter::Intercepter>) {
    println!("intercepters: {}", query.iter().count());
}

pub fn count_wreckages(query: Query<&wreckage::Wreckage>) {
    println!("wreckages: {}", query.iter().count());
}

pub fn count_debris(query: Query<&wreckage::WreckageDebris>) {
    println!("debris: {}", query.iter().count());
}
