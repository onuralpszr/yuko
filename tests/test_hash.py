import yuko
import pytest


# test md5
@pytest.mark.parametrize(
    "hash, expected",
    [
        ("d41d8cd98f00b204e9800998ecf8427e", True),
        ("098f6bcd4621d373cade4e832627b4f6", True),
        ("5d41402abc4b2a76b9719d911017c592", True),
        ("7dcd4ce23d88e2ee95838f7b014b6284", True),
        ("6dcd4ce23d88e2ee95838f7b014b6284", True),
        ("1234567890abcdefg1234567890abcdefg", False),
        ("abcdefg1234567890abcdefg1234567890", False),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
        ("abcdefg1234567890abcdefg1234567890abcdefg1234567890", False),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
    ],
)
def test_md5(hash: str, expected: bool):
    assert yuko.md5(hash) == expected


@pytest.mark.parametrize(
    "hash, expected",
    [
        ("da39a3ee5e6b4b0d3255bfef95601890afd80709", True),
        ("DA39A3EE5E6B4B0D3255BFEF95601890AFD80709", True),
        ("za39a3ee5e6b4b0d3255bfef95601890afd80709", False),
        ("da39e5e6b4b0d3255bfef95601890afd80709", False),
        ("daaaa39a3ee5e6b4b0d3255bfef95601890afd80709", False),
        ("5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8", True),
        ("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed", True),
        ("3ca25ae354e192b26879f651a51d92aa8a34d8d3", True),
        ("1234567890abcdefg1234567890abcdefg1234567890", False),
        ("abcdefg1234567890abcdefg1234567890abcdefg1234567890", False),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
    ],
)
def test_sha1(hash: str, expected: bool):
    assert yuko.sha1(hash) == expected


# test sha224
@pytest.mark.parametrize(
    "hash, expected",
    [
        ("40997ec67c716eb08a4506df950cf3fec202edf2590c7cfe88456990", True),
        ("90a69457550f15e6b468b8c5ed4c2a456524e8e8e61e9a57e10ab8a4", True),
        ("a0a2b3c4d5e6f7081920a1b2c3d4e5f6a0a2b3c4d5e6f7081920a1b2", True),
        ("b1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d0a1b2c3d4", False),
        ("d3e4f5a6b7c8d9e0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5", False),
        ("1234567890abcdefg", False),
        ("abcdefg1234567890", False),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
        ("abcdefg1234567890abcdefg1234567890abcdefg1234567890", False),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
    ],
)
def test_sha224(hash: str, expected: bool):
    assert yuko.sha224(hash) == expected


# test sha256
@pytest.mark.parametrize(
    "hash, expected",
    [
        ("6dcd4ce23d88e2ee95838f7b014b6284ffdb5a5a7b42c4a6e38e4e4f0a6d1df5", True),
        ("3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d", True),
        ("6dcd4ce23d88e2ee95838f7b014b6284ffdb5a5a7b42c4a6e38e4e4f0a6d1df6", True),
        ("1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg", False),
        ("abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890", False),
        (
            "1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
    ],
)
def test_sha256(hash: str, expected: bool):
    assert yuko.sha256(hash) == expected


# test sha512
@pytest.mark.parametrize(
    "hash, expected",
    [
        (
            "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3aef53f2385d66c10c",
            True,
        ),
        (
            "ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff",
            True,
        ),
        ("d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592", False),
        ("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", False),
        (
            "1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
        (
            "abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
        (
            "1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
        (
            "abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
        (
            "1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg1234567890abcdefg",
            False,
        ),
    ],
)
def test_sha512(hash: str, expected: bool):
    assert yuko.sha512(hash) == expected
