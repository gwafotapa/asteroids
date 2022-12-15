use bevy::prelude::*;

use crate::Health;

#[derive(Component)]
pub struct Blast;

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Parent>), With<Blast>>,
) {
    for (blast, mut health, parent) in query.iter_mut() {
        health.0 -= 1;
        if health.0 <= 0 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[blast]);
            }
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Blast>>) {
    for (blast, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(blast).despawn();
        }
    }
}
