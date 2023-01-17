use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use rand::Rng;
use std::f32::consts::{PI, SQRT_2};

use crate::{
    blast::BlastEvent,
    boss::Attack,
    collision::{cache::Cache, detection::triangle::Triangle, Aabb, Collider, Topology},
    fire::{Fire, FireEvent},
    spaceship::{self, Spaceship},
    AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity, PLANE_Z, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

const SQRT_3: f32 = 1.73205080756887729352744634151;

const CORE_RADIUS: f32 = 10.0;
const CORE_VERTICES: usize = 32;
const CORE_AREA: f32 = PI * CORE_RADIUS * CORE_RADIUS;
const WING_EDGE: f32 = 25.0;
const WING_HEIGHT: f32 = WING_EDGE * SQRT_3 / 2.0;
const WING_AREA: f32 = WING_EDGE * WING_HEIGHT / 2.0; // area of an equilateral triangle of edge 15.0
const AREA: f32 = CORE_AREA + 2.0 * WING_AREA;
const MASS: f32 = AREA;
const MOMENT_OF_INERTIA: f32 = 0.5 * MASS * AREA / PI;

const ACCELERATION: f32 = 300.0;
// const ANGULAR_DRAG: f32 = 0.25;
const ANGULAR_DRAG: f32 = 0.25;
const ATTACK_COLOR: Color = Color::RED;
const BLAST_RADIUS: f32 = 5.0;
const BLAST_VERTICES: usize = 16;
const COLOR: Color = Color::rgb(0.25, 1.0, 0.25);
const DRAG: f32 = 0.05;
const HEALTH: u32 = 3;
const FIRE_DAMAGES: u32 = 1;
const FIRE_RADIUS: f32 = 5.0 / FIRE_RANGE as f32;
const FIRE_RANGE: u32 = 100;
const FIRE_VELOCITY: f32 = 400.0;
const FIRE_VERTICES: usize = 32;
const FIRE_IMPACT_RADIUS: f32 = 15.0;
const FIRE_IMPACT_VERTICES: usize = 32;
const ROTATION_SPEED: f32 = 50.0;

const A1: Vec3 = Vec3::ZERO;
const A2: Vec3 = Vec3 {
    x: -WING_HEIGHT,
    y: WING_EDGE / 2.0,
    z: 0.0,
};
const A3: Vec3 = Vec3 {
    x: -WING_HEIGHT,
    y: -WING_EDGE / 2.0,
    z: 0.0,
};

const B1: Vec3 = Vec3::ZERO;
const B2: Vec3 = Vec3 {
    x: WING_HEIGHT,
    y: -WING_EDGE / 2.0,
    z: 0.0,
};
const B3: Vec3 = Vec3 {
    x: WING_HEIGHT,
    y: WING_EDGE / 2.0,
    z: 0.0,
};

const TRIANGLES: [Triangle; 2] = [Triangle(A1, A2, A3), Triangle(B1, B2, B3)];

#[derive(Component)]
pub struct Intercepter;

#[derive(Component)]
pub struct IntercepterPart;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_camera: Query<&Transform, With<Camera>>,
) {
    let mut rng = rand::thread_rng();
    let Vec3 { x: xc, y: yc, z: _ } = query_camera.single().translation;
    if rng.gen_range(0..10) != 0 {
        return;
    }

    let phi = rng.gen_range(0.0..2.0 * PI);
    let translation = Vec3::new(
        xc + 2.0 * WINDOW_WIDTH * phi.cos(),
        yc + 2.0 * WINDOW_WIDTH * phi.sin(),
        PLANE_Z,
    );
    println!("{}", translation);
    let intercepter = commands
        .spawn(Intercepter)
        .insert(Mass(MASS))
        .insert(MomentOfInertia(MOMENT_OF_INERTIA))
        .insert(Velocity(Vec3::ZERO))
        .insert(AngularVelocity(0.0))
        .insert(SpatialBundle {
            transform: Transform::from_translation(translation),
            // transform: Transform::from_translation(Vec3::new(xc + 100.0, yc, 500.0)),
            ..Default::default()
        })
        .id();

    let mut positions = Vec::with_capacity(CORE_VERTICES + 2 * 3);
    // let mut normals = Vec::with_capacity(CORE_VERTICES + 2 * 3);
    // let mut uvs = Vec::with_capacity(CORE_VERTICES + 2 * 3);

    let step = std::f32::consts::TAU / CORE_VERTICES as f32;
    for i in 0..CORE_VERTICES {
        let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
        let (sin, cos) = theta.sin_cos();

        positions.push([cos * CORE_RADIUS, sin * CORE_RADIUS, 0.0]);
        // normals.push([0.0, 0.0, 1.0]);
        // uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
    }

    let mut indices = Vec::with_capacity((CORE_VERTICES - 2) * 3 + 2);
    for i in 1..(CORE_VERTICES as u32 - 1) {
        indices.extend_from_slice(&[0, i + 1, i]);
    }

    positions.extend(
        TRIANGLES
            .iter()
            .flat_map(|triangle| triangle.to_array())
            .map(|vec3| vec3.to_array()),
    );

    indices.extend_from_slice(&[
        CORE_VERTICES as u32,
        CORE_VERTICES as u32 + 1,
        CORE_VERTICES as u32 + 2,
    ]);
    indices.extend_from_slice(&[
        CORE_VERTICES as u32 + 3,
        CORE_VERTICES as u32 + 4,
        CORE_VERTICES as u32 + 5,
    ]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));

    let intercepter_part = commands
        .spawn((Intercepter, Part))
        .insert(Health(HEALTH))
        .insert(Collider {
            aabb: Aabb {
                hw: CORE_RADIUS,
                hh: CORE_RADIUS,
            },
            topology: Topology::Disk {
                radius: CORE_RADIUS,
            },
        })
        .insert(ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(COLOR.into()),
            ..Default::default()
        })
        .id();

    commands.entity(intercepter).add_child(intercepter_part);
}

