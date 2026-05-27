"""Event subscription helpers.

Provides an `EventFilter` dataclass mirroring `agcli::events::EventFilter` plus
the async iterators returned from `AsyncClient.subscribe_events`,
`AsyncClient.subscribe_events_filtered`, and `AsyncClient.subscribe_blocks`.

Closing the iterator (calling `await stream.close()` or letting it be garbage
collected) cancels the underlying chain subscription task.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any, AsyncIterator

from agcli import _agcli

EVENT_CATEGORIES: tuple[str, ...] = (
    "all",
    "staking",
    "registration",
    "transfer",
    "weights",
    "subnet",
    "delegation",
    "keys",
    "swap",
    "governance",
    "crowdloan",
)


@dataclass
class EventFilter:
    """Subscription filter mirroring `agcli::events::EventFilter`.

    `category` matches the Rust enum variant (`all`, `staking`, `registration`,
    `transfer`, `weights`, `subnet`, `delegation`, `keys`, `swap`, `governance`,
    `crowdloan`). `netuid` and `account` apply additional narrowing on top of
    the category, mirroring the `subscribe_events_filtered` arguments.
    """

    category: str = "all"
    netuid: int | None = None
    account: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


EventStream = _agcli.EventStream


class _EventStreamWrapper:
    """Async iterator wrapper around the native EventStream."""

    def __init__(self, inner: _agcli.EventStream) -> None:
        self._inner = inner

    def __aiter__(self) -> "_EventStreamWrapper":
        return self

    async def __anext__(self) -> dict[str, Any]:
        return await self._inner.__anext__()

    async def close(self) -> None:
        await self._inner.close()

    async def __aenter__(self) -> "_EventStreamWrapper":
        return self

    async def __aexit__(self, *exc: Any) -> None:
        await self._inner.close()


def _wrap_stream(stream: _agcli.EventStream) -> AsyncIterator[dict[str, Any]]:
    return _EventStreamWrapper(stream)


__all__ = [
    "EVENT_CATEGORIES",
    "EventFilter",
    "EventStream",
    "_wrap_stream",
]
