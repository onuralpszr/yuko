pub mod country_code_valid;
pub mod email_valid;

use country_code_valid::country_code;
use email_valid::email;
use pyo3::prelude::*;

#[pymodule]
fn validx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    m.add_function(wrap_pyfunction!(email, m)?)?;
    Ok(())
}
