import pytest
import yuko


# pytest parametrize
@pytest.mark.parametrize(
    ("domain", "expected"),
    [
        ("domain", False),
        ("example.com", True),
        ("example", False),
        ("subdomain.example.com", True),
        ("www.example.com", True),
        ("example.com.", False),
        ("example@domain.com", False),
        ("123example.com", True),
        ("example-.com", False),
        ("my-domain.co.uk", True),
        ("test123.test-domain.com", True),
        ("example_domain.com", True),
        (".example12.com", False),
        ("example  12.com", False),
    ],
)
def test_domain(domain: str, expected: bool):
    assert yuko.domain(domain, False, False) == expected
