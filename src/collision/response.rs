use bevy::prelude::*;

use crate::{collision::detection::Contact, AngularVelocity, Mass, MomentOfInertia, Velocity};

const RESTITUTION: f32 = 1.0;
// https://en.wikipedia.org/wiki/Elastic_collision
// https://fotino.me/2d-rigid-body-collision-response/
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
    contact: Contact,
) {
    // let [x1, x2] = [
    //     transform1.translation.truncate(),
    //     transform2.translation.truncate(),
    // ];
    // println!(
    //     "t1: {}\nt2: {}\ncontact: {}\nnormal: {}",
    //     transform1.translation, transform2.translation, contact.point, contact.normal
    // );
    let [m1, m2] = [mass1.0, mass2.0];
    let [i1, i2] = [moment_of_inertia1.0, moment_of_inertia2.0];
    let [v1, v2] = [velocity1.0, velocity2.0];
    let [w1, w2] = [angular_velocity1.0, angular_velocity2.0];

    let r1 = (contact.point - transform1.translation.truncate()).extend(0.0);
    let r2 = (contact.point - transform2.translation.truncate()).extend(0.0);
    let n = contact.normal.extend(0.0);
    let j = -(1.0 + RESTITUTION)
        * (v1.dot(n) - v2.dot(n) + w1 * (r1.cross(n)).z - w2 * (r2.cross(n)).z)
        / (1.0 / m1 + 1.0 / m2 + (r1.cross(n)).z.powi(2) / i1 + (r2.cross(n)).z.powi(2) / i2);
    println!("j: {}", j);

    let r1n = (contact.point - transform1.translation.truncate())
        .perp()
        .extend(0.0);
    let r2n = (contact.point - transform2.translation.truncate())
        .perp()
        .extend(0.0);
    let j2 = -(1.0 + RESTITUTION) * (v1 + w1 * r1n - v2 - w2 * r2n).dot(n)
        / (1.0 / m1 + 1.0 / m2 + (r1n.dot(n)).powi(2) / i1 + (r2n.dot(n)).powi(2) / i2);
    // println!("j: {}\nj2: {}\n", j, j2);

    velocity1.0 = v1 + j / m1 * n;
    velocity2.0 = v2 - j / m2 * n;

    angular_velocity1.0 = w1 + j / i1 * r1.cross(n).z;
    angular_velocity2.0 = w2 - j / i2 * r2.cross(n).z;

    // angular_velocity1.0 = w1 + j / i1 * r1n.dot(n);
    // angular_velocity2.0 = w2 - j / i2 * r2n.dot(n);
    // angular_velocity1.0 = w1 - j / i1 * r1n.dot(n);
    // angular_velocity2.0 = w2 + j / i2 * r2n.dot(n);

    // let tmp = 2.0 * (u1 - u2).dot(x1 - x2) / ((m1 + m2) * (x1 - x2).dot(x1 - x2)) * (x1 - x2);
    // let [v1, v2] = [u1 - m2 * tmp, u2 + m1 * tmp];
    // println!("u1: {}\nv1: {}\nu2: {}\nv2: {}\n", u1, w1, u2, v2);
    // velocity1.0 = v1.extend(velocity1.0.z);
    // velocity2.0 = v2.extend(velocity2.0.z);

    // angular_velocity1.0 = ((i1 - i2) * w1 + 2.0 * i2 * w2) / (i1 + i2);
    // angular_velocity2.0 = ((i2 - i1) * w2 + 2.0 * i1 * w1) / (i1 + i2);
}
