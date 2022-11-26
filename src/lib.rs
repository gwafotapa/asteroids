use bevy::prelude::*;
// use std::time::Instant;
// use std::f32::consts::SQRT_2;pub

pub mod asteroid;
pub mod boss;
pub mod collision;
pub mod map;
pub mod spaceship;

use spaceship::{Spaceship, SPACESHIP_Z};

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

const PLANE_Z: f32 = 500.0;
// pub const WINDOW_WIDTH: f32 = 1920.0;
// pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
// pub const WINDOW_WIDTH: f32 = 800.0;
// pub const WINDOW_HEIGHT: f32 = 600.0;
const INITIAL_DISTANCE_TO_BOSS: usize = 0000;
const CAMERA_Z: f32 = 1000.0;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Fire {
    impact_radius: f32,
    impact_vertices: usize,
}

#[derive(Component)]
pub struct Blast;

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Level {
    distance_to_boss: usize,
    boss_spawned: bool,
}

#[derive(Component)]
pub struct Health(i32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Debris;

#[derive(Component, Eq, PartialEq)]
pub enum CameraPosition {
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
        .insert(CameraPosition::Center);
}

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            distance_to_boss: INITIAL_DISTANCE_TO_BOSS,
            boss_spawned: false,
        });
}

pub fn distance_to_boss(mut query: Query<(&mut Level, &mut Text)>) {
    for (mut level, mut text) in &mut query {
        if level.distance_to_boss > 0 {
            level.distance_to_boss -= 1;
        }
        text.sections[0].value = format!("Distance: {:12}", level.distance_to_boss);
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
    mut query_camera: Query<(&mut CameraPosition, &mut Transform), With<Camera>>,
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

    if let Ok((_entity, mut transform, mut velocity)) = query_spaceship.get_single_mut() {
        let (mut cam_position, mut cam_transform) = query_camera.get_single_mut().unwrap();
        if keys.any_just_pressed([KeyCode::Space, KeyCode::C]) {
            *cam_position = if *cam_position == CameraPosition::Center {
                CameraPosition::Rear
            } else {
                CameraPosition::Center
            };
        }

        // if keys.any_just_pressed([KeyCode::Space, KeyCode::R]) {
        //     spaceship::attack(commands, meshes, materials, entity, &transform);
        // }

        if keys.any_pressed([KeyCode::H, KeyCode::Left]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), 0.04);
            transform.rotation *= rotation;
            // cam_transform.rotation *= rotation;
        } else if keys.any_pressed([KeyCode::L, KeyCode::Right]) {
            let rotation = Quat::from_axis_angle(Vec3::from([0.0, 0.0, 1.0]), -0.04);
            transform.rotation *= rotation;
            // cam_transform.rotation *= rotation;
        }

        if keys.any_pressed([KeyCode::K, KeyCode::Up]) {
            // Spaceship::accelerate(&*transform, &mut velocity);

            let direction = transform.rotation * Vec3::X;
            velocity.0 += spaceship::ACCELERATION * direction;
            // if velocity.0.length() > spaceship::SPEED_MAX {
            //     velocity.0 = spaceship::SPEED_MAX * velocity.0.normalize();
            // }
        } else if keys.any_pressed([KeyCode::J, KeyCode::Down]) {
            // Spaceship::decelerate(&*transform, &mut velocity);
            let direction = transform.rotation * Vec3::NEG_X;
            velocity.0 += 0.5 * spaceship::ACCELERATION * direction;
            // if velocity.0.length() > 0.5 * spaceship::SPEED_MAX {
            //     velocity.0 = 0.5 * spaceship::SPEED_MAX * velocity.0.normalize();
            // }
        }
        // } else {
        //     Spaceship::decelerate();
        // }
        // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        //     // Either delete or backspace was just pressed
        // }

        let ship_xyz = transform.translation;
        let cam_xyz = cam_transform.translation;

        velocity.0 *= 1.0 - spaceship::DRAG;
        debug!("Spaceship velocity: {}", velocity.0);

        transform.translation += velocity.0;
        cam_transform.translation += velocity.0;
        // cam_transform.translation = transform.translation;

        if *cam_position == CameraPosition::Rear {
            // In Rear position, the camera places itself so that the ship is at distance 100.0
            // from the window border with the velocity vector of the ship (not the ship itself)
            // pointing at the camera (which is always at the center of the window).
            // The window is a rectangle of dimensions WINDOW_WIDTH and WINDOW_HEIGHT.
            // Consider the inside rectangle obtained by removing a 100-width strip
            // from the window. We aim to place the camera on this rectangle.
            // The diagonals of this rectangle split its area into 4 quadrants.
            // The computation depends on which quadrant the destination of the camera is.
            let cam_destination;
            if (velocity.0.y / velocity.0.x).abs()
                > (WINDOW_HEIGHT - 200.0) / (WINDOW_WIDTH - 200.0)
            {
                if velocity.0.y > 0.0 {
                    // Upper quadrant
                    let y = (WINDOW_HEIGHT - 200.0) / 2.0;
                    cam_destination = transform.translation
                        + Vec3 {
                            x: y * velocity.0.x / velocity.0.y,
                            y,
                            z: CAMERA_Z - SPACESHIP_Z,
                        };
                } else {
                    // Lower quadrant
                    let y = -(WINDOW_HEIGHT - 200.0) / 2.0;
                    cam_destination = transform.translation
                        + Vec3 {
                            x: y * velocity.0.x / velocity.0.y,
                            y,
                            z: CAMERA_Z - SPACESHIP_Z,
                        };
                }
            } else {
                if velocity.0.x > 0.0 {
                    // Right quadrant
                    let x = (WINDOW_WIDTH - 200.0) / 2.0;
                    cam_destination = transform.translation
                        + Vec3 {
                            x,
                            y: velocity.0.y / velocity.0.x * x,
                            z: CAMERA_Z - SPACESHIP_Z,
                        };
                } else {
                    // Lower quadrant
                    let x = -(WINDOW_WIDTH - 200.0) / 2.0;
                    cam_destination = transform.translation
                        + Vec3 {
                            x,
                            y: velocity.0.y / velocity.0.x * x,
                            z: CAMERA_Z - SPACESHIP_Z,
                        };
                }
            }
            let cam_path = cam_destination - cam_transform.translation;
            cam_transform.translation += 0.05 * cam_path;
        } else {
            let direction = Vec3 {
                x: transform.translation.x - cam_transform.translation.x,
                y: transform.translation.y - cam_transform.translation.y,
                z: 0.0,
            };
            cam_transform.translation += 0.05 * direction;
        }
    }
}

