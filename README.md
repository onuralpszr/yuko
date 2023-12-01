# ValidX  🦀 🤝 🐍

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/onuralpszr/validX/main.svg)](https://results.pre-commit.ci/latest/github/onuralpszr/validX/main)
[![Build & Release - CI](https://github.com/onuralpszr/validX/actions/workflows/CI.yml/badge.svg)](https://github.com/onuralpszr/validX/actions/workflows/CI.yml)
[![Coverage CI](https://github.com/onuralpszr/validX/actions/workflows/Coverage-CI.yml/badge.svg)](https://github.com/onuralpszr/validX/actions/workflows/Coverage-CI.yml)
[![codecov](https://codecov.io/gh/onuralpszr/validX/graph/badge.svg?token=NWUYIBUCBA)](https://codecov.io/gh/onuralpszr/validX)
[![CodeFactor](https://www.codefactor.io/repository/github/onuralpszr/validx/badge)](https://www.codefactor.io/repository/github/onuralpszr/validx)


ValidX is a Python package that provides Rust-backed validators for common validation tasks. This project aims to enhance the performance of validation operations by leveraging Rust's speed ⚡️, while still being easily accessible from Python.

## 🛠 Installation

The package is not yet available on PyPI, but you can install it directly from the source:

```bash
pip install git+https://github.com/onuralpszr/validX.git
```

## 🔥 Usage

```python
>>> import validx
>>> validx.email('loremipsum@example.com')
True
>>> validx.email('loremipsum')
False
>>> validx.country_code('TR')
True
>>> validx.country_code('INVALID')
False
>>> validx.ip_address("127.0.0.1","ipv4")
True
>>> validx.ip_address("257.0.0.1","ipv4")
False
>>> validx.ip_address("1:2:3:4:5:6:7:8", "ipv6")
True
>>> validx.ip_address("0.0.0.0", "both")
True
>>> validx.domain("example.com")
True
>>> validx.domain("example.com.")
False
>>> validx.domain("example")
False
>>> validx.mac_address("01:23:45:67:ab:CD")
True
>>> validx.mac_address("00:1A:2B:3C:4D:ZZ")
False

```

## 🔮 Features

ValidX provides validators of the following:

- Email Validation
- Country Code(Alpha-3, Alpha-2, Numeric) Validation
- IP Address (ipv4, ipv6, both) Validation
- Domain Validation
- Mac Address Validation

## 💻 Adding New Validators
If you want to contribute a new validator, please follow these guidelines:

* Create a new Rust module for your validator in the src directory.
* Implement the validator logic in Rust.
* Expose the validator as a Python module using the pyo3 crate.
* Update the Python package accordingly.

## Credits

This project inspired by [Validators](https://github.com/python-validators/validators)
