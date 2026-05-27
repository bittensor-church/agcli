"""Typed views of native binding objects."""

from __future__ import annotations

from agcli import _agcli

Balance = _agcli.Balance
AlphaBalance = _agcli.AlphaBalance
NetUid = _agcli.NetUid
Network = _agcli.Network
SubnetInfo = _agcli.SubnetInfo
NeuronInfo = _agcli.NeuronInfo
NeuronInfoLite = _agcli.NeuronInfoLite
Metagraph = _agcli.Metagraph
StakeInfo = _agcli.StakeInfo
DelegateInfo = _agcli.DelegateInfo
DynamicInfo = _agcli.DynamicInfo
SubnetHyperparameters = _agcli.SubnetHyperparameters
ChainIdentity = _agcli.ChainIdentity
SubnetIdentity = _agcli.SubnetIdentity

__all__ = [
    "AlphaBalance",
    "Balance",
    "ChainIdentity",
    "DelegateInfo",
    "DynamicInfo",
    "Metagraph",
    "NetUid",
    "Network",
    "NeuronInfo",
    "NeuronInfoLite",
    "StakeInfo",
    "SubnetHyperparameters",
    "SubnetIdentity",
    "SubnetInfo",
]
