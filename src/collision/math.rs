use bevy::prelude::*;

use crate::collision::HitBox;

pub fn point_in_triangle_2d(p1: Vec3, p2: Vec3, p3: Vec3, p: Vec3) -> bool {
    let denominator = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
    let a = ((p2.y - p3.y) * (p.x - p3.x) + (p3.x - p2.x) * (p.y - p3.y)) / denominator;
    let b = ((p3.y - p1.y) * (p.x - p3.x) + (p1.x - p3.x) * (p.y - p3.y)) / denominator;
    let c = 1.0 - a - b;

    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}

pub fn rectangles_intersect(
    center1: Vec3,
    envelop1: HitBox,
    center2: Vec3,
    envelop2: HitBox,
) -> bool {
    let intersect_x = (center1.x - center2.x).abs() <= envelop1.half_x + envelop2.half_x;
    let intersect_y = (center1.y - center2.y).abs() <= envelop1.half_y + envelop2.half_y;

    return intersect_x && intersect_y;
}

// Determines if the circle of center o and radius r intersects the line segment [mn].
// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
fn circle_intersects_line_segment(m: Vec2, n: Vec2, o: Vec2, r: f32) -> bool {
    let mn = n - m;
    let om = m - o;

    let a = mn.dot(mn);
    let b = 2.0 * om.dot(mn);
    let c = om.dot(om) - r * r;

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        return false;
    }

    let t1 = (-b - delta.sqrt()) / (2.0 * a);
    let t2 = (-b + delta.sqrt()) / (2.0 * a);

    if (t1 >= 0.0 && t1 <= 1.0) {
        // t1 is the intersection.
        // Moreover, if t2 is another intersection, t1 is closer to point m than t2 since t1 < t2.
        // Geometrically, line segment [mn] either impales or pokes the circle.
        return true;
    }

    // Here t1 didn't intersect so we are either started inside the circle or completely past it
    if (t2 >= 0.0 && t2 <= 1.0) {
        // Geometrically, this is the called the "exit wound" case.
        return true;
    }

    // No intersection.
    // Line segment falls short or is past the circle or is completely inside.
    false
}

fn circle_intersects_triangle() {}
