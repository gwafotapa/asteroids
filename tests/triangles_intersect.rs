use asteroids::collision::*;
use bevy::prelude::*;

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
const Q: Vec2 = Vec2 { x: 5.0, y: 1.0 };
const R: Vec2 = Vec2 { x: -3.0, y: 2.0 };
const S: Vec2 = Vec2 { x: -7.0, y: 0.0 };
const T: Vec2 = Vec2 { x: -6.0, y: -1.0 };
const U: Vec2 = Vec2 { x: -5.0, y: 2.0 };
const V: Vec2 = Vec2 { x: -3.0, y: 4.0 };
const W: Vec2 = Vec2 { x: 0.0, y: 8.0 };
const Z: Vec2 = Vec2 { x: -1.0, y: 4.0 };

const ABC: [Vec2; 3] = [A, B, C];
const DEF: [Vec2; 3] = [D, E, F];
const GHI: [Vec2; 3] = [G, H, I];
const JKL: [Vec2; 3] = [J, K, L];
const MNO: [Vec2; 3] = [M, N, O];
const PQR: [Vec2; 3] = [P, Q, R];
const STU: [Vec2; 3] = [S, T, U];
const VWZ: [Vec2; 3] = [V, W, Z];

#[test]
fn intersect_triangles() {
    assert!(detection::triangles_intersect(ABC, DEF).is_some());
    assert!(detection::triangles_intersect(ABC, GHI).is_none());
    assert!(detection::triangles_intersect(ABC, STU).is_none());
    assert!(detection::triangles_intersect(ABC, VWZ).is_none());
    assert!(detection::triangles_intersect(DEF, GHI).is_none());
    assert!(detection::triangles_intersect(DEF, JKL).is_none());
    assert!(detection::triangles_intersect(DEF, PQR).is_some());
    assert!(detection::triangles_intersect(DEF, VWZ).is_some());
    assert!(detection::triangles_intersect(GHI, JKL).is_some());
    assert!(detection::triangles_intersect([H, I, G], PQR).is_some());
    assert!(detection::triangles_intersect(GHI, MNO).is_none());
    assert!(detection::triangles_intersect(GHI, PQR).is_some());
    assert!(detection::triangles_intersect(GHI, STU).is_none());
    assert!(detection::triangles_intersect(GHI, [J, L, Q]).is_some());
    assert!(detection::triangles_intersect(JKL, PQR).is_none());
    assert!(detection::triangles_intersect(MNO, VWZ).is_none());
    assert!(detection::triangles_intersect(PQR, STU).is_none());
    assert!(detection::triangles_intersect(PQR, VWZ).is_none());
}
