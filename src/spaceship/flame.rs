use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};

use super::{Spaceship, S10, S13, S14, S7, S9};
use crate::{component::Part, keyboard_bindings::KeyboardBindings};

const COLOR: Color = Color::YELLOW;

#[derive(Component)]
pub struct FlameRear;

#[derive(Component)]
pub struct FlameFront;

pub fn rear_spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query_spaceship: Query<Entity, (With<Spaceship>, Without<Part>)>,
) {
    let mut flame = Mesh::new(PrimitiveTopology::TriangleList);
    let v_pos = vec![[0.0, 0.0, 0.0], [0.0, -6.0, 0.0], [0.0, 6.0, 0.0]];
    flame.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    let flame = commands
        .spawn(FlameRear)
        .insert(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(flame)),
            transform: Transform::from_xyz(S7.x, 0.0, -1.0),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    let spaceship = query_spaceship.single();
    commands.entity(spaceship).add_child(flame);
}

pub fn front_spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query_spaceship: Query<Entity, (With<Spaceship>, Without<Part>)>,
) {
    let flame_front_left = commands
        .spawn(FlameFront)
        .insert(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Circle {
                radius: 0.3,
                vertices: 16,
            }))),
            transform: Transform::from_xyz(S9.x, (S9.y + S10.y) / 2.0, -1.0)
                .with_scale(Vec3::from([0.0, 0.0, 1.0])),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    let flame_front_right = commands
        .spawn(FlameFront)
        .insert(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Circle {
                radius: 0.3,
                vertices: 16,
            }))),
            transform: Transform::from_xyz(S13.x, (S13.y + S14.y) / 2.0, -1.0)
                .with_scale(Vec3::from([0.0, 0.0, 1.0])),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    let spaceship = query_spaceship.single();
    commands
        .entity(spaceship)
        .push_children(&[flame_front_left, flame_front_right]);
}

pub fn rear_update(
    mut meshes: ResMut<Assets<Mesh>>,
    keys: Res<Input<KeyCode>>,
    query: Query<&Mesh2dHandle, With<FlameRear>>,
    query_bindings: Query<&KeyboardBindings>,
) {
    if let Ok(mesh) = query.get_single() {
        if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(vertices)) = meshes
            .get_mut(&mesh.0)
            .unwrap()
            .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            if keys.any_pressed([query_bindings.single().accelerate(), KeyCode::Up]) {
                if vertices[0][0] > -20.0 {
                    vertices[0][0] -= 4.0;
                } else {
                    vertices[0][0] += 4.0;
                }
            } else if vertices[0][0] < 0.0 {
                vertices[0][0] += 4.0;
            }
        }
    }
}

pub fn front_update(
    mut query: Query<&mut Transform, With<FlameFront>>,
    keys: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
) {
    for mut transform in query.iter_mut() {
        if keys.any_pressed([query_bindings.single().decelerate(), KeyCode::Down]) {
            if transform.scale.x < 10.0 {
                transform.scale.x += 4.0;
                transform.scale.y += 4.0;
            } else {
                transform.scale.x -= 4.0;
                transform.scale.y -= 4.0;
            }
        } else if transform.scale.x > 0.0 {
            transform.scale.x -= 4.0;
            transform.scale.y -= 4.0;
        }
    }
}
