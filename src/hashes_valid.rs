use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn md5_hash(md5_hash: String) -> PyResult<bool> {
    // The maximum length of an email is 320 characters per RFC 3696

    let re:Regex = Regex::new(r"^[0-9a-f]{32}$").unwrap();

    if re.is_match(&md5_hash) {
        return Ok(true);
    }
    return Ok(false);
}
