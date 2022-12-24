use bevy::prelude::*;

use crate::{AngularVelocity, Mass, Velocity};

// https://en.wikipedia.org/wiki/Elastic_collision
pub fn compute(
    transform1: &Transform,
    transform2: &Transform,
    mass1: Mass,
    mass2: Mass,
    mut velocity1: &mut Velocity,
    mut velocity2: &mut Velocity,
    angular_velocity1: Option<&mut AngularVelocity>,
    angular_velocity2: Option<&mut AngularVelocity>,
) {
    let [v1, v2] = [velocity1.0.truncate(), velocity2.0.truncate()];
    let [x1, x2] = [
        transform1.translation.truncate(),
        transform2.translation.truncate(),
    ];
    let [m1, m2] = [mass1.0, mass2.0];
    let tmp = 2.0 * (v1 - v2).dot(x1 - x2) / ((m1 + m2) * (x1 - x2).dot(x1 - x2)) * (x1 - x2);
    let [w1, w2] = [v1 - m2 * tmp, v2 + m1 * tmp];
    // println!("v1: {}\nw1: {}\nv2: {}\nw2: {}\n", v1, w1, v2, w2);
    velocity1.0 = w1.extend(velocity1.0.z);
    velocity2.0 = w2.extend(velocity2.0.z);

    angular_velocity1.map(|w| w.0 = -w.0);
    angular_velocity2.map(|w| w.0 = -w.0);
}
