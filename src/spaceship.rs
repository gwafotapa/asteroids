use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};

use super::{Attack, Blast, Direction, Fire, Health, RectangularEnvelop, Velocity};

const ALTITUDE: f32 = 100.0;
const HEALTH: usize = 10;

const O: Vec3 = Vec3::ZERO;
const A: Vec3 = Vec3 {
    x: -3.0,
    y: -3.0,
    z: 0.0,
};
const B: Vec3 = Vec3 {
    x: 3.0,
    y: 0.0,
    z: 0.0,
};
const C: Vec3 = Vec3 {
    x: -2.0,
    y: 0.0,
    z: 0.0,
};
const D: Vec3 = Vec3 {
    x: -3.0,
    y: 3.0,
    z: 0.0,
};
const E: Vec3 = Vec3 {
    x: -4.0,
    y: -2.0,
    z: 0.0,
};
const F: Vec3 = Vec3 {
    x: -3.0,
    y: 0.0,
    z: 0.0,
};
const G: Vec3 = Vec3 {
    x: -4.0,
    y: 2.0,
    z: 0.0,
};
const MIDPOINT_AB: Vec3 = Vec3 {
    x: (A.x + B.x) / 2.0,
    y: (A.y + B.y) / 2.0,
    z: (A.z + B.z) / 2.0,
};
const MIDPOINT_DB: Vec3 = Vec3 {
    x: (D.x + B.x) / 2.0,
    y: (D.y + B.y) / 2.0,
    z: (D.z + B.z) / 2.0,
};
pub const TRIANGLE_LIST: [Vec3; 12] = [A, B, C, D, C, B, E, O, F, G, F, O];
const RECTANGULAR_ENVELOP: RectangularEnvelop = RectangularEnvelop {
    half_x: 3.5,
    half_y: 3.0,
};
pub const ENVELOP: [Vec3; 7] = [E, A, B, D, G, MIDPOINT_AB, MIDPOINT_DB];
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
const ACCELERATION: f32 = 0.05;
pub const ATTACK_SOURCE: Vec3 = B;
const SPACESHIP_COLOR: Color = Color::BLUE;
pub const ATTACK_COLOR: Color = Color::YELLOW_GREEN;
const BLAST_RADIUS: f32 = 0.4;
const BLAST_VERTICES: usize = 8;
const FIRE_RADIUS: f32 = 2.0;
const FIRE_VERTICES: usize = 4;

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

pub fn spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spaceship = Mesh::new(PrimitiveTopology::TriangleList);

    let v_pos = TRIANGLE_LIST.map(|x| x.to_array()).to_vec();
    let v_normals = vec![[0., 0., 1.]; 12];
    let v_uvs = vec![[1., 1.]; 12];
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
        .spawn()
        .insert(Spaceship)
        .insert(Health(HEALTH))
        .insert(Velocity(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }))
        .insert(RECTANGULAR_ENVELOP)
        .insert(Attack {
            source: ATTACK_SOURCE,
            color: ATTACK_COLOR,
            blast_radius: BLAST_RADIUS,
            blast_vertices: BLAST_VERTICES,
            fire_radius: FIRE_RADIUS,
            fire_vertices: FIRE_VERTICES,
        })
        .insert_bundle(ColorMesh2dBundle {
            // mesh: Mesh2dHandle(meshes.add(spaceship)),
            mesh: meshes.add(spaceship).into(),
            transform: Transform::from_xyz(-300., 0., ALTITUDE).with_scale(Vec3 {
                x: 10.0,
                y: 10.0,
                z: 1.0,
            }),
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
    attack: &Attack,
) {
    // let (spaceship, transform) = query.single();

    let blast = commands
        .spawn()
        .insert(Blast)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: attack.blast_radius,
                    vertices: attack.blast_vertices,
                }))
                .into(),
            // transform: transform.clone().with_scale(Vec3::splat(5.0)),
            transform: Transform::from_translation(attack.source),
            // .with_scale(Vec3::splat(1.0)),
            material: materials.add(attack.color.into()),
            ..default()
        })
        .id();

    commands.entity(spaceship).push_children(&[blast]);

    commands
        .spawn()
        .insert(Fire)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: attack.fire_radius,
                    vertices: attack.fire_vertices,
                }))
                .into(),
            transform: Transform::from_translation(
                transform.translation + attack.source * transform.scale,
            ),
            material: materials.add(attack.color.into()),
            ..default()
        });
}
