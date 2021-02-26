use closest_pairs::closest::task_1::Task1;
use closest_pairs::geometry::Point;
use closest_pairs::quick_select::quick_select_points;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::prelude::*;
use std::time::Duration;

pub fn task_1(c: &mut Criterion) {
    const ITERATIONS: usize = 30;
    let mut group = c.benchmark_group("task_1");

    let mut rng = StdRng::seed_from_u64(0xC0FF_EE00);
    let trials = (10..=ITERATIONS).map(|x| {
        (0_usize..(1.5_f32.powf(x as f32) as usize))
            .map(|_| Point {
                x: rng.gen::<u64>() as f64,
                y: rng.gen::<u64>() as f64,
            })
            .collect_vec()
    });

    for trial in trials {
        group.bench_with_input(
            BenchmarkId::from_parameter(trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task1::new(t.clone()),
                    |mut data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
}

pub fn quick_select(c: &mut Criterion) {
    const ITERATIONS: usize = 35;
    let mut group = c.benchmark_group("quick_select");

    let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF);
    let trials = (20..=ITERATIONS).map(|x| {
        (0_usize..(1.5_f32.powf(x as f32) as usize))
            .map(|_| Point {
                x: rng.gen::<u64>() as f64,
                y: rng.gen::<u64>() as f64,
            })
            .collect_vec()
    });

    for trial in trials {
        let m = trial.len() / 2;
        group.bench_with_input(
            BenchmarkId::from_parameter(trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || t.clone(),
                    |mut data| {
                        quick_select_points(&mut data, m);
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
}

pub fn points_distance(c: &mut Criterion) {
    let a = Point {
        x: rand::random::<u64>() as f64,
        y: rand::random::<u64>() as f64,
    };

    let b = Point {
        x: rand::random::<u64>() as f64,
        y: rand::random::<u64>() as f64,
    };

    let mut group = c.benchmark_group("points_distance");

    group.bench_function("points_distance", |bn| {
        bn.iter(|| a.distance_to(black_box(b)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0));
    targets = points_distance, quick_select, task_1
}

criterion_main!(benches);
