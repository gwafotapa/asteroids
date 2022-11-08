use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    asteroid::Asteroid,
    boss::{self, Boss},
    spaceship::{self, Spaceship},
    Enemy, Fire, Health, Velocity, ALTITUDE,
};

mod math;

#[derive(Component, Clone, Copy)]
pub struct RectangularEnvelop {
    pub half_x: f32,
    pub half_y: f32,
}

#[derive(Component)]
pub struct Impact;

#[derive(Component)]
pub struct Debris;

pub fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Velocity, &RectangularEnvelop), With<Spaceship>>,
    asteroid_query: Query<(&Transform, &Asteroid, &RectangularEnvelop)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((s_entity, s_transform, s_velocity, s_rectangular_envelop)) =
        spaceship_query.get_single()
    {
        for (a_transform, asteroid, a_rectangular_envelop) in asteroid_query.iter() {
            if math::rectangles_intersect(
                s_transform.translation,
                *s_rectangular_envelop,
                a_transform.translation,
                *a_rectangular_envelop,
            ) {
                for point in spaceship::ENVELOP {
                    if a_transform
                        .translation
                        // .distance((point + s_transform.translation) * s_transform.scale.x)
                        .distance(point * s_transform.scale + s_transform.translation)
                        < asteroid.radius
                    {
                        commands.entity(s_entity).despawn();
                        let mut rng = rand::thread_rng();
                        for _ in 1..10 {
                            let debris_dx = rng.gen_range(-30.0..30.0);
                            let debris_x = s_transform.translation.x + debris_dx;
                            let debris_dy = rng.gen_range(-20.0..20.0);
                            let debris_y = s_transform.translation.y + debris_dy;

                            let velocity = Vec3 {
                                x: rng.gen_range(-0.5..0.5),
                                y: rng.gen_range(-0.5..0.5),
                                z: 0.0,
                            };

                            commands
                                .spawn()
                                .insert(Debris)
                                .insert(Velocity(s_velocity.0 + velocity))
                                .insert_bundle(MaterialMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: 10.0,
                                            vertices: 4,
                                        }))
                                        .into(),
                                    transform: Transform::from_xyz(debris_x, debris_y, ALTITUDE)
                                        .with_scale(Vec3::splat(4.0)),
                                    material: materials.add(Color::BLUE.into()),
                                    ..default()
                                });
                        }

                        return;
                    }
                }
            }
        }
    }
}

pub fn detect_collision_fire_asteroid(
    mut commands: Commands,
    fire_query: Query<(Entity, &Transform), With<Fire>>,
    mut asteroid_query: Query<(
        Entity,
        &Transform,
        &Asteroid,
        &mut Health,
        &Velocity,
        &RectangularEnvelop,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (fire_entity, fire_transform) in fire_query.iter() {
        for (
            asteroid_entity,
            asteroid_transform,
            asteroid,
            mut asteroid_health,
            asteroid_velocity,
            asteroid_envelop,
        ) in asteroid_query.iter_mut()
        {
            if math::rectangles_intersect(
                fire_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                asteroid_transform.translation,
                *asteroid_envelop,
            ) {
                if fire_transform
                    .translation
                    .distance(asteroid_transform.translation)
                    < asteroid.radius
                {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: 4.0,
                                    vertices: 8,
                                }))
                                .into(),
                            transform: fire_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(spaceship::ATTACK_COLOR.into()),
                            ..default()
                        });

                    commands.entity(fire_entity).despawn();

                    asteroid_health.0 -= 1;
                    if asteroid_health.0 == 0 {
                        commands.entity(asteroid_entity).despawn();
                        let mut rng = rand::thread_rng();
                        for _ in 1..asteroid.radius as usize {
                            let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                            let debris_x = asteroid_transform.translation.x + debris_dx;
                            let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                            let debris_dy = rng.gen_range(-dy_max..dy_max);
                            let debris_y = asteroid_transform.translation.y + debris_dy;
                            // let z = rng.gen_range(
                            //     asteroid_transform.translation.z - asteroid.radius
                            //         ..asteroid_transform.translation.z + asteroid.radius,
                            // );

                            let velocity = Vec3 {
                                x: rng.gen_range(-0.5..0.5),
                                y: rng.gen_range(-0.5..0.5),
                                // z: rng.gen_range(-0.5..0.5),
                                z: 0.0,
                            };

                            commands
                                .spawn()
                                .insert(Debris)
                                .insert(Velocity(asteroid_velocity.0 + velocity))
                                // .insert(Velocity(asteroid_velocity.0 * 0.5))
                                .insert_bundle(MaterialMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: rng.gen_range(
                                                asteroid.radius / 100.0..asteroid.radius / 20.0,
                                            ),
                                            vertices: 8,
                                        }))
                                        .into(),
                                    transform: Transform::from_xyz(
                                        debris_x,
                                        debris_y,
                                        ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                                    )
                                    .with_scale(Vec3::splat(4.0)),
                                    material: materials.add(Color::PURPLE.into()),
                                    ..default()
                                });
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn update_debris(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Debris>>,
) {
    for (mut transform, velocity, debris) in query.iter_mut() {
        transform.translation += velocity.0;
        transform.scale -= 0.1;
        // if transform.translation.x < -WINDOW_WIDTH / 2.0
        //     || transform.translation.x > WINDOW_WIDTH / 2.0
        //     || transform.translation.y < -WINDOW_HEIGHT / 2.0
        //     || transform.translation.y > WINDOW_HEIGHT / 2.0
        if transform.scale.x < 0.05 {
            commands.entity(debris).despawn();
        }
    }
}

