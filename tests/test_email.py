import yuko
import pytest


# pytest parametrize
@pytest.mark.parametrize(
    "email, expected",
    [
        ("email", False),
        ("example@example.com", True),
        ("example@example", False),
        ("email@here.com", True),
        ("weirder-email@here.and.there.com", True),
        ("email@127.local.home.arpa", True),
        ("example@valid-----hyphens.com", True),
        ("example@valid-with-hyphens.com", True),
    ],
)
def test_email(email: str, expected: bool):
    assert yuko.email(email) == expected
