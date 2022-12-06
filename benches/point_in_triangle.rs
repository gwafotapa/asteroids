use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use asteroids::collision::math;

const A: Vec2 = Vec2 { x: -2.0, y: -1.0 };
const C: Vec2 = Vec2 { x: -1.0, y: 5.0 };
const B: Vec2 = Vec2 { x: 9.0, y: 2.0 };
const D: Vec2 = Vec2 { x: -1.0, y: 4.0 };

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("point in triangle", |b| {
        b.iter(|| math::point_in_triangle(black_box(D), black_box([A, B, C])))
    });
}

pub fn criterion_benchmark_bis(c: &mut Criterion) {
    c.bench_function("point in triangle", |b| {
        b.iter(|| math::point_in_triangle_bis(black_box(D), black_box([A, B, C])))
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_bis,);
criterion_main!(benches);
