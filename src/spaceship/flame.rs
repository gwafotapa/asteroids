use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Mesh2dHandle,
};

use super::{Spaceship, S7};

const COLOR: Color = Color::YELLOW;

#[derive(Component)]
pub struct Flame;

pub fn spawn(
    query_spaceship: Query<Entity, With<Spaceship>>,
    mut commands: Commands,
    // We will add a new Mesh for the star being created
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut flame = Mesh::new(PrimitiveTopology::TriangleList);
    let v_pos = vec![
        // [0.0, 0.0, 0.0],
        // [0.0, -100.0, 0.0],
        // [0.0, 100.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, -6.0, 0.0],
        [0.0, 6.0, 0.0],
    ];
    flame.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // let mut v_color: Vec<u32> = vec![Color::YELLOW.as_linear_rgba_u32(); 3];
    // // v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 3]);
    // flame.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
    //     v_color,
    // );

    let indices = vec![0, 1, 2];
    flame.set_indices(Some(Indices::U32(indices)));

    // We can now spawn the entities for the star and the camera
    let flame = commands
        .spawn(Flame)
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

pub fn update(
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh2dHandle, With<Flame>>,
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
            if keys.any_pressed([KeyCode::K, KeyCode::Up]) {
                if vertices[0][0] > -20.0 {
                    vertices[0][0] -= 4.0;
                    // vertices[3][0] -= 100.0;
                } else {
                    vertices[0][0] += 4.0;
                    // vertices[3][0] += 100.0;
                }
                // println!("{:?}", vertices[0]);
            } else {
                if vertices[0][0] < 0.0 {
                    vertices[0][0] += 4.0;
                    // vertices[3][0] += 100.0;
                }
            }
        }

        // if keys.any_just_released([KeyCode::K, KeyCode::Up]) {
        //     visibility.is_visible = false;
        // }
    }
}
