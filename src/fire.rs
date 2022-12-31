use bevy::prelude::*;

use crate::{collision::impact::Impact, AngularVelocity, Health, Velocity};

#[derive(Component)]
pub struct Fire {
    pub impact_radius: f32,
    pub impact_vertices: usize,
}

#[derive(Component)]
pub struct Enemy;

pub fn movement(mut query: Query<(&mut Transform, &Velocity, &AngularVelocity), With<Fire>>) {
    for (mut transform, velocity, angular_velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        transform.rotation *= Quat::from_axis_angle(Vec3::Z, angular_velocity.0);
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
