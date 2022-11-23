use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const MAP_SIZE: usize = 33;
pub const MAP_CENTER_X: f32 = (MAP_SIZE / 2) as f32 * WINDOW_WIDTH + WINDOW_WIDTH / 2.;
pub const MAP_CENTER_Y: f32 = (MAP_SIZE / 2) as f32 * WINDOW_HEIGHT + WINDOW_HEIGHT / 2.;
const COLOR: Color = Color::WHITE;
const STARS_BY_SECTOR: usize = 50;
const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;
const SECTOR_Z: f32 = 0.0;

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
//     (MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1),
//     (MAP_SIZE / 2 - 1, MAP_SIZE / 2),
//     (MAP_SIZE / 2 - 1, MAP_SIZE / 2 + 1),
//     (MAP_SIZE / 2, MAP_SIZE / 2 - 1),
//     (MAP_SIZE / 2, MAP_SIZE / 2),
//     (MAP_SIZE / 2, MAP_SIZE / 2 + 1),
//     (MAP_SIZE / 2 + 1, MAP_SIZE / 2 - 1),
//     (MAP_SIZE / 2 + 1, MAP_SIZE / 2),
//     (MAP_SIZE / 2 + 1, MAP_SIZE / 2 + 1),
// ];

// #[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
// enum Location {
//     Unexplored,
//     Explored,
//     Current,
// }

#[derive(Component, Debug)]
pub struct Map {
    sectors: Vec<Vec<Option<Entity>>>,
    current_sector_at: [usize; 2],
}

#[derive(Component)]
pub struct Sector;

#[derive(Component)]
pub struct Star;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    assert!(MAP_SIZE > 2);
    let mut rng = rand::thread_rng();
    let mut map = Map {
        sectors: vec![vec![None; MAP_SIZE]; MAP_SIZE],
        current_sector_at: [MAP_SIZE / 2, MAP_SIZE / 2],
    };
    // for i in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
    //     for j in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
    // sectors[i][j] = Location::Explored;
    //     }
    // }
    // sectors[MAP_SIZE / 2][MAP_SIZE / 2] = Location::Current;
    // println!("{:?}", map);

    for i in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
        for j in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
            // for i in 0..MAP_SIZE {
            //     for j in 0..MAP_SIZE {
            //         let visibility = Visibility {
            //             is_visible: i >= MAP_SIZE / 2 - 1
            //                 && i <= MAP_SIZE / 2 + 1
            //                 && j >= MAP_SIZE / 2 - 1
            //                 && j <= MAP_SIZE / 2 + 1,
            //         };

            let sector = commands
                .spawn(Sector)
                .insert(SpatialBundle {
                    transform: Transform::from_xyz(
                        (i as f32 + 0.5) * WINDOW_WIDTH,
                        (j as f32 + 0.5) * WINDOW_HEIGHT,
                        SECTOR_Z,
                    ),
                    // visibility,
                    ..default()
                })
                .id();

            map.sectors[i][j] = Some(sector);

            for _ in 0..STARS_BY_SECTOR {
                let star = commands
                    .spawn(Star)
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: RADIUS,
                                vertices: VERTICES,
                            }))
                            .into(),
                        transform: Transform::from_xyz(
                            rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                            rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                            BACKGROUND,
                        ),
                        material: materials.add(COLOR.into()),
                        ..default()
                    })
                    .id();

                commands.entity(sector).add_child(star);
            }
        }
    }

    commands.spawn(map);
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_map: Query<&mut Map>,
    query_camera: Query<&Transform, With<Camera>>,
    mut query_sector: Query<&mut Visibility, With<Sector>>,
) {
    let mut rng = rand::thread_rng();
    let mut map = query_map.single_mut();
    let camera_xyz = query_camera.single().translation;

    let [camera_i, camera_j] = [
        (camera_xyz.x / WINDOW_WIDTH).trunc() as usize,
        (camera_xyz.y / WINDOW_HEIGHT).trunc() as usize,
    ];
    if map.current_sector_at == [camera_i, camera_j] {
        return;
    }

    // Turn off the visibility of sectors at distance 2
    for [i, j] in adjacent_sectors(map.current_sector_at) {
        let delta_i = if camera_i > i {
            camera_i - i
        } else {
            i - camera_i
        };
        if delta_i > 1 {
            query_sector
                .get_mut(map.sectors[i][j].unwrap())
                .unwrap()
                .is_visible = false;
            continue;
        }
        let delta_j = if camera_j > j {
            camera_j - j
        } else {
            j - camera_j
        };
        if delta_j > 1 {
            query_sector
                .get_mut(map.sectors[i][j].unwrap())
                .unwrap()
                .is_visible = false;
        }
    }

    for [i, j] in adjacent_sectors([camera_i, camera_j]) {
        if map.sectors[i][j] == None {
            // Create new sector
            let sector = commands
                .spawn(Sector)
                .insert(SpatialBundle {
                    transform: Transform::from_xyz(
                        (i as f32 + 0.5) * WINDOW_WIDTH,
                        (j as f32 + 0.5) * WINDOW_HEIGHT,
                        SECTOR_Z,
                    ),
                    ..default()
                })
                .id();

            map.sectors[i][j] = Some(sector);

            for _ in 0..STARS_BY_SECTOR {
                let star = commands
                    .spawn(Star)
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: RADIUS,
                                vertices: VERTICES,
                            }))
                            .into(),
                        transform: Transform::from_xyz(
                            rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                            rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                            BACKGROUND,
                        ),
                        material: materials.add(COLOR.into()),
                        ..default()
                    })
                    .id();
                commands.entity(sector).add_child(star);
            }
        } else {
            // Turn on sector visibility
            query_sector
                .get_mut(map.sectors[i][j].unwrap())
                .unwrap()
                .is_visible = true;
        }
    }

    // map.sectors[camera_i][camera_j] = Location::Current; // Useless ?
    map.current_sector_at = [camera_i, camera_j];

    println!("{:?}", map);
}

fn adjacent_sectors([i, j]: [usize; 2]) -> Vec<[usize; 2]> {
    let mut sector_x = Vec::with_capacity(3);
    if i > 0 {
        sector_x.push(i - 1);
    }
    sector_x.push(i);
    if i < MAP_SIZE - 1 {
        sector_x.push(i + 1);
    }

    let mut sector_y = Vec::with_capacity(3);
    if j > 0 {
        sector_y.push(j - 1);
    }
    sector_y.push(j);
    if j < MAP_SIZE - 1 {
        sector_y.push(j + 1);
    }

    sector_x
        .into_iter()
        .flat_map(|x| sector_y.iter().map(move |&y| [x, y]))
        .collect()
}

// pub fn update(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Transform, &Velocity), With<Star>>,
// ) {
//     for (star, mut transform, velocity) in query.iter_mut() {
//         transform.translation += velocity.0;
//         if transform.translation.x < -WINDOW_WIDTH / 2.0 {
//             commands.entity(star).despawn();
//         }
//     }
// }