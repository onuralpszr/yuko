pub mod country_code;

use pyo3::prelude::*;
use country_code::country_code_validator;


#[pymodule]
fn validx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code_validator, m)?)?;
    Ok(())
}
