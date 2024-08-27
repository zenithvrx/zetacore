use crate::Record;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VectorStoreError {
    #[error("top_k maximum value is 10,000 records")]
    TopKTooLarge,
    #[error("Vectors cannot be empty")]
    EmptyVector,
    #[error("Vector magnitude cannot be zero")]
    ZeroMagnitude,
    #[error("Vectors are not equal length")]
    UnequalVectorLengths,
    #[error("Error calculating similarity for record id {0}: {1}")]
    SimilarityCalculationError(String, #[source] Box<VectorStoreError>),
}

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
        ids.iter()
            .filter_map(|&id| self.records.iter().find(|r| r.id() == id))
            .cloned()
            .collect()
    }

    pub fn delete(&mut self, ids: &[&str]) {
        self.records.retain(|r| !ids.contains(&r.id()))
    }

    pub fn query(
        &self,
        vector: &[f32],
        top_k: usize,
    ) -> Result<Vec<(&Record, f32)>, VectorStoreError> {
        const MAX_TOP_K: usize = 10_000;

        if top_k > MAX_TOP_K {
            return Err(VectorStoreError::TopKTooLarge);
        }

        let mut result = Vec::with_capacity(self.records().len());

        for record in self.records() {
            let similarity = Self::cosine_similarity(vector, &record.values());

            match similarity {
                Ok(similarity) => result.push((record, similarity)),
                Err(e) => {
                    return Err(VectorStoreError::SimilarityCalculationError(
                        record.id().to_string(),
                        Box::new(e),
                    ))
                }
            }
        }

        result.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));

        Ok(result.into_iter().take(top_k).collect())
    }

    pub fn list(&self) -> Vec<&str> {
        self.records.iter().map(|record| &*record.id()).collect()
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, VectorStoreError> {
        if a.is_empty() || b.is_empty() {
            return Err(VectorStoreError::EmptyVector);
        }

        let dot_prod = Self::dot_product(a, b)?;
        let mag_a = Self::magnitude(a);
        let mag_b = Self::magnitude(b);

        if mag_a == 0.0 || mag_b == 0.0 {
            return Err(VectorStoreError::ZeroMagnitude);
        }

        Ok(dot_prod / (mag_a * mag_b))
    }

    fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, VectorStoreError> {
        if a.len() != b.len() {
            return Err(VectorStoreError::UnequalVectorLengths);
        }

        Ok(a.iter().zip(b).map(|(&x, &y)| x * y).sum())
    }

    fn magnitude(a: &[f32]) -> f32 {
        a.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}
