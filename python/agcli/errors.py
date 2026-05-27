"""Error types mirroring agcli Rust exit codes."""

from __future__ import annotations

from agcli import _agcli

# Re-export the native exception; it carries `.code` and `.hint` attributes.
AgcliError = _agcli.AgcliError


class NetworkError(AgcliError):
    """Exit code 10 — connection, DNS, transport failures."""


class AuthError(AgcliError):
    """Exit code 11 — wallet/password/key errors."""


class ValidationError(AgcliError):
    """Exit code 12 — invalid input or parse failures."""


class ChainError(AgcliError):
    """Exit code 13 — on-chain/runtime rejections."""


class TimeoutError(AgcliError):
    """Exit code 15 — operation deadline exceeded."""
