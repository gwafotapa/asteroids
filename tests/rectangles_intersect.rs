use asteroids::collision::{math::rectangles_intersect, Aabb};
use bevy::prelude::*;

const A: Vec2 = Vec2 { x: -8.0, y: 7.0 };
const RECT_A: Aabb = Aabb { hw: 2.0, hh: 1.0 };
const B: Vec2 = Vec2 { x: -4.5, y: 3.5 };
const RECT_B: Aabb = Aabb { hw: 2.5, hh: 3.5 };
const C: Vec2 = Vec2 { x: -2.0, y: 4.5 };
const RECT_C: Aabb = Aabb { hw: 1.0, hh: 0.5 };
const D: Vec2 = Vec2 { x: -6.0, y: -2.5 };
const RECT_D: Aabb = Aabb { hw: 2.0, hh: 1.0 };
const E: Vec2 = Vec2 { x: -1.0, y: -2.5 };
const RECT_E: Aabb = Aabb { hw: 3.0, hh: 0.5 };
const F: Vec2 = Vec2 { x: -0.5, y: -2.5 };
const RECT_F: Aabb = Aabb { hw: 0.5, hh: 1.5 };
const G: Vec2 = Vec2 { x: 4.0, y: 0.0 };
const RECT_G: Aabb = Aabb { hw: 1.0, hh: 3.0 };
const H: Vec2 = Vec2 { x: 3.5, y: 2.5 };
const RECT_H: Aabb = Aabb { hw: 2.5, hh: 1.5 };

#[test]
fn intersect_rectangles() {
    assert!(rectangles_intersect(A, RECT_A, A, RECT_A));
    assert!(rectangles_intersect(A, RECT_A, B, RECT_B));
    assert!(!rectangles_intersect(A, RECT_A, C, RECT_C));
    assert!(!rectangles_intersect(A, RECT_A, D, RECT_D));
    assert!(!rectangles_intersect(A, RECT_A, E, RECT_E));
    assert!(rectangles_intersect(B, RECT_B, C, RECT_C));
    assert!(!rectangles_intersect(B, RECT_B, D, RECT_D));
    assert!(!rectangles_intersect(B, RECT_B, F, RECT_F));
    assert!(!rectangles_intersect(B, RECT_B, G, RECT_G));
    assert!(!rectangles_intersect(B, RECT_B, H, RECT_H));
    assert!(rectangles_intersect(C, RECT_C, B, RECT_B));
    assert!(rectangles_intersect(E, RECT_E, F, RECT_F));
    assert!(!rectangles_intersect(F, RECT_F, G, RECT_G));
    assert!(!rectangles_intersect(E, RECT_E, G, RECT_G));
    assert!(!rectangles_intersect(E, RECT_E, H, RECT_H));
    assert!(rectangles_intersect(G, RECT_G, H, RECT_H));
    assert!(!rectangles_intersect(H, RECT_H, A, RECT_A));
    assert!(!rectangles_intersect(H, RECT_H, B, RECT_B));
    assert!(!rectangles_intersect(H, RECT_H, C, RECT_C));
}
