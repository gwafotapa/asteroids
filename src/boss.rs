use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::{PI, SQRT_2};

use crate::{
    asteroid::Asteroid,
    blast::Blast,
    collision::{math, HitBox, Impact, Surface, Topology, Triangle},
    spaceship::Spaceship,
    Debris, Direction, Enemy, Fire, Health, Level, Velocity, PLANE_Z, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const INNER_RADIUS: f32 = 100.0;
const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;

#[derive(Component)]
pub struct BossCore {
    pub edges: usize,
}

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

const ACCELERATION: f32 = 0.01;
const ATTACK_COLOR: Color = Color::RED;
const BLAST_RADIUS: f32 = 15.0;
const BLAST_VERTICES: usize = 32;
const COLOR: Color = Color::rgb(0.25, 0.5, 0.25);
const CORE_HEALTH: i32 = 50;
const EDGE_HEALTH: i32 = 10;
const FIRE_VELOCITY: f32 = 8.0;
const FIRE_RADIUS: f32 = 5.0;
const FIRE_VERTICES: usize = 32;
const IMPACT_RADIUS: f32 = 15.0;
const IMPACT_VERTICES: usize = 32;
const INITIAL_POSITION: Vec3 = Vec3 {
    // x: WINDOW_WIDTH / 2.0 + OUTER_RADIUS,
    x: WINDOW_WIDTH / 2.0,
    y: 0.0,
    z: PLANE_Z,
};
// const ROTATION_SPEED: f32 = 0.05;
const ROTATION_SPEED: f32 = 0.0;

#[derive(Component)]
pub struct Attack(Vec3);

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
const EDGE: [Triangle; 1] = [[E1, E2, E3]];

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
    mut query_level: Query<&mut Level>,
    query_asteroid: Query<With<Asteroid>>,
) {
    let mut level = query_level.single_mut();
    if !level.boss_spawned && level.distance_to_boss == 0 && query_asteroid.is_empty() {
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

        let boss_core = commands
            .spawn_empty()
            .insert(BossCore { edges: EDGES })
            .insert(Health(CORE_HEALTH))
            .insert(Velocity(Vec3::ZERO))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform: Transform::from_translation(INITIAL_POSITION),

                material: materials.add(COLOR.into()),
                ..default()
            })
            .insert(Surface {
                topology: Topology::Triangles(&CORE_TRIANGLES),
                hitbox: HitBox {
                    half_x: 108.3, // sqrt(100^2 + (100sqrt(2) - 100)^2)
                    half_y: 108.3,
                },
            })
            .id();

        // Add the edges
        for i in 0..EDGES {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            let vertices_position = vec![E1.to_array(), E2.to_array(), E3.to_array()];
            let vertices_normal = vec![[0.0, 0.0, 1.0]; 3];
            let vertices_uv = vec![[0.0, 0.0]; 3];

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

            let boss_edge = commands
                .spawn_empty()
                .insert(BossEdge)
                .insert(Health(EDGE_HEALTH))
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
                    topology: Topology::Triangles(&EDGE),
                    hitbox: HitBox {
                        half_x: OUTER_RADIUS - INNER_RADIUS,
                        half_y: OUTER_RADIUS - INNER_RADIUS,
                    },
                })
                .insert(Attack(E2))
                .id();

            commands.entity(boss_core).add_child(boss_edge);
        }
        level.boss_spawned = true;
    }
}

pub fn advance(
    mut query_boss: Query<(&BossCore, &mut Transform, &mut Velocity)>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<BossCore>)>,
) {
    if let Ok((core, mut transform, mut velocity)) = query_boss.get_single_mut() {
        if core.edges > 0 {
            let mut rng = rand::thread_rng();
            let mut acceleration = Vec::new();

            if velocity.0.x > -1.0 && transform.translation.x > -WINDOW_WIDTH / 4.0 {
                acceleration.push(Direction::Left);
            }
            if velocity.0.x < 1.0 && transform.translation.x < WINDOW_WIDTH / 4.0 {
                acceleration.push(Direction::Right);
            }
            if velocity.0.y > -1.0 && transform.translation.y > -WINDOW_HEIGHT / 3.0 {
                acceleration.push(Direction::Down);
            }
            if velocity.0.y < 1.0 && transform.translation.y < WINDOW_HEIGHT / 3.0 {
                acceleration.push(Direction::Up);
            }

            velocity.0 += match acceleration.choose(&mut rng).unwrap() {
                Direction::Left => Vec3::from([-ACCELERATION, 0.0, 0.0]),
                Direction::Right => Vec3::from([ACCELERATION, 0.0, 0.0]),
                Direction::Down => Vec3::from([0.0, -ACCELERATION, 0.0]),
                Direction::Up => Vec3::from([0.0, ACCELERATION, 0.0]),
            };
            transform.rotation *=
                Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), ROTATION_SPEED);
        } else {
            if let Ok(s_transform) = query_spaceship.get_single() {
                let direction = (s_transform.translation - transform.translation).normalize();
                let acceleration = -velocity.0 / 2.0 + 3.0 * direction;
                velocity.0 += acceleration;
                transform.rotation *=
                    Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), 2.0 * ROTATION_SPEED);
            }
        }

        transform.translation += velocity.0;

        // Don't move out of the screen
        if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            transform.translation.x = -WINDOW_WIDTH / 2.0;
        }
        if transform.translation.x > WINDOW_WIDTH / 2.0 {
            transform.translation.x = WINDOW_WIDTH / 2.0;
        }
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 {
            transform.translation.y = -WINDOW_HEIGHT / 2.0;
        }
        if transform.translation.y > WINDOW_HEIGHT / 2.0 {
            transform.translation.y = WINDOW_HEIGHT / 2.0;
        }
    }
}

pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss_core: Query<&Transform, With<BossCore>>,
    query_boss_edge: Query<(&Attack, Entity, &Transform), With<BossEdge>>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
) {
    if let Ok(b_transform) = query_boss_core.get_single() {
        if let Ok(s_transform) = query_spaceship.get_single() {
            for (bp_attack, bp_entity, bp_transform) in query_boss_edge.iter() {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..100000) == 0 {
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

                    let blast = commands
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
                            transform: Transform::from_translation(bp_attack.0),
                            material: materials.add(ATTACK_COLOR.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(bp_entity).add_child(blast);

                    commands
                        .spawn_empty()
                        .insert(Fire {
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss_part: Query<
        (
            Option<&BossEdge>,
            Option<&Children>,
            Entity,
            &Handle<ColorMaterial>,
            &GlobalTransform,
            &Health,
            &Surface,
        ),
        Or<(With<BossCore>, With<BossEdge>)>,
    >,
    mut query_blast_impact: Query<&mut Transform, Or<(With<Blast>, With<Impact>)>>,
    mut query_boss_core: Query<(&mut BossCore, Entity, &Velocity)>,
) {
    if let Ok((mut core, core_entity, core_velocity)) = query_boss_core.get_single_mut() {
        for (maybe_edge, maybe_children, entity, color, transform, health, surface) in
            query_boss_part.iter()
        {
            if health.0 > 0 {
                continue;
            }

            if maybe_edge.is_some() {
                core.edges -= 1;
                commands.entity(core_entity).remove_children(&[entity]);
            }

            if let Some(children) = maybe_children {
                for child in children {
                    commands.entity(*child).remove::<Parent>();
                    if let Ok(mut child_transform) =
                        query_blast_impact.get_component_mut::<Transform>(*child)
                    {
                        child_transform.translation =
                            transform.transform_point(child_transform.translation);
                    }
                }
            }

            let color = materials.get(color).unwrap().color;
            let mut rng = rand::thread_rng();

            if let Topology::Triangles(triangles) = surface.topology {
                let mut triangles = triangles.iter();
                while let Some(&[a, b, c]) = triangles.next() {
                    for _ in 0..10 {
                        let mut debris;
                        'outer: loop {
                            debris = Vec3 {
                                x: rng.gen_range(-surface.hitbox.half_x..surface.hitbox.half_x),
                                y: rng.gen_range(-surface.hitbox.half_y..surface.hitbox.half_y),
                                z: 0.0,
                            };
                            if math::point_in_triangle(
                                debris.truncate(),
                                a.truncate(),
                                b.truncate(),
                                c.truncate(),
                            ) {
                                break 'outer;
                            }
                        }
                        debris.z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

                        let dv = Vec3 {
                            x: rng.gen_range(-0.5..0.5),
                            y: rng.gen_range(-0.5..0.5),
                            z: 0.0,
                        };

                        commands
                            .spawn_empty()
                            .insert(Debris)
                            .insert(Velocity(core_velocity.0 + dv))
                            // .insert(Velocity(dv))
                            .insert(MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(Mesh::from(shape::Circle {
                                        radius: rng.gen_range(2.0..15.0),
                                        vertices: 8,
                                    }))
                                    .into(),
                                transform: Transform::from_translation(
                                    transform.transform_point(debris),
                                ),
                                material: materials.add(color.into()),
                                ..default()
                            });
                    }
                }
            } else {
                panic!("Boss topology should be triangles.");
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Health), Or<(With<BossCore>, With<BossEdge>)>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
