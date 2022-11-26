use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const BACKGROUND: f32 = 0.0;
const RADIUS: f32 = 1.0;
const VERTICES: usize = 4;
const COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct Star;

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let mut rng = rand::thread_rng();
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

    star
}
