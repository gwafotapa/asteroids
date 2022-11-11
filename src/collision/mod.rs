use std::collections::linked_list;

use bevy::{prelude::*, render::primitives::Sphere, sprite::MaterialMesh2dBundle};

use crate::{
    asteroid::{self, Asteroid},
    boss::{self, Boss, BossPart},
    collision::math::{circle_intersects_triangle, point_in_triangle, rectangles_intersect},
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
        (_, Topology::Triangles(triangles1), _, _, Topology::Triangles(triangles2), _) => {
            unimplemented!()
        }
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
        (point, Topology::Point, _, triangles, Topology::Triangles(triangles_list), hitbox)
        | (triangles, Topology::Triangles(triangles_list), hitbox, point, Topology::Point, _) => {
            if point.translation.x < triangles.translation.x - hitbox.half_x
                || point.translation.x > triangles.translation.x + hitbox.half_x
                || point.translation.y < triangles.translation.y - hitbox.half_y
                || point.translation.y > triangles.translation.y + hitbox.half_y
            {
                false
            } else {
                for &[a, b, c] in triangles_list.iter() {
                    if point_in_triangle(
                        a.truncate(),
                        b.truncate(),
                        c.truncate(),
                        triangles
                            .rotation
                            .inverse()
                            .mul_vec3(point.translation - triangles.translation)
                            .truncate(),
                    ) {
                        return true;
                    }
                }
                false
            }
        }
        (
            circle_transform,
            Topology::Circle(radius),
            circle_hitbox,
            triangles_transform,
            Topology::Triangles(triangles),
            triangles_hitbox,
        )
        | (
            triangles_transform,
            Topology::Triangles(triangles),
            triangles_hitbox,
            circle_transform,
            Topology::Circle(radius),
            circle_hitbox,
        ) => {
            if !rectangles_intersect(
                circle_transform.translation.truncate(),
                circle_hitbox,
                triangles_transform.translation.truncate(),
                triangles_hitbox,
            ) {
                return false;
            }
            for triangle in triangles {
                if circle_intersects_triangle(
                    triangle[0].truncate(),
                    triangle[1].truncate(),
                    triangle[2].truncate(),
                    circle_transform.translation.truncate(),
                    radius,
                ) {
                    return true;
                }
            }
            false
        }
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
                s_transform.translation.truncate(),
                *s_hit_box,
                a_transform.translation.truncate(),
                *a_hit_box,
            ) {
                for point in spaceship::ENVELOP {
                    if a_transform
                        .translation
                        .distance(point * s_transform.scale + s_transform.translation)
                        < asteroid.radius
                    {
                        commands.entity(s_entity).despawn();
                        spaceship::explode(
                            commands,
                            meshes,
                            materials,
                            s_entity,
                            s_transform,
                            s_velocity,
                        );
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
                            fire_transform.translation - asteroid_transform.translation,
                        ),
                        material: materials.add(fire.color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(asteroid_entity).add_child(impact);
                commands.entity(fire_entity).despawn();

                asteroid_health.0 -= 1;
                if asteroid_health.0 == 0 {
                    asteroid::explode(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid,
                        asteroid_entity,
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
    fire_query: Query<(&Fire, Entity, &Transform, &Surface), Without<Enemy>>,
    mut boss_query: Query<(Entity, &Transform, &mut Health, &Velocity, &Surface), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_velocity, boss_surface)) =
        boss_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform, fire_surface) in fire_query.iter() {
            if collision(fire_transform, fire_surface, boss_transform, boss_surface) {
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
                            boss_transform
                                .rotation
                                .inverse()
                                .mul_vec3(fire_transform.translation - boss_transform.translation),
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
                fire_transform.translation.truncate(),
                HitBox {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                boss_transform.translation.truncate(),
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
                    //     collision = math::point_in_triangle(a, b, c, fire_transform.translation);
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
    fire_query: Query<(&Fire, Entity, &Transform, &Surface), With<Enemy>>,
    mut spaceship_query: Query<
        (Entity, &Transform, &mut Health, &Velocity, &Surface),
        With<Spaceship>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((spaceship, spaceship_transform, mut spaceship_health, velocity, spaceship_surface)) =
        spaceship_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform, fire_surface) in fire_query.iter() {
            if collision(
                fire_transform,
                fire_surface,
                spaceship_transform,
                spaceship_surface,
            ) {
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
                            fire_transform.translation - spaceship_transform.translation,
                        ),
                        material: materials.add(fire.color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(spaceship).add_child(impact);
                commands.entity(fire_entity).despawn();

                spaceship_health.0 -= 1;
                if spaceship_health.0 == 0 {
                    spaceship::explode(
                        commands,
                        meshes,
                        materials,
                        spaceship,
                        spaceship_transform,
                        velocity,
                    );
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
