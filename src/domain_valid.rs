use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn domain(domain: String, rfc_2782: bool, rfc_1034: bool) -> PyResult<bool> {
    let whitespace_regex = Regex::new(r"\s").unwrap();

    if !whitespace_regex.is_match(&domain) {
        let domain_regex = format!(
            r"^(?:[a-zA-Z0-9{}](?:[a-zA-Z0-9-_]{{0,61}}[A-Za-z0-9])?\.)+[A-Za-z0-9][A-Za-z0-9-_]{{0,61}}[A-Za-z]{}",
            if rfc_2782 { "_" } else { "" },
            if rfc_1034 { ".$" } else { "$" }
        );

        let domain_regex = Regex::new(&domain_regex).unwrap();
        return Ok(domain_regex.is_match(&domain));
    }

    return Ok(false);
}