pub fn update_impacts(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Impact>>,
) {
    for (mut transform, impact) in query.iter_mut() {
        // transform.scale -= Vec3::ONE;
        transform.scale -= 0.5;
        // println!("{}", transform.scale);
        // if transform.scale == Vec3::ONE {
        if transform.scale.x < 0.25 {
            commands.entity(impact).despawn();
        }
    }
}

pub fn detect_collision_fire_boss(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), Without<Enemy>>,
    mut boss_query: Query<(Entity, &Transform, &mut Health, &RectangularEnvelop), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_envelop)) = boss_query.get_single_mut() {
        for (fire, fire_entity, fire_transform) in fire_query.iter() {
            if math::rectangles_intersect(
                fire_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                boss_transform.translation,
                *boss_envelop,
            ) {
                let boss_polygon = boss::POLYGON.map(|x| x + boss_transform.translation);
                let triangle_list = boss::create_triangle_list_from_polygon(
                    &boss_polygon,
                    boss_transform.translation,
                );
                let mut iter_triangle = triangle_list.iter();
                let mut collision = false;
                let mut p1 = iter_triangle.next();
                let mut p2 = iter_triangle.next();
                let mut p3 = iter_triangle.next();
                while !collision && p3.is_some() {
                    collision = math::point_in_triangle_2d(
                        *p1.unwrap(),
                        *p2.unwrap(),
                        *p3.unwrap(),
                        fire_transform.translation,
                    );
                    p1 = iter_triangle.next();
                    p2 = iter_triangle.next();
                    p3 = iter_triangle.next();
                }
                if collision {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            transform: fire_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(fire.color.into()),
                            ..default()
                        });

                    commands.entity(fire_entity).despawn();

                    boss_health.0 -= 1;
                    if boss_health.0 == 0 {
                        commands.entity(boss).despawn_recursive();
                        //                 commands.entity(asteroid_entity).despawn();
                        //                 let mut rng = rand::thread_rng();
                        //                 for _ in 1..asteroid.radius as usize {
                        //                     let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                        //                     let debris_x = asteroid_transform.translation.x + debris_dx;
                        //                     let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                        //                     let debris_dy = rng.gen_range(-dy_max..dy_max);
                        //                     let debris_y = asteroid_transform.translation.y + debris_dy;
                        //                     // let z = rng.gen_range(
                        //                     //     asteroid_transform.translation.z - asteroid.radius
                        //                     //         ..asteroid_transform.translation.z + asteroid.radius,
                        //                     // );

                        //                     let velocity = Vec3 {
                        //                         x: rng.gen_range(-0.5..0.5),
                        //                         y: rng.gen_range(-0.5..0.5),
                        //                         // z: rng.gen_range(-0.5..0.5),
                        //                         z: 0.0,
                        //                     };

                        //                     commands
                        //                         .spawn()
                        //                         .insert(Debris)
                        //                         .insert(Velocity(asteroid_velocity.0 + velocity))
                        //                         // .insert(Velocity(asteroid_velocity.0 * 0.5))
                        //                         .insert_bundle(MaterialMesh2dBundle {
                        //                             mesh: meshes
                        //                                 .add(Mesh::from(shape::Circle {
                        //                                     radius: rng.gen_range(
                        //                                         asteroid.radius / 100.0..asteroid.radius / 20.0,
                        //                                     ),
                        //                                     vertices: 8,
                        //                                 }))
                        //                                 .into(),
                        //                             transform: Transform::from_xyz(
                        //                                 debris_x,
                        //                                 debris_y,
                        //                                 ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                        //                             )
                        //                             .with_scale(Vec3::splat(4.0)),
                        //                             material: materials.add(Color::PURPLE.into()),
                        //                             ..default()
                        //                         });
                        //                 }
                        break;
                    }
                }
            }
        }
    }
}

