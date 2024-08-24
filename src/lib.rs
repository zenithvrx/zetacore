#[derive(Debug, PartialEq, Clone)]
pub struct Embedding {
    identifier: u64,
    vector: Vec<f64>,
}

pub struct VectorStore {
    embeddings: Vec<Embedding>,
}

fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
    if a.len() != b.len() {
        return Err("vectors are not equal length");
    }

    let result = (a.iter().zip(b.iter())).map(|(x, y)| x * y).sum();
    Ok(result)
}

fn magnitude(a: &[f64]) -> f64 {
    let s: f64 = a.iter().map(|x| x.powi(2)).sum();
    s.sqrt()
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
    let result = dot_product(a, b)? / (magnitude(a) * magnitude(b));
    Ok(result)
}

impl VectorStore {
    pub fn new(embeddings: Vec<Embedding>) -> VectorStore {
        VectorStore { embeddings }
    }

    pub fn add(&mut self, embedding: Embedding) -> () {
        self.embeddings.push(embedding);
    }

    pub fn remove(&mut self, identifier: u64) -> () {
        let index = self
            .embeddings
            .iter()
            .position(|x| x.identifier == identifier);
        if let Some(i) = index {
            self.embeddings.remove(i);
        }
    }

    pub fn query(
        &self,
        input_vector: &[f64],
        n: usize,
    ) -> Result<Vec<(&Embedding, f64)>, &'static str> {
        let similarity_results: Result<Vec<f64>, &'static str> = self
            .embeddings
            .iter()
            .map(|x| cosine_similarity(input_vector, &x.vector))
            .collect();

        let mut result: Vec<(&Embedding, f64)> =
            self.embeddings.iter().zip(similarity_results?).collect();
        result.sort_by(
            |a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less), // Handle NaN
        );

        let first_n = result[..n].to_vec();

        Ok(first_n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vector_store() {
        let empty_embeddings = vec![];
        let vector_store = VectorStore::new(empty_embeddings);
        assert!(vector_store.embeddings.is_empty());
    }

    #[test]
    fn add_embedding() {
        let mut vector_store = VectorStore::new(vec![]);

        let embedding = Embedding {
            identifier: 1,
            vector: vec![1.2, 3.5, 2.9],
        };
        vector_store.add(embedding.clone());

        assert_eq!(vector_store.embeddings[0], embedding);
    }

    #[test]
    fn remove_embedding() {
        let mut vector_store = VectorStore::new(vec![]);

        let embedding = Embedding {
            identifier: 1,
            vector: vec![1.2, 3.5, 2.9],
        };
        vector_store.add(embedding.clone());
        vector_store.remove(1);

        assert!(vector_store.embeddings.is_empty());
    }

    #[test]
    fn query() {
        let mut vector_store = VectorStore::new(vec![]);

        let embedding1 = Embedding {
            identifier: 1,
            vector: vec![1.2, 3.5, 2.9],
        };
        let embedding2 = Embedding {
            identifier: 2,
            vector: vec![3.2, 4.580, 9.38],
        };
        let embedding3 = Embedding {
            identifier: 3,
            vector: vec![12.3, 0.324, 8.25],
        };
        let embedding4 = Embedding {
            identifier: 4,
            vector: vec![3.56, 6.43, 4.23],
        };

        vector_store.add(embedding1.clone());
        vector_store.add(embedding2.clone());
        vector_store.add(embedding3.clone());
        vector_store.add(embedding4.clone());

        let query_vector = vec![1.0, 1.0, 1.0];

        let result = vector_store.query(&query_vector, 3);

        println!("{:#?}", result);
    }
}
