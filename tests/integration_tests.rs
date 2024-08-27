use holocron::{Record, VectorStore};

#[test]
fn init_vector_store() {
    let store = VectorStore::new(vec![]);
    assert!(store.records().is_empty());
}

#[test]
fn add_to_vector_store() {
    let mut store = VectorStore::new(vec![]);

    let records = vec![
        Record::new("vec1", vec![1.2, 2.0]),
        Record::new("vec2", vec![4.0, 9.5]),
        Record::new("vec3", vec![9.3, 7.6]),
        Record::new("vec4", vec![3.4, 3.1]),
    ];
    store.add(&records);

    assert_eq!(store.records(), records);
}

#[test]
fn get_from_vector_store() {
    let mut store = VectorStore::new(vec![]);

    let records = vec![
        Record::new("vec1", vec![1.2, 2.0]),
        Record::new("vec2", vec![4.0, 9.5]),
        Record::new("vec3", vec![9.3, 7.6]),
        Record::new("vec4", vec![3.4, 3.1]),
    ];
    store.add(&records);

    let result = store.get(&["vec1", "vec4"]);

    assert_eq!(
        result,
        vec![
            Record::new("vec1", vec![1.2, 2.0]),
            Record::new("vec4", vec![3.4, 3.1])
        ]
    )
}

#[test]
fn delete_from_vector_store() {
    let mut store = VectorStore::new(vec![]);

    let records = vec![
        Record::new("vec1", vec![1.2, 2.0]),
        Record::new("vec2", vec![4.0, 9.5]),
        Record::new("vec3", vec![9.3, 7.6]),
        Record::new("vec4", vec![3.4, 3.1]),
    ];
    store.add(&records);

    store.delete(&["vec3", "vec4"]);

    assert_eq!(
        store.records(),
        vec![
            Record::new("vec1", vec![1.2, 2.0]),
            Record::new("vec2", vec![4.0, 9.5]),
        ]
    )
}

#[test]
fn query_vector_store() {
    let mut store = VectorStore::new(vec![]);

    let records = vec![
        Record::new("vec1", vec![1.2, 2.0]),
        Record::new("vec2", vec![4.0, 9.5]),
        Record::new("vec3", vec![9.3, 7.6]),
        Record::new("vec4", vec![3.4, 3.1]),
    ];
    store.add(&records);

    let vector = vec![1.0, 1.0];
    let result = store.query(&vector, 3).unwrap();

    assert_eq!(result.len(), 3, "Should return top 3 results");
    assert_eq!(result[0].0.id(), "vec4", "First result should be vec4");
    assert_eq!(result[1].0.id(), "vec3", "Second result should be vec3");
    assert_eq!(result[2].0.id(), "vec1", "Third result should be vec1");
}

#[test]
fn list_vector_store_ids() {
    let mut store = VectorStore::new(vec![]);

    let records = vec![
        Record::new("vec1", vec![1.2, 2.0]),
        Record::new("vec2", vec![4.0, 9.5]),
        Record::new("vec3", vec![9.3, 7.6]),
        Record::new("vec4", vec![3.4, 3.1]),
    ];
    store.add(&records);

    assert_eq!(store.list(), vec!["vec1", "vec2", "vec3", "vec4"])
}

#[test]
fn query_vector_store_error_handling() {
    let store = VectorStore::new(vec![]);
    let vector = vec![1.0, 1.0];
    let result = store.query(&vector, 10001);
    assert!(result.is_err(), "Should return an error for top_k > 10000");
}

#[test]
fn get_non_existent_records() {
    let store = VectorStore::new(vec![Record::new("vec1", vec![1.0, 2.0])]);
    let result = store.get(&["non_existent"]);
    assert!(
        result.is_empty(),
        "Should return empty vec for non-existent IDs"
    );
}

#[test]
fn delete_non_existent_records() {
    let mut store = VectorStore::new(vec![Record::new("vec1", vec![1.0, 2.0])]);
    store.delete(&["non_existent"]);
    assert_eq!(store.records().len(), 1, "Should not delete any records");
}

#[test]
fn query_empty_vector_store() {
    let store = VectorStore::new(vec![]);
    let vector = vec![1.0, 1.0];
    let result = store.query(&vector, 3).unwrap();
    assert!(
        result.is_empty(),
        "Query on empty store should return empty result"
    );
}

#[test]
fn record_with_metadata() {
    use std::collections::HashMap;

    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    let record = Record::new_with_metadata("vec1", vec![1.0, 2.0], Some(metadata));

    assert_eq!(record.id(), "vec1");
    assert_eq!(record.values(), &[1.0, 2.0]);
    assert_eq!(
        record.metadata().unwrap().get("key1"),
        Some(&"value1".to_string())
    );
}
