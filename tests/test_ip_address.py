import yuko
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "ip_address, protocol, expected",
    [
        ("127.0.0.1", "ipv4", True),
        ("0.0.0.0", "ipv4", True),
        ("257.0.0.0", "ipv4", False),
        ("1.2.3.04", "ipv4", False),
        ("192.168.000.001", "ipv4", False),
        ("٧.2٥.3٣.243", "ipv4", False),
        ("::1", "ipv6", True),
        ("::ffff:", "ipv6", False),
        ("::faff:,", "ipv6", False),
        ("::", "ipv6", True),
        ("::1", "ipv6", True),
        ("1::", "ipv6", True),
        ("1:2:3:4:5:6:7:8", "ipv6", True),
        ("12345::", "ipv6", False),
        ("dead:beef:0:0:0:0000:42:1", "ipv6", True),
        ("abcd:ef::42:1", "ipv6", True),
        ("0:0:0:0:0:ffff:1.2.3.4", "ipv6", True),
        ("::192.168.30.2", "ipv6", True),
        ("0000:0000:0000:0000:0000::", "ipv6", True),
        ("0:a:b:c:d:e:f::", "ipv6", True),
        ("0:a:b:c:d:e:u::", "ipv6", False),
        ("1.2.3.04", "both", False),
        ("1.02.3.4", "both", False),
        ("192.168.000.001", "both", False),
        ("016.016.016.016", "both", False),
        ("1:2:3:4:5:6:7:8", "both", True),
        ("fe80::1", "both", True),
        ("1:2:3:4:5:6:7:8", "both", True),
        ("0.0.0.0", "both", True),
    ],
)
def test_ip_address(ip_address: str, protocol: str, expected: bool):
    assert yuko.ip_address(ip_address, protocol) == expected
