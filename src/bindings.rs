use crate::{Record as RustRecord, VectorStore as RustVectorStore};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

struct PyRecord {
    inner: RustRecord,
}

impl PyRecord {
    fn from_pydict<'py>(dict: Bound<'py, PyDict>) -> PyResult<Self> {
        let id: String = dict
            .get_item("id")?
            .ok_or_else(|| PyValueError::new_err("Record must have an 'id'"))?
            .extract()?;

        let values: Vec<f32> = dict
            .get_item("values")?
            .ok_or_else(|| PyValueError::new_err("Record must have 'values'"))?
            .extract()?;

        // Metadata is optional
        let metadata: Option<HashMap<String, String>> = match dict.get_item("metadata")? {
            Some(meta) => Some(meta.extract()?),
            None => None,
        };

        Ok(PyRecord {
            inner: RustRecord::new_with_metadata(id, values, metadata),
        })
    }

    fn to_pydict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);

        dict.set_item("id", self.inner.id())?;
        dict.set_item("values", self.inner.values().to_vec())?;

        if let Some(metadata) = self.inner.metadata() {
            dict.set_item("metadata", metadata.clone().into_py(py))?;
        }

        Ok(dict.into())
    }
}

#[pyclass]
struct VectorStore {
    inner: RustVectorStore,
}

#[pymethods]
impl VectorStore {
    #[new]
    fn new<'py>(records: Bound<'py, PyList>) -> PyResult<Self> {
        let rust_records: Vec<RustRecord> = records
            .iter()
            .map(|item| {
                let dict = item.downcast::<PyDict>()?;
                Ok(PyRecord::from_pydict(dict.clone())?.inner)
            })
            .collect::<PyResult<_>>()?;

        Ok(VectorStore {
            inner: RustVectorStore::new(rust_records),
        })
    }

    fn add<'py>(&mut self, records: Bound<'py, PyList>) -> PyResult<()> {
        let rust_records: Vec<RustRecord> = records
            .iter()
            .map(|item| {
                let dict = item.downcast::<PyDict>()?;
                Ok(PyRecord::from_pydict(dict.clone())?.inner)
            })
            .collect::<PyResult<_>>()?;

        self.inner.add(&rust_records);
        Ok(())
    }

    fn get(&self, ids: Vec<String>) -> PyResult<Vec<PyObject>> {
        Python::with_gil(|py| {
            // method expects Vec<&str>, convert
            let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

            self.inner
                .get(&id_refs)
                .into_iter()
                .map(|record| PyRecord { inner: record }.to_pydict(py))
                .collect()
        })
    }

    fn query(&self, vector: Vec<f32>, top_k: usize) -> PyResult<Vec<(PyObject, f32)>> {
        Python::with_gil(|py| {
            self.inner
                .query(&vector, top_k)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
                .into_iter()
                .map(|(r, s)| Ok((PyRecord { inner: r.clone() }.to_pydict(py)?, s)))
                .collect()
        })
    }

    fn delete(&mut self, ids: Vec<String>) {
        let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

        self.inner.delete(&id_refs);
    }

    fn list(&self) -> Vec<&str> {
        self.inner.list()
    }
}

#[pymodule]
fn zetacore(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VectorStore>()?;
    Ok(())
}
