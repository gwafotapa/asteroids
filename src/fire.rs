use bevy::prelude::*;

use crate::{Health, Velocity};

#[derive(Component)]
pub struct Fire {
    pub impact_radius: f32,
    pub impact_vertices: usize,
}

pub fn update(mut query: Query<(&mut Health, &mut Transform, &Velocity), With<Fire>>) {
    for (mut health, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        transform.scale -= 0.015;
        // transform.scale *= 0.97;
        // transform.scale.z = 1.0;
        if transform.scale.x < 0.1 {
            health.0 = 0;
        }
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
