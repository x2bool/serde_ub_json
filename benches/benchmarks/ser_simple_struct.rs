use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct SimpleStruct {
    field1: i32,
    field2: String,
}

fn bench_ser_simple_struct(c: &mut Criterion) {
    let value = SimpleStruct {
        field1: i32::MAX,
        field2: "test".to_string(),
    };

    let mut group = c.benchmark_group("ser_simple_struct");

    group.bench_function(
        BenchmarkId::new("ser_simple_struct_json", "SimpleStruct"),
        |b| b.iter(|| serde_json::to_vec(black_box(&value)))
    );

    group.bench_function(
        BenchmarkId::new("ser_simple_struct_ub_json", "SimpleStruct"),
        |b| b.iter(|| serde_ub_json::to_bytes(black_box(&value)))
    );

    group.finish();
}

criterion_group!(benches, bench_ser_simple_struct);
