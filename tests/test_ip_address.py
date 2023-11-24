import validx
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "ip_address, protocol, expected",
    [
        ("127.0.0.1", "ipv4", True),
        ("0.0.0.0", "ipv4", True),
        ("257.0.0.0", "ipv4", False),
        ("::1", "ipv6", True),
        ("::ffff:", "ipv6", False),
        ("::faff:,", "ipv6", False),
        ("::", "ipv6", True),
        ("::1", "ipv6", True),
        ("1::", "ipv6", True),
        ("dead:beef:0:0:0:0000:42:1", "ipv6", True),
        ("abcd:ef::42:1", "ipv6", True),
        ("0:0:0:0:0:ffff:1.2.3.4", "ipv6", True),
        ("::192.168.30.2", "ipv6", True),
        ("0000:0000:0000:0000:0000::", "ipv6", True),
        ("0:a:b:c:d:e:f::", "ipv6", True),
        ("0:a:b:c:d:e:u::", "ipv6", False),
    ],
)
def test_ip_address(ip_address: str, protocol: str, expected: bool):
    assert validx.ip_address(ip_address, protocol) == expected
