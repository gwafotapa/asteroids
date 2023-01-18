use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    collision::{Aabb, Collider, Topology},
    AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity, PLANE_Z, WINDOW_WIDTH,
};

#[derive(Clone, Component, Copy)]
pub struct Asteroid;

pub struct AsteroidEvent {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub vertices: usize,
    pub color: Color,
    pub health: Health,
    pub mass: Mass,
    pub moment_of_inertia: MomentOfInertia,
    pub velocity: Velocity,
    pub angular_velocity: AngularVelocity,
}

const COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const HEALTH_MAX: u32 = 60;
const VELOCITY_MIN: f32 = 100.0;
const VELOCITY_MAX: f32 = 500.0;

const VERTICES: usize = 16;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_camera: Query<&Transform, With<Camera>>,
) {
    let mut rng = rand::thread_rng();
    let Vec3 { x: xc, y: yc, z: _ } = query_camera.single().translation;
    if rng.gen_range(0..10) == 0 {
        let phi = rng.gen_range(0.0..2.0 * PI);
        let translation = Vec3::new(
            xc + 2.0 * WINDOW_WIDTH * phi.cos(),
            yc + 2.0 * WINDOW_WIDTH * phi.sin(),
            PLANE_Z,
        );
        let health = Health(rng.gen_range(10..HEALTH_MAX + 1));
        let radius = (health.0 * 2) as f32;
        let area = PI * radius.powi(2);
        let mass = Mass(area);
        let moment_of_inertia = MomentOfInertia(0.5 * mass.0 * radius.powi(2));
        let rho = rng.gen_range(VELOCITY_MIN..VELOCITY_MAX);
        let theta = rng.gen_range(0.0..2.0 * PI);
        let velocity = Velocity(Vec3::new(rho * theta.cos(), rho * theta.sin(), 0.0));
        let angular_velocity = AngularVelocity(0.0);

        let asteroid = commands
            .spawn(Asteroid)
            .insert(mass)
            .insert(moment_of_inertia)
            .insert(velocity)
            .insert(angular_velocity)
            .insert(SpatialBundle {
                transform: Transform::from_translation(translation),
                ..Default::default()
            })
            .id();

        let asteroid_part = commands
            .spawn((Asteroid, Part))
            .insert(health)
            .insert(Collider {
                aabb: Aabb {
                    hw: radius,
                    hh: radius,
                },
                topology: Topology::Disk { radius },
            })
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: VERTICES,
                    }))
                    .into(),
                material: materials.add(COLOR.into()),
                ..Default::default()
            })
            .id();

        commands.entity(asteroid).add_child(asteroid_part);
    }
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
//                 topology: Topology::Disk(radius),
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

pub fn update(
    mut commands: Commands,
    mut query_asteroid: Query<
        (&AngularVelocity, Entity, &mut Transform, &Velocity),
        With<Asteroid>,
    >,
    query_camera: Query<&Transform, (With<Camera>, Without<Asteroid>)>,
    time: Res<Time>,
) {
    for (a_angular_velocity, a_id, mut a_transform, a_velocity) in query_asteroid.iter_mut() {
        if (query_camera.single().translation - a_transform.translation)
            .truncate()
            .length()
            > 2.5 * WINDOW_WIDTH
        {
            commands.entity(a_id).despawn_recursive();
        } else {
            a_transform.translation += a_velocity.0 * time.delta_seconds();
            a_transform.rotation *=
                Quat::from_axis_angle(Vec3::Z, a_angular_velocity.0 * time.delta_seconds());
        }
    }
}

// pub fn before_despawn(
//     mut commands: Commands,
//     query_asteroid: Query<(Option<&Children>, &GlobalTransform, &Health), With<Asteroid>>,
//     mut query_impact: Query<&mut Transform, With<Impact>>,
// ) {
//     for (a_children, a_transform, a_health) in query_asteroid.iter() {
//         if a_health.0 > 0 {
//             continue;
//         }

//         if let Some(children) = a_children {
//             for child in children {
//                 commands.entity(*child).remove::<Parent>();
//                 query_impact
//                     .get_component_mut::<Transform>(*child)
//                     .unwrap()
//                     .translation += a_transform.translation();
//             }
//         }
//     }
// }

// pub fn wreck(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     query_asteroid: Query<(
//         &Asteroid,
//         &Handle<ColorMaterial>,
//         &GlobalTransform,
//         &Health,
//         // &Velocity,
//     )>,
// ) {
//     for (asteroid, color, transform, health) in query_asteroid.iter() {
//         if health.0 > 0 {
//             continue;
//         }

//         let mut rng = rand::thread_rng();
//         let color = materials.get(color).unwrap().color;
//         let area = PI * asteroid.radius * asteroid.radius;

//         for _ in 0..(area / 16.0).round() as usize {
//             let rho = rng.gen_range(0.0..asteroid.radius);
//             let theta = rng.gen_range(0.0..2.0 * PI);
//             let (sin, cos) = theta.sin_cos();
//             let (x, y) = (rho * cos, rho * sin);
//             let z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
//             let debris_translation = transform.translation() + Vec3::new(x, y, z);

//             let dv = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

//             commands
//                 .spawn(Debris)
//                 // .insert(Velocity(velocity.0 + dv))
//                 .insert(Velocity(dv))
//                 .insert(ColorMesh2dBundle {
//                     mesh: meshes
//                         .add(Mesh::from(shape::Circle {
//                             radius: rng.gen_range(1.0..asteroid.radius / 10.0),
//                             vertices: 8,
//                         }))
//                         .into(),
//                     transform: Transform::from_translation(debris_translation),
//                     material: materials.add(color.into()),
//                     ..default()
//                 });
//         }
//     }
// }

// pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Asteroid>>) {
//     for (entity, health) in query.iter() {
//         if health.0 <= 0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }
