use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};
use rand::Rng;
use std::f32::consts::{PI, SQRT_2};

use crate::{
    blast::Blast,
    collision::{cache::Cache, detection::triangle::Triangle, Aabb, Collider, Topology},
    fire::{Enemy, Fire},
    spaceship::{self, Spaceship},
    AngularVelocity, Health, Mass, MomentOfInertia, Velocity, PLANE_Z,
};

pub const BOSS_Z: f32 = PLANE_Z;
pub const DISTANCE_TO_BOSS: f32 = 1000.0;
const INNER_RADIUS: f32 = 100.0;
const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;
const AREA: f32 = PI * (INNER_RADIUS + OUTER_RADIUS) / 2.0 * (INNER_RADIUS + OUTER_RADIUS) / 2.0;
// const MASS: f32 = 20.0;
const MASS: f32 = AREA;
const MOMENT_OF_INERTIA: f32 =
    0.5 * MASS * (INNER_RADIUS + OUTER_RADIUS) / 2.0 * (INNER_RADIUS + OUTER_RADIUS) / 2.0;

// #[derive(Component)]
// pub struct Boss;

#[derive(Component)]
pub struct Boss {
    pub edges: usize,
}

#[derive(Component)]
pub struct BossPart;

#[derive(Component)]
pub struct BossCore;

#[derive(Component)]
pub struct BossEdge;

// const A0: Vec3 = Vec3 {
//     x: -OUTER_RADIUS,
//     y: 0.0,
//     z: 0.0,
// };
const A1: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
// const A2: Vec3 = Vec3 {
//     x: -INNER_RADIUS,
//     y: -INNER_RADIUS,
//     z: 0.0,
// };
const A3: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
// const A4: Vec3 = Vec3 {
//     x: 0.0,
//     y: -OUTER_RADIUS,
//     z: 0.0,
// };
const A5: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
// const A6: Vec3 = Vec3 {
//     x: INNER_RADIUS,
//     y: -INNER_RADIUS,
//     z: 0.0,
// };
const A7: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
// const A8: Vec3 = Vec3 {
//     x: OUTER_RADIUS,
//     y: 0.0,
//     z: 0.0,
// };
const A9: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};
// const A10: Vec3 = Vec3 {
//     x: INNER_RADIUS,
//     y: INNER_RADIUS,
//     z: 0.0,
// };
const A11: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
// const A12: Vec3 = Vec3 {
//     x: 0.0,
//     y: OUTER_RADIUS,
//     z: 0.0,
// };
const A13: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
// const A14: Vec3 = Vec3 {
//     x: -INNER_RADIUS,
//     y: INNER_RADIUS,
//     z: 0.0,
// };
const A15: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};

// /// Counter clockwise
// pub const POLYGON: [Vec3; 16] = [
//     A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15,
// ];

const ACCELERATION: f32 = 500.0;
const ANGULAR_DRAG: f32 = 0.25;
const DRAG: f32 = 0.05;
const ATTACK_COLOR: Color = Color::RED;
const BLAST_RADIUS: f32 = 15.0;
const BLAST_VERTICES: usize = 32;
const COLOR: Color = Color::rgb(0.25, 0.5, 0.25);
const CORE_HEALTH: i32 = 50;
const EDGE_HEALTH: i32 = 10;
const FIRE_VELOCITY: f32 = 400.0;
const FIRE_RADIUS: f32 = 5.0 / FIRE_RANGE as f32;
const FIRE_RANGE: u32 = 100;
const FIRE_VERTICES: usize = 32;
const FIRE_HEALTH: i32 = 100;
const FIRE_IMPACT_RADIUS: f32 = 15.0;
const FIRE_IMPACT_VERTICES: usize = 32;

// const INITIAL_POSITION: Vec3 = Vec3 {
//     // x: WINDOW_WIDTH / 2.0 + OUTER_RADIUS,
//     x: WINDOW_WIDTH / 2.0,
//     y: 0.0,
//     z: BOSS_Z,
// };
const ROTATION_SPEED: f32 = 0.0;
// const ROTATION_SPEED: f32 = 20.0;

#[derive(Component)]
pub struct Attack(Vec3);

// The body is a collection of 6 triangles. It is a single part of the boss.
const CORE_PARTS: usize = 6;
const CORE_TRIANGLES: [Triangle; CORE_PARTS] = [
    Triangle(A1, A3, A15),
    Triangle(A3, A13, A15),
    Triangle(A3, A11, A13),
    Triangle(A3, A5, A11),
    Triangle(A5, A9, A11),
    Triangle(A5, A7, A9),
];

// There are 8 egdes.
// Each edge is a triangle and constitutes a whole part of the boss.
const EDGES: usize = 8;
const E1: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: 0.0,
    z: 0.0,
};
const E2: Vec3 = Vec3 {
    x: 0.0,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};
const E3: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: 0.0,
    z: 0.0,
};
const EDGE_TRIANGLES: [Triangle; 1] = [Triangle(E1, E2, E3)];

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

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // query: Query<&Compass>,
    // query_asteroid: Query<With<Asteroid>>,
) {
    // Compute boss starting position
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..2.0 * PI);
    let x = DISTANCE_TO_BOSS * theta.cos() + spaceship::POSITION.x;
    let y = DISTANCE_TO_BOSS * theta.sin() + spaceship::POSITION.y;
    let translation = Vec3::new(x, y, BOSS_Z);

    let boss = commands
        .spawn(Boss { edges: EDGES })
        .insert(Mass(MASS))
        .insert(Velocity(Vec3::ZERO))
        .insert(MomentOfInertia(MOMENT_OF_INERTIA))
        .insert(AngularVelocity(0.0))
        .insert(SpatialBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .id();

    // let translation = query.single().target;
    // if !level.boss_spawned && level.distance_to_boss == 0 && query_asteroid.is_empty() {

    // Build core
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices_position: Vec<[f32; 3]> = CORE_TRIANGLES
        .iter()
        .flat_map(|triangle| triangle.to_array())
        .map(|vertex| vertex.to_array())
        .collect();
    // let vertices_normal = vec![[0.0, 0.0, 1.0]; 3 * CORE_PARTS];
    // let vertices_uv = vec![[0.0, 0.0]; 3 * CORE_PARTS];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);
    let mesh_handle = meshes.add(mesh);

    // println!(
    //     "boss\narea: {}\nmass: {}\nmoment of inertia: {}\n",
    //     AREA, MASS, MOMENT_OF_INERTIA
    // );

    let boss_core = commands
        // .spawn(Boss)
        .spawn(BossCore)
        .insert(BossPart)
        .insert(Health(CORE_HEALTH))
        .insert(Collider {
            aabb: Aabb {
                hw: 108.3, // sqrt(100^2 + (100sqrt(2) - 100)^2)
                hh: 108.3,
            },
            topology: Topology::Triangles {
                mesh_handle: Mesh2dHandle(mesh_handle.clone_weak()),
            },
        })
        .insert(ColorMesh2dBundle {
            mesh: mesh_handle.into(),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    commands.entity(boss).add_child(boss_core);

    // Add the edges
    for i in 0..EDGES {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices_position: Vec<[f32; 3]> = EDGE_TRIANGLES
            .iter()
            .flat_map(|triangle| triangle.to_array())
            .map(|vertex| vertex.to_array())
            .collect();

        // vec![E1.to_array(), E2.to_array(), E3.to_array()];
        // let vertices_normal = vec![[0.0, 0.0, 1.0]; 3];
        // let vertices_uv = vec![[0.0, 0.0]; 3];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);
        let mesh_handle = meshes.add(mesh);

        let boss_edge = commands
            // .spawn(Boss)
            .spawn(BossEdge)
            .insert(BossPart)
            .insert(Health(EDGE_HEALTH))
            .insert(Collider {
                aabb: Aabb {
                    hw: OUTER_RADIUS - INNER_RADIUS,
                    hh: OUTER_RADIUS - INNER_RADIUS,
                },
                topology: Topology::Triangles {
                    mesh_handle: Mesh2dHandle(mesh_handle.clone_weak()),
                },
            })
            .insert(ColorMesh2dBundle {
                mesh: mesh_handle.into(),
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
            .insert(Attack(E2))
            .id();

        commands.entity(boss).add_child(boss_edge);
    }
    // level.boss_spawned = true;
    // }
}

pub fn movement(
    mut query_boss: Query<(&mut AngularVelocity, &Boss, &mut Transform, &mut Velocity)>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Boss>)>,
    cache: Res<Cache>,
    query_boss_entity: Query<Entity, Or<(With<BossCore>, With<BossEdge>)>>,
    time: Res<Time>,
) {
    if let Ok((mut angular_velocity, boss, mut b_transform, mut velocity)) =
        query_boss.get_single_mut()
    {
        'no_collision: {
            for id in &query_boss_entity {
                if cache.contains_entity(id) {
                    break 'no_collision;
                }
            }

            if let Ok(s_transform) = query_spaceship.get_single() {
                if boss.edges > 0 {
                    let mut direction =
                        (s_transform.translation - b_transform.translation).normalize();
                    let mut rng = rand::thread_rng();
                    let angle = rng.gen_range(-PI / 2.0..PI / 2.0);
                    direction = Quat::from_axis_angle(Vec3::Z, angle) * direction;
                    velocity.0 += ACCELERATION * time.delta_seconds() * direction;
                    angular_velocity.0 += ROTATION_SPEED * time.delta_seconds();
                } else {
                    let direction = (s_transform.translation - b_transform.translation).normalize();
                    velocity.0 += 2.0 * ACCELERATION * time.delta_seconds() * direction;
                    angular_velocity.0 += 2.0 * ROTATION_SPEED * time.delta_seconds();
                }
            } else {
                // velocity.0 += Vec3::ZERO;
                angular_velocity.0 -= ROTATION_SPEED * time.delta_seconds();
            }

            velocity.0 *= 1.0 - DRAG;
            angular_velocity.0 *= 1.0 - ANGULAR_DRAG;
        }

        b_transform.translation += velocity.0 * time.delta_seconds();
        b_transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time.delta_seconds());
    }
}

pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss: Query<&Transform, With<Boss>>,
    query_boss_edge: Query<(&Attack, Entity, &Transform), With<BossEdge>>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
) {
    if let Ok(b_transform) = query_boss.get_single() {
        if let Ok(s_transform) = query_spaceship.get_single() {
            for (bp_attack, bp_entity, bp_transform) in query_boss_edge.iter() {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..100) == 0 {
                    let attack_absolute_translation =
                        b_transform.transform_point(bp_transform.transform_point(bp_attack.0));

                    // Compute coordinates of vector from boss to spaceship
                    let bs = s_transform.translation - b_transform.translation;
                    // Compute coordinates of vector from boss to attack source
                    let bc = attack_absolute_translation - b_transform.translation;
                    // Scalar product sign determines whether or not attack has line of sight
                    if bs.truncate().dot(bc.truncate()) < 0.0 {
                        continue;
                    }

                    let blast = commands
                        .spawn(Blast)
                        .insert(Health(1))
                        .insert(ColorMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: BLAST_RADIUS,
                                    vertices: BLAST_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(bp_attack.0),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(bp_entity).add_child(blast);

                    commands
                        .spawn(Fire {
                            impact_radius: FIRE_IMPACT_RADIUS,
                            impact_vertices: FIRE_IMPACT_VERTICES,
                        })
                        .insert(Enemy)
                        .insert(Health(FIRE_HEALTH))
                        .insert(Mass(1.0))
                        .insert(MomentOfInertia(1.0))
                        .insert(Velocity(
                            (s_transform.translation - attack_absolute_translation).normalize()
                                * FIRE_VELOCITY,
                        ))
                        .insert(AngularVelocity(1.0))
                        // .insert(Topology::Point)
                        .insert(Collider {
                            aabb: Aabb { hw: 0.0, hh: 0.0 },
                            topology: Topology::Point,
                        })
                        .insert(ColorMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: FIRE_RADIUS,
                                    vertices: FIRE_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(attack_absolute_translation)
                                .with_scale(Vec3::new(FIRE_RANGE as f32, FIRE_RANGE as f32, 0.0)),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        });
                }
            }
        }
    }
}

// pub fn before_despawn(
//     mut commands: Commands,
//     mut query_boss_part: Query<
//         (
//             // Option<&BossEdge>,
// 	    &BossEdge,
//             Option<&Children>,
//             Entity,
//             // &GlobalTransform,
//             &mut Transform,
//             &Health,
//         ),
//         // Or<(With<BossCore>, With<BossEdge>)>,
//     >,
//     // mut query_blast_impact: Query<&mut Transform, Or<(With<Blast>, With<Impact>)>>,
//     mut query_boss_core: Query<(&mut BossCore, Entity, &Transform, &Velocity)>,
// ) {
//     if let Ok((mut core, core_id, core_transform, core_velocity)) = query_boss_core.get_single_mut()
//     {
//         for (maybe_edge, maybe_children, id, mut transform, health) in query_boss_part.iter_mut() {
//             if health.0 > 0 {
//                 continue;
//             }

//             if maybe_edge.is_some() {
//                 core.edges -= 1;
//                 commands.entity(core_id).remove_children(&[id]);
//                 commands.entity(id).insert(*core_velocity);
//                 transform.translation = core_transform.transform_point(transform.translation);
//             }

//             // if let Some(children) = maybe_children {
//             //     for child in children {
//             //         if let Ok(mut child_transform) =
//             //             query_blast_impact.get_component_mut::<Transform>(*child)
//             //         {
//             //             commands.entity(id).remove_children(&[*child]);
//             //             commands.entity(*child).remove::<Parent>();
//             //             child_transform.translation =
//             //                 transform.transform_point(child_transform.translation);
//             //         }
//             //     }
//             // }
//         }
//     }
// }

pub fn cut_off_edge(
    mut commands: Commands,
    mut query_boss_edge: Query<(Entity, &Health, &mut Transform), With<BossEdge>>,
    mut query_boss: Query<
        (&AngularVelocity, &mut Boss, Entity, &Transform, &Velocity),
        Without<BossEdge>,
    >,
) {
    if let Ok((b_angular_velocity, mut boss, b_id, b_transform, b_velocity)) =
        query_boss.get_single_mut()
    {
        for (e_id, e_health, mut e_transform) in query_boss_edge.iter_mut() {
            if e_health.0 > 0 {
                continue;
            }

            boss.edges -= 1;
            commands.entity(b_id).remove_children(&[e_id]);
            let e_translation_relative_b = b_transform.rotation * e_transform.translation;
            let e_tangential_velocity =
                (Vec3::new(0.0, 0.0, b_angular_velocity.0)).cross(e_translation_relative_b);
            // println!(
            //     "boss angular velocity: {}\n, edge position: {}\n, tangential velocity: {}\n",
            //     b_angular_velocity.0, e_translation_relative_b, e_tangential_velocity
            // );
            commands
                .entity(e_id)
                .insert(Velocity(b_velocity.0 + e_tangential_velocity));
            e_transform.translation = b_transform.translation + e_translation_relative_b;
            e_transform.rotation *= b_transform.rotation;
            // if let Some(children) = maybe_children {
            //     for child in children {
            //         if let Ok(mut child_transform) =
            //             query_blast_impact.get_component_mut::<Transform>(*child)
            //         {
            //             commands.entity(id).remove_children(&[*child]);
            //             commands.entity(*child).remove::<Parent>();
            //             child_transform.translation =
            //                 transform.transform_point(child_transform.translation);
            //         }
            //     }
            // }
        }
    }
}

// pub fn wreck(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     query_boss_part: Query<
//         (
//             Option<&BossCore>,
//             &Handle<ColorMaterial>,
//             &GlobalTransform,
//             &Health,
//         ),
//         Or<(With<BossCore>, With<BossEdge>)>,
//     >,
//     mut query_boss_core: Query<&Velocity, With<BossCore>>,
// ) {
//     if let Ok(core_velocity) = query_boss_core.get_single_mut() {
//         for (maybe_core, color, transform, health) in query_boss_part.iter() {
//             if health.0 > 0 {
//                 continue;
//             }

//             let color = materials.get(color).unwrap().color;
//             let mut rng = rand::thread_rng();
//             let triangles = if maybe_core.is_some() {
//                 CORE_TRIANGLES.iter()
//             } else {
//                 EDGE_TRIANGLES.iter()
//             };

//             for triangle in triangles {
//                 // Arbitrary number of debris per triangle : area/16
//                 for _ in 0..(triangle.area() / 16.0).round() as usize {
//                     let p = triangle.xy().random_point();
//                     let debris_relative =
//                         Vec3::new(p.x, p.y, if rng.gen_bool(0.5) { 1.0 } else { -1.0 });
//                     let debris = transform.transform_point(debris_relative);
//                     let dv = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

//                     commands
//                         .spawn(Debris)
//                         .insert(Velocity(core_velocity.0 + dv))
//                         .insert(ColorMesh2dBundle {
//                             mesh: meshes
//                                 .add(Mesh::from(shape::Circle {
//                                     radius: rng.gen_range(2.0..15.0),
//                                     vertices: 8,
//                                 }))
//                                 .into(),
//                             transform: Transform::from_translation(debris),
//                             material: materials.add(color.into()),
//                             ..default()
//                         });
//                 }
//             }
//         }
//     }
// }

// pub fn despawn(
//     mut commands: Commands,
//     query: Query<(Entity, &Health), Or<(With<BossCore>, With<BossEdge>)>>,
// ) {
//     for (entity, health) in query.iter() {
//         if health.0 <= 0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }
