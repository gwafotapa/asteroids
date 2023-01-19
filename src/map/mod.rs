use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use star::StarsEvent;

pub mod star;

const SECTOR_Z: f32 = 0.0;

#[derive(Clone, Component, Debug)]
pub struct Sector {
    i: isize,
    j: isize,
    neighboors: Vec<Entity>,
    seed: u64,
}

#[derive(Debug, Resource)]
pub struct CurrentSectorId(Entity);

pub fn spawn(mut stars_event: EventWriter<StarsEvent>, mut commands: Commands) {
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
                    ..default()
                })
                .id();

            let seed = rng.gen::<u64>();
            sectors.push((
                sector_id,
                Sector {
                    i,
                    j,
                    neighboors: Vec::new(),
                    seed,
                },
            ));

            stars_event.send(StarsEvent { sector_id, seed });
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
                    sector_0.neighboors.push(*sector_id_1);
                    sector_1.neighboors.push(*sector_id_0);
                }
            }
            k += 1;
        } else {
            break;
        }
    }

    debug!("{:?}", sectors);

    for (sector_id, sector) in sectors.into_iter() {
        commands.entity(sector_id).insert(sector);
    }
}

pub fn update(
    mut commands: Commands,
    mut current_sector_id: ResMut<CurrentSectorId>,
    mut query_sector: Query<(Entity, &mut Sector, &mut Visibility)>,
    mut stars_event: EventWriter<StarsEvent>,
    query_camera: Query<&Transform, With<Camera>>,
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

    // Update current sector and turn off the visibility of sectors now at distance 2
    for sector_id in current_sector.neighboors {
        let (sector_id, sector, mut visibility) = query_sector.get_mut(sector_id).unwrap();
        if [sector.i, sector.j] == [camera_i, camera_j] {
            current_sector_id.0 = sector_id;
            continue;
        }

        if (camera_i - sector.i).abs() > 1 || (camera_j - sector.j).abs() > 1 {
            visibility.is_visible = false;
            commands.entity(sector_id).despawn_descendants();
        }
    }

    // Create up to 3 new sectors. Turn on the visibility of old sectors when needed
    let mut new_sectors: Vec<(Entity, Sector)> = Vec::with_capacity(3);
    for di in [-1isize, 0, 1] {
        'outer: for dj in [-1isize, 0, 1] {
            // Check if that sector is already known
            for (sector_id, sector, mut visibility) in &mut query_sector {
                if [sector.i, sector.j] == [camera_i + di, camera_j + dj] {
                    if !visibility.is_visible {
                        visibility.is_visible = true;
                        stars_event.send(StarsEvent {
                            sector_id,
                            seed: sector.seed,
                        });
                    }
                    continue 'outer;
                }
            }

            // If not, create it
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

            let seed = rng.gen::<u64>();
            let [i, j] = [camera_i + di, camera_j + dj];
            new_sectors.push((
                new_sector_id,
                Sector {
                    i,
                    j,
                    neighboors: Vec::new(),
                    seed,
                },
            ));

            // Populate this new sector with stars
            stars_event.send(StarsEvent {
                sector_id: new_sector_id,
                seed,
            });
        }
    }

    // Update the field 'neighboors' of old sectors with new sectors and vice versa
    for (sector_id, mut sector, _) in &mut query_sector {
        for (new_sector_id, new_sector) in &mut new_sectors {
            if (sector.i - new_sector.i).abs() <= 1 && (sector.j - new_sector.j).abs() <= 1 {
                sector.neighboors.push(*new_sector_id);
                new_sector.neighboors.push(sector_id);
            }
        }
    }

    // Complete the field 'neighboors' of new sectors (with new_sectors)
    let mut i = 0;
    loop {
        let mut iter = new_sectors.iter_mut().skip(i);
        if let Some((new_sector_id_0, new_sector_0)) = iter.next() {
            for (new_sector_id_1, new_sector_1) in iter {
                if (new_sector_0.i - new_sector_1.i).abs() <= 1
                    && (new_sector_0.j - new_sector_1.j).abs() <= 1
                {
                    new_sector_0.neighboors.push(*new_sector_id_1);
                    new_sector_1.neighboors.push(*new_sector_id_0);
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
