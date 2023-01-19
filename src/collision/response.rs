// https://en.wikipedia.org/wiki/Elastic_collision
// https://fotino.me/2d-rigid-body-collision-response
// https://www.chrishecker.com/Rigid_Body_Dynamics
use bevy::prelude::*;

use crate::{
    collision::detection::Contact,
    component::{AngularVelocity, Mass, MomentOfInertia, Velocity},
};

pub fn compute_velocities(
    velocity1: &mut Velocity,
    velocity2: &mut Velocity,
    angular_velocity1: &mut AngularVelocity,
    angular_velocity2: &mut AngularVelocity,
    transform1: Transform,
    transform2: Transform,
    mass1: Mass,
    mass2: Mass,
    moment_of_inertia1: MomentOfInertia,
    moment_of_inertia2: MomentOfInertia,
    contact: Contact,
) {
    let [m1, m2] = [mass1.0, mass2.0];
    let [i1, i2] = [moment_of_inertia1.0, moment_of_inertia2.0];
    let [v1, v2] = [velocity1.0, velocity2.0];
    let [w1, w2] = [angular_velocity1.0, angular_velocity2.0];
    let n = contact.normal.extend(0.0);
    let r1 = (contact.point - transform1.translation.truncate()).extend(0.0);
    let r2 = (contact.point - transform2.translation.truncate()).extend(0.0);
    const RESTITUTION: f32 = 1.0;

    let j = -(1.0 + RESTITUTION) * ((v1 - v2).dot(n) + (w1 * r1 - w2 * r2).cross(n).z)
        / (1.0 / m1 + 1.0 / m2 + (r1.cross(n)).z.powi(2) / i1 + (r2.cross(n)).z.powi(2) / i2);

    velocity1.0 += j / m1 * n;
    velocity2.0 -= j / m2 * n;

    angular_velocity1.0 += j / i1 * r1.cross(n).z;
    angular_velocity2.0 -= j / i2 * r2.cross(n).z;
}
