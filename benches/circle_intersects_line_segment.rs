use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use asteroids::collision::math;

const A: Vec2 = Vec2 { x: -2.0, y: -1.0 };
const B: Vec2 = Vec2 { x: 9.0, y: 2.0 };
const C: Vec2 = Vec2 { x: -1.0, y: 5.0 };

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("circle intersects line segment", |b| {
        b.iter(|| {
            math::circle_intersects_line_segment(
                black_box(C),
                black_box(7.0),
                black_box(A),
                black_box(B),
            )
        })
    });
}

pub fn criterion_benchmark_0(c: &mut Criterion) {
    c.bench_function("circle intersects line segment 0", |b| {
        b.iter(|| {
            math::circle_intersects_line_segment_0(
                black_box(C),
                black_box(7.0),
                black_box(A),
                black_box(B),
            )
        })
    });
}

pub fn criterion_benchmark_1(c: &mut Criterion) {
    c.bench_function("circle intersects line segment 1", |b| {
        b.iter(|| {
            math::circle_intersects_line_segment_1(
                black_box(C),
                black_box(7.0),
                black_box(A),
                black_box(B),
            )
        })
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    criterion_benchmark_0,
    criterion_benchmark_1,
);
criterion_main!(benches);
