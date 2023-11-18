use pyo3::prelude::*;

#[pyfunction]
pub fn country_code_validator(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
}
