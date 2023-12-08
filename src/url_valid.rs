use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn url(url: String) -> PyResult<bool> {
    let whitespace_regex = Regex::new(r"\s").unwrap();

    if !whitespace_regex.is_match(&url) {
        let url_regex = Regex::new(
            r"^(?i)(ftp|http|https):\/\/([a-z0-9-]+\.)*[a-z0-9-]+(\.[a-z]{2,})?(:\d{1,5})?(\/[^\s]*)?$",
        )
        .unwrap();
        return Ok(url_regex.is_match(&url));
    }

    return Ok(false);
}
