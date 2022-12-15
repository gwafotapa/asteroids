use bevy::prelude::*;

use crate::{Health, Velocity};

#[derive(Component)]
pub struct Fire {
    pub scale_down: f32,
    pub impact_radius: f32,
    pub impact_vertices: usize,
}

#[derive(Component)]
pub struct Enemy;

pub fn update(mut query: Query<(&Fire, &mut Health, &mut Transform, &Velocity)>) {
    for (fire, mut health, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        health.0 -= 1;
        transform.scale -= fire.scale_down;
        // transform.scale *= 0.97;
        // transform.scale.z = 1.0;
        // if transform.scale.x < 0.1 {
        //     health.0 = 0;
        // }
        // if transform.translation.x > WINDOW_WIDTH / 2.0
        //     || transform.translation.x < -WINDOW_WIDTH / 2.0
        //     || transform.translation.y > WINDOW_HEIGHT / 2.0
        //     || transform.translation.y < -WINDOW_HEIGHT / 2.0
        // {
        //     health.0 = 0;
        // }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Fire>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
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
