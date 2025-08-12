use rustracer::utils::init_headless_from_path;
use std::{path::Path, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};

// fn criterion_benchmark_09(c: &mut Criterion) {
//     c.bench_function("scene chap 9", |b| {
//         b.iter(|| init_headless_from_path(Path::new("scenes/ch09.yml")))
//     });
// }

fn criterion_benchmark_10(c: &mut Criterion) {
    c.bench_function("scene chap 10", |b| {
        b.iter(|| init_headless_from_path(Path::new("scenes/ch10.yml")))
    });
}

fn criterion_benchmark_11(c: &mut Criterion) {
    c.bench_function("scene chap 11", |b| {
        b.iter(|| init_headless_from_path(Path::new("scenes/ch11.yml")))
    });
}

fn criterion_benchmark_11_a(c: &mut Criterion) {
    c.bench_function("scene chap 11 a", |b| {
        b.iter(|| init_headless_from_path(Path::new("scenes/ch11_reflection.yml")))
    });
}

fn criterion_benchmark_11_b(c: &mut Criterion) {
    c.bench_function("scene chap 11 b", |b| {
        b.iter(|| init_headless_from_path(Path::new("scenes/ch11_refraction.yml")))
    });
}

fn criterion_benchmark_11_c(c: &mut Criterion) {
    c.bench_function("scene chap 11 c", |b| {
        b.iter(|| init_headless_from_path(Path::new("scenes/ch11_reflect-refract.yml")))
    });
}

criterion_group!(
   name =  benches;
     config = Criterion::default().sample_size(20).measurement_time(Duration::from_secs(100));
     targets =
    // criterion_benchmark_09,
    criterion_benchmark_10,
    criterion_benchmark_11,
    criterion_benchmark_11_a,
    criterion_benchmark_11_b,
    criterion_benchmark_11_c
);
criterion_main!(benches);
