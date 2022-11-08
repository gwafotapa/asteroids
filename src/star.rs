use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{Velocity, ALTITUDE, WINDOW_HEIGHT, WINDOW_WIDTH};

const INITIAL_COUNT_OF_STARS_BY_VELOCITY: usize = 10;
const MAX_SPEED_OF_STARS: usize = 10;

#[derive(Component)]
pub struct Star;

pub fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for speed in 1..(MAX_SPEED_OF_STARS + 1) {
        let z = ALTITUDE - (MAX_SPEED_OF_STARS / 2 + speed) as f32 + 0.5;
        for _i in 0..INITIAL_COUNT_OF_STARS_BY_VELOCITY {
            let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
            let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

            commands
                .spawn()
                .insert(Star)
                .insert(Velocity(Vec3::from([-(speed as f32), 0., 0.])))
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: 1.0,
                            vertices: 4,
                        }))
                        .into(),
                    transform: Transform::from_translation(Vec3 { x, y, z }),
                    material: materials.add(Color::rgb(1., 1., 1.).into()),
                    ..default()
                });
        }
    }
}

pub fn add_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(1..MAX_SPEED_OF_STARS + 1) as f32;
    let velocity = Vec3::from([-speed, 0., 0.]);

    let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);
    let z = ALTITUDE - (MAX_SPEED_OF_STARS / 2) as f32 + speed + 0.5;

    commands
        .spawn()
        .insert(Star)
        .insert(Velocity(velocity))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: 1.0,
                    vertices: 4,
                }))
                .into(),
            transform: Transform::from_translation(Vec3 {
                x: WINDOW_WIDTH / 2.0,
                y,
                z,
            }),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        });
}

pub fn update_stars(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Star>>,
) {
    for (mut transform, velocity, star) in query.iter_mut() {
        transform.translation += velocity.0;
        //     for value in mesh.attributes() {
        //         println!("{}", value);
        //     }
        if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            commands.entity(star).despawn();
        }
    }
    // for mesh in meshes.get_handle() {}
}