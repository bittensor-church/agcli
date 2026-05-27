import pytest

from agcli import (
    AuthError,
    ChainError,
    IOError,
    NetworkError,
    TimeoutError,
    ValidationError,
    _agcli,
)


@pytest.mark.parametrize(
    ("message", "error_type"),
    [
        ("Failed to connect to endpoint", NetworkError),
        ("Wrong password for wallet coldkey", AuthError),
        ("Invalid SS58 address: bad checksum", ValidationError),
        ("Extrinsic failed: insufficient balance", ChainError),
        ("Operation timed out after 30s", TimeoutError),
        ("Permission denied writing to /tmp/file", IOError),
    ],
)
def test_error_subclass_mapping(message: str, error_type: type[Exception]) -> None:
    with pytest.raises(error_type):
        _agcli.raise_test_error(message)
