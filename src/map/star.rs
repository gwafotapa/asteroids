use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;
const COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct Star;

pub struct StarEvent {
    pub sector: Entity,
}

pub fn spawn(
    mut star_event: EventReader<StarEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for ev in star_event.iter() {
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

        commands.entity(ev.sector).add_child(star);
    }
}
