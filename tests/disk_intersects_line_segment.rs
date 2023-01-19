use asteroids::*;
use bevy::prelude::*;

const A: Vec2 = Vec2 { x: -5.0, y: 1.0 };
const B: Vec2 = Vec2 { x: -4.0, y: -3.0 };
const C: Vec2 = Vec2 { x: -2.0, y: 2.0 };
// const D: Vec2 = Vec2 { x: -4.0, y: 2.0 };
const E: Vec2 = Vec2 { x: -5.0, y: -4.0 };
// const F: Vec2 = Vec2 { x: -3.0, y: -4.0 };
const G: Vec2 = Vec2 { x: -7.0, y: 4.0 };
const H: Vec2 = Vec2 { x: 1.0, y: 2.0 };
const I: Vec2 = Vec2 { x: 2.0, y: -1.0 };
const J: Vec2 = Vec2 { x: 6.0, y: -1.0 };
const K: Vec2 = Vec2 { x: -6.0, y: -4.0 };
const L: Vec2 = Vec2 { x: -5.0, y: -5.0 };
const M: Vec2 = Vec2 { x: 2.0, y: -2.0 };
// const N: Vec2 = Vec2 { x: 2.0, y: 1.0 };
const O: Vec2 = Vec2 { x: -11.0, y: 3.0 };
const P: Vec2 = Vec2 { x: -7.0, y: 2.0 };

#[test]
fn intersect_disk_and_line_segment() {
    assert!(detection::disk_intersects_line_segment(E, 2.0, A, B).is_some());
    assert!(detection::disk_intersects_line_segment(C, 2.0, A, B).is_none());
    assert!(detection::disk_intersects_line_segment(E, 2.0, K, L).is_some());
    assert!(detection::disk_intersects_line_segment(C, 2.0, G, H).is_some());
    assert!(detection::disk_intersects_line_segment(M, 3.0, G, H).is_none());
    assert!(detection::disk_intersects_line_segment(M, 3.0, O, P).is_none());
    assert!(detection::disk_intersects_line_segment(M, 3.0, I, J).is_some());
}
