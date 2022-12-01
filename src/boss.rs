use bevy::{
    prelude::*,
    render::mesh::{PrimitiveTopology, VertexAttributeValues},
    sprite::Mesh2dHandle,
};
use rand::Rng;
use std::f32::consts::{PI, SQRT_2};

use crate::{
    // asteroid::Asteroid,
    blast::Blast,
    collision::{impact::Impact, math, HitBox},
    // compass::Compass,
    debris::Debris,
    fire::Fire,
    spaceship::{self, Spaceship},
    Enemy,
    Health,
    Velocity,
    PLANE_Z,
    // WINDOW_HEIGHT,
    // WINDOW_WIDTH,
};

pub const BOSS_Z: f32 = PLANE_Z;
pub const DISTANCE_TO_BOSS: f32 = 500.0;
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

const ACCELERATION: f32 = 0.15;
const DRAG: f32 = 0.05;
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
// const INITIAL_POSITION: Vec3 = Vec3 {
//     // x: WINDOW_WIDTH / 2.0 + OUTER_RADIUS,
//     x: WINDOW_WIDTH / 2.0,
//     y: 0.0,
//     z: BOSS_Z,
// };
// const ROTATION_SPEED: f32 = 0.05;
const ROTATION_SPEED: f32 = 0.01;

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
// const EDGE: [Triangle; 1] = [[E1, E2, E3]];

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

    // let translation = query.single().target;
    // if !level.boss_spawned && level.distance_to_boss == 0 && query_asteroid.is_empty() {

    // Build core
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices_position: Vec<[f32; 3]> = CORE_TRIANGLES
        .iter()
        .flatten()
        .map(|x| x.to_array())
        .collect();
    // let vertices_normal = vec![[0.0, 0.0, 1.0]; 3 * CORE_PARTS];
    // let vertices_uv = vec![[0.0, 0.0]; 3 * CORE_PARTS];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

    let boss_core = commands
        .spawn(BossCore { edges: EDGES })
        .insert(Health(CORE_HEALTH))
        .insert(Velocity(Vec3::ZERO))
        .insert(ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_translation(translation),
            material: materials.add(COLOR.into()),
            ..default()
        })
        // .insert(Topology::Triangles)
        .insert(HitBox {
            half_x: 108.3, // sqrt(100^2 + (100sqrt(2) - 100)^2)
            half_y: 108.3,
        })
        .id();

    // Add the edges
    for i in 0..EDGES {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices_position = vec![E1.to_array(), E2.to_array(), E3.to_array()];
        // let vertices_normal = vec![[0.0, 0.0, 1.0]; 3];
        // let vertices_uv = vec![[0.0, 0.0]; 3];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

        let boss_edge = commands
            .spawn(BossEdge)
            .insert(Health(EDGE_HEALTH))
            .insert(ColorMesh2dBundle {
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
            // .insert(Topology::Triangles)
            .insert(HitBox {
                half_x: OUTER_RADIUS - INNER_RADIUS,
                half_y: OUTER_RADIUS - INNER_RADIUS,
            })
            .insert(Attack(E2))
            .id();

        commands.entity(boss_core).add_child(boss_edge);
    }
    // level.boss_spawned = true;
    // }
}

pub fn advance(
    mut query_boss: Query<(&BossCore, &mut Transform, &mut Velocity)>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<BossCore>)>,
) {
    if let Ok((boss, mut b_transform, mut velocity)) = query_boss.get_single_mut() {
        let (acceleration, rotation_speed);
        if let Ok(s_transform) = query_spaceship.get_single() {
            if boss.edges > 0 {
                let mut direction = (s_transform.translation - b_transform.translation).normalize();
                let mut rng = rand::thread_rng();
                let angle = rng.gen_range(-PI / 2.0..PI / 2.0);
                direction = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle) * direction;
                acceleration = ACCELERATION * direction;
                rotation_speed = ROTATION_SPEED;
            } else {
                let direction = (s_transform.translation - b_transform.translation).normalize();
                acceleration = 2.0 * ACCELERATION * direction;
                rotation_speed = 2.0 * ROTATION_SPEED;
            }
        } else {
            acceleration = Vec3::ZERO;
            rotation_speed = ROTATION_SPEED;
        }

        velocity.0 += acceleration;
        velocity.0 *= 1.0 - DRAG;
        b_transform.translation += velocity.0;
        b_transform.rotation *= Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), rotation_speed);
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
                        .insert(Health(2))
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
                            impact_radius: IMPACT_RADIUS,
                            impact_vertices: IMPACT_VERTICES,
                        })
                        .insert(Health(1))
                        .insert(Enemy)
                        .insert(Velocity(
                            (s_transform.translation - attack_absolute_translation).normalize()
                                * FIRE_VELOCITY,
                        ))
                        // .insert(Topology::Point)
                        .insert(HitBox {
                            half_x: 0.0,
                            half_y: 0.0,
                        })
                        .insert(ColorMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: FIRE_RADIUS,
                                    vertices: FIRE_VERTICES,
                                }))
                                .into(),
                            transform: Transform::from_translation(attack_absolute_translation),
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
            &HitBox,
            &Mesh2dHandle,
        ),
        Or<(With<BossCore>, With<BossEdge>)>,
    >,
    mut query_blast_impact: Query<&mut Transform, Or<(With<Blast>, With<Impact>)>>,
    mut query_boss_core: Query<(&mut BossCore, Entity, &Velocity)>,
) {
    if let Ok((mut core, core_entity, core_velocity)) = query_boss_core.get_single_mut() {
        for (maybe_edge, maybe_children, entity, color, transform, health, hitbox, mesh) in
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
            let mut debris: Vec<(Vec3, Vec3)> = Vec::new();

            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .get(&mesh.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                for triangle in vertices.chunks(3) {
                    for _ in 0..10 {
                        let mut debris_translation;
                        'outer: loop {
                            debris_translation = Vec3 {
                                x: rng.gen_range(-hitbox.half_x..hitbox.half_x),
                                y: rng.gen_range(-hitbox.half_y..hitbox.half_y),
                                z: 0.0,
                            };
                            if math::point_in_triangle(
                                debris_translation.truncate(),
                                Vec3::from(triangle[0]).truncate(),
                                Vec3::from(triangle[1]).truncate(),
                                Vec3::from(triangle[2]).truncate(),
                            ) {
                                break 'outer;
                            }
                        }
                        debris_translation.z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

                        let debris_velocity = Vec3 {
                            x: rng.gen_range(-0.5..0.5),
                            y: rng.gen_range(-0.5..0.5),
                            z: 0.0,
                        };

                        debris.push((debris_translation, debris_velocity));
                    }
                }
            } else {
                panic!("Cannot find boss' mesh to create debris.");
            }

            for (translation, velocity) in debris {
                commands
                    .spawn(Debris)
                    .insert(Velocity(core_velocity.0 + velocity))
                    // .insert(Velocity(dv))
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: rng.gen_range(2.0..15.0),
                                vertices: 8,
                            }))
                            .into(),
                        transform: Transform::from_translation(
                            transform.transform_point(translation),
                        ),
                        material: materials.add(color.into()),
                        ..default()
                    });
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
