use std::collections::HashMap;

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

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn metadata(&self) -> Option<&HashMap<String, String>> {
        self.metadata.as_ref()
    }
}