pub fn detect_collision_fire_spaceship(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), With<Enemy>>,
    mut spaceship_query: Query<
        (Entity, &Transform, &mut Health, &RectangularEnvelop),
        With<Spaceship>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((spaceship, spaceship_transform, mut spaceship_health, spaceship_envelop)) =
        spaceship_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform) in fire_query.iter() {
            if math::rectangles_intersect(
                fire_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                spaceship_transform.translation,
                RectangularEnvelop {
                    half_x: spaceship_envelop.half_x * spaceship_transform.scale.x,
                    half_y: spaceship_envelop.half_y * spaceship_transform.scale.y,
                },
            ) {
                let mut iter_triangle = spaceship::TRIANGLE_LIST
                    .into_iter()
                    .map(|x| x.mul_add(spaceship_transform.scale, spaceship_transform.translation));
                let mut collision = false;
                let mut p1 = iter_triangle.next();
                let mut p2 = iter_triangle.next();
                let mut p3 = iter_triangle.next();
                while !collision && p3.is_some() {
                    collision = math::point_in_triangle_2d(
                        p1.unwrap(),
                        p2.unwrap(),
                        p3.unwrap(),
                        fire_transform.translation,
                    );
                    p1 = iter_triangle.next();
                    p2 = iter_triangle.next();
                    p3 = iter_triangle.next();
                }

                if collision {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            transform: fire_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(fire.color.into()),
                            ..default()
                        });

                    commands.entity(fire_entity).despawn();

                    spaceship_health.0 -= 1;
                    if spaceship_health.0 == 0 {
                        commands.entity(spaceship).despawn_recursive();
                        //                 commands.entity(asteroid_entity).despawn();
                        //                 let mut rng = rand::thread_rng();
                        //                 for _ in 1..asteroid.radius as usize {
                        //                     let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                        //                     let debris_x = asteroid_transform.translation.x + debris_dx;
                        //                     let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                        //                     let debris_dy = rng.gen_range(-dy_max..dy_max);
                        //                     let debris_y = asteroid_transform.translation.y + debris_dy;
                        //                     // let z = rng.gen_range(
                        //                     //     asteroid_transform.translation.z - asteroid.radius
                        //                     //         ..asteroid_transform.translation.z + asteroid.radius,
                        //                     // );

                        //                     let velocity = Vec3 {
                        //                         x: rng.gen_range(-0.5..0.5),
                        //                         y: rng.gen_range(-0.5..0.5),
                        //                         // z: rng.gen_range(-0.5..0.5),
                        //                         z: 0.0,
                        //                     };

                        //                     commands
                        //                         .spawn()
                        //                         .insert(Debris)
                        //                         .insert(Velocity(asteroid_velocity.0 + velocity))
                        //                         // .insert(Velocity(asteroid_velocity.0 * 0.5))
                        //                         .insert_bundle(MaterialMesh2dBundle {
                        //                             mesh: meshes
                        //                                 .add(Mesh::from(shape::Circle {
                        //                                     radius: rng.gen_range(
                        //                                         asteroid.radius / 100.0..asteroid.radius / 20.0,
                        //                                     ),
                        //                                     vertices: 8,
                        //                                 }))
                        //                                 .into(),
                        //                             transform: Transform::from_xyz(
                        //                                 debris_x,
                        //                                 debris_y,
                        //                                 ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                        //                             )
                        //                             .with_scale(Vec3::splat(4.0)),
                        //                             material: materials.add(Color::PURPLE.into()),
                        //                             ..default()
                        //                         });
                        //                 }
                        break;
                    }
                }
            }
        }
    }
}
