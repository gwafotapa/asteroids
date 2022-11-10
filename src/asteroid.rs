use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    collision::{HitBox, Surface, Topology},
    Debris, Health, Level, Velocity, ALTITUDE, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const MAX_SPEED: usize = 5;
const MAX_HEALTH: usize = 6;
pub const COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

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
        let health = rng.gen_range(1..MAX_HEALTH + 1);
        let radius = (health * 20) as f32;
        let speed = rng.gen_range(1..MAX_SPEED + 1) as f32;
        let velocity = Vec3::from([-speed, 0., 0.]);
        let x = WINDOW_WIDTH / 2.0 + (MAX_HEALTH * 20) as f32;
        let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

        commands
            .spawn()
            .insert(Asteroid { radius })
            .insert(Health(health))
            .insert(Velocity(velocity))
            .insert(Surface {
                topology: Topology::Circle(radius),
                hitbox: HitBox {
                    half_x: radius,
                    half_y: radius,
                },
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: 16,
                    }))
                    .into(),
                transform: Transform::from_xyz(x, y, ALTITUDE),
                material: materials.add(ColorMaterial::from(COLOR)),
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

pub fn explode(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asteroid: &Asteroid,
    transform: &Transform,
    velocity: &Velocity,
) {
    let mut rng = rand::thread_rng();
    for _ in 1..asteroid.radius as usize {
        let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
        let debris_x = transform.translation.x + debris_dx;
        let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
        let debris_dy = rng.gen_range(-dy_max..dy_max);
        let debris_y = transform.translation.y + debris_dy;
        // let z = rng.gen_range(
        //     transform.translation.z - asteroid.radius
        //         ..transform.translation.z + asteroid.radius,
        // );

        let dv = Vec3 {
            x: rng.gen_range(-0.5..0.5),
            y: rng.gen_range(-0.5..0.5),
            // z: rng.gen_range(-0.5..0.5),
            z: 0.0,
        };

        commands
            .spawn()
            .insert(Debris)
            .insert(Velocity(velocity.0 + dv))
            // .insert(Velocity(velocity.0 * 0.5))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: rng.gen_range(asteroid.radius / 100.0..asteroid.radius / 20.0),
                        vertices: 8,
                    }))
                    .into(),
                transform: Transform::from_xyz(
                    debris_x,
                    debris_y,
                    ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                ),
                material: materials.add(COLOR.into()),
                ..default()
            });
    }
}
