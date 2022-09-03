use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct SimpleStruct {
    field1: i32,
    field2: String,
}

fn bench_de_simple_struct(c: &mut Criterion) {
    let json = b"{\"field1\":1024,\"field2\":\"test\"}";

    let mut ub_json = vec![b'{'];
    ub_json.extend_from_slice(b"#");
    ub_json.extend_from_slice(b"i");
    ub_json.extend_from_slice(&2i8.to_be_bytes());

    ub_json.extend_from_slice(b"i");
    ub_json.extend_from_slice(&6i8.to_be_bytes());
    ub_json.extend_from_slice(b"field1");
    ub_json.extend_from_slice(b"l");
    ub_json.extend_from_slice(&1024i32.to_be_bytes());

    ub_json.extend_from_slice(b"i");
    ub_json.extend_from_slice(&6i8.to_be_bytes());
    ub_json.extend_from_slice(b"field2");
    ub_json.extend_from_slice(b"S");
    ub_json.extend_from_slice(b"i");
    ub_json.extend_from_slice(&4i8.to_be_bytes());
    ub_json.extend_from_slice(b"test");

    let mut group = c.benchmark_group("de_simple_struct");

    group.bench_function(
        BenchmarkId::new("de_simple_struct_json", "SimpleStruct"),
        |b| b.iter(|| serde_json::from_slice::<'_, SimpleStruct>(black_box(json)).unwrap())
    );

    group.bench_function(
        BenchmarkId::new("de_simple_struct_ub_json", "SimpleStruct"),
        |b| b.iter(|| serde_ub_json::from_bytes::<'_, SimpleStruct>(black_box(&ub_json)).unwrap())
    );

    group.finish();
}

criterion_group!(benches, bench_de_simple_struct);
