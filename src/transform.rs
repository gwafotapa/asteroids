use bevy::prelude::*;

use crate::{AngularVelocity, Velocity};

pub fn global_of(child: Transform, parent: Transform) -> Transform {
    Transform {
        translation: parent.transform_point(child.translation),
        rotation: parent.rotation * child.rotation,
        scale: parent.scale * child.scale,
    }
}

pub fn at(
    time: f32,
    transform: Transform,
    velocity: Velocity,
    angular_velocity: AngularVelocity,
) -> Transform {
    Transform {
        translation: transform.translation + velocity.0 * time,
        rotation: transform.rotation * Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time),
        scale: transform.scale,
    }
}
