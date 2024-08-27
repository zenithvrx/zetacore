use std::collections::HashMap;
pub mod bindings;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    id: String,
    values: Vec<f32>,
    metadata: Option<HashMap<String, String>>,
}

impl Record {
    pub fn new(id: impl Into<String>, values: Vec<f32>) -> Self {
        Self {
            id: id.into(),
            values,
            metadata: None,
        }
    }

    pub fn new_with_metadata(
        id: impl Into<String>,
        values: Vec<f32>,
        metadata: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            id: id.into(),
            values,
            metadata,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub struct VectorStore {
    records: Vec<Record>,
}

impl VectorStore {
    pub fn new(records: Vec<Record>) -> Self {
        VectorStore { records }
    }

    pub fn add(&mut self, records: &[Record]) {
        self.records.extend_from_slice(records);
    }

    pub fn get(&self, ids: &[&str]) -> Vec<Record> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            for record in &self.records {
                if record.id == *id {
                    result.push(record.clone());
                    break;
                }
            }
        }

        result
    }

    pub fn delete(&mut self, ids: &[&str]) {
        for id in ids {
            if let Some(pos) = self.records.iter().position(|record| record.id == *id) {
                self.records.swap_remove(pos);
            }
        }
    }

    pub fn query(&self, vector: &[f32], top_k: usize) -> Result<Vec<(&Record, f32)>, &'static str> {
        const MAX_TOP_K: usize = 10_000;

        if top_k > MAX_TOP_K {
            return Err("top_k maximum value is 10,000 records");
        }

        let mut result: Vec<(&Record, f32)> = self
            .records
            .iter()
            .map(|record| {
                Self::cosine_similarity(vector, &record.values)
                    .map(|similarity| (record, similarity))
            })
            .collect::<Result<_, _>>()?;

        result.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));

        Ok(result.into_iter().take(top_k).collect())
    }

    pub fn list(&self) -> Vec<&str> {
        self.records.iter().map(|record| &*record.id).collect()
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        if a.is_empty() || b.is_empty() {
            return Err("Vectors cannot be empty");
        }

        let dot_prod = Self::dot_product(a, b)?;
        let mag_a = Self::magnitude(a);
        let mag_b = Self::magnitude(b);

        if mag_a == 0.0 || mag_b == 0.0 {
            return Err("Vector magnitude cannot be zero");
        }

        Ok(dot_prod / (mag_a * mag_b))
    }

    fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        if a.len() != b.len() {
            return Err("Vectors are not equal length");
        }

        Ok(a.iter().zip(b).map(|(&x, &y)| x * y).sum())
    }

    fn magnitude(a: &[f32]) -> f32 {
        a.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_vector_store() {
        let store = VectorStore::new(vec![]);
        assert!(store.records.is_empty());
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

        assert_eq!(store.records, records);
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
            store.records,
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
        assert_eq!(result[0].0.id, "vec4", "First result should be vec4");
        assert_eq!(result[1].0.id, "vec3", "Second result should be vec3");
        assert_eq!(result[2].0.id, "vec1", "Third result should be vec1");
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
