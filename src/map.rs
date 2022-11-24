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

#[derive(Component, Debug)]
pub struct Map {
    sectors: Vec<Vec<Option<Entity>>>,
    current_sector_at: [usize; 2],
}

type SectorXY = [isize; 2];

#[derive(Clone, Component)]
pub struct Sector {
    xy: SectorXY,
    adjacent_sectors: Vec<Entity>,
}

#[derive(Resource, Debug)]
pub struct CurrentSectorId(Entity);

#[derive(Component)]
pub struct Star;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // assert!(MAP_SIZE > 2);
    let mut rng = rand::thread_rng();
    // let mut map = Map {
    //     sectors: vec![vec![None; MAP_SIZE]; MAP_SIZE],
    //     current_sector_at: [MAP_SIZE / 2, MAP_SIZE / 2],
    // };

    let mut sectors: Vec<(Entity, SectorXY)> = Vec::with_capacity(9);
    // for i in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {
    //     for j in [MAP_SIZE / 2 - 1, MAP_SIZE / 2, MAP_SIZE / 2 + 1] {

    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            // for i in 0..MAP_SIZE {
            //     for j in 0..MAP_SIZE {
            //         let visibility = Visibility {
            //             is_visible: i >= MAP_SIZE / 2 - 1
            //                 && i <= MAP_SIZE / 2 + 1
            //                 && j >= MAP_SIZE / 2 - 1
            //                 && j <= MAP_SIZE / 2 + 1,
            //         };

            let sector = commands
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

            sectors.push((sector, [i, j]));

            // map.sectors[i][j] = Some(sector);

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

                commands.entity(sector).add_child(star);
            }
        }
    }

    for (sector0, [x0, y0]) in &sectors {
        let mut adjacent_sectors = Vec::with_capacity(8);
        for (sector1, [x1, y1]) in &sectors {
            if sector0 != sector1 && (x0 - x1).abs() <= 1 && (y0 - y1).abs() <= 1 {
                adjacent_sectors.push(*sector1);
            }
        }
        println!("{:?}", adjacent_sectors);
        commands.entity(*sector0).insert(Sector {
            xy: [*x0, *y0],
            adjacent_sectors,
        });
    }
    println!("{:?}", sectors);
    // commands.spawn(map);
    commands.insert_resource(CurrentSectorId(
        sectors
            .iter()
            .find(|&&(_, [i, j])| i == 0 && j == 0)
            .unwrap()
            .0,
    ));
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut query_map: Query<&mut Map>,
    mut current_sector_id: ResMut<CurrentSectorId>,
    query_camera: Query<&Transform, With<Camera>>,
    mut query_sector: Query<(&Sector, &mut Visibility)>,
) {
    let mut rng = rand::thread_rng();
    // let mut map = query_map.single_mut();
    let camera_xyz = query_camera.single().translation;

    let [camera_i, camera_j] = [
        (camera_xyz.x / WINDOW_WIDTH).trunc() as isize,
        (camera_xyz.y / WINDOW_HEIGHT).trunc() as isize,
    ];
    let current_sector = query_sector
        .get_component::<Sector>(current_sector_id.0)
        .unwrap()
        .clone();
    if current_sector.xy == [camera_i, camera_j] {
        return;
    }

    // Turn off the visibility of sectors at distance 2
    // for [i, j] in adjacent_sectors([x, y]) {
    // let camera_sector;
    for sector_id in current_sector.adjacent_sectors {
        let (sector, mut visibility) = query_sector.get_mut(sector_id).unwrap();
        if sector.xy == [camera_i, camera_j] {
            // camera_sector = sector;
            current_sector_id.0 = sector_id;
        }
        let [i, j] = sector.xy;
        let delta_i = if camera_i > i {
            camera_i - i
        } else {
            i - camera_i
        };
        if delta_i > 1 {
            // query_sector
            //     .get_component_mut::<Visibility>(map.sectors[i][j].unwrap())
            //     .unwrap()
            //     .is_visible = false;
            visibility.is_visible = false;
            continue;
        }
        let delta_j = if camera_j > j {
            camera_j - j
        } else {
            j - camera_j
        };
        if delta_j > 1 {
            // query_sector
            //     .get_component_mut::<Visibility>(map.sectors[i][j].unwrap())
            //     .unwrap()
            //     .is_visible = false;
            visibility.is_visible = false;
        }
    }

    let mut new_sectors: Vec<(Entity, SectorXY)> = Vec::with_capacity(3);
    // for [i, j] in adjacent_sectors([camera_i, camera_j]) {
    for i in [-1isize, 0, 1] {
        for j in [-1isize, 0, 1] {
            // if map.sectors[i][j] == None {
            for (sector, mut visibility) in &mut query_sector {
                if sector.xy == [camera_i + i, camera_j + j] {
                    visibility.is_visible = true;
                    continue;
                }
                // Create new sector
                let new_sector = commands
                    .spawn(SpatialBundle {
                        transform: Transform::from_xyz(
                            (i as f32 + 0.5) * WINDOW_WIDTH,
                            (j as f32 + 0.5) * WINDOW_HEIGHT,
                            SECTOR_Z,
                        ),
                        ..default()
                    })
                    .id();

                new_sectors.push((new_sector, [i, j]));

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
                    commands.entity(new_sector).add_child(star);
                }
                // } else {
                //     // Turn on sector visibility
                //     query_sector
                //         .get_component_mut::<Visibility>(map.sectors[i][j].unwrap())
                //         .unwrap()
                //         .is_visible = true;
                // }
            }
        }
    }
    // map.sectors[camera_i][camera_j] = Location::Current; // Useless ?
    // map.current_sector_at = [camera_i, camera_j];

    // println!("{:?}", map);

    // TODO
    // Reste à construire les listes d'adjacence des nouveaux secteurs (et insérer Sector)
    // et à rajouter aux anciennes listes d'adjacence ces nouveaux secteurs
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
