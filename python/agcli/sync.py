"""Synchronous chain client facade."""

from __future__ import annotations

from agcli import _agcli
from agcli.types import Balance, NetUid, Network


class SyncClient:
    """Blocking wrapper around the native ``agcli._agcli.ClientSync``."""

    def __init__(self, inner: _agcli.ClientSync) -> None:
        self._inner = inner

    @classmethod
    def connect(cls, url: str) -> SyncClient:
        return cls(_agcli.ClientSync.connect(url))

    @classmethod
    def connect_with_retry(cls, urls: list[str]) -> SyncClient:
        return cls(_agcli.ClientSync.connect_with_retry(urls))

    @classmethod
    def best_connection(cls, urls: list[str]) -> SyncClient:
        return cls(_agcli.ClientSync.best_connection(urls))

    @classmethod
    def connect_network(cls, network: Network | None = None) -> SyncClient:
        return cls(_agcli.ClientSync.connect_network(network))

    @classmethod
    def from_config(cls) -> SyncClient:
        return cls(_agcli.ClientSync.from_config())

    @property
    def endpoint(self) -> str:
        return self._inner.endpoint

    @property
    def network(self) -> Network:
        return self._inner.network

    def set_dry_run(self, enabled: bool) -> None:
        self._inner.set_dry_run(enabled)

    def is_dry_run(self) -> bool:
        return self._inner.is_dry_run()

    def set_finalization_timeout(self, timeout: int) -> None:
        self._inner.set_finalization_timeout(timeout)

    def finalization_timeout(self) -> int:
        return self._inner.finalization_timeout()

    def set_mortality_blocks(self, blocks: int) -> None:
        self._inner.set_mortality_blocks(blocks)

    def mortality_blocks(self) -> int:
        return self._inner.mortality_blocks()

    def reconnect(self) -> None:
        self._inner.reconnect()

    def is_alive(self) -> bool:
        return self._inner.is_alive()

    def invalidate_cache(self) -> None:
        self._inner.invalidate_cache()

    def get_balance(self, address: str) -> Balance:
        return self._inner.get_balance(address)

    def get_total_issuance(self) -> Balance:
        return self._inner.get_total_issuance()

    def get_metagraph(self, netuid: NetUid | int) -> dict:
        return self._inner.get_metagraph(netuid)

    def __getattr__(self, name: str):
        return getattr(self._inner, name)
