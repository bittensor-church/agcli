"""Live polling helpers — thin wrappers over `agcli::live::*`.

These functions print formatted output to stdout (matching the Rust SDK
behaviour) and run until the underlying chain stream ends or the call is
cancelled. They are best-effort interactive helpers; programmatic consumers
should subscribe to events directly via `AsyncClient.subscribe_events` or use
the typed query methods.
"""

from __future__ import annotations

from agcli import _agcli
from agcli.client import AsyncClient
from agcli.types import NetUid


async def live_dynamic(client: AsyncClient, interval_secs: int = 5) -> None:
    """Stream dynamic info deltas every `interval_secs` seconds."""
    await _agcli.live.live_dynamic(client._inner, interval_secs)


async def live_metagraph(
    client: AsyncClient, netuid: NetUid | int, interval_secs: int = 5
) -> None:
    """Stream per-subnet metagraph snapshots every `interval_secs` seconds."""
    await _agcli.live.live_metagraph(client._inner, netuid, interval_secs)


async def live_portfolio(
    client: AsyncClient, coldkey_ss58: str, interval_secs: int = 5
) -> None:
    """Stream per-coldkey portfolio snapshots every `interval_secs` seconds."""
    await _agcli.live.live_portfolio(client._inner, coldkey_ss58, interval_secs)


__all__ = ["live_dynamic", "live_metagraph", "live_portfolio"]
