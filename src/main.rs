use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use rand::Rng;

const INITIAL_COUNT_OF_STARS_BY_VELOCITY: usize = 10;
const MAX_VELOCITY_OF_STARS: usize = 10;
const MAX_VELOCITY_OF_ASTEROIDS: usize = 5;

#[derive(Component)]
struct Stars {}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Spaceship {}

#[derive(Component)]
struct Asteroid {
    radius: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: 800.,
            height: 600.,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_startup_system(spaceship)
        .add_startup_system(setup_stars)
        .add_system(add_stars)
        .add_system(update_stars)
        .add_system(asteroids)
        .add_system(keyboard_input)
        .add_system(detect_collision_spaceship_asteroid)
        .add_system(bevy::window::close_on_esc)
        // .add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     debug_globaltransform.after(TransformSystem::TransformPropagate),
        // )
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spaceship = Mesh::new(PrimitiveTopology::TriangleList);

    let v_pos = vec![
        [4., -0.5, 0.],
        [-2., 1.5, 0.],
        [-4., -2.5, 0.],
        [1., -0.5, 0.],
        [-3., 2.5, 0.],
        [-3., -0.5, 0.],
    ];
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
    // for i in 2..=10 {
    //     indices.extend_from_slice(&[0, i, i - 1]);
    // }
    spaceship.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn()
        .insert(Spaceship {})
        .insert_bundle(ColorMesh2dBundle {
            // mesh: Mesh2dHandle(meshes.add(spaceship)),
            mesh: meshes.add(spaceship).into(),
            transform: Transform::from_xyz(-300., 0., 0.).with_scale(Vec3::splat(10.0)),
            material: materials.add(Color::rgb(0.25, 0., 1.).into()),
            ..default()
        });

    // commands
    //     .spawn_bundle((
    //         ColoredMesh2d::default(),
    //         Mesh2dHandle(meshes.add(spaceship)),
    //         Transform::default(),
    //         GlobalTransform::default(),
    //         Visibility::default(),
    //         ComputedVisibility::default(),
    //     ))
    //     .insert(Spaceship {});
}

// /// A marker component for colored 2d meshes
// #[derive(Component, Default)]
// pub struct ColoredMesh2d;

fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for velocity in 1..MAX_VELOCITY_OF_STARS + 1 {
        let mut vertices = Vec::new();
        for _i in 0..INITIAL_COUNT_OF_STARS_BY_VELOCITY {
            let x = rng.gen_range(-400. ..400.);
            let y = rng.gen_range(-300. ..300.);
            vertices.push(([x, y, 0.0], [0., 1., 0.], [1., 1.]));
        }

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut stars = Mesh::new(PrimitiveTopology::PointList);
        stars.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        stars.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        stars.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        commands
            .spawn()
            .insert(Stars {})
            .insert(Velocity(Vec3::from([-(velocity as f32), 0., 0.])))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(stars).into(),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                ..default()
            });
    }
}

fn add_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut vertices = Vec::new();
    let velocity = Vec3::from([
        -(rng.gen_range(1..MAX_VELOCITY_OF_STARS + 1) as f32),
        0.,
        0.,
    ]);

    for _j in 0..1 {
        let y = rng.gen_range(-300. ..300.);
        vertices.push(([400., y, 0.0], [0., 1., 0.], [1., 1.]));
    }

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut stars = Mesh::new(PrimitiveTopology::PointList);
    stars.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    stars.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    stars.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    // stars.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    commands
        .spawn()
        .insert(Stars {})
        .insert(Velocity(velocity))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(stars).into(),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        });
}

fn update_stars(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform, &Velocity), With<Stars>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0
        //     for value in mesh.attributes() {
        //         println!("{}", value);
        //     }
    }
    // for mesh in meshes.get_handle() {}
}

// /// Print the up-to-date global coordinates of the player as of **this frame**.
// fn debug_globaltransform(query: Query<&GlobalTransform, With<Stars>>) {
//     for mesh in query.iter() {
//         debug!("Mesh at: {:?}", mesh.translation());
//     }
// }

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Spaceship>>) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if keys.any_pressed([KeyCode::H, KeyCode::Left]) {
        // W is being held down
        let mut transform = query.single_mut();
        transform.translation += Vec3::from([-2., 0., 0.]);
    }

    if keys.any_pressed([KeyCode::J, KeyCode::Down]) {
        // W is being held down
        let mut transform = query.single_mut();
        transform.translation += Vec3::from([0., -2., 0.]);
    }

    if keys.any_pressed([KeyCode::K, KeyCode::Up]) {
        // W is being held down
        let mut transform = query.single_mut();
        transform.translation += Vec3::from([0., 2., 0.]);
    }

    if keys.any_pressed([KeyCode::L, KeyCode::Right]) {
        // W is being held down
        let mut transform = query.single_mut();
        transform.translation += Vec3::from([2., 0., 0.]);
    }

    // // we can check multiple at once with `.any_*`
    // if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
    //     // Either the left or right shift are being held down
    // }

    // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
    //     // Either delete or backspace was just pressed
    // }
}

fn asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
) {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0..100) == 0 {
        let radius = rng.gen_range(10..50) as f32;
        let velocity = Vec3::from([
            -(rng.gen_range(1..MAX_VELOCITY_OF_ASTEROIDS + 1) as f32),
            0.,
            0.,
        ]);
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: 16,
                    }))
                    .into(),
                transform: Transform::from_xyz(450., rng.gen_range(-250..250) as f32, 0.),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .insert(Asteroid { radius })
            .insert(Velocity(velocity));
    }

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Spaceship)>,
    asteroid_query: Query<(&Transform, &Asteroid)>,
) {
    let (spaceship_entity, spaceship_transform, _) = spaceship_query.single();
    for (asteroid_transform, asteroid) in asteroid_query.iter() {
        if spaceship_transform
            .translation
            .distance(asteroid_transform.translation)
            < asteroid.radius * 1.5
        {
            commands.entity(spaceship_entity).despawn();
        }
    }
}
