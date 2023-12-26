import pytest

import yuko


@pytest.mark.parametrize(
    ("mac_address", "expected"),
    [
        ("01:23:45:67:ab:CD", True),
        ("01-23-45-67-ab-CD", True),
        ("01:2F:45:37:ab:CD", True),
        ("A1-2F-4E-68-ab-CD", True),
        ("00:1A:2B:3C:4D:5E", True),
        ("FF:FF:FF:FF:FF:FF", True),
        ("11:22:33:44:55:66", True),
        ("AA:BB:CC:DD:EE:FF", True),
        ("00:1A:2B:3C:4D:5E:6F", False),  # Invalid MAC address length
        ("00:1A:2B:3C:4D", False),  # Invalid MAC address length
        ("00:1A:2B:3C:4D:ZZ", False),  # Invalid hexadecimal digit
        ("00-00:-00-00-00", False),
        ("01:23-45:67-89:gh", False),
        ("", False),  # Empty MAC address
    ],
)
def test_mac_address(mac_address: str, expected: bool):
    assert yuko.mac_address(mac_address) == expected
