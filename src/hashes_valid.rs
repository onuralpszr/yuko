use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn md5(value: String) -> PyResult<bool> {
    // The maximum length of an email is 320 characters per RFC 3696

    let re: Regex = Regex::new(r"^[0-9a-f]{32}$").unwrap();

    if re.is_match(&value.to_lowercase()) {
        return Ok(true);
    }
    return Ok(false);
}

#[pyfunction]
pub fn sha1(value: String) -> PyResult<bool> {
    let re: Regex = Regex::new(r"^[0-9a-f]{40}$").unwrap();

    if re.is_match(&value.to_lowercase()) {
        return Ok(true);
    }
    return Ok(false);
}

#[pyfunction]
pub fn sha224(value: String) -> PyResult<bool> {
    let re: Regex = Regex::new(r"^[0-9a-f]{56}$").unwrap();

    if re.is_match(&value.to_lowercase()) {
        return Ok(true);
    }

    return Ok(false);
}

#[pyfunction]
pub fn sha256(value: String) -> PyResult<bool> {
    let re: Regex = Regex::new(r"^[0-9a-f]{64}$").unwrap();

    if re.is_match(&value.to_lowercase()) {
        return Ok(true);
    }

    return Ok(false);
}

#[pyfunction]
pub fn sha512(value: String) -> PyResult<bool> {
    let re: Regex = Regex::new(r"^[0-9a-f]{128}$").unwrap();

    if re.is_match(&value.to_lowercase()) {
        return Ok(true);
    }

    return Ok(false);
}
