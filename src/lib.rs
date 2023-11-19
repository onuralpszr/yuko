pub mod country_code_valid;

use pyo3::prelude::*;
use country_code_valid::country_code;


#[pymodule]
fn validx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    Ok(())
}
