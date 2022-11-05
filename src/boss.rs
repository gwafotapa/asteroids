use bevy::prelude::*;
use std::f32::consts::SQRT_2;

const INNER_RADIUS: f32 = 100.0;
pub const OUTER_RADIUS: f32 = INNER_RADIUS * SQRT_2;

/// Counter clockwise
pub const POLYGON: [Vec3; 16] = [
    Vec3 {
        x: -OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: INNER_RADIUS - OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS - OUTER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: -OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS - INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: -INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: INNER_RADIUS - OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: OUTER_RADIUS - INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: OUTER_RADIUS - INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: OUTER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: INNER_RADIUS - OUTER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: INNER_RADIUS,
        z: 0.0,
    },
    Vec3 {
        x: -INNER_RADIUS,
        y: OUTER_RADIUS - INNER_RADIUS,
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
