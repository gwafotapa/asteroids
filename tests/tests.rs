use asteroids::*;
use bevy::prelude::*;

const O: Vec3 = Vec3::ZERO;
const A: Vec3 = Vec3 {
    x: -2.0,
    y: -1.0,
    z: 0.0,
};
const B: Vec3 = Vec3 {
    x: -1.0,
    y: 5.0,
    z: 0.0,
};
const C: Vec3 = Vec3 {
    x: 9.0,
    y: 2.0,
    z: 0.0,
};
const D: Vec3 = Vec3 {
    x: -1.0,
    y: 4.0,
    z: 0.0,
};
const E: Vec3 = Vec3 {
    x: -2.0,
    y: 0.0,
    z: 0.0,
};
const F: Vec3 = Vec3 {
    x: 5.0,
    y: 1.0,
    z: 0.0,
};
const G: Vec3 = Vec3 {
    x: 8.0,
    y: 2.0,
    z: 0.0,
};
const H: Vec3 = Vec3 {
    x: 2.0,
    y: 4.0,
    z: 0.0,
};
const I: Vec3 = Vec3 {
    x: 6.0,
    y: 3.0,
    z: 0.0,
};
const J: Vec3 = Vec3 {
    x: -4.0,
    y: 3.0,
    z: 0.0,
};
const K: Vec3 = Vec3 {
    x: -2.0,
    y: -6.0,
    z: 0.0,
};
const L: Vec3 = Vec3 {
    x: 11.0,
    y: -9.0,
    z: 0.0,
};
const M: Vec3 = Vec3 {
    x: 8.0,
    y: 6.0,
    z: 0.0,
};
const N: Vec3 = Vec3 {
    x: 2.0,
    y: 0.0,
    z: 0.0,
};

#[test]
fn point_in_triangle_2d() {
    assert!(is_point_in_triangle_2d(A, B, C, D));
    assert!(!is_point_in_triangle_2d(A, B, C, E));
    assert!(is_point_in_triangle_2d(A, B, C, F));
    assert!(is_point_in_triangle_2d(A, B, C, G));
    assert!(is_point_in_triangle_2d(A, B, C, H));
    assert!(!is_point_in_triangle_2d(A, B, C, I));
    assert!(!is_point_in_triangle_2d(A, B, C, J));
    assert!(!is_point_in_triangle_2d(A, B, C, K));
    assert!(!is_point_in_triangle_2d(A, B, C, L));
    assert!(!is_point_in_triangle_2d(A, B, C, M));
    assert!(!is_point_in_triangle_2d(A, B, C, N));
    assert!(is_point_in_triangle_2d(A, B, C, O));
    assert!(is_point_in_triangle_2d(J, M, K, A));
    assert!(is_point_in_triangle_2d(J, M, K, N));
    assert!(is_point_in_triangle_2d(J, M, K, H));
    assert!(!is_point_in_triangle_2d(J, M, K, F));
    assert!(!is_point_in_triangle_2d(J, M, K, I));
    assert!(!is_point_in_triangle_2d(J, M, K, D));
}
