use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    asteroid::{self, Asteroid},
    boss::{self, Boss, BossPart},
    collision::math::{
        circle_intersects_triangle, point_in_rectangle, point_in_triangle, rectangles_intersect,
    },
    spaceship::{self, Spaceship},
    Debris, Enemy, Fire, Health, Velocity,
};

pub mod math;

pub type Triangle = [Vec3; 3];

#[derive(Clone, Copy)]
pub enum Topology {
    Point,
    Circle(f32),
    Triangles(&'static [Triangle]),
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
    transform1: &GlobalTransform,
    surface1: &Surface,
    transform2: &GlobalTransform,
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
            transform1.translation() == transform2.translation()
        }
        (
            circle1,
            Topology::Circle(radius1),
            hitbox1,
            circle2,
            Topology::Circle(radius2),
            hitbox2,
        ) => {
            if !rectangles_intersect(
                circle1.translation().truncate(),
                hitbox1,
                circle2.translation().truncate(),
                hitbox2,
            ) {
                return false;
            }

            circle1.translation().distance(circle2.translation()) < radius1 + radius2
        }
        (_, Topology::Triangles(triangles1), _, _, Topology::Triangles(triangles2), _) => {
            unimplemented!()
        }
        (point, Topology::Point, _, circle, Topology::Circle(radius), hitbox)
        | (circle, Topology::Circle(radius), hitbox, point, Topology::Point, _) => {
            if !point_in_rectangle(
                point.translation().truncate(),
                circle.translation().truncate(),
                hitbox.half_x,
                hitbox.half_y,
            ) {
                return false;
            }

            point.translation().distance(circle.translation()) < radius
        }
        (point, Topology::Point, _, triangles, Topology::Triangles(triangles_list), hitbox)
        | (triangles, Topology::Triangles(triangles_list), hitbox, point, Topology::Point, _) => {
            if !point_in_rectangle(
                point.translation().truncate(),
                triangles.translation().truncate(),
                hitbox.half_x,
                hitbox.half_y,
            ) {
                return false;
            }

            for &[a, b, c] in triangles_list.iter() {
                if point_in_triangle(
                    triangles
                        .to_scale_rotation_translation()
                        .1
                        .inverse()
                        .mul_vec3(point.translation() - triangles.translation())
                        .truncate(),
                    a.truncate(),
                    b.truncate(),
                    c.truncate(),
                ) {
                    return true;
                }
            }

            false
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
                circle_transform.translation().truncate(),
                circle_hitbox,
                triangles_transform.translation().truncate(),
                triangles_hitbox,
            ) {
                return false;
            }

            for triangle in triangles {
                if circle_intersects_triangle(
                    triangle[0].truncate(),
                    triangle[1].truncate(),
                    triangle[2].truncate(),
                    triangles_transform
                        .to_scale_rotation_translation()
                        .1
                        .inverse()
                        .mul_vec3(
                            circle_transform.translation() - triangles_transform.translation(),
                        )
                        .truncate(),
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
    mut spaceship_query: Query<(&GlobalTransform, &Surface, &mut Health), With<Spaceship>>,
    asteroid_query: Query<(&GlobalTransform, &Surface), With<Asteroid>>,
) {
    if let Ok((s_transform, s_surface, mut s_health)) = spaceship_query.get_single_mut() {
        for (a_transform, a_surface) in asteroid_query.iter() {
            if collision(s_transform, s_surface, a_transform, a_surface) {
                s_health.0 = 0;
                return;
            }
        }
    }
}

pub fn detect_collision_fire_asteroid(
    mut commands: Commands,
    fire_query: Query<(Entity, &Fire, &GlobalTransform, &Surface)>,
    mut asteroid_query: Query<(
        Entity,
        &GlobalTransform,
        &Asteroid,
        &mut Health,
        &Velocity,
        &Surface,
        // Option<&Children>,
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
            // asteroid_children,
        ) in asteroid_query.iter_mut()
        {
            if collision(
                fire_transform,
                fire_surface,
                asteroid_transform,
                asteroid_surface,
            ) {
                commands.entity(fire_entity).despawn();

                commands
                    .spawn_empty()
                    .insert(Impact)
                    // .insert(Velocity(asteroid_velocity.0))
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(fire_transform.translation()),
                        material: materials.add(fire.color.into()),
                        ..default()
                    });

                asteroid_health.0 -= 1;
                // let mut impact_translation = fire_transform.translation();
                // if asteroid_health.0 == 0 {
                // asteroid::explode(
                //     &mut commands,
                //     &mut meshes,
                //     &mut materials,
                //     asteroid,
                //     asteroid_entity,
                //     asteroid_transform,
                //     asteroid_velocity,
                //     // asteroid_children,
                // );
                // } else {
                //     impact_translation -= asteroid_transform.translation();
                //     commands.entity(asteroid_entity).add_child(impact);
                // }

                break;
            }
        }
    }
}

