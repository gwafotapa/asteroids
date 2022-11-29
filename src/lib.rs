use bevy::{prelude::*, render::mesh::PrimitiveTopology};
use rand::Rng;
// use std::time::Instant;
// use std::f32::consts::SQRT_2;pub

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod collision;
pub mod debris;
pub mod fire;
pub mod map;
pub mod spaceship;

use boss::BOSS_Z;
use spaceship::{Spaceship, SPACESHIP_Z};

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

const COMPASS_POSITION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0 - 10.0,
    y: WINDOW_HEIGHT / 2.0 - 10.0,
    z: -CAMERA_Z,
};

const PLANE_Z: f32 = 500.0;
// pub const WINDOW_WIDTH: f32 = 1920.0;
// pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
// pub const WINDOW_WIDTH: f32 = 800.0;
// pub const WINDOW_HEIGHT: f32 = 600.0;
const DISTANCE_TO_TARGET: f32 = 5000.0;
const CAMERA_Z: f32 = 1000.0;
const CAMERA_SPEED: f32 = 0.05;
const CAMERA_REAR_GAP: f32 = 100.0;

#[derive(Component)]
pub struct Velocity(Vec3);

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Level {
    target: Vec3,
    // boss_spawned: bool,
}

#[derive(Component)]
pub struct Health(i32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Eq, PartialEq)]
pub enum CameraPositioning {
    Center,
    Rear,
}

pub fn camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            // transform: Transform::from_xyz(MAP_CENTER_X, MAP_CENTER_Y, CAMERA_Z),
            // transform: Transform::from_xyz(0.0, 0.0, CAMERA_Z),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, CAMERA_Z),
            ..default()
        })
        .insert(CameraPositioning::Center);
}

#[derive(Component)]
pub struct Compass;

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Camera>>,
) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-DISTANCE_TO_TARGET..DISTANCE_TO_TARGET);
    let y_max = (DISTANCE_TO_TARGET.powi(2) - x.powi(2)).sqrt();
    let y = rng.gen_range(-y_max..y_max);

    commands
        .spawn(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                // format!("Distance: {:12}", INITIAL_DISTANCE),
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::GRAY,
                },
            )
            // Set the alignment of the Text
            // .with_text_alignment(TextAlignment::TOP_RIGHT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                // align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(WINDOW_WIDTH - 150.0),
                    // bottom: Val::Px(10.0),
                    // right: Val::Px(1000.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Level {
            target: Vec3::new(x, y, BOSS_Z),
            // boss_spawned: false,
        });

    let camera = query.single();
    let c1 = Vec3::new(75.0, 0.0, 0.0);
    let c2 = Vec3::new(-50.0, 50.0, 0.0);
    let c3 = Vec3::new(-25.0, 0.0, 0.0);
    let c4 = Vec3::new(-50.0, -50.0, 0.0);
    let v_pos = vec![c1, c2, c3, c3, c4, c1];
    let mut compass = Mesh::new(PrimitiveTopology::TriangleList);
    compass.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    commands.spawn(Compass).insert(ColorMesh2dBundle {
        mesh: meshes.add(compass).into(),
        transform: Transform::from_translation(camera.translation + COMPASS_POSITION)
            .with_scale(Vec3::splat(0.13)),
        material: materials.add(Color::WHITE.into()),
        ..default()
    });
}

pub fn compass(
    mut query_level: Query<(&Level, &mut Text)>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
    query_camera: Query<&Transform, With<Camera>>,
    mut query_compass: Query<&mut Transform, (With<Compass>, Without<Spaceship>, Without<Camera>)>,
) {
    if let Ok(spaceship) = query_spaceship.get_single() {
        let (level, mut text) = query_level.single_mut();
        let camera = query_camera.single();
        let mut compass = query_compass.single_mut();
        compass.translation = camera.translation + COMPASS_POSITION;
        compass.rotation = Quat::from_rotation_arc_2d(
            Vec2::X,
            (level.target - spaceship.translation)
                .truncate()
                .normalize(),
        );
        let distance = (level.target - spaceship.translation).length();
        text.sections[0].value = format!("Target: {:>7.0}", distance);
    }
}

// /// Print the up-to-date global coordinates of the player as of **this frame**.
// pub fn debug_globaltransform(query: Query<&GlobalTransform, With<Star>>) {
//     for mesh in query.iter() {
//         debug!("Mesh at: {:?}", mesh.translation());
//     }
// }

