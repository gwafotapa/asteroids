use asteroids::collision::math::point_in_triangle;
use bevy::prelude::*;

const O: Vec2 = Vec2::ZERO;
const A: Vec2 = Vec2 { x: -2.0, y: -1.0 };
const B: Vec2 = Vec2 { x: -1.0, y: 5.0 };
const C: Vec2 = Vec2 { x: 9.0, y: 2.0 };
const D: Vec2 = Vec2 { x: -1.0, y: 4.0 };
const E: Vec2 = Vec2 { x: -2.0, y: 0.0 };
const F: Vec2 = Vec2 { x: 5.0, y: 1.0 };
const G: Vec2 = Vec2 { x: 8.0, y: 2.0 };
const H: Vec2 = Vec2 { x: 2.0, y: 4.0 };
const I: Vec2 = Vec2 { x: 6.0, y: 3.0 };
const J: Vec2 = Vec2 { x: -4.0, y: 3.0 };
const K: Vec2 = Vec2 { x: -2.0, y: -6.0 };
const L: Vec2 = Vec2 { x: 11.0, y: -9.0 };
const M: Vec2 = Vec2 { x: 8.0, y: 6.0 };
const N: Vec2 = Vec2 { x: 2.0, y: 0.0 };

#[test]
fn points_in_triangles_2d() {
    assert!(point_in_triangle(A, B, C, D));
    assert!(!point_in_triangle(A, B, C, E));
    assert!(point_in_triangle(A, B, C, F));
    assert!(point_in_triangle(A, B, C, G));
    assert!(point_in_triangle(A, B, C, H));
    assert!(!point_in_triangle(A, B, C, I));
    assert!(!point_in_triangle(A, B, C, J));
    assert!(!point_in_triangle(A, B, C, K));
    assert!(!point_in_triangle(A, B, C, L));
    assert!(!point_in_triangle(A, B, C, M));
    assert!(!point_in_triangle(A, B, C, N));
    assert!(point_in_triangle(A, B, C, O));
    assert!(point_in_triangle(J, M, K, A));
    assert!(point_in_triangle(J, M, K, N));
    assert!(point_in_triangle(J, M, K, H));
    assert!(!point_in_triangle(J, M, K, F));
    assert!(!point_in_triangle(J, M, K, I));
    assert!(!point_in_triangle(J, M, K, D));
}
