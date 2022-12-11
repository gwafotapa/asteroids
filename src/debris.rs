use bevy::prelude::*;

use crate::Velocity;

#[derive(Component)]
pub struct Debris;

pub fn scale_down(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Debris>>,
) {
    for (debris, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        transform.scale -= 0.01;
        // if transform.translation.x < -WINDOW_WIDTH / 2.0
        //     || transform.translation.x > WINDOW_WIDTH / 2.0
        //     || transform.translation.y < -WINDOW_HEIGHT / 2.0
        //     || transform.translation.y > WINDOW_HEIGHT / 2.0
        if transform.scale.x < 0.005 {
            commands.entity(debris).despawn();
        }
    }
}
