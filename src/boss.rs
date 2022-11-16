use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::{PI, SQRT_2};

use crate::{
    asteroid::Asteroid,
    collision::{math, HitBox, Surface, Topology, Triangle},
    spaceship::Spaceship,
    Blast, Debris, Direction, Enemy, Fire, Health, Level, Velocity, ALTITUDE, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

const INNER_RADIUS: f32 = 100.0;
const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossPart;

const A0: Vec3 = Vec3 {
    x: -OUTER_RADIUS,
    y: 0.0,
    z: 0.0,
};
const A1: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
const A2: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A3: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A4: Vec3 = Vec3 {
    x: 0.0,
    y: -OUTER_RADIUS,
    z: 0.0,
};
const A5: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A6: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A7: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
const A8: Vec3 = Vec3 {
    x: OUTER_RADIUS,
    y: 0.0,
    z: 0.0,
};
const A9: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};
const A10: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A11: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A12: Vec3 = Vec3 {
    x: 0.0,
    y: OUTER_RADIUS,
    z: 0.0,
};
const A13: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A14: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A15: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};

/// Counter clockwise
pub const POLYGON: [Vec3; 16] = [
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15,
];

const INITIAL_POSITION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0 + OUTER_RADIUS,
    y: 0.0,
    z: ALTITUDE,
};
const ACCELERATION: f32 = 0.01;
const COLOR: Color = Color::rgb(0.25, 0.5, 0.25);
const HEALTH: i32 = 10;

pub const ATTACK_COLOR: Color = Color::RED;
const FIRE_VELOCITY: f32 = 4.0;
const ATTACK_SOURCE: [Vec3; 8] = [A0, A2, A4, A6, A8, A10, A12, A14];
const BLAST_RADIUS: f32 = 15.0;
const BLAST_VERTICES: usize = 32;
const FIRE_RADIUS: f32 = 5.0;
const FIRE_VERTICES: usize = 32;
const IMPACT_RADIUS: f32 = 15.0;
const IMPACT_VERTICES: usize = 32;
const ROTATION_SPEED: f32 = 0.0;

#[derive(Component)]
pub struct Attack(Vec3);

// const TRIANGLES: [[Vec3; 3]; 14] = [
//     [A1, A2, A3],
//     [A3, A4, A5],
//     [A5, A6, A7],
//     [A7, A8, A9],
//     [A9, A10, A11],
//     [A11, A12, A13],
//     [A13, A14, A15],
//     [A15, A0, A1],
//     [A1, A3, A15],
//     [A3, A13, A15],
//     [A3, A11, A13],
//     [A3, A5, A11],
//     [A5, A9, A11],
//     [A5, A7, A9],
// ];

// There are 8 egdes.
// Each edge is a triangle and constitutes a whole part of the boss.
const EDGES: usize = 8;
const EDGES_TRIANGLES: [[Triangle; 1]; EDGES] = [
    [[A1, A2, A3]],
    [[A3, A4, A5]],
    [[A5, A6, A7]],
    [[A7, A8, A9]],
    [[A9, A10, A11]],
    [[A11, A12, A13]],
    [[A13, A14, A15]],
    [[A15, A0, A1]],
];

// The body is a collection of 6 triangles. It is a single part of the boss.
const CORE_PARTS: usize = 6;
const CORE_TRIANGLES: [[Vec3; 3]; CORE_PARTS] = [
    [A1, A3, A15],
    [A3, A13, A15],
    [A3, A11, A13],
    [A3, A5, A11],
    [A5, A9, A11],
    [A5, A7, A9],
];

const C1: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: 0.0,
    z: 0.0,
};

const C2: Vec3 = Vec3 {
    x: 0.0,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};

const C3: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: 0.0,
    z: 0.0,
};

const EDGE: [Triangle; 1] = [[C1, C2, C3]];
// pub fn triangles_from_polygon(polygon: &[Vec3], center: Vec3) -> Vec<Vec3> {
//     let mut triangles = Vec::new();
//     for (&a, &b) in polygon
//         .iter()
//         .zip(polygon.iter().skip(1).chain(polygon.iter().take(1)))
//     {
//         triangles.extend_from_slice(&[center, a, b]);
//     }
//     triangles
// }

