use closest_pairs::closest::task_1::Task1;
use closest_pairs::closest::task_3_1::Task3QuickSort;
use closest_pairs::closest::task_3_2::Task3SortedY;
use closest_pairs::closest::task_4::Task4;
use closest_pairs::geometry::Point;
use closest_pairs::quick_select::quick_select_points;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::prelude::*;
use std::time::Duration;

const FACTOR: f32 = 1.5;

pub fn closest_pair_average(c: &mut Criterion) {
    const ITER_MIN: usize = 10;
    const ITER_MAX: usize = 30;

    let mut group = c.benchmark_group("closest_pair_average");
    let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF);

    let trials = (ITER_MIN..=ITER_MAX).map(|x| {
        (0_usize..(FACTOR.powf(x as f32) as usize))
            .map(|_| Point {
                x: rng.gen::<i64>() as f64,
                y: rng.gen::<i64>() as f64,
            })
            .collect_vec()
    });

    for trial in trials {
        // Task 1
        group.bench_with_input(
            BenchmarkId::new("task_1", trial.len()),
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

        // Task 3: Sort before starting
        group.bench_with_input(
            BenchmarkId::new("task_3_sort_first", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task3QuickSort::new(t.clone()),
                    |mut data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        // Task 3: Ensure y-sorted
        group.bench_with_input(
            BenchmarkId::new("task_3_maintain_y", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task3SortedY::new(t.clone()),
                    |mut data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        // Task 4: Simple-Randomised
        group.bench_with_input(
            BenchmarkId::new("task_4", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task4::new(t.clone()),
                    |data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
}

pub fn closest_pair_worst(c: &mut Criterion) {
    const ITER_MIN: usize = 5;
    const ITER_MAX: usize = 25;

    let mut group = c.benchmark_group("closest_pair_worst");
    let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF);

    let trials = (ITER_MIN..=ITER_MAX).map(|x| {
        let mut v = (0_usize..(FACTOR.powf(x as f32) as usize))
            .map(|xv| Point {
                x: xv as f64,
                y: rng.gen::<i64>() as f64,
            })
            .collect_vec();
        v.rotate_right(1);
        v
    });

    for trial in trials {
        // Task 1
        group.bench_with_input(
            BenchmarkId::new("task_1", trial.len()),
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

        // Task 3: Sort before starting
        group.bench_with_input(
            BenchmarkId::new("task_3_sort_first", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task3QuickSort::new(t.clone()),
                    |mut data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        // Task 3: Ensure y-sorted
        group.bench_with_input(
            BenchmarkId::new("task_3_maintain_y", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task3SortedY::new(t.clone()),
                    |mut data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        // Task 4: Simple-Randomised
        group.bench_with_input(
            BenchmarkId::new("task_4", trial.len()),
            &trial,
            move |b, t| {
                b.iter_batched(
                    || Task4::new(t.clone()),
                    |data| {
                        data.find_closest_pair();
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
}

pub fn quick_select_average(c: &mut Criterion) {
    const ITERATIONS: usize = 35;
    let mut group = c.benchmark_group("quick_select_average");
    let mut rng = StdRng::seed_from_u64(0xC0DE_BEEF);

    let trials = (20..=ITERATIONS).map(|x| {
        (0_usize..(1.5_f32.powf(x as f32) as usize))
            .map(|_| Point {
                x: rng.gen::<i64>() as f64,
                y: rng.gen::<i64>() as f64,
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

pub fn quick_select_worst(c: &mut Criterion) {
    const ITERATIONS: usize = 30;
    let mut rng = StdRng::seed_from_u64(0xC0DE_BEEF);
    let mut group = c.benchmark_group("quick_select_worst");

    let trials = (15..=ITERATIONS).map(|x| {
        let mut v = (0_usize..(1.5_f32.powf(x as f32) as usize))
            .map(|xv| Point {
                x: xv as f64,
                y: rng.gen::<i64>() as f64,
            })
            .collect_vec();
        v.rotate_right(1);
        v
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
    let mut rng = StdRng::seed_from_u64(0xBAB5_EED5);

    let a = Point {
        x: rng.gen::<i64>() as f64,
        y: rng.gen::<i64>() as f64,
    };

    let b = Point {
        x: rng.gen::<i64>() as f64,
        y: rng.gen::<i64>() as f64,
    };

    let mut group = c.benchmark_group("geometry");

    group.bench_function("points_distance", |bn| {
        bn.iter(|| a.distance_to(black_box(b)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0)).warm_up_time(Duration::new(3, 0)).sample_size(50);
    targets = points_distance, quick_select_average, quick_select_worst, closest_pair_average, closest_pair_worst
}

criterion_main!(benches);
