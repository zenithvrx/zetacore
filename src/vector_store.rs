use crate::Record;

pub struct VectorStore {
    records: Vec<Record>,
}

impl VectorStore {
    pub fn new(records: Vec<Record>) -> Self {
        VectorStore { records }
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn add(&mut self, records: &[Record]) {
        self.records.extend_from_slice(records);
    }

    pub fn get(&self, ids: &[&str]) -> Vec<Record> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            for record in &self.records {
                if record.id() == *id {
                    result.push(record.clone());
                    break;
                }
            }
        }

        result
    }

    pub fn delete(&mut self, ids: &[&str]) {
        for id in ids {
            if let Some(pos) = self.records.iter().position(|record| record.id() == *id) {
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
                Self::cosine_similarity(vector, &record.values())
                    .map(|similarity| (record, similarity))
            })
            .collect::<Result<_, _>>()?;

        result.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));

        Ok(result.into_iter().take(top_k).collect())
    }

    pub fn list(&self) -> Vec<&str> {
        self.records.iter().map(|record| &*record.id()).collect()
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
