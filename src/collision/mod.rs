use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    asteroid::{self, Asteroid},
    boss::{self, Boss},
    spaceship::{self, Spaceship},
    Debris, Enemy, Fire, Health, Velocity,
};

pub mod math;

#[derive(Component, Clone, Copy)]
pub struct RectangularEnvelop {
    pub half_x: f32,
    pub half_y: f32,
}

#[derive(Component)]
pub struct Impact;

pub fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Velocity, &RectangularEnvelop), With<Spaceship>>,
    asteroid_query: Query<(&Transform, &Asteroid, &RectangularEnvelop)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
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
                        .distance(point * s_transform.scale + s_transform.translation)
                        < asteroid.radius
                    {
                        commands.entity(s_entity).despawn();
                        spaceship::explode(commands, meshes, materials, s_transform, s_velocity);
                        return;
                    }
                }
            }
        }
    }
}

pub fn detect_collision_fire_asteroid(
    mut commands: Commands,
    fire_query: Query<(Entity, &Fire, &Transform)>,
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
    for (fire_entity, fire, fire_transform) in fire_query.iter() {
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
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            transform: *fire_transform,
                            material: materials.add(fire.color.into()),
                            ..default()
                        });

                    commands.entity(fire_entity).despawn();

                    asteroid_health.0 -= 1;
                    if asteroid_health.0 == 0 {
                        commands.entity(asteroid_entity).despawn();
                        asteroid::explode(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            asteroid,
                            asteroid_transform,
                            asteroid_velocity,
                        );
                    }
                    break;
                }
            }
        }
    }
}

pub fn detect_collision_fire_boss(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), Without<Enemy>>,
    mut boss_query: Query<
        (
            Entity,
            &Transform,
            &mut Health,
            &RectangularEnvelop,
            &Velocity,
        ),
        With<Boss>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_envelop, boss_velocity)) =
        boss_query.get_single_mut()
    {
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
                let triangles = boss::triangles_from_polygon(
                    &boss::POLYGON
                        .map(|x| boss_transform.rotation.mul_vec3(x) + boss_transform.translation),
                    boss_transform.translation,
                );
                let mut iter_triangles = triangles.chunks(3);
                let mut collision = false;
                while let Some(&[a, b, c]) = iter_triangles.next() {
                    collision = math::point_in_triangle_2d(a, b, c, fire_transform.translation);
                    if collision {
                        break;
                    }
                }
                if collision {
                    let impact = commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            transform: Transform::from_translation(
                                boss_transform.rotation.inverse().mul_vec3(
                                    fire_transform.translation - boss_transform.translation,
                                ),
                            ),

                            // transform: *fire_transform,
                            material: materials.add(fire.color.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(boss).add_child(impact);
                    commands.entity(fire_entity).despawn();

                    boss_health.0 -= 1;
                    if boss_health.0 == 0 {
                        commands.entity(boss).despawn_recursive();
                        boss::explode(commands, meshes, materials, boss_transform, boss_velocity);
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
        (
            Entity,
            &Transform,
            &mut Health,
            &RectangularEnvelop,
            &Velocity,
        ),
        With<Spaceship>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((spaceship, spaceship_transform, mut spaceship_health, spaceship_envelop, velocity)) =
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
                let triangles = spaceship::TRIANGLE_LIST
                    .map(|x| x.mul_add(spaceship_transform.scale, spaceship_transform.translation));
                let mut iter_triangles = triangles.chunks(3);
                let mut collision = false;
                while let Some(&[a, b, c]) = iter_triangles.next() {
                    collision = math::point_in_triangle_2d(a, b, c, fire_transform.translation);
                    if collision {
                        break;
                    }
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
                            transform: *fire_transform,
                            material: materials.add(fire.color.into()),
                            ..default()
                        });

                    commands.entity(fire_entity).despawn();

                    spaceship_health.0 -= 1;
                    if spaceship_health.0 == 0 {
                        commands.entity(spaceship).despawn();
                        spaceship::explode(
                            commands,
                            meshes,
                            materials,
                            spaceship_transform,
                            velocity,
                        );
                        break;
                    }
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

pub fn update_impacts(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Impact>>,
) {
    for (mut transform, impact) in query.iter_mut() {
        transform.scale -= 0.05;
        if transform.scale.x < 0.05 {
            commands.entity(impact).despawn();
        }
    }
}