pub fn movement(
    mut query_intercepter: Query<
        (&mut AngularVelocity, &mut Transform, &mut Velocity),
        With<Intercepter>,
    >,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>, Without<Intercepter>)>,
    // cache: Res<Cache>,
    time: Res<Time>,
) {
    for (mut angular_velocity, mut i_transform, mut velocity) in query_intercepter.iter_mut() {
        // if !cache.contains_entity(id) {
        if let Ok(s_transform) = query_spaceship.get_single() {
            let mut direction = (s_transform.translation - i_transform.translation).normalize();
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(-PI / 2.0..PI / 2.0);
            direction = Quat::from_axis_angle(Vec3::Z, angle) * direction;
            velocity.0 += ACCELERATION * time.delta_seconds() * direction;
            // angular_velocity.0 += ROTATION_SPEED * time.delta_seconds();
            // let direction = (s_transform.translation - i_transform.translation).normalize();
            // velocity.0 += 2.0 * ACCELERATION * time.delta_seconds() * direction;
            // angular_velocity.0 += 2.0 * ROTATION_SPEED * time.delta_seconds();

            let looking_at =
                (i_transform.rotation * Quat::from_axis_angle(Vec3::Z, PI / 2.0) * Vec3::X)
                    .truncate()
                    .normalize();

            let should_look_at = (s_transform.translation - i_transform.translation)
                .truncate()
                .normalize();

            let should_rotate = Quat::from_rotation_arc_2d(looking_at, should_look_at);

            angular_velocity.0 += if should_rotate.to_axis_angle().0.z > 0.0 {
                ROTATION_SPEED
            } else {
                -ROTATION_SPEED
            } * time.delta_seconds();
        };
        // else {
        // velocity.0 += Vec3::ZERO;
        // angular_velocity.0 -= ROTATION_SPEED * time.delta_seconds();
        // }

        velocity.0 *= 1.0 - DRAG;
        angular_velocity.0 *= 1.0 - ANGULAR_DRAG;
        // }

        i_transform.translation += velocity.0 * time.delta_seconds();
        i_transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time.delta_seconds());
    }
}

// pub fn attack(
//     mut blast_event: EventWriter<BlastEvent>,
//     mut fire_event: EventWriter<FireEvent>,
//     query_boss: Query<&Transform, (With<Boss>, Without<Part>)>,
//     query_boss_edge: Query<(&Attack, &Transform), With<Boss>>,
//     query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>)>,
// ) {
//     if let Ok(b_transform) = query_boss.get_single() {
//         if let Ok(s_transform) = query_spaceship.get_single() {
//             for (bp_attack, bp_transform) in query_boss_edge.iter() {
//                 let mut rng = rand::thread_rng();
//                 if rng.gen_range(0..10) == 0 {
//                     let attack_absolute_translation =
//                         b_transform.transform_point(bp_transform.transform_point(bp_attack.0));

//                     // Compute coordinates of vector from boss to spaceship
//                     let bs = s_transform.translation - b_transform.translation;
//                     // Compute coordinates of vector from boss to attack source
//                     let bc = attack_absolute_translation - b_transform.translation;
//                     // Scalar product sign determines whether or not attack has line of sight
//                     // if bs.truncate().dot(bc.truncate()) < 0.0 {
//                     if bs.truncate().angle_between(bc.truncate()).abs() > PI / 6.0 {
//                         continue;
//                     }

//                     blast_event.send(BlastEvent {
//                         radius: BLAST_RADIUS,
//                         vertices: BLAST_VERTICES,
//                         color: ATTACK_COLOR,
//                         translation: attack_absolute_translation,
//                     });

//                     fire_event.send(FireEvent {
//                         fire: Fire {
//                             impact_radius: FIRE_IMPACT_RADIUS,
//                             impact_vertices: FIRE_IMPACT_VERTICES,
//                         },
//                         enemy: true,
//                         damages: FIRE_DAMAGES,
//                         radius: FIRE_RADIUS,
//                         vertices: FIRE_VERTICES,
//                         color: ATTACK_COLOR,
//                         range: FIRE_RANGE as f32,
//                         translation: attack_absolute_translation,
//                         velocity: Velocity(
//                             (s_transform.translation - attack_absolute_translation).normalize()
//                                 * FIRE_VELOCITY,
//                         ),
//                     });
//                 }
//             }
//         }
//     }
// }
