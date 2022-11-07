use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{Health, Level, RectangularEnvelop, Velocity, ALTITUDE, WINDOW_HEIGHT, WINDOW_WIDTH};

const MAX_SPEED_OF_ASTEROIDS: usize = 5;
const MAX_HEALTH_OF_ASTEROIDS: usize = 6;

#[derive(Component)]
pub struct Asteroid {
    pub radius: f32,
}
pub fn asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroid_query: Query<(&mut Transform, &Velocity, &Asteroid, Entity)>,
    level_query: Query<&Level>,
) {
    let mut rng = rand::thread_rng();

    if level_query.single().distance_to_boss > 0 && rng.gen_range(0..100) == 0 {
        let health = rng.gen_range(1..MAX_HEALTH_OF_ASTEROIDS + 1);
        let radius = (health * 20) as f32;
        let speed = rng.gen_range(1..MAX_SPEED_OF_ASTEROIDS + 1) as f32;
        let velocity = Vec3::from([-speed, 0., 0.]);
        let x = WINDOW_WIDTH / 2.0 + (MAX_HEALTH_OF_ASTEROIDS * 20) as f32;
        let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

        commands
            .spawn()
            .insert(Asteroid { radius })
            .insert(Health(health))
            .insert(Velocity(velocity))
            .insert(RectangularEnvelop {
                half_x: radius,
                half_y: radius,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: 16,
                    }))
                    .into(),
                transform: Transform::from_xyz(x, y, ALTITUDE),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            });
    }

    for (mut transform, velocity, asteroid, entity) in asteroid_query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - asteroid.radius {
            commands.entity(entity).despawn();
        }
    }
}
