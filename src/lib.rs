#![allow(clippy::type_complexity)]
use bevy::{prelude::*, sprite::Mesh2dHandle};

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
    mut query_color: Query<(&mut Handle<ColorMaterial>, &ComputedVisibility)>,
) {
    if query_spaceship.get_single().is_err() {
        for (color, visibility) in &mut query_color {
            if visibility.is_visible() {
                let color = materials.get_mut(&color).unwrap();
                let [mut r, mut g, mut b, _] = color.color.as_rgba_f32();
                r *= 0.95;
                g *= 0.95;
                b *= 0.95;
                color.color = Color::rgb(r, g, b);
            }
        }
    }
}
