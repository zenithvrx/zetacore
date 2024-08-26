mod common;
use common::create_random_record;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use holocron::{Record, VectorStore};

fn benchmark_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    let dimension = 1536;
    let sizes = [10, 100, 1_000, 5_000, 10_000, 50_000, 100_000];

    for size in sizes.iter() {
        let records: Vec<Record> = (0..*size)
            .map(|_| create_random_record(dimension))
            .collect();

        group.bench_with_input(BenchmarkId::new("add", size), size, |b, &size| {
            b.iter(|| {
                let mut store = VectorStore::new(vec![]);
                store.add(black_box(&records[..size]));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_add);
criterion_main!(benches);
