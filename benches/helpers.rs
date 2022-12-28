use std::collections::HashSet;

use adventofcode::common::grid::minmax::minmax_ivec2;
use criterion::{criterion_group, criterion_main, Criterion};
use glam::IVec2;

fn bench_minmax_ivec2(c: &mut Criterion) {
    c.bench_function("minmax IVec2", |b| {
        let mut vecs = HashSet::new();
        for x in 0..100 {
            for y in 0..100 {
                vecs.insert(IVec2::new(x, y));
            }
        }

        b.iter(|| minmax_ivec2(vecs.iter()))
    });
}

criterion_group!(helpers, bench_minmax_ivec2);
criterion_main!(helpers);
