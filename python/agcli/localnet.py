"""Docker-based local chain management — re-exports `agcli::localnet`."""

from __future__ import annotations

from agcli import _agcli

LocalnetConfig = _agcli.LocalnetConfig
LocalnetInfo = _agcli.LocalnetInfo
LocalnetStatus = _agcli.LocalnetStatus
DevAccount = _agcli.DevAccount

DEFAULT_IMAGE = _agcli.localnet.DEFAULT_IMAGE
DEFAULT_CONTAINER = _agcli.localnet.DEFAULT_CONTAINER
DEFAULT_WS = _agcli.localnet.DEFAULT_WS


def dev_accounts() -> list[DevAccount]:
    """Return the pre-funded dev accounts (Alice, Bob) available on localnet."""
    return _agcli.localnet.dev_accounts()


async def start(config: LocalnetConfig | None = None) -> LocalnetInfo:
    """Start a localnet Docker container, returning `LocalnetInfo` on success."""
    return await _agcli.localnet.start(config)


def stop(container_name: str = DEFAULT_CONTAINER) -> None:
    """Stop and remove a localnet container."""
    _agcli.localnet.stop(container_name)


async def status(
    container_name: str | None = None, port: int = 9944
) -> LocalnetStatus:
    """Return current status for a named localnet container (`started_at` + legacy `uptime`)."""
    return await _agcli.localnet.status(container_name, port)


async def reset(config: LocalnetConfig | None = None) -> LocalnetInfo:
    """Stop and re-start a localnet container."""
    return await _agcli.localnet.reset(config)


def logs(container_name: str = DEFAULT_CONTAINER, tail: int | None = None) -> str:
    """Return container logs."""
    return _agcli.localnet.logs(container_name, tail)


__all__ = [
    "DEFAULT_CONTAINER",
    "DEFAULT_IMAGE",
    "DEFAULT_WS",
    "DevAccount",
    "LocalnetConfig",
    "LocalnetInfo",
    "LocalnetStatus",
    "dev_accounts",
    "logs",
    "reset",
    "start",
    "status",
    "stop",
]
