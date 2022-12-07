use bevy::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use asteroids::collision::math;

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

const INPUTS: [(Vec2, [Vec2; 3]); 18] = [
    (D, [A, B, C]),
    (E, [A, B, C]),
    (F, [A, B, C]),
    (G, [A, B, C]),
    (H, [A, B, C]),
    (I, [A, B, C]),
    (J, [A, B, C]),
    (K, [A, B, C]),
    (L, [A, B, C]),
    (M, [A, B, C]),
    (N, [A, B, C]),
    (O, [A, B, C]),
    (A, [J, K, M]),
    (N, [J, K, M]),
    (H, [J, K, M]),
    (F, [J, K, M]),
    (I, [J, K, M]),
    (D, [J, K, M]),
];

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("point_in_triangle");
    let mut count = 0;
    for input in INPUTS {
        group.bench_with_input(
            BenchmarkId::new("original", format!("input #{}", count)),
            &input,
            |b, &(p, t)| b.iter(|| math::point_in_triangle(p, t)),
        );
        group.bench_with_input(
            BenchmarkId::new("alternative", format!("input #{}", count)),
            &input,
            |b, &(p, t)| b.iter(|| math::point_in_triangle_bis(p, t)),
        );
        count += 1;
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