pub fn add_boss_parts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_level: Query<&mut Level>,
    query_asteroid: Query<With<Asteroid>>,
) {
    let mut level = query_level.single_mut();
    if !level.boss_spawned && level.distance_to_boss == 0 && query_asteroid.is_empty() {
        let boss = commands
            .spawn_empty()
            .insert(Boss)
            .insert(Velocity(Vec3::ZERO))
            // .insert(HitBox {
            //     half_x: OUTER_RADIUS,
            //     half_y: OUTER_RADIUS,
            // })
            .insert(SpatialBundle {
                transform: Transform::from_translation(INITIAL_POSITION),
                ..default()
            })
            .id();

        // Build core
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices_position: Vec<[f32; 3]> = CORE_TRIANGLES
            .iter()
            .flatten()
            .map(|x| x.to_array())
            .collect();
        let vertices_normal = vec![[0.0, 0.0, 1.0]; 3 * CORE_PARTS];
        let vertices_uv = vec![[0.0, 0.0]; 3 * CORE_PARTS];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

        let boss_body = commands
            .spawn_empty()
            .insert(BossPart)
            .insert(Health(HEALTH))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(COLOR.into()),
                ..default()
            })
            .insert(Surface {
                topology: Topology::Triangles(&CORE_TRIANGLES),
                hitbox: HitBox {
                    half_x: INNER_RADIUS,
                    half_y: INNER_RADIUS,
                },
            })
            .id();

        commands.entity(boss).add_child(boss_body);

        // Add edges
        // for triangle in EDGES_TRIANGLES {
        for i in 0..EDGES {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            // let vertices_position: Vec<[f32; 3]> = EDGES_TRIANGLES[i]
            //     .iter()
            //     .flatten()
            //     .map(|x| x.to_array())
            //     .collect();
            let vertices_position = vec![C1.to_array(), C2.to_array(), C3.to_array()];
            let vertices_normal = vec![[0.0, 0.0, 1.0]; 3];
            let vertices_uv = vec![[0.0, 0.0]; 3];

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

            let boss_edge = commands
                .spawn_empty()
                .insert(BossPart)
                .insert(Health(HEALTH))
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    transform: Transform::from_xyz(
                        INNER_RADIUS * (i as f32 * PI / 4.0).cos(),
                        INNER_RADIUS * (i as f32 * PI / 4.0).sin(),
                        0.0,
                    )
                    .with_rotation(Quat::from_axis_angle(
                        Vec3::from([0.0, 0.0, 1.0]),
                        (i + 6) as f32 * PI / 4.0,
                    )),
                    material: materials.add(COLOR.into()),
                    ..default()
                })
                .insert(Surface {
                    // topology: Topology::Triangles(&EDGES_TRIANGLES[i]),
                    // hitbox: math::triangle_hitbox(
                    //     EDGES_TRIANGLES[i][0][0].truncate(),
                    //     EDGES_TRIANGLES[i][0][1].truncate(),
                    //     EDGES_TRIANGLES[i][0][2].truncate(),
                    // ),
                    topology: Topology::Triangles(&EDGE),
                    hitbox: HitBox {
                        half_x: OUTER_RADIUS - INNER_RADIUS,
                        half_y: OUTER_RADIUS - INNER_RADIUS,
                    },
                })
                // .insert(Attack(EDGES_TRIANGLES[i][0][1]))
                .insert(Attack(C2))
                .id();

            commands.entity(boss).add_child(boss_edge);
        }
        level.boss_spawned = true;
    }
}

// pub fn add_boss(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query_level: Query<&mut Level>,
//     query_asteroid: Query<&Asteroid>,
// ) {
//     let mut level = query_level.single_mut();
//     if !level.boss.spawn_emptyed && level.distance_to_boss == 0 && query_asteroid.is_empty() {
//         let mut boss = Mesh::new(PrimitiveTopology::TriangleList);
//         let vertices_position = triangles_from_polygon(&POLYGON, Vec3::ZERO)
//             .into_iter()
//             .map(|x| x.to_array())
//             .collect::<Vec<_>>();
//         // let vertices_position = TRIANGLES
//         //     .iter()
//         //     .flatten()
//         //     .map(|x| x.to_array())
//         //     .collect::<Vec<_>>();
//         let mut vertices_normal = Vec::new();
//         let mut vertices_uv = Vec::new();
//         for _ in &vertices_position {
//             vertices_normal.push([0.0, 0.0, 1.0]);
//             vertices_uv.push([0.0, 0.0]);
//         }

