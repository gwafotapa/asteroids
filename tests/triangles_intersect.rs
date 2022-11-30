// use asteroids::collision;
use bevy::prelude::*;

pub type Triangle = [Vec2; 3];

const A: Vec2 = Vec2 { x: -9.0, y: 6.0 };
const B: Vec2 = Vec2 { x: -7.0, y: 2.0 };
const C: Vec2 = Vec2 { x: -2.0, y: 8.0 };
const D: Vec2 = Vec2 { x: -5.0, y: 5.0 };
const E: Vec2 = Vec2 { x: -4.0, y: 1.0 };
const F: Vec2 = Vec2 { x: 2.0, y: 5.0 };
const G: Vec2 = Vec2 { x: -5.0, y: 4.0 };
const H: Vec2 = Vec2 { x: -10.0, y: -2.0 };
const I: Vec2 = Vec2 { x: -3.0, y: -4.0 };
const J: Vec2 = Vec2 { x: -11.0, y: 4.0 };
const K: Vec2 = Vec2 { x: -8.0, y: 0.0 };
const L: Vec2 = Vec2 { x: -12.0, y: 3.0 };
const M: Vec2 = Vec2 { x: -7.0, y: -3.0 };
const N: Vec2 = Vec2 { x: -10.0, y: -6.0 };
const O: Vec2 = Vec2 { x: -4.0, y: -6.0 };
const P: Vec2 = Vec2 { x: -4.0, y: -1.0 };
const Q: Vec2 = Vec2 { x: -3.0, y: 2.0 };
const R: Vec2 = Vec2 { x: 5.0, y: 1.0 };
const S: Vec2 = Vec2 { x: -7.0, y: 0.0 };
const T: Vec2 = Vec2 { x: -6.0, y: -1.0 };
const U: Vec2 = Vec2 { x: -5.0, y: 2.0 };
const V: Vec2 = Vec2 { x: -3.0, y: 4.0 };
const W: Vec2 = Vec2 { x: 0.0, y: 8.0 };
const Z: Vec2 = Vec2 { x: -1.0, y: 4.0 };

const ABC: Triangle = [A, B, C];
const DEF: Triangle = [D, E, F];
const GHI: Triangle = [G, H, I];
const JKL: Triangle = [J, K, L];
const MNO: Triangle = [M, N, O];
const PQR: Triangle = [P, Q, R];
const STU: Triangle = [S, T, U];
const VWZ: Triangle = [V, W, Z];

fn triangles_intersect(t1: Triangle, t2: Triangle) -> bool {
    false
}

#[test]
fn intersect_triangles() {
    assert!(triangles_intersect(ABC, DEF));
    assert!(!triangles_intersect(ABC, GHI));
    assert!(!triangles_intersect(ABC, STU));
    assert!(!triangles_intersect(ABC, VWZ));
    assert!(!triangles_intersect(DEF, GHI));
    assert!(!triangles_intersect(DEF, JKL));
    assert!(triangles_intersect(DEF, PQR));
    assert!(triangles_intersect(DEF, VWZ));
    assert!(triangles_intersect(GHI, JKL));
    assert!(!triangles_intersect(GHI, MNO));
    assert!(triangles_intersect(GHI, PQR));
    assert!(triangles_intersect(GHI, STU));
    assert!(triangles_intersect(GHI, [J, L, R]));
    assert!(!triangles_intersect(JKL, PQR));
    assert!(!triangles_intersect(MNO, VWZ));
    assert!(!triangles_intersect(PQR, STU));
    assert!(!triangles_intersect(PQR, VWZ));
}
