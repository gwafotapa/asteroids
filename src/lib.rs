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
// pub mod debris;
pub mod fire;
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
const DIM_FACTOR: f32 = 0.92;
const DIM_TIMER: u32 = 50;

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

pub fn turn_down_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_main_menu: Query<Entity, With<ui::main_menu::MainMenu>>,
    mut query_camera: Query<(&mut Camera, &mut UiCameraConfig)>,
    query_without_camera: Query<Entity, Without<Camera>>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    let (mut camera, mut config) = query_camera.single_mut();
    *timer += 1;
    if *timer == DIM_TIMER {
        if let Ok(main_menu) = query_main_menu.get_single() {
            commands.entity(main_menu).despawn_recursive();
            commands.insert_resource(NextState(GameState::GameSetup));
            camera.is_active = false;
            config.show_ui = false;
        } else {
            for id in &query_without_camera {
                commands.entity(id).despawn();
            }
            commands.insert_resource(NextState(GameState::MainMenu));
            config.show_ui = true;
        }
        *timer = 0;
    }
}

pub fn turn_up_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    *timer += 1;
    if *timer == DIM_TIMER {
        commands.insert_resource(NextState(GameState::InGame));
        // query_camera.single_mut().show_ui = true;
        *timer = 0;
    }
}

pub fn kill_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let kill_factor = DIM_FACTOR.powi(DIM_TIMER as i32);

    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [kill_factor, kill_factor, kill_factor];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [kill_factor, kill_factor, kill_factor];
        }
    }
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
