use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};
use std::f32::consts::PI;

use crate::{
    blast::BlastEvent,
    collision::{detection::triangle::Triangle, Aabb, Collider, Topology},
    fire::{Fire, FireEvent},
    keyboard::KeyboardBindings,
    AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity, WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub mod flame;

pub const HEALTH: u32 = 100;
const AREA: f32 = (S2.x - S4.x) * S4.y + (S6.x - S8.x) * S8.y + (S10.x - S11.x) * (S10.y - S9.y)
    - (S6.x + 13.0) * 12.0; // looking at assets/spaceship.ggb
const MASS: f32 = AREA;
// const MASS: f32 = 1.0;
const MOMENT_OF_INERTIA: f32 = 0.5 * MASS * AREA / PI;
pub const TRANSLATION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0,
    y: WINDOW_HEIGHT / 2.0,
    z: Z,
};
pub const Z: f32 = 500.0;

const ACCELERATION: f32 = 500.0;
const ANGULAR_DRAG: f32 = 0.1;
const ATTACK_COLOR: Color = Color::YELLOW;
const ATTACK_SOURCE: Vec3 = S2;
const BLAST_RADIUS: f32 = 8.0;
const BLAST_VERTICES: usize = 8;
const COLOR: Color = Color::BLUE;
const DRAG: f32 = 0.01;
const FIRE_DAMAGES: u32 = 1;
const FIRE_IMPACT_RADIUS: f32 = 12.0;
const FIRE_IMPACT_VERTICES: usize = 16;
const FIRE_RADIUS: f32 = 3.0 / FIRE_RANGE as f32;
const FIRE_RANGE: u32 = 20;
const FIRE_VELOCITY: Vec3 = Vec3 {
    x: 1200.0,
    y: 0.0,
    z: 0.0,
};
const FIRE_VERTICES: usize = 4;
const ROTATION_SPEED: f32 = 20.0;

// Center of gravity of the spaceship
// const SG: Vec3 = Vec3 {
//     x: -11.0 + 4.0,
//     y: 0.0,
//     z: 0.0,
// };
const S1: Vec3 = Vec3 {
    x: -26.0,
    y: -30.0,
    z: 0.0,
};
const S2: Vec3 = Vec3 {
    x: 34.0,
    y: 0.0,
    z: 0.0,
};
const S3: Vec3 = Vec3 {
    x: -16.0,
    y: 0.0,
    z: 0.0,
};
const S4: Vec3 = Vec3 {
    x: -26.0,
    y: 30.0,
    z: 0.0,
};
const S5: Vec3 = Vec3 {
    x: -36.0,
    y: -20.0,
    z: 0.0,
};
const S6: Vec3 = Vec3 {
    x: 15.0,
    y: 0.0,
    z: 0.0,
};
const S7: Vec3 = Vec3 {
    x: -26.0,
    y: 0.0,
    z: 0.0,
};
const S8: Vec3 = Vec3 {
    x: -36.0,
    y: 20.0,
    z: 0.0,
};
const S9: Vec3 = Vec3 {
    x: 1.0,
    y: 16.0,
    z: 0.0,
};
const S10: Vec3 = Vec3 {
    x: 1.0,
    y: 22.0,
    z: 0.0,
};
const S11: Vec3 = Vec3 {
    x: -11.0,
    y: 22.0,
    z: 0.0,
};
const S12: Vec3 = Vec3 {
    x: -11.0,
    y: -22.0,
    z: 0.0,
};
const S13: Vec3 = Vec3 {
    x: 1.0,
    y: -22.0,
    z: 0.0,
};
const S14: Vec3 = Vec3 {
    x: 1.0,
    y: -16.0,
    z: 0.0,
};
pub const TRIANGLES: [Triangle; 6] = [
    Triangle(S1, S2, S3),
    Triangle(S4, S3, S2),
    Triangle(S5, S6, S7),
    Triangle(S8, S7, S6),
    Triangle(S9, S10, S11),
    Triangle(S12, S13, S14),
];
const AABB: Aabb = Aabb { hw: S2.x, hh: S4.y };
// pub const ENVELOP: [Vec3; 7] = [E, A, B, D, G, MIDPOINT_AB, MIDPOINT_DB];
// const TRIANGLELIST: [[f32; 3]; 6] = [
//     [40.0, -5.0, 0.0],
//     [-20.0, 15.0, 0.0],
//     [-40.0, -25.0, 0.0],
//     [10.0, -5.0, 0.0],
//     [-30.0, 25.0, 0.0],
//     [-30.0, -5.0, 0.0],
// ];