//         boss.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
//         boss.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
//         boss.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

//         commands
//             .spawn_empty()
//             .insert(Boss)
//             .insert(Health(HEALTH))
//             .insert(Velocity(Vec3::ZERO))
//             .insert(Surface {
//                 topology: Topology::Triangles(&TRIANGLES),
//                 hitbox: HitBox {
//                     half_x: OUTER_RADIUS,
//                     half_y: OUTER_RADIUS,
//                 },
//             })
//             .insert(MaterialMesh2dBundle {
//                 mesh: meshes.add(boss).into(),
//                 transform: Transform::from_translation(INITIAL_POSITION),
//                 material: materials.add(COLOR.into()),
//                 ..default()
//             });

//         level.boss.spawn_emptyed = true;
//     }
// }

pub fn move_boss(mut query: Query<(&mut Transform, &mut Velocity), With<Boss>>) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut rng = rand::thread_rng();
        let mut acceleration = Vec::new();
        if transform.translation.x < WINDOW_WIDTH / 3.0 {
            acceleration.push(Direction::Left);
        }
        if transform.translation.x > -WINDOW_WIDTH / 3.0 {
            acceleration.push(Direction::Right);
        }
        if transform.translation.y < WINDOW_HEIGHT / 3.0 {
            acceleration.push(Direction::Up);
        }
        if transform.translation.y > -WINDOW_HEIGHT / 3.0 {
            acceleration.push(Direction::Down);
        }

        velocity.0 += match acceleration.choose(&mut rng).unwrap() {
            Direction::Left => Vec3::from([ACCELERATION, 0.0, 0.0]),
            Direction::Right => Vec3::from([-ACCELERATION, 0.0, 0.0]),
            Direction::Up => Vec3::from([0.0, ACCELERATION, 0.0]),
            Direction::Down => Vec3::from([0.0, -ACCELERATION, 0.0]),
            // _ => unreachable!(),
        };
        transform.translation += velocity.0;
        transform.rotation *= Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), ROTATION_SPEED);
    }
}

pub fn attack_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss: Query<(Entity, &Transform), With<Boss>>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
) {
    if let Ok((b_entity, b_transform)) = query_boss.get_single() {
        if let Ok(s_transform) = query_spaceship.get_single() {
            let mut rng = rand::thread_rng();
            for canon_relative_position in ATTACK_SOURCE {
                if rng.gen_range(0..100) == 0 {
                    let canon_absolute_position = b_transform.translation
                        + b_transform.rotation.mul_vec3(canon_relative_position);
                    // + Vec3::from([0.0, 0.0, 1.0]);

                    // Compute coordinates of vector from boss to spaceship
                    let vec_boss_spaceship = s_transform.translation - b_transform.translation;
                    // Compute coordinates of vector from boss to canon
                    let vec_boss_center_canon = canon_absolute_position - b_transform.translation;
                    let scalar_product = vec_boss_spaceship.x * vec_boss_center_canon.x
                        + vec_boss_spaceship.y * vec_boss_center_canon.y;
                    // Scalar product sign determines whether or not canon has line of sight
                    if scalar_product < 0.0 {
                        continue;
                    }

                    let blast = commands
                        .spawn_empty()
                        .insert(Blast)
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: BLAST_RADIUS,
                                    vertices: BLAST_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(canon_relative_position),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(b_entity).add_child(blast);

                    commands
                        .spawn_empty()
                        .insert(Fire {
                            color: ATTACK_COLOR,
                            impact_radius: IMPACT_RADIUS,
                            impact_vertices: IMPACT_VERTICES,
                        })
                        .insert(Health(1))
                        .insert(Enemy)
                        .insert(Velocity(
                            (s_transform.translation - canon_absolute_position).normalize()
                                * FIRE_VELOCITY,
                        ))
                        .insert(Surface {
                            topology: Topology::Point,
                            hitbox: HitBox {
                                half_x: 0.0,
                                half_y: 0.0,
                            },
                        })
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: FIRE_RADIUS,
                                    vertices: FIRE_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(canon_absolute_position),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        });
                }
            }
        }
    }
}

