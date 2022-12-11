use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};
use rand::Rng;

use crate::{
    blast::Blast,
    collision::{impact::Impact, math::triangle::Triangle, Aabb, Collider, Topology},
    debris::Debris,
    fire::Fire,
    Health, Velocity, WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub mod flame;

const HEALTH: i32 = 10;
pub const SPACESHIP_Z: f32 = 500.0;

// Center of gravity of the spaceship
const SG: Vec3 = Vec3 {
    x: -11.0,
    y: 0.0,
    z: 0.0,
};
const S1: Vec3 = Vec3 {
    x: -30.0 - SG.x,
    y: -30.0 - SG.y,
    z: 0.0 - SG.z,
};
const S2: Vec3 = Vec3 {
    x: 30.0 - SG.x,
    y: 0.0 - SG.y,
    z: 0.0 - SG.z,
};
const S3: Vec3 = Vec3 {
    x: -20.0 - SG.x,
    y: 0.0 - SG.y,
    z: 0.0 - SG.z,
};
const S4: Vec3 = Vec3 {
    x: -30.0 - SG.x,
    y: 30.0 - SG.y,
    z: 0.0 - SG.z,
};
const S5: Vec3 = Vec3 {
    x: -40.0 - SG.x,
    y: -20.0 - SG.y,
    z: 0.0 - SG.z,
};
const S6: Vec3 = Vec3 {
    x: -SG.x,
    y: 0.0 - SG.y,
    z: 0.0 - SG.z,
};
const S7: Vec3 = Vec3 {
    x: -30.0 - SG.x,
    y: 0.0 - SG.y,
    z: 0.0 - SG.z,
};
const S8: Vec3 = Vec3 {
    x: -40.0 - SG.x,
    y: 20.0 - SG.y,
    z: 0.0 - SG.z,
};
const S9: Vec3 = Vec3 {
    x: 8.0,
    y: 16.0,
    z: 0.0 - SG.z,
};
const S10: Vec3 = Vec3 {
    x: 8.0,
    y: 22.0,
    z: 0.0 - SG.z,
};
const S11: Vec3 = Vec3 {
    x: -4.0,
    y: 22.0,
    z: 0.0 - SG.z,
};
const S12: Vec3 = Vec3 {
    x: -4.0,
    y: -22.0,
    z: 0.0 - SG.z,
};
const S13: Vec3 = Vec3 {
    x: 8.0,
    y: -22.0,
    z: 0.0 - SG.z,
};
const S14: Vec3 = Vec3 {
    x: 8.0,
    y: -16.0,
    z: 0.0 - SG.z,
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

// const VELOCITY_MAX: f32 = 5.0;
pub const ACCELERATION: f32 = 0.1;
pub const DRAG: f32 = 0.01;
pub const POSITION: Vec3 = Vec3 {
    // x: -WINDOW_WIDTH / 4.0,
    // x: -WINDOW_WIDTH / 2.0,
    // x: MAP_CENTER_X,
    // y: MAP_CENTER_Y,
    // x: 0.0,
    // y: 0.0,
    x: WINDOW_WIDTH / 2.0,
    y: WINDOW_HEIGHT / 2.0,
    // y: -crate::WINDOW_HEIGHT,
    z: SPACESHIP_Z,
};
const ATTACK_SOURCE: Vec3 = S2;
const SPACESHIP_COLOR: Color = Color::BLUE;
const ATTACK_COLOR: Color = Color::YELLOW;
const BLAST_RADIUS: f32 = 8.0;
const BLAST_VERTICES: usize = 8;
const FIRE_RADIUS: f32 = 3.0;
const FIRE_VERTICES: usize = 4;
const FIRE_HEALTH: i32 = 20;
const FIRE_IMPACT_RADIUS: f32 = 12.0;
const FIRE_IMPACT_VERTICES: usize = 16;
const FIRE_VELOCITY: Vec3 = Vec3 {
    x: 20.0,
    y: 0.0,
    z: 0.0,
};
pub const SPEED_MAX: f32 = 12.0;

#[derive(Component)]
pub struct Spaceship;

impl Spaceship {
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
    pub fn accelerate(transform: &Transform, velocity: &mut Velocity) {
        let direction = transform.rotation * Vec3::X;
        velocity.0 += ACCELERATION * direction;
    }

    pub fn decelerate(transform: &Transform, velocity: &mut Velocity) {
        let direction = transform.rotation * Vec3::NEG_X;
        velocity.0 += 0.5 * ACCELERATION * direction;
    }
}

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

    commands
        .spawn(Spaceship)
        .insert(Health(HEALTH))
        .insert(Velocity(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }))
        // .insert(AABB)
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
            transform: Transform::from_translation(POSITION),
            // material: materials.add(Color::rgb(0.25, 0., 1.).into()),
            material: materials.add(SPACESHIP_COLOR.into()),
            ..default()
        });
}