pub fn keyboard_input(
    // commands: Commands,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    mut query_camera: Query<(&mut CameraPositioning, &mut Transform), With<Camera>>,
    mut query_spaceship: Query<
        (Entity, &mut Transform, &mut Velocity),
        (With<Spaceship>, Without<Camera>),
    >,
) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if let Ok((_s_id, mut s_transform, mut s_velocity)) = query_spaceship.get_single_mut() {
        let (mut camera_positioning, mut c_transform) = query_camera.get_single_mut().unwrap();
        if keys.any_just_pressed([KeyCode::Space, KeyCode::C]) {
            *camera_positioning = if *camera_positioning == CameraPositioning::Center {
                CameraPositioning::Rear
            } else {
                CameraPositioning::Center
            };
        }

        if keys.any_pressed([KeyCode::H, KeyCode::Left]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), 0.04);
            s_transform.rotation *= rotation;
            // c_transform.rotation *= rotation;
        } else if keys.any_pressed([KeyCode::L, KeyCode::Right]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), -0.04);
            s_transform.rotation *= rotation;
            // c_transform.rotation *= rotation;
        }

        if keys.any_pressed([KeyCode::K, KeyCode::Up]) {
            // Spaceship::accelerate(&*s_transform, &mut s_velocity);

            let direction = s_transform.rotation * Vec3::X;
            s_velocity.0 += spaceship::ACCELERATION * direction;
            // if s_velocity.0.length() > spaceship::SPEED_MAX {
            //     s_velocity.0 = spaceship::SPEED_MAX * s_velocity.0.normalize();
            // }
        } else if keys.any_pressed([KeyCode::J, KeyCode::Down]) {
            // Spaceship::decelerate(&*s_transform, &mut s_velocity);
            let direction = s_transform.rotation * Vec3::NEG_X;
            s_velocity.0 += 0.5 * spaceship::ACCELERATION * direction;
            // if s_velocity.0.length() > 0.5 * spaceship::SPEED_MAX {
            //     s_velocity.0 = 0.5 * spaceship::SPEED_MAX * s_velocity.0.normalize();
            // }
        }
        // } else {
        //     Spaceship::decelerate();
        // }
        // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        //     // Either delete or backspace was just pressed
        // }

        s_velocity.0 *= 1.0 - spaceship::DRAG;
        debug!("Spaceship velocity: {}", s_velocity.0);

        s_transform.translation += s_velocity.0;
        c_transform.translation += s_velocity.0;
        // c_transform.translation = s_transform.translation;

        if *camera_positioning == CameraPositioning::Rear {
            // In Rear position, the camera places itself so that the ship is at distance 100.0
            // from the window border with the velocity vector of the ship (not the ship itself)
            // pointing at the camera (which is always at the center of the window).
            // The window is a rectangle of dimensions WINDOW_WIDTH and WINDOW_HEIGHT.
            // Consider the inside rectangle obtained by removing a 100-width strip
            // from the window. We aim to place the camera on this rectangle.
            // The diagonals of this rectangle split its area into 4 quadrants.
            // The computation depends on which quadrant the destination of the camera is.
            let c_destination;
            if s_velocity.0 == Vec3::ZERO {
                c_destination = s_transform.translation;
            } else if s_velocity.0.x == 0.0
                || (s_velocity.0.y / s_velocity.0.x).abs()
                    > (WINDOW_HEIGHT / 2.0 - CAMERA_REAR_GAP)
                        / (WINDOW_WIDTH / 2.0 - CAMERA_REAR_GAP)
            {
                let y = if s_velocity.0.y > 0.0 {
                    // Upper quadrant
                    WINDOW_HEIGHT / 2.0 - CAMERA_REAR_GAP
                } else {
                    // Lower quadrant
                    -(WINDOW_HEIGHT / 2.0 - CAMERA_REAR_GAP)
                };
                c_destination = s_transform.translation
                    + Vec3 {
                        x: y * s_velocity.0.x / s_velocity.0.y,
                        y,
                        z: CAMERA_Z - SPACESHIP_Z,
                    };
            } else {
                let x = if s_velocity.0.x > 0.0 {
                    // Right quadrant
                    WINDOW_WIDTH / 2.0 - CAMERA_REAR_GAP
                } else {
                    // Lower quadrant
                    -(WINDOW_WIDTH / 2.0 - CAMERA_REAR_GAP)
                };

                c_destination = s_transform.translation
                    + Vec3 {
                        x,
                        y: s_velocity.0.y / s_velocity.0.x * x,
                        z: CAMERA_Z - SPACESHIP_Z,
                    };
            }

            let c_path = c_destination - c_transform.translation;
            c_transform.translation += CAMERA_SPEED * c_path;
        } else {
            let direction = Vec3 {
                x: s_transform.translation.x - c_transform.translation.x,
                y: s_transform.translation.y - c_transform.translation.y,
                z: 0.0,
            };
            c_transform.translation += CAMERA_SPEED * direction;
        }
    }
}
