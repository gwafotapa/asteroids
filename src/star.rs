use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{Velocity, WINDOW_HEIGHT, WINDOW_WIDTH};

const INITIAL_COUNT_BY_VELOCITY: usize = 10;
const SPEED_MAX: usize = 10;
const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;

#[derive(Component)]
pub struct Star;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for speed in 1..(SPEED_MAX + 1) {
        for _i in 0..INITIAL_COUNT_BY_VELOCITY {
            commands
                .spawn_empty()
                .insert(Star)
                .insert(Velocity(Vec3::from([-(speed as f32), 0., 0.])))
                .insert(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: RADIUS,
                            vertices: VERTICES,
                        }))
                        .into(),
                    transform: Transform::from_translation(Vec3 {
                        x: rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                        y: rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                        z: BACKGROUND,
                    }),
                    material: materials.add(Color::WHITE.into()),
                    ..default()
                });
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(1..SPEED_MAX + 1) as f32;
    let velocity = Vec3::from([-speed, 0., 0.]);

    commands
        .spawn_empty()
        .insert(Star)
        .insert(Velocity(velocity))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: RADIUS,
                    vertices: VERTICES,
                }))
                .into(),
            transform: Transform::from_translation(Vec3 {
                x: WINDOW_WIDTH / 2.0,
                y: rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                z: BACKGROUND,
            }),
            material: materials.add(Color::WHITE.into()),
            ..default()
        });
}

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Star>>,
) {
    for (star, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            commands.entity(star).despawn();
        }
    }
}
