"""AdminUtils sudo helpers — re-exports `agcli::admin`.

Each `set_*` function takes `(client, wallet, netuid, value)` (or
`(client, wallet, value)` for global params) and returns the extrinsic hash as
a hex string. `raw_admin_call` lets you invoke any AdminUtils extrinsic by name
with a positional list of arguments (ints, bools, or strings).
"""

from __future__ import annotations

from typing import Any

from agcli import _agcli
from agcli.client import AsyncClient
from agcli.types import NetUid
from agcli.wallet import Wallet


def known_params() -> list[tuple[str, str, list[str]]]:
    """Return `(name, description, arg_types)` for every known admin param."""
    return _agcli.admin.known_params()


async def raw_admin_call(
    client: AsyncClient,
    wallet: Wallet,
    call_name: str,
    args: list[Any],
) -> str:
    return await _agcli.admin.raw_admin_call(
        client._inner, wallet._inner, call_name, args
    )


async def set_tempo(client: AsyncClient, wallet: Wallet, netuid: NetUid | int, tempo: int) -> str:
    return await _agcli.admin.set_tempo(client._inner, wallet._inner, netuid, tempo)


async def set_max_allowed_validators(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, max: int
) -> str:
    return await _agcli.admin.set_max_allowed_validators(client._inner, wallet._inner, netuid, max)


async def set_max_allowed_uids(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, max: int
) -> str:
    return await _agcli.admin.set_max_allowed_uids(client._inner, wallet._inner, netuid, max)


async def set_immunity_period(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, period: int
) -> str:
    return await _agcli.admin.set_immunity_period(client._inner, wallet._inner, netuid, period)


async def set_min_allowed_weights(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, min: int
) -> str:
    return await _agcli.admin.set_min_allowed_weights(client._inner, wallet._inner, netuid, min)


async def set_max_weight_limit(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, limit: int
) -> str:
    return await _agcli.admin.set_max_weight_limit(client._inner, wallet._inner, netuid, limit)


async def set_weights_set_rate_limit(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, limit: int
) -> str:
    return await _agcli.admin.set_weights_set_rate_limit(client._inner, wallet._inner, netuid, limit)


async def set_commit_reveal_weights_enabled(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, enabled: bool
) -> str:
    return await _agcli.admin.set_commit_reveal_weights_enabled(
        client._inner, wallet._inner, netuid, enabled
    )


async def set_difficulty(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, difficulty: int
) -> str:
    return await _agcli.admin.set_difficulty(client._inner, wallet._inner, netuid, difficulty)


async def set_bonds_moving_average(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, avg: int
) -> str:
    return await _agcli.admin.set_bonds_moving_average(client._inner, wallet._inner, netuid, avg)


async def set_target_registrations_per_interval(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, target: int
) -> str:
    return await _agcli.admin.set_target_registrations_per_interval(
        client._inner, wallet._inner, netuid, target
    )


async def set_activity_cutoff(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, cutoff: int
) -> str:
    return await _agcli.admin.set_activity_cutoff(client._inner, wallet._inner, netuid, cutoff)


async def set_serving_rate_limit(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, limit: int
) -> str:
    return await _agcli.admin.set_serving_rate_limit(client._inner, wallet._inner, netuid, limit)


async def set_default_take(client: AsyncClient, wallet: Wallet, take: int) -> str:
    return await _agcli.admin.set_default_take(client._inner, wallet._inner, take)


async def set_tx_rate_limit(client: AsyncClient, wallet: Wallet, limit: int) -> str:
    return await _agcli.admin.set_tx_rate_limit(client._inner, wallet._inner, limit)


async def set_min_difficulty(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, min: int
) -> str:
    return await _agcli.admin.set_min_difficulty(client._inner, wallet._inner, netuid, min)


async def set_max_difficulty(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, max: int
) -> str:
    return await _agcli.admin.set_max_difficulty(client._inner, wallet._inner, netuid, max)


async def set_adjustment_interval(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, interval: int
) -> str:
    return await _agcli.admin.set_adjustment_interval(
        client._inner, wallet._inner, netuid, interval
    )


async def set_adjustment_alpha(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, alpha: int
) -> str:
    return await _agcli.admin.set_adjustment_alpha(client._inner, wallet._inner, netuid, alpha)


