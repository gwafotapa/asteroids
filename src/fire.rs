use bevy::prelude::*;

use crate::{
    collision::{damages::Damageable, impact::Impact, Aabb},
    AngularVelocity, Collider, Health, Mass, MomentOfInertia, Topology, Velocity,
};

#[derive(Component)]
pub struct Fire {
    pub impact_radius: f32,
    pub impact_vertices: usize,
}

#[derive(Component)]
pub struct Enemy;

impl Damageable for Fire {}

pub struct FireEvent {
    fire: Fire,
    radius: f32,
    vertices: usize,
    color: Color,
    range: f32,
    translation: Vec3,
    velocity: Velocity,
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fire_event: EventReader<FireEvent>,
) {
    for ev in fire_event.iter() {
        commands
            .spawn(Fire {
                impact_radius: ev.fire.impact_radius,
                impact_vertices: ev.fire.impact_vertices,
            })
            .insert(Health(1))
            .insert(Mass(1.0))
            .insert(MomentOfInertia(1.0))
            .insert(ev.velocity)
            .insert(AngularVelocity(0.0))
            .insert(Collider {
                aabb: Aabb { hw: 0.0, hh: 0.0 },
                topology: Topology::Point,
            })
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: ev.radius,
                        vertices: ev.vertices,
                    }))
                    .into(),
                transform: Transform::from_translation(ev.translation)
                    .with_scale(Vec3::new(ev.range, ev.range, 0.0)),
                material: materials.add(ev.color.into()),
                ..Default::default()
            });
    }
}

pub fn movement(
    mut query: Query<(&mut Transform, &Velocity, &AngularVelocity), With<Fire>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, angular_velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time.delta_seconds());
        transform.scale -= Vec3::new(1.0, 1.0, 0.0);
    }
}

pub fn impact(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Handle<ColorMaterial>, &Fire, &Health, &Transform)>,
) {
    for (color, fire, health, transform) in query.iter_mut() {
        if health.0 <= 0 {
            commands
                .spawn(Impact)
                .insert(Health(10))
                .insert(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: fire.impact_radius,
                            vertices: fire.impact_vertices,
                        }))
                        .into(),
                    transform: Transform::from_translation(transform.translation),
                    material: color.clone(),
                    ..default()
                });
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health, &Transform), With<Fire>>) {
    for (entity, health, transform) in query.iter() {
        if health.0 <= 0 || transform.scale == Vec3::ZERO {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(fire)]
pub fn wreck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Handle<ColorMaterial>, &Fire, &Health, &Transform)>,
) {
    for (color, fire, health, transform) in query.iter() {
        if health.0 <= 0 {
            let color = materials.get(color).unwrap().color;
            commands
                .spawn(Impact)
                .insert(Health(10))
                .insert(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: fire.impact_radius,
                            vertices: fire.impact_vertices,
                        }))
                        .into(),
                    transform: *transform,
                    material: materials.add(color.into()),
                    ..default()
                });
        }
    }
}
