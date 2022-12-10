#![allow(clippy::type_complexity)]
use bevy::prelude::*;

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod camera;
pub mod collision;
pub mod compass;
pub mod debris;
pub mod fire;
pub mod game_state;
pub mod map;
pub mod spaceship;

pub use game_state::GameState;

const PLANE_Z: f32 = 500.0;
// pub const WINDOW_WIDTH: f32 = 1920.0;
// pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
// pub const WINDOW_WIDTH: f32 = 800.0;
// pub const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Component)]
pub struct Velocity(Vec3);

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Health(i32);

pub fn dim_light(
    query_spaceship: Query<With<spaceship::Spaceship>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_visible_entities: Query<(&mut Handle<ColorMaterial>, &ComputedVisibility)>,
) {
    if query_spaceship.get_single().is_err() {
        for (color_material, visibility) in &mut query_visible_entities {
            if visibility.is_visible() {
                materials.get_mut(&color_material).unwrap().color *= [0.95, 0.95, 0.95];
            }
        }
    }
}