async def set_kappa(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, kappa: int
) -> str:
    return await _agcli.admin.set_kappa(client._inner, wallet._inner, netuid, kappa)


async def set_rho(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, rho: int
) -> str:
    return await _agcli.admin.set_rho(client._inner, wallet._inner, netuid, rho)


async def set_min_burn(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, min_burn: int
) -> str:
    return await _agcli.admin.set_min_burn(client._inner, wallet._inner, netuid, min_burn)


async def set_max_burn(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, max_burn: int
) -> str:
    return await _agcli.admin.set_max_burn(client._inner, wallet._inner, netuid, max_burn)


async def set_liquid_alpha_enabled(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, enabled: bool
) -> str:
    return await _agcli.admin.set_liquid_alpha_enabled(client._inner, wallet._inner, netuid, enabled)


async def set_alpha_values(
    client: AsyncClient,
    wallet: Wallet,
    netuid: NetUid | int,
    alpha_low: int,
    alpha_high: int,
) -> str:
    return await _agcli.admin.set_alpha_values(
        client._inner, wallet._inner, netuid, alpha_low, alpha_high
    )


async def set_yuma3_enabled(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, enabled: bool
) -> str:
    return await _agcli.admin.set_yuma3_enabled(client._inner, wallet._inner, netuid, enabled)


async def set_bonds_penalty(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, penalty: int
) -> str:
    return await _agcli.admin.set_bonds_penalty(client._inner, wallet._inner, netuid, penalty)


async def set_subnet_moving_alpha(client: AsyncClient, wallet: Wallet, alpha: int) -> str:
    return await _agcli.admin.set_subnet_moving_alpha(client._inner, wallet._inner, alpha)


async def set_mechanism_count(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, count: int
) -> str:
    return await _agcli.admin.set_mechanism_count(client._inner, wallet._inner, netuid, count)


async def set_mechanism_emission_split(
    client: AsyncClient,
    wallet: Wallet,
    netuid: NetUid | int,
    split: list[int],
) -> str:
    return await _agcli.admin.set_mechanism_emission_split(
        client._inner, wallet._inner, netuid, split
    )


async def set_stake_threshold(client: AsyncClient, wallet: Wallet, threshold: int) -> str:
    return await _agcli.admin.set_stake_threshold(client._inner, wallet._inner, threshold)


async def set_nominator_min_required_stake(
    client: AsyncClient, wallet: Wallet, min_stake: int
) -> str:
    return await _agcli.admin.set_nominator_min_required_stake(
        client._inner, wallet._inner, min_stake
    )


async def set_network_registration_allowed(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, allowed: bool
) -> str:
    return await _agcli.admin.set_network_registration_allowed(
        client._inner, wallet._inner, netuid, allowed
    )


async def set_network_pow_registration_allowed(
    client: AsyncClient, wallet: Wallet, netuid: NetUid | int, allowed: bool
) -> str:
    return await _agcli.admin.set_network_pow_registration_allowed(
        client._inner, wallet._inner, netuid, allowed
    )


__all__ = [
    "known_params",
    "raw_admin_call",
    "set_activity_cutoff",
    "set_adjustment_alpha",
    "set_adjustment_interval",
    "set_alpha_values",
    "set_bonds_moving_average",
    "set_bonds_penalty",
    "set_commit_reveal_weights_enabled",
    "set_default_take",
    "set_difficulty",
    "set_immunity_period",
    "set_kappa",
    "set_liquid_alpha_enabled",
    "set_max_allowed_uids",
    "set_max_allowed_validators",
    "set_max_burn",
    "set_max_difficulty",
    "set_max_weight_limit",
    "set_mechanism_count",
    "set_mechanism_emission_split",
    "set_min_allowed_weights",
    "set_min_burn",
    "set_min_difficulty",
    "set_network_pow_registration_allowed",
    "set_network_registration_allowed",
    "set_nominator_min_required_stake",
    "set_rho",
    "set_serving_rate_limit",
    "set_stake_threshold",
    "set_subnet_moving_alpha",
    "set_target_registrations_per_interval",
    "set_tempo",
    "set_tx_rate_limit",
    "set_weights_set_rate_limit",
    "set_yuma3_enabled",
]
