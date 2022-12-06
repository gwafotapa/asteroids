use asteroids::collision::math::point_in_triangle;
use bevy::prelude::*;

const O: Vec2 = Vec2::ZERO;
const A: Vec2 = Vec2 { x: -2.0, y: -1.0 };
const C: Vec2 = Vec2 { x: -1.0, y: 5.0 };
const B: Vec2 = Vec2 { x: 9.0, y: 2.0 };
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
fn points_in_triangles() {
    assert!(point_in_triangle(D, [A, B, C]));
    assert!(!point_in_triangle(E, [A, B, C]));
    assert!(point_in_triangle(F, [A, B, C]));
    assert!(point_in_triangle(G, [A, B, C]));
    assert!(point_in_triangle(H, [A, B, C]));
    assert!(!point_in_triangle(I, [A, B, C]));
    assert!(!point_in_triangle(J, [A, B, C]));
    assert!(!point_in_triangle(K, [A, B, C]));
    assert!(!point_in_triangle(L, [A, B, C]));
    assert!(!point_in_triangle(M, [A, B, C]));
    assert!(!point_in_triangle(N, [A, B, C]));
    assert!(point_in_triangle(O, [A, B, C]));
    assert!(point_in_triangle(A, [J, K, M]));
    assert!(point_in_triangle(N, [J, K, M]));
    assert!(point_in_triangle(H, [J, K, M]));
    assert!(!point_in_triangle(F, [J, K, M]));
    assert!(!point_in_triangle(I, [J, K, M]));
    assert!(!point_in_triangle(D, [J, K, M]));
}
