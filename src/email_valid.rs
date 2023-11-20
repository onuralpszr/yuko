use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn email(email: String) -> PyResult<bool> {
    // The maximum length of an email is 320 characters per RFC 3696
    // section 3.
    if !email.contains("@") || email.len() > 320 {
        return Ok(false);
    }

    // TODO: use more complex regex
    // https://github.com/python-validators/validators/blob/master/src/validators/email.py
    let re = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();

    if re.is_match(&email) {
        return Ok(true);
    }
    return Ok(false);
}
