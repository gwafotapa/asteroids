use bevy::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use asteroids::collision::math;

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

const INPUTS: [(Vec2, f32, Vec2, Vec2); 7] = [
    (E, 2.0, A, B),
    (C, 2.0, A, B),
    (E, 2.0, K, L),
    (C, 2.0, G, H),
    (M, 3.0, G, H),
    (M, 3.0, O, P),
    (M, 3.0, I, J),
];

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("disk_intersects_line_segment");
    let mut count = 0;
    for input in INPUTS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("input #{}", count)),
            &input,
            |b, &(c, r, m, n)| b.iter(|| math::disk_intersects_line_segment(c, r, m, n)),
        );
        count += 1;
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
