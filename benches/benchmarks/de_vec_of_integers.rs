use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use serde::Deserialize;

fn bench_de_vec_of_i8(c: &mut Criterion) {
    let mut json = vec![b'['];
    json.extend_from_slice(b"0");
    for i in 1..=i8::MAX {
        json.extend_from_slice(b",");
        json.extend_from_slice(i.to_string().as_bytes());
    }
    json.extend_from_slice(b"]");

    let mut ub_json = vec![b'['];
    ub_json.extend_from_slice(b"i");
    ub_json.extend_from_slice(&0i8.to_be_bytes());
    for i in 1..=i8::MAX {
        ub_json.extend_from_slice(b"i");
        ub_json.extend_from_slice(&i.to_be_bytes());
    }
    ub_json.extend_from_slice(b"]");

    let mut group = c.benchmark_group("de_vec_of_i8");

    group.bench_function(
        BenchmarkId::new("de_vec_of_i8_json", "Vec<i8>"),
        |b| b.iter(|| serde_json::from_slice::<'_, Vec<i8>>(black_box(&json)).unwrap())
    );

    group.bench_function(
        BenchmarkId::new("de_vec_of_i8_ub_json", "Vec<i8>"),
        |b| b.iter(|| serde_ub_json::from_bytes::<'_, Vec<i8>>(black_box(&ub_json)).unwrap())
    );

    group.finish();
}

fn bench_de_vec_of_i16(c: &mut Criterion) {
    let mut json = vec![b'['];
    json.extend_from_slice(b"0");
    for i in 1..=i8::MAX {
        json.extend_from_slice(b",");
        json.extend_from_slice(i.to_string().as_bytes());
    }
    json.extend_from_slice(b"]");

    let mut ub_json = vec![b'['];
    ub_json.extend_from_slice(b"I");
    ub_json.extend_from_slice(&0i16.to_be_bytes());
    for i in 1..=i8::MAX {
        ub_json.extend_from_slice(b"I");
        ub_json.extend_from_slice(&(i as i16).to_be_bytes());
    }
    ub_json.extend_from_slice(b"]");

    let mut group = c.benchmark_group("de_vec_of_i16");

    group.bench_function(
        BenchmarkId::new("de_vec_of_i16_json", "Vec<i16>"),
        |b| b.iter(|| serde_json::from_slice::<'_, Vec<i16>>(black_box(&json)).unwrap())
    );

    group.bench_function(
        BenchmarkId::new("de_vec_of_i16_ub_json", "Vec<i16>"),
        |b| b.iter(|| serde_ub_json::from_bytes::<'_, Vec<i16>>(black_box(&ub_json)).unwrap())
    );

    group.finish();
}

criterion_group!(benches, bench_de_vec_of_i8, bench_de_vec_of_i16);
