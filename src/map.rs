use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{Velocity, PLANE_Z, WINDOW_HEIGHT, WINDOW_WIDTH};

const MAP_SIZE: usize = 5;
pub const MAP_CENTER: usize = MAP_SIZE / 2;
const COLOR: Color = Color::WHITE;
const INITIAL_COUNT_BY_SECTOR: usize = 50;
const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;

// const SECTOR_CENTERS: [(f32, f32); 9] = [
//     (-WINDOW_WIDTH, WINDOW_HEIGHT),
//     (0., WINDOW_HEIGHT),
//     (WINDOW_WIDTH, WINDOW_HEIGHT),
//     (-WINDOW_WIDTH, 0.),
//     (0., 0.),
//     (WINDOW_WIDTH, 0.),
//     (-WINDOW_WIDTH, -WINDOW_HEIGHT),
//     (0., -WINDOW_HEIGHT),
//     (WINDOW_WIDTH, -WINDOW_HEIGHT),
// ];
// const INITIAL_SECTORS: [(usize, usize); 9] = [
//     (MAP_CENTER - 1, MAP_CENTER - 1),
//     (MAP_CENTER - 1, MAP_CENTER),
//     (MAP_CENTER - 1, MAP_CENTER + 1),
//     (MAP_CENTER, MAP_CENTER - 1),
//     (MAP_CENTER, MAP_CENTER),
//     (MAP_CENTER, MAP_CENTER + 1),
//     (MAP_CENTER + 1, MAP_CENTER - 1),
//     (MAP_CENTER + 1, MAP_CENTER),
//     (MAP_CENTER + 1, MAP_CENTER + 1),
// ];

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
enum Location {
    Unexplored,
    Explored,
    Current,
}

#[derive(Component)]
pub struct Map {
    sectors: Vec<Vec<Location>>,
    current_sector: [usize; 2],
}

#[derive(Component)]
struct Sector;

#[derive(Component)]
pub struct Star;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    assert!(MAP_SIZE > 2);
    let mut sectors = vec![vec![Location::Unexplored; MAP_SIZE]; MAP_SIZE];
    // for (x, y) in INITIAL_SECTORS {
    //     sectors[x][y] = Location::Explored;
    // }
    for i in [MAP_CENTER - 1, MAP_CENTER, MAP_CENTER + 1] {
        for j in [MAP_CENTER - 1, MAP_CENTER, MAP_CENTER + 1] {
            sectors[i][j] = Location::Explored;
        }
    }
    sectors[MAP_CENTER][MAP_CENTER] = Location::Current;
    // println!("{:?}", map);

    commands.spawn(Map {
        sectors,
        current_sector: [MAP_CENTER, MAP_CENTER],
    });

    let mut rng = rand::thread_rng();
    for i in [MAP_CENTER - 1, MAP_CENTER, MAP_CENTER + 1] {
        for j in [MAP_CENTER - 1, MAP_CENTER, MAP_CENTER + 1] {
            let sector = commands
                .spawn(Sector)
                .insert(SpatialBundle {
                    transform: Transform::from_xyz(
                        i as f32 * WINDOW_WIDTH,
                        j as f32 * WINDOW_HEIGHT,
                        PLANE_Z,
                    ),
                    ..default()
                })
                .id();
            for _ in 0..INITIAL_COUNT_BY_SECTOR {
                let star = commands
                    .spawn(Star)
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: RADIUS,
                                vertices: VERTICES,
                            }))
                            .into(),
                        transform: Transform::from_translation(Vec3 {
                            x: rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                            y: rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                            z: BACKGROUND,
                        }),
                        material: materials.add(COLOR.into()),
                        ..default()
                    })
                    .id();
                commands.entity(sector).add_child(star);
            }
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_map: Query<&mut Map>,
    query_camera: Query<&Transform, With<Camera>>,
) {
    let mut map = query_map.single_mut();
    let transform = query_camera.single();
    let mut rng = rand::thread_rng();

    let mut current_sector = map.current_sector;
    if transform.translation.x > WINDOW_WIDTH / 2. && current_sector[1] < MAP_SIZE - 1 {
        current_sector[1] += 1;
    } else if transform.translation.x < -WINDOW_WIDTH / 2. && current_sector[1] > 0 {
        current_sector[1] -= 1;
    } else if transform.translation.y > WINDOW_HEIGHT / 2. && current_sector[0] > 0 {
        current_sector[0] -= 1;
    } else if transform.translation.y < -WINDOW_HEIGHT / 2. && current_sector[0] < MAP_SIZE - 1 {
        current_sector[0] += 1;
    } else {
        return;
    }
    map.sectors[current_sector[0]][current_sector[1]] = Location::Current;

    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            let sector_x = (current_sector[0] as i32 + i) as usize; // TODO
            let sector_y = (current_sector[1] as i32 + j) as usize; // TODO
            if map.sectors[sector_y][sector_x] == Location::Unexplored {
                map.sectors[sector_y][sector_x] = Location::Explored;
                commands
                    .spawn_empty()
                    .insert(Star)
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: RADIUS,
                                vertices: VERTICES,
                            }))
                            .into(),
                        transform: Transform::from_translation(Vec3 {
                            x: rng.gen_range(
                                (sector_x - MAP_CENTER - 1) as f32 * WINDOW_WIDTH / 2.0
                                    ..(sector_x - MAP_CENTER + 1) as f32 * WINDOW_WIDTH / 2.0,
                            ),
                            y: rng.gen_range(
                                (sector_y - MAP_CENTER - 1) as f32 * WINDOW_HEIGHT / 2.0
                                    ..(sector_y - MAP_CENTER - 1) as f32 * WINDOW_HEIGHT / 2.0,
                            ),
                            z: BACKGROUND,
                        }),
                        material: materials.add(COLOR.into()),
                        ..default()
                    });
            }
        }
    }
}

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Star>>,
) {
    for (star, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            commands.entity(star).despawn();
        }
    }
}
