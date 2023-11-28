import validx
import pytest


@pytest.mark.parametrize(
    "hash, expected",
    [
        ("da39a3ee5e6b4b0d3255bfef95601890afd80709", True),
        ("DA39A3EE5E6B4B0D3255BFEF95601890AFD80709", True),
        ("za39a3ee5e6b4b0d3255bfef95601890afd80709", False),
        ("da39e5e6b4b0d3255bfef95601890afd80709", False),
        ("daaaa39a3ee5e6b4b0d3255bfef95601890afd80709", False),
    ],
)
def test_sha1(hash: str, expected: bool):
    assert validx.sha1(hash) == expected
