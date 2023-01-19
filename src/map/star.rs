use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::constant::{WINDOW_HEIGHT, WINDOW_WIDTH};

const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;
const COLOR: Color = Color::WHITE;
pub const STARS_PER_SECTOR: usize = 50;

#[derive(Component)]
pub struct Star;

pub struct StarsEvent {
    pub sector_id: Entity,
    pub seed: u64,
}

pub fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stars_event: EventReader<StarsEvent>,
) {
    for ev in stars_event.iter() {
        let mut rng = Pcg32::seed_from_u64(ev.seed);
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

            commands.entity(ev.sector_id).add_child(star);
        }
    }
}
