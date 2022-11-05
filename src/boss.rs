use bevy::prelude::*;
use std::f32::consts::SQRT_2;

const BOSS_INNER_RADIUS: f32 = 100.0;
const BOSS_OUTER_RADIUS: f32 = BOSS_INNER_RADIUS * SQRT_2;

/// Counter clockwise
pub const BOSS_POLYGON: [Vec3; 16] = [
    Vec3 {
        x: -BOSS_OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: -BOSS_INNER_RADIUS,
        y: BOSS_INNER_RADIUS - BOSS_OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -BOSS_INNER_RADIUS,
        y: -BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS - BOSS_OUTER_RADIUS,
        y: -BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: -BOSS_OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_OUTER_RADIUS - BOSS_INNER_RADIUS,
        y: -BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS,
        y: -BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS,
        y: BOSS_INNER_RADIUS - BOSS_OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS,
        y: BOSS_OUTER_RADIUS - BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS,
        y: BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_OUTER_RADIUS - BOSS_INNER_RADIUS,
        y: BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: BOSS_OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: BOSS_INNER_RADIUS - BOSS_OUTER_RADIUS,
        y: BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -BOSS_INNER_RADIUS,
        y: BOSS_INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -BOSS_INNER_RADIUS,
        y: BOSS_OUTER_RADIUS - BOSS_INNER_RADIUS,
        z: 0.0,
    },
];

pub fn create_triangle_list_from_polygon(polygon: &[Vec3], center: Vec3) -> Vec<Vec3> {
    let mut triangle_list = Vec::new();
    let mut iter = polygon.iter();
    let mut p1 = iter.next();
    let mut p2 = iter.next();
    let p0 = p1;
    while p2.is_some() {
        triangle_list.push(center);
        triangle_list.push(*p1.unwrap());
        triangle_list.push(*p2.unwrap());
        p1 = p2;
        p2 = iter.next();
    }
    triangle_list.push(center);
    triangle_list.push(*p1.unwrap());
    triangle_list.push(*p0.unwrap());

    triangle_list
}
