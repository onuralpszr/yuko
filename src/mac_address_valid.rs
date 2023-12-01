use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn mac_address(mac_address: String) -> PyResult<bool> {
    let mac_address_regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    return Ok(mac_address_regex.is_match(&mac_address));
}
