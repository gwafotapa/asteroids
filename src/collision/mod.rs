use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    asteroid::Asteroid,
    boss::{BossCore, BossEdge},
    collision::math::{
        circle_intersects_triangle, point_in_rectangle, point_in_triangle, rectangles_intersect,
    },
    spaceship::Spaceship,
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
        (_, Topology::Triangles(_triangles1), _, _, Topology::Triangles(_triangles2), _) => {
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
                    triangles_transform
                        .to_scale_rotation_translation()
                        .1
                        .inverse()
                        .mul_vec3(
                            circle_transform.translation() - triangles_transform.translation(),
                        )
                        .truncate(),
                    radius,
                    triangle[0].truncate(),
                    triangle[1].truncate(),
                    triangle[2].truncate(),
                ) {
                    return true;
                }
            }

            false
        }
    }
}

pub fn detect_collision_spaceship_asteroid(
    mut query_spaceship: Query<(&GlobalTransform, &mut Health, &Surface), With<Spaceship>>,
    query_asteroid: Query<(&GlobalTransform, &Surface), With<Asteroid>>,
) {
    if let Ok((s_transform, mut s_health, s_surface)) = query_spaceship.get_single_mut() {
        for (a_transform, a_surface) in query_asteroid.iter() {
            if collision(s_transform, s_surface, a_transform, a_surface) {
                s_health.0 = 0;
                return;
            }
        }
    }
}

pub fn detect_collision_fire_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Handle<ColorMaterial>,
            &Fire,
            &GlobalTransform,
            &mut Health,
            &Surface,
        ),
        Without<Asteroid>,
    >,
    mut query_asteroid: Query<(Entity, &GlobalTransform, &mut Health, &Surface), With<Asteroid>>,
) {
    for (f_color, fire, f_transform, mut f_health, f_surface) in query_fire.iter_mut() {
        for (a_entity, a_transform, mut a_health, a_surface) in query_asteroid.iter_mut() {
            if collision(f_transform, f_surface, a_transform, a_surface) {
                a_health.0 -= 1;
                f_health.0 -= 1;
                let color = materials.get(f_color).unwrap().color;

                let impact = commands
                    .spawn_empty()
                    .insert(Impact)
                    .insert(Health(10))
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(
                            f_transform.translation() - a_transform.translation(),
                        ),
                        material: materials.add(color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(a_entity).add_child(impact);

                break;
            }
        }
    }
}

// pub fn detect_collision_fire_boss(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query_fire: Query<
//         (Entity, &Fire, &GlobalTransform, &mut Health, &Surface),
//         (Without<Boss>, Without<Enemy>),
//     >,
//     mut query_boss: Query<(Entity, &GlobalTransform, &mut Health, &Surface, &Velocity), With<Boss>>,
// ) {
//     if let Ok((b_entity, b_transform, mut b_health, b_surface, b_velocity)) =
//         query_boss.get_single_mut()
//     {
//         for (f_entity, fire, f_transform, mut f_health, f_surface) in query_fire.iter_mut() {
//             if collision(f_transform, f_surface, b_transform, b_surface) {
//                 b_health.0 -= 1;
//                 f_health.0 -= 1;

//                 if b_health.0 == 0 {
//                     commands.entity(b_entity).despawn_recursive();
//                     // boss::explode(commands, meshes, materials, b_transform, b_velocity);
//                     break;
//                 }
//                 let impact = commands
//                     .spawn_empty()
//                     .insert(Impact)
//                     .insert(Health(1))
//                     // .insert(Velocity(b_velocity.0))
//                     .insert(MaterialMesh2dBundle {
//                         mesh: meshes
//                             .add(Mesh::from(shape::Circle {
//                                 radius: fire.impact_radius,
//                                 vertices: fire.impact_vertices,
//                             }))
//                             .into(),
//                         // transform: Transform::from_translation(
//                         //     b_transform
//                         //         .to_scale_rotation_translation()
//                         //         .1
//                         //         .inverse()
//                         //         .mul_vec3(
//                         //             f_transform.translation() - b_transform.translation(),
//                         //         ),
//                         // ),
//                         transform: Transform::from_translation(f_transform.translation()),
//                         // transform: *f_transform,
//                         material: materials.add(fire.color.into()),
//                         ..default()
//                     })
//                     .id();

//                 // commands.entity(b_entity).add_child(impact);
//             }
//         }
//     }
// }

pub fn detect_collision_fire_boss_part(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Handle<ColorMaterial>,
            Entity,
            &Fire,
            &GlobalTransform,
            &mut Health,
            &Surface,
            &mut Velocity,
        ),
        Without<Enemy>,
    >,
    // mut query_boss_core: Query<
    //     (
    //         Option<&Children>,
    //         &Handle<ColorMaterial>,
    //         Entity,
    //         &GlobalTransform,
    //         &mut Health,
    //         &Surface,
    //     ),
    //     (With<BossCore>, Without<Fire>),
    // >,
    mut query_boss_part: Query<
        (
            Option<&BossCore>,
            &Handle<ColorMaterial>,
            Entity,
            &GlobalTransform,
            &mut Health,
            &Surface,
        ),
        (Or<(With<BossEdge>, With<BossCore>)>, Without<Fire>),
    >,
) {
    // let (bc_children, bc_color, bc_entity, bc_transform, mut bc_health, bc_surface) =
    //     query_boss_core.single();
    for (f_color, f_entity, fire, f_transform, mut f_health, f_surface, mut f_velocity) in
        query_fire.iter_mut()
    {
        for (bc, bp_color, bp_entity, bp_transform, mut bp_health, bp_surface) in
            query_boss_part.iter_mut()
        {
            if collision(f_transform, f_surface, bp_transform, bp_surface) {
                if bc.is_none() || bc.unwrap().edges == 0 {
                    f_health.0 -= 1;
                    bp_health.0 -= 1;

                    let f_color = materials.get(f_color).unwrap().color;
                    let bp_color = materials.get_mut(bp_color).unwrap();
                    let [mut r, mut g, mut b, _] = bp_color.color.as_rgba_f32();
                    let [r2, g2, b2, _] = f_color.as_rgba_f32();
                    r += (r2 - r) / (1. + bp_health.0 as f32);
                    g += (g2 - g) / (1. + bp_health.0 as f32);
                    b += (b2 - b) / (1. + bp_health.0 as f32);
                    bp_color.color = Color::rgb(r, g, b);

                    let impact = commands
                        .spawn_empty()
                        .insert(Impact)
                        .insert(Health(10))
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: fire.impact_radius,
                                    vertices: fire.impact_vertices,
                                }))
                                .into(),
                            transform: Transform::from_translation(
                                bp_transform
                                    .to_scale_rotation_translation()
                                    .1
                                    .inverse()
                                    .mul_vec3(
                                        f_transform.translation() - bp_transform.translation(),
                                    ),
                            ),
                            material: materials.add(f_color.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(bp_entity).add_child(impact);
                } else {
                    f_velocity.0 = -f_velocity.0;
                    commands.entity(f_entity).insert(Enemy);
                }
            }
        }
    }
}

