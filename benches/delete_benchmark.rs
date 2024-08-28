mod common;
use common::create_random_record;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use zetacore::{Record, VectorStore};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn benchmark_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete");

    let dimension = 1536;
    let sizes = [10, 100, 1_000, 5_000, 10_000, 50_000, 100_000];

    for size in sizes.iter() {
        let records: Vec<Record> = (0..*size)
            .map(|_| create_random_record(dimension))
            .collect();

        let mut rng = thread_rng();

        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let store = VectorStore::new(records.clone());

                    // Select ~10% of IDs to delete, randomly
                    let num_ids_to_delete = (size / 10).max(1);
                    let ids_to_delete: Vec<String> = records
                        .choose_multiple(&mut rng, num_ids_to_delete)
                        .map(|r| r.id().to_string())
                        .collect();

                    (store, ids_to_delete)
                },
                |(mut store, ids_to_delete)| {
                    store.delete(black_box(
                        &ids_to_delete
                            .iter()
                            .map(AsRef::as_ref)
                            .collect::<Vec<&str>>(),
                    ));
                },
            );
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_delete);
criterion_main!(benches);
