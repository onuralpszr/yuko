use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn domain(domain: String) -> PyResult<bool> {
    // Check for whitespace
    let whitespace_regex = Regex::new(r"\s").unwrap();
    if whitespace_regex.is_match(&domain) {
        return Ok(false);
    }

    // Define regex for domain
    let domain_regex =
        Regex::new(r"^(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$")
            .unwrap();
    return Ok(domain_regex.is_match(&domain));
}