// pub fn update_bullets(
//     mut commands: Commands,
//     mut query: Query<(&mut Transform, &Velocity, Entity), With<Fire>>,
// ) {
//     for (mut transform, velocity, entity) in query.iter_mut() {
//         transform.translation += velocity.0;
//         // transform.translation += Vec3 {
//         //     x: 4.0,
//         //     y: 0.0,
//         //     z: 0.0,
//         // };
//         if transform.translation.x > WINDOW_WIDTH / 2.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

pub fn move_fire(mut query: Query<(&mut Health, &mut Transform, &Velocity), With<Fire>>) {
    for (mut health, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x > WINDOW_WIDTH / 2.0
            || transform.translation.x < -WINDOW_WIDTH / 2.0
            || transform.translation.y > WINDOW_HEIGHT / 2.0
            || transform.translation.y < -WINDOW_HEIGHT / 2.0
        {
            health.0 = 0;
        }
    }
}

pub fn despawn_fire(mut commands: Commands, query: Query<(Entity, &Health), With<Fire>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_blast(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Parent>), With<Blast>>,
) {
    for (blast, mut health, parent) in query.iter_mut() {
        health.0 -= 1;
        if health.0 <= 0 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[blast]);
            }
        }
    }
}

pub fn despawn_blast(mut commands: Commands, query: Query<(Entity, &Health), With<Blast>>) {
    for (blast, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(blast).despawn();
        }
    }
}
