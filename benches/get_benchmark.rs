mod common;
use common::create_random_record;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use metacore::{Record, VectorStore};

fn benchmark_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    let dimension = 1536;
    let sizes = [
        10, 100, 1_000, 5_000, 10_000, 50_000, 100_000, 500_000, 1_000_000,
    ];

    for size in sizes.iter() {
        let records: Vec<Record> = (0..*size)
            .map(|_| create_random_record(dimension))
            .collect();

        let store = VectorStore::new(records.clone());

        // Create a set of IDs to retrieve (e.g., 10% of the total)
        let num_ids_to_retrieve = (size / 10).max(1);
        let ids_to_retrieve: Vec<String> = records
            .iter()
            .take(num_ids_to_retrieve)
            .map(|r| r.id().to_string())
            .collect();

        group.bench_with_input(BenchmarkId::new("get", size), size, |b, &size| {
            b.iter(|| {
                let result = store.get(black_box(
                    &ids_to_retrieve
                        .iter()
                        .map(AsRef::as_ref)
                        .collect::<Vec<&str>>(),
                ));
                black_box(result);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_get);
criterion_main!(benches);
