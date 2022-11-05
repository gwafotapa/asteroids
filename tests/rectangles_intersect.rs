use asteroids::*;
use bevy::prelude::*;

const A: Vec3 = Vec3 {
    x: -8.0,
    y: 7.0,
    z: 0.0,
};
const RECT_A: RectangularEnvelop = RectangularEnvelop {
    half_x: 2.0,
    half_y: 1.0,
};
const B: Vec3 = Vec3 {
    x: -4.5,
    y: 3.5,
    z: 0.0,
};
const RECT_B: RectangularEnvelop = RectangularEnvelop {
    half_x: 2.5,
    half_y: 3.5,
};
const C: Vec3 = Vec3 {
    x: -2.0,
    y: 4.5,
    z: 0.0,
};
const RECT_C: RectangularEnvelop = RectangularEnvelop {
    half_x: 1.0,
    half_y: 0.5,
};
const D: Vec3 = Vec3 {
    x: -6.0,
    y: -2.5,
    z: 0.0,
};
const RECT_D: RectangularEnvelop = RectangularEnvelop {
    half_x: 2.0,
    half_y: 1.0,
};
const E: Vec3 = Vec3 {
    x: -1.0,
    y: -2.5,
    z: 0.0,
};
const RECT_E: RectangularEnvelop = RectangularEnvelop {
    half_x: 3.0,
    half_y: 0.5,
};
const F: Vec3 = Vec3 {
    x: -0.5,
    y: -2.5,
    z: 0.0,
};
const RECT_F: RectangularEnvelop = RectangularEnvelop {
    half_x: 0.5,
    half_y: 1.5,
};
const G: Vec3 = Vec3 {
    x: 4.0,
    y: 0.0,
    z: 0.0,
};
const RECT_G: RectangularEnvelop = RectangularEnvelop {
    half_x: 1.0,
    half_y: 3.0,
};
const H: Vec3 = Vec3 {
    x: 3.5,
    y: 2.5,
    z: 0.0,
};
const RECT_H: RectangularEnvelop = RectangularEnvelop {
    half_x: 2.5,
    half_y: 1.5,
};

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
    assert!(!rectangles_intersect(D, RECT_D, E, RECT_E));
    assert!(rectangles_intersect(E, RECT_E, F, RECT_F));
    assert!(!rectangles_intersect(F, RECT_F, G, RECT_G));
    assert!(!rectangles_intersect(E, RECT_E, G, RECT_G));
    assert!(!rectangles_intersect(E, RECT_E, H, RECT_H));
    assert!(rectangles_intersect(G, RECT_G, H, RECT_H));
    assert!(!rectangles_intersect(H, RECT_H, A, RECT_A));
    assert!(!rectangles_intersect(H, RECT_H, B, RECT_B));
    assert!(!rectangles_intersect(H, RECT_H, C, RECT_C));
}