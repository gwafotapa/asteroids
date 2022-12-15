use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};

use super::{Health, Spaceship, S10, S13, S14, S7, S9};

const COLOR: Color = Color::YELLOW;

#[derive(Component)]
pub struct FlameRear;

#[derive(Component)]
pub struct FlameFront;

pub fn rear_spawn(
    query_spaceship: Query<Entity, With<Spaceship>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut flame = Mesh::new(PrimitiveTopology::TriangleList);
    let v_pos = vec![[0.0, 0.0, 0.0], [0.0, -6.0, 0.0], [0.0, 6.0, 0.0]];
    flame.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // let mut v_color: Vec<u32> = vec![Color::YELLOW.as_linear_rgba_u32(); 3];
    // // v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 3]);
    // flame.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
    //     v_color,
    // );

    // let indices = vec![0, 1, 2];
    // flame.set_indices(Some(Indices::U32(indices)));

    let flame = commands
        .spawn(FlameRear)
        .insert(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(flame)),
            transform: Transform::from_xyz(S7.x, 0.0, -1.0),
            // .with_scale(Vec3::from([0.02, 0.02, 1.0])),
            // .with_scale(Vec3::from([1.0, 1.0, 1.0])),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    let spaceship = query_spaceship.single();
    commands.entity(spaceship).add_child(flame);
}

pub fn front_spawn(
    query_spaceship: Query<Entity, With<Spaceship>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh2dHandle, With<FlameRear>>,
) {
    if let Ok(mesh) = query.get_single() {
        // meshes
        //     .get_mut(&mesh.0)
        //     .unwrap()
        //     .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        //     .unwrap()
        //     .as_float3()
        //     .unwrap()[0] = [-800.0, 0.0, 0.0];

        // println!(
        //     "indices: {:?}",
        //     meshes
        //         .get_mut(&mesh.0)
        //         .unwrap()
        //         .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        //         .unwrap() // .as_float3()
        //                   // .unwrap()[0]
        // );

        // if keys.any_just_pressed([KeyCode::K, KeyCode::Up]) {
        //     visibility.is_visible = true;
        // }

        if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(vertices)) = meshes
            .get_mut(&mesh.0)
            .unwrap()
            .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            if keys.any_pressed([KeyCode::O, KeyCode::Up]) {
                if vertices[0][0] > -20.0 {
                    vertices[0][0] -= 4.0;
                } else {
                    vertices[0][0] += 4.0;
                }
            } else if vertices[0][0] < 0.0 {
                vertices[0][0] += 4.0;
            }
        }

        // if keys.any_just_released([KeyCode::K, KeyCode::Up]) {
        //     visibility.is_visible = false;
        // }
    }
}

pub fn front_update(
    keys: Res<Input<KeyCode>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<FlameFront>>,
) {
    for mut transform in query.iter_mut() {
        // meshes
        //     .get_mut(&mesh.0)
        //     .unwrap()
        //     .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        //     .unwrap()
        //     .as_float3()
        //     .unwrap()[0] = [-800.0, 0.0, 0.0];

        // println!(
        //     "indices: {:?}",
        //     meshes
        //         .get_mut(&mesh.0)
        //         .unwrap()
        //         .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        //         .unwrap() // .as_float3()
        //                   // .unwrap()[0]
        // );

        // if keys.any_just_pressed([KeyCode::K, KeyCode::Up]) {
        //     visibility.is_visible = true;
        // }

        if keys.any_pressed([KeyCode::L, KeyCode::Down]) {
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
        // if keys.any_just_released([KeyCode::K, KeyCode::Up]) {
        //     visibility.is_visible = false;
        // }
    }
}

pub fn despawn(
    mut commands: Commands,
    query_flame: Query<Entity, Or<(With<FlameRear>, With<FlameFront>)>>,
    query_spaceship: Query<(Entity, &Health), With<Spaceship>>,
) {
    if let Ok((spaceship, health)) = query_spaceship.get_single() {
        if health.0 <= 0 {
            for flame in query_flame.iter() {
                commands.entity(spaceship).remove_children(&[flame]);
                commands.entity(flame).despawn();
            }
        }
    }
}
