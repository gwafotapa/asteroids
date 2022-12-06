use bevy::prelude::*;
use rand::Rng;

use crate::{
    collision::{Aabb, Collider, Topology},
    debris::Debris,
    Health, Velocity, PLANE_Z, WINDOW_HEIGHT, WINDOW_WIDTH,
};

// const SPEED_MAX: usize = 5;
const HEALTH_MAX: i32 = 6;
const COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const ASTEROID_Z: f32 = PLANE_Z;

#[derive(Component)]
pub struct Asteroid {
    pub radius: f32,
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let mut rng = rand::thread_rng();
    let health = rng.gen_range(1..HEALTH_MAX + 1);
    let radius = (health * 20) as f32;
    // let speed = rng.gen_range(1..SPEED_MAX + 1) as f32;
    // let velocity = Vec3::from([-speed, 0., 0.]);
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
    let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

    let asteroid = commands
        .spawn(Asteroid { radius })
        .insert(Health(health))
        // .insert(Velocity(velocity))
        // .insert(Topology::Circle)
        .insert(Collider {
            aabb: Aabb {
                hw: radius,
                hh: radius,
            },
            topology: Topology::Circle { radius },
        })
        .insert(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius,
                    vertices: 16,
                }))
                .into(),
            transform: Transform::from_xyz(x, y, ASTEROID_Z),
            material: materials.add(ColorMaterial::from(COLOR)),
            ..default()
        })
        .id();

    asteroid
}

// pub fn asteroids(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query_asteroid: Query<(&Asteroid, Entity, &mut Transform, &Velocity)>,
//     query_level: Query<&Level>,
// ) {
//     let mut rng = rand::thread_rng();

//     if query_level.single().distance_to_boss > 0 && rng.gen_range(0..100) == 0 {
//         let health = rng.gen_range(1..HEALTH_MAX + 1);
//         let radius = (health * 20) as f32;
//         let speed = rng.gen_range(1..SPEED_MAX + 1) as f32;
//         let velocity = Vec3::from([-speed, 0., 0.]);
//         let x = WINDOW_WIDTH / 2.0 + (HEALTH_MAX * 20) as f32;
//         let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

//         commands
//             .spawn_empty()
//             .insert(Asteroid { radius })
//             .insert(Health(health))
//             .insert(Velocity(velocity))
//             .insert(Surface {
//                 topology: Topology::Circle(radius),
//                 aabb: Aabb {
//                     hw: radius,
//                     hh: radius,
//                 },
//             })
//             .insert(ColorMesh2dBundle {
//                 mesh: meshes
//                     .add(Mesh::from(shape::Circle {
//                         radius,
//                         vertices: 16,
//                     }))
//                     .into(),
//                 transform: Transform::from_xyz(x, y, ASTEROID_Z),
//                 material: materials.add(ColorMaterial::from(COLOR)),
//                 ..default()
//             });
//     }

//     for (asteroid, entity, mut transform, velocity) in query_asteroid.iter_mut() {
//         transform.translation += velocity.0;
//         if transform.translation.x < -WINDOW_WIDTH / 2.0 - asteroid.radius {
//             commands.entity(entity).despawn();
//         }
//     }
// }

pub fn explode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_asteroid: Query<(
        &Asteroid,
        // Option<&Children>,
        &Handle<ColorMaterial>,
        &GlobalTransform,
        &Health,
        // &Velocity,
    )>,
    // mut query_impact: Query<&mut Transform, With<Impact>>,
) {
    // for (asteroid, children, color, transform, health, velocity) in query_asteroid.iter() {
    for (asteroid, color, transform, health) in query_asteroid.iter() {
        if health.0 > 0 {
            continue;
        }

        // if let Some(children) = children {
        //     for child in children {
        //         commands.entity(*child).remove::<Parent>();
        //         query_impact
        //             .get_component_mut::<Transform>(*child)
        //             .unwrap()
        //             .translation += transform.translation();
        //     }
        // }

        let color = materials.get(color).unwrap().color;
        let mut rng = rand::thread_rng();

        for _ in 1..asteroid.radius as usize {
            let x = rng.gen_range(-asteroid.radius..asteroid.radius);
            let y_max = (asteroid.radius.powi(2) - x.powi(2)).sqrt();
            let y = rng.gen_range(-y_max..y_max);
            let z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            let debris_translation = transform.translation() + Vec3::new(x, y, z);

            let dv = Vec3 {
                x: rng.gen_range(-0.5..0.5),
                y: rng.gen_range(-0.5..0.5),
                z: 0.0,
            };

            commands
                .spawn(Debris)
                // .insert(Velocity(velocity.0 + dv))
                .insert(Velocity(dv))
                .insert(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: rng.gen_range(asteroid.radius / 100.0..asteroid.radius / 20.0),
                            vertices: 8,
                        }))
                        .into(),
                    transform: Transform::from_translation(debris_translation),
                    material: materials.add(color.into()),
                    ..default()
                });
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Asteroid>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
