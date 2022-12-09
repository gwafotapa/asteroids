use bevy::prelude::*;

use crate::{
    spaceship::{Spaceship, SPACESHIP_Z},
    Velocity, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const CAMERA_Z: f32 = 1000.0;
const CAMERA_SPEED: f32 = 0.05;
const CAMERA_REAR_GAP: f32 = 100.0;

#[derive(Component, Eq, PartialEq)]
pub enum CameraPositioning {
    Center,
    Rear,
}

pub fn spawn(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            // transform: Transform::from_xyz(MAP_CENTER_X, MAP_CENTER_Y, CAMERA_Z),
            // transform: Transform::from_xyz(0.0, 0.0, CAMERA_Z),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, CAMERA_Z),
            ..default()
        })
        .insert(CameraPositioning::Center)
        .insert(UiCameraConfig { show_ui: false });
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    mut query_camera: Query<(&mut CameraPositioning, &mut Transform), With<Camera>>,
    query_spaceship: Query<(&Transform, &Velocity), (With<Spaceship>, Without<Camera>)>,
) {
    if let Ok((s_transform, s_velocity)) = query_spaceship.get_single() {
        let (mut camera_positioning, mut c_transform) = query_camera.get_single_mut().unwrap();
        c_transform.translation += s_velocity.0;

        if keys.any_just_pressed([KeyCode::Space, KeyCode::C]) {
            *camera_positioning = if *camera_positioning == CameraPositioning::Center {
                CameraPositioning::Rear
            } else {
                CameraPositioning::Center
            };
        }

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
