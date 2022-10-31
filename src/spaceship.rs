use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

const SPACESHIP_TRIANGLELIST: [[f32; 3]; 6] = [
    [40.0, -5.0, 0.0],
    [-20.0, 15.0, 0.0],
    [-40.0, -25.0, 0.0],
    [10.0, -5.0, 0.0],
    [-30.0, 25.0, 0.0],
    [-30.0, -5.0, 0.0],
];

const SPACESHIP_ENVELOP: [[f32; 3]; 6] = [
    [40.0, -5.0, 0.0],
    [-30.0, 25.0, 0.0],
    [-40.0, -25.0, 0.0],
    [-30.0, -5.0, 0.0],
    [-5.0, 10.0, 0.0],
    [0.0, -15.0, 0.0],
];

#[derive(Component)]
pub struct Spaceship {
    envelop: Vec<Vec3>,
}

impl Spaceship {
    pub fn envelop(&self) -> &Vec<Vec3> {
        &self.envelop
    }
}

pub fn spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spaceship = Mesh::new(PrimitiveTopology::TriangleList);

    let v_pos = SPACESHIP_TRIANGLELIST.to_vec();
    let v_normals = vec![[0., 0., 1.]; 6];
    let v_uvs = vec![[1., 1.]; 6];
    spaceship.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    spaceship.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normals);
    spaceship.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uvs);

    // let mut v_color: Vec<u32> = vec![Color::BLUE.as_linear_rgba_u32()];
    // v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 2]);
    // spaceship.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 10, VertexFormat::Uint32),
    //     v_color,
    // );

    let indices = vec![0, 1, 2, 3, 4, 5];
    spaceship.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn()
        .insert(Spaceship {
            envelop: SPACESHIP_ENVELOP.map(|x| Vec3::from(x)).to_vec(),
        })
        .insert_bundle(ColorMesh2dBundle {
            // mesh: Mesh2dHandle(meshes.add(spaceship)),
            mesh: meshes.add(spaceship).into(),
            transform: Transform::from_xyz(-300., 0., 0.),
            // .with_scale(Vec3::splat(10.0)),
            material: materials.add(Color::rgb(0.25, 0., 1.).into()),
            ..default()
        });
}
