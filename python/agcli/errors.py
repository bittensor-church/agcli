"""Error types mirroring agcli Rust exit codes."""

from __future__ import annotations

from agcli import _agcli

AgcliError = _agcli.AgcliError
NetworkError = _agcli.NetworkError
AuthError = _agcli.AuthError
ValidationError = _agcli.ValidationError
ChainError = _agcli.ChainError
TimeoutError = _agcli.TimeoutError
IOError = _agcli.IOError

__all__ = [
    "AgcliError",
    "AuthError",
    "ChainError",
    "IOError",
    "NetworkError",
    "TimeoutError",
    "ValidationError",
]
