

<p align="center">
  <a href="https://github.com/onuralpszr/yuko"><img src="https://raw.githubusercontent.com/onuralpszr/yuko/develop/logo/yuko_logo.png" alt="Yuko"></a>
</p>

<h1 style="font-style: italic;" align="center">Yuko (有効)</h1>


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/onuralpszr/yuko/main.svg)](https://results.pre-commit.ci/latest/github/onuralpszr/yuko/main)
[![Build & Release - CI](https://github.com/onuralpszr/yuko/actions/workflows/CI.yml/badge.svg)](https://github.com/onuralpszr/yuko/actions/workflows/CI.yml)
[![Coverage CI](https://github.com/onuralpszr/yuko/actions/workflows/Coverage-CI.yml/badge.svg)](https://github.com/onuralpszr/yuko/actions/workflows/Coverage-CI.yml)
[![codecov](https://codecov.io/gh/onuralpszr/yuko/graph/badge.svg?token=NWUYIBUCBA)](https://codecov.io/gh/onuralpszr/yuko)
[![CodeFactor](https://www.codefactor.io/repository/github/onuralpszr/yuko/badge)](https://www.codefactor.io/repository/github/onuralpszr/yuko)

<p align="center">
<a href="https://pypi.org/project/yuko" target="_blank">
    <img src="https://img.shields.io/pypi/v/yuko?color=%2334D058&label=pypi%20package" alt="Package version">
</a>
<a href="https://pypi.org/project/yuko" target="_blank">
    <img src="https://img.shields.io/pypi/dm/yuko?color=red" alt="Download Count">
</a>
<a href="https://pypi.org/project/yuko" target="_blank">
    <img src="https://img.shields.io/pypi/pyversions/yuko.svg?color=%2334D058" alt="Supported Python versions">
</a>
<a href="https://pypi.org/project/yuko" target="_blank">
    <img src="https://img.shields.io/pypi/status/yuko?color=orange" alt="Project Status">
</a>
<a href="https://makeapullrequest.com" target="_blank">
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" alt="PRs Welcome">
</a>
</p>


Yuko is a Python package that provides Rust-backed validators for common validation tasks. This project aims to enhance the performance of validation operations by leveraging Rust's speed ⚡️, while still being easily accessible from Python.

## 🛠 Installation

```bash
pip install yuko
```

## 🔥 Usage

```python
>>> import yuko
>>> yuko.email('loremipsum@example.com')
True
>>> yuko.email('loremipsum')
False
>>> yuko.country_code('TR')
True
>>> yuko.country_code('INVALID')
False
>>> yuko.ip_address("127.0.0.1","ipv4")
True
>>> yuko.ip_address("257.0.0.1","ipv4")
False
>>> yuko.ip_address("1:2:3:4:5:6:7:8", "ipv6")
True
>>> yuko.ip_address("0.0.0.0", "both")
True
>>> yuko.domain("example.com")
True
>>> yuko.domain("example.com.")
False
>>> yuko.domain("example")
False
>>> yuko.mac_address("01:23:45:67:ab:CD")
True
>>> yuko.mac_address("00:1A:2B:3C:4D:ZZ")
False
>>> yuko.md5("d41d8cd98f00b204e9800998ecf8427e")
True

```

## 🔮 Features

Yuko provides validators of the following:

- Email Validation
- Country Code(Alpha-3, Alpha-2, Numeric) Validation
- IP Address (ipv4, ipv6, both) Validation
- Domain Validation
- Mac Address Validation
- Hash (MD5, SHA1, SHA224, SHA256, SHA512) Validation
- URL Validation

## 💻 Adding New Validators
If you want to contribute a new validator, please follow these guidelines:

* Create a new Rust module for your validator in the src directory.
* Implement the validator logic in Rust.
* Expose the validator as a Python module using the pyo3 crate.
* Update the Python package accordingly.

## Credits

This project inspired by [Validators](https://github.com/python-validators/validators)