// const ENVELOP: [[f32; 3]; 6] = [
//     [40.0, -5.0, 0.0],
//     [-30.0, 25.0, 0.0],
//     [-40.0, -25.0, 0.0],
//     [-30.0, -5.0, 0.0],
//     [-5.0, 10.0, 0.0],
//     [0.0, -15.0, 0.0],
// ];

#[derive(Component)]
pub struct Spaceship;

// impl Spaceship {
//     pub fn accelerate(velocity: &mut Velocity) {

//         velocity.0 +=
//     }

//     pub fn decelerate_x(velocity: &mut Velocity) {
//         if velocity.0.x > 0.0 {
//             velocity.0.x -= ACCELERATION / 2.0;
//         } else if velocity.0.x < 0.0 {
//             velocity.0.x += ACCELERATION / 2.0;
//         }
//     }

//     pub fn decelerate_y(velocity: &mut Velocity) {
//         if velocity.0.y > 0.0 {
//             velocity.0.y -= ACCELERATION / 2.0;
//         } else if velocity.0.y < 0.0 {
//             velocity.0.y += ACCELERATION / 2.0;
//         }
//     }

//     pub fn accelerate(transform: &Transform, velocity: &mut Velocity) {
//         let direction = transform.rotation * Vec3::X;
//         velocity.0 += ACCELERATION * direction;
//     }

//     pub fn decelerate(transform: &Transform, velocity: &mut Velocity) {
//         let direction = transform.rotation * Vec3::NEG_X;
//         velocity.0 += 0.5 * ACCELERATION * direction;
//     }
// }

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let v_pos: Vec<[f32; 3]> = TRIANGLES
        .iter()
        .flat_map(|triangle| triangle.to_array())
        .map(|vertex| vertex.to_array())
        .collect();
    // let v_normals = vec![[0.0, 0.0, 1.0]; 12];
    // let v_uvs = vec![[1.0, 1.0]; 12];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    // spaceship.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normals);
    // spaceship.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uvs);

    // let mut v_color: Vec<u32> = vec![Color::BLUE.as_linear_rgba_u32()];
    // v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 2]);
    // spaceship.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 10, VertexFormat::Uint32),
    //     v_color,
    // );

    // let indices = vec![0, 1, 2, 3, 4, 5];
    // spaceship.set_indices(Some(Indices::U32(indices)));

    let mesh_handle = meshes.add(mesh);
    // println!(
    //     "spaceship\narea: {}\nmass: {}\nmoment of inertia: {}\n",
    //     AREA, MASS, MOMENT_OF_INERTIA
    // );

    let spaceship = commands
        .spawn(Spaceship)
        .insert(Mass(MASS))
        .insert(MomentOfInertia(MOMENT_OF_INERTIA))
        .insert(Velocity(Vec3::ZERO))
        .insert(AngularVelocity(0.0))
        .insert(SpatialBundle {
            transform: Transform::from_translation(TRANSLATION),
            ..Default::default()
        })
        .id();

    let spaceship_part = commands
        .spawn((Spaceship, Part))
        .insert(Health(HEALTH))
        .insert(Collider {
            aabb: AABB,
            topology: Topology::Triangles {
                mesh_handle: Mesh2dHandle(mesh_handle.clone_weak()),
            },
        })
        // .insert(Attack {
        //     source: ATTACK_SOURCE,
        //     color: ATTACK_COLOR,
        //     blast_radius: BLAST_RADIUS,
        //     blast_vertices: BLAST_VERTICES,
        //     fire_radius: FIRE_RADIUS,
        //     fire_vertices: FIRE_VERTICES,
        // })
        .insert(ColorMesh2dBundle {
            // mesh: Mesh2dHandle(meshes.add(spaceship)),
            mesh: mesh_handle.into(),
            // material: materials.add(Color::rgb(0.25, 0., 1.).into()),
            material: materials.add(COLOR.into()),
            ..Default::default()
        })
        .id();

    commands.entity(spaceship).add_child(spaceship_part);
}

pub fn attack(
    mut blast_event: EventWriter<BlastEvent>,
    mut fire_event: EventWriter<FireEvent>,
    keys: Res<Input<KeyCode>>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>)>,
    query_bindings: Query<&KeyboardBindings>,
) {
    if !keys.just_pressed(query_bindings.single().fire()) {
        return;
    }

    if let Ok(transform) = query_spaceship.get_single() {
        blast_event.send(BlastEvent {
            radius: BLAST_RADIUS,
            vertices: BLAST_VERTICES,
            color: ATTACK_COLOR,
            translation: transform.translation + transform.rotation * ATTACK_SOURCE,
        });

        fire_event.send(FireEvent {
            fire: Fire {
                impact_radius: FIRE_IMPACT_RADIUS,
                impact_vertices: FIRE_IMPACT_VERTICES,
            },
            enemy: false,
            damages: FIRE_DAMAGES,
            radius: FIRE_RADIUS,
            vertices: FIRE_VERTICES,
            color: ATTACK_COLOR,
            range: FIRE_RANGE as f32,
            translation: transform.translation + transform.rotation * ATTACK_SOURCE,
            velocity: Velocity(transform.rotation * FIRE_VELOCITY),
        });
    }
}

