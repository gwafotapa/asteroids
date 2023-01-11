use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    asteroid::AsteroidEvent,
    AngularVelocity,
    // map::ASTEROIDS_MAX_PER_SECTOR,
    Health,
    Mass,
    MomentOfInertia,
    Velocity,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use star::StarsEvent;
const ASTEROIDS_MAX_PER_SECTOR: usize = 5;
const ASTEROID_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const ASTEROID_HEALTH_MAX: u32 = 60;
const ASTEROID_VELOCITY_MIN: f32 = 100.0;
const ASTEROID_VELOCITY_MAX: f32 = 500.0;

const SECTOR_Z: f32 = 0.0;

pub mod star;

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
                    // visibility,
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

            // for _ in 0..STARS_PER_SECTOR {
            stars_event.send(StarsEvent { sector_id, seed });
            // }
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
    mut asteroid_event: EventWriter<AsteroidEvent>,
    mut stars_event: EventWriter<StarsEvent>,
    mut commands: Commands,
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
                    // visibility.is_visible = true;
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
            // for _ in 0..STARS_PER_SECTOR {
            stars_event.send(StarsEvent {
                sector_id: new_sector_id,
                seed,
            });
            // }

            // Populate this new sector with asteroids
            // let population = rng.gen_range(0..ASTEROIDS_MAX_PER_SECTOR + 1);
            // let asteroids = asteroid::spawn(&mut commands, &mut meshes, &mut materials, population);
            // commands.entity(new_sector_id).push_children(&asteroids);
            for _ in 0..rng.gen_range(0..ASTEROIDS_MAX_PER_SECTOR + 1) {
                let xmin = i as f32 * WINDOW_WIDTH;
                let ymin = j as f32 * WINDOW_HEIGHT;
                let x = rng.gen_range(xmin..xmin + WINDOW_WIDTH);
                let y = rng.gen_range(ymin..ymin + WINDOW_HEIGHT);
                let health = Health(rng.gen_range(10..ASTEROID_HEALTH_MAX + 1));
                let radius = (health.0 * 2) as f32;
                let area = PI * radius.powi(2);
                let mass = Mass(area);
                let moment_of_inertia = MomentOfInertia(0.5 * mass.0 * radius.powi(2));
                let rho = rng.gen_range(ASTEROID_VELOCITY_MIN..ASTEROID_VELOCITY_MAX);
                let theta = rng.gen_range(0.0..2.0 * PI);
                let velocity = Velocity(Vec3::from([rho * theta.cos(), rho * theta.sin(), 0.]));
                let angular_velocity = AngularVelocity(rng.gen_range(0.0..0.01));

                asteroid_event.send(AsteroidEvent {
                    x,
                    y,
                    radius,
                    vertices: 16,
                    color: ASTEROID_COLOR,
                    health,
                    mass,
                    moment_of_inertia,
                    velocity,
                    angular_velocity,
                });
            }
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
