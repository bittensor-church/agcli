"""Declarative test environment scaffolding — re-exports `agcli::scaffold`."""

from __future__ import annotations

from agcli import _agcli

NeuronConfig = _agcli.NeuronConfig
SubnetConfig = _agcli.SubnetConfig
ChainConfig = _agcli.ChainConfig
ScaffoldConfig = _agcli.ScaffoldConfig
NeuronResult = _agcli.NeuronResult
SubnetResult = _agcli.SubnetResult
ScaffoldResult = _agcli.ScaffoldResult


def load_config(path: str) -> ScaffoldConfig:
    """Load a scaffold config from a TOML file."""
    return _agcli.scaffold.load_config(path)


async def run(config: ScaffoldConfig) -> ScaffoldResult:
    """Run the full scaffold orchestration. Requires Docker + localnet."""
    return await _agcli.scaffold.run(config)


__all__ = [
    "ChainConfig",
    "NeuronConfig",
    "NeuronResult",
    "ScaffoldConfig",
    "ScaffoldResult",
    "SubnetConfig",
    "SubnetResult",
    "load_config",
    "run",
]