// pub fn before_despawn(
//     mut commands: Commands,
//     query_spaceship: Query<(Option<&Children>, &Health, &Transform), With<Spaceship>>,
//     mut query_blast_impact: Query<
//         &mut Transform,
//         (Or<(With<Blast>, With<Impact>)>, Without<Spaceship>),
//     >,
// ) {
//     if let Ok((s_children, s_health, s_transform)) = query_spaceship.get_single() {
//         if s_health.0 > 0 {
//             return;
//         }

//         if let Some(children) = s_children {
//             for child in children {
//                 if let Ok(mut child_transform) =
//                     query_blast_impact.get_component_mut::<Transform>(*child)
//                 {
//                     commands.entity(*child).remove::<Parent>();
//                     child_transform.translation =
//                         s_transform.transform_point(child_transform.translation);
//                 }
//             }
//         }
//     }
// }

// pub fn wreck(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     query: Query<(&Handle<ColorMaterial>, &Transform, &Health, &Velocity), With<Spaceship>>,
// ) {
//     if let Ok((s_color, s_transform, s_health, s_velocity)) = query.get_single() {
//         if s_health.0 > 0 {
//             return;
//         }

//         let color = materials.get(s_color).unwrap().color;
//         let mut rng = rand::thread_rng();

//         for triangle in TRIANGLES {
//             // Arbitrary number of debris per triangle : area/16
//             for _ in 0..(triangle.area() / 16.0).round() as usize {
//                 let p = triangle.xy().random_point();
//                 let debris_relative =
//                     Vec3::new(p.x, p.y, if rng.gen_bool(0.5) { 1.0 } else { -1.0 });
//                 let debris = s_transform.transform_point(debris_relative);
//                 let dv = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

//                 commands
//                     .spawn(Debris)
//                     .insert(Velocity(s_velocity.0 + dv))
//                     .insert(ColorMesh2dBundle {
//                         mesh: meshes
//                             .add(Mesh::from(shape::Circle {
//                                 radius: rng.gen_range(1.0..10.0),
//                                 vertices: 4 * rng.gen_range(1..5),
//                             }))
//                             .into(),
//                         transform: Transform::from_translation(debris),
//                         material: materials.add(color.into()),
//                         ..default()
//                     });
//             }
//         }
//     }
// }

// pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Spaceship>>) {
//     for (entity, health) in query.iter() {
//         if health.0 <= 0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

pub fn movement(
    mut query_spaceship: Query<
        (&mut AngularVelocity, &mut Transform, &mut Velocity),
        (With<Spaceship>, Without<Part>),
    >,
    keys: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
    time: Res<Time>,
) {
    let bindings = query_bindings.single();

    if let Ok((mut s_angular_velocity, mut s_transform, mut s_velocity)) =
        query_spaceship.get_single_mut()
    {
        if keys.any_pressed([bindings.rotate_left(), KeyCode::Left]) {
            s_angular_velocity.0 += ROTATION_SPEED * time.delta_seconds();
            // s_angular_velocity.0 =
            // ROTATION_SPEED_MAX.min(s_angular_velocity.0 + ROTATION_SPEED);
        } else if keys.any_pressed([bindings.rotate_right(), KeyCode::Right]) {
            s_angular_velocity.0 -= ROTATION_SPEED * time.delta_seconds();
            // s_angular_velocity.0 =
            //     -ROTATION_SPEED_MAX.max(s_angular_velocity.0 - ROTATION_SPEED);
        }

        if keys.any_pressed([bindings.accelerate(), KeyCode::Up]) {
            s_velocity.0 += ACCELERATION * time.delta_seconds() * (s_transform.rotation * Vec3::X);
        } else if keys.any_pressed([bindings.decelerate(), KeyCode::Down]) {
            s_velocity.0 +=
                0.5 * ACCELERATION * time.delta_seconds() * (s_transform.rotation * Vec3::NEG_X);
        }

        s_velocity.0 *= 1.0 - DRAG;
        s_angular_velocity.0 *= 1.0 - ANGULAR_DRAG;
        // debug!("Spaceship velocity: {}", s_velocity.0);

        s_transform.translation += s_velocity.0 * time.delta_seconds();
        s_transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, s_angular_velocity.0 * time.delta_seconds());

        // println!(
        //     "spaceship velocity: {}, angular velocity: {}",
        //     s_velocity.0, s_angular_velocity.0
        // );
    }
}
