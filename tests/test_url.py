import yuko
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "url, expected",
    [
        ("invalidurl", False),
        ("http://example.com", True),
        ("https://example.com", True),
        ("http://example.com/", True),
        ("http://example.com/path", True),
        ("http://example.com/path/", True),
        ("ftp://example.com", True),
        ("http://", False),  # Invalid URL example
        ("https://www.example&.com", False),  # Invalid URL example
        ("https://www.example.com/path with spaces", False),  # Invalid URL example
    ],
)
def test_url(url: str, expected: bool):
    assert yuko.url(url) == expected
