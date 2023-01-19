use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    collision::{Aabb, Collider, Topology},
    AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity, PLANE_Z, WINDOW_WIDTH,
};

#[derive(Clone, Component, Copy)]
pub struct Asteroid;

pub fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        const HEALTH_MAX: u32 = 60;
        let health = Health(rng.gen_range(10..HEALTH_MAX + 1));
        let radius = (health.0 * 2) as f32;
        let area = PI * radius.powi(2);
        let mass = Mass(area);
        let moment_of_inertia = MomentOfInertia(0.5 * mass.0 * radius.powi(2));
        const VELOCITY_MIN: f32 = 100.0;
        const VELOCITY_MAX: f32 = 500.0;
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

        const VERTICES: usize = 16;
        const COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

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

pub fn movement(
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
