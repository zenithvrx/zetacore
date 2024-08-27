mod record;
mod vector_store;

pub use record::Record;
pub use vector_store::VectorStore;

pub mod bindings;

#[cfg(test)]
mod tests {
    use super::*;

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
        // println!("{:#?}", result);

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
}