pub fn detect_collision_fire_boss(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &GlobalTransform, &Surface), Without<Enemy>>,
    mut boss_query: Query<(Entity, &GlobalTransform, &mut Health, &Velocity, &Surface), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_velocity, boss_surface)) =
        boss_query.get_single_mut()
    {
        for (fire, fire_entity, fire_transform, fire_surface) in fire_query.iter() {
            if collision(fire_transform, fire_surface, boss_transform, boss_surface) {
                let impact = commands
                    .spawn_empty()
                    .insert(Impact)
                    // .insert(Velocity(boss_velocity.0))
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        // transform: Transform::from_translation(
                        //     boss_transform
                        //         .to_scale_rotation_translation()
                        //         .1
                        //         .inverse()
                        //         .mul_vec3(
                        //             fire_transform.translation() - boss_transform.translation(),
                        //         ),
                        // ),
                        transform: Transform::from_translation(fire_transform.translation()),
                        // transform: *fire_transform,
                        material: materials.add(fire.color.into()),
                        ..default()
                    })
                    .id();

                // commands.entity(boss).add_child(impact);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_fire: Query<(&Fire, Entity, &GlobalTransform, &Surface), Without<Enemy>>,
    mut q_boss: Query<(Entity, &GlobalTransform, &Velocity), With<Boss>>,
    mut q_boss_part: Query<(Entity, &GlobalTransform, &Surface, &mut Health), With<BossPart>>,
) {
    if let Ok((b_entity, b_transform, b_velocity)) = q_boss.get_single_mut() {
        for (fire, f_entity, f_transform, f_surface) in q_fire.iter() {
            for (bp_entity, bp_transform, bp_surface, mut bp_health) in q_boss_part.iter_mut() {
                if collision(f_transform, f_surface, bp_transform, bp_surface) {
                    let impact = commands
                        .spawn_empty()
                        .insert(Impact)
                        // .insert(Velocity(b_velocity.0))
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            // transform: Transform::from_translation(
                            //     bp_transform
                            //         .to_scale_rotation_translation()
                            //         .1
                            //         .inverse()
                            //         .mul_vec3(
                            //             f_transform.translation() - bp_transform.translation(),
                            //         ),
                            // ),
                            transform: Transform::from_translation(f_transform.translation()),
                            // transform: *fire_transform,
                            material: materials.add(fire.color.into()),
                            ..default()
                        })
                        .id();

                    // commands.entity(bp_entity).add_child(impact);
                    commands.entity(f_entity).despawn();

                    bp_health.0 -= 1;
                    if bp_health.0 == 0 {
                        commands.entity(bp_entity).despawn();
                        // boss::explode(commands, meshes, materials, boss_transform, boss_velocity);
                        break;
                    }
                }
            }
        }
    }
}

pub fn detect_collision_fire_spaceship(
    mut commands: Commands,
    fire_query: Query<(&Fire, Entity, &GlobalTransform, &Surface), With<Enemy>>,
    mut spaceship_query: Query<
        (Entity, &GlobalTransform, &mut Health, &Velocity, &Surface),
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
                    .spawn_empty()
                    .insert(Impact)
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        // transform: Transform::from_translation(
                        //     fire_transform.translation() - spaceship_transform.translation(),
                        // ),
                        transform: Transform::from_translation(fire_transform.translation()),
                        material: materials.add(fire.color.into()),
                        ..default()
                    })
                    .id();

                // commands.entity(spaceship).add_child(impact);
                commands.entity(fire_entity).despawn();

                spaceship_health.0 -= 1;
                if spaceship_health.0 == 0 {
                    break;
                }
            }
        }
    }
}

pub fn detect_collision_asteroid_asteroid(
    mut query: Query<(
        &GlobalTransform,
        &Surface,
        Entity,
        &Asteroid,
        &Velocity,
        &mut Health,
        Option<&Children>,
    )>,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((transform1, surface1, entity1, asteroid1, velocity1, mut health1, children1)) =
            iter.next()
        {
            for (transform2, surface2, entity2, asteroid2, velocity2, mut health2, children2) in
                iter
            {
                if collision(transform1, surface1, transform2, surface2) {
                    if asteroid1.radius < asteroid2.radius {
                        health1.0 = 0;
                        break;
                    } else {
                        health2.0 = 0;
                    }
                }
            }
            i += 1;
        } else {
            break;
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
        // transform.translation += velocity.0;
        transform.scale -= 0.1;
        if transform.scale.x < 0.05 {
            // commands.entity(parent.get()).remove_children(&[impact]);
            commands.entity(impact).despawn();
        }
    }
}
