import validx
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "email, expected",
    [("email", False), ("example@example.com", True), ("example@example", False)],
)
def test_email(email: str, expected: bool):
    assert validx.email(email) == expected
