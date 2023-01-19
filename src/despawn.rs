use bevy::prelude::*;

use crate::component::Health;

pub fn with<C: Component>(mut commands: Commands, query: Query<(Entity, &Health), With<C>>) {
    for (entity, health) in query.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn recursive_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
