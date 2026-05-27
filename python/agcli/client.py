"""Async chain client facade."""

from __future__ import annotations

from typing import Any

from agcli import _agcli
from agcli.types import Balance, NetUid, Network


class AsyncClient:
    """Async-first wrapper around the native ``agcli._agcli.Client``."""

    def __init__(self, inner: _agcli.Client) -> None:
        self._inner = inner

    @classmethod
    async def connect(cls, url: str) -> AsyncClient:
        return cls(await _agcli.Client.connect(url))

    @classmethod
    async def connect_with_retry(cls, urls: list[str]) -> AsyncClient:
        return cls(await _agcli.Client.connect_with_retry(urls))

    @classmethod
    async def connect_network(cls, network: Network | None = None) -> AsyncClient:
        return cls(await _agcli.Client.connect_network(network))

    @classmethod
    async def from_config(cls) -> AsyncClient:
        return cls(await _agcli.Client.from_config())

    @property
    def endpoint(self) -> str:
        return self._inner.endpoint

    async def get_balance(self, address: str) -> Balance:
        return await self._inner.get_balance(address)

    async def get_all_subnets(self) -> list[dict[str, Any]]:
        return await self._inner.get_all_subnets()

    async def get_subnet_info(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_info(netuid)

    async def get_subnet_hyperparams(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_hyperparams(netuid)

    async def get_dynamic_info(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_dynamic_info(netuid)

    async def get_metagraph(self, netuid: NetUid | int) -> dict[str, Any]:
        return await self._inner.get_metagraph(netuid)

    async def get_neuron(self, netuid: NetUid | int, uid: int) -> dict[str, Any] | None:
        return await self._inner.get_neuron(netuid, uid)

    async def get_neurons_lite(self, netuid: NetUid | int) -> list[dict[str, Any]]:
        return await self._inner.get_neurons_lite(netuid)

    async def get_stake_for_coldkey(self, coldkey: str) -> list[dict[str, Any]]:
        return await self._inner.get_stake_for_coldkey(coldkey)

    async def get_delegates(self) -> list[dict[str, Any]]:
        return await self._inner.get_delegates()

    async def get_delegate(self, hotkey: str) -> dict[str, Any] | None:
        return await self._inner.get_delegate(hotkey)

    async def get_identity(self, ss58: str) -> dict[str, Any] | None:
        return await self._inner.get_identity(ss58)

    async def get_subnet_identity(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_identity(netuid)

    def __repr__(self) -> str:
        return f"AsyncClient(endpoint={self.endpoint!r})"