pub fn attack_boss_parts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss: Query<&Transform, With<Boss>>,
    query_boss_part: Query<(&Attack, Entity, &Transform), With<BossPart>>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
) {
    if let Ok(b_transform) = query_boss.get_single() {
        if let Ok(s_transform) = query_spaceship.get_single() {
            for (bp_attack, bp_entity, bp_transform) in query_boss_part.iter() {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..100) == 0 {
                    // let canon_absolute_position =
                    //     b_transform.translation + b_transform.rotation.mul_vec3(bp_attack.0);
                    let canon_absolute_position =
                        b_transform.transform_point(bp_transform.transform_point(bp_attack.0));

                    // Compute coordinates of vector from boss to spaceship
                    let vec_boss_spaceship = s_transform.translation - b_transform.translation;
                    // Compute coordinates of vector from boss to canon
                    let vec_boss_center_canon = canon_absolute_position - b_transform.translation;
                    let scalar_product = vec_boss_spaceship.x * vec_boss_center_canon.x
                        + vec_boss_spaceship.y * vec_boss_center_canon.y;
                    // Scalar product sign determines whether or not canon has line of sight
                    if scalar_product < 0.0 {
                        continue;
                    }

                    commands
                        .spawn_empty()
                        .insert(Blast)
                        .insert(Health(2))
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: BLAST_RADIUS,
                                    vertices: BLAST_VERTICES,
                                }))
                                .into(),
                            // transform: Transform::from_translation(bp_attack.0),
                            transform: Transform::from_translation(canon_absolute_position),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        });

                    // commands.entity(bp_entity).add_child(blast);

                    commands
                        .spawn_empty()
                        .insert(Fire {
                            color: ATTACK_COLOR,
                            impact_radius: IMPACT_RADIUS,
                            impact_vertices: IMPACT_VERTICES,
                        })
                        .insert(Health(1))
                        .insert(Enemy)
                        .insert(Velocity(
                            (s_transform.translation - canon_absolute_position).normalize()
                                * FIRE_VELOCITY,
                        ))
                        .insert(Surface {
                            topology: Topology::Point,
                            hitbox: HitBox {
                                half_x: 0.0,
                                half_y: 0.0,
                            },
                        })
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: FIRE_RADIUS,
                                    vertices: FIRE_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(canon_absolute_position),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        });
                }
            }
        }
    }
}

pub fn explode(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // transform: &GlobalTransform,
    // velocity: &Velocity,
    query: Query<(Entity, &Health, &GlobalTransform), With<BossPart>>,
) {
    for (bp_entity, bp_health, bp_transform) in query.iter() {
        if bp_health.0 > 0 {
            continue;
        }

        // commands.entity(bp_entity).despawn();

        // let mut rng = rand::thread_rng();
        // for _ in 1..100 {
        //     let mut debris;
        //     'outer: loop {
        //         let rho = rng.gen_range(0.0..OUTER_RADIUS);
        //         let theta = rng.gen_range(0.0..2.0 * PI);
        //         debris = Vec3 {
        //             x: rho * theta.cos(),
        //             y: rho * theta.sin(),
        //             z: 0.0,
        //         };
        //         let triangles = triangles_from_polygon(&POLYGON, Vec3::ZERO);
        //         let mut iter_triangles = triangles.chunks(3);
        //         while let Some(&[a, b, c]) = iter_triangles.next() {
        //             if math::point_in_triangle(
        //                 debris.truncate(),
        //                 a.truncate(),
        //                 b.truncate(),
        //                 c.truncate(),
        //             ) {
        //                 break 'outer;
        //             }
        //         }
        //     }
        //     debris.z += ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        //     let debris_translation = transform.translation + debris * transform.scale;
        //     let dv = Vec3 {
        //         x: rng.gen_range(-0.5..0.5),
        //         y: rng.gen_range(-0.5..0.5),
        //         z: 0.0,
        //     };

        //     commands
        //         .spawn_empty()
        //         .insert(Debris)
        //         .insert(Velocity(velocity.0 + dv))
        //         .insert(MaterialMesh2dBundle {
        //             mesh: meshes
        //                 .add(Mesh::from(shape::Circle {
        //                     radius: 20.0,
        //                     vertices: 8,
        //                 }))
        //                 .into(),
        //             transform: Transform::from_translation(debris_translation),
        //             material: materials.add(COLOR.into()),
        //             ..default()
        //         });
        // }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<BossPart>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
