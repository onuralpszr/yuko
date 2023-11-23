use pyo3::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr};

fn validate_ipv4_address(ip: &str) -> bool {
    // validate ipv4 address
    ip.parse::<Ipv4Addr>().is_ok()
}

fn validate_ipv6_address(ip: &str) -> bool {
    // validate ipv6 address
    ip.parse::<Ipv6Addr>().is_ok()
}

fn validate_ipv46_address(ip: &str) -> bool {
    // validate ipv4 or ipv6 address
    validate_ipv4_address(ip) || validate_ipv6_address(ip)
}

// IP_ADDRESS_VALIDATOR_MAP is a map of protocol to validator function.
static IP_ADDRESS_VALIDATOR_MAP: &'static [(&str, fn(&str) -> bool)] = &[
    ("both", validate_ipv46_address),
    ("ipv4", validate_ipv4_address),
    ("ipv6", validate_ipv6_address),
];

fn match_ip_address_validator(ip_address: &str, protocol: &str) -> bool {
    // validate ip address, if protocol is not in IP_ADDRESS_VALIDATOR_MAP, return false
    for (key, func) in IP_ADDRESS_VALIDATOR_MAP {
        if protocol == *key {
            return func(ip_address);
        }
    }
    false
}

#[pyfunction]
pub fn ip_address(ip_address: String, protocol: String) -> PyResult<bool> {
    // validate ip address
    Ok(match_ip_address_validator(&ip_address, &protocol))
}
