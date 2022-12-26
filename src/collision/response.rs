use bevy::prelude::*;

use crate::{AngularVelocity, Mass, MomentOfInertia, Velocity};

// https://en.wikipedia.org/wiki/Elastic_collision
pub fn compute(
    transform1: &Transform,
    transform2: &Transform,
    mass1: Mass,
    mass2: Mass,
    moment_of_inertia1: MomentOfInertia,
    moment_of_inertia2: MomentOfInertia,
    mut velocity1: &mut Velocity,
    mut velocity2: &mut Velocity,
    angular_velocity1: &mut AngularVelocity,
    angular_velocity2: &mut AngularVelocity,
) {
    let [x1, x2] = [
        transform1.translation.truncate(),
        transform2.translation.truncate(),
    ];
    let [m1, m2] = [mass1.0, mass2.0];
    let [i1, i2] = [moment_of_inertia1.0, moment_of_inertia2.0];
    let [u1, u2] = [velocity1.0.truncate(), velocity2.0.truncate()];
    let [w1, w2] = [angular_velocity1.0, angular_velocity2.0];

    let tmp = 2.0 * (u1 - u2).dot(x1 - x2) / ((m1 + m2) * (x1 - x2).dot(x1 - x2)) * (x1 - x2);
    let [v1, v2] = [u1 - m2 * tmp, u2 + m1 * tmp];
    // println!("u1: {}\nv1: {}\nu2: {}\nv2: {}\n", u1, w1, u2, v2);
    velocity1.0 = v1.extend(velocity1.0.z);
    velocity2.0 = v2.extend(velocity2.0.z);

    angular_velocity1.0 = ((i1 - i2) * w1 + 2.0 * i2 * w2) / (i1 + i2);
    angular_velocity2.0 = ((i2 - i1) * w2 + 2.0 * i1 * w1) / (i1 + i2);
}
