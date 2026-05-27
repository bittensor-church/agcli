"""Async chain client facade."""

from __future__ import annotations

from typing import Any

from agcli import _agcli
from agcli.types import (
    Balance,
    ChainIdentity,
    DelegateInfo,
    DynamicInfo,
    Metagraph,
    NetUid,
    Network,
    NeuronInfo,
    NeuronInfoLite,
    StakeInfo,
    SubnetHyperparameters,
    SubnetIdentity,
    SubnetInfo,
)

HashInput = bytes | str


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
    async def best_connection(cls, urls: list[str]) -> AsyncClient:
        return cls(await _agcli.Client.best_connection(urls))

    @classmethod
    async def connect_network(cls, network: Network | None = None) -> AsyncClient:
        return cls(await _agcli.Client.connect_network(network))

    @classmethod
    async def from_config(cls) -> AsyncClient:
        return cls(await _agcli.Client.from_config())

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

    async def reconnect(self) -> None:
        await self._inner.reconnect()

    async def is_alive(self) -> bool:
        return await self._inner.is_alive()

    async def invalidate_cache(self) -> None:
        await self._inner.invalidate_cache()

    async def get_balance(self, address: str) -> Balance:
        return await self._inner.get_balance(address)

    async def get_balance_at_hash(self, address: str, block_hash: HashInput) -> Balance:
        return await self._inner.get_balance_at_hash(address, block_hash)

    async def get_balance_at_block(self, address: str, block_number: int) -> Balance:
        return await self._inner.get_balance_at_block(address, block_number)

    async def get_balances_multi(self, addresses: list[str]) -> list[tuple[str, dict[str, Any]]]:
        return await self._inner.get_balances_multi(addresses)

    async def get_block_hash(self, block_number: int) -> str:
        return await self._inner.get_block_hash(block_number)

    async def get_block_number(self) -> int:
        return await self._inner.get_block_number()

    async def get_finalized_block_number(self) -> int:
        return await self._inner.get_finalized_block_number()

    async def pin_latest_block(self) -> str:
        return await self._inner.pin_latest_block()

    async def get_total_stake_at_block(self, block_number: int) -> Balance:
        return await self._inner.get_total_stake_at_block(block_number)

    async def get_block_header(
        self, block_hash: HashInput
    ) -> tuple[int, str, str, str]:
        return await self._inner.get_block_header(block_hash)

    async def get_block_extrinsic_count(self, block_hash: HashInput) -> int:
        return await self._inner.get_block_extrinsic_count(block_hash)

    async def get_block_timestamp(self, block_hash: HashInput) -> int | None:
        return await self._inner.get_block_timestamp(block_hash)

    async def get_total_issuance(self) -> Balance:
        return await self._inner.get_total_issuance()

    async def get_total_stake(self) -> Balance:
        return await self._inner.get_total_stake()

    async def get_total_networks(self) -> int:
        return await self._inner.get_total_networks()

    async def get_block_emission(self) -> Balance:
        return await self._inner.get_block_emission()

    async def get_subnet_registration_cost(self) -> Balance:
        return await self._inner.get_subnet_registration_cost()

    async def get_network_overview(self) -> tuple[int, dict[str, Any], int, dict[str, Any], dict[str, Any]]:
        return await self._inner.get_network_overview()

    async def get_total_issuance_at(self, block_hash: HashInput) -> Balance:
        return await self._inner.get_total_issuance_at(block_hash)

    async def get_total_stake_at(self, block_hash: HashInput) -> Balance:
        return await self._inner.get_total_stake_at(block_hash)

    async def get_total_networks_at(self, block_hash: HashInput) -> int:
        return await self._inner.get_total_networks_at(block_hash)

    async def get_block_emission_at(self, block_hash: HashInput) -> Balance:
        return await self._inner.get_block_emission_at(block_hash)

    async def get_block_number_at(self, block_hash: HashInput) -> int:
        return await self._inner.get_block_number_at(block_hash)

    async def get_all_subnets(self) -> list[dict[str, Any]]:
        return await self._inner.get_all_subnets()

    async def get_all_subnets_typed(self) -> list[SubnetInfo]:
        return [SubnetInfo(v) for v in await self.get_all_subnets()]

    async def get_subnet_info(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_info(netuid)

    async def get_subnet_info_typed(self, netuid: NetUid | int) -> SubnetInfo | None:
        value = await self.get_subnet_info(netuid)
        return None if value is None else SubnetInfo(value)

    async def get_subnet_hyperparams(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_hyperparams(netuid)

    async def get_subnet_hyperparams_typed(
        self, netuid: NetUid | int
    ) -> SubnetHyperparameters | None:
        value = await self.get_subnet_hyperparams(netuid)
        return None if value is None else SubnetHyperparameters(value)

    async def get_dynamic_info(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_dynamic_info(netuid)

    async def get_dynamic_info_typed(self, netuid: NetUid | int) -> DynamicInfo | None:
        value = await self.get_dynamic_info(netuid)
        return None if value is None else DynamicInfo(value)

    async def get_all_dynamic_info(self) -> list[dict[str, Any]]:
        return await self._inner.get_all_dynamic_info()

    async def get_all_dynamic_info_typed(self) -> list[DynamicInfo]:
        return [DynamicInfo(v) for v in await self.get_all_dynamic_info()]

    async def get_metagraph(self, netuid: NetUid | int) -> dict[str, Any]:
        return await self._inner.get_metagraph(netuid)

    async def get_metagraph_typed(self, netuid: NetUid | int) -> Metagraph:
        return Metagraph(await self.get_metagraph(netuid))

    async def get_neuron(self, netuid: NetUid | int, uid: int) -> dict[str, Any] | None:
        return await self._inner.get_neuron(netuid, uid)

    async def get_neuron_typed(self, netuid: NetUid | int, uid: int) -> NeuronInfo | None:
        value = await self.get_neuron(netuid, uid)
        return None if value is None else NeuronInfo(value)

    async def get_neurons_lite(self, netuid: NetUid | int) -> list[dict[str, Any]]:
        return await self._inner.get_neurons_lite(netuid)

    async def get_neurons_lite_typed(self, netuid: NetUid | int) -> list[NeuronInfoLite]:
        return [NeuronInfoLite(v) for v in await self.get_neurons_lite(netuid)]

    async def get_stake_for_coldkey(self, coldkey: str) -> list[dict[str, Any]]:
        return await self._inner.get_stake_for_coldkey(coldkey)

    async def get_stake_for_coldkey_typed(self, coldkey: str) -> list[StakeInfo]:
        return [StakeInfo(v) for v in await self.get_stake_for_coldkey(coldkey)]

    async def get_delegates(self) -> list[dict[str, Any]]:
        return await self._inner.get_delegates()

    async def get_delegates_typed(self) -> list[DelegateInfo]:
        return [DelegateInfo(v) for v in await self.get_delegates()]

    async def get_delegate(self, hotkey: str) -> dict[str, Any] | None:
        return await self._inner.get_delegate(hotkey)

    async def get_delegate_typed(self, hotkey: str) -> DelegateInfo | None:
        value = await self.get_delegate(hotkey)
        return None if value is None else DelegateInfo(value)

    async def get_identity(self, ss58: str) -> dict[str, Any] | None:
        return await self._inner.get_identity(ss58)

    async def get_identity_typed(self, ss58: str) -> ChainIdentity | None:
        value = await self.get_identity(ss58)
        return None if value is None else ChainIdentity(value)

    async def get_subnet_identity(self, netuid: NetUid | int) -> dict[str, Any] | None:
        return await self._inner.get_subnet_identity(netuid)

    async def get_subnet_identity_typed(
        self, netuid: NetUid | int
    ) -> SubnetIdentity | None:
        value = await self.get_subnet_identity(netuid)
        return None if value is None else SubnetIdentity(value)

    async def get_subnet_info_pinned(
        self, netuid: NetUid | int, block_hash: HashInput
    ) -> dict[str, Any] | None:
        return await self._inner.get_subnet_info_pinned(netuid, block_hash)

    async def get_subnet_hyperparams_pinned(
        self, netuid: NetUid | int, block_hash: HashInput
    ) -> dict[str, Any] | None:
        return await self._inner.get_subnet_hyperparams_pinned(netuid, block_hash)

    async def get_identity_pinned(self, ss58: str, block_hash: HashInput) -> dict[str, Any] | None:
        return await self._inner.get_identity_pinned(ss58, block_hash)

    async def get_subnet_identity_pinned(
        self, netuid: NetUid | int, block_hash: HashInput
    ) -> dict[str, Any] | None:
        return await self._inner.get_subnet_identity_pinned(netuid, block_hash)

    async def get_delegate_pinned(
        self, hotkey: str, block_hash: HashInput
    ) -> dict[str, Any] | None:
        return await self._inner.get_delegate_pinned(hotkey, block_hash)

    async def list_proxies_pinned(self, ss58: str, block_hash: HashInput) -> list[tuple[str, str, int]]:
        return await self._inner.list_proxies_pinned(ss58, block_hash)

    async def get_coldkey_swap_scheduled_pinned(
        self, ss58: str, block_hash: HashInput
    ) -> tuple[int, str] | None:
        return await self._inner.get_coldkey_swap_scheduled_pinned(ss58, block_hash)

    async def get_child_keys_pinned(
        self, hotkey_ss58: str, netuid: NetUid | int, block_hash: HashInput
    ) -> list[tuple[int, str]]:
        return await self._inner.get_child_keys_pinned(hotkey_ss58, netuid, block_hash)

    async def get_pending_child_keys_pinned(
        self, hotkey_ss58: str, netuid: NetUid | int, block_hash: HashInput
    ) -> tuple[list[tuple[int, str]], int] | None:
        return await self._inner.get_pending_child_keys_pinned(hotkey_ss58, netuid, block_hash)

    async def get_stake_for_coldkey_pinned(
        self, coldkey: str, block_hash: HashInput
    ) -> list[dict[str, Any]]:
        return await self._inner.get_stake_for_coldkey_pinned(coldkey, block_hash)

    async def get_stake_for_coldkey_at_block(
        self, coldkey: str, block_number: int
    ) -> list[dict[str, Any]]:
        return await self._inner.get_stake_for_coldkey_at_block(coldkey, block_number)

    async def get_identity_at_block(self, ss58: str, block_number: int) -> dict[str, Any] | None:
        return await self._inner.get_identity_at_block(ss58, block_number)

    async def get_all_subnets_at_block(self, block_number: int) -> list[dict[str, Any]]:
        return await self._inner.get_all_subnets_at_block(block_number)

    async def get_all_dynamic_info_at_block(self, block_number: int) -> list[dict[str, Any]]:
        return await self._inner.get_all_dynamic_info_at_block(block_number)

    async def get_dynamic_info_at_block(
        self, netuid: NetUid | int, block_number: int
    ) -> dict[str, Any] | None:
        return await self._inner.get_dynamic_info_at_block(netuid, block_number)

    async def get_neurons_lite_at_block(
        self, netuid: NetUid | int, block_number: int
    ) -> list[dict[str, Any]]:
        return await self._inner.get_neurons_lite_at_block(netuid, block_number)

    async def get_neuron_at_block(
        self, netuid: NetUid | int, uid: int, block_number: int
    ) -> dict[str, Any] | None:
        return await self._inner.get_neuron_at_block(netuid, uid, block_number)

    async def get_delegates_at_block(self, block_number: int) -> list[dict[str, Any]]:
        return await self._inner.get_delegates_at_block(block_number)

    async def get_total_issuance_at_block(self, block_number: int) -> Balance:
        return await self._inner.get_total_issuance_at_block(block_number)

    async def list_proxies(self, ss58: str) -> list[tuple[str, str, int]]:
        return await self._inner.list_proxies(ss58)

    async def list_multisig_pending(self, multisig_ss58: str) -> list[tuple[str, int, int, int, int]]:
        return await self._inner.list_multisig_pending(multisig_ss58)

    async def list_proxy_announcements(self, ss58: str) -> list[tuple[str, str, int]]:
        return await self._inner.list_proxy_announcements(ss58)

    async def get_coldkey_swap_scheduled(self, ss58: str) -> tuple[int, str] | None:
        return await self._inner.get_coldkey_swap_scheduled(ss58)

    async def get_child_keys(self, hotkey_ss58: str, netuid: NetUid | int) -> list[tuple[int, str]]:
        return await self._inner.get_child_keys(hotkey_ss58, netuid)

    async def get_parent_keys(self, hotkey_ss58: str, netuid: NetUid | int) -> list[tuple[int, str]]:
        return await self._inner.get_parent_keys(hotkey_ss58, netuid)

    async def get_pending_child_keys(
        self, hotkey_ss58: str, netuid: NetUid | int
    ) -> tuple[list[tuple[int, str]], int] | None:
        return await self._inner.get_pending_child_keys(hotkey_ss58, netuid)

    async def get_delegated(self, hotkey_ss58: str) -> list[dict[str, Any]]:
        return await self._inner.get_delegated(hotkey_ss58)

    async def get_weight_commits(
        self, netuid: NetUid | int, hotkey_ss58: str
    ) -> list[tuple[str, int, int, int]] | None:
        return await self._inner.get_weight_commits(netuid, hotkey_ss58)

    async def get_all_weight_commits(
        self, netuid: NetUid | int
    ) -> list[tuple[str, list[tuple[str, int, int, int]]]]:
        return await self._inner.get_all_weight_commits(netuid)

    async def get_reveal_period_epochs(self, netuid: NetUid | int) -> int:
        return await self._inner.get_reveal_period_epochs(netuid)

    async def get_weights_for_uid(self, netuid: NetUid | int, uid: int) -> list[tuple[int, int]]:
        return await self._inner.get_weights_for_uid(netuid, uid)

    async def get_all_weights(self, netuid: NetUid | int) -> list[tuple[int, list[tuple[int, int]]]]:
        return await self._inner.get_all_weights(netuid)

    async def get_commit_reveal_weights_version(self) -> int:
        return await self._inner.get_commit_reveal_weights_version()

    async def get_commitment(
        self, netuid: int, hotkey_ss58: str
    ) -> tuple[int, list[str]] | None:
        return await self._inner.get_commitment(netuid, hotkey_ss58)

    async def get_all_commitments(self, netuid: int) -> list[tuple[str, int, list[str]]]:
        return await self._inner.get_all_commitments(netuid)

    async def get_block_info_for_pow(self) -> tuple[int, str]:
        return await self._inner.get_block_info_for_pow()

    async def get_difficulty(self, netuid: NetUid | int) -> int:
        return await self._inner.get_difficulty(netuid)

    async def fetch_portfolio(self, coldkey_ss58: str) -> dict[str, Any]:
        return await self._inner.fetch_portfolio(coldkey_ss58)

    def __repr__(self) -> str:
        return f"AsyncClient(endpoint={self.endpoint!r})"