pub fn attack(
    keys: Res<Input<KeyCode>>,
    query: Query<(Entity, &Transform), With<Spaceship>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keys.just_pressed(KeyCode::R) {
        return;
    }

    if let Ok((spaceship, transform)) = query.get_single() {
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
                transform: Transform::from_translation(ATTACK_SOURCE + Vec3::new(0.0, 0.0, 1.0)),
                material: materials.add(ATTACK_COLOR.into()),
                ..default()
            })
            .id();

        commands.entity(spaceship).add_child(blast);

        commands
            .spawn(Fire {
                scale_down: 1.0 / FIRE_HEALTH as f32,
                impact_radius: FIRE_IMPACT_RADIUS,
                impact_vertices: FIRE_IMPACT_VERTICES,
            })
            .insert(Health(FIRE_HEALTH))
            .insert(Velocity(transform.rotation * FIRE_VELOCITY))
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
                transform: Transform::from_translation(
                    transform.translation + transform.rotation * ATTACK_SOURCE,
                ),
                material: materials.add(ATTACK_COLOR.into()),
                ..default()
            });
    }
}

pub fn before_despawn(
    mut commands: Commands,
    query_spaceship: Query<(Option<&Children>, &Health, &Transform), With<Spaceship>>,
    mut query_blast_impact: Query<
        &mut Transform,
        (Or<(With<Blast>, With<Impact>)>, Without<Spaceship>),
    >,
) {
    if let Ok((s_children, s_health, s_transform)) = query_spaceship.get_single() {
        if s_health.0 > 0 {
            return;
        }

        if let Some(children) = s_children {
            for child in children {
                if let Ok(mut child_transform) =
                    query_blast_impact.get_component_mut::<Transform>(*child)
                {
                    commands.entity(*child).remove::<Parent>();
                    child_transform.translation =
                        s_transform.transform_point(child_transform.translation);
                }
            }
        }
    }
}

pub fn explode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Handle<ColorMaterial>, &Transform, &Health, &Velocity), With<Spaceship>>,
) {
    if let Ok((s_color, s_transform, s_health, s_velocity)) = query.get_single() {
        if s_health.0 > 0 {
            return;
        }

        let color = materials.get(s_color).unwrap().color;
        let mut rng = rand::thread_rng();

        for Triangle(a, b, c) in TRIANGLES {
            let [ab, ac] = [(b - a).truncate(), (c - a).truncate()];
            let area = ab.perp_dot(ac) / 2.0; // .abs() unnecessary since triangles are CCW

            // Arbitrary number of debris per triangle : area/16
            for _ in 0..(area / 16.0).round() as usize {
                let x = rng.gen_range(0.0..=1.0);
                let y = rng.gen_range(0.0..=1.0 - x);

                // Debris translation in 2d relatively to the spaceship
                let debris_relative_2d = a.truncate() + x * ab + y * ac;

                let debris_relative = Vec3::new(
                    debris_relative_2d.x,
                    debris_relative_2d.y,
                    if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                );
                let debris = s_transform.transform_point(debris_relative);

                let dv = Vec3 {
                    x: rng.gen_range(-0.5..0.5),
                    y: rng.gen_range(-0.5..0.5),
                    z: 0.0,
                };

                commands
                    .spawn(Debris)
                    .insert(Velocity(s_velocity.0 + dv))
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: rng.gen_range(1.0..10.0),
                                vertices: 4 * rng.gen_range(1..5),
                            }))
                            .into(),
                        transform: Transform::from_translation(debris),
                        material: materials.add(color.into()),
                        ..default()
                    });
            }
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Spaceship>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn movement(
    // commands: Commands,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    mut query_spaceship: Query<(Entity, &mut Transform, &mut Velocity), With<Spaceship>>,
) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if let Ok((_s_id, mut s_transform, mut s_velocity)) = query_spaceship.get_single_mut() {
        if keys.any_pressed([KeyCode::K, KeyCode::Left]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), 0.04);
            s_transform.rotation *= rotation;
            // c_transform.rotation *= rotation;
        } else if keys.any_pressed([KeyCode::M, KeyCode::Right]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), -0.04);
            s_transform.rotation *= rotation;
            // c_transform.rotation *= rotation;
        }

        if keys.any_pressed([KeyCode::O, KeyCode::Up]) {
            // accelerate(&*s_transform, &mut s_velocity);

            let direction = s_transform.rotation * Vec3::X;
            s_velocity.0 += ACCELERATION * direction;
            // if s_velocity.0.length() > SPEED_MAX {
            //     s_velocity.0 = SPEED_MAX * s_velocity.0.normalize();
            // }
        } else if keys.any_pressed([KeyCode::L, KeyCode::Down]) {
            // decelerate(&*s_transform, &mut s_velocity);
            let direction = s_transform.rotation * Vec3::NEG_X;
            s_velocity.0 += 0.5 * ACCELERATION * direction;
            // if s_velocity.0.length() > 0.5 * SPEED_MAX {
            //     s_velocity.0 = 0.5 * SPEED_MAX * s_velocity.0.normalize();
            // }
        }
        // } else {
        //     decelerate();
        // }
        // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        //     // Either delete or backspace was just pressed
        // }

        s_velocity.0 *= 1.0 - DRAG;
        debug!("Spaceship velocity: {}", s_velocity.0);

        s_transform.translation += s_velocity.0;
    }
}
