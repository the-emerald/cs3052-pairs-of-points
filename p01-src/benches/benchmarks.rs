use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use closest_pairs::geometry::Point;
use rand::prelude::*;
use itertools::Itertools;
use closest_pairs::quick_select::quick_select_points;
use std::time::Duration;

pub fn quick_select(c: &mut Criterion) {
    const ITERATIONS: usize = 35;

    let mut rng = rand::thread_rng();
    let trials = (20..=ITERATIONS)
        .map(|x| {
            (0_usize..(1.5_f32.powf(x as f32) as usize)).map(|y| {
                Point {
                    x: rng.gen::<u64>() as f64,
                    y: rng.gen::<u64>() as f64
                }
            })
                .collect_vec()
        });

    for trial in trials {
        let m = trial.len()/2;
        c.bench_with_input(BenchmarkId::new("quick_select", trial.len()),
                           &trial,
                           move |b, t|  {
                               b.iter_batched(|| t.clone(), |mut data| { quick_select_points(&mut data, m); }, BatchSize::LargeInput);
                           });
    }
}

pub fn points_distance(c: &mut Criterion) {
    let a = Point {
        x: rand::random::<u64>() as f64,
        y: rand::random::<u64>() as f64
    };

    let b = Point {
        x: rand::random::<u64>() as f64,
        y: rand::random::<u64>() as f64
    };

    c.bench_function("points_distance", |bn| bn.iter(||
        a.distance_to(black_box(b))
    ));
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = points_distance, quick_select
}

criterion_main!(benches);