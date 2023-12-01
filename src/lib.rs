pub mod country_code_valid;
pub mod email_valid;
pub mod hashes_valid;
pub mod ip_address_valid;

use country_code_valid::country_code;
use email_valid::email;
use hashes_valid::{md5, sha1, sha224, sha256, sha512};
use ip_address_valid::ip_address;
use pyo3::prelude::*;

#[pymodule]
fn validx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    m.add_function(wrap_pyfunction!(email, m)?)?;
    m.add_function(wrap_pyfunction!(ip_address, m)?)?;
    m.add_function(wrap_pyfunction!(md5, m)?)?;
    m.add_function(wrap_pyfunction!(sha1, m)?)?;
    m.add_function(wrap_pyfunction!(sha224, m)?)?;
    m.add_function(wrap_pyfunction!(sha256, m)?)?;
    m.add_function(wrap_pyfunction!(sha512, m)?)?;
    Ok(())
}
