use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};
use rand::Rng;
use std::f32::consts::{PI, SQRT_2};

use crate::{
    blast::BlastEvent,
    collision::detection::{triangle::Triangle, Aabb, Collider, Topology},
    component::{
        AngularVelocity, Attack, ColorDamaged, Health, Indestructible, Mass, MomentOfInertia, Part,
        Velocity,
    },
    constant::WINDOW_Z,
    fire::{Fire, FireEvent},
    spaceship::{self, Spaceship},
};

const INNER_RADIUS: f32 = 100.0;
const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;
const COLOR: Color = Color::rgb(0.25, 0.5, 0.25);

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossPart;

#[derive(Component)]
pub struct BossCore;

#[derive(Component)]
pub struct BossEdge;

const A1: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
const A3: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A5: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: -INNER_RADIUS,
    z: 0.0,
};
const A7: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: INNER_RADIUS - OUTER_RADIUS,
    z: 0.0,
};
const A9: Vec3 = Vec3 {
    x: INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};
const A11: Vec3 = Vec3 {
    x: OUTER_RADIUS - INNER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A13: Vec3 = Vec3 {
    x: INNER_RADIUS - OUTER_RADIUS,
    y: INNER_RADIUS,
    z: 0.0,
};
const A15: Vec3 = Vec3 {
    x: -INNER_RADIUS,
    y: OUTER_RADIUS - INNER_RADIUS,
    z: 0.0,
};

// The body is a collection of 6 triangles. It is a single part of the boss.
const CORE_TRIANGLES: [Triangle; 6] = [
    Triangle(A1, A3, A15),
    Triangle(A3, A13, A15),
    Triangle(A3, A11, A13),
    Triangle(A3, A5, A11),
    Triangle(A5, A9, A11),
    Triangle(A5, A7, A9),
];

// There are 8 egdes.
// Each edge is a triangle and constitutes a whole part of the boss.
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

pub fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..2.0 * PI);
    const DISTANCE_TO_BOSS: f32 = 10000.0;
    let x = DISTANCE_TO_BOSS * theta.cos() + spaceship::TRANSLATION.x;
    let y = DISTANCE_TO_BOSS * theta.sin() + spaceship::TRANSLATION.y;
    let translation = Vec3::new(x, y, WINDOW_Z);
    const AREA: f32 =
        PI * (INNER_RADIUS + OUTER_RADIUS) / 2.0 * (INNER_RADIUS + OUTER_RADIUS) / 2.0;
    const MASS: f32 = AREA;
    const MOMENT_OF_INERTIA: f32 =
        0.5 * MASS * (INNER_RADIUS + OUTER_RADIUS) / 2.0 * (INNER_RADIUS + OUTER_RADIUS) / 2.0;

    let boss = commands
        .spawn(Boss)
        .insert(Mass(MASS))
        .insert(MomentOfInertia(MOMENT_OF_INERTIA))
        .insert(Velocity(Vec3::ZERO))
        .insert(AngularVelocity(0.0))
        .insert(SpatialBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .id();

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
    const CORE_HEALTH: u32 = 50;

    let boss_core = commands
        .spawn((Boss, Part))
        .insert(BossCore)
        .insert(Health(CORE_HEALTH))
        .insert(Indestructible)
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
    const EDGES: usize = 8;
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
        const EDGE_HEALTH: u32 = 15;

        let boss_edge = commands
            .spawn((Boss, Part))
            .insert(BossEdge)
            .insert(Health(EDGE_HEALTH))
            .insert(ColorDamaged(Color::GRAY))
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
}

pub fn movement(
    mut query_boss: Query<(&mut AngularVelocity, &mut Transform, &mut Velocity), With<Boss>>,
    query_boss_edge: Query<With<BossEdge>>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>, Without<Boss>)>,
    time: Res<Time>,
) {
    if let Ok((mut angular_velocity, mut b_transform, mut velocity)) = query_boss.get_single_mut() {
        const ACCELERATION: f32 = 500.0;
        const ROTATION_SPEED: f32 = 20.0;

        if let Ok(s_transform) = query_spaceship.get_single() {
            if !query_boss_edge.is_empty() {
                let mut direction = (s_transform.translation - b_transform.translation).normalize();
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
            angular_velocity.0 -= ROTATION_SPEED * time.delta_seconds();
        }

        const DRAG: f32 = 0.05;
        velocity.0 *= 1.0 - DRAG;
        const ANGULAR_DRAG: f32 = 0.25;
        angular_velocity.0 *= 1.0 - ANGULAR_DRAG;

        b_transform.translation += velocity.0 * time.delta_seconds();
        b_transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time.delta_seconds());
    }
}

pub fn attack(
    mut blast_event: EventWriter<BlastEvent>,
    mut fire_event: EventWriter<FireEvent>,
    query_boss: Query<&Transform, (With<Boss>, Without<Part>)>,
    query_boss_edge: Query<(&Attack, &Transform), With<Boss>>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>)>,
) {
    if let Ok(b_transform) = query_boss.get_single() {
        if let Ok(s_transform) = query_spaceship.get_single() {
            for (bp_attack, bp_transform) in query_boss_edge.iter() {
                let mut rng = rand::thread_rng();
                const ATTACK_RATE: usize = 10;
                if rng.gen_range(0..ATTACK_RATE) == 0 {
                    let attack_absolute_translation =
                        b_transform.transform_point(bp_transform.transform_point(bp_attack.0));

                    let bs = s_transform.translation - b_transform.translation;
                    let bc = attack_absolute_translation - b_transform.translation;
                    if bs.truncate().angle_between(bc.truncate()).abs() > PI / 6.0 {
                        continue;
                    }

                    const BLAST_RADIUS: f32 = 15.0;
                    const BLAST_VERTICES: usize = 32;
                    const ATTACK_COLOR: Color = Color::RED;

                    blast_event.send(BlastEvent {
                        radius: BLAST_RADIUS,
                        vertices: BLAST_VERTICES,
                        color: ATTACK_COLOR,
                        translation: attack_absolute_translation,
                    });

                    const FIRE_IMPACT_RADIUS: f32 = 15.0;
                    const FIRE_IMPACT_VERTICES: usize = 32;
                    const FIRE_DAMAGES: u32 = 1;
                    const FIRE_RADIUS: f32 = 5.0 / FIRE_RANGE as f32;
                    const FIRE_VERTICES: usize = 32;
                    const FIRE_RANGE: u32 = 100;
                    const FIRE_VELOCITY: f32 = 400.0;

                    fire_event.send(FireEvent {
                        fire: Fire {
                            impact_radius: FIRE_IMPACT_RADIUS,
                            impact_vertices: FIRE_IMPACT_VERTICES,
                        },
                        enemy: true,
                        damages: FIRE_DAMAGES,
                        radius: FIRE_RADIUS,
                        vertices: FIRE_VERTICES,
                        color: ATTACK_COLOR,
                        range: FIRE_RANGE as f32,
                        translation: attack_absolute_translation,
                        velocity: Velocity(
                            (s_transform.translation - attack_absolute_translation).normalize()
                                * FIRE_VELOCITY,
                        ),
                    });
                }
            }
        }
    }
}

pub fn lone_core(
    mut commands: Commands,
    query_core: Query<Entity, With<BossCore>>,
    query_edge: Query<With<BossEdge>>,
) {
    if let Ok(core) = query_core.get_single() {
        if query_edge.is_empty() {
            commands.entity(core).remove::<Indestructible>();
        }
    }
}
