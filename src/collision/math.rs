use bevy::prelude::*;

use crate::collision::HitBox;

// Determines if point p is in the rectangle of center c, half width x and half height y
pub fn point_in_rectangle(p: Vec2, c: Vec2, x: f32, y: f32) -> bool {
    p.x >= c.x - x && p.x <= c.x + x && p.y >= c.y - y && p.y <= c.y + y
}

// Determines if point p is in the triangle (p1 p2 p3)
pub fn point_in_triangle(p: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> bool {
    let denominator = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
    let a = ((p2.y - p3.y) * (p.x - p3.x) + (p3.x - p2.x) * (p.y - p3.y)) / denominator;
    let b = ((p3.y - p1.y) * (p.x - p3.x) + (p1.x - p3.x) * (p.y - p3.y)) / denominator;
    let c = 1.0 - a - b;

    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}

pub fn rectangles_intersect(
    position1: Vec2,
    hitbox1: HitBox,
    position2: Vec2,
    hitbox2: HitBox,
) -> bool {
    let intersect_x = (position1.x - position2.x).abs() <= hitbox1.half_x + hitbox2.half_x;
    let intersect_y = (position1.y - position2.y).abs() <= hitbox1.half_y + hitbox2.half_y;

    return intersect_x && intersect_y;
}

// Determines if the circle of center o and radius r intersects the line segment [mn].
// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
pub fn circle_intersects_line_segment(m: Vec2, n: Vec2, o: Vec2, r: f32) -> bool {
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

    if t1 >= 0.0 && t1 <= 1.0 {
        // t1 is the intersection.
        // Moreover, if t2 is another intersection, t1 is closer to point m than t2 since t1 < t2.
        // Geometrically, line segment [mn] either impales or pokes the circle.
        return true;
    }

    // Here t1 didn't intersect so we are either started inside the circle or completely past it
    if t2 >= 0.0 && t2 <= 1.0 {
        // Geometrically, this is the called the "exit wound" case.
        return true;
    }

    // No intersection.
    // Line segment falls short or is past the circle or is completely inside.
    false
}

// Determines if the disk of center o and radius r intersects the triangle abc
pub fn circle_intersects_triangle(a: Vec2, b: Vec2, c: Vec2, o: Vec2, r: f32) -> bool {
    a.distance(o) < r
        || circle_intersects_line_segment(a, b, o, r)
        || circle_intersects_line_segment(b, c, o, r)
        || circle_intersects_line_segment(c, a, o, r)
        || point_in_triangle(o, a, b, c)
}

// pub fn triangle_hitbox(a: Vec2, b: Vec2, c: Vec2) -> HitBox {
//     let x1 = a.x.min(b.x).min(c.x);
//     let x2 = a.x.max(b.x).max(c.x);
//     let y1 = a.y.min(b.y).min(c.y);
//     let y2 = a.y.max(b.y).max(c.y);

//     HitBox {
//         center_x: (x1 + x2) / 2.0,
//         center_y: (y1 + y2) / 2.0,
//         half_x: (x2 - x1) / 2.0,
//         half_y: (y2 - y1) / 2.0,
//     }
// }
