use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const MAP_SIZE: usize = 33;
pub const MAP_CENTER_X: f32 = (MAP_SIZE / 2) as f32 * WINDOW_WIDTH + WINDOW_WIDTH / 2.;
pub const MAP_CENTER_Y: f32 = (MAP_SIZE / 2) as f32 * WINDOW_HEIGHT + WINDOW_HEIGHT / 2.;
const COLOR: Color = Color::WHITE;
const STARS_PER_SECTOR: usize = 50;
const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;
const SECTOR_Z: f32 = 0.0;

#[derive(Clone, Component, Debug)]
pub struct Sector {
    i: isize,
    j: isize,
    adjacent_sectors: Vec<Entity>,
}

#[derive(Debug, Resource)]
pub struct CurrentSectorId(Entity);

#[derive(Component)]
pub struct Star;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut sectors: Vec<(Entity, Sector)> = Vec::with_capacity(9);

    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            let sector_id = commands
                .spawn(SpatialBundle {
                    transform: Transform::from_xyz(
                        (i as f32 + 0.5) * WINDOW_WIDTH,
                        (j as f32 + 0.5) * WINDOW_HEIGHT,
                        SECTOR_Z,
                    ),
                    // visibility,
                    ..default()
                })
                .id();

            sectors.push((
                sector_id,
                Sector {
                    i,
                    j,
                    adjacent_sectors: Vec::new(),
                },
            ));

            for _ in 0..STARS_PER_SECTOR {
                let star_id = commands
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

                commands.entity(sector_id).add_child(star_id);
            }
        }
    }

    commands.insert_resource(CurrentSectorId(
        sectors
            .iter()
            .find(|&(_, sector)| sector.i == 0 && sector.j == 0)
            .unwrap()
            .0,
    ));

    let mut k = 0;
    loop {
        let mut iter = sectors.iter_mut().skip(k);
        if let Some((sector_id_0, sector_0)) = iter.next() {
            for (sector_id_1, sector_1) in iter {
                if (sector_0.i - sector_1.i).abs() <= 1 && (sector_0.j - sector_1.j).abs() <= 1 {
                    sector_0.adjacent_sectors.push(*sector_id_1);
                    sector_1.adjacent_sectors.push(*sector_id_0);
                }
            }
            k += 1;
        } else {
            break;
        }
    }

    println!("{:?}", sectors);

    for (sector_id, sector) in sectors.into_iter() {
        commands.entity(sector_id).insert(sector);
    }
    // for (sector0, [x0, y0]) in &sectors {
    //     let mut adjacent_sectors = Vec::with_capacity(8);
    //     for (sector1, [x1, y1]) in &sectors {
    //         if sector0 != sector1 && (x0 - x1).abs() <= 1 && (y0 - y1).abs() <= 1 {
    //             adjacent_sectors.push(*sector1);
    //         }
    //     }
    //     println!("{:?}", adjacent_sectors);
    //     commands.entity(*sector0).insert(Sector {
    //         xy: [*x0, *y0],
    //         adjacent_sectors,
    //     });
    // }
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_sector_id: ResMut<CurrentSectorId>,
    query_camera: Query<&Transform, With<Camera>>,
    mut query_sector: Query<(Entity, &mut Sector, &mut Visibility)>,
) {
    let mut rng = rand::thread_rng();
    let camera_xyz = query_camera.single().translation;

    let camera_i = (camera_xyz.x / WINDOW_WIDTH).floor() as isize;
    let camera_j = (camera_xyz.y / WINDOW_HEIGHT).floor() as isize;

    let current_sector = query_sector
        .get_component::<Sector>(current_sector_id.0)
        .unwrap()
        .clone();
    if [current_sector.i, current_sector.j] == [camera_i, camera_j] {
        return;
    }
    debug!("Camera({}, {})", camera_i, camera_j);
    debug!("Current sector({}, {})", current_sector.i, current_sector.j);

    // Turn off the visibility of sectors at distance 2
    for sector_id in current_sector.adjacent_sectors {
        let (_, sector, mut visibility) = query_sector.get_mut(sector_id).unwrap();
        if [sector.i, sector.j] == [camera_i, camera_j] {
            current_sector_id.0 = sector_id;
            continue;
        }

        if (camera_i - sector.i).abs() > 1 || (camera_j - sector.j).abs() > 1 {
            visibility.is_visible = false;
        }
    }

    let mut new_sectors: Vec<(Entity, Sector)> = Vec::with_capacity(3);
    for di in [-1isize, 0, 1] {
        'outer: for dj in [-1isize, 0, 1] {
            for (_, sector, mut visibility) in &mut query_sector {
                if [sector.i, sector.j] == [camera_i + di, camera_j + dj] {
                    visibility.is_visible = true;
                    continue 'outer;
                }
            }

            // Create new sector
            let new_sector_id = commands
                .spawn(SpatialBundle {
                    transform: Transform::from_xyz(
                        ((camera_i + di) as f32 + 0.5) * WINDOW_WIDTH,
                        ((camera_j + dj) as f32 + 0.5) * WINDOW_HEIGHT,
                        SECTOR_Z,
                    ),
                    ..default()
                })
                .id();

            new_sectors.push((
                new_sector_id,
                Sector {
                    i: camera_i + di,
                    j: camera_j + dj,
                    adjacent_sectors: Vec::new(),
                },
            ));

            for _ in 0..STARS_PER_SECTOR {
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
                commands.entity(new_sector_id).add_child(star);
            }
        }
    }

    for (sector_id, mut sector, _) in &mut query_sector {
        for (new_sector_id, new_sector) in &mut new_sectors {
            if (sector.i - new_sector.i).abs() <= 1 && (sector.j - new_sector.j).abs() <= 1 {
                sector.adjacent_sectors.push(*new_sector_id);
                new_sector.adjacent_sectors.push(sector_id);
            }
        }
    }

    let mut i = 0;
    loop {
        let mut iter = new_sectors.iter_mut().skip(i);
        if let Some((new_sector_id_0, new_sector_0)) = iter.next() {
            for (new_sector_id_1, new_sector_1) in iter {
                if (new_sector_0.i - new_sector_1.i).abs() <= 1
                    && (new_sector_0.j - new_sector_1.j).abs() <= 1
                {
                    new_sector_0.adjacent_sectors.push(*new_sector_id_1);
                    new_sector_1.adjacent_sectors.push(*new_sector_id_0);
                }
            }
            i += 1;
        } else {
            break;
        }
    }

    for (new_sector_id, new_sector) in new_sectors {
        debug!("{:?}: {:?}", new_sector_id, new_sector);
        commands.entity(new_sector_id).insert(new_sector);
    }
}

// fn adjacent_sectors([i, j]: [usize; 2]) -> Vec<[usize; 2]> {
//     let mut sector_x = Vec::with_capacity(3);
//     if i > 0 {
//         sector_x.push(i - 1);
//     }
//     sector_x.push(i);
//     if i < MAP_SIZE - 1 {
//         sector_x.push(i + 1);
//     }

//     let mut sector_y = Vec::with_capacity(3);
//     if j > 0 {
//         sector_y.push(j - 1);
//     }
//     sector_y.push(j);
//     if j < MAP_SIZE - 1 {
//         sector_y.push(j + 1);
//     }

//     sector_x
//         .into_iter()
//         .flat_map(|x| sector_y.iter().map(move |&y| [x, y]))
//         .collect()
// }

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
