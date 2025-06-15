use criterion::{black_box, criterion_group, criterion_main, Criterion};
use papaya::HashMap;
use seize::Collector;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const SIZE: usize = 100_000;

fn par_vs_iter(c: &mut Criterion) {
    let map = HashMap::<usize, usize>::builder()
        .collector(Collector::new())
        .build();
    for i in 0..SIZE {
        map.pin().insert(i, i);
    }

    let map_ref = map.pin_owned();

    let mut group = c.benchmark_group("par_iter");
    group.bench_function("iter", |b| {
        b.iter(|| {
            let sum: usize = map_ref.iter().map(|(_, &v)| v).sum();
            black_box(sum);
        })
    });

    #[cfg(feature = "rayon")]
    group.bench_function("par_iter", |b| {
        b.iter(|| {
            let sum: usize = map_ref.par_iter().map(|(_, &v)| v).sum();
            black_box(sum);
        })
    });

    group.finish();
}

criterion_group!(benches, par_vs_iter);
criterion_main!(benches);
