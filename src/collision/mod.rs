use std::collections::linked_list;

use bevy::{prelude::*, render::primitives::Sphere, sprite::MaterialMesh2dBundle};

use crate::{
    asteroid::{self, Asteroid},
    boss::{self, Boss, BossPart},
    spaceship::{self, Spaceship},
    Debris, Enemy, Fire, Health, Velocity,
};

pub mod math;

#[derive(Clone, Copy)]
pub enum Topology {
    Point,
    Circle(f32),
    Triangles(&'static [[Vec3; 3]]),
}

#[derive(Component, Clone, Copy)]
pub struct Surface {
    pub topology: Topology,
    pub hitbox: HitBox,
}

#[derive(Component, Clone, Copy)]
pub struct HitBox {
    pub half_x: f32,
    pub half_y: f32,
}

#[derive(Component)]
pub struct Impact;

fn collision(
    transform1: &Transform,
    surface1: &Surface,
    transform2: &Transform,
    surface2: &Surface,
) -> bool {
    match (
        transform1,
        surface1.topology,
        surface1.hitbox,
        transform2,
        surface2.topology,
        surface2.hitbox,
    ) {
        (_, Topology::Point, _, _, Topology::Point, _) => {
            transform1.translation == transform2.translation
        }
        (_, Topology::Circle(radius1), _, _, Topology::Circle(radius2), _) => unimplemented!(),
        (_, Topology::Triangles(list1), _, _, Topology::Triangles(list2), _) => unimplemented!(),
        (point, Topology::Point, _, circle, Topology::Circle(radius), hitbox)
        | (circle, Topology::Circle(radius), hitbox, point, Topology::Point, _) => {
            if point.translation.x < circle.translation.x - hitbox.half_x
                || point.translation.x > circle.translation.x + hitbox.half_x
                || point.translation.y < circle.translation.y - hitbox.half_y
                || point.translation.y > circle.translation.y + hitbox.half_y
            {
                false
            } else {
                point.translation.distance(circle.translation) < radius
            }
        }
        (_, Topology::Point, _, _, Topology::Triangles(list), _)
        | (_, Topology::Triangles(list), _, _, Topology::Point, _) => unimplemented!(),
        (_, Topology::Circle(radius), _, _, Topology::Triangles(list), _)
        | (_, Topology::Triangles(list), _, _, Topology::Circle(radius), _) => unimplemented!(),
    }
}

pub fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Velocity, &HitBox), With<Spaceship>>,
    asteroid_query: Query<(&Transform, &Asteroid, &HitBox)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((s_entity, s_transform, s_velocity, s_hit_box)) = spaceship_query.get_single() {
        for (a_transform, asteroid, a_hit_box) in asteroid_query.iter() {
            if math::rectangles_intersect(
                s_transform.translation,
                *s_hit_box,
                a_transform.translation,
                *a_hit_box,
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
    fire_query: Query<(Entity, &Fire, &Transform, &Surface)>,
    mut asteroid_query: Query<(
        Entity,
        &Transform,
        &Asteroid,
        &mut Health,
        &Velocity,
        &Surface,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (fire_entity, fire, fire_transform, fire_surface) in fire_query.iter() {
        for (
            asteroid_entity,
            asteroid_transform,
            asteroid,
            mut asteroid_health,
            asteroid_velocity,
            asteroid_surface,
        ) in asteroid_query.iter_mut()
        {
            if collision(
                fire_transform,
                fire_surface,
                asteroid_transform,
                asteroid_surface,
            ) {
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

pub fn detect_collision_fire_boss(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), Without<Enemy>>,
    mut boss_query: Query<(Entity, &Transform, &mut Health, &HitBox, &Velocity), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_envelop, boss_velocity)) =
        boss_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform) in fire_query.iter() {
            if math::rectangles_intersect(
                fire_transform.translation,
                HitBox {
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

pub fn detect_collision_fire_boss_parts(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), Without<Enemy>>,
    mut boss_query: Query<(Entity, &Transform, &HitBox, &Velocity, &Children), With<Boss>>,
    mut boss_parts_query: Query<&Health>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, boss_envelop, boss_velocity, boss_children)) =
        boss_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform) in fire_query.iter() {
            if math::rectangles_intersect(
                fire_transform.translation,
                HitBox {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                boss_transform.translation,
                *boss_envelop,
            ) {
                for child in boss_children.iter() {
                    // let triangles = boss::triangles_from_polygon(
                    //     &boss::POLYGON.map(|x| {
                    //         boss_transform.rotation.mul_vec3(x) + boss_transform.translation
                    //     }),
                    //     boss_transform.translation,
                    // );
                    // let mut iter_triangles = triangles.chunks(3);
                    // let mut collision = false;
                    // while let Some(&[a, b, c]) = iter_triangles.next() {
                    //     collision = math::point_in_triangle_2d(a, b, c, fire_transform.translation);
                    //     if collision {
                    //         break;
                    //     }
                    // }
                    // if collision {
                    //     let impact = commands
                    //         .spawn()
                    //         .insert(Impact)
                    //         .insert_bundle(MaterialMesh2dBundle {
                    //             mesh: meshes
                    //                 .add(Mesh::from(shape::Circle {
                    //                     radius: fire.impact_radius,
                    //                     vertices: fire.impact_vertices,
                    //                 }))
                    //                 .into(),
                    //             transform: Transform::from_translation(
                    //                 boss_transform.rotation.inverse().mul_vec3(
                    //                     fire_transform.translation - boss_transform.translation,
                    //                 ),
                    //             ),

                    //             // transform: *fire_transform,
                    //             material: materials.add(fire.color.into()),
                    //             ..default()
                    //         })
                    //         .id();

                    //     commands.entity(boss).add_child(impact);
                    //     commands.entity(fire_entity).despawn();

                    //     boss_health.0 -= 1;
                    //     if boss_health.0 == 0 {
                    //         commands.entity(boss).despawn_recursive();
                    //         boss::explode(
                    //             commands,
                    //             meshes,
                    //             materials,
                    //             boss_transform,
                    //             boss_velocity,
                    //         );
                    //         break;
                    //     }
                    // }
                }
            }
        }
    }
}

pub fn detect_collision_fire_spaceship(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &Transform), With<Enemy>>,
    mut spaceship_query: Query<
        (Entity, &Transform, &mut Health, &HitBox, &Velocity),
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
                HitBox {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                spaceship_transform.translation,
                HitBox {
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
