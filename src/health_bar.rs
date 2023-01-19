use bevy::{
    prelude::*,
    render::mesh::{PrimitiveTopology, VertexAttributeValues},
    sprite::Mesh2dHandle,
};

use crate::{Health, Spaceship, WINDOW_HEIGHT, WINDOW_WIDTH};

const COLOR_CONTENTS: Color = Color::BLUE;
const COLOR_OUTLINE: Color = Color::WHITE;
const CONTENTS_POSITION: Vec3 = Vec3 {
    x: OUTLINE_POSITION.x,
    y: OUTLINE_POSITION.y,
    z: OUTLINE_POSITION.z - 1.0,
};
const HEALTH_MAX: f32 = crate::spaceship::HEALTH as f32;
const HEIGHT: f32 = 10.0;
const OUTLINE_POSITION: Vec3 = Vec3 {
    x: -WINDOW_WIDTH / 2.0,
    y: WINDOW_HEIGHT / 2.0,
    z: 0.0, // above the background of stars
};
const WIDTH: f32 = 200.0;

#[derive(Component)]
pub struct HealthBarContents;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_camera: Query<Entity, With<Camera>>,
) {
    let h1 = Vec3::new(0.0, 0.0, 0.0);
    let h2 = Vec3::new(0.0, -HEIGHT, 0.0);
    let h3 = Vec3::new(WIDTH, -HEIGHT, 0.0);
    let h4 = Vec3::new(WIDTH, 0.0, 0.0);
    let h_pos = [h1, h2, h3, h4, h1].map(|h| h.to_array()).to_vec();
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, h_pos);

    let health_bar_outline = commands
        .spawn(ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_translation(OUTLINE_POSITION),
            material: materials.add(COLOR_OUTLINE.into()),
            ..default()
        })
        .id();

    let h_pos = [h1, h2, h3, h3, h4, h1].map(|h| h.to_array()).to_vec();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, h_pos);

    let health_bar_contents = commands
        .spawn(HealthBarContents)
        .insert(ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_translation(CONTENTS_POSITION),
            material: materials.add(COLOR_CONTENTS.into()),
            ..default()
        })
        .id();

    commands
        .entity(query_camera.single())
        .push_children(&[health_bar_outline, health_bar_contents]);
}

pub fn update(
    mut meshes: ResMut<Assets<Mesh>>,
    query_health_bar: Query<&Mesh2dHandle, With<HealthBarContents>>,
    query_spaceship: Query<&Health, With<Spaceship>>,
) {
    if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
        .get_mut(&query_health_bar.single().0)
        .unwrap()
        .attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        let health = query_spaceship.get_single().map_or(0, |h| h.0);
        let health_bar_width = health as f32 / HEALTH_MAX * WIDTH;
        vertices[2][0] = health_bar_width;
        vertices[3][0] = health_bar_width;
        vertices[4][0] = health_bar_width;
    }
}
