use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use serde::Serialize;

fn bench_ser_vec_of_i8(c: &mut Criterion) {
    let mut value = vec![0i8; (i8::MAX as usize) + 1];
    for i in 0..value.len() {
        value[i] = i as i8;
    }

    let mut group = c.benchmark_group("ser_vec_of_i8");

    group.bench_function(
        BenchmarkId::new("ser_vec_of_i8_json", "Vec<i8>"),
        |b| b.iter(|| serde_json::to_vec(black_box(&value)))
    );

    group.bench_function(
        BenchmarkId::new("ser_vec_of_i8_ub_json", "Vec<i8>"),
        |b| b.iter(|| serde_ub_json::to_bytes(black_box(&value)))
    );

    group.finish();
}

fn bench_ser_vec_of_i16(c: &mut Criterion) {
    let mut value = vec![0i16; (i8::MAX as usize) + 1];
    for i in 0..value.len() {
        value[i] = i as i16;
    }

    let mut group = c.benchmark_group("ser_simple_struct");

    group.bench_function(
        BenchmarkId::new("ser_vec_of_i16_json", "Vec<i16>"),
        |b| b.iter(|| serde_json::to_vec(black_box(&value)))
    );

    group.bench_function(
        BenchmarkId::new("ser_vec_of_i16_ub_json", "Vec<i16>"),
        |b| b.iter(|| serde_ub_json::to_bytes(black_box(&value)))
    );

    group.finish();
}

criterion_group!(benches, bench_ser_vec_of_i8, bench_ser_vec_of_i16);
