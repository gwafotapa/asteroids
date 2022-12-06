use asteroids::collision::math::point_in_triangle_1;
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
    assert!(point_in_triangle_1(D, [A, B, C]));
    assert!(!point_in_triangle_1(E, [A, B, C]));
    assert!(point_in_triangle_1(F, [A, B, C]));
    assert!(point_in_triangle_1(G, [A, B, C]));
    assert!(point_in_triangle_1(H, [A, B, C]));
    assert!(!point_in_triangle_1(I, [A, B, C]));
    assert!(!point_in_triangle_1(J, [A, B, C]));
    assert!(!point_in_triangle_1(K, [A, B, C]));
    assert!(!point_in_triangle_1(L, [A, B, C]));
    assert!(!point_in_triangle_1(M, [A, B, C]));
    assert!(!point_in_triangle_1(N, [A, B, C]));
    assert!(point_in_triangle_1(O, [A, B, C]));
    assert!(point_in_triangle_1(A, [J, K, M]));
    assert!(point_in_triangle_1(N, [J, K, M]));
    assert!(point_in_triangle_1(H, [J, K, M]));
    assert!(!point_in_triangle_1(F, [J, K, M]));
    assert!(!point_in_triangle_1(I, [J, K, M]));
    assert!(!point_in_triangle_1(D, [J, K, M]));
}
