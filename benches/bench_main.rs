use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::ser_simple_struct::benches,
    benchmarks::ser_vec_of_integers::benches,
    benchmarks::de_simple_struct::benches,
    benchmarks::de_vec_of_integers::benches,
}
