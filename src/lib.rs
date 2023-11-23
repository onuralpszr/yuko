pub mod country_code_valid;
pub mod email_valid;
pub mod ip_address_valid;

use country_code_valid::country_code;
use email_valid::email;
use ip_address_valid::ip_address;
use pyo3::prelude::*;

#[pymodule]
fn validx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    m.add_function(wrap_pyfunction!(email, m)?)?;
    m.add_function(wrap_pyfunction!(ip_address, m)?)?;
    Ok(())
}
