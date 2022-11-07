use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::{PI, SQRT_2};

use super::{
    Asteroid, Boss, BossPart, Direction, Health, Level, RectangularEnvelop, Velocity,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

const ALTITUDE: f32 = 100.0;
const INNER_RADIUS: f32 = 100.0;
const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;

/// Counter clockwise
pub const POLYGON: [Vec3; 16] = [
    Vec3 {
        x: -OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: INNER_RADIUS - OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS - OUTER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: -OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS - INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: INNER_RADIUS - OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: OUTER_RADIUS - INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS - INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS - OUTER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: OUTER_RADIUS - INNER_RADIUS,
        z: 0.0,
    },
];

const SIZE: f32 = 100.0;
const INITIAL_POSITION: Vec3 = Vec3 {
    x: 300.0,
    y: 0.0,
    z: ALTITUDE,
};
const ACCELERATION: f32 = 0.1;
const COLOR: Color = Color::ORANGE;
const HEALTH: usize = 10;

pub const BULLET_COLOR: Color = Color::RED;
const POSITIONS_OF_CANONS: [Vec3; 8] = [
    Vec3 {
        x: SIZE * SQRT_2,
        y: 0.0,
        z: ALTITUDE,
    },
    Vec3 {
        x: -SIZE * SQRT_2,
        y: 0.0,
        z: ALTITUDE,
    },
    Vec3 {
        x: 0.0,
        y: SIZE * SQRT_2,
        z: ALTITUDE,
    },
    Vec3 {
        x: 0.0,
        y: -SIZE * SQRT_2,
        z: ALTITUDE,
    },
    Vec3 {
        x: SIZE,
        y: SIZE,
        z: ALTITUDE,
    },
    Vec3 {
        x: -SIZE,
        y: -SIZE,
        z: ALTITUDE,
    },
    Vec3 {
        x: SIZE,
        y: -SIZE,
        z: ALTITUDE,
    },
    Vec3 {
        x: -SIZE,
        y: SIZE,
        z: ALTITUDE,
    },
];

#[derive(Component)]
pub struct Fire;

pub fn create_triangle_list_from_polygon(polygon: &[Vec3], center: Vec3) -> Vec<Vec3> {
    let mut triangle_list = Vec::new();
    let mut iter = polygon.iter();
    let mut p1 = iter.next();
    let mut p2 = iter.next();
    let p0 = p1;
    while p2.is_some() {
        triangle_list.push(center);
        triangle_list.push(*p1.unwrap());
        triangle_list.push(*p2.unwrap());
        p1 = p2;
        p2 = iter.next();
    }
    triangle_list.push(center);
    triangle_list.push(*p1.unwrap());
    triangle_list.push(*p0.unwrap());

    triangle_list
}

pub fn add_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level_query: Query<&mut Level>,
    asteroid_query: Query<&Asteroid>,
) {
    let mut level = level_query.single_mut();
    if !level.boss_spawned && level.distance_to_boss == 0 && asteroid_query.is_empty() {
        let boss = commands
            .spawn()
            .insert(Boss)
            .insert(Health(HEALTH))
            .insert(Velocity(Vec3::ZERO))
            .insert(RectangularEnvelop {
                half_x: OUTER_RADIUS,
                half_y: OUTER_RADIUS,
            })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(INITIAL_POSITION),
                ..default()
            })
            .id();

        let boss_part1 = commands
            .spawn()
            .insert(BossPart)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: (200.0, 200.0).into(),
                        flip: false,
                    }))
                    .into(),
                material: materials.add(Color::ORANGE.into()),
                ..default()
            })
            .id();

        let boss_part2 = commands
            .spawn()
            .insert(BossPart)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: (2.0 * SIZE, 2.0 * SIZE).into(),
                        flip: false,
                    }))
                    .into(),
                transform: Transform::identity().with_rotation(Quat::from_rotation_z(PI / 4.0)),
                material: materials.add(COLOR.into()),
                ..default()
            })
            .id();

        commands
            .entity(boss)
            .push_children(&[boss_part1, boss_part2]);

        level.boss_spawned = true;
    }
}

pub fn add_boss_2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level_query: Query<&mut Level>,
    asteroid_query: Query<&Asteroid>,
) {
    let mut level = level_query.single_mut();
    if !level.boss_spawned && level.distance_to_boss == 0 && asteroid_query.is_empty() {
        let mut boss = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices_position = create_triangle_list_from_polygon(&POLYGON, Vec3::ZERO)
            .into_iter()
            .map(|x| x.to_array())
            .collect::<Vec<_>>();
        let mut vertices_normal = Vec::new();
        let mut vertices_uv = Vec::new();
        for _ in &vertices_position {
            vertices_normal.push([0.0, 0.0, 1.0]);
            vertices_uv.push([0.0, 0.0]);
        }

        boss.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices_position);
        boss.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal);
        boss.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices_uv);

        commands
            .spawn()
            .insert(Boss)
            .insert(Health(HEALTH))
            .insert(Velocity(Vec3::ZERO))
            .insert(RectangularEnvelop {
                half_x: OUTER_RADIUS,
                half_y: OUTER_RADIUS,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(boss).into(),
                transform: Transform::from_translation(INITIAL_POSITION),
                material: materials.add(COLOR.into()),
                ..default()
            });

        level.boss_spawned = true;
    }
}

pub fn move_boss(mut query: Query<(&mut Transform, &mut Velocity), With<Boss>>) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut rng = rand::thread_rng();
        let mut acceleration = Vec::new();
        if transform.translation.x < WINDOW_WIDTH / 2.0 {
            acceleration.push(Direction::Left);
        }
        if transform.translation.x > -WINDOW_WIDTH / 2.0 {
            acceleration.push(Direction::Right);
        }
        if transform.translation.y < WINDOW_HEIGHT / 2.0 {
            acceleration.push(Direction::Up);
        }
        if transform.translation.y > -WINDOW_HEIGHT / 2.0 {
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
    }
}

pub fn attack_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_boss: Query<&Transform, With<Boss>>,
    query_spaceship: Query<&Transform, With<crate::Spaceship>>,
) {
    if let Ok(boss_transform) = query_boss.get_single() {
        if let Ok(spaceship_transform) = query_spaceship.get_single() {
            let mut rng = rand::thread_rng();
            for canon_relative_position in POSITIONS_OF_CANONS {
                if rng.gen_range(0..100) == 0 {
                    let canon_absolute_position = boss_transform.translation
                        + canon_relative_position
                        + Vec3::from([0.0, 0.0, 1.0]);
                    commands
                        .spawn()
                        .insert(Fire)
                        .insert(Velocity(
                            (spaceship_transform.translation - canon_absolute_position).normalize()
                                * 4.0,
                        ))
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: 10.0,
                                    vertices: 8,
                                }))
                                .into(),
                            transform: Transform::from_translation(canon_absolute_position),
                            material: materials.add(BULLET_COLOR.into()),
                            ..default()
                        });
                }
            }
        }
    }
}
