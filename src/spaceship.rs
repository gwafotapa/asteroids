use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    collision::{math::point_in_triangle, HitBox, Impact, Surface, Topology, Triangle},
    Blast, Debris, Direction, Fire, Health, Velocity, ALTITUDE,
};

const HEALTH: i32 = 10;

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
// const MIDPOINT_AB: Vec3 = Vec3 {
//     x: (A.x + B.x) / 2.0,
//     y: (A.y + B.y) / 2.0,
//     z: (A.z + B.z) / 2.0,
// };
// const MIDPOINT_DB: Vec3 = Vec3 {
//     x: (D.x + B.x) / 2.0,
//     y: (D.y + B.y) / 2.0,
//     z: (D.z + B.z) / 2.0,
// };
// pub const TRIANGLE_LIST: [Vec3; 12] = [A, B, C, D, C, B, E, O, F, G, F, O];
pub const TRIANGLES: [Triangle; 4] = [[S1, S2, S3], [S4, S3, S2], [S5, S6, S7], [S8, S7, S6]];
const HITBOX: HitBox = HitBox {
    half_x: S2.x,
    half_y: S4.y,
};
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
const ACCELERATION: f32 = 0.1;
const POSITION: Vec3 = Vec3 {
    // x: -WINDOW_WIDTH / 4.0,
    // x: -WINDOW_WIDTH / 2.0,
    x: 0.0,
    y: 0.0,
    // y: -crate::WINDOW_HEIGHT,
    z: ALTITUDE,
};
pub const ATTACK_SOURCE: Vec3 = S2;
const SPACESHIP_COLOR: Color = Color::BLUE;
pub const ATTACK_COLOR: Color = Color::YELLOW;
const BLAST_RADIUS: f32 = 8.0;
const BLAST_VERTICES: usize = 8;
const FIRE_RADIUS: f32 = 3.0;
const FIRE_VERTICES: usize = 4;
const IMPACT_RADIUS: f32 = 12.0;
const IMPACT_VERTICES: usize = 16;
const FIRE_VELOCITY: Vec3 = Vec3 {
    x: 12.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Component)]
pub struct Spaceship;

impl Spaceship {
    // pub fn envelop(&self) -> &Vec<Vec3> {
    //     &self.envelop
    // }

    pub fn accelerate(velocity: &mut Velocity, direction: Direction) {
        // if self.velocity.length() < VELOCITY_MAX {
        velocity.0 += match direction {
            Direction::Left => Vec3 {
                x: -ACCELERATION,
                y: 0.0,
                z: 0.0,
            },
            Direction::Down => Vec3 {
                x: 0.0,
                y: -ACCELERATION,
                z: 0.0,
            },
            Direction::Up => Vec3 {
                x: 0.0,
                y: ACCELERATION,
                z: 0.0,
            },
            Direction::Right => Vec3 {
                x: ACCELERATION,
                y: 0.0,
                z: 0.0,
            },
        };
        // }
    }

    pub fn decelerate_x(velocity: &mut Velocity) {
        if velocity.0.x > 0.0 {
            velocity.0.x -= ACCELERATION / 2.0;
        } else if velocity.0.x < 0.0 {
            velocity.0.x += ACCELERATION / 2.0;
        }
    }

    pub fn decelerate_y(velocity: &mut Velocity) {
        if velocity.0.y > 0.0 {
            velocity.0.y -= ACCELERATION / 2.0;
        } else if velocity.0.y < 0.0 {
            velocity.0.y += ACCELERATION / 2.0;
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spaceship = Mesh::new(PrimitiveTopology::TriangleList);

    let v_pos: Vec<[f32; 3]> = TRIANGLES.iter().flatten().map(|x| x.to_array()).collect();
    let v_normals = vec![[0.0, 0.0, 1.0]; 12];
    let v_uvs = vec![[1.0, 1.0]; 12];
    spaceship.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    spaceship.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normals);
    spaceship.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uvs);

    // let mut v_color: Vec<u32> = vec![Color::BLUE.as_linear_rgba_u32()];
    // v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 2]);
    // spaceship.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 10, VertexFormat::Uint32),
    //     v_color,
    // );

    // let indices = vec![0, 1, 2, 3, 4, 5];
    // spaceship.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn_empty()
        .insert(Spaceship)
        .insert(Health(HEALTH))
        .insert(Velocity(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }))
        .insert(Surface {
            topology: Topology::Triangles(&TRIANGLES),
            hitbox: HITBOX,
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
            mesh: meshes.add(spaceship).into(),
            transform: Transform::from_translation(POSITION),
            // material: materials.add(Color::rgb(0.25, 0., 1.).into()),
            material: materials.add(SPACESHIP_COLOR.into()),
            ..default()
        });
}

pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spaceship: Entity,
    transform: &Transform,
    // attack: &Attack,
) {
    // let (spaceship, transform) = query.single();

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
            // transform: transform.clone().with_scale(Vec3::splat(5.0)),
            transform: Transform::from_translation(ATTACK_SOURCE),
            // .with_scale(Vec3::splat(1.0)),
            material: materials.add(ATTACK_COLOR.into()),
            ..default()
        })
        .id();

    commands.entity(spaceship).push_children(&[blast]);

    commands
        .spawn_empty()
        .insert(Fire {
            impact_radius: IMPACT_RADIUS,
            impact_vertices: IMPACT_VERTICES,
        })
        .insert(Health(1))
        .insert(Velocity(FIRE_VELOCITY))
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
            transform: Transform::from_translation(
                transform.translation + ATTACK_SOURCE * transform.scale,
            ),
            material: materials.add(ATTACK_COLOR.into()),
            ..default()
        });
}

pub fn explode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (
            Option<&Children>,
            &Handle<ColorMaterial>,
            &Health,
            &GlobalTransform,
            &Velocity,
        ),
        With<Spaceship>,
    >,
    mut query_blast_impact: Query<&mut Transform, Or<(With<Blast>, With<Impact>)>>,
) {
    if let Ok((s_children, s_color, s_health, s_transform, s_velocity)) = query.get_single() {
        if s_health.0 > 0 {
            return;
        }

        if let Some(children) = s_children {
            for child in children {
                commands.entity(*child).remove::<Parent>();
                if let Ok(mut child_transform) =
                    query_blast_impact.get_component_mut::<Transform>(*child)
                {
                    child_transform.translation =
                        s_transform.transform_point(child_transform.translation);
                }
            }
        }

        let color = materials.get(s_color).unwrap().color;
        let mut rng = rand::thread_rng();
        for _ in 1..10 {
            let mut debris;
            'outer: loop {
                debris = Vec3 {
                    x: rng.gen_range(S5.x..S2.x),
                    y: rng.gen_range(S1.y..S4.y),
                    z: 0.0,
                };
                let mut triangles = TRIANGLES.iter();
                while let Some(&[a, b, c]) = triangles.next() {
                    if point_in_triangle(
                        debris.truncate(),
                        a.truncate(),
                        b.truncate(),
                        c.truncate(),
                    ) {
                        break 'outer;
                    }
                }
            }
            debris.z += ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            let debris_translation = s_transform.translation() + debris;
            let dv = Vec3 {
                x: rng.gen_range(-0.5..0.5),
                y: rng.gen_range(-0.5..0.5),
                z: 0.0,
            };

            commands
                .spawn_empty()
                .insert(Debris)
                .insert(Velocity(s_velocity.0 + dv))
                .insert(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: 10.0,
                            vertices: 4,
                        }))
                        .into(),
                    transform: Transform::from_translation(debris_translation),
                    material: materials.add(color.into()),
                    ..default()
                });
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
