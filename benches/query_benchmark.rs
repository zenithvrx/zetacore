mod common;
use common::create_random_record;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use zetacore::{Record, VectorStore};
use rand::Rng;

fn create_random_vector(dimension: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dimension).map(|_| rng.gen_range(-10.0..10.0)).collect()
}

fn benchmark_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("query");

    let dimension = 1536;
    let sizes = [10, 100, 1_000, 10_000, 100_000, 1_000_000];

    for size in sizes.iter() {
        let mut store = VectorStore::new(vec![]);
        let records: Vec<Record> = (0..*size)
            .map(|_| create_random_record(dimension))
            .collect();
        store.add(&records);

        let query_vector = create_random_vector(dimension);

        group.bench_with_input(BenchmarkId::new("query", size), size, |b, &size| {
            b.iter(|| store.query(&query_vector, size.min(10)))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_query);
criterion_main!(benches);
