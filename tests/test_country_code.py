import yuko
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "country_code, expected",
    [
        ("TR", True),
        ("AA", False),
        ("FR", True),
        ("001", False),
        ("422", True),
        ("TUR", True),
        ("BBB", False),
    ],
)
def test_country_code(country_code: str, expected: bool):
    assert yuko.country_code(country_code) == expected
