use bevy::prelude::*;
use rand::Rng;

use crate::{Velocity, PLANE_Z, WINDOW_HEIGHT, WINDOW_WIDTH};

pub const MAP_SIZE: usize = 3;
pub const MAP_CENTER_X: f32 = (MAP_SIZE / 2) as f32 * WINDOW_WIDTH + WINDOW_WIDTH / 2.;
pub const MAP_CENTER_Y: f32 = (MAP_SIZE / 2) as f32 * WINDOW_HEIGHT + WINDOW_HEIGHT / 2.;
const COLOR: Color = Color::WHITE;
const COUNT_BY_SECTOR: usize = 50;
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
    for i in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
        for j in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
            sectors[i][j] = Location::Explored;
        }
    }
    sectors[MAP_SIZE / 2][MAP_SIZE / 2] = Location::Current;
    // println!("{:?}", map);

    commands.spawn(Map {
        sectors,
        current_sector: [MAP_SIZE / 2, MAP_SIZE / 2],
    });

    let mut rng = rand::thread_rng();
    for i in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
        for j in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
            let sector = commands
                .spawn(Sector)
                .insert(SpatialBundle {
                    transform: Transform::from_xyz(
                        (i as f32 + 0.5) * WINDOW_WIDTH,
                        (j as f32 + 0.5) * WINDOW_HEIGHT,
                        PLANE_Z,
                    ),
                    ..default()
                })
                .id();
            for _ in 0..COUNT_BY_SECTOR {
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
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_map: Query<&mut Map>,
    query_camera: Query<&Transform, With<Camera>>,
) {
    let mut map = query_map.single_mut();
    let camera_xyz = query_camera.single().translation;
    let mut rng = rand::thread_rng();

    let [i, j] = [
        (camera_xyz.x / WINDOW_WIDTH).trunc() as usize,
        (camera_xyz.y / WINDOW_HEIGHT).trunc() as usize,
    ];
    if [i, j] == map.current_sector {
        return;
    }

    map.current_sector = [i, j];
    map.sectors[i][j] = Location::Current;
    let mut sector_x = vec![i];
    let mut sector_y = vec![j];
    if i > 0 {
        sector_x.push(i - 1);
    }
    if i < MAP_SIZE - 1 {
        sector_x.push(i + 1);
    }
    if j > 0 {
        sector_y.push(j - 1);
    }
    if j < MAP_SIZE - 1 {
        sector_y.push(j + 1);
    }

    'outer: for i in sector_x {
        for &j in &sector_y {
            if map.sectors[i][j] == Location::Unexplored {
                map.sectors[i][j] = Location::Explored;
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
                for _ in 0..COUNT_BY_SECTOR {
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
