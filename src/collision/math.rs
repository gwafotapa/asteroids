use bevy::prelude::*;

use super::RectangularEnvelop;

pub fn point_in_triangle_2d(p1: Vec3, p2: Vec3, p3: Vec3, p: Vec3) -> bool {
    let denominator = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
    let a = ((p2.y - p3.y) * (p.x - p3.x) + (p3.x - p2.x) * (p.y - p3.y)) / denominator;
    let b = ((p3.y - p1.y) * (p.x - p3.x) + (p1.x - p3.x) * (p.y - p3.y)) / denominator;
    let c = 1.0 - a - b;

    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}

pub fn rectangles_intersect(
    center1: Vec3,
    envelop1: RectangularEnvelop,
    center2: Vec3,
    envelop2: RectangularEnvelop,
) -> bool {
    let intersect_x = (center1.x - center2.x).abs() <= envelop1.half_x + envelop2.half_x;
    let intersect_y = (center1.y - center2.y).abs() <= envelop1.half_y + envelop2.half_y;

    return intersect_x && intersect_y;
}
