use bevy::prelude::*;

use crate::Health;

#[derive(Component)]
pub struct Impact;

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Parent>, &mut Transform), With<Impact>>,
) {
    for (entity, mut health, parent, mut transform) in query.iter_mut() {
        health.0 -= 1;
        // if health.0 > 5 {
        // transform.scale += 0.1;
        // } else if health.0 > 0 {
        transform.scale -= 0.1;
        // } else {
        if health.0 <= 0 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[entity]);
            }
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Impact>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