pub fn detect_collision_fire_spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Handle<ColorMaterial>,
            &Fire,
            &GlobalTransform,
            &mut Health,
            &Surface,
        ),
        (With<Enemy>, Without<Spaceship>),
    >,
    mut query_spaceship: Query<(Entity, &GlobalTransform, &mut Health, &Surface), With<Spaceship>>,
) {
    if let Ok((s_entity, s_transform, mut s_health, s_surface)) = query_spaceship.get_single_mut() {
        for (f_color, fire, f_transform, mut f_health, f_surface) in query_fire.iter_mut() {
            if collision(f_transform, f_surface, s_transform, s_surface) {
                f_health.0 -= 1;
                s_health.0 -= 1;
                let f_color = materials.get(f_color).unwrap().color;

                let impact = commands
                    .spawn_empty()
                    .insert(Impact)
                    .insert(Health(10))
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(
                            f_transform.translation() - s_transform.translation(),
                        ),
                        // transform: Transform::from_translation(f_transform.translation()),
                        material: materials.add(f_color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(s_entity).add_child(impact);
            }
        }
    }
}

pub fn detect_collision_asteroid_asteroid(
    mut query: Query<(&Asteroid, &GlobalTransform, &mut Health, &Surface)>,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((asteroid1, transform1, mut health1, surface1)) = iter.next() {
            for (asteroid2, transform2, mut health2, surface2) in iter {
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

pub fn update_impacts(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Parent>, &mut Transform), With<Impact>>,
) {
    for (entity, mut health, parent, mut transform) in query.iter_mut() {
        health.0 -= 1;
        // if health.0 > 5 {
        // transform.scale += 0.1;
        // } else if health.0 > 0 {
        transform.scale -= 0.1;
        // } else {
        if health.0 <= 0 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[entity]);
            }
        }
    }
}

pub fn despawn_impacts(mut commands: Commands, query: Query<(Entity, &Health), With<Impact>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
