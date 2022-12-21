use bevy::prelude::*;

use crate::{Mass, Velocity};

pub fn compute(
    mut velocity1: Mut<Velocity>,
    mut velocity2: Mut<Velocity>,
    transform1: &Transform,
    transform2: &Transform,
    mass1: Mass,
    mass2: Mass,
) {
    let [v1, v2] = [velocity1.0.truncate(), velocity2.0.truncate()];
    let [x1, x2] = [
        transform1.translation.truncate(),
        transform2.translation.truncate(),
    ];
    let [m1, m2] = [mass1.0, mass2.0];
    let tmp = 2.0 * (v1 - v2).dot(x1 - x2) / ((m1 + m2) * (x1 - x2).dot(x1 - x2)) * (x1 - x2);
    let [w1, w2] = [v1 - m2 * tmp, v2 + m1 * tmp];
    println!("v1: {}\nw1: {}\nv2: {}\nw2: {}\n", v1, w1, v2, w2);
    velocity1.0 = w1.extend(velocity1.0.z);
    velocity2.0 = w2.extend(velocity2.0.z);
}
